// RO:WHAT — Active nonvalue bidirectional local integration flow for ROC->ROX and ROX->ROC.
// RO:WHY — Proves both directions can move through proof, RPC quorum, coordinator, and relayer dry-run surfaces.
// RO:INTERACTS — rox-anchor-core bindings, rox-anchor-proof package review, rox-anchor-rpc-proof quorum, coordinator, relayer.
// RO:INVARIANTS — accepted local evidence can dry-run only; no live submission, no production mint, no production burn, no ROC release.
// RO:SECURITY — no live RPC, wallet calls, Anchor transaction submission, minting, burning, bridge settlement, staking, liquidity, or exchange behavior.
// RO:TEST — cargo test -p rox-anchor-coordinator --test local_nonvalue_bidirectional_flow.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorDirection, ChallengePosture, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture, TokenAccountId,
};
use rox_anchor_proof::{
    EvidenceBundle, ExpectedProofBinding, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_relayer::{
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision,
};

fn package_for(direction: AnchorDirection) -> ProofPackage {
    let (
        source_domain,
        target_domain,
        operation_id,
        idempotency_key,
        nonce,
        source_account,
        target_account,
        token_account,
    ) = match direction {
        AnchorDirection::RocToRox => (
            "internal-roc",
            "solana-localnet",
            "op-roc-to-rox-local-nonvalue-0001",
            "idem-roc-to-rox-local-nonvalue-0001",
            "nonce-roc-to-rox-local-nonvalue-0001",
            "roc-source-account-local-0001",
            "rox-recipient-owner-local-0001",
            "rox-recipient-token-account-local-0001",
        ),
        AnchorDirection::RoxToRoc => (
            "solana-localnet",
            "internal-roc",
            "op-rox-to-roc-local-nonvalue-0001",
            "idem-rox-to-roc-local-nonvalue-0001",
            "nonce-rox-to-roc-local-nonvalue-0001",
            "rox-burn-source-owner-local-0001",
            "roc-release-review-account-local-0001",
            "rox-burn-source-token-account-local-0001",
        ),
    };

    let binding = AnchorBinding::new(
        DomainId::new(source_domain).unwrap(),
        DomainId::new(target_domain).unwrap(),
        direction,
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new(token_account).unwrap(),
    );

    ProofPackage::new(
        binding,
        OperationId::new(operation_id).unwrap(),
        IdempotencyKey::new(idempotency_key).unwrap(),
        Nonce::new(nonce).unwrap(),
        AccountId::new(source_account).unwrap(),
        AccountId::new(target_account).unwrap(),
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    )
}

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

fn matching_observations(
    expected: &ExpectedRpcBinding,
    signature: &'static str,
) -> Vec<RpcObservation> {
    vec![
        RpcObservation::new(
            "rpc-a",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            signature,
            200,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "rpc-b",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            signature,
            201,
            RpcCommitmentLevel::Finalized,
        ),
    ]
}

fn run_local_nonvalue_flow(direction: AnchorDirection, signature: &'static str) {
    let package = package_for(direction);
    let expected = package.expected_binding_snapshot();
    let expected_rpc = expected_rpc_binding(&expected);
    let observations = matching_observations(&expected_rpc, signature);

    let request = CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        observations,
        ReplaySet::default(),
    );

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 220);

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            request.package.operation_id.clone(),
            request.package.idempotency_key.clone(),
            "local-anchor-nonvalue-dry-run",
            decision.proof_review,
        ))
        .unwrap();

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.proof_decision, ReviewDecision::Accepted);
    assert!(receipt.attempts_used > 0);
    assert!(!receipt.live_submission);
    assert_eq!(relayer.receipts().len(), 1);
}

#[test]
fn local_nonvalue_roc_to_rox_flow_reaches_dry_run_receipt() {
    run_local_nonvalue_flow(
        AnchorDirection::RocToRox,
        "sig-local-nonvalue-roc-to-rox-0001",
    );
}

#[test]
fn local_nonvalue_rox_to_roc_flow_reaches_dry_run_receipt_without_roc_release() {
    run_local_nonvalue_flow(
        AnchorDirection::RoxToRoc,
        "sig-local-nonvalue-rox-to-roc-0001",
    );
}
