//! RO:WHAT — Local coordinator model for ROX Anchor review work.
//! RO:WHY — Assembles local RPC observations, queues review requests, and hands decisions to rox-anchor-proof.
//! RO:INTERACTS — rox-anchor-core, rox-anchor-proof, and rox-anchor-rpc-proof.
//! RO:INVARIANTS — coordinator orchestrates review; it does not invent bridge finality or mutate value.
//! RO:SECURITY — no live RPC calls, wallet calls, transaction submission, minting, burning, or settlement.
//! RO:TEST — crate-local tests cover valid handoff, stale evidence, duplicate queue items, and readiness.

#![forbid(unsafe_code)]

pub mod audit;
pub mod config;
pub mod decision;
pub mod observer;
pub mod queue;
pub mod readiness;
pub mod redaction;

pub use audit::*;
pub use config::*;
pub use decision::*;
pub use observer::*;
pub use queue::*;
pub use readiness::*;
pub use redaction::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
    use rox_anchor_proof::{fixtures, ReplaySet, ReviewDecision};
    use rox_anchor_rpc_proof::{
        ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision,
        RpcQuorumFindingCode,
    };

    fn config() -> CoordinatorConfig {
        CoordinatorConfig::new(2, 100, 4)
    }

    fn expected_rpc_binding() -> ExpectedRpcBinding {
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

    fn request() -> CoordinatorReviewRequest {
        CoordinatorReviewRequest::new(
            fixtures::valid_package(),
            fixtures::expected_proof_binding(),
            expected_rpc_binding(),
            vec![
                observation("rpc-a", "sig-same-111111111111", 40),
                observation("rpc-b", "sig-same-111111111111", 41),
            ],
            ReplaySet::default(),
        )
    }

    #[test]
    fn valid_observations_handoff_to_proof_and_accept() {
        let decision = review_coordinator_request(&request(), config(), 50);

        assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
        assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Agreement);
        assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);
        assert!(decision.is_accepted());
    }

    #[test]
    fn stale_rpc_evidence_prevents_acceptance() {
        let stale_request = CoordinatorReviewRequest::new(
            fixtures::valid_package(),
            fixtures::expected_proof_binding(),
            expected_rpc_binding(),
            vec![
                observation("rpc-a", "sig-same-111111111111", 10),
                observation("rpc-b", "sig-same-111111111111", 11),
            ],
            ReplaySet::default(),
        );

        let decision =
            review_coordinator_request(&stale_request, CoordinatorConfig::new(2, 5, 4), 50);

        assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
        assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
        assert!(decision
            .rpc_review
            .has_finding(RpcQuorumFindingCode::StaleEvidence));
        assert!(!decision.is_accepted());
    }

    #[test]
    fn duplicate_operation_is_rejected_by_queue() {
        let mut queue = CoordinatorQueue::new(config());
        let first = queue.push(request());
        let second = queue.push(request());

        assert_eq!(first, Ok(()));
        assert_eq!(second, Err(CoordinatorQueueError::DuplicateOperation));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn queue_capacity_is_enforced() {
        let mut queue = CoordinatorQueue::new(CoordinatorConfig::new(2, 100, 1));
        let first = queue.push(request());

        let mut second_request = request();
        second_request.package.operation_id = OperationId::new("op-roc-to-rox-0002").unwrap();
        let second = queue.push(second_request);

        assert_eq!(first, Ok(()));
        assert_eq!(second, Err(CoordinatorQueueError::QueueFull));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn readiness_requires_queue_capacity_and_rpc_config() {
        let readiness = review_coordinator_readiness(CoordinatorConfig::new(0, 0, 0));

        assert!(!readiness.ready);
        assert!(readiness.has_finding(CoordinatorReadinessFinding::MissingQueueCapacity));
        assert!(readiness.has_finding(CoordinatorReadinessFinding::RpcProofNotReady));
    }

    #[test]
    fn redacted_report_keeps_decision_shape() {
        let decision = review_coordinator_request(&request(), config(), 50);
        let report = redacted_coordinator_report(&decision);

        assert!(report.contains("coordinator_status=Accepted"));
        assert!(report.contains("rpc_decision=Agreement"));
        assert!(report.contains("proof_decision=Accepted"));
    }
}
