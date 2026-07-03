// RO:WHAT — Local quorum/evidence posture unit tests for rox-anchor-proof.
// RO:WHY — Exercises multi-source evidence posture without RPC calls, network authority, wallet access, or settlement.
// RO:INTERACTS — crates/rox-anchor-proof quorum module through path-based test imports.
// RO:INVARIANTS — Quorum evidence review is local posture only; a single observation is not authority.
// RO:SECURITY — No network, no wallet, no Solana/Anchor runtime, no bridge runtime, no value movement.
// RO:TEST — Future targeted local rustc/cargo test gate only when explicitly authorized.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local Phase 4 unit-test source does not authorize runtime.

#![forbid(unsafe_code)]

#[path = "../../crates/rox-anchor-proof/src/quorum.rs"]
mod quorum;

use quorum::{
    review_quorum_evidence_counts_for_local_review_only, QuorumEvidenceCount,
    QuorumEvidenceReviewDecision, QuorumEvidenceReviewFinding,
};

#[test]
fn matching_minimum_evidence_is_present_for_local_review_only() {
    let review = review_quorum_evidence_counts_for_local_review_only(
        QuorumEvidenceCount::new(3, 0, 0, 2),
    );

    assert_eq!(review.decision, QuorumEvidenceReviewDecision::EvidencePresent);
    assert!(review.has_finding(QuorumEvidenceReviewFinding::MatchingEvidencePresent));
    assert!(!review.is_runtime_authorized());
    assert!(!review.is_finality_claim());
    assert!(!review.is_settlement_claim());
}

#[test]
fn minimum_not_met_is_evidence_incomplete() {
    let review = review_quorum_evidence_counts_for_local_review_only(
        QuorumEvidenceCount::new(1, 0, 0, 2),
    );

    assert_eq!(
        review.decision,
        QuorumEvidenceReviewDecision::EvidenceIncomplete
    );
    assert!(review.has_finding(QuorumEvidenceReviewFinding::MinimumNotMet));
}

#[test]
fn disputed_evidence_is_quorum_disputed() {
    let review = review_quorum_evidence_counts_for_local_review_only(
        QuorumEvidenceCount::new(2, 1, 0, 2),
    );

    assert_eq!(review.decision, QuorumEvidenceReviewDecision::QuorumDisputed);
    assert!(review.has_finding(QuorumEvidenceReviewFinding::DisputedEvidence));
}

#[test]
fn single_observation_is_not_authority() {
    let review = review_quorum_evidence_counts_for_local_review_only(
        QuorumEvidenceCount::new(1, 0, 0, 1),
    );

    assert_eq!(review.decision, QuorumEvidenceReviewDecision::EvidencePresent);
    assert!(review.has_finding(QuorumEvidenceReviewFinding::SingleObservationNotAuthority));
}
