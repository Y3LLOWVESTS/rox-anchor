// RO:WHAT — Local state-transition review unit tests for rox-anchor-proof.
// RO:WHY — Exercises the non-value proof state machine without runtime authority or settlement claims.
// RO:INTERACTS — crates/rox-anchor-proof validate, package, replay, quorum, challenge, recovery modules.
// RO:INVARIANTS — State transitions are local review only; LocalReviewConsistent is not finality.
// RO:SECURITY — No network, no wallet, no Solana/Anchor runtime, no bridge runtime, no value movement.
// RO:TEST — Future targeted local rustc/cargo test gate only when explicitly authorized.

#![forbid(unsafe_code)]

#[path = "../../crates/rox-anchor-proof/src/challenge.rs"]
mod challenge;
#[path = "../../crates/rox-anchor-proof/src/package.rs"]
mod package;
#[path = "../../crates/rox-anchor-proof/src/quorum.rs"]
mod quorum;
#[path = "../../crates/rox-anchor-proof/src/recovery.rs"]
mod recovery;
#[path = "../../crates/rox-anchor-proof/src/replay.rs"]
mod replay;
#[path = "../../crates/rox-anchor-proof/src/validate.rs"]
mod validate;

use challenge::ChallengeGatePosture;
use package::{CommitmentReviewLevel, EvidencePosture, ProofDirection, ProofPackageShape};
use quorum::QuorumObservationPosture;
use recovery::{HaltPosture, RecoveryPosture};
use replay::ExpectedProofBinding;
use validate::{
    review_package_for_local_review_only, review_package_state_transition_for_local_review_only,
    review_state_transition_for_local_review_only, LocalProofReviewDecision, LocalProofState,
    StateTransitionIntent, StateTransitionReviewDecision, StateTransitionReviewFinding,
};

fn valid_package() -> ProofPackageShape {
    ProofPackageShape {
        schema_version: "rox-anchor-proof-package-fixture-v1".to_string(),
        source_domain: "internal-roc-local-fixture".to_string(),
        target_domain: "rox-anchor-local-fixture".to_string(),
        direction: ProofDirection::RocToRox,
        operation_id: "op_fixture_valid_0001".to_string(),
        idempotency_key: "idem_fixture_valid_0001".to_string(),
        nonce: "nonce_fixture_valid_0001".to_string(),
        cluster: "local-fixture-cluster".to_string(),
        program_id: "local-fixture-program".to_string(),
        mint: "local-fixture-mint".to_string(),
        token_account: "local-fixture-token-account".to_string(),
        commitment_level: CommitmentReviewLevel::ReviewOnly,
        evidence_posture: EvidencePosture::ConsistentForLocalReviewOnly,
        quorum_posture: QuorumObservationPosture::EvidencePresent,
        challenge_status: ChallengeGatePosture::Closed,
        halt_status: HaltPosture::NotHalted,
        recovery_status: RecoveryPosture::NotRequired,
    }
}

#[test]
fn proof_packaged_can_become_local_review_consistent_when_review_is_valid() {
    let package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    let proof_review = review_package_for_local_review_only(&package, &expected);
    assert_eq!(
        proof_review.decision,
        LocalProofReviewDecision::ValidForLocalReviewOnly
    );

    let transition = review_state_transition_for_local_review_only(
        StateTransitionIntent::new(
            LocalProofState::ProofPackaged,
            LocalProofState::LocalReviewConsistent,
        ),
        &proof_review,
    );

    assert_eq!(
        transition.decision,
        StateTransitionReviewDecision::ValidForLocalReviewOnly
    );
    assert!(transition.has_finding(StateTransitionReviewFinding::LocalReviewTransitionAccepted));
    assert!(!transition.is_runtime_authorized());
    assert!(!transition.is_finality_claim());
    assert!(!transition.is_settlement_claim());
}

#[test]
fn halted_state_cannot_jump_to_local_review_consistent() {
    let package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    let proof_review = review_package_for_local_review_only(&package, &expected);

    let transition = review_state_transition_for_local_review_only(
        StateTransitionIntent::new(
            LocalProofState::Halted,
            LocalProofState::LocalReviewConsistent,
        ),
        &proof_review,
    );

    assert_eq!(
        transition.decision,
        StateTransitionReviewDecision::ReviewRejected
    );
    assert!(transition.has_finding(StateTransitionReviewFinding::HaltedStateTransitionRejected));
}

#[test]
fn recovery_queue_cannot_bypass_review_to_local_consistency() {
    let package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    let proof_review = review_package_for_local_review_only(&package, &expected);

    let transition = review_state_transition_for_local_review_only(
        StateTransitionIntent::new(
            LocalProofState::RecoveryQueued,
            LocalProofState::LocalReviewConsistent,
        ),
        &proof_review,
    );

    assert_eq!(
        transition.decision,
        StateTransitionReviewDecision::ReviewRejected
    );
    assert!(transition.has_finding(StateTransitionReviewFinding::RecoveryBypassRejected));
}

#[test]
fn replay_rejected_package_cannot_become_local_review_consistent() {
    let mut package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    package.nonce = "different-nonce".to_string();

    let transition = review_package_state_transition_for_local_review_only(
        &package,
        &expected,
        &[],
        StateTransitionIntent::new(
            LocalProofState::ProofPackaged,
            LocalProofState::LocalReviewConsistent,
        ),
    );

    assert_eq!(
        transition.decision,
        StateTransitionReviewDecision::ReviewRejected
    );
    assert!(transition.has_finding(StateTransitionReviewFinding::ReplayRejectedTransition));
}

// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local Phase 4 unit-test source does not authorize runtime.
