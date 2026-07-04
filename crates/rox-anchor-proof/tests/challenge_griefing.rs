// RO:WHAT — Chaos test for repeated challenge pressure against local proof review.
// RO:WHY — Proves challenge griefing cannot accidentally turn into accepted finality.
// RO:INTERACTS — rox-anchor-proof challenge review, proof package review, and rox-anchor-core challenge posture.
// RO:INVARIANTS — open/accepted challenges block acceptance deterministically; rejected/expired challenges do not.
// RO:SECURITY — local proof review only; no live RPC, wallet calls, minting, burning, settlement, staking, liquidity, or deployment.
// RO:TEST — cargo test -p rox-anchor-proof --test challenge_griefing.

#![forbid(unsafe_code)]

use rox_anchor_core::{AnchorLifecycleState, ChallengePosture};
use rox_anchor_proof::{
    fixtures, review_proof_package, ProofFindingCode, ProofReview, ReplaySet, ReviewDecision,
};

fn finding_codes(review: &ProofReview) -> Vec<ProofFindingCode> {
    review.findings.iter().map(|finding| finding.code).collect()
}

#[test]
fn repeated_open_challenge_reviews_remain_blocked_and_deterministic() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();
    let mut first_snapshot: Option<(ReviewDecision, AnchorLifecycleState, Vec<ProofFindingCode>)> =
        None;

    for _attempt in 0..64 {
        let mut package = fixtures::valid_package();
        package.challenge_posture = ChallengePosture::Open;

        let review = review_proof_package(&package, &expected, &replay);
        let snapshot = (
            review.decision,
            review.lifecycle_state,
            finding_codes(&review),
        );

        assert_eq!(snapshot.0, ReviewDecision::Blocked);
        assert_eq!(snapshot.1, AnchorLifecycleState::ChallengeOpen);
        assert_eq!(snapshot.2, vec![ProofFindingCode::ChallengeOpen]);
        assert!(!snapshot.2.contains(&ProofFindingCode::PackageAccepted));

        if let Some(previous) = &first_snapshot {
            assert_eq!(&snapshot, previous);
        } else {
            first_snapshot = Some(snapshot);
        }
    }
}

#[test]
fn accepted_challenge_blocks_even_with_satisfied_evidence() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();

    let mut package = fixtures::valid_package();
    package.challenge_posture = ChallengePosture::Accepted;

    let review = review_proof_package(&package, &expected, &replay);
    let codes = finding_codes(&review);

    assert_eq!(review.decision, ReviewDecision::Blocked);
    assert_eq!(codes, vec![ProofFindingCode::ChallengeAccepted]);
    assert!(!codes.contains(&ProofFindingCode::PackageAccepted));
}

#[test]
fn resolved_or_expired_challenge_postures_do_not_block_clean_package() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();

    for posture in [ChallengePosture::Rejected, ChallengePosture::Expired] {
        let mut package = fixtures::valid_package();
        package.challenge_posture = posture;

        let review = review_proof_package(&package, &expected, &replay);
        let codes = finding_codes(&review);

        assert_eq!(review.decision, ReviewDecision::Accepted);
        assert_eq!(
            review.lifecycle_state,
            AnchorLifecycleState::FinalityEligible
        );
        assert_eq!(codes, vec![ProofFindingCode::PackageAccepted]);
    }
}
