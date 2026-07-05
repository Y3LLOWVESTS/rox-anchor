use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, EvidenceBundle, ReplaySet};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, CappedTestnetSubmissionStatus, RelayerConfig, RelayerDryRun,
    RelayerReceiptStatus, RelayerSubmissionRequest, TestnetRelayerAuditRecord,
    TransactionSimulationPlan, TransactionSimulationStatus,
};

fn capped_testnet_config() -> RelayerConfig {
    RelayerConfig::new_with_safety(
        2,
        8,
        AnchorSafetyProfile::new(
            AnchorEnvironmentMode::TestnetOnly,
            AnchorCluster::Testnet,
            ClusterAllowlist::testnet_experiments(),
            SubmissionMode::TestnetSubmitCapped,
        ),
    )
}

fn simulation_config() -> RelayerConfig {
    RelayerConfig::new(2, 8)
}

fn limits() -> CappedTestnetSubmissionLimits {
    CappedTestnetSubmissionLimits::new(2, 2, 100, true)
}

fn accepted_review() -> rox_anchor_proof::ProofReview {
    review_proof_package(
        &fixtures::valid_package(),
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn blocked_review() -> rox_anchor_proof::ProofReview {
    let mut package = fixtures::valid_package();
    package.evidence = EvidenceBundle::new(0, 2, 0);

    review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn accepted_pipeline() -> TestnetRelayerAuditRecord {
    let package = fixtures::valid_package();
    let mut relayer = RelayerDryRun::new(simulation_config());

    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "audit-testnet-shadow-target",
            accepted_review(),
        ))
        .expect("accepted receipt should fit capacity");

    let simulation = simulate_transaction_plan(
        simulation_config(),
        TransactionSimulationPlan::from_dry_run_receipt(receipt.clone(), true, 1),
    );

    let capped = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(true),
    );

    TestnetRelayerAuditRecord::from_pipeline(&receipt, &simulation, &capped, true)
}

#[test]
fn accepted_pipeline_renders_deterministic_safe_audit_record() {
    let audit = accepted_pipeline();

    assert_eq!(audit.version, "relayer-testnet-audit-v1");
    assert_eq!(audit.target, "audit-testnet-shadow-target");
    assert_eq!(audit.relayer_status, "DryRunAccepted");
    assert_eq!(audit.proof_decision, "Accepted");
    assert_eq!(audit.simulation_status, "Simulated");
    assert_eq!(audit.capped_submission_status, "Authorized");
    assert_eq!(audit.attempts_used, 1);
    assert_eq!(audit.instruction_count, 1);
    assert_eq!(audit.requested_attempts, 1);
    assert_eq!(audit.requested_operations, 1);
    assert_eq!(audit.amount_units, 10);
    assert!(audit.receipt_persisted);
    assert!(audit.authorized);
    assert!(audit.live_submission_permitted);
    assert!(!audit.live_submission_attempted);
    assert!(!audit.network_submitted);
    assert!(audit.pipeline_consistent);
    assert!(audit.is_safe_for_display());

    let expected = [
        format!("audit_record={}", audit.version),
        format!("operation_id={}", audit.operation_id),
        format!("idempotency_key={}", audit.idempotency_key),
        "target=audit-testnet-shadow-target".to_owned(),
        "relayer_status=DryRunAccepted".to_owned(),
        "proof_decision=Accepted".to_owned(),
        "attempts_used=1".to_owned(),
        "simulation_status=Simulated".to_owned(),
        "instruction_count=1".to_owned(),
        "capped_submission_status=Authorized".to_owned(),
        "requested_attempts=1".to_owned(),
        "requested_operations=1".to_owned(),
        "amount_units=10".to_owned(),
        "receipt_persisted=true".to_owned(),
        "authorized=true".to_owned(),
        "live_submission_permitted=true".to_owned(),
        "live_submission_attempted=false".to_owned(),
        "network_submitted=false".to_owned(),
        "pipeline_consistent=true".to_owned(),
        "display_safe=true".to_owned(),
    ]
    .join("\n");

    assert_eq!(audit.render(), expected);
}

#[test]
fn blocked_pipeline_is_auditable_but_not_authorized() {
    let package = fixtures::valid_package();
    let mut relayer = RelayerDryRun::new(simulation_config());

    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "audit-blocked-shadow-target",
            blocked_review(),
        ))
        .expect("blocked receipt should fit capacity");

    let simulation = simulate_transaction_plan(
        simulation_config(),
        TransactionSimulationPlan::from_dry_run_receipt(receipt.clone(), false, 1),
    );

    let capped = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(true),
    );

    let audit = TestnetRelayerAuditRecord::from_pipeline(&receipt, &simulation, &capped, true);
    let report = audit.render();

    assert_eq!(receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(
        simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert_eq!(
        capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_eq!(audit.relayer_status, "ProofBlocked");
    assert_eq!(audit.capped_submission_status, "SimulationNotAccepted");
    assert!(!audit.authorized);
    assert!(!audit.live_submission_permitted);
    assert!(!audit.live_submission_attempted);
    assert!(!audit.network_submitted);
    assert!(audit.pipeline_consistent);
    assert!(audit.is_safe_for_display());
    assert!(report.contains("authorized=false"));
    assert!(report.contains("display_safe=true"));
}

#[test]
fn audit_record_exposes_pipeline_inconsistency_without_hiding_it() {
    let package = fixtures::valid_package();
    let mut relayer = RelayerDryRun::new(simulation_config());

    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "audit-consistent-target",
            accepted_review(),
        ))
        .expect("accepted receipt should fit capacity");

    let simulation = simulate_transaction_plan(
        simulation_config(),
        TransactionSimulationPlan::from_dry_run_receipt(receipt.clone(), true, 1),
    );

    let mut capped = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(true),
    );
    capped.target = "audit-tampered-target".to_owned();

    let audit = TestnetRelayerAuditRecord::from_pipeline(&receipt, &simulation, &capped, true);
    let report = audit.render();

    assert!(!audit.pipeline_consistent);
    assert!(!audit.is_safe_for_display());
    assert!(report.contains("target=audit-tampered-target"));
    assert!(report.contains("pipeline_consistent=false"));
    assert!(report.contains("display_safe=false"));
}

#[test]
fn audit_record_marks_sensitive_looking_targets_unsafe_for_display() {
    let mut audit = accepted_pipeline();

    audit.target = "local-secret-keypair-wallet-path".to_owned();

    assert!(audit.pipeline_consistent);
    assert!(!audit.is_safe_for_display());

    let report = audit.render();

    assert!(report.contains("target=local-secret-keypair-wallet-path"));
    assert!(report.contains("display_safe=false"));
}
