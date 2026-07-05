//! RO:WHAT — Coordinator review handoff into RPC proof and proof validation.
//! RO:WHY — Ensures coordinator decisions are derived from shared proof engines, not duplicate rules.
//! RO:INTERACTS — rox-anchor-proof and rox-anchor-rpc-proof.
//! RO:INVARIANTS — valid coordinator acceptance requires RPC agreement and proof acceptance.
//! RO:SECURITY — local review only; no live RPC, transaction submission, mint, burn, or settlement.
//! RO:TEST — covered by valid handoff, stale evidence, and simulation-gate tests.

use rox_anchor_core::{AnchorOperationalBlocker, AnchorOperationalPosture};
use rox_anchor_proof::{
    review_proof_package, ExpectedProofBinding, ProofPackage, ProofReview, ReplaySet,
    ReviewDecision,
};
use rox_anchor_rpc_proof::{
    review_rpc_observations, ExpectedRpcBinding, RpcObservation, RpcProofConfig, RpcQuorumDecision,
    RpcQuorumReview,
};

use crate::CoordinatorConfig;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorReviewRequest {
    pub package: ProofPackage,
    pub expected: ExpectedProofBinding,
    pub expected_rpc: ExpectedRpcBinding,
    pub observations: Vec<RpcObservation>,
    pub replay: ReplaySet,
}

impl CoordinatorReviewRequest {
    pub fn new(
        package: ProofPackage,
        expected: ExpectedProofBinding,
        expected_rpc: ExpectedRpcBinding,
        observations: Vec<RpcObservation>,
        replay: ReplaySet,
    ) -> Self {
        Self {
            package,
            expected,
            expected_rpc,
            observations,
            replay,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorDecisionStatus {
    Accepted,
    BlockedProof,
    RejectedProof,
    RejectedEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorDecision {
    pub status: CoordinatorDecisionStatus,
    pub rpc_review: RpcQuorumReview,
    pub proof_review: ProofReview,
}

impl CoordinatorDecision {
    pub fn is_accepted(&self) -> bool {
        self.status == CoordinatorDecisionStatus::Accepted
    }

    pub fn permits_transaction_simulation(&self) -> bool {
        self.is_accepted()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorFinalizationGateStatus {
    Permitted,
    CoordinatorNotAccepted,
    ChallengeBlocked,
    Halted,
    RecoveryBlocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorFinalizationGate {
    pub status: CoordinatorFinalizationGateStatus,
    pub coordinator_status: CoordinatorDecisionStatus,
    pub posture: AnchorOperationalPosture,
    pub permits_finalization: bool,
}

impl CoordinatorFinalizationGate {
    pub fn is_permitted(&self) -> bool {
        self.status == CoordinatorFinalizationGateStatus::Permitted
    }
}

pub fn review_coordinator_finalization_gate(
    decision: &CoordinatorDecision,
    posture: AnchorOperationalPosture,
) -> CoordinatorFinalizationGate {
    let status = if !decision.is_accepted() {
        CoordinatorFinalizationGateStatus::CoordinatorNotAccepted
    } else {
        match posture.primary_blocker() {
            AnchorOperationalBlocker::None => CoordinatorFinalizationGateStatus::Permitted,
            AnchorOperationalBlocker::Challenge => {
                CoordinatorFinalizationGateStatus::ChallengeBlocked
            }
            AnchorOperationalBlocker::Halt => CoordinatorFinalizationGateStatus::Halted,
            AnchorOperationalBlocker::Recovery => {
                CoordinatorFinalizationGateStatus::RecoveryBlocked
            }
        }
    };

    CoordinatorFinalizationGate {
        status,
        coordinator_status: decision.status,
        posture,
        permits_finalization: status == CoordinatorFinalizationGateStatus::Permitted,
    }
}

pub fn review_coordinator_request(
    request: &CoordinatorReviewRequest,
    config: CoordinatorConfig,
    current_slot: u64,
) -> CoordinatorDecision {
    review_coordinator_request_with_rpc_config(request, config.rpc, current_slot)
}

pub fn review_coordinator_request_with_rpc_config(
    request: &CoordinatorReviewRequest,
    rpc_config: RpcProofConfig,
    current_slot: u64,
) -> CoordinatorDecision {
    let rpc_review = review_rpc_observations(
        &request.observations,
        &request.expected_rpc,
        rpc_config,
        current_slot,
    );

    let mut package = request.package.clone();
    package.evidence = rpc_review.to_evidence_bundle();

    let proof_review = review_proof_package(&package, &request.expected, &request.replay);

    let status = if rpc_review.decision == RpcQuorumDecision::Rejected {
        CoordinatorDecisionStatus::RejectedEvidence
    } else {
        match proof_review.decision {
            ReviewDecision::Accepted => CoordinatorDecisionStatus::Accepted,
            ReviewDecision::Blocked => CoordinatorDecisionStatus::BlockedProof,
            ReviewDecision::Rejected => CoordinatorDecisionStatus::RejectedProof,
        }
    };

    CoordinatorDecision {
        status,
        rpc_review,
        proof_review,
    }
}
