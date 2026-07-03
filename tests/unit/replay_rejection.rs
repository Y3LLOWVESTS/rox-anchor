// RO:WHAT — Replay and binding rejection unit tests for rox-anchor-proof.
// RO:WHY — Proves local nonce, mint, and token-account mismatch review rejects unsafe evidence.
// RO:INTERACTS — crates/rox-anchor-proof replay, package, validate modules.
// RO:INVARIANTS — Replay rejection is local evidence review only, not finality or settlement.
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
    review_package_for_local_review_only, review_package_with_seen_nonces_for_local_review_only,
    LocalProofReviewDecision, ProofReviewFinding,
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
fn reused_nonce_is_replay_rejected() {
    let package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    let seen = [package.nonce.as_str()];

    let review =
        review_package_with_seen_nonces_for_local_review_only(&package, &expected, &seen);

    assert_eq!(review.decision, LocalProofReviewDecision::ReviewRejected);
    assert!(review.has_finding(ProofReviewFinding::ReusedNonce));
}

#[test]
fn mint_mismatch_is_review_rejected() {
    let mut package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    package.mint = "unexpected-local-fixture-mint".to_string();

    let review = review_package_for_local_review_only(&package, &expected);

    assert_eq!(review.decision, LocalProofReviewDecision::ReviewRejected);
    assert!(review.has_finding(ProofReviewFinding::MintMismatch));
}

#[test]
fn token_account_mismatch_is_review_rejected() {
    let mut package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    package.token_account = "unexpected-token-account".to_string();

    let review = review_package_for_local_review_only(&package, &expected);

    assert_eq!(review.decision, LocalProofReviewDecision::ReviewRejected);
    assert!(review.has_finding(ProofReviewFinding::TokenAccountMismatch));
}

// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local Phase 4 unit-test source does not authorize runtime.
