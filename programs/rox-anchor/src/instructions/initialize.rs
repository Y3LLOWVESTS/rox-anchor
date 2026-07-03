//! RO:WHAT — Initialize the ROX Anchor config account.
//! RO:WHY — Establishes local program authority and ROX mint authority binding.
//! RO:INTERACTS — RoxAnchorConfig, InitializeConfigArgs, and RoxAnchorInitialized event.
//! RO:INVARIANTS — config must bind non-default authority, ROX mint, and mint authority.
//! RO:SECURITY — no mint/burn/settlement behavior.
//! RO:TEST — cargo test -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{InitializeConfigArgs, RoxAnchorConfig, RoxAnchorInitialized};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = RoxAnchorConfig::SPACE)]
    pub config: Account<'info, RoxAnchorConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, args: InitializeConfigArgs) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let authority = ctx.accounts.payer.key();

    config.initialize(authority, args)?;

    emit!(RoxAnchorInitialized {
        authority,
        rox_mint: config.rox_mint,
        mint_authority: config.mint_authority,
        mint_authority_bump: config.mint_authority_bump,
    });

    Ok(())
}
