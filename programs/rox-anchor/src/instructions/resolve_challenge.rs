//! RO:WHAT — Resolve an open challenge.
//! RO:WHY — Accepted challenges block finalization; rejected challenges can later become finality eligible.
//! RO:INTERACTS — config, PDA-bound operation record, and challenge-resolved event.
//! RO:INVARIANTS — only challenge-open operations can be resolved.
//! RO:SECURITY — state transition only; no value movement.
//! RO:TEST — cargo check -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{RoxAnchorChallengeResolved, RoxAnchorConfig, RoxAnchorOperation};

#[derive(Accounts)]
pub struct ResolveChallenge<'info> {
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

pub fn handler(ctx: Context<ResolveChallenge>, accepted: bool) -> Result<()> {
    let operation = &mut ctx.accounts.operation;

    operation.resolve_challenge(accepted)?;

    emit!(RoxAnchorChallengeResolved {
        operation_id_hash: operation.operation_id_hash,
        accepted,
    });

    Ok(())
}
