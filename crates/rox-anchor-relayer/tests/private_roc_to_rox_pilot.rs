//! RO:WHAT — Tests BUILD_PLAN3 Phase 12 private ROC-to-ROX relayer pilot path.
//! RO:WHY — Proves forward pilot can dry-run, simulate, and authorize capped testnet only after gates pass.
//! RO:INTERACTS — proof review, internal ROC burn intent, relayer dry-run, private simulation, sender authorization.
//! RO:INVARIANTS — authorization requires accepted proof, read-only RPC gate, receipt, caps, and explicit approval.
//! RO:SECURITY — no real ROC burn, public mint, wallet loading, signing, transaction send, or settlement occurs.
//! RO:TEST — cargo test -p rox-anchor-relayer --test private_roc_to_rox_pilot.

#![forbid(unsafe_code)]

use rox_anchor_core::{
    AccountId, AnchorCluster, AnchorEnvironmentMode, AnchorOperationalPosture, AnchorSafetyProfile,
    ClusterAllowlist, InternalRocDryRunBurnIntent, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
use rox_anchor_relayer::{
    authorize_private_testnet_sender, simulate_private_pilot_transaction_plan,
    CappedTestnetSubmissionLimits, PrivatePilotSimulationPlan, PrivatePilotSimulationStatus,
    PrivatePilotTransactionKind, PrivatePilotTransactionStep, PrivateTestnetSenderRequest,
    PrivateTestnetSenderStatus, RelayerConfig, RelayerDryRun, RelayerPrivatePilotConfig,
    RelayerReceiptStatus, RelayerSubmissionRequest, TransactionSimulationPlan,
    PRIVATE_TESTNET_CAPPED_SEND_APPROVAL,
};

fn pilot_config_text() -> &'static str {
    r#"
environment_mode=testnet-only
cluster=devnet
submission_mode=testnet-submit-capped
rpc_url=https://api.devnet.solana.com/private-roc-to-rox-token-redacted
payer_keypair_path=/external/private-pilot/payer.json
operator_label=private-roc-to-rox-operator
asset_label=test-only-rox-private-pilot
receipt_output_path=/external/private-pilot/receipts/roc-to-rox-receipt.json
observed_signature=privateroctotroxsig111111111111111111111111111
"#
}

fn simulate_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    )
}

fn submit_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    )
}

fn simulate_config() -> RelayerConfig {
    RelayerConfig::new_with_safety(3, 16, simulate_safety())
}

fn submit_config() -> RelayerConfig {
    RelayerConfig::new_with_safety(2, 16, submit_safety())
}

fn external_config() -> RelayerPrivatePilotConfig {
    RelayerPrivatePilotConfig::from_external_config_text(submit_config(), pilot_config_text())
        .expect("static private ROC-to-ROX config should validate")
}

fn burn_intent() -> InternalRocDryRunBurnIntent {
    let package = fixtures::valid_package();

    InternalRocDryRunBurnIntent::new(
        simulate_safety(),
        package.operation_id,
        package.idempotency_key,
        package.nonce,
        AccountId::new("crablink-private-roc-burn-source-relayer-0001").unwrap(),
        "test-only-private-roc-to-rox-burn-intent",
        10,
    )
    .expect("static private ROC-to-ROX burn intent should validate")
}

fn accepted_simulation(
    read_only_rpc_verified: bool,
) -> rox_anchor_relayer::PrivatePilotSimulationResult {
    let package = fixtures::valid_package();
    let review = review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    );

    let mut relayer = RelayerDryRun::new(simulate_config());
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "private-roc-to-rox-test-rox-mint-target",
            review,
        ))
        .expect("static private ROC-to-ROX dry-run should fit receipt capacity");

    assert_eq!(dry_run.status, RelayerReceiptStatus::DryRunAccepted);

    let base = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(read_only_rpc_verified)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Observe,
                "observe-test-only-roc-burn",
                1,
            ),
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Finalize,
                "finalize-test-only-rox-mint",
                1,
            ),
        ]);

    simulate_private_pilot_transaction_plan(simulate_config(), plan)
}

fn authorized_request() -> PrivateTestnetSenderRequest {
    PrivateTestnetSenderRequest::new(accepted_simulation(true))
        .with_external_config(external_config())
        .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
        .with_receipt_output_path_declared(true)
        .with_limits(CappedTestnetSubmissionLimits::new(2, 1, 25, true))
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
}

#[test]
fn private_roc_to_rox_pilot_authorizes_only_test_rox_capped_path_without_sending() {
    let intent = burn_intent();
    let intent_report = intent.redacted_report_lines().join("\n");
    assert!(intent_report.contains("real_internal_roc_burn: disabled"));
    assert!(intent_report.contains("ron_ledger_mutation: disabled"));
    assert!(intent_report.contains("paid_content_unlock: disabled"));

    let simulation = accepted_simulation(true);
    assert_eq!(simulation.status, PrivatePilotSimulationStatus::Simulated);
    assert!(simulation.is_simulated());
    assert!(!simulation.live_submission);
    assert!(!simulation.wallet_key_loading);
    assert!(!simulation.internal_roc_mutation);

    let authorization = authorize_private_testnet_sender(authorized_request());

    assert_eq!(authorization.status, PrivateTestnetSenderStatus::Authorized);
    assert!(authorization.authorized);
    assert!(authorization.live_submission_permitted);
    assert!(!authorization.live_submission_attempted);
    assert!(!authorization.network_submitted);
    assert!(!authorization.wallet_key_loading);
    assert!(!authorization.signing);

    let report = authorization.redacted_report_lines().join("\n");
    assert!(report.contains("private_testnet_sender: explicit_capped_authorization"));
    assert!(report.contains("status: Authorized"));
    assert!(report.contains("capped_submit_status: Authorized"));
    assert!(report.contains("internal_roc_mutation: disabled"));
    assert!(report.contains("settlement_claim: none"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "transaction sent",
        "mint complete",
        "burn complete",
        "public mint complete",
        "settlement complete",
        "access granted",
        "roc released",
    ] {
        assert!(
            !report.to_ascii_lowercase().contains(forbidden),
            "report must not contain unsafe phrase: {forbidden}\n{report}"
        );
    }
}

#[test]
fn private_roc_to_rox_pilot_rejects_missing_read_only_rpc_gate() {
    let simulation = accepted_simulation(false);
    assert_eq!(
        simulation.status,
        PrivatePilotSimulationStatus::ReadOnlyRpcNotVerified
    );

    let authorization = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(simulation)
            .with_external_config(external_config())
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true),
    );

    assert_eq!(
        authorization.status,
        PrivateTestnetSenderStatus::SimulationNotAccepted
    );
    assert!(!authorization.authorized);
    assert!(!authorization.live_submission_permitted);
}

#[test]
fn private_roc_to_rox_pilot_rejects_halted_posture_before_authorization() {
    let authorization = authorize_private_testnet_sender(
        authorized_request().with_operational_posture(AnchorOperationalPosture::halted()),
    );

    assert_eq!(
        authorization.status,
        PrivateTestnetSenderStatus::PendingOperationalBlocker
    );
    assert!(!authorization.authorized);
    assert!(!authorization.live_submission_permitted);
}
