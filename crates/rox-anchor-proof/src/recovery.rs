//! RO:WHAT — Halt and recovery posture review for ROX Anchor proof validation.
//! RO:WHY — Halted or recovery-required cases must not pass local acceptance.
//! RO:INTERACTS — rox-anchor-core HaltPosture, RecoveryPosture, and validate.rs.
//! RO:INVARIANTS — halt/recovery blockers are explicit and deterministic.
//! RO:SECURITY — classification only; no recovery authority or runtime mutation.
//! RO:TEST — covered by halt and recovery blocking tests.

use rox_anchor_core::{HaltPosture, RecoveryPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltReview {
    Active,
    HaltRequested,
    Halted,
}

impl HaltReview {
    pub fn blocks_acceptance(self) -> bool {
        match self {
            Self::Active => false,
            Self::HaltRequested | Self::Halted => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryReview {
    Clear,
    Required,
    InReview,
    Rejected,
}

impl RecoveryReview {
    pub fn blocks_acceptance(self) -> bool {
        match self {
            Self::Clear => false,
            Self::Required | Self::InReview | Self::Rejected => true,
        }
    }
}

pub fn review_halt(posture: HaltPosture) -> HaltReview {
    match posture {
        HaltPosture::Active | HaltPosture::ResumeEligible => HaltReview::Active,
        HaltPosture::HaltRequested => HaltReview::HaltRequested,
        HaltPosture::Halted => HaltReview::Halted,
    }
}

pub fn review_recovery(posture: RecoveryPosture) -> RecoveryReview {
    match posture {
        RecoveryPosture::NotRequired | RecoveryPosture::Resolved => RecoveryReview::Clear,
        RecoveryPosture::Required => RecoveryReview::Required,
        RecoveryPosture::InReview => RecoveryReview::InReview,
        RecoveryPosture::Rejected => RecoveryReview::Rejected,
    }
}
