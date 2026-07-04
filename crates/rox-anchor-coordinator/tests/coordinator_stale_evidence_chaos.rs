// RO:WHAT — Chaos test for stale RPC evidence through coordinator and relayer dry-run boundaries.
// RO:WHY — Proves stale observations cannot become accepted coordinator decisions or relayer attempts.
// RO:INTERACTS — rox-anchor-rpc-proof quorum review, rox-anchor-coordinator decision, rox-anchor-relayer dry-run.
// RO:INVARIANTS — stale evidence is rejected deterministically; fresh evidence with the same binding can still pass.
// RO:SECURITY — local model only; no live RPC, wallet calls, transaction submission, minting, burning, settlement, staking, liquidity, or deployment.
// RO:TEST — cargo test -p rox-anchor-coordinator --test coordinator_stale_evidence_chaos.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_proof::{fixtures, ExpectedProofBinding, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision, RpcQuorumFindingCode,
};

fn expected_rpc_binding(expected: &ExpectedProofBinding) -> ExpectedRpcBinding {
    let binding = expected.binding.clone();

    ExpectedRpcBinding::new(
        binding.cluster,
        binding.program_id,
        binding.mint,
        binding.token_account,
        expected.operation_id.clone(),
        RpcCommitmentLevel::Finalized,
    )
}

fn observations_at_slot(expected: &ExpectedRpcBinding, slot: u64) -> Vec<RpcObservation> {
    vec![
        RpcObservation::new(
            "rpc-a",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "sig-same-stale-chaos-0001",
            slot,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "rpc-b",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "sig-same-stale-chaos-0001",
            slot,
            RpcCommitmentLevel::Finalized,
        ),
    ]
}

fn request_at_slot(slot: u64) -> CoordinatorReviewRequest {
    let package = fixtures::valid_package();
    let expected = package.expected_binding_snapshot();
    let expected_rpc = expected_rpc_binding(&expected);
    let observations = observations_at_slot(&expected_rpc, slot);

    CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        observations,
        ReplaySet::default(),
    )
}

#[test]
fn stale_rpc_evidence_is_rejected_before_relayer_attempts() {
    let request = request_at_slot(10);
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

    assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
    assert!(decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::StaleEvidence));
    assert!(!decision.is_accepted());

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                request.package.operation_id.clone(),
                request.package.idempotency_key.clone(),
                "local-anchor-stale-chaos-dry-run",
                decision.proof_review,
            )
            .with_requested_attempts(3),
        )
        .unwrap();

    assert_ne!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.attempts_used, 0);
    assert!(!receipt.live_submission);
    assert_eq!(relayer.receipts().len(), 1);
}

#[test]
fn repeated_stale_evidence_reviews_are_deterministic() {
    let mut first_snapshot: Option<String> = None;

    for _attempt in 0..32 {
        let request = request_at_slot(10);
        let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

        assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
        assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
        assert!(decision
            .rpc_review
            .has_finding(RpcQuorumFindingCode::StaleEvidence));
        assert!(!decision.is_accepted());

        let snapshot = format!(
            "status={:?};rpc={:?};findings={:?};proof={:?};accepted={}",
            decision.status,
            decision.rpc_review.decision,
            decision.rpc_review.findings,
            decision.proof_review.decision,
            decision.is_accepted()
        );

        if let Some(previous) = &first_snapshot {
            assert_eq!(&snapshot, previous);
        } else {
            first_snapshot = Some(snapshot);
        }
    }
}

#[test]
fn fresh_rpc_evidence_after_stale_case_can_still_accept() {
    let request = request_at_slot(450);
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 500);

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
                "local-anchor-fresh-after-stale-chaos-dry-run",
                decision.proof_review,
            )
            .with_requested_attempts(2),
        )
        .unwrap();

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.proof_decision, ReviewDecision::Accepted);
    assert_eq!(receipt.attempts_used, 2);
    assert!(!receipt.live_submission);
}
