//! RO:WHAT — Tests BUILD_PLAN3 Phase 7 simulation-only private pilot transaction plans.
//! RO:WHY — Proves pilot simulation cannot bypass read-only RPC verification, proof, coordinator, or relayer gates.
//! RO:INTERACTS — PrivatePilotSimulationPlan, TransactionSimulationPlan, RelayerDryRun, and proof review.
//! RO:INVARIANTS — simulation never submits, loads wallets, mints, burns, settles, or mutates ROC.
//! RO:SECURITY — no live RPC, key loading, wallet, transaction send, mint, burn, bridge settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-relayer --test private_pilot_simulation.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, EvidenceBundle, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    simulate_private_pilot_transaction_plan, PrivatePilotSimulationPlan,
    PrivatePilotSimulationStatus, PrivatePilotTransactionKind, PrivatePilotTransactionStep,
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
    TransactionSimulationPlan,
};

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

fn request_with_review(review: rox_anchor_proof::ProofReview) -> RelayerSubmissionRequest {
    let package = fixtures::valid_package();

    RelayerSubmissionRequest::new(
        package.operation_id,
        package.idempotency_key,
        "private-pilot-simulation-target",
        review,
    )
    .with_requested_attempts(1)
}

fn dry_run_receipt(review: rox_anchor_proof::ProofReview) -> rox_anchor_relayer::RelayerReceipt {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    relayer
        .submit_dry_run(request_with_review(review))
        .expect("dry-run should produce local receipt")
}

fn accepted_plan(
    read_only_verified: bool,
    coordinator_accepted: bool,
    instruction_count: u16,
) -> PrivatePilotSimulationPlan {
    let receipt = dry_run_receipt(accepted_review());
    let base = TransactionSimulationPlan::from_dry_run_receipt(
        receipt,
        coordinator_accepted,
        instruction_count,
    );

    PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(read_only_verified)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Observe,
                "observe-test-evidence",
                1,
            ),
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Finalize,
                "finalize-test-intent",
                1,
            ),
        ])
}

#[test]
fn private_pilot_simulation_accepts_only_after_read_only_and_dry_run_gates() {
    let result = simulate_private_pilot_transaction_plan(
        RelayerConfig::new(3, 16),
        accepted_plan(true, true, 2),
    );

    assert_eq!(result.status, PrivatePilotSimulationStatus::Simulated);
    assert!(result.is_simulated());
    assert!(result.read_only_rpc_verified);
    assert_eq!(result.step_count, 2);
    assert_eq!(result.planned_instruction_count, 2);
    assert!(!result.live_submission);
    assert!(!result.wallet_key_loading);
    assert!(!result.internal_roc_mutation);

    let base = result
        .base_result
        .as_ref()
        .expect("base result should exist");
    assert_eq!(base.proof_decision, ReviewDecision::Accepted);
    assert_eq!(base.relayer_status, RelayerReceiptStatus::DryRunAccepted);
    assert!(!base.live_submission);

    let report = result.redacted_report_lines().join("\n");
    assert!(report.contains("private_pilot_simulation: local_only"));
    assert!(report.contains("simulated: true"));
    assert!(report.contains("read_only_rpc_verified: true"));
    assert!(report.contains("live_submission: false"));
    assert!(report.contains("wallet_key_loading: false"));
    assert!(report.contains("internal_roc_mutation: false"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
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
fn private_pilot_simulation_requires_read_only_rpc_verification() {
    let result = simulate_private_pilot_transaction_plan(
        RelayerConfig::new(3, 16),
        accepted_plan(false, true, 2),
    );

    assert_eq!(
        result.status,
        PrivatePilotSimulationStatus::ReadOnlyRpcNotVerified
    );
    assert!(result.base_result.is_none());
    assert!(!result.live_submission);
}

#[test]
fn private_pilot_simulation_rejects_missing_steps_and_instruction_mismatch() {
    let receipt = dry_run_receipt(accepted_review());
    let base = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);

    let missing_steps = PrivatePilotSimulationPlan::new(base.clone())
        .with_read_only_rpc_verified(true)
        .with_steps(Vec::new());

    let missing_result =
        simulate_private_pilot_transaction_plan(RelayerConfig::new(3, 16), missing_steps);
    assert_eq!(
        missing_result.status,
        PrivatePilotSimulationStatus::MissingTransactionSteps
    );

    let mismatched_steps = PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(true)
        .with_steps(vec![PrivatePilotTransactionStep::new(
            PrivatePilotTransactionKind::Initialize,
            "initialize-only-one-instruction",
            1,
        )]);

    let mismatch_result =
        simulate_private_pilot_transaction_plan(RelayerConfig::new(3, 16), mismatched_steps);
    assert_eq!(
        mismatch_result.status,
        PrivatePilotSimulationStatus::InstructionCountMismatch
    );
    assert!(mismatch_result.base_result.is_none());
}

#[test]
fn private_pilot_simulation_rejects_missing_coordinator_acceptance() {
    let result = simulate_private_pilot_transaction_plan(
        RelayerConfig::new(3, 16),
        accepted_plan(true, false, 2),
    );

    assert_eq!(
        result.status,
        PrivatePilotSimulationStatus::CoordinatorNotAccepted
    );
    assert!(!result.live_submission);
}

#[test]
fn private_pilot_simulation_rejects_blocked_proof_and_relayer_dry_run() {
    let receipt = dry_run_receipt(blocked_review());
    assert_eq!(receipt.status, RelayerReceiptStatus::ProofBlocked);

    let base = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(true)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(PrivatePilotTransactionKind::Observe, "observe", 1),
            PrivatePilotTransactionStep::new(PrivatePilotTransactionKind::Finalize, "finalize", 1),
        ]);

    let result = simulate_private_pilot_transaction_plan(RelayerConfig::new(3, 16), plan);

    assert_eq!(
        result.status,
        PrivatePilotSimulationStatus::ProofNotAccepted
    );
    assert!(!result.live_submission);
    assert_eq!(
        result
            .base_result
            .as_ref()
            .expect("base result should exist")
            .relayer_status,
        RelayerReceiptStatus::ProofBlocked
    );
}

#[test]
fn private_pilot_simulation_rejects_unsafe_submitting_scope() {
    let unsafe_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );
    let config = RelayerConfig::new_with_safety(3, 16, unsafe_safety);

    let result = simulate_private_pilot_transaction_plan(config, accepted_plan(true, true, 2));

    assert_eq!(result.status, PrivatePilotSimulationStatus::UnsafeScope);
    assert!(!result.live_submission);
}
