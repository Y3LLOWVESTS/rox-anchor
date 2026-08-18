//! RO:WHAT — Halt sensitive ROX Anchor transitions.
//! RO:WHY — Gives program state an explicit blocker before finalize/mint behavior is added.
//! RO:INTERACTS — RoxAnchorConfig and halted event.
//! RO:INVARIANTS — signer must match the dedicated config halt authority.
//! RO:SECURITY — halt state only; no live deployment or value movement.
//! RO:TEST — cargo check -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{RoxAnchorConfig, RoxAnchorHalted};

#[derive(Accounts)]
pub struct Halt<'info> {
    #[account(mut)]
    pub config: Account<'info, RoxAnchorConfig>,
    pub halt_authority: Signer<'info>,
}

pub fn handler(ctx: Context<Halt>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let authority = ctx.accounts.halt_authority.key();

    config.halt(authority)?;

    emit!(RoxAnchorHalted { authority });

    Ok(())
}
