//! RO:WHAT — CLI kill-switch drill report for halt and recovery authority behavior.
//! RO:WHY — BUILD_PLAN2 Phase 12 requires operator-visible halt/recovery drill output.
//! RO:INTERACTS — rox-anchor-core kill-switch review, authority map, posture model, and CLI dispatch.
//! RO:INVARIANTS — drill output is local/report-only and never claims network submission or finality.
//! RO:SECURITY — no live RPC, wallet/key loading, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test kill_switch_drill_command.

use rox_anchor_core::{
    review_kill_switch_drill, AnchorCluster, AnchorEnvironmentMode, AnchorOperationalPosture,
    AnchorSafetyProfile, AuthorityAssignment, AuthorityKeyId, AuthorityMap,
    AuthoritySeparationMode, ChallengePosture, ClusterAllowlist, ClusterId, HaltPosture,
    KillSwitchAction, KillSwitchDrillRequest, KillSwitchDrillStage, MintId, OperationId,
    OperatorRole, ProgramId, RecoveryPosture, SubmissionMode, TokenAccountId,
};

use rox_anchor_coordinator::{
    review_coordinator_incident_drill, review_coordinator_request, CoordinatorConfig,
    CoordinatorIncidentDrillEvidence, CoordinatorIncidentStage, CoordinatorReviewRequest,
};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
use rox_anchor_relayer::{
    authorize_private_testnet_sender, review_pilot_incident_receipt,
    simulate_private_pilot_transaction_plan, CappedTestnetSubmissionLimits,
    PilotIncidentReceiptEvidence, PilotReceiptId, PrivatePilotSimulationPlan,
    PrivatePilotSimulationResult, PrivatePilotTransactionKind, PrivatePilotTransactionStep,
    PrivateTestnetSenderAuthorization, PrivateTestnetSenderRequest, RelayerConfig, RelayerDryRun,
    RelayerPrivatePilotConfig, RelayerSubmissionRequest, TransactionSimulationPlan,
    PRIVATE_TESTNET_CAPPED_SEND_APPROVAL,
};
use rox_anchor_rpc_proof::{
    review_readback_after_send, review_rpc_observations, ExpectedRpcBinding,
    ReadOnlyRpcObservationReview, RpcCommitmentLevel, RpcObservation, RpcProofConfig,
};

use crate::CliError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DrillAuthority {
    Halt,
    Recovery,
    Upgrade,
    Wrong,
}

impl DrillAuthority {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "halt" | "halt-authority" | "halt_authority" => Ok(Self::Halt),
            "recovery" | "recovery-authority" | "recovery_authority" => Ok(Self::Recovery),
            "upgrade" | "upgrade-authority" | "upgrade_authority" => Ok(Self::Upgrade),
            "wrong" | "wrong-authority" | "wrong_authority" => Ok(Self::Wrong),
            other => Err(CliError::UnknownDrillFlag(format!(
                "--authority value `{other}`; expected halt, recovery, upgrade, or wrong"
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Halt => "halt_authority",
            Self::Recovery => "recovery_authority",
            Self::Upgrade => "upgrade_authority",
            Self::Wrong => "wrong_authority",
        }
    }

    fn key(self) -> AuthorityKeyId {
        match self {
            Self::Halt => key("halt-authority-phase12-cli-key"),
            Self::Recovery => key("recovery-authority-phase12-cli-key"),
            Self::Upgrade => key("upgrade-authority-phase12-cli-key"),
            Self::Wrong => key("wrong-authority-phase12-cli-key"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DrillPosture {
    Clear,
    ChallengeOpen,
    ChallengeAccepted,
    Halted,
    RecoveryRequired,
    RecoveryInReview,
    HaltedRecoveryRequired,
    RecoveryResolved,
}

impl DrillPosture {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "clear" => Ok(Self::Clear),
            "challenge-open" | "challenge_open" => Ok(Self::ChallengeOpen),
            "challenge-accepted" | "challenge_accepted" => Ok(Self::ChallengeAccepted),
            "halted" => Ok(Self::Halted),
            "recovery-required" | "recovery_required" => Ok(Self::RecoveryRequired),
            "recovery-in-review" | "recovery_in_review" => Ok(Self::RecoveryInReview),
            "halted-recovery-required" | "halted_recovery_required" => {
                Ok(Self::HaltedRecoveryRequired)
            }
            "recovery-resolved" | "recovery_resolved" => Ok(Self::RecoveryResolved),
            other => Err(CliError::UnknownDrillFlag(format!(
                "--posture value `{other}`; expected clear, challenge-open, challenge-accepted, halted, recovery-required, recovery-in-review, halted-recovery-required, or recovery-resolved"
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::ChallengeOpen => "challenge_open",
            Self::ChallengeAccepted => "challenge_accepted",
            Self::Halted => "halted",
            Self::RecoveryRequired => "recovery_required",
            Self::RecoveryInReview => "recovery_in_review",
            Self::HaltedRecoveryRequired => "halted_recovery_required",
            Self::RecoveryResolved => "recovery_resolved",
        }
    }

    fn posture(self) -> AnchorOperationalPosture {
        match self {
            Self::Clear => AnchorOperationalPosture::clear(),
            Self::ChallengeOpen => AnchorOperationalPosture::new(
                ChallengePosture::Open,
                HaltPosture::Active,
                RecoveryPosture::NotRequired,
            ),
            Self::ChallengeAccepted => AnchorOperationalPosture::new(
                ChallengePosture::Accepted,
                HaltPosture::Active,
                RecoveryPosture::NotRequired,
            ),
            Self::Halted => AnchorOperationalPosture::halted(),
            Self::RecoveryRequired => AnchorOperationalPosture::recovery_required(),
            Self::RecoveryInReview => AnchorOperationalPosture::new(
                ChallengePosture::Clear,
                HaltPosture::Active,
                RecoveryPosture::InReview,
            ),
            Self::HaltedRecoveryRequired => AnchorOperationalPosture::halted_recovery_required(),
            Self::RecoveryResolved => AnchorOperationalPosture::recovery_resolved(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DrillArgs {
    stage: KillSwitchDrillStage,
    action: KillSwitchAction,
    posture: DrillPosture,
    authority: DrillAuthority,
    help: bool,
}

impl Default for DrillArgs {
    fn default() -> Self {
        Self {
            stage: KillSwitchDrillStage::BeforeProofAcceptance,
            action: KillSwitchAction::Halt,
            posture: DrillPosture::Clear,
            authority: DrillAuthority::Halt,
            help: false,
        }
    }
}

pub fn run_drill(args: &[String]) -> Result<String, CliError> {
    if args.first().map(String::as_str) == Some("phase15") {
        return Ok(phase15_authority_drill_report());
    }

    if args.first().map(String::as_str) == Some("phase14") {
        return Ok(phase14_incident_drill_report());
    }

    let args = parse_args(args)?;

    if args.help {
        return Ok(drill_help());
    }

    let request = KillSwitchDrillRequest::new(
        args.stage,
        args.action,
        args.posture.posture(),
        args.authority.key(),
    );

    let review = review_kill_switch_drill(&authority_map(), &request);

    let mut lines = vec![
        "rox-anchor kill-switch drill".to_string(),
        "command: drill".to_string(),
        format!("stage: {}", args.stage.as_str()),
        format!("action: {}", args.action.as_str()),
        format!("posture_fixture: {}", args.posture.as_str()),
        format!("presented_authority: {}", args.authority.as_str()),
        format!("review_accepted: {}", review.is_accepted()),
    ];

    for line in review.render_lines() {
        lines.push(line);
    }

    lines.extend([
        "runtime: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "rpc_submission: disabled".to_string(),
        "network_submitted: false".to_string(),
        "mint_burn_execution: disabled".to_string(),
        "roc_release: disabled".to_string(),
        "settlement_claim: none".to_string(),
        "public_bridge_authorization: none".to_string(),
    ]);

    Ok(lines.join("\n"))
}

fn phase14_incident_drill_report() -> String {
    let decision = accepted_phase14_decision();

    let coordinator_drills = [
        (
            "halt_before_simulation",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterProofAcceptanceBeforeSimulation,
                decision.clone(),
                AnchorOperationalPosture::halted(),
            ),
        ),
        (
            "halt_after_simulation_before_submit",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
                decision.clone(),
                AnchorOperationalPosture::halted(),
            ),
        ),
        (
            "halt_after_capped_submit",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterCappedTestnetSubmission,
                decision.clone(),
                AnchorOperationalPosture::halted(),
            )
            .with_network_submitted(true)
            .with_readback_present(true),
        ),
        (
            "recovery_during_pending_operation",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
                decision.clone(),
                AnchorOperationalPosture::recovery_required(),
            ),
        ),
        (
            "operator_approval_omitted",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
                decision.clone(),
                AnchorOperationalPosture::clear(),
            )
            .with_operator_approval_present(false),
        ),
        (
            "wrong_authority_attempted",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
                decision.clone(),
                AnchorOperationalPosture::clear(),
            )
            .with_wrong_authority_attempted(true),
        ),
        (
            "readback_missing_after_send",
            CoordinatorIncidentDrillEvidence::new(
                CoordinatorIncidentStage::AfterCappedTestnetSubmission,
                decision,
                AnchorOperationalPosture::clear(),
            )
            .with_network_submitted(true)
            .with_readback_present(false),
        ),
    ];

    let relayer_drills = phase14_relayer_receipt_drills();
    let relayer_sender_drills = phase14_relayer_sender_drills();
    let rpc_drills = phase14_rpc_readback_drills();

    let total_drills = coordinator_drills.len()
        + relayer_drills.len()
        + relayer_sender_drills.len()
        + rpc_drills.len();

    let mut lines = vec![
        "rox-anchor phase14 incident drill".to_string(),
        "command: drill phase14".to_string(),
        "scope: live_testnet_chaos_and_incident_drills".to_string(),
        "source: rox-anchor-coordinator incident review".to_string(),
        "source: rox-anchor-relayer incident receipt review".to_string(),
        "source: rox-anchor-relayer capped sender authorization review".to_string(),
        "source: rox-anchor-rpc-proof readback review".to_string(),
        "mode: local_report_only".to_string(),
        "runtime: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "rpc_submission: disabled".to_string(),
        "signing: disabled".to_string(),
        "mint_burn_execution: disabled".to_string(),
        "internal_roc_mutation: disabled".to_string(),
        "settlement_claim: none".to_string(),
        "public_bridge_authorization: none".to_string(),
        format!("drill_count: {total_drills}"),
        format!("coordinator_drill_count: {}", coordinator_drills.len()),
        format!("relayer_receipt_drill_count: {}", relayer_drills.len()),
        format!(
            "relayer_sender_drill_count: {}",
            relayer_sender_drills.len()
        ),
        format!("rpc_readback_drill_count: {}", rpc_drills.len()),
    ];

    for (name, evidence) in coordinator_drills {
        let review = review_coordinator_incident_drill(evidence);
        lines.push(format!("coordinator_drill: {name}"));
        lines.push(format!("drill: {name}"));
        lines.extend(review.redacted_report_lines());
    }

    for (name, evidence) in relayer_drills {
        let review = review_pilot_incident_receipt(evidence);
        lines.push(format!("relayer_receipt_drill: {name}"));
        lines.push(format!("drill: {name}"));
        lines.extend(review.redacted_report_lines());
    }

    for (name, report_lines) in relayer_sender_drills {
        lines.push(format!("relayer_sender_drill: {name}"));
        lines.push(format!("drill: {name}"));
        lines.extend(report_lines);
    }

    for (name, review) in rpc_drills {
        lines.push(format!("rpc_readback_drill: {name}"));
        lines.push(format!("drill: {name}"));
        lines.extend(review.redacted_report_lines());
    }

    lines.push(
        "phase14_summary: incidents_fail_safe_without_runtime_finality_or_settlement_claims"
            .to_string(),
    );

    lines.join("\n")
}

fn phase14_relayer_receipt_drills() -> [(&'static str, PilotIncidentReceiptEvidence); 7] {
    [
        (
            "missing_receipt_file",
            phase14_receipt_evidence().with_receipt_file_present(false),
        ),
        (
            "receipt_tamper",
            phase14_receipt_evidence().with_receipt_chain_valid(false),
        ),
        (
            "duplicate_receipt",
            phase14_receipt_evidence().with_duplicate_receipt(true),
        ),
        (
            "duplicate_operation_id",
            phase14_receipt_evidence().with_duplicate_operation_id(true),
        ),
        (
            "duplicate_idempotency_key",
            phase14_receipt_evidence().with_duplicate_idempotency_key(true),
        ),
        (
            "nonce_reuse",
            phase14_receipt_evidence().with_nonce_reused(true),
        ),
        (
            "receipt_readback_missing_after_send",
            phase14_receipt_evidence()
                .with_network_submitted(true)
                .with_readback_present(false),
        ),
    ]
}

fn phase14_receipt_evidence() -> PilotIncidentReceiptEvidence {
    let package = fixtures::valid_package();

    PilotIncidentReceiptEvidence::new(package.operation_id, package.idempotency_key, package.nonce)
        .with_receipt_id(
            PilotReceiptId::new("phase14-cli-receipt-0001")
                .expect("static receipt id should validate"),
        )
}

fn phase14_relayer_sender_drills() -> [(&'static str, Vec<String>); 2] {
    let disabled_simulation = phase14_sender_simulation();
    let disabled_authorization = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(disabled_simulation.clone())
            .with_external_config(phase14_sender_external_config(SubmissionMode::SimulateOnly))
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true)
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10),
    );

    let cap_exceeded_simulation = phase14_sender_simulation();
    let cap_exceeded_authorization = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(cap_exceeded_simulation.clone())
            .with_external_config(phase14_sender_external_config(
                SubmissionMode::TestnetSubmitCapped,
            ))
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true)
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(1_000)
            .with_limits(CappedTestnetSubmissionLimits::new(2, 2, 100, true)),
    );

    [
        (
            "simulation_passes_but_send_disabled",
            phase14_sender_drill_lines(
                "simulation_passes_but_send_disabled",
                &disabled_simulation,
                &disabled_authorization,
            ),
        ),
        (
            "send_enabled_but_cap_exceeded",
            phase14_sender_drill_lines(
                "send_enabled_but_cap_exceeded",
                &cap_exceeded_simulation,
                &cap_exceeded_authorization,
            ),
        ),
    ]
}

fn phase14_sender_drill_lines(
    scenario: &str,
    simulation: &PrivatePilotSimulationResult,
    authorization: &PrivateTestnetSenderAuthorization,
) -> Vec<String> {
    let mut lines = vec![
        "phase14_sender_authorization_review: local_only".to_string(),
        format!("scenario: {scenario}"),
        format!("simulation_status: {:?}", simulation.status),
        format!("simulation_passed: {}", simulation.is_simulated()),
        format!(
            "simulation_read_only_rpc_verified: {}",
            simulation.read_only_rpc_verified
        ),
        format!("simulation_live_submission: {}", simulation.live_submission),
    ];

    lines.extend(authorization.redacted_report_lines());
    lines
}

fn phase14_sender_simulation() -> PrivatePilotSimulationResult {
    let package = fixtures::valid_package();
    let review = review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    );

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "phase14-cli-sender-target",
            review,
        ))
        .expect("static phase14 sender fixture should produce dry-run receipt");

    let base_plan = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base_plan)
        .with_read_only_rpc_verified(true)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(PrivatePilotTransactionKind::Observe, "observe", 1),
            PrivatePilotTransactionStep::new(PrivatePilotTransactionKind::Finalize, "finalize", 1),
        ]);

    simulate_private_pilot_transaction_plan(RelayerConfig::new(3, 16), plan)
}

fn phase14_sender_external_config(submission_mode: SubmissionMode) -> RelayerPrivatePilotConfig {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        submission_mode,
    );

    let relayer = RelayerConfig::new_with_safety(2, 16, safety);
    let mode_label = match submission_mode {
        SubmissionMode::DryRunOnly => "dry-run-only",
        SubmissionMode::SimulateOnly => "simulate-only",
        SubmissionMode::TestnetSubmitCapped => "testnet-submit-capped",
    };

    let text = format!(
        r#"
environment_mode=testnet-only
cluster=devnet
submission_mode={mode_label}
rpc_url=https://api.devnet.solana.com/phase14-redacted-token
payer_keypair_path=/external/private-pilot/payer.json
operator_label=phase14-private-pilot-operator
asset_label=test-only-rox-asset
receipt_output_path=/external/private-pilot/receipts/phase14.json
observed_signature=phase14sendersignature11111111111111111111111111
"#
    );

    RelayerPrivatePilotConfig::from_external_config_text(relayer, &text)
        .expect("static phase14 sender external config should validate")
}

fn phase14_rpc_readback_drills(
) -> [(&'static str, rox_anchor_rpc_proof::ReadbackAfterSendReview); 3] {
    [
        (
            "rpc_disagreement_during_readback",
            review_readback_after_send(true, &phase14_rpc_disputed_readback_review()),
        ),
        (
            "rpc_stale_readback_after_send",
            review_readback_after_send(true, &phase14_rpc_stale_readback_review()),
        ),
        (
            "rpc_readback_missing_after_send",
            review_readback_after_send(true, &phase14_rpc_missing_readback_review()),
        ),
    ]
}

fn phase14_rpc_missing_readback_review() -> ReadOnlyRpcObservationReview {
    let expected = phase14_expected_rpc_binding();
    let quorum = review_rpc_observations(&[], &expected, RpcProofConfig::new(2, 100), 500);

    ReadOnlyRpcObservationReview {
        current_slot: 500,
        observations_checked: 0,
        quorum,
    }
}

fn phase14_rpc_disputed_readback_review() -> ReadOnlyRpcObservationReview {
    let expected = phase14_expected_rpc_binding();
    let observations = vec![
        phase14_rpc_observation("phase14-rpc-a", "phase14-sig-cli-a-111111111111", 450),
        phase14_rpc_observation("phase14-rpc-b", "phase14-sig-cli-b-222222222222", 451),
    ];

    let observations_checked = observations.len().min(u16::MAX as usize) as u16;
    let quorum =
        review_rpc_observations(&observations, &expected, RpcProofConfig::new(2, 100), 500);

    ReadOnlyRpcObservationReview {
        current_slot: 500,
        observations_checked,
        quorum,
    }
}

fn phase14_rpc_stale_readback_review() -> ReadOnlyRpcObservationReview {
    let expected = phase14_expected_rpc_binding();
    let observations = vec![
        phase14_rpc_observation("phase14-rpc-a", "phase14-sig-cli-stale-111111111111", 100),
        phase14_rpc_observation("phase14-rpc-b", "phase14-sig-cli-stale-111111111111", 101),
    ];

    let observations_checked = observations.len().min(u16::MAX as usize) as u16;
    let quorum = review_rpc_observations(&observations, &expected, RpcProofConfig::new(2, 5), 500);

    ReadOnlyRpcObservationReview {
        current_slot: 500,
        observations_checked,
        quorum,
    }
}

fn accepted_phase14_decision() -> rox_anchor_coordinator::CoordinatorDecision {
    let package = fixtures::valid_package();
    let expected = fixtures::expected_proof_binding();
    let expected_rpc = phase14_expected_rpc_binding();

    let request = CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        vec![
            phase14_rpc_observation("phase14-rpc-a", "phase14-sig-cli-111111111111", 450),
            phase14_rpc_observation("phase14-rpc-b", "phase14-sig-cli-111111111111", 451),
        ],
        ReplaySet::default(),
    );

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

    debug_assert!(
        decision.is_accepted(),
        "static phase14 CLI fixture should be accepted before incident drill blockers are applied"
    );

    decision
}

fn phase14_expected_rpc_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        ClusterId::new("localnet").expect("static cluster should validate"),
        ProgramId::new("RoxAnchorProgram111111111111111111111111")
            .expect("static program id should validate"),
        MintId::new("RoxMint111111111111111111111111111111111")
            .expect("static mint id should validate"),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111")
            .expect("static token account id should validate"),
        OperationId::new("op-roc-to-rox-0001").expect("static operation id should validate"),
        RpcCommitmentLevel::Confirmed,
    )
}

fn phase14_rpc_observation(source: &str, signature: &str, slot: u64) -> RpcObservation {
    RpcObservation::new(
        source,
        ClusterId::new("localnet").expect("static cluster should validate"),
        ProgramId::new("RoxAnchorProgram111111111111111111111111")
            .expect("static program id should validate"),
        MintId::new("RoxMint111111111111111111111111111111111")
            .expect("static mint id should validate"),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111")
            .expect("static token account id should validate"),
        OperationId::new("op-roc-to-rox-0001").expect("static operation id should validate"),
        signature,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn phase15_authority_drill_report() -> String {
    let strict_authorities = phase15_strict_authorities();
    let shared_test_authorities = phase15_shared_test_only_authorities();

    let mut lines = vec![
        "rox-anchor phase15 authority drill".to_string(),
        "command: drill phase15".to_string(),
        "scope: authority_upgrade_halt_recovery_operational_drills".to_string(),
        "source: rox-anchor-core authority map review".to_string(),
        "source: rox-anchor-core kill-switch review".to_string(),
        "mode: local_report_only".to_string(),
        "runtime: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "private_key_material: not_loaded".to_string(),
        "rpc_submission: disabled".to_string(),
        "signing: disabled".to_string(),
        "mint_burn_execution: disabled".to_string(),
        "internal_roc_mutation: disabled".to_string(),
        "production_safety_claim: none".to_string(),
        "settlement_claim: none".to_string(),
        "public_bridge_authorization: none".to_string(),
        "drill_count: 8".to_string(),
        format!(
            "strict_authority_separation_valid: {}",
            strict_authorities.validate_critical_authorities().is_ok()
        ),
        format!(
            "explicit_test_only_shared_authority_valid: {}",
            shared_test_authorities
                .validate_critical_authorities()
                .is_ok()
        ),
    ];

    lines.extend(phase15_role_checklist_lines());
    lines.extend(phase15_wrong_authority_drill_lines());
    lines.extend(phase15_halt_status_drill_lines());
    lines.extend(phase15_recovery_from_halt_drill_lines());
    lines.extend(phase15_key_rotation_intent_lines());

    lines.push(
        "phase15_summary: authority_drills_are_local_redacted_and_operator_readable".to_string(),
    );

    lines.join("\n")
}

fn phase15_role_checklist_lines() -> Vec<String> {
    let mut lines = vec![
        "phase15_authority_checklist: local_only".to_string(),
        "upgrade_authority_checklist: external_key_required".to_string(),
        "mint_authority_checklist: separated_or_explicit_test_only".to_string(),
        "halt_authority_checklist: can_block_acceptance_simulation_submission_finalization"
            .to_string(),
        "recovery_authority_checklist: can_only_recover_from_halted_recovery_required_state"
            .to_string(),
    ];

    for role in OperatorRole::ALL {
        lines.push(format!(
            "operator_role: {} critical={}",
            role.as_str(),
            role.is_critical_authority()
        ));
    }

    lines
}

fn phase15_wrong_authority_drill_lines() -> Vec<String> {
    let request = KillSwitchDrillRequest::new(
        KillSwitchDrillStage::AfterCappedTestnetSubmission,
        KillSwitchAction::Halt,
        AnchorOperationalPosture::clear(),
        key("phase15-wrong-authority-key-0001"),
    );

    let review = review_kill_switch_drill(&phase15_strict_authorities(), &request);

    vec![
        "phase15_wrong_authority_rejection_drill: local_only".to_string(),
        format!("status: {:?}", review.status),
        format!("accepted: {}", review.is_accepted()),
        format!("action_permitted: {}", review.action_permitted),
        format!("blocks_submission: {}", review.blocks_submission),
        "private_key_material: not_loaded".to_string(),
        "transaction_submission: not_performed".to_string(),
        "settlement_claim: none".to_string(),
    ]
}

fn phase15_halt_status_drill_lines() -> Vec<String> {
    let request = KillSwitchDrillRequest::new(
        KillSwitchDrillStage::AfterSimulationBeforeSubmission,
        KillSwitchAction::Halt,
        AnchorOperationalPosture::clear(),
        key("phase15-halt-authority-key-0003"),
    );

    let review = review_kill_switch_drill(&phase15_strict_authorities(), &request);

    vec![
        "phase15_halted_system_read_only_status_drill: local_only".to_string(),
        format!("status: {:?}", review.status),
        format!("accepted: {}", review.is_accepted()),
        format!("action_permitted: {}", review.action_permitted),
        format!("blocks_acceptance: {}", review.blocks_acceptance),
        format!("blocks_simulation: {}", review.blocks_simulation),
        format!("blocks_submission: {}", review.blocks_submission),
        format!("blocks_finalization: {}", review.blocks_finalization),
        "read_only_status: available".to_string(),
        "rpc_submission: disabled".to_string(),
        "settlement_claim: none".to_string(),
    ]
}

fn phase15_recovery_from_halt_drill_lines() -> Vec<String> {
    let request = KillSwitchDrillRequest::new(
        KillSwitchDrillStage::AfterCappedTestnetSubmission,
        KillSwitchAction::Recover,
        AnchorOperationalPosture::halted_recovery_required(),
        key("phase15-recovery-authority-key-0004"),
    );

    let review = review_kill_switch_drill(&phase15_strict_authorities(), &request);

    vec![
        "phase15_recovery_from_halt_drill: local_only".to_string(),
        format!("status: {:?}", review.status),
        format!("accepted: {}", review.is_accepted()),
        format!("action_permitted: {}", review.action_permitted),
        format!("blocks_acceptance: {}", review.blocks_acceptance),
        format!("blocks_simulation: {}", review.blocks_simulation),
        format!("blocks_submission: {}", review.blocks_submission),
        format!("blocks_finalization: {}", review.blocks_finalization),
        "post_recovery_runtime_claim: none".to_string(),
        "transaction_submission: not_performed".to_string(),
        "settlement_claim: none".to_string(),
    ]
}

fn phase15_key_rotation_intent_lines() -> Vec<String> {
    vec![
        "phase15_key_rotation_intent_drill: local_only".to_string(),
        "rotation_scope: intent_only".to_string(),
        "old_upgrade_authority: redacted_authority_key".to_string(),
        "new_upgrade_authority: redacted_authority_key".to_string(),
        "old_mint_authority: redacted_authority_key".to_string(),
        "new_mint_authority: redacted_authority_key".to_string(),
        "old_halt_authority: redacted_authority_key".to_string(),
        "new_halt_authority: redacted_authority_key".to_string(),
        "old_recovery_authority: redacted_authority_key".to_string(),
        "new_recovery_authority: redacted_authority_key".to_string(),
        "private_key_material: not_loaded".to_string(),
        "keypair_path: redacted".to_string(),
        "signing: disabled".to_string(),
        "upgrade_execution: not_performed".to_string(),
        "production_safety_claim: none".to_string(),
    ]
}

fn phase15_strict_authorities() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(
                OperatorRole::UpgradeAuthority,
                key("phase15-upgrade-authority-key-0001"),
            ),
            AuthorityAssignment::new(
                OperatorRole::MintAuthority,
                key("phase15-mint-authority-key-0002"),
            ),
            AuthorityAssignment::new(
                OperatorRole::HaltAuthority,
                key("phase15-halt-authority-key-0003"),
            ),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("phase15-recovery-authority-key-0004"),
            ),
            AuthorityAssignment::new(
                OperatorRole::Observer,
                key("phase15-observer-authority-key-0005"),
            ),
            AuthorityAssignment::new(
                OperatorRole::Coordinator,
                key("phase15-coordinator-authority-key-0006"),
            ),
            AuthorityAssignment::new(
                OperatorRole::Relayer,
                key("phase15-relayer-authority-key-0007"),
            ),
        ],
    )
}

fn phase15_shared_test_only_authorities() -> AuthorityMap {
    let shared = key("phase15-shared-test-only-authority-key-0001");

    AuthorityMap::new(
        AuthoritySeparationMode::ExplicitTestOnlyShared,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::MintAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::RecoveryAuthority, shared),
        ],
    )
}

fn parse_args(args: &[String]) -> Result<DrillArgs, CliError> {
    let mut parsed = DrillArgs::default();
    let mut index = 0;

    while index < args.len() {
        let arg = args[index].as_str();

        match arg {
            "--help" | "-h" => {
                parsed.help = true;
                index += 1;
            }
            "--stage" => {
                parsed.stage = parse_stage(required_value(args, index, "--stage")?)?;
                index += 2;
            }
            "--action" => {
                parsed.action = parse_action(required_value(args, index, "--action")?)?;
                index += 2;
            }
            "--posture" => {
                parsed.posture = DrillPosture::parse(required_value(args, index, "--posture")?)?;
                index += 2;
            }
            "--authority" => {
                parsed.authority =
                    DrillAuthority::parse(required_value(args, index, "--authority")?)?;
                index += 2;
            }
            _ => {
                if let Some(value) = arg.strip_prefix("--stage=") {
                    parsed.stage = parse_stage(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--action=") {
                    parsed.action = parse_action(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--posture=") {
                    parsed.posture = DrillPosture::parse(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--authority=") {
                    parsed.authority = DrillAuthority::parse(value)?;
                    index += 1;
                } else {
                    return Err(CliError::UnknownDrillFlag(arg.to_owned()));
                }
            }
        }
    }

    Ok(parsed)
}

fn required_value<'a>(
    args: &'a [String],
    index: usize,
    flag: &'static str,
) -> Result<&'a str, CliError> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| CliError::UnknownDrillFlag(format!("{flag} requires a value")))
}

fn parse_stage(value: &str) -> Result<KillSwitchDrillStage, CliError> {
    match value {
        "before-proof" | "before-proof-acceptance" | "before_proof_acceptance" => {
            Ok(KillSwitchDrillStage::BeforeProofAcceptance)
        }
        "after-proof"
        | "after-proof-before-simulation"
        | "after_proof_acceptance_before_simulation" => {
            Ok(KillSwitchDrillStage::AfterProofAcceptanceBeforeSimulation)
        }
        "after-simulation"
        | "after-simulation-before-submission"
        | "after_simulation_before_submission" => {
            Ok(KillSwitchDrillStage::AfterSimulationBeforeSubmission)
        }
        "after-submit" | "after-capped-submit" | "after_capped_testnet_submission" => {
            Ok(KillSwitchDrillStage::AfterCappedTestnetSubmission)
        }
        other => Err(CliError::UnknownDrillFlag(format!(
            "--stage value `{other}`; expected before-proof, after-proof, after-simulation, or after-submit"
        ))),
    }
}

fn parse_action(value: &str) -> Result<KillSwitchAction, CliError> {
    match value {
        "halt" => Ok(KillSwitchAction::Halt),
        "recover" | "recovery" => Ok(KillSwitchAction::Recover),
        other => Err(CliError::UnknownDrillFlag(format!(
            "--action value `{other}`; expected halt or recover"
        ))),
    }
}

fn authority_map() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(
                OperatorRole::UpgradeAuthority,
                key("upgrade-authority-phase12-cli-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::MintAuthority,
                key("mint-authority-phase12-cli-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::HaltAuthority,
                key("halt-authority-phase12-cli-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-authority-phase12-cli-key"),
            ),
        ],
    )
}

fn key(value: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(value).expect("static CLI authority id should validate")
}

fn drill_help() -> String {
    [
        "rox-anchor kill-switch drill",
        "",
        "usage:",
        "  rox-anchor drill [--stage <name>] [--action <halt|recover>] [--posture <name>] [--authority <name>]",
        "  rox-anchor drill phase14",
        "  rox-anchor drill phase15",
        "",
        "stages:",
        "  before-proof",
        "  after-proof",
        "  after-simulation",
        "  after-submit",
        "",
        "actions:",
        "  halt",
        "  recover",
        "",
        "postures:",
        "  clear",
        "  challenge-open",
        "  challenge-accepted",
        "  halted",
        "  recovery-required",
        "  recovery-in-review",
        "  halted-recovery-required",
        "  recovery-resolved",
        "",
        "authorities:",
        "  halt",
        "  recovery",
        "  upgrade",
        "  wrong",
        "",
        "security:",
        "  local report only",
        "  no RPC submission",
        "  no wallet/key loading",
        "  no mint/burn execution",
        "  no ROC release",
        "  no settlement or finality claim",
        "  phase14 incident drills use coordinator, relayer, and rpc-proof incident review",
        "  phase15 authority drills use core authority and kill-switch review",
    ]
    .join("\n")
}
