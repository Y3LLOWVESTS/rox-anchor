//! RO:WHAT — Evidence/quorum classification for ROX Anchor proof review.
//! RO:WHY — Separates evidence completeness from binding/replay/challenge decisions.
//! RO:INTERACTS — validate.rs and proof package fixtures.
//! RO:INVARIANTS — missing or disputed evidence blocks acceptance.
//! RO:SECURITY — local evidence counters only; no RPC polling or finality claims.
//! RO:TEST — covered by missing/disputed evidence tests.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceBundle {
    pub observation_count: u16,
    pub required_observations: u16,
    pub dispute_count: u16,
}

impl EvidenceBundle {
    pub fn new(observation_count: u16, required_observations: u16, dispute_count: u16) -> Self {
        Self {
            observation_count,
            required_observations,
            dispute_count,
        }
    }

    pub fn satisfied(required_observations: u16) -> Self {
        Self::new(required_observations, required_observations, 0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuorumPosture {
    Satisfied,
    MissingEvidence,
    Disputed,
}

pub fn classify_quorum(evidence: EvidenceBundle) -> QuorumPosture {
    if evidence.dispute_count > 0 {
        return QuorumPosture::Disputed;
    }

    if evidence.required_observations == 0
        || evidence.observation_count < evidence.required_observations
    {
        return QuorumPosture::MissingEvidence;
    }

    QuorumPosture::Satisfied
}
