//! RO:WHAT — Recover program/operation state after approved recovery.
//! RO:WHY — Clears halt/recovery blockers through explicit authority-controlled transition.
//! RO:INTERACTS — config, PDA-bound operation record, and recovery event.
//! RO:INVARIANTS — wrong authority cannot recover; finalized operation cannot recover.
//! RO:SECURITY — state transition only; no value movement.
//! RO:TEST — cargo check -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{RoxAnchorConfig, RoxAnchorOperation, RoxAnchorRecovered};

#[derive(Accounts)]
pub struct Recover<'info> {
    #[account(mut)]
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
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<Recover>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let operation = &mut ctx.accounts.operation;
    let authority = ctx.accounts.authority.key();

    config.recover(authority)?;
    operation.recover()?;

    emit!(RoxAnchorRecovered {
        authority,
        operation_id_hash: operation.operation_id_hash,
    });

    Ok(())
}
