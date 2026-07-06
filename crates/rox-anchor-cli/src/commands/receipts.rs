//! RO:WHAT — CLI display for private-pilot receipt ledger reports.
//! RO:WHY — Exposes BUILD_PLAN3 Phase 9 receipt/audit trail behavior without sending transactions.
//! RO:INTERACTS — rox-anchor-proof fixtures and rox-anchor-relayer pilot receipt ledger.
//! RO:INVARIANTS — display is deterministic, redacted, local-only, and never claims settlement.
//! RO:SECURITY — no RPC, wallet/key loading, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — covered by private pilot receipt display tests.

use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, PilotReceiptEntry, PilotReceiptId, PilotReceiptKind,
    PilotReceiptLedger, RelayerConfig, RelayerDryRun, RelayerSubmissionRequest,
    TransactionSimulationPlan,
};

pub fn receipt_report() -> String {
    let package = fixtures::valid_package();
    let review = review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    );

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let dry_run_receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id.clone(),
            package.idempotency_key.clone(),
            "private-pilot-receipt-cli-target",
            review,
        ))
        .expect("static private-pilot receipt dry-run should fit receipt capacity");

    let simulation = simulate_transaction_plan(
        RelayerConfig::new(3, 16),
        TransactionSimulationPlan::from_dry_run_receipt(dry_run_receipt.clone(), true, 2),
    );

    let capped = authorize_capped_testnet_submission(
        private_testnet_submit_config(),
        CappedTestnetSubmissionLimits::new(2, 2, 100, true),
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(true),
    );

    let mut ledger = PilotReceiptLedger::new(package.operation_id.clone());

    append_entry(
        &mut ledger,
        "pilot-receipt-0001-proof-review",
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        PilotReceiptKind::ProofReview,
        (
            "proof_review",
            format!("{:?}", dry_run_receipt.proof_decision),
        ),
        dry_run_receipt.target.clone(),
    );

    append_entry(
        &mut ledger,
        "pilot-receipt-0002-simulation",
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        PilotReceiptKind::TransactionSimulation,
        ("transaction_simulation", format!("{:?}", simulation.status)),
        simulation.target.clone(),
    );

    append_entry(
        &mut ledger,
        "pilot-receipt-0003-send-authorization",
        package.operation_id,
        package.idempotency_key,
        PilotReceiptKind::SendAuthorization,
        ("send_authorization", format!("{:?}", capped.status)),
        capped.target,
    );

    [
        "command: receipts".to_string(),
        "scope: private_pilot_local_receipt_ledger".to_string(),
        "network_submission: disabled_in_cli_report".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "settlement_claim: none".to_string(),
        ledger.redacted_report(),
    ]
    .join("\n")
}

fn append_entry(
    ledger: &mut PilotReceiptLedger,
    receipt_id: &str,
    operation_id: rox_anchor_core::OperationId,
    idempotency_key: rox_anchor_core::IdempotencyKey,
    kind: PilotReceiptKind,
    labels: (&str, String),
    target: String,
) {
    let (stage_label, outcome_label) = labels;
    let entry = PilotReceiptEntry::new(
        PilotReceiptId::new(receipt_id).expect("static private-pilot receipt ID should validate"),
        operation_id,
        idempotency_key,
        kind,
        (stage_label, outcome_label),
        target,
        ledger.tip_link().to_string(),
    );

    ledger
        .append_entry(entry)
        .expect("static private-pilot receipt ledger entry should validate");
}

fn private_testnet_submit_config() -> RelayerConfig {
    use rox_anchor_core::{
        AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
    };

    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Testnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    );

    RelayerConfig::new_with_safety(3, 16, safety)
}
