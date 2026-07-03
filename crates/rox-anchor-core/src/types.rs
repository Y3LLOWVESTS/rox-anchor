// RO:WHAT — Core type skeletons for future-gated ROX Anchor proof design.
// RO:WHY — Captures local-only shape names without implementing proof validation or runtime behavior.
// RO:INTERACTS — ids, state, labels, and Phase 2 proof package design.
// RO:INVARIANTS — ProofPackageSkeleton is evidence shape only and does not authorize runtime.
// RO:SECURITY — No RPC, wallet, Solana/Anchor, bridge runtime, deployment, minting, burning, staking, liquidity, or external settlement.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

use crate::ids::{AnchorDomain, AnchorId, IdempotencyKey, Nonce};

/// Future-gated direction label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorDirection {
    RocToRox,
    RoxToRoc,
}

/// Challenge posture label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengePosture {
    NotOpened,
    Open,
    Challenged,
    Accepted,
    Rejected,
    Expired,
}

/// Halt posture label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltPosture {
    NotHalted,
    HaltRequested,
    Halted,
    ResumeEligible,
}

/// Recovery posture label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryPosture {
    NotRequired,
    Queued,
    Reviewed,
    Rejected,
    Resolved,
}

/// Evidence-shape placeholder only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofPackageSkeleton {
    pub schema_version: String,
    pub source_domain: AnchorDomain,
    pub target_domain: AnchorDomain,
    pub direction: AnchorDirection,
    pub operation_id: AnchorId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
    pub challenge_posture: ChallengePosture,
    pub halt_posture: HaltPosture,
    pub recovery_posture: RecoveryPosture,
}

impl ProofPackageSkeleton {
    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }
}
