//! RO:WHAT — Initialize the ROX Anchor config account.
//! RO:WHY — Establishes separated workflow/halt/recovery roles and ROX mint PDA binding.
//! RO:INTERACTS — RoxAnchorConfig, InitializeConfigArgs, and RoxAnchorInitialized event.
//! RO:INVARIANTS — live config requires pairwise-separated operator roles and the derived ROX mint authority PDA.
//! RO:SECURITY — initialization binds authority only; no mint/burn/settlement behavior.
//! RO:TEST — cargo test -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{InitializeConfigArgs, RoxAnchorConfig, RoxAnchorInitialized};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = RoxAnchorConfig::SPACE)]
    pub config: Account<'info, RoxAnchorConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub halt_authority: Signer<'info>,
    pub recovery_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, args: InitializeConfigArgs) -> Result<()> {
    let config_key = ctx.accounts.config.key();
    let authority = ctx.accounts.payer.key();
    let halt_authority = ctx.accounts.halt_authority.key();
    let recovery_authority = ctx.accounts.recovery_authority.key();

    let expected_args =
        RoxAnchorConfig::derived_initialize_args(ctx.program_id, &config_key, args.rox_mint)?;

    require!(
        args == expected_args,
        crate::RoxAnchorError::MintAuthorityMismatch
    );

    let config = &mut ctx.accounts.config;

    config.initialize_with_separated_authorities(
        authority,
        halt_authority,
        recovery_authority,
        args,
    )?;

    emit!(RoxAnchorInitialized {
        authority,
        halt_authority,
        recovery_authority,
        rox_mint: config.rox_mint,
        mint_authority: config.mint_authority,
        mint_authority_bump: config.mint_authority_bump,
    });

    Ok(())
}
