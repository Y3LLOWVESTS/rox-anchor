//! RO:WHAT — Local RPC quorum/evidence classification.
//! RO:WHY — Turns local RPC observations into deterministic agreement, missing, disputed, or rejected posture.
//! RO:INTERACTS — rpc.rs, config.rs, commitment.rs, and rox-anchor-proof EvidenceBundle.
//! RO:INVARIANTS — mismatch, stale, and low-commitment evidence cannot pass agreement.
//! RO:SECURITY — local classification only; no network, wallet, submission, mint, burn, or settlement.
//! RO:TEST — crate-local tests cover agreement, missing evidence, equivocation, stale, and mismatch cases.

use std::collections::{BTreeMap, BTreeSet};

use rox_anchor_proof::EvidenceBundle;

use crate::{ExpectedRpcBinding, RpcObservation, RpcProofConfig};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcQuorumDecision {
    Agreement,
    MissingEvidence,
    Disputed,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcQuorumFindingCode {
    SourceAccepted,
    MissingEvidence,
    RpcEquivocation,
    SourceEquivocation,
    StaleEvidence,
    InsufficientCommitment,
    ClusterMismatch,
    ProgramIdMismatch,
    MintMismatch,
    TokenAccountMismatch,
    OperationIdMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcQuorumReview {
    pub decision: RpcQuorumDecision,
    pub findings: Vec<RpcQuorumFindingCode>,
    pub accepted_observations: u16,
    pub required_observations: u16,
}

impl RpcQuorumReview {
    pub fn has_finding(&self, code: RpcQuorumFindingCode) -> bool {
        self.findings.contains(&code)
    }

    pub fn to_evidence_bundle(&self) -> EvidenceBundle {
        let dispute_count = match self.decision {
            RpcQuorumDecision::Disputed | RpcQuorumDecision::Rejected => 1,
            RpcQuorumDecision::Agreement | RpcQuorumDecision::MissingEvidence => 0,
        };

        EvidenceBundle::new(
            self.accepted_observations,
            self.required_observations,
            dispute_count,
        )
    }
}

pub fn review_rpc_observations(
    observations: &[RpcObservation],
    expected: &ExpectedRpcBinding,
    config: RpcProofConfig,
    current_slot: u64,
) -> RpcQuorumReview {
    let required = config.required_observations.max(1);
    let mut findings = Vec::new();
    let mut accepted_by_source = BTreeMap::<String, String>::new();
    let mut rejected = false;
    let mut disputed = false;

    for observation in observations {
        let mut observation_rejected = false;

        if observation.cluster != expected.cluster {
            findings.push(RpcQuorumFindingCode::ClusterMismatch);
            observation_rejected = true;
        }

        if observation.program_id != expected.program_id {
            findings.push(RpcQuorumFindingCode::ProgramIdMismatch);
            observation_rejected = true;
        }

        if observation.mint != expected.mint {
            findings.push(RpcQuorumFindingCode::MintMismatch);
            observation_rejected = true;
        }

        if observation.token_account != expected.token_account {
            findings.push(RpcQuorumFindingCode::TokenAccountMismatch);
            observation_rejected = true;
        }

        if observation.operation_id != expected.operation_id {
            findings.push(RpcQuorumFindingCode::OperationIdMismatch);
            observation_rejected = true;
        }

        if !observation
            .commitment
            .meets_minimum(expected.minimum_commitment)
        {
            findings.push(RpcQuorumFindingCode::InsufficientCommitment);
            observation_rejected = true;
        }

        if current_slot.saturating_sub(observation.slot) > config.stale_after_slots {
            findings.push(RpcQuorumFindingCode::StaleEvidence);
            observation_rejected = true;
        }

        if observation_rejected {
            rejected = true;
            continue;
        }

        match accepted_by_source.insert(observation.source.clone(), observation.signature.clone()) {
            Some(previous_signature) if previous_signature != observation.signature => {
                findings.push(RpcQuorumFindingCode::SourceEquivocation);
                disputed = true;
            }
            _ => findings.push(RpcQuorumFindingCode::SourceAccepted),
        }
    }

    let accepted_observations = accepted_by_source.len().min(u16::MAX as usize) as u16;

    if rejected {
        return RpcQuorumReview {
            decision: RpcQuorumDecision::Rejected,
            findings: dedup_findings(findings),
            accepted_observations,
            required_observations: required,
        };
    }

    let accepted_signatures = accepted_by_source
        .values()
        .cloned()
        .collect::<BTreeSet<_>>();

    if disputed || accepted_signatures.len() > 1 {
        findings.push(RpcQuorumFindingCode::RpcEquivocation);
        return RpcQuorumReview {
            decision: RpcQuorumDecision::Disputed,
            findings: dedup_findings(findings),
            accepted_observations,
            required_observations: required,
        };
    }

    if accepted_observations < required {
        findings.push(RpcQuorumFindingCode::MissingEvidence);
        return RpcQuorumReview {
            decision: RpcQuorumDecision::MissingEvidence,
            findings: dedup_findings(findings),
            accepted_observations,
            required_observations: required,
        };
    }

    RpcQuorumReview {
        decision: RpcQuorumDecision::Agreement,
        findings: dedup_findings(findings),
        accepted_observations,
        required_observations: required,
    }
}

fn dedup_findings(findings: Vec<RpcQuorumFindingCode>) -> Vec<RpcQuorumFindingCode> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();

    for finding in findings {
        if seen.insert(format!("{finding:?}")) {
            deduped.push(finding);
        }
    }

    deduped
}
