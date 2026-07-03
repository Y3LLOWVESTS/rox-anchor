//! RO:WHAT — Finalize an eligible ROX Anchor operation.
//! RO:WHY — Terminal state transition after local proof/challenge/recovery blockers are clear.
//! RO:INTERACTS — config, PDA-bound operation record, and finalized event.
//! RO:INVARIANTS — halted/challenged/recovery-required/finalized/mismatched operations cannot finalize.
//! RO:SECURITY — state transition only; no token mint/burn behavior yet.
//! RO:TEST — cargo check -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{RoxAnchorConfig, RoxAnchorFinalized, RoxAnchorOperation};

#[derive(Accounts)]
pub struct Finalize<'info> {
    pub config: Account<'info, RoxAnchorConfig>,
    #[account(
        mut,
        seeds = [
            RoxAnchorOperation::SEED_PREFIX,
            config.key().as_ref(),
            operation.operation_id_hash.as_ref(),
        ],
        bump = operation.operation_bump
    )]
    pub operation: Account<'info, RoxAnchorOperation>,
}

pub fn handler(ctx: Context<Finalize>) -> Result<()> {
    let config = &ctx.accounts.config;
    let operation = &mut ctx.accounts.operation;

    operation.finalize(config)?;

    emit!(RoxAnchorFinalized {
        operation_id_hash: operation.operation_id_hash,
    });

    Ok(())
}
