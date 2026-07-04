//! RO:WHAT — Finalize an eligible ROX Anchor operation.
//! RO:WHY — Terminal state transition after local proof/challenge/recovery blockers are clear.
//! RO:INTERACTS — config, configured authority signer, PDA-bound operation record, and finalized event.
//! RO:INVARIANTS — only configured authority can finalize; halted/challenged/recovery-required/finalized/mismatched operations cannot finalize.
//! RO:SECURITY — state transition/event emission only; no token mint/burn behavior yet.
//! RO:TEST — cargo test -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{AnchorTransferDirection, RoxAnchorConfig, RoxAnchorFinalized, RoxAnchorOperation};

#[derive(Accounts)]
pub struct Finalize<'info> {
    pub config: Account<'info, RoxAnchorConfig>,
    pub authority: Signer<'info>,
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
    finalize_with_direction(ctx, None)
}

pub fn handler_roc_to_rox(ctx: Context<Finalize>) -> Result<()> {
    finalize_with_direction(ctx, Some(AnchorTransferDirection::RocToRox))
}

pub fn handler_rox_to_roc(ctx: Context<Finalize>) -> Result<()> {
    finalize_with_direction(ctx, Some(AnchorTransferDirection::RoxToRoc))
}

fn finalize_with_direction(
    ctx: Context<Finalize>,
    expected_direction: Option<AnchorTransferDirection>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    config.require_authority(ctx.accounts.authority.key())?;

    let operation = &mut ctx.accounts.operation;

    let finalize_plan = match expected_direction {
        Some(direction) => operation.finalize_for_direction(config, direction)?,
        None => operation.finalize(config)?,
    };

    let finalized_event = RoxAnchorFinalized::from_operation_plan(
        ctx.accounts.authority.key(),
        operation,
        finalize_plan,
    )?;

    emit!(finalized_event);

    Ok(())
}
