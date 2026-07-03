// RO:WHAT — Quorum posture and local evidence-count review for the local proof validator.
// RO:WHY — Keeps multi-source evidence review conservative without implementing observation, RPC, or proof finality.
// RO:INTERACTS — package and validate local review modules.
// RO:INVARIANTS — Quorum posture is evidence posture only and does not authorize runtime.
// RO:SECURITY — No network calls, no wallet calls, no Solana/Anchor runtime, no bridge runtime, no settlement behavior.
// RO:TEST — Static Phase 4 checker and local unit-test source only.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local validator does not authorize runtime.

/// Local-only source agreement posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuorumObservationPosture {
    NotEvaluated,
    EvidencePresent,
    EvidenceIncomplete,
    Disputed,
}

/// Quorum findings for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuorumReviewFinding {
    EvidenceIncomplete,
    QuorumDisputed,
}

/// Local-only quorum review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuorumReview {
    pub posture: QuorumObservationPosture,
    pub finding: Option<QuorumReviewFinding>,
}

pub type QuorumReviewSkeleton = QuorumReview;

impl QuorumReview {
    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }
}

/// Local-only evidence counts for quorum posture review.
///
/// Counts are supplied by fixtures or local callers. This type performs no RPC,
/// no observation, no network IO, and no runtime authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuorumEvidenceCount {
    pub matching_observations: u16,
    pub disputed_observations: u16,
    pub missing_observations: u16,
    pub minimum_matching_observations: u16,
}

impl QuorumEvidenceCount {
    pub fn new(
        matching_observations: u16,
        disputed_observations: u16,
        missing_observations: u16,
        minimum_matching_observations: u16,
    ) -> Self {
        Self {
            matching_observations,
            disputed_observations,
            missing_observations,
            minimum_matching_observations,
        }
    }

    pub fn has_dispute(self) -> bool {
        self.disputed_observations > 0
    }

    pub fn has_missing(self) -> bool {
        self.missing_observations > 0
    }

    pub fn minimum_met(self) -> bool {
        self.matching_observations >= self.minimum_matching_observations
            && self.minimum_matching_observations > 0
    }

    pub fn single_observation_only(self) -> bool {
        self.matching_observations == 1 && self.minimum_matching_observations <= 1
    }
}

/// Local quorum evidence review decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuorumEvidenceReviewDecision {
    EvidencePresent,
    EvidenceIncomplete,
    QuorumDisputed,
    RuntimeNotAuthorized,
}

/// Local quorum evidence findings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuorumEvidenceReviewFinding {
    MatchingEvidencePresent,
    MissingEvidence,
    MinimumNotMet,
    DisputedEvidence,
    SingleObservationNotAuthority,
    RuntimeAuthorizationRejected,
}

/// Local-only quorum evidence review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuorumEvidenceReview {
    pub counts: QuorumEvidenceCount,
    pub decision: QuorumEvidenceReviewDecision,
    pub findings: Vec<QuorumEvidenceReviewFinding>,
}

impl QuorumEvidenceReview {
    pub fn has_finding(&self, finding: QuorumEvidenceReviewFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub fn review_quorum_posture(posture: QuorumObservationPosture) -> QuorumReview {
    let finding = match posture {
        QuorumObservationPosture::NotEvaluated
        | QuorumObservationPosture::EvidenceIncomplete => {
            Some(QuorumReviewFinding::EvidenceIncomplete)
        }
        QuorumObservationPosture::Disputed => Some(QuorumReviewFinding::QuorumDisputed),
        QuorumObservationPosture::EvidencePresent => None,
    };

    QuorumReview { posture, finding }
}

pub fn review_quorum_evidence_counts_for_local_review_only(
    counts: QuorumEvidenceCount,
) -> QuorumEvidenceReview {
    let mut findings = Vec::new();

    if counts.single_observation_only() {
        findings.push(QuorumEvidenceReviewFinding::SingleObservationNotAuthority);
    }

    if counts.has_dispute() {
        findings.push(QuorumEvidenceReviewFinding::DisputedEvidence);
        return QuorumEvidenceReview {
            counts,
            decision: QuorumEvidenceReviewDecision::QuorumDisputed,
            findings,
        };
    }

    if counts.has_missing() {
        findings.push(QuorumEvidenceReviewFinding::MissingEvidence);
    }

    if !counts.minimum_met() {
        findings.push(QuorumEvidenceReviewFinding::MinimumNotMet);
        return QuorumEvidenceReview {
            counts,
            decision: QuorumEvidenceReviewDecision::EvidenceIncomplete,
            findings,
        };
    }

    findings.push(QuorumEvidenceReviewFinding::MatchingEvidencePresent);

    QuorumEvidenceReview {
        counts,
        decision: QuorumEvidenceReviewDecision::EvidencePresent,
        findings,
    }
}

pub fn quorum_evidence_review_authorizes_runtime() -> bool {
    false
}

pub fn quorum_evidence_review_calls_rpc() -> bool {
    false
}

pub fn quorum_evidence_review_is_finality() -> bool {
    false
}

pub fn quorum_evidence_review_is_settlement() -> bool {
    false
}
