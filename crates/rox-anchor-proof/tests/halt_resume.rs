// RO:WHAT — Chaos test for halt/resume posture changes in local proof review.
// RO:WHY — Proves halt pressure blocks acceptance deterministically and clearing halt restores clean-package review.
// RO:INTERACTS — rox-anchor-proof review, rox-anchor-core halt/recovery postures, lifecycle states, and findings.
// RO:INVARIANTS — halted packages block; active halt does not block; recovery-required still blocks after halt clears.
// RO:SECURITY — local proof review only; no live RPC, wallet calls, transaction submission, minting, burning, settlement, staking, liquidity, or deployment.
// RO:TEST — cargo test -p rox-anchor-proof --test halt_resume.

#![forbid(unsafe_code)]

use rox_anchor_core::{AnchorLifecycleState, HaltPosture, RecoveryPosture};
use rox_anchor_proof::{
    fixtures, review_proof_package, ProofFindingCode, ProofReview, ReplaySet, ReviewDecision,
};

fn finding_codes(review: &ProofReview) -> Vec<ProofFindingCode> {
    review.findings.iter().map(|finding| finding.code).collect()
}

#[test]
fn repeated_halted_reviews_remain_blocked_and_deterministic() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();
    let mut first_snapshot: Option<(ReviewDecision, AnchorLifecycleState, Vec<ProofFindingCode>)> =
        None;

    for _attempt in 0..64 {
        let mut package = fixtures::valid_package();
        package.halt_posture = HaltPosture::Halted;

        let review = review_proof_package(&package, &expected, &replay);
        let snapshot = (
            review.decision,
            review.lifecycle_state,
            finding_codes(&review),
        );

        assert_eq!(snapshot.0, ReviewDecision::Blocked);
        assert_eq!(snapshot.1, AnchorLifecycleState::Halted);
        assert_eq!(snapshot.2, vec![ProofFindingCode::Halted]);
        assert!(!snapshot.2.contains(&ProofFindingCode::PackageAccepted));

        if let Some(previous) = &first_snapshot {
            assert_eq!(&snapshot, previous);
        } else {
            first_snapshot = Some(snapshot);
        }
    }
}

#[test]
fn active_halt_posture_restores_acceptance_for_clean_package() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();

    let mut halted = fixtures::valid_package();
    halted.halt_posture = HaltPosture::Halted;

    let halted_review = review_proof_package(&halted, &expected, &replay);
    assert_eq!(halted_review.decision, ReviewDecision::Blocked);
    assert_eq!(halted_review.lifecycle_state, AnchorLifecycleState::Halted);
    assert_eq!(
        finding_codes(&halted_review),
        vec![ProofFindingCode::Halted]
    );

    let mut resumed = halted;
    resumed.halt_posture = HaltPosture::Active;

    let resumed_review = review_proof_package(&resumed, &expected, &replay);
    assert_eq!(resumed_review.decision, ReviewDecision::Accepted);
    assert_eq!(
        resumed_review.lifecycle_state,
        AnchorLifecycleState::FinalityEligible
    );
    assert_eq!(
        finding_codes(&resumed_review),
        vec![ProofFindingCode::PackageAccepted]
    );
}

#[test]
fn recovery_required_still_blocks_after_halt_is_cleared() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();

    let mut package = fixtures::valid_package();
    package.halt_posture = HaltPosture::Active;
    package.recovery_posture = RecoveryPosture::Required;

    let review = review_proof_package(&package, &expected, &replay);
    let codes = finding_codes(&review);

    assert_eq!(review.decision, ReviewDecision::Blocked);
    assert_eq!(
        review.lifecycle_state,
        AnchorLifecycleState::RecoveryRequired
    );
    assert_eq!(codes, vec![ProofFindingCode::RecoveryRequired]);
    assert!(!codes.contains(&ProofFindingCode::PackageAccepted));
}

#[test]
fn active_halt_and_no_recovery_remain_deterministically_accepted() {
    let expected = fixtures::expected_proof_binding();
    let replay = ReplaySet::default();
    let mut first_snapshot: Option<(ReviewDecision, AnchorLifecycleState, Vec<ProofFindingCode>)> =
        None;

    for _attempt in 0..64 {
        let mut package = fixtures::valid_package();
        package.halt_posture = HaltPosture::Active;
        package.recovery_posture = RecoveryPosture::NotRequired;

        let review = review_proof_package(&package, &expected, &replay);
        let snapshot = (
            review.decision,
            review.lifecycle_state,
            finding_codes(&review),
        );

        assert_eq!(snapshot.0, ReviewDecision::Accepted);
        assert_eq!(snapshot.1, AnchorLifecycleState::FinalityEligible);
        assert_eq!(snapshot.2, vec![ProofFindingCode::PackageAccepted]);

        if let Some(previous) = &first_snapshot {
            assert_eq!(&snapshot, previous);
        } else {
            first_snapshot = Some(snapshot);
        }
    }
}
