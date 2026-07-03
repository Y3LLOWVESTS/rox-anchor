// RO:WHAT — Local halt/recovery review unit tests for rox-anchor-proof.
// RO:WHY — Exercises recovery posture without hidden value paths, runtime authority, wallet access, or settlement.
// RO:INTERACTS — crates/rox-anchor-proof recovery module through path-based test imports.
// RO:INVARIANTS — Recovery review is local posture only and never creates value movement.
// RO:SECURITY — No network, no wallet, no Solana/Anchor runtime, no bridge runtime, no value movement.
// RO:TEST — Future targeted local rustc/cargo test gate only when explicitly authorized.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local Phase 4 unit-test source does not authorize runtime.

#![forbid(unsafe_code)]

#[path = "../../crates/rox-anchor-proof/src/recovery.rs"]
mod recovery;

use recovery::{
    review_halt_recovery_for_local_review_only, HaltPosture, HaltRecoveryReviewDecision,
    HaltRecoveryReviewFinding, RecoveryActionIntent, RecoveryCaseKind, RecoveryPosture,
};

#[test]
fn no_recovery_needed_is_local_review_only() {
    let review = review_halt_recovery_for_local_review_only(
        HaltPosture::NotHalted,
        RecoveryPosture::NotRequired,
        RecoveryCaseKind::NotRequired,
    );

    assert_eq!(
        review.decision,
        HaltRecoveryReviewDecision::ValidForLocalReviewOnly
    );
    assert_eq!(review.action_intent, RecoveryActionIntent::NoActionRequired);
    assert!(!review.is_runtime_authorized());
    assert!(!review.is_hidden_value_path());
    assert!(!review.is_finality_claim());
    assert!(!review.is_settlement_claim());
}

#[test]
fn halted_case_stays_halted_for_review() {
    let review = review_halt_recovery_for_local_review_only(
        HaltPosture::Halted,
        RecoveryPosture::NotRequired,
        RecoveryCaseKind::HaltedForReview,
    );

    assert_eq!(review.decision, HaltRecoveryReviewDecision::Halted);
    assert_eq!(review.action_intent, RecoveryActionIntent::KeepHalted);
    assert!(review.has_finding(HaltRecoveryReviewFinding::Halted));
}

#[test]
fn recovery_required_queues_review() {
    let review = review_halt_recovery_for_local_review_only(
        HaltPosture::NotHalted,
        RecoveryPosture::ReviewRequired,
        RecoveryCaseKind::OperatorReviewRequired,
    );

    assert_eq!(
        review.decision,
        HaltRecoveryReviewDecision::EvidenceIncomplete
    );
    assert_eq!(review.action_intent, RecoveryActionIntent::QueueReview);
    assert!(review.has_finding(HaltRecoveryReviewFinding::RecoveryReviewRequired));
    assert!(review.has_finding(HaltRecoveryReviewFinding::OperatorReviewRequired));
}

#[test]
fn evidence_mismatch_rejects_local_review() {
    let review = review_halt_recovery_for_local_review_only(
        HaltPosture::NotHalted,
        RecoveryPosture::NotRequired,
        RecoveryCaseKind::EvidenceMismatch,
    );

    assert_eq!(review.decision, HaltRecoveryReviewDecision::ReviewRejected);
    assert_eq!(review.action_intent, RecoveryActionIntent::RejectEvidence);
    assert!(review.has_finding(HaltRecoveryReviewFinding::EvidenceMismatch));
}
