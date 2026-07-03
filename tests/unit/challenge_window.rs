// RO:WHAT — Local challenge-window review unit tests for rox-anchor-proof.
// RO:WHY — Exercises challenge timing and posture review without runtime, RPC, wallet, or settlement authority.
// RO:INTERACTS — crates/rox-anchor-proof challenge module through path-based test imports.
// RO:INVARIANTS — Challenge-window review is evidence posture only; local review is not finality.
// RO:SECURITY — No network, no wallet, no Solana/Anchor runtime, no bridge runtime, no value movement.
// RO:TEST — Future targeted local rustc/cargo test gate only when explicitly authorized.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local Phase 4 unit-test source does not authorize runtime.

#![forbid(unsafe_code)]

#[path = "../../crates/rox-anchor-proof/src/challenge.rs"]
mod challenge;

use challenge::{
    review_challenge_window_for_local_review_only, ChallengeGatePosture,
    ChallengeWindowClockFinding, ChallengeWindowReviewDecision, ChallengeWindowTiming,
};

#[test]
fn closed_challenge_window_is_local_review_only() {
    let review = review_challenge_window_for_local_review_only(
        ChallengeGatePosture::Closed,
        ChallengeWindowTiming::unopened(100),
    );

    assert_eq!(
        review.decision,
        ChallengeWindowReviewDecision::ValidForLocalReviewOnly
    );
    assert!(review.has_finding(ChallengeWindowClockFinding::ChallengeResolved));
    assert!(!review.is_runtime_authorized());
    assert!(!review.is_finality_claim());
    assert!(!review.is_settlement_claim());
}

#[test]
fn open_challenge_before_delay_remains_open() {
    let review = review_challenge_window_for_local_review_only(
        ChallengeGatePosture::Open,
        ChallengeWindowTiming::opened(10, 12, 5, 20),
    );

    assert_eq!(review.decision, ChallengeWindowReviewDecision::ChallengeOpen);
    assert!(review.has_finding(ChallengeWindowClockFinding::WindowOpen));
    assert!(review.has_finding(ChallengeWindowClockFinding::ReviewDelayNotElapsed));
    assert!(!review.review_delay_elapsed());
}

#[test]
fn expired_challenge_is_evidence_incomplete_not_finality() {
    let review = review_challenge_window_for_local_review_only(
        ChallengeGatePosture::Open,
        ChallengeWindowTiming::opened(10, 40, 5, 20),
    );

    assert_eq!(
        review.decision,
        ChallengeWindowReviewDecision::EvidenceIncomplete
    );
    assert!(review.has_finding(ChallengeWindowClockFinding::WindowExpired));
    assert!(review.is_expired());
    assert!(!review.is_finality_claim());
}

#[test]
fn accepted_challenge_rejects_local_review() {
    let review = review_challenge_window_for_local_review_only(
        ChallengeGatePosture::Accepted,
        ChallengeWindowTiming::opened(10, 15, 5, 20),
    );

    assert_eq!(review.decision, ChallengeWindowReviewDecision::ReviewRejected);
    assert!(review.has_finding(ChallengeWindowClockFinding::ChallengeAcceptedRejected));
}
