//! RO:WHAT — Shared lifecycle states and blocker classification for ROX Anchor.
//! RO:WHY — Gives proof, CLI, local services, and the future program one state vocabulary.
//! RO:INTERACTS — labels, proof validation, coordinator decisions, relayer dry-run, and Anchor state.
//! RO:INVARIANTS — unsafe challenge/halt/recovery/evidence states block acceptance/finalization.
//! RO:SECURITY — state classification only; does not authorize value movement.
//! RO:TEST — covered by rox-anchor-core lifecycle tests.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AnchorLifecycleState {
    Draft,
    Requested,
    Observed,
    ProofPackaged,
    EvidenceIncomplete,
    QuorumDisputed,
    ChallengeOpen,
    ChallengeAccepted,
    ChallengeRejected,
    HaltRequested,
    Halted,
    RecoveryRequired,
    RecoveryInReview,
    RecoveryResolved,
    FinalityEligible,
    Finalized,
    Failed,
    Abandoned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReviewBlocker {
    None,
    Evidence,
    Challenge,
    Halt,
    Recovery,
    Rejected,
    Terminal,
}

impl AnchorLifecycleState {
    pub fn blocker(self) -> ReviewBlocker {
        match self {
            Self::Draft
            | Self::Requested
            | Self::Observed
            | Self::ProofPackaged
            | Self::FinalityEligible => ReviewBlocker::None,
            Self::EvidenceIncomplete | Self::QuorumDisputed => ReviewBlocker::Evidence,
            Self::ChallengeOpen | Self::ChallengeAccepted => ReviewBlocker::Challenge,
            Self::HaltRequested | Self::Halted => ReviewBlocker::Halt,
            Self::RecoveryRequired | Self::RecoveryInReview => ReviewBlocker::Recovery,
            Self::ChallengeRejected | Self::Failed => ReviewBlocker::Rejected,
            Self::RecoveryResolved | Self::Finalized | Self::Abandoned => ReviewBlocker::Terminal,
        }
    }

    pub fn blocks_acceptance(self) -> bool {
        !matches!(self.blocker(), ReviewBlocker::None)
    }

    pub fn blocks_finalization(self) -> bool {
        !matches!(self, Self::FinalityEligible)
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::ChallengeRejected
                | Self::RecoveryResolved
                | Self::Finalized
                | Self::Failed
                | Self::Abandoned
        )
    }
}
