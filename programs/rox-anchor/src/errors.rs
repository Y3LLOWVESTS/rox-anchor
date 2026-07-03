//! RO:WHAT — Anchor program errors for ROX Anchor state rules.
//! RO:WHY — Gives instruction handlers deterministic rejection reasons.
//! RO:INTERACTS — observe, challenge, halt, recover, finalize, and mint/burn binding helpers.
//! RO:INVARIANTS — unsafe challenge/halt/recovery/finality and binding transitions are rejected.
//! RO:SECURITY — errors only; no side effects.
//! RO:TEST — covered by handler compile checks and state helper tests.

use anchor_lang::prelude::*;

#[error_code]
pub enum RoxAnchorError {
    #[msg("ROX Anchor program is halted")]
    ProgramHalted,
    #[msg("ROX Anchor operation has an open challenge")]
    ChallengeOpen,
    #[msg("ROX Anchor operation requires recovery")]
    RecoveryRequired,
    #[msg("ROX Anchor operation is not eligible for this transition")]
    InvalidStateTransition,
    #[msg("ROX Anchor operation binding is invalid")]
    InvalidBinding,
    #[msg("ROX Anchor config binding is invalid")]
    InvalidConfigBinding,
    #[msg("ROX Anchor operation is already finalized")]
    AlreadyFinalized,
    #[msg("ROX Anchor operation binding mismatch")]
    OperationBindingMismatch,
    #[msg("ROX Anchor operation PDA binding mismatch")]
    OperationPdaMismatch,
    #[msg("ROX Anchor direction binding mismatch")]
    DirectionBindingMismatch,
    #[msg("ROX Anchor mint binding mismatch")]
    MintBindingMismatch,
    #[msg("ROX Anchor mint authority mismatch")]
    MintAuthorityMismatch,
    #[msg("ROX Anchor token account binding mismatch")]
    TokenAccountBindingMismatch,
    #[msg("ROX Anchor amount binding mismatch")]
    AmountBindingMismatch,
    #[msg("ROX Anchor burn evidence binding mismatch")]
    BurnEvidenceBindingMismatch,
    #[msg("ROX Anchor authority mismatch")]
    AuthorityMismatch,
}
