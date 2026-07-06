//! RO:WHAT — Phase 14 testnet chaos drills for relayer pending-operation, receipt, replay, and capped-submit behavior.
//! RO:WHY — Proves halt/recovery, cap, receipt, replay, and missing-readback failures stop unsafe submission-shaped paths.
//! RO:INTERACTS — RelayerDryRun, simulation plans, capped submit authorization, receipt review, and core operational posture.
//! RO:INVARIANTS — halted pending operations cannot simulate or authorize; incident receipts fail safe; capped submit never submits.
//! RO:SECURITY — no RPC, wallet, key loading, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-relayer --test testnet_chaos_drills.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorOperationalPosture, AnchorSafetyProfile,
    ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, CappedTestnetSubmissionStatus, RelayerConfig, RelayerDryRun,
    RelayerReceiptStatus, RelayerSubmissionRequest, TransactionSimulationPlan,
    TransactionSimulationStatus,
};

fn accepted_review() -> rox_anchor_proof::ProofReview {
    review_proof_package(
        &fixtures::valid_package(),
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn request_with_posture(posture: AnchorOperationalPosture) -> RelayerSubmissionRequest {
    let package = fixtures::valid_package();

    RelayerSubmissionRequest::new(
        package.operation_id,
        package.idempotency_key,
        "phase13-chaos-relayer-target",
        accepted_review(),
    )
    .with_operational_posture(posture)
}

fn simulation_config() -> RelayerConfig {
    RelayerConfig::new(3, 16)
}

fn capped_testnet_config() -> RelayerConfig {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Testnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    );

    RelayerConfig::new_with_safety(3, 16, safety)
}

fn simulate_only_testnet_config() -> RelayerConfig {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Testnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );

    RelayerConfig::new_with_safety(3, 16, safety)
}

fn capped_limits() -> CappedTestnetSubmissionLimits {
    CappedTestnetSubmissionLimits::new(2, 2, 100, true)
}

fn accepted_simulation_result() -> rox_anchor_relayer::TransactionSimulationResult {
    let mut relayer = RelayerDryRun::new(simulation_config());
    let receipt = relayer
        .submit_dry_run(request_with_posture(AnchorOperationalPosture::clear()))
        .expect("accepted proof should produce a dry-run receipt");

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);

    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);
    let simulation = simulate_transaction_plan(simulation_config(), plan);

    assert_eq!(simulation.status, TransactionSimulationStatus::Simulated);
    simulation
}

fn approved_plan() -> CappedTestnetSubmissionPlan {
    CappedTestnetSubmissionPlan::from_simulation_result(accepted_simulation_result())
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true)
}

#[test]
fn halt_during_pending_operation_blocks_relayer_simulation_and_capped_submit() {
    let mut relayer = RelayerDryRun::new(simulation_config());
    let halted_receipt = relayer
        .submit_dry_run(request_with_posture(AnchorOperationalPosture::halted()))
        .expect("halted pending operation should produce refusal receipt");

    assert_eq!(halted_receipt.status, RelayerReceiptStatus::Halted);
    assert_eq!(halted_receipt.proof_decision, ReviewDecision::Accepted);
    assert_eq!(halted_receipt.attempts_used, 0);
    assert!(!halted_receipt.live_submission);

    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(halted_receipt, true, 2);
    let simulation = simulate_transaction_plan(simulation_config(), simulation_plan);

    assert_eq!(
        simulation.status,
        TransactionSimulationStatus::RelayerDryRunNotAccepted
    );
    assert!(!simulation.simulated);
    assert!(!simulation.live_submission);

    let capped_plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let capped =
        authorize_capped_testnet_submission(capped_testnet_config(), capped_limits(), capped_plan);

    assert_eq!(
        capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!capped.authorized);
    assert!(!capped.live_submission_permitted);
    assert!(!capped.live_submission_attempted);
    assert!(!capped.network_submitted);
}

#[test]
fn recovery_resolved_pending_operation_can_return_to_dry_run_without_submission() {
    let mut relayer = RelayerDryRun::new(simulation_config());
    let recovered_receipt = relayer
        .submit_dry_run(request_with_posture(
            AnchorOperationalPosture::recovery_resolved(),
        ))
        .expect("recovered pending operation should produce a dry-run receipt");

    assert_eq!(
        recovered_receipt.status,
        RelayerReceiptStatus::DryRunAccepted
    );
    assert_eq!(recovered_receipt.proof_decision, ReviewDecision::Accepted);
    assert_eq!(recovered_receipt.attempts_used, 1);
    assert!(!recovered_receipt.live_submission);

    let simulation_plan =
        TransactionSimulationPlan::from_dry_run_receipt(recovered_receipt, true, 2);
    let simulation = simulate_transaction_plan(simulation_config(), simulation_plan);

    assert_eq!(simulation.status, TransactionSimulationStatus::Simulated);
    assert!(simulation.simulated);
    assert!(!simulation.live_submission);
}

#[test]
fn simulation_passes_but_send_disabled_scope_refuses_capped_submit() {
    let plan = approved_plan();

    let result =
        authorize_capped_testnet_submission(simulate_only_testnet_config(), capped_limits(), plan);

    assert_eq!(result.status, CappedTestnetSubmissionStatus::UnsafeScope);
    assert_eq!(result.proof_decision, ReviewDecision::Accepted);
    assert_eq!(result.relayer_status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(
        result.simulation_status,
        TransactionSimulationStatus::Simulated
    );
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
}

#[test]
fn send_enabled_but_caps_stop_operation_and_amount_drills() {
    let operation_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        approved_plan().with_requested_operations(3),
    );

    assert_eq!(
        operation_result.status,
        CappedTestnetSubmissionStatus::OperationCapExceeded
    );
    assert!(!operation_result.authorized);
    assert!(!operation_result.network_submitted);

    let amount_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        approved_plan().with_amount_units(101),
    );

    assert_eq!(
        amount_result.status,
        CappedTestnetSubmissionStatus::AmountCapExceeded
    );
    assert!(!amount_result.authorized);
    assert!(!amount_result.network_submitted);
}

fn phase14_incident_evidence() -> rox_anchor_relayer::PilotIncidentReceiptEvidence {
    let package = fixtures::valid_package();

    rox_anchor_relayer::PilotIncidentReceiptEvidence::new(
        package.operation_id,
        package.idempotency_key,
        package.nonce,
    )
    .with_receipt_id(
        rox_anchor_relayer::PilotReceiptId::new("phase14-receipt-incident-0001")
            .expect("receipt id should validate"),
    )
}

#[test]
fn phase14_receipt_incidents_fail_safe_for_missing_tampered_and_duplicate_receipts() {
    use rox_anchor_relayer::{review_pilot_incident_receipt, PilotIncidentReceiptStatus};

    let missing =
        review_pilot_incident_receipt(phase14_incident_evidence().with_receipt_file_present(false));
    assert_eq!(
        missing.status,
        PilotIncidentReceiptStatus::MissingReceiptFile
    );
    assert!(missing.fail_safe);
    assert!(!missing.live_submission_claim);
    assert!(!missing.production_settlement_claim);

    let tampered =
        review_pilot_incident_receipt(phase14_incident_evidence().with_receipt_chain_valid(false));
    assert_eq!(tampered.status, PilotIncidentReceiptStatus::ReceiptTamper);
    assert!(tampered.fail_safe);

    let duplicate =
        review_pilot_incident_receipt(phase14_incident_evidence().with_duplicate_receipt(true));
    assert_eq!(
        duplicate.status,
        PilotIncidentReceiptStatus::DuplicateReceipt
    );
    assert!(duplicate.fail_safe);

    let report = duplicate.redacted_report_lines().join("\n");
    assert!(report.contains("phase14_incident_receipt_review: local_only"));
    assert!(report.contains("status: DuplicateReceipt"));
    assert!(report.contains("operator_action: halt_or_recover_before_retry"));
    assert!(report.contains("wallet_key_loading: false"));
    assert!(report.contains("signing: false"));
    assert!(report.contains("internal_roc_mutation: disabled"));
    assert!(report.contains("settlement_claim: none"));
}

#[test]
fn phase14_replay_incidents_fail_safe_for_duplicate_operation_id_idempotency_and_nonce() {
    use rox_anchor_relayer::{review_pilot_incident_receipt, PilotIncidentReceiptStatus};

    let duplicate_operation = review_pilot_incident_receipt(
        phase14_incident_evidence().with_duplicate_operation_id(true),
    );
    assert_eq!(
        duplicate_operation.status,
        PilotIncidentReceiptStatus::DuplicateOperationId
    );
    assert!(duplicate_operation.fail_safe);

    let duplicate_idempotency = review_pilot_incident_receipt(
        phase14_incident_evidence().with_duplicate_idempotency_key(true),
    );
    assert_eq!(
        duplicate_idempotency.status,
        PilotIncidentReceiptStatus::DuplicateIdempotencyKey
    );
    assert!(duplicate_idempotency.fail_safe);

    let nonce_reuse =
        review_pilot_incident_receipt(phase14_incident_evidence().with_nonce_reused(true));
    assert_eq!(nonce_reuse.status, PilotIncidentReceiptStatus::NonceReuse);
    assert!(nonce_reuse.fail_safe);

    for review in [duplicate_operation, duplicate_idempotency, nonce_reuse] {
        assert!(!review.live_submission_claim);
        assert!(!review.production_settlement_claim);
        assert!(review
            .redacted_report_lines()
            .join("\n")
            .contains("fail_safe: true"));
    }
}

#[test]
fn phase14_readback_missing_after_send_requires_fail_safe_recovery_without_settlement_claim() {
    use rox_anchor_relayer::{review_pilot_incident_receipt, PilotIncidentReceiptStatus};

    let review = review_pilot_incident_receipt(
        phase14_incident_evidence()
            .with_network_submitted(true)
            .with_readback_present(false),
    );

    assert_eq!(
        review.status,
        PilotIncidentReceiptStatus::MissingReadbackAfterSend
    );
    assert!(review.fail_safe);
    assert!(review.network_submitted);
    assert!(!review.readback_present);
    assert!(!review.live_submission_claim);
    assert!(!review.production_settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("network_submitted: true"));
    assert!(report.contains("readback_present: false"));
    assert!(report.contains("operator_action: halt_or_recover_before_retry"));
    assert!(report.contains("settlement_claim: none"));
}
