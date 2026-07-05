// RO:WHAT — Chaos tests for RPC equivocation classification.
// RO:WHY — Proves conflicting RPC claims cannot become agreement or clean proof evidence.
// RO:INTERACTS — rox-anchor-rpc-proof quorum review and rox-anchor-proof evidence bundle mapping.
// RO:INVARIANTS — conflicting signatures dispute; same-source equivocation is flagged; repeated review is deterministic.
// RO:SECURITY — local evidence classification only; no live RPC, wallet calls, transaction submission, minting, burning, settlement, staking, liquidity, or deployment.
// RO:TEST — cargo test -p rox-anchor-rpc-proof --test rpc_equivocation_chaos.

#![forbid(unsafe_code)]

use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    review_rpc_observations, ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation,
    RpcProofConfig, RpcQuorumDecision, RpcQuorumFindingCode, RpcQuorumReview,
};

fn expected_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        RpcCommitmentLevel::Finalized,
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

fn finding_codes(review: &RpcQuorumReview) -> Vec<RpcQuorumFindingCode> {
    review.findings.clone()
}

type RpcEquivocationSnapshot = (
    RpcQuorumDecision,
    Vec<RpcQuorumFindingCode>,
    u16,
    u16,
    u16,
    u16,
    u16,
);

#[test]
fn conflicting_rpc_signatures_are_disputed_not_agreement() {
    let observations = vec![
        observation("rpc-a", "sig-left-equivocation-111111111111", 40),
        observation("rpc-b", "sig-right-equivocation-2222222222", 41),
    ];

    let review = review_rpc_observations(
        &observations,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
        50,
    );

    assert_eq!(review.decision, RpcQuorumDecision::Disputed);
    assert_ne!(review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(review.accepted_observations, 2);
    assert_eq!(review.required_observations, 2);
    assert!(review.has_finding(RpcQuorumFindingCode::RpcEquivocation));
    assert!(!review.has_finding(RpcQuorumFindingCode::MissingEvidence));

    let evidence = review.to_evidence_bundle();
    assert_eq!(evidence.observation_count, 2);
    assert_eq!(evidence.required_observations, 2);
    assert_eq!(evidence.dispute_count, 1);
}

#[test]
fn same_source_equivocation_is_flagged_and_disputed() {
    let observations = vec![
        observation("rpc-a", "sig-source-first-111111111111", 40),
        observation("rpc-a", "sig-source-second-2222222222", 41),
        observation("rpc-b", "sig-source-second-2222222222", 42),
    ];

    let review = review_rpc_observations(
        &observations,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
        50,
    );

    assert_eq!(review.decision, RpcQuorumDecision::Disputed);
    assert_ne!(review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(review.accepted_observations, 2);
    assert!(review.has_finding(RpcQuorumFindingCode::SourceEquivocation));
    assert!(review.has_finding(RpcQuorumFindingCode::RpcEquivocation));

    let evidence = review.to_evidence_bundle();
    assert_eq!(evidence.observation_count, 2);
    assert_eq!(evidence.required_observations, 2);
    assert_eq!(evidence.dispute_count, 1);
}

#[test]
fn repeated_rpc_equivocation_reviews_are_deterministic() {
    let observations = vec![
        observation("rpc-a", "sig-left-repeat-111111111111", 40),
        observation("rpc-b", "sig-right-repeat-2222222222", 41),
        observation("rpc-c", "sig-right-repeat-2222222222", 42),
    ];

    let expected = expected_binding();
    let config = RpcProofConfig::new(2, 100);
    let mut first_snapshot: Option<RpcEquivocationSnapshot> = None;

    for _attempt in 0..64 {
        let review = review_rpc_observations(&observations, &expected, config, 50);
        let evidence = review.to_evidence_bundle();
        let snapshot = (
            review.decision,
            finding_codes(&review),
            review.accepted_observations,
            review.required_observations,
            evidence.observation_count,
            evidence.required_observations,
            evidence.dispute_count,
        );

        assert_eq!(snapshot.0, RpcQuorumDecision::Disputed);
        assert!(snapshot.1.contains(&RpcQuorumFindingCode::RpcEquivocation));
        assert_eq!(snapshot.6, 1);

        if let Some(previous) = &first_snapshot {
            assert_eq!(&snapshot, previous);
        } else {
            first_snapshot = Some(snapshot);
        }
    }
}

#[test]
fn clean_rpc_agreement_after_equivocation_case_can_still_accept() {
    let equivocated = vec![
        observation("rpc-a", "sig-left-before-clean-111111111111", 40),
        observation("rpc-b", "sig-right-before-clean-2222222222", 41),
    ];

    let expected = expected_binding();
    let config = RpcProofConfig::new(2, 100);

    let disputed_review = review_rpc_observations(&equivocated, &expected, config, 50);
    assert_eq!(disputed_review.decision, RpcQuorumDecision::Disputed);
    assert!(disputed_review.has_finding(RpcQuorumFindingCode::RpcEquivocation));

    let clean = vec![
        observation("rpc-a", "sig-clean-after-equivocation-3333333333", 45),
        observation("rpc-b", "sig-clean-after-equivocation-3333333333", 46),
    ];

    let clean_review = review_rpc_observations(&clean, &expected, config, 50);
    assert_eq!(clean_review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(clean_review.accepted_observations, 2);
    assert!(clean_review.has_finding(RpcQuorumFindingCode::SourceAccepted));
    assert!(!clean_review.has_finding(RpcQuorumFindingCode::RpcEquivocation));
    assert!(!clean_review.has_finding(RpcQuorumFindingCode::SourceEquivocation));

    let evidence = clean_review.to_evidence_bundle();
    assert_eq!(evidence.observation_count, 2);
    assert_eq!(evidence.required_observations, 2);
    assert_eq!(evidence.dispute_count, 0);
}
