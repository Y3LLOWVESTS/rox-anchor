//! RO:WHAT — Anchor events for ROX Anchor state transitions.
//! RO:WHY — Makes local program transitions observable once validator tests are added.
//! RO:INTERACTS — instruction handlers.
//! RO:INVARIANTS — events mirror state transitions and do not authorize settlement by themselves.
//! RO:SECURITY — event emission only; no hidden value movement.
//! RO:TEST — compile-tested through cargo check -p rox-anchor.

use anchor_lang::prelude::*;

#[event]
pub struct RoxAnchorInitialized {
    pub authority: Pubkey,
    pub rox_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
}

#[event]
pub struct RoxAnchorBurnObserved {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
}

#[event]
pub struct RoxAnchorChallengeOpened {
    pub operation_id_hash: [u8; 32],
}

#[event]
pub struct RoxAnchorChallengeResolved {
    pub operation_id_hash: [u8; 32],
    pub accepted: bool,
}

#[event]
pub struct RoxAnchorHalted {
    pub authority: Pubkey,
}

#[event]
pub struct RoxAnchorRecovered {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
}

#[event]
pub struct RoxAnchorFinalized {
    pub operation_id_hash: [u8; 32],
}
