//! RO:WHAT — Deterministic coordinator audit records for testnet-shaped review decisions.
//! RO:WHY — Phase 11 requires inspectable RPC/proof/coordinator reports before broader testnet use.
//! RO:INTERACTS — CoordinatorReviewRequest, CoordinatorDecision, RPC quorum review, and proof review.
//! RO:INVARIANTS — accepted coordinator status must match RPC agreement and proof acceptance.
//! RO:SECURITY — local audit only; no live RPC, signing, submission, mint, burn, or settlement.
//! RO:TEST — covered by coordinator audit record tests.

use rox_anchor_proof::ReviewDecision;
use rox_anchor_rpc_proof::RpcQuorumDecision;

use crate::{CoordinatorDecision, CoordinatorDecisionStatus, CoordinatorReviewRequest};

const AUDIT_RECORD_VERSION: &str = "coordinator-testnet-audit-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorAuditRecord {
    pub version: &'static str,
    pub operation_id: String,
    pub idempotency_key: String,
    pub nonce: String,
    pub direction: String,
    pub source_domain: String,
    pub target_domain: String,
    pub cluster: String,
    pub program_id: String,
    pub mint: String,
    pub token_account: String,
    pub current_slot: u64,
    pub observation_count: u16,
    pub rpc_decision: String,
    pub rpc_findings: Vec<String>,
    pub accepted_observations: u16,
    pub required_observations: u16,
    pub proof_decision: String,
    pub proof_lifecycle_state: String,
    pub proof_findings: Vec<String>,
    pub coordinator_status: String,
    pub permits_simulation: bool,
    pub status_consistent: bool,
}

impl CoordinatorAuditRecord {
    pub fn from_review(
        request: &CoordinatorReviewRequest,
        decision: &CoordinatorDecision,
        current_slot: u64,
    ) -> Self {
        let status_consistent = decision.status == expected_status(decision);
        let binding = &request.package.binding;

        Self {
            version: AUDIT_RECORD_VERSION,
            operation_id: request.package.operation_id.to_string(),
            idempotency_key: request.package.idempotency_key.to_string(),
            nonce: request.package.nonce.to_string(),
            direction: binding.direction.as_str().to_owned(),
            source_domain: binding.source_domain.to_string(),
            target_domain: binding.target_domain.to_string(),
            cluster: binding.cluster.to_string(),
            program_id: binding.program_id.to_string(),
            mint: binding.mint.to_string(),
            token_account: binding.token_account.to_string(),
            current_slot,
            observation_count: request.observations.len().min(u16::MAX as usize) as u16,
            rpc_decision: format!("{:?}", decision.rpc_review.decision),
            rpc_findings: decision
                .rpc_review
                .findings
                .iter()
                .map(|finding| format!("{finding:?}"))
                .collect(),
            accepted_observations: decision.rpc_review.accepted_observations,
            required_observations: decision.rpc_review.required_observations,
            proof_decision: format!("{:?}", decision.proof_review.decision),
            proof_lifecycle_state: format!("{:?}", decision.proof_review.lifecycle_state),
            proof_findings: decision
                .proof_review
                .findings
                .iter()
                .map(|finding| format!("{:?}:{:?}", finding.severity, finding.code))
                .collect(),
            coordinator_status: format!("{:?}", decision.status),
            permits_simulation: decision.permits_transaction_simulation(),
            status_consistent,
        }
    }

    pub fn is_safe_for_display(&self) -> bool {
        self.status_consistent
            && !contains_sensitive_hint(&self.source_domain)
            && !contains_sensitive_hint(&self.target_domain)
            && !contains_sensitive_hint(&self.program_id)
            && !contains_sensitive_hint(&self.mint)
            && !contains_sensitive_hint(&self.token_account)
    }

    pub fn render(&self) -> String {
        [
            format!("audit_record={}", self.version),
            format!("operation_id={}", self.operation_id),
            format!("idempotency_key={}", self.idempotency_key),
            format!("nonce={}", self.nonce),
            format!("direction={}", self.direction),
            format!("source_domain={}", self.source_domain),
            format!("target_domain={}", self.target_domain),
            format!("cluster={}", self.cluster),
            format!("program_id={}", self.program_id),
            format!("mint={}", self.mint),
            format!("token_account={}", self.token_account),
            format!("current_slot={}", self.current_slot),
            format!("observation_count={}", self.observation_count),
            format!("rpc_decision={}", self.rpc_decision),
            format!("rpc_findings={}", self.rpc_findings.join(",")),
            format!("accepted_observations={}", self.accepted_observations),
            format!("required_observations={}", self.required_observations),
            format!("proof_decision={}", self.proof_decision),
            format!("proof_lifecycle_state={}", self.proof_lifecycle_state),
            format!("proof_findings={}", self.proof_findings.join(",")),
            format!("coordinator_status={}", self.coordinator_status),
            format!("permits_simulation={}", self.permits_simulation),
            format!("status_consistent={}", self.status_consistent),
            format!("display_safe={}", self.is_safe_for_display()),
        ]
        .join("\n")
    }
}

fn expected_status(decision: &CoordinatorDecision) -> CoordinatorDecisionStatus {
    if decision.rpc_review.decision == RpcQuorumDecision::Rejected {
        return CoordinatorDecisionStatus::RejectedEvidence;
    }

    match decision.proof_review.decision {
        ReviewDecision::Accepted => CoordinatorDecisionStatus::Accepted,
        ReviewDecision::Blocked => CoordinatorDecisionStatus::BlockedProof,
        ReviewDecision::Rejected => CoordinatorDecisionStatus::RejectedProof,
    }
}

fn contains_sensitive_hint(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    [
        "secret",
        "keypair",
        "wallet",
        "mnemonic",
        "seed",
        "private",
        "credential",
        "password",
        "rpc-url",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}
