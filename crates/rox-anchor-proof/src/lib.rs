//! RO:WHAT — Deterministic local proof-review engine for ROX Anchor.
//! RO:WHY — Reviews proof packages before CLI, coordinator, RPC proof, relayer, or Anchor state code trusts them.
//! RO:INTERACTS — rox-anchor-core IDs/states, package validation, replay, quorum, challenge, halt, and recovery helpers.
//! RO:INVARIANTS — reject replay; reject binding mismatches; block unsafe challenge/halt/recovery cases.
//! RO:SECURITY — local validation only; no wallet calls, live RPC submission, minting, burning, or settlement.
//! RO:TEST — crate-local unit tests cover valid, mismatch, replay, quorum, challenge, halt, and recovery outcomes.

#![forbid(unsafe_code)]

pub mod challenge;
pub mod fixtures;
pub mod package;
pub mod quorum;
pub mod recovery;
pub mod replay;
pub mod validate;

pub use challenge::*;
pub use package::*;
pub use quorum::*;
pub use recovery::*;
pub use replay::*;
pub use validate::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rox_anchor_core::{
        AnchorDirection, ChallengePosture, ClusterId, DomainId, HaltPosture, IdempotencyKey,
        MintId, Nonce, OperationId, ProgramId, RecoveryPosture, TokenAccountId,
    };

    fn finding_codes(review: &ProofReview) -> Vec<ProofFindingCode> {
        review.findings.iter().map(|finding| finding.code).collect()
    }

    #[test]
    fn valid_fixture_is_accepted_and_finality_eligible() {
        let package = fixtures::valid_package();
        let expected = fixtures::expected_proof_binding();
        let replay = ReplaySet::default();

        let review = review_proof_package(&package, &expected, &replay);

        assert_eq!(review.decision, ReviewDecision::Accepted);
        assert_eq!(
            review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::FinalityEligible
        );
        assert_eq!(
            finding_codes(&review),
            vec![ProofFindingCode::PackageAccepted]
        );
    }

    #[test]
    fn binding_mismatches_are_rejected_in_deterministic_order() {
        let mut package = fixtures::valid_package();
        package.binding.source_domain = DomainId::new("wrong-source").unwrap();
        package.binding.target_domain = DomainId::new("wrong-target").unwrap();
        package.binding.direction = AnchorDirection::RoxToRoc;
        package.binding.cluster = ClusterId::new("wrong-cluster").unwrap();
        package.binding.program_id =
            ProgramId::new("WrongProgram111111111111111111111111").unwrap();
        package.binding.mint = MintId::new("WrongMint111111111111111111111111111111").unwrap();
        package.binding.token_account =
            TokenAccountId::new("WrongTokenAccount111111111111111111").unwrap();
        package.operation_id = OperationId::new("wrong-operation").unwrap();
        package.idempotency_key = IdempotencyKey::new("wrong-idempotency").unwrap();
        package.nonce = Nonce::new("wrong-nonce").unwrap();

        let expected = fixtures::expected_proof_binding();
        let replay = ReplaySet::default();

        let review = review_proof_package(&package, &expected, &replay);

        assert_eq!(review.decision, ReviewDecision::Rejected);
        assert_eq!(
            review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::Failed
        );
        assert_eq!(
            finding_codes(&review),
            vec![
                ProofFindingCode::SourceDomainMismatch,
                ProofFindingCode::TargetDomainMismatch,
                ProofFindingCode::DirectionMismatch,
                ProofFindingCode::ClusterMismatch,
                ProofFindingCode::ProgramIdMismatch,
                ProofFindingCode::MintMismatch,
                ProofFindingCode::TokenAccountMismatch,
                ProofFindingCode::OperationIdMismatch,
                ProofFindingCode::IdempotencyKeyMismatch,
                ProofFindingCode::NonceMismatch,
            ]
        );
    }

    #[test]
    fn replayed_operation_id_idempotency_key_or_nonce_is_rejected() {
        let package = fixtures::valid_package();
        let expected = fixtures::expected_proof_binding();
        let replay = ReplaySet::from_package(&package);

        let review = review_proof_package(&package, &expected, &replay);

        assert_eq!(review.decision, ReviewDecision::Rejected);
        assert_eq!(
            review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::Failed
        );
        assert_eq!(
            finding_codes(&review),
            vec![
                ProofFindingCode::ReplayOperationId,
                ProofFindingCode::ReplayIdempotencyKey,
                ProofFindingCode::ReplayNonce,
            ]
        );
    }

    #[test]
    fn missing_or_disputed_evidence_blocks_acceptance() {
        let expected = fixtures::expected_proof_binding();
        let replay = ReplaySet::default();

        let mut missing = fixtures::valid_package();
        missing.evidence = EvidenceBundle::new(0, 2, 0);
        let missing_review = review_proof_package(&missing, &expected, &replay);
        assert_eq!(missing_review.decision, ReviewDecision::Blocked);
        assert_eq!(
            missing_review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::EvidenceIncomplete
        );
        assert_eq!(
            finding_codes(&missing_review),
            vec![ProofFindingCode::EvidenceMissing]
        );

        let mut disputed = fixtures::valid_package();
        disputed.evidence = EvidenceBundle::new(2, 2, 1);
        let disputed_review = review_proof_package(&disputed, &expected, &replay);
        assert_eq!(disputed_review.decision, ReviewDecision::Blocked);
        assert_eq!(
            disputed_review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::QuorumDisputed
        );
        assert_eq!(
            finding_codes(&disputed_review),
            vec![ProofFindingCode::QuorumDisputed]
        );
    }

    #[test]
    fn challenge_halt_and_recovery_postures_block_acceptance() {
        let expected = fixtures::expected_proof_binding();
        let replay = ReplaySet::default();

        let mut challenged = fixtures::valid_package();
        challenged.challenge_posture = ChallengePosture::Open;
        let challenge_review = review_proof_package(&challenged, &expected, &replay);
        assert_eq!(challenge_review.decision, ReviewDecision::Blocked);
        assert_eq!(
            challenge_review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::ChallengeOpen
        );
        assert_eq!(
            finding_codes(&challenge_review),
            vec![ProofFindingCode::ChallengeOpen]
        );

        let mut halted = fixtures::valid_package();
        halted.halt_posture = HaltPosture::Halted;
        let halt_review = review_proof_package(&halted, &expected, &replay);
        assert_eq!(halt_review.decision, ReviewDecision::Blocked);
        assert_eq!(
            halt_review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::Halted
        );
        assert_eq!(finding_codes(&halt_review), vec![ProofFindingCode::Halted]);

        let mut recovery = fixtures::valid_package();
        recovery.recovery_posture = RecoveryPosture::Required;
        let recovery_review = review_proof_package(&recovery, &expected, &replay);
        assert_eq!(recovery_review.decision, ReviewDecision::Blocked);
        assert_eq!(
            recovery_review.lifecycle_state,
            rox_anchor_core::AnchorLifecycleState::RecoveryRequired
        );
        assert_eq!(
            finding_codes(&recovery_review),
            vec![ProofFindingCode::RecoveryRequired]
        );
    }
}
