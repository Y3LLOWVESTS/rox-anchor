//! RO:WHAT — Tests BUILD_PLAN3 Phase 13 private ROX-to-ROC relayer pilot path.
//! RO:WHY — Proves reverse pilot can dry-run and simulate release intent without releasing real ROC.
//! RO:INTERACTS — proof review, internal ROC release intent, relayer dry-run, private simulation.
//! RO:INVARIANTS — simulation requires accepted proof, read-only RPC gate, and never mutates ROC directly.
//! RO:SECURITY — no real ROC release, svc-wallet call, ron-ledger mutation, signing, send, or settlement.
//! RO:TEST — cargo test -p rox-anchor-relayer --test private_rox_to_roc_pilot.

#![forbid(unsafe_code)]

use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, InternalRocDryRunReleaseIntent, MintId, Nonce, OperationId, ProgramId,
    RecoveryPosture, SubmissionMode, TokenAccountId,
};
use rox_anchor_proof::{
    review_proof_package, EvidenceBundle, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_relayer::{
    simulate_private_pilot_transaction_plan, PrivatePilotSimulationPlan,
    PrivatePilotSimulationStatus, PrivatePilotTransactionKind, PrivatePilotTransactionStep,
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
    TransactionSimulationPlan,
};

fn simulate_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    )
}

fn simulate_config() -> RelayerConfig {
    RelayerConfig::new_with_safety(3, 16, simulate_safety())
}

fn binding() -> AnchorBinding {
    AnchorBinding::new(
        DomainId::new("solana-devnet-rox-private-pilot-test").unwrap(),
        DomainId::new("internal-roc-private-pilot-test").unwrap(),
        AnchorDirection::RoxToRoc,
        ClusterId::new("devnet").unwrap(),
        ProgramId::new("PrivatePilotRoxAnchorProgram11111111").unwrap(),
        MintId::new("TestOnlyPrivatePilotRoxMint111111111").unwrap(),
        TokenAccountId::new("PrivatePilotRoxBurnSourceToken111111").unwrap(),
    )
}

fn package() -> ProofPackage {
    ProofPackage::new(
        binding(),
        OperationId::new("private-rox-to-roc-op-0001").unwrap(),
        IdempotencyKey::new("private-rox-to-roc-idem-0001").unwrap(),
        Nonce::new("private-rox-to-roc-nonce-0001").unwrap(),
        AccountId::new("private-rox-burn-source-0001").unwrap(),
        AccountId::new("crablink-private-roc-release-target-0001").unwrap(),
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    )
}

fn release_intent() -> InternalRocDryRunReleaseIntent {
    let package = package();

    InternalRocDryRunReleaseIntent::new(
        simulate_safety(),
        package.operation_id,
        package.idempotency_key,
        package.nonce,
        package.target_account,
        "test-only-private-rox-to-roc-release-intent",
        10,
    )
    .expect("static private ROX-to-ROC release intent should validate")
}

fn accepted_simulation(
    read_only_rpc_verified: bool,
) -> rox_anchor_relayer::PrivatePilotSimulationResult {
    let package = package();
    let expected = package.expected_binding_snapshot();
    let review = review_proof_package(&package, &expected, &ReplaySet::default());

    assert_eq!(review.decision, ReviewDecision::Accepted);

    let mut relayer = RelayerDryRun::new(simulate_config());
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "private-rox-to-roc-release-intent-target",
            review,
        ))
        .expect("static private ROX-to-ROC dry-run should fit receipt capacity");

    assert_eq!(dry_run.status, RelayerReceiptStatus::DryRunAccepted);

    let base = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(read_only_rpc_verified)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Observe,
                "observe-test-rox-burn-evidence",
                1,
            ),
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Finalize,
                "produce-internal-roc-release-intent-only",
                1,
            ),
        ]);

    simulate_private_pilot_transaction_plan(simulate_config(), plan)
}

#[test]
fn private_rox_to_roc_pilot_produces_only_dry_run_release_intent() {
    let intent = release_intent();
    let intent_report = intent.redacted_report_lines().join("\n");
    assert!(intent_report.contains("internal_roc_release_intent: dry_run_output"));
    assert!(intent_report.contains("real_internal_roc_release: disabled"));
    assert!(intent_report.contains("future_real_roc_path: svc-wallet -> ron-ledger only"));
    assert!(intent_report.contains("svc_wallet_call: disabled"));
    assert!(intent_report.contains("ron_ledger_mutation: disabled"));
    assert!(intent_report.contains("paid_content_unlock: disabled"));

    let simulation = accepted_simulation(true);
    assert_eq!(simulation.status, PrivatePilotSimulationStatus::Simulated);
    assert!(simulation.is_simulated());
    assert!(!simulation.live_submission);
    assert!(!simulation.wallet_key_loading);
    assert!(!simulation.internal_roc_mutation);

    let report = simulation.redacted_report_lines().join("\n");
    assert!(report.contains("private_pilot_simulation: local_only"));
    assert!(report.contains("status: Simulated"));
    assert!(report.contains("internal_roc_mutation: false"));
    assert!(report.contains("network_submission: disabled"));
    assert!(!report.contains("network_submitted: true"));
    assert!(report.contains("settlement_claim: none"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "transaction sent",
        "roc released",
        "ron ledger mutated",
        "wallet issued",
        "release complete",
        "settlement complete",
        "access granted",
    ] {
        assert!(
            !report.to_ascii_lowercase().contains(forbidden),
            "report must not contain unsafe phrase: {forbidden}\n{report}"
        );
    }
}

#[test]
fn private_rox_to_roc_pilot_rejects_missing_read_only_rpc_gate() {
    let simulation = accepted_simulation(false);
    assert_eq!(
        simulation.status,
        PrivatePilotSimulationStatus::ReadOnlyRpcNotVerified
    );
    assert!(!simulation.is_simulated());
    assert!(!simulation.live_submission);
    assert!(!simulation.internal_roc_mutation);
}

#[test]
fn private_rox_to_roc_replay_rejection_prevents_release_simulation() {
    let package = package();
    let expected = package.expected_binding_snapshot();
    let review = review_proof_package(&package, &expected, &ReplaySet::from_package(&package));

    assert_eq!(review.decision, ReviewDecision::Rejected);

    let mut relayer = RelayerDryRun::new(simulate_config());
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "private-rox-to-roc-release-intent-target",
            review,
        ))
        .expect("rejected proof still produces bounded receipt");

    assert_eq!(dry_run.status, RelayerReceiptStatus::ProofRejected);
    assert!(!dry_run.live_submission);
}
