//! RO:WHAT — Display-safe status labels for ROX Anchor review and local UX.
//! RO:WHY — Prevents every crate from inventing its own acceptance/finality wording.
//! RO:INTERACTS — AnchorLifecycleState, proof decisions, CLI reports, and future CrabLink display.
//! RO:INVARIANTS — labels are deterministic display strings, not authority or settlement.
//! RO:SECURITY — no fake success/finality/settlement wording from labels alone.
//! RO:TEST — covered by rox-anchor-core label tests.

use crate::AnchorLifecycleState;

pub const STATUS_DRAFT: &str = "Draft";
pub const STATUS_REQUESTED: &str = "Requested";
pub const STATUS_OBSERVED: &str = "Observed";
pub const STATUS_PROOF_PACKAGED: &str = "Proof packaged";
pub const STATUS_EVIDENCE_INCOMPLETE: &str = "Evidence incomplete";
pub const STATUS_QUORUM_DISPUTED: &str = "Quorum disputed";
pub const STATUS_CHALLENGE_OPEN: &str = "Challenge open";
pub const STATUS_CHALLENGE_ACCEPTED: &str = "Challenge accepted";
pub const STATUS_CHALLENGE_REJECTED: &str = "Challenge rejected";
pub const STATUS_HALT_REQUESTED: &str = "Halt requested";
pub const STATUS_HALTED: &str = "Halted";
pub const STATUS_RECOVERY_REQUIRED: &str = "Recovery required";
pub const STATUS_RECOVERY_IN_REVIEW: &str = "Recovery in review";
pub const STATUS_RECOVERY_RESOLVED: &str = "Recovery resolved";
pub const STATUS_FINALITY_ELIGIBLE: &str = "Finality eligible";
pub const STATUS_FINALIZED: &str = "Finalized";
pub const STATUS_FAILED: &str = "Failed";
pub const STATUS_ABANDONED: &str = "Abandoned";

pub const SAFE_STATUS_LABELS: &[&str] = &[
    STATUS_DRAFT,
    STATUS_REQUESTED,
    STATUS_OBSERVED,
    STATUS_PROOF_PACKAGED,
    STATUS_EVIDENCE_INCOMPLETE,
    STATUS_QUORUM_DISPUTED,
    STATUS_CHALLENGE_OPEN,
    STATUS_CHALLENGE_ACCEPTED,
    STATUS_CHALLENGE_REJECTED,
    STATUS_HALT_REQUESTED,
    STATUS_HALTED,
    STATUS_RECOVERY_REQUIRED,
    STATUS_RECOVERY_IN_REVIEW,
    STATUS_RECOVERY_RESOLVED,
    STATUS_FINALITY_ELIGIBLE,
    STATUS_FINALIZED,
    STATUS_FAILED,
    STATUS_ABANDONED,
];

pub fn label_for_lifecycle_state(state: AnchorLifecycleState) -> &'static str {
    match state {
        AnchorLifecycleState::Draft => STATUS_DRAFT,
        AnchorLifecycleState::Requested => STATUS_REQUESTED,
        AnchorLifecycleState::Observed => STATUS_OBSERVED,
        AnchorLifecycleState::ProofPackaged => STATUS_PROOF_PACKAGED,
        AnchorLifecycleState::EvidenceIncomplete => STATUS_EVIDENCE_INCOMPLETE,
        AnchorLifecycleState::QuorumDisputed => STATUS_QUORUM_DISPUTED,
        AnchorLifecycleState::ChallengeOpen => STATUS_CHALLENGE_OPEN,
        AnchorLifecycleState::ChallengeAccepted => STATUS_CHALLENGE_ACCEPTED,
        AnchorLifecycleState::ChallengeRejected => STATUS_CHALLENGE_REJECTED,
        AnchorLifecycleState::HaltRequested => STATUS_HALT_REQUESTED,
        AnchorLifecycleState::Halted => STATUS_HALTED,
        AnchorLifecycleState::RecoveryRequired => STATUS_RECOVERY_REQUIRED,
        AnchorLifecycleState::RecoveryInReview => STATUS_RECOVERY_IN_REVIEW,
        AnchorLifecycleState::RecoveryResolved => STATUS_RECOVERY_RESOLVED,
        AnchorLifecycleState::FinalityEligible => STATUS_FINALITY_ELIGIBLE,
        AnchorLifecycleState::Finalized => STATUS_FINALIZED,
        AnchorLifecycleState::Failed => STATUS_FAILED,
        AnchorLifecycleState::Abandoned => STATUS_ABANDONED,
    }
}
