// RO:WHAT — Conservative display/status label constants for the disabled rox-anchor-core skeleton.
// RO:WHY — Keeps status wording failure-closed and non-authoritative before any UI or runtime exists.
// RO:INTERACTS — state and future CrabLink display-only status design.
// RO:INVARIANTS — Labels are display posture only and do not authorize runtime.
// RO:SECURITY — No finality claims, no cache truth, no wallet/RPC authority, no Solana/Anchor runtime, no settlement behavior.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

pub const STATUS_NOT_AVAILABLE: &str = "Not available";
pub const STATUS_PLANNING_ONLY: &str = "Planning only";
pub const STATUS_PENDING_OBSERVATION: &str = "Pending observation";
pub const STATUS_EVIDENCE_INCOMPLETE: &str = "Evidence incomplete";
pub const STATUS_QUORUM_DISPUTED: &str = "Quorum disputed";
pub const STATUS_CHALLENGE_OPEN: &str = "Challenge open";
pub const STATUS_CHALLENGED: &str = "Challenged";
pub const STATUS_EXPIRED: &str = "Expired";
pub const STATUS_FAILED: &str = "Failed";
pub const STATUS_HALTED: &str = "Halted";
pub const STATUS_RECOVERY_REVIEW_REQUIRED: &str = "Recovery review required";
pub const STATUS_STALE: &str = "Stale status";
pub const STATUS_OFFLINE_UNKNOWN: &str = "Offline — status unknown";
pub const STATUS_FINALITY_ELIGIBLE_NOT_COMPLETE: &str = "Finality eligible — not complete";

pub const SAFE_STATUS_LABELS: &[&str] = &[
    STATUS_NOT_AVAILABLE,
    STATUS_PLANNING_ONLY,
    STATUS_PENDING_OBSERVATION,
    STATUS_EVIDENCE_INCOMPLETE,
    STATUS_QUORUM_DISPUTED,
    STATUS_CHALLENGE_OPEN,
    STATUS_CHALLENGED,
    STATUS_EXPIRED,
    STATUS_FAILED,
    STATUS_HALTED,
    STATUS_RECOVERY_REVIEW_REQUIRED,
    STATUS_STALE,
    STATUS_OFFLINE_UNKNOWN,
    STATUS_FINALITY_ELIGIBLE_NOT_COMPLETE,
];
