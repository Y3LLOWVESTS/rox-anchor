//! RO:WHAT — Tests BUILD_PLAN2 Phase 5 simulation-only transaction planning.
//! RO:WHY — Proves simulation cannot bypass proof, coordinator, relayer dry-run, or safety gates.
//! RO:INTERACTS — RelayerDryRun, RelayerReceipt, TransactionSimulationPlan, and AnchorSafetyProfile.
//! RO:INVARIANTS — successful simulation never sets live_submission=true and never sends a transaction.
//! RO:SECURITY — no RPC, key loading, wallet, mint, burn, transaction send, bridge settlement, or live value movement.
//! RO:TEST — run with cargo test -p rox-anchor-relayer --test transaction_simulation.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, EvidenceBundle, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    simulate_transaction_plan, RelayerConfig, RelayerDryRun, RelayerReceiptStatus,
    RelayerSubmissionRequest, TransactionSimulationPlan, TransactionSimulationStatus,
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
        "simulation-only-target",
        review,
    )
}

fn accepted_dry_run_receipt() -> rox_anchor_relayer::RelayerReceipt {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    relayer
        .submit_dry_run(request_with_review(accepted_review()))
        .expect("accepted proof should dry-run")
}

#[test]
fn accepted_proof_coordinator_and_relayer_path_can_simulate_without_live_submission() {
    let receipt = accepted_dry_run_receipt();
    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);

    let result = simulate_transaction_plan(RelayerConfig::new(3, 16), plan);

    assert_eq!(result.status, TransactionSimulationStatus::Simulated);
    assert!(result.is_simulated());
    assert!(result.simulated);
    assert!(!result.live_submission);
    assert_eq!(result.proof_decision, ReviewDecision::Accepted);
    assert_eq!(result.relayer_status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(result.instruction_count, 2);
}

#[test]
fn simulation_rejects_missing_coordinator_acceptance_even_after_relayer_dry_run() {
    let receipt = accepted_dry_run_receipt();
    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, false, 2);

    let result = simulate_transaction_plan(RelayerConfig::new(3, 16), plan);

    assert_eq!(
        result.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert!(!result.simulated);
    assert!(!result.live_submission);
}

#[test]
fn simulation_rejects_blocked_proof_reviews() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(request_with_review(blocked_review()))
        .expect("blocked proof should create non-attempt receipt");
    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);

    let result = simulate_transaction_plan(RelayerConfig::new(3, 16), plan);

    assert_eq!(result.status, TransactionSimulationStatus::ProofNotAccepted);
    assert_eq!(result.proof_decision, ReviewDecision::Blocked);
    assert_eq!(result.relayer_status, RelayerReceiptStatus::ProofBlocked);
    assert!(!result.live_submission);
}

#[test]
fn simulation_rejects_duplicate_relayer_dry_run_receipts() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let first = relayer
        .submit_dry_run(request_with_review(accepted_review()))
        .expect("first dry-run should be accepted");
    let duplicate = relayer
        .submit_dry_run(request_with_review(accepted_review()))
        .expect("duplicate should produce receipt");

    assert_eq!(first.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(duplicate.status, RelayerReceiptStatus::DuplicateRequest);

    let plan = TransactionSimulationPlan::from_dry_run_receipt(duplicate, true, 2);
    let result = simulate_transaction_plan(RelayerConfig::new(3, 16), plan);

    assert_eq!(
        result.status,
        TransactionSimulationStatus::RelayerDryRunNotAccepted
    );
    assert_eq!(result.proof_decision, ReviewDecision::Accepted);
    assert_eq!(
        result.relayer_status,
        RelayerReceiptStatus::DuplicateRequest
    );
    assert!(!result.live_submission);
}

#[test]
fn simulation_rejects_empty_instruction_plan() {
    let receipt = accepted_dry_run_receipt();
    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 0);

    let result = simulate_transaction_plan(RelayerConfig::new(3, 16), plan);

    assert_eq!(
        result.status,
        TransactionSimulationStatus::EmptyInstructionPlan
    );
    assert!(!result.live_submission);
}

#[test]
fn simulation_rejects_unsafe_or_submitting_scope() {
    let receipt = accepted_dry_run_receipt();
    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);
    let unsafe_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );
    let config = RelayerConfig::new_with_safety(3, 16, unsafe_safety);

    let result = simulate_transaction_plan(config, plan);

    assert_eq!(result.status, TransactionSimulationStatus::UnsafeScope);
    assert!(!result.live_submission);
}
