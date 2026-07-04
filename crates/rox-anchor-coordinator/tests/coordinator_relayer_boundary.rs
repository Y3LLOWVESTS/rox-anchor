// RO:WHAT — Active integration test from coordinator proof review into relayer dry-run.
// RO:WHY — Proves accepted local evidence can hand off to the relayer without inventing new rules.
// RO:INTERACTS — rox-anchor-proof fixtures, rox-anchor-rpc-proof quorum, rox-anchor-coordinator decision, and rox-anchor-relayer dry-run.
// RO:INVARIANTS — coordinator acceptance comes from RPC/proof review; relayer remains dry-run and never performs live submission.
// RO:SECURITY — no live RPC, wallet calls, Anchor transaction submission, minting, burning, bridge settlement, or value movement.
// RO:TEST — cargo test -p rox-anchor-coordinator --test coordinator_relayer_boundary.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_proof::{fixtures, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision,
};

fn expected_rpc_binding() -> ExpectedRpcBinding {
    let expected = fixtures::expected_proof_binding();
    let binding = expected.binding.clone();

    ExpectedRpcBinding::new(
        binding.cluster,
        binding.program_id,
        binding.mint,
        binding.token_account,
        expected.operation_id,
        RpcCommitmentLevel::Finalized,
    )
}

fn matching_observations(expected: &ExpectedRpcBinding) -> Vec<RpcObservation> {
    vec![
        RpcObservation::new(
            "rpc-a",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "sig-same-accepted-0001",
            100,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "rpc-b",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "sig-same-accepted-0001",
            100,
            RpcCommitmentLevel::Finalized,
        ),
    ]
}

fn accepted_request() -> CoordinatorReviewRequest {
    let package = fixtures::valid_package();
    let expected = package.expected_binding_snapshot();
    let expected_rpc = expected_rpc_binding();
    let observations = matching_observations(&expected_rpc);

    CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        observations,
        ReplaySet::default(),
    )
}

#[test]
fn accepted_coordinator_review_produces_dry_run_relayer_receipt() {
    let request = accepted_request();

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);
    assert!(decision.is_accepted());

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                request.package.operation_id.clone(),
                request.package.idempotency_key.clone(),
                "local-anchor-dry-run",
                decision.proof_review,
            )
            .with_requested_attempts(2),
        )
        .unwrap();

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.proof_decision, ReviewDecision::Accepted);
    assert_eq!(receipt.attempts_used, 2);
    assert!(!receipt.live_submission);
    assert_eq!(relayer.receipts().len(), 1);
}

#[test]
fn rejected_rpc_evidence_does_not_become_dry_run_submission() {
    let mut request = accepted_request();
    request.observations[0].cluster = rox_anchor_core::ClusterId::new("wrong-localnet").unwrap();

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);

    assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
    assert!(!decision.is_accepted());

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            request.package.operation_id.clone(),
            request.package.idempotency_key.clone(),
            "local-anchor-dry-run",
            decision.proof_review,
        ))
        .unwrap();

    assert_ne!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.attempts_used, 0);
    assert!(!receipt.live_submission);
}

#[test]
fn rpc_signature_disagreement_blocks_relayer_acceptance() {
    let mut request = accepted_request();
    request.observations[1].signature = "sig-conflicting-observation-0001".to_string();

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);

    assert_eq!(decision.status, CoordinatorDecisionStatus::BlockedProof);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Disputed);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Blocked);
    assert!(!decision.is_accepted());

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                request.package.operation_id.clone(),
                request.package.idempotency_key.clone(),
                "local-anchor-dry-run",
                decision.proof_review,
            )
            .with_requested_attempts(3),
        )
        .unwrap();

    assert_eq!(receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(receipt.proof_decision, ReviewDecision::Blocked);
    assert_eq!(receipt.attempts_used, 0);
    assert!(!receipt.live_submission);
    assert_eq!(relayer.receipts().len(), 1);
}
