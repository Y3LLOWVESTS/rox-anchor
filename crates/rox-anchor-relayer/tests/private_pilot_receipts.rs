//! RO:WHAT — Tests BUILD_PLAN3 Phase 9 private-pilot receipt ledger behavior.
//! RO:WHY — Proves pilot receipt trails reject replay, mismatch, unsafe live claims, and settlement claims.
//! RO:INTERACTS — PilotReceiptLedger, PilotReceiptEntry, proof fixtures, dry-run, simulation, and capped authorization.
//! RO:INVARIANTS — receipt IDs are unique; operation IDs match; live submission cannot be claimed without send evidence.
//! RO:SECURITY — no live RPC, wallet loading, signing, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-relayer --test private_pilot_receipts.

#![forbid(unsafe_code)]

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, IdempotencyKey,
    OperationId, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, PilotReceiptEntry, PilotReceiptId, PilotReceiptKind,
    PilotReceiptLedger, PilotReceiptLedgerError, RelayerConfig, RelayerDryRun,
    RelayerSubmissionRequest, TransactionSimulationPlan,
};

fn accepted_review() -> rox_anchor_proof::ProofReview {
    review_proof_package(
        &fixtures::valid_package(),
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn submit_config() -> RelayerConfig {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    );

    RelayerConfig::new_with_safety(3, 16, safety)
}

fn accepted_pipeline() -> (
    PilotReceiptLedger,
    rox_anchor_core::OperationId,
    rox_anchor_core::IdempotencyKey,
) {
    let package = fixtures::valid_package();
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));

    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id.clone(),
            package.idempotency_key.clone(),
            "phase9-private-pilot-target",
            accepted_review(),
        ))
        .expect("accepted proof should fit receipt capacity");

    let simulation = simulate_transaction_plan(
        RelayerConfig::new(3, 16),
        TransactionSimulationPlan::from_dry_run_receipt(receipt.clone(), true, 2),
    );

    let capped = authorize_capped_testnet_submission(
        submit_config(),
        CappedTestnetSubmissionLimits::new(2, 2, 100, true),
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(true),
    );

    let mut ledger = PilotReceiptLedger::new(package.operation_id.clone());

    append(
        &mut ledger,
        "phase9-receipt-0001-proof",
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        PilotReceiptKind::ProofReview,
        ("proof_review", format!("{:?}", receipt.proof_decision)),
        receipt.target.clone(),
    )
    .expect("proof receipt should append");

    append(
        &mut ledger,
        "phase9-receipt-0002-simulation",
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        PilotReceiptKind::TransactionSimulation,
        ("transaction_simulation", format!("{:?}", simulation.status)),
        simulation.target.clone(),
    )
    .expect("simulation receipt should append");

    append(
        &mut ledger,
        "phase9-receipt-0003-authorization",
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        PilotReceiptKind::SendAuthorization,
        ("send_authorization", format!("{:?}", capped.status)),
        capped.target,
    )
    .expect("authorization receipt should append");

    (ledger, package.operation_id, package.idempotency_key)
}

fn append(
    ledger: &mut PilotReceiptLedger,
    receipt_id: &str,
    operation_id: OperationId,
    idempotency_key: IdempotencyKey,
    kind: PilotReceiptKind,
    labels: (&str, String),
    target: String,
) -> Result<(), PilotReceiptLedgerError> {
    let (stage_label, outcome_label) = labels;
    let entry = PilotReceiptEntry::new(
        PilotReceiptId::new(receipt_id)?,
        operation_id,
        idempotency_key,
        kind,
        (stage_label, outcome_label),
        target,
        ledger.tip_link().to_string(),
    );

    ledger.append_entry(entry)
}

#[test]
fn pilot_receipt_ledger_records_deterministic_redacted_chain() {
    let (ledger, operation_id, _) = accepted_pipeline();
    let report = ledger.redacted_report();

    assert_eq!(ledger.entries().len(), 3);
    assert_eq!(ledger.expected_operation_id(), &operation_id);
    assert!(ledger.tip_link().starts_with("pilot-link-"));
    assert!(report.contains("pilot_receipt_ledger=pilot-receipt-ledger-v1"));
    assert!(report.contains("entry_count=3"));
    assert!(report.contains("kind=proof_review"));
    assert!(report.contains("kind=transaction_simulation"));
    assert!(report.contains("kind=send_authorization"));
    assert!(report.contains("outcome_label=Authorized"));
    assert!(report.contains("live_submission_default=false"));
    assert!(report.contains("production_settlement_claim=false"));
}

#[test]
fn duplicate_receipt_ids_are_rejected() {
    let (mut ledger, operation_id, idempotency_key) = accepted_pipeline();
    let duplicate = PilotReceiptEntry::new(
        PilotReceiptId::new("phase9-receipt-0003-authorization").unwrap(),
        operation_id,
        idempotency_key,
        PilotReceiptKind::ReadbackVerification,
        ("readback", "verified"),
        "phase9-private-pilot-target",
        ledger.tip_link().to_string(),
    );

    let error = ledger.append_entry(duplicate).unwrap_err();

    assert!(matches!(
        error,
        PilotReceiptLedgerError::DuplicateReceiptId { .. }
    ));
}

#[test]
fn mismatched_operation_ids_are_rejected() {
    let (mut ledger, _, idempotency_key) = accepted_pipeline();
    let wrong_operation_id = OperationId::new("phase9-wrong-operation").unwrap();
    let entry = PilotReceiptEntry::new(
        PilotReceiptId::new("phase9-receipt-0004-wrong-operation").unwrap(),
        wrong_operation_id,
        idempotency_key,
        PilotReceiptKind::ReadbackVerification,
        ("readback", "verified"),
        "phase9-private-pilot-target",
        ledger.tip_link().to_string(),
    );

    let error = ledger.append_entry(entry).unwrap_err();

    assert!(matches!(
        error,
        PilotReceiptLedgerError::OperationIdMismatch { .. }
    ));
}

#[test]
fn stale_prior_links_are_rejected() {
    let (mut ledger, operation_id, idempotency_key) = accepted_pipeline();
    let entry = PilotReceiptEntry::new(
        PilotReceiptId::new("phase9-receipt-0004-stale-link").unwrap(),
        operation_id,
        idempotency_key,
        PilotReceiptKind::ReadbackVerification,
        ("readback", "verified"),
        "phase9-private-pilot-target",
        "pilot-link-stale",
    );

    let error = ledger.append_entry(entry).unwrap_err();

    assert!(matches!(
        error,
        PilotReceiptLedgerError::ChainLinkMismatch { .. }
    ));
}

#[test]
fn live_submission_claim_without_send_is_rejected() {
    let (mut ledger, operation_id, idempotency_key) = accepted_pipeline();
    let entry = PilotReceiptEntry::new(
        PilotReceiptId::new("phase9-receipt-0004-false-live").unwrap(),
        operation_id,
        idempotency_key,
        PilotReceiptKind::TransactionSignature,
        ("transaction_signature", "signature-present"),
        "phase9-private-pilot-target",
        ledger.tip_link().to_string(),
    )
    .with_live_submission_claimed(true)
    .with_transaction_signature("5Jphase9PrivateSignature1111222233334444");

    let error = ledger.append_entry(entry).unwrap_err();

    assert!(matches!(
        error,
        PilotReceiptLedgerError::LiveSubmissionClaimWithoutSend { .. }
    ));
}

#[test]
fn production_settlement_claim_is_rejected() {
    let (mut ledger, operation_id, idempotency_key) = accepted_pipeline();
    let entry = PilotReceiptEntry::new(
        PilotReceiptId::new("phase9-receipt-0004-settlement").unwrap(),
        operation_id,
        idempotency_key,
        PilotReceiptKind::ReadbackVerification,
        ("readback", "verified"),
        "phase9-private-pilot-target",
        ledger.tip_link().to_string(),
    )
    .with_production_settlement_claimed(true);

    let error = ledger.append_entry(entry).unwrap_err();

    assert!(matches!(
        error,
        PilotReceiptLedgerError::ProductionSettlementClaim { .. }
    ));
}

#[test]
fn sensitive_paths_are_redacted_in_display_report() {
    let package = fixtures::valid_package();
    let mut ledger = PilotReceiptLedger::new(package.operation_id.clone());
    append(
        &mut ledger,
        "phase9-receipt-0001-sensitive",
        package.operation_id,
        package.idempotency_key,
        PilotReceiptKind::RpcQuorum,
        ("rpc_quorum", "verified".to_string()),
        "/external/private-pilot/keys/payer.json".to_string(),
    )
    .expect("sensitive-looking target should append but render redacted");

    let report = ledger.redacted_report();

    assert!(report.contains("target=[redacted-sensitive-value]"));
    assert!(!report.contains("/external/private-pilot/keys/payer.json"));
}
