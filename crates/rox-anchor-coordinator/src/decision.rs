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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorIncidentStage {
    AfterProofAcceptanceBeforeSimulation,
    AfterSimulationBeforeSubmission,
    AfterCappedTestnetSubmission,
}

impl CoordinatorIncidentStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AfterProofAcceptanceBeforeSimulation => {
                "after_proof_acceptance_before_simulation"
            }
            Self::AfterSimulationBeforeSubmission => "after_simulation_before_submission",
            Self::AfterCappedTestnetSubmission => "after_capped_testnet_submission",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorIncidentStatus {
    Ready,
    CoordinatorNotAccepted,
    FinalizationBlocked,
    OperatorApprovalOmitted,
    WrongAuthorityAttempted,
    MissingReadbackAfterSend,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorIncidentDrillEvidence {
    pub stage: CoordinatorIncidentStage,
    pub decision: CoordinatorDecision,
    pub posture: AnchorOperationalPosture,
    pub operator_approval_present: bool,
    pub wrong_authority_attempted: bool,
    pub network_submitted: bool,
    pub readback_present: bool,
}

impl CoordinatorIncidentDrillEvidence {
    pub fn new(
        stage: CoordinatorIncidentStage,
        decision: CoordinatorDecision,
        posture: AnchorOperationalPosture,
    ) -> Self {
        Self {
            stage,
            decision,
            posture,
            operator_approval_present: true,
            wrong_authority_attempted: false,
            network_submitted: false,
            readback_present: true,
        }
    }

    pub fn with_operator_approval_present(mut self, operator_approval_present: bool) -> Self {
        self.operator_approval_present = operator_approval_present;
        self
    }

    pub fn with_wrong_authority_attempted(mut self, wrong_authority_attempted: bool) -> Self {
        self.wrong_authority_attempted = wrong_authority_attempted;
        self
    }

    pub fn with_network_submitted(mut self, network_submitted: bool) -> Self {
        self.network_submitted = network_submitted;
        self
    }

    pub fn with_readback_present(mut self, readback_present: bool) -> Self {
        self.readback_present = readback_present;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorIncidentDrillReview {
    pub stage: CoordinatorIncidentStage,
    pub status: CoordinatorIncidentStatus,
    pub fail_safe: bool,
    pub coordinator_status: CoordinatorDecisionStatus,
    pub rpc_decision: RpcQuorumDecision,
    pub proof_decision: ReviewDecision,
    pub finalization_gate_status: CoordinatorFinalizationGateStatus,
    pub operator_approval_present: bool,
    pub wrong_authority_attempted: bool,
    pub network_submitted: bool,
    pub readback_present: bool,
    pub permits_simulation: bool,
    pub permits_submission: bool,
    pub permits_finalization: bool,
    pub success_claim: bool,
    pub finality_claim: bool,
    pub settlement_claim: bool,
}

impl CoordinatorIncidentDrillReview {
    pub fn is_ready(&self) -> bool {
        self.status == CoordinatorIncidentStatus::Ready
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            "phase14_coordinator_incident_drill: local_only".to_string(),
            format!("stage: {}", self.stage.as_str()),
            format!("status: {:?}", self.status),
            format!("fail_safe: {}", self.fail_safe),
            format!("coordinator_status: {:?}", self.coordinator_status),
            format!("rpc_decision: {:?}", self.rpc_decision),
            format!("proof_decision: {:?}", self.proof_decision),
            format!(
                "finalization_gate_status: {:?}",
                self.finalization_gate_status
            ),
            format!(
                "operator_approval_present: {}",
                self.operator_approval_present
            ),
            format!(
                "wrong_authority_attempted: {}",
                self.wrong_authority_attempted
            ),
            format!("network_submitted: {}", self.network_submitted),
            format!("readback_present: {}", self.readback_present),
            format!("permits_simulation: {}", self.permits_simulation),
            format!("permits_submission: {}", self.permits_submission),
            format!("permits_finalization: {}", self.permits_finalization),
            format!("success_claim: {}", self.success_claim),
            format!("finality_claim: {}", self.finality_claim),
            format!(
                "settlement_claim: {}",
                if self.settlement_claim {
                    "present"
                } else {
                    "none"
                }
            ),
            "transaction_submission: not_performed_by_coordinator".to_string(),
            "wallet_key_loading: disabled".to_string(),
            "signing: disabled".to_string(),
            "mint_burn_execution: disabled".to_string(),
            "internal_roc_mutation: disabled".to_string(),
            "public_bridge_authorization: none".to_string(),
        ]
    }
}

pub fn review_coordinator_incident_drill(
    evidence: CoordinatorIncidentDrillEvidence,
) -> CoordinatorIncidentDrillReview {
    let finalization_gate =
        review_coordinator_finalization_gate(&evidence.decision, evidence.posture);

    let status = if evidence.wrong_authority_attempted {
        CoordinatorIncidentStatus::WrongAuthorityAttempted
    } else if evidence.network_submitted && !evidence.readback_present {
        CoordinatorIncidentStatus::MissingReadbackAfterSend
    } else if matches!(
        evidence.stage,
        CoordinatorIncidentStage::AfterSimulationBeforeSubmission
            | CoordinatorIncidentStage::AfterCappedTestnetSubmission
    ) && !evidence.operator_approval_present
    {
        CoordinatorIncidentStatus::OperatorApprovalOmitted
    } else if !evidence.decision.is_accepted() {
        CoordinatorIncidentStatus::CoordinatorNotAccepted
    } else if !finalization_gate.permits_finalization {
        CoordinatorIncidentStatus::FinalizationBlocked
    } else {
        CoordinatorIncidentStatus::Ready
    };

    let base_accepted = evidence.decision.is_accepted() && !evidence.wrong_authority_attempted;
    let permits_simulation = base_accepted && !evidence.posture.blocks_simulation();
    let permits_submission = permits_simulation
        && evidence.operator_approval_present
        && !evidence.posture.blocks_submission();
    let permits_finalization =
        base_accepted && finalization_gate.permits_finalization && evidence.readback_present;

    CoordinatorIncidentDrillReview {
        stage: evidence.stage,
        status,
        fail_safe: status != CoordinatorIncidentStatus::Ready,
        coordinator_status: evidence.decision.status,
        rpc_decision: evidence.decision.rpc_review.decision,
        proof_decision: evidence.decision.proof_review.decision,
        finalization_gate_status: finalization_gate.status,
        operator_approval_present: evidence.operator_approval_present,
        wrong_authority_attempted: evidence.wrong_authority_attempted,
        network_submitted: evidence.network_submitted,
        readback_present: evidence.readback_present,
        permits_simulation,
        permits_submission,
        permits_finalization,
        success_claim: false,
        finality_claim: false,
        settlement_claim: false,
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
