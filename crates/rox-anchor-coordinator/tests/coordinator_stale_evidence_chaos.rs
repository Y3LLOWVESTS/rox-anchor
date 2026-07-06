// RO:WHAT — Chaos test for stale RPC evidence through coordinator and relayer dry-run boundaries.
// RO:WHY — Proves stale observations cannot become accepted coordinator decisions or relayer attempts.
// RO:INTERACTS — rox-anchor-rpc-proof quorum review, rox-anchor-coordinator decision, rox-anchor-relayer dry-run.
// RO:INVARIANTS — stale evidence is rejected deterministically; fresh evidence with the same binding can still pass.
// RO:SECURITY — local model only; no live RPC, wallet calls, transaction submission, minting, burning, settlement, staking, liquidity, or deployment.
// RO:TEST — cargo test -p rox-anchor-coordinator --test coordinator_stale_evidence_chaos.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorIncidentDrillEvidence, CoordinatorIncidentStage, CoordinatorIncidentStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_core::AnchorOperationalPosture;
use rox_anchor_proof::{fixtures, ExpectedProofBinding, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision, RpcQuorumFindingCode,
};

fn expected_rpc_binding(expected: &ExpectedProofBinding) -> ExpectedRpcBinding {
    let binding = expected.binding.clone();

    ExpectedRpcBinding::new(
        binding.cluster,
        binding.program_id,
        binding.mint,
        binding.token_account,
        expected.operation_id.clone(),
        RpcCommitmentLevel::Finalized,
    )
}

fn observations_at_slot(expected: &ExpectedRpcBinding, slot: u64) -> Vec<RpcObservation> {
    vec![
        RpcObservation::new(
            "rpc-a",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "sig-same-stale-chaos-0001",
            slot,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "rpc-b",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "sig-same-stale-chaos-0001",
            slot,
            RpcCommitmentLevel::Finalized,
        ),
    ]
}

fn request_at_slot(slot: u64) -> CoordinatorReviewRequest {
    let package = fixtures::valid_package();
    let expected = package.expected_binding_snapshot();
    let expected_rpc = expected_rpc_binding(&expected);
    let observations = observations_at_slot(&expected_rpc, slot);

    CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        observations,
        ReplaySet::default(),
    )
}

#[test]
fn stale_rpc_evidence_is_rejected_before_relayer_attempts() {
    let request = request_at_slot(10);
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

    assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
    assert!(decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::StaleEvidence));
    assert!(!decision.is_accepted());

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                request.package.operation_id.clone(),
                request.package.idempotency_key.clone(),
                "local-anchor-stale-chaos-dry-run",
                decision.proof_review,
            )
            .with_requested_attempts(3),
        )
        .unwrap();

    assert_ne!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.attempts_used, 0);
    assert!(!receipt.live_submission);
    assert_eq!(relayer.receipts().len(), 1);
}

#[test]
fn repeated_stale_evidence_reviews_are_deterministic() {
    let mut first_snapshot: Option<String> = None;

    for _attempt in 0..32 {
        let request = request_at_slot(10);
        let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

        assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
        assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
        assert!(decision
            .rpc_review
            .has_finding(RpcQuorumFindingCode::StaleEvidence));
        assert!(!decision.is_accepted());

        let snapshot = format!(
            "status={:?};rpc={:?};findings={:?};proof={:?};accepted={}",
            decision.status,
            decision.rpc_review.decision,
            decision.rpc_review.findings,
            decision.proof_review.decision,
            decision.is_accepted()
        );

        if let Some(previous) = &first_snapshot {
            assert_eq!(&snapshot, previous);
        } else {
            first_snapshot = Some(snapshot);
        }
    }
}

#[test]
fn fresh_rpc_evidence_after_stale_case_can_still_accept() {
    let request = request_at_slot(450);
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);
    assert!(decision.is_accepted());

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                request.package.operation_id.clone(),
                request.package.idempotency_key.clone(),
                "local-anchor-fresh-after-stale-chaos-dry-run",
                decision.proof_review,
            )
            .with_requested_attempts(2),
        )
        .unwrap();

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.proof_decision, ReviewDecision::Accepted);
    assert_eq!(receipt.attempts_used, 2);
    assert!(!receipt.live_submission);
}

fn accepted_phase14_decision() -> rox_anchor_coordinator::CoordinatorDecision {
    let request = request_at_slot(450);
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);
    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    decision
}

#[test]
fn phase14_coordinator_halt_before_simulation_fails_safe_with_inspectable_report() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterProofAcceptanceBeforeSimulation,
            decision,
            AnchorOperationalPosture::halted(),
        ),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::FinalizationBlocked
    );
    assert!(review.fail_safe);
    assert!(!review.permits_simulation);
    assert!(!review.permits_submission);
    assert!(!review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("phase14_coordinator_incident_drill: local_only"));
    assert!(report.contains("stage: after_proof_acceptance_before_simulation"));
    assert!(report.contains("status: FinalizationBlocked"));
    assert!(report.contains("finalization_gate_status: Halted"));
    assert!(report.contains("transaction_submission: not_performed_by_coordinator"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
    assert!(report.contains("mint_burn_execution: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
    assert!(report.contains("settlement_claim: none"));
}

#[test]
fn phase14_operator_approval_omitted_blocks_send_shaped_coordinator_stage() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
            decision,
            AnchorOperationalPosture::clear(),
        )
        .with_operator_approval_present(false),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::OperatorApprovalOmitted
    );
    assert!(review.fail_safe);
    assert!(review.permits_simulation);
    assert!(!review.permits_submission);
    assert!(review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("stage: after_simulation_before_submission"));
    assert!(report.contains("operator_approval_present: false"));
    assert!(report.contains("status: OperatorApprovalOmitted"));
    assert!(report.contains("permits_submission: false"));
    assert!(report.contains("public_bridge_authorization: none"));
}

#[test]
fn phase14_wrong_authority_attempt_is_coordinator_visible_and_never_runtime_success() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
            decision,
            AnchorOperationalPosture::clear(),
        )
        .with_wrong_authority_attempted(true),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::WrongAuthorityAttempted
    );
    assert!(review.fail_safe);
    assert!(!review.permits_simulation);
    assert!(!review.permits_submission);
    assert!(!review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("wrong_authority_attempted: true"));
    assert!(report.contains("status: WrongAuthorityAttempted"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
}

#[test]
fn phase14_coordinator_readback_missing_after_submit_fails_safe_without_finality_claim() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterCappedTestnetSubmission,
            decision,
            AnchorOperationalPosture::clear(),
        )
        .with_network_submitted(true)
        .with_readback_present(false),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::MissingReadbackAfterSend
    );
    assert!(review.fail_safe);
    assert!(review.permits_simulation);
    assert!(review.permits_submission);
    assert!(!review.permits_finalization);
    assert!(review.network_submitted);
    assert!(!review.readback_present);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("stage: after_capped_testnet_submission"));
    assert!(report.contains("network_submitted: true"));
    assert!(report.contains("readback_present: false"));
    assert!(report.contains("status: MissingReadbackAfterSend"));
    assert!(report.contains("finality_claim: false"));
    assert!(report.contains("settlement_claim: none"));

    for forbidden in [
        "settlement complete",
        "finality: confirmed",
        "mint complete",
        "burn complete",
        "access granted",
        "roc released",
        "loaded wallet",
        "loaded keypair",
    ] {
        assert!(
            !report.to_ascii_lowercase().contains(forbidden),
            "report must not contain unsafe wording: {forbidden}\n{report}"
        );
    }
}

#[test]
fn phase14_rejected_coordinator_decision_cannot_be_promoted_by_incident_report() {
    let request = request_at_slot(10);
    let rejected = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);
    assert_eq!(rejected.status, CoordinatorDecisionStatus::RejectedEvidence);

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
            rejected,
            AnchorOperationalPosture::clear(),
        ),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::CoordinatorNotAccepted
    );
    assert!(review.fail_safe);
    assert!(!review.permits_simulation);
    assert!(!review.permits_submission);
    assert!(!review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);
}

#[test]
fn phase14_halt_after_simulation_before_submit_blocks_submission_and_finalization_claims() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
            decision,
            AnchorOperationalPosture::halted(),
        ),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::FinalizationBlocked
    );
    assert!(review.fail_safe);
    assert!(!review.permits_submission);
    assert!(!review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("stage: after_simulation_before_submission"));
    assert!(report.contains("status: FinalizationBlocked"));
    assert!(report.contains("finalization_gate_status: Halted"));
    assert!(report.contains("permits_submission: false"));
    assert!(report.contains("finality_claim: false"));
    assert!(report.contains("settlement_claim: none"));
    assert!(report.contains("transaction_submission: not_performed_by_coordinator"));
    assert!(report.contains("mint_burn_execution: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
}

#[test]
fn phase14_halt_after_capped_submit_blocks_finalization_until_safe_readback_review() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterCappedTestnetSubmission,
            decision,
            AnchorOperationalPosture::halted(),
        )
        .with_network_submitted(true)
        .with_readback_present(true),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::FinalizationBlocked
    );
    assert!(review.fail_safe);
    assert!(review.network_submitted);
    assert!(review.readback_present);
    assert!(!review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("stage: after_capped_testnet_submission"));
    assert!(report.contains("network_submitted: true"));
    assert!(report.contains("readback_present: true"));
    assert!(report.contains("finalization_gate_status: Halted"));
    assert!(report.contains("permits_finalization: false"));
    assert!(report.contains("success_claim: false"));
    assert!(report.contains("finality_claim: false"));
    assert!(report.contains("settlement_claim: none"));
}

#[test]
fn phase14_recovery_during_pending_operation_blocks_submission_and_finalization() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
            decision,
            AnchorOperationalPosture::recovery_required(),
        ),
    );

    assert_eq!(
        review.status,
        CoordinatorIncidentStatus::FinalizationBlocked
    );
    assert!(review.fail_safe);
    assert!(!review.permits_submission);
    assert!(!review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("stage: after_simulation_before_submission"));
    assert!(report.contains("status: FinalizationBlocked"));
    assert!(report.contains("finalization_gate_status: RecoveryBlocked"));
    assert!(report.contains("permits_submission: false"));
    assert!(report.contains("permits_finalization: false"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
    assert!(report.contains("public_bridge_authorization: none"));
}

#[test]
fn phase14_clear_posture_incident_review_is_ready_but_still_makes_no_runtime_claims() {
    let decision = accepted_phase14_decision();

    let review = rox_anchor_coordinator::review_coordinator_incident_drill(
        CoordinatorIncidentDrillEvidence::new(
            CoordinatorIncidentStage::AfterSimulationBeforeSubmission,
            decision,
            AnchorOperationalPosture::clear(),
        ),
    );

    assert_eq!(review.status, CoordinatorIncidentStatus::Ready);
    assert!(review.is_ready());
    assert!(!review.fail_safe);
    assert!(review.permits_simulation);
    assert!(review.permits_submission);
    assert!(review.permits_finalization);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("status: Ready"));
    assert!(report.contains("success_claim: false"));
    assert!(report.contains("finality_claim: false"));
    assert!(report.contains("settlement_claim: none"));
    assert!(report.contains("transaction_submission: not_performed_by_coordinator"));
    assert!(report.contains("mint_burn_execution: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
}
