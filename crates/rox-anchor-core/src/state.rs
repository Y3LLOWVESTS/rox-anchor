// RO:WHAT — State labels for the disabled rox-anchor-core skeleton.
// RO:WHY — Provides conservative state names from the Phase 2 design without implementing a proof engine.
// RO:INTERACTS — types, labels, and Phase 2 state/proof design docs.
// RO:INVARIANTS — State labels are evidence posture only and do not authorize runtime.
// RO:SECURITY — No client finality, cache finality, RPC finality, coordinator finality, relayer finality, or settlement behavior.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

/// Conservative evidence-state labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorState {
    Draft,
    Requested,
    Observed,
    ProofPackaged,
    EvidenceInsufficient,
    QuorumDisputed,
    ChallengeOpen,
    Challenged,
    ChallengeRejected,
    ChallengeAccepted,
    Expired,
    FinalityEligible,
    FinalizedByDecisionGate,
    Failed,
    RecoveryQueued,
    Recovered,
    HaltRequested,
    Halted,
    ResumeEligible,
    Abandoned,
}

/// Failure-closed posture classification for review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureClosedPosture {
    ContinueReview,
    Blocked,
    Halted,
    Abandoned,
}

impl AnchorState {
    pub fn posture(self) -> FailureClosedPosture {
        match self {
            Self::EvidenceInsufficient
            | Self::QuorumDisputed
            | Self::ChallengeAccepted
            | Self::Expired
            | Self::Failed
            | Self::RecoveryQueued => FailureClosedPosture::Blocked,
            Self::HaltRequested | Self::Halted => FailureClosedPosture::Halted,
            Self::Abandoned => FailureClosedPosture::Abandoned,
            _ => FailureClosedPosture::ContinueReview,
        }
    }

    pub fn is_finality_claim(self) -> bool {
        false
    }
}
