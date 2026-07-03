//! RO:WHAT — Shared domain and posture types for ROX Anchor.
//! RO:WHY — Keeps direction, binding, challenge, halt, and recovery semantics centralized.
//! RO:INTERACTS — rox-anchor-proof package review and future Anchor state transition code.
//! RO:INVARIANTS — challenge/halt/recovery postures must block unsafe acceptance when active.
//! RO:SECURITY — local type model only; no settlement, wallet, RPC, or mint/burn side effects.
//! RO:TEST — covered by posture and binding tests in rox-anchor-core.

use crate::{ClusterId, DomainId, MintId, ProgramId, TokenAccountId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AnchorDirection {
    RocToRox,
    RoxToRoc,
}

impl AnchorDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RocToRox => "roc_to_rox",
            Self::RoxToRoc => "rox_to_roc",
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Self::RocToRox => Self::RoxToRoc,
            Self::RoxToRoc => Self::RocToRox,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorBinding {
    pub source_domain: DomainId,
    pub target_domain: DomainId,
    pub direction: AnchorDirection,
    pub cluster: ClusterId,
    pub program_id: ProgramId,
    pub mint: MintId,
    pub token_account: TokenAccountId,
}

impl AnchorBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_domain: DomainId,
        target_domain: DomainId,
        direction: AnchorDirection,
        cluster: ClusterId,
        program_id: ProgramId,
        mint: MintId,
        token_account: TokenAccountId,
    ) -> Self {
        Self {
            source_domain,
            target_domain,
            direction,
            cluster,
            program_id,
            mint,
            token_account,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChallengePosture {
    Clear,
    Open,
    Accepted,
    Rejected,
    Expired,
}

impl ChallengePosture {
    pub fn blocks_acceptance(self) -> bool {
        matches!(self, Self::Open | Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HaltPosture {
    Active,
    HaltRequested,
    Halted,
    ResumeEligible,
}

impl HaltPosture {
    pub fn blocks_acceptance(self) -> bool {
        matches!(self, Self::HaltRequested | Self::Halted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RecoveryPosture {
    NotRequired,
    Required,
    InReview,
    Resolved,
    Rejected,
}

impl RecoveryPosture {
    pub fn blocks_acceptance(self) -> bool {
        matches!(self, Self::Required | Self::InReview | Self::Rejected)
    }
}
