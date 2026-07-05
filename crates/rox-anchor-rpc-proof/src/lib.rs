//! RO:WHAT — Local RPC evidence and quorum classification for ROX Anchor.
//! RO:WHY — Models RPC agreement, missing evidence, stale evidence, and equivocation before live RPC is added.
//! RO:INTERACTS — rox-anchor-core typed bindings and rox-anchor-proof EvidenceBundle handoff.
//! RO:INVARIANTS — local classification only; no network calls or separate finality rules.
//! RO:SECURITY — no live RPC submission, wallet calls, deployment, minting, burning, or settlement.
//! RO:TEST — crate-local tests cover agreement, missing evidence, equivocation, stale evidence, and binding mismatch.

#![forbid(unsafe_code)]

pub mod audit;
pub mod commitment;
pub mod config;
pub mod quorum;
pub mod readiness;
pub mod redaction;
pub mod rpc;

pub use audit::*;
pub use commitment::*;
pub use config::*;
pub use quorum::*;
pub use readiness::*;
pub use redaction::*;
pub use rpc::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};

    fn expected_binding() -> ExpectedRpcBinding {
        ExpectedRpcBinding::new(
            ClusterId::new("localnet").unwrap(),
            ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
            MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
            TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
            OperationId::new("op-roc-to-rox-0001").unwrap(),
            RpcCommitmentLevel::Confirmed,
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

    #[test]
    fn matching_rpc_observations_agree_and_map_to_proof_evidence() {
        let observations = vec![
            observation("rpc-a", "sig-same-111111111111", 40),
            observation("rpc-b", "sig-same-111111111111", 41),
        ];

        let review = review_rpc_observations(
            &observations,
            &expected_binding(),
            RpcProofConfig::new(2, 100),
            50,
        );

        assert_eq!(review.decision, RpcQuorumDecision::Agreement);
        assert_eq!(review.accepted_observations, 2);
        assert!(review.has_finding(RpcQuorumFindingCode::SourceAccepted));

        let evidence = review.to_evidence_bundle();
        assert_eq!(evidence.observation_count, 2);
        assert_eq!(evidence.required_observations, 2);
        assert_eq!(evidence.dispute_count, 0);
    }

    #[test]
    fn too_few_matching_sources_is_missing_evidence() {
        let observations = vec![observation("rpc-a", "sig-same-111111111111", 40)];

        let review = review_rpc_observations(
            &observations,
            &expected_binding(),
            RpcProofConfig::new(2, 100),
            50,
        );

        assert_eq!(review.decision, RpcQuorumDecision::MissingEvidence);
        assert_eq!(review.accepted_observations, 1);
        assert!(review.has_finding(RpcQuorumFindingCode::MissingEvidence));
    }

    #[test]
    fn conflicting_signatures_are_disputed_evidence() {
        let observations = vec![
            observation("rpc-a", "sig-left-111111111111", 40),
            observation("rpc-b", "sig-right-2222222222", 41),
        ];

        let review = review_rpc_observations(
            &observations,
            &expected_binding(),
            RpcProofConfig::new(2, 100),
            50,
        );

        assert_eq!(review.decision, RpcQuorumDecision::Disputed);
        assert!(review.has_finding(RpcQuorumFindingCode::RpcEquivocation));

        let evidence = review.to_evidence_bundle();
        assert_eq!(evidence.dispute_count, 1);
    }

    #[test]
    fn stale_observation_is_rejected() {
        let observations = vec![
            observation("rpc-a", "sig-same-111111111111", 10),
            observation("rpc-b", "sig-same-111111111111", 11),
        ];

        let review = review_rpc_observations(
            &observations,
            &expected_binding(),
            RpcProofConfig::new(2, 5),
            50,
        );

        assert_eq!(review.decision, RpcQuorumDecision::Rejected);
        assert!(review.has_finding(RpcQuorumFindingCode::StaleEvidence));
    }

    #[test]
    fn binding_mismatch_is_rejected() {
        let mut bad = observation("rpc-a", "sig-same-111111111111", 40);
        bad.cluster = ClusterId::new("wrong-cluster").unwrap();

        let review =
            review_rpc_observations(&[bad], &expected_binding(), RpcProofConfig::new(1, 100), 50);

        assert_eq!(review.decision, RpcQuorumDecision::Rejected);
        assert!(review.has_finding(RpcQuorumFindingCode::ClusterMismatch));
    }

    #[test]
    fn readiness_rejects_zero_thresholds() {
        let readiness = review_rpc_proof_readiness(RpcProofConfig::new(0, 0));

        assert!(!readiness.ready);
        assert!(readiness.has_finding(RpcProofReadinessFinding::MissingRequiredObservations));
        assert!(readiness.has_finding(RpcProofReadinessFinding::MissingStaleSlotWindow));
    }

    #[test]
    fn redaction_shortens_signatures_without_hiding_shape() {
        let redacted = redact_signature("abcdef0123456789");

        assert_eq!(redacted, "abcdef01...6789");
    }
}
