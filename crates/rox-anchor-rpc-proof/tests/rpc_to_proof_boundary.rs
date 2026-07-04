// RO:WHAT — Active integration test from RPC quorum evidence into proof-package review.
// RO:WHY — Proves RPC evidence posture feeds the proof engine instead of becoming a separate acceptance rule.
// RO:INTERACTS — rox-anchor-rpc-proof quorum review and rox-anchor-proof package validation.
// RO:INVARIANTS — RPC agreement can satisfy evidence; missing/disputed RPC evidence blocks acceptance.
// RO:SECURITY — local evidence model only; no live RPC, wallet, transaction submission, minting, burning, or settlement.
// RO:TEST — cargo test -p rox-anchor-rpc-proof --test rpc_to_proof_boundary.

#![forbid(unsafe_code)]

use rox_anchor_proof::{
    fixtures, review_proof_package, ProofFindingCode, ReplaySet, ReviewDecision,
};
use rox_anchor_rpc_proof::{
    review_rpc_observations, ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation,
    RpcProofConfig, RpcQuorumDecision,
};

fn expected_rpc_binding() -> ExpectedRpcBinding {
    let expected = fixtures::expected_proof_binding();
    let binding = expected.binding;

    ExpectedRpcBinding::new(
        binding.cluster,
        binding.program_id,
        binding.mint,
        binding.token_account,
        expected.operation_id,
        RpcCommitmentLevel::Finalized,
    )
}

fn observation(source: &str, signature: &str, slot: u64) -> RpcObservation {
    let expected = expected_rpc_binding();

    RpcObservation::new(
        source,
        expected.cluster,
        expected.program_id,
        expected.mint,
        expected.token_account,
        expected.operation_id,
        signature,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

#[test]
fn rpc_agreement_becomes_proof_accepted_evidence() {
    let rpc_review = review_rpc_observations(
        &[
            observation("rpc-a", "sig-agreed-roc-to-rox-0001", 100),
            observation("rpc-b", "sig-agreed-roc-to-rox-0001", 101),
        ],
        &expected_rpc_binding(),
        RpcProofConfig::new(2, 20),
        110,
    );

    assert_eq!(rpc_review.decision, RpcQuorumDecision::Agreement);

    let mut package = fixtures::valid_package();
    let expected_proof = package.expected_binding_snapshot();
    package.evidence = rpc_review.to_evidence_bundle();

    let proof_review = review_proof_package(&package, &expected_proof, &ReplaySet::default());

    assert_eq!(proof_review.decision, ReviewDecision::Accepted);
    assert!(proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::PackageAccepted));
}

#[test]
fn missing_rpc_evidence_blocks_proof_acceptance() {
    let rpc_review = review_rpc_observations(
        &[observation("rpc-a", "sig-single-roc-to-rox-0001", 100)],
        &expected_rpc_binding(),
        RpcProofConfig::new(2, 20),
        110,
    );

    assert_eq!(rpc_review.decision, RpcQuorumDecision::MissingEvidence);

    let mut package = fixtures::valid_package();
    let expected_proof = package.expected_binding_snapshot();
    package.evidence = rpc_review.to_evidence_bundle();

    let proof_review = review_proof_package(&package, &expected_proof, &ReplaySet::default());

    assert_eq!(proof_review.decision, ReviewDecision::Blocked);
    assert!(proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::EvidenceMissing));
}

#[test]
fn disputed_rpc_evidence_blocks_proof_acceptance() {
    let rpc_review = review_rpc_observations(
        &[
            observation("rpc-a", "sig-left-roc-to-rox-0001", 100),
            observation("rpc-b", "sig-right-roc-to-rox-0001", 101),
        ],
        &expected_rpc_binding(),
        RpcProofConfig::new(2, 20),
        110,
    );

    assert_eq!(rpc_review.decision, RpcQuorumDecision::Disputed);

    let mut package = fixtures::valid_package();
    let expected_proof = package.expected_binding_snapshot();
    package.evidence = rpc_review.to_evidence_bundle();

    let proof_review = review_proof_package(&package, &expected_proof, &ReplaySet::default());

    assert_eq!(proof_review.decision, ReviewDecision::Blocked);
    assert!(proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::QuorumDisputed));
}
