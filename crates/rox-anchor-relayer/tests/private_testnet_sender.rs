//! RO:WHAT — Tests BUILD_PLAN3 Phase 8 explicit capped private testnet sender authorization.
//! RO:WHY — Proves the first send-capable path is explicit, capped, external-config-backed, and still non-executing in tests.
//! RO:INTERACTS — private pilot config, private simulation result, capped authorization, safety profile, and posture blockers.
//! RO:INVARIANTS — no default send, no missing approval, no missing receipt path, no blocked posture, no failed simulation.
//! RO:SECURITY — no live RPC, wallet loading, signing, transaction send, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-relayer --test private_testnet_sender.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorOperationalPosture, AnchorSafetyProfile,
    ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, EvidenceBundle, ReplaySet};
use rox_anchor_relayer::{
    authorize_private_testnet_sender, simulate_private_pilot_transaction_plan,
    CappedTestnetSubmissionLimits, PrivatePilotSimulationPlan, PrivatePilotTransactionKind,
    PrivatePilotTransactionStep, PrivateTestnetSenderRequest, PrivateTestnetSenderStatus,
    RelayerConfig, RelayerDryRun, RelayerPrivatePilotConfig, RelayerSubmissionRequest,
    TransactionSimulationPlan, PRIVATE_TESTNET_CAPPED_SEND_APPROVAL,
};

fn pilot_config_text() -> &'static str {
    r#"
environment_mode=testnet-only
cluster=devnet
submission_mode=testnet-submit-capped
rpc_url=https://api.devnet.solana.com/private-pilot-token-redacted
payer_keypair_path=/external/private-pilot/payer.json
operator_label=private-pilot-operator
asset_label=test-only-rox-asset
receipt_output_path=/external/private-pilot/receipts/receipt.json
observed_signature=privatepilotsignature1111111111111111111111111111
"#
}

fn submit_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    )
}

fn submitting_config() -> RelayerConfig {
    RelayerConfig::new_with_safety(2, 16, submit_safety())
}

fn external_config() -> RelayerPrivatePilotConfig {
    RelayerPrivatePilotConfig::from_external_config_text(submitting_config(), pilot_config_text())
        .expect("static private pilot config should validate")
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

fn simulation_result_from_review(
    review: rox_anchor_proof::ProofReview,
    read_only_verified: bool,
) -> rox_anchor_relayer::PrivatePilotSimulationResult {
    let package = fixtures::valid_package();

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "private-testnet-sender-target",
            review,
        ))
        .expect("static dry-run should fit capacity");

    let base = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(read_only_verified)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(PrivatePilotTransactionKind::Observe, "observe", 1),
            PrivatePilotTransactionStep::new(PrivatePilotTransactionKind::Finalize, "finalize", 1),
        ]);

    simulate_private_pilot_transaction_plan(RelayerConfig::new(3, 16), plan)
}

fn accepted_simulation() -> rox_anchor_relayer::PrivatePilotSimulationResult {
    simulation_result_from_review(accepted_review(), true)
}

fn authorized_request() -> PrivateTestnetSenderRequest {
    PrivateTestnetSenderRequest::new(accepted_simulation())
        .with_external_config(external_config())
        .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
        .with_receipt_output_path_declared(true)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
}

#[test]
fn private_testnet_sender_authorizes_only_after_every_phase8_gate() {
    let result = authorize_private_testnet_sender(authorized_request());

    assert_eq!(result.status, PrivateTestnetSenderStatus::Authorized);
    assert!(result.is_authorized());
    assert!(result.authorized);
    assert!(result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
    assert!(!result.wallet_key_loading);
    assert!(!result.signing);
    assert!(result.external_config_validated);
    assert_eq!(
        result.receipt_output_path_redacted.as_deref(),
        Some("<redacted-external-path>/*.json")
    );

    let capped = result
        .capped_result
        .as_ref()
        .expect("authorized result should include capped authorization");
    assert!(capped.is_authorized());
    assert!(capped.live_submission_permitted);
    assert!(!capped.live_submission_attempted);
    assert!(!capped.network_submitted);

    let report = result.redacted_report_lines().join("\n");
    assert!(report.contains("private_testnet_sender: explicit_capped_authorization"));
    assert!(report.contains("status: Authorized"));
    assert!(report.contains("live_submission_permitted: true"));
    assert!(report.contains("live_submission_attempted: false"));
    assert!(report.contains("network_submitted: false"));
    assert!(report.contains("wallet_key_loading: false"));
    assert!(report.contains("signing: false"));
    assert!(report.contains("internal_roc_mutation: disabled"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "transaction sent",
        "mint complete",
        "burn complete",
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
fn private_testnet_sender_rejects_missing_external_config() {
    let result = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(accepted_simulation())
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true),
    );

    assert_eq!(
        result.status,
        PrivateTestnetSenderStatus::MissingExternalConfig
    );
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
}

#[test]
fn private_testnet_sender_rejects_unsafe_external_config_mode() {
    let unsafe_config = RelayerPrivatePilotConfig::from_external_config_text(
        RelayerConfig::new(2, 16),
        pilot_config_text(),
    )
    .expect("pilot config text itself should parse");

    let result = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(accepted_simulation())
            .with_external_config(unsafe_config)
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true),
    );

    assert_eq!(
        result.status,
        PrivateTestnetSenderStatus::UnsafeExternalConfig
    );
    assert!(!result.authorized);
}

#[test]
fn private_testnet_sender_rejects_missing_operator_approval_and_receipt_output_path() {
    let missing_approval = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(accepted_simulation())
            .with_external_config(external_config())
            .with_receipt_output_path_declared(true),
    );

    assert_eq!(
        missing_approval.status,
        PrivateTestnetSenderStatus::MissingOperatorApproval
    );
    assert!(!missing_approval.authorized);

    let missing_receipt = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(accepted_simulation())
            .with_external_config(external_config())
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL),
    );

    assert_eq!(
        missing_receipt.status,
        PrivateTestnetSenderStatus::MissingReceiptOutputPath
    );
    assert!(!missing_receipt.authorized);
}

#[test]
fn private_testnet_sender_rejects_pending_operational_blockers() {
    let result = authorize_private_testnet_sender(
        authorized_request().with_operational_posture(AnchorOperationalPosture::halted()),
    );

    assert_eq!(
        result.status,
        PrivateTestnetSenderStatus::PendingOperationalBlocker
    );
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
}

#[test]
fn private_testnet_sender_rejects_unverified_or_blocked_simulation() {
    let unverified_simulation = simulation_result_from_review(accepted_review(), false);
    let unverified_result = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(unverified_simulation)
            .with_external_config(external_config())
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true),
    );

    assert_eq!(
        unverified_result.status,
        PrivateTestnetSenderStatus::SimulationNotAccepted
    );

    let blocked_simulation = simulation_result_from_review(blocked_review(), true);
    let blocked_result = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(blocked_simulation)
            .with_external_config(external_config())
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true),
    );

    assert_eq!(
        blocked_result.status,
        PrivateTestnetSenderStatus::SimulationNotAccepted
    );
}

#[test]
fn private_testnet_sender_surfaces_capped_authorization_rejections() {
    let result = authorize_private_testnet_sender(
        authorized_request()
            .with_amount_units(1_000)
            .with_limits(CappedTestnetSubmissionLimits::new(2, 2, 100, true)),
    );

    assert_eq!(
        result.status,
        PrivateTestnetSenderStatus::CappedAuthorizationRejected
    );
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
    assert!(result.capped_result.is_some());
}
