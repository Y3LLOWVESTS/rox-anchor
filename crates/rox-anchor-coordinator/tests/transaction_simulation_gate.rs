//! RO:WHAT — Tests coordinator gate for BUILD_PLAN2 Phase 5 simulation planning.
//! RO:WHY — Proves only accepted coordinator decisions are eligible for relayer-side simulation.
//! RO:INTERACTS — CoordinatorDecision, RPC proof review, rox-anchor-proof, and relayer simulation plan.
//! RO:INVARIANTS — blocked/rejected coordinator decisions cannot be treated as simulation-approved.
//! RO:SECURITY — fake/local review only; no live RPC, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-coordinator --test transaction_simulation_gate.

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorReviewRequest,
};
use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
use rox_anchor_relayer::{
    simulate_transaction_plan, RelayerConfig, RelayerDryRun, RelayerSubmissionRequest,
    TransactionSimulationPlan, TransactionSimulationStatus,
};
use rox_anchor_rpc_proof::{ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation};

fn expected_rpc_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        RpcCommitmentLevel::Confirmed,
    )
}

fn observation(source: &str, signature: &str, slot: u64) -> RpcObservation {
    RpcObservation::new(
        source,
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        signature,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn accepted_review() -> rox_anchor_proof::ProofReview {
    review_proof_package(
        &fixtures::valid_package(),
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn simulation_result_for_coordinator_gate(
    coordinator_accepted: bool,
) -> rox_anchor_relayer::TransactionSimulationResult {
    let package = fixtures::valid_package();
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "coordinator-gated-simulation",
            accepted_review(),
        ))
        .expect("dry-run should produce receipt");

    let plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, coordinator_accepted, 2);

    simulate_transaction_plan(RelayerConfig::new(3, 16), plan)
}

#[test]
fn accepted_coordinator_decision_permits_simulation_gate() {
    let package = fixtures::valid_package();
    let request = CoordinatorReviewRequest::new(
        package,
        fixtures::expected_proof_binding(),
        expected_rpc_binding(),
        vec![
            observation("rpc-a", "sig-simulation-gate-111111111111", 90),
            observation("rpc-b", "sig-simulation-gate-111111111111", 91),
        ],
        ReplaySet::default(),
    );

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 4), 100);

    assert!(decision.permits_transaction_simulation());

    let result = simulation_result_for_coordinator_gate(decision.permits_transaction_simulation());
    assert_eq!(result.status, TransactionSimulationStatus::Simulated);
    assert!(!result.live_submission);
}

#[test]
fn rejected_evidence_coordinator_decision_blocks_simulation_gate() {
    let package = fixtures::valid_package();
    let request = CoordinatorReviewRequest::new(
        package,
        fixtures::expected_proof_binding(),
        expected_rpc_binding(),
        vec![
            observation("rpc-a", "sig-stale-simulation-gate-111111111111", 1),
            observation("rpc-b", "sig-stale-simulation-gate-111111111111", 2),
        ],
        ReplaySet::default(),
    );

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 5, 4), 100);

    assert!(!decision.permits_transaction_simulation());

    let result = simulation_result_for_coordinator_gate(decision.permits_transaction_simulation());
    assert_eq!(
        result.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert!(!result.live_submission);
}
