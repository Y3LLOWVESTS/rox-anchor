//! RO:WHAT — Deterministic audit records for local RPC quorum review.
//! RO:WHY — Phase 11 requires inspectable RPC evidence reports before coordinator/relayer action.
//! RO:INTERACTS — ExpectedRpcBinding, RpcObservation, RpcQuorumReview, and signature redaction.
//! RO:INVARIANTS — audit output reflects quorum review and never upgrades evidence to finality.
//! RO:SECURITY — local report only; no live RPC submission, wallet calls, minting, burning, or settlement.
//! RO:TEST — covered by RPC proof audit record tests.

use crate::{redact_signature, ExpectedRpcBinding, RpcObservation, RpcQuorumReview};

const AUDIT_RECORD_VERSION: &str = "rpc-proof-audit-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcObservationAuditEntry {
    pub source: String,
    pub cluster: String,
    pub program_id: String,
    pub mint: String,
    pub token_account: String,
    pub operation_id: String,
    pub redacted_signature: String,
    pub slot: u64,
    pub commitment: String,
}

impl RpcObservationAuditEntry {
    pub fn from_observation(observation: &RpcObservation) -> Self {
        Self {
            source: observation.source.clone(),
            cluster: observation.cluster.to_string(),
            program_id: observation.program_id.to_string(),
            mint: observation.mint.to_string(),
            token_account: observation.token_account.to_string(),
            operation_id: observation.operation_id.to_string(),
            redacted_signature: redact_signature(&observation.signature),
            slot: observation.slot,
            commitment: format!("{:?}", observation.commitment),
        }
    }

    fn render(&self, index: usize) -> Vec<String> {
        vec![
            format!("observation.{index}.source={}", self.source),
            format!("observation.{index}.cluster={}", self.cluster),
            format!("observation.{index}.program_id={}", self.program_id),
            format!("observation.{index}.mint={}", self.mint),
            format!("observation.{index}.token_account={}", self.token_account),
            format!("observation.{index}.operation_id={}", self.operation_id),
            format!("observation.{index}.signature={}", self.redacted_signature),
            format!("observation.{index}.slot={}", self.slot),
            format!("observation.{index}.commitment={}", self.commitment),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcProofAuditRecord {
    pub version: &'static str,
    pub expected_cluster: String,
    pub expected_program_id: String,
    pub expected_mint: String,
    pub expected_token_account: String,
    pub expected_operation_id: String,
    pub minimum_commitment: String,
    pub current_slot: u64,
    pub observation_count: u16,
    pub accepted_observations: u16,
    pub required_observations: u16,
    pub decision: String,
    pub findings: Vec<String>,
    pub observations: Vec<RpcObservationAuditEntry>,
    pub evidence_consistent: bool,
}

impl RpcProofAuditRecord {
    pub fn from_review(
        expected: &ExpectedRpcBinding,
        observations: &[RpcObservation],
        review: &RpcQuorumReview,
        current_slot: u64,
    ) -> Self {
        let expected_observation_count = observations.len().min(u16::MAX as usize) as u16;
        let evidence_consistent = review.accepted_observations <= expected_observation_count
            && review.required_observations > 0;

        Self {
            version: AUDIT_RECORD_VERSION,
            expected_cluster: expected.cluster.to_string(),
            expected_program_id: expected.program_id.to_string(),
            expected_mint: expected.mint.to_string(),
            expected_token_account: expected.token_account.to_string(),
            expected_operation_id: expected.operation_id.to_string(),
            minimum_commitment: format!("{:?}", expected.minimum_commitment),
            current_slot,
            observation_count: expected_observation_count,
            accepted_observations: review.accepted_observations,
            required_observations: review.required_observations,
            decision: format!("{:?}", review.decision),
            findings: review
                .findings
                .iter()
                .map(|finding| format!("{finding:?}"))
                .collect(),
            observations: observations
                .iter()
                .map(RpcObservationAuditEntry::from_observation)
                .collect(),
            evidence_consistent,
        }
    }

    pub fn is_safe_for_display(&self) -> bool {
        self.evidence_consistent
            && !contains_sensitive_hint(&self.expected_cluster)
            && !contains_sensitive_hint(&self.expected_program_id)
            && !contains_sensitive_hint(&self.expected_mint)
            && !contains_sensitive_hint(&self.expected_token_account)
            && !contains_sensitive_hint(&self.expected_operation_id)
            && self.observations.iter().all(|observation| {
                !contains_sensitive_hint(&observation.source)
                    && !contains_sensitive_hint(&observation.cluster)
                    && !contains_sensitive_hint(&observation.program_id)
                    && !contains_sensitive_hint(&observation.mint)
                    && !contains_sensitive_hint(&observation.token_account)
                    && !contains_sensitive_hint(&observation.operation_id)
                    && !contains_sensitive_hint(&observation.redacted_signature)
            })
    }

    pub fn render(&self) -> String {
        let mut lines = vec![
            format!("audit_record={}", self.version),
            format!("expected_cluster={}", self.expected_cluster),
            format!("expected_program_id={}", self.expected_program_id),
            format!("expected_mint={}", self.expected_mint),
            format!("expected_token_account={}", self.expected_token_account),
            format!("expected_operation_id={}", self.expected_operation_id),
            format!("minimum_commitment={}", self.minimum_commitment),
            format!("current_slot={}", self.current_slot),
            format!("observation_count={}", self.observation_count),
            format!("accepted_observations={}", self.accepted_observations),
            format!("required_observations={}", self.required_observations),
            format!("decision={}", self.decision),
            format!("findings={}", self.findings.join(",")),
            format!("evidence_consistent={}", self.evidence_consistent),
            format!("display_safe={}", self.is_safe_for_display()),
        ];

        for (index, observation) in self.observations.iter().enumerate() {
            lines.extend(observation.render(index));
        }

        lines.join("\n")
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
