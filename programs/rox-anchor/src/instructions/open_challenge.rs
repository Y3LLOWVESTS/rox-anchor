//! RO:WHAT — Open a challenge against an observed operation.
//! RO:WHY — Moves operation state into a blocker before finalize can run.
//! RO:INTERACTS — config, PDA-bound operation record, and challenge-open event.
//! RO:INVARIANTS — finalized/recovery-required/challenge-open operations cannot be reopened.
//! RO:SECURITY — state transition only; no value movement.
//! RO:TEST — cargo check -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{RoxAnchorChallengeOpened, RoxAnchorConfig, RoxAnchorOperation};

#[derive(Accounts)]
pub struct OpenChallenge<'info> {
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

pub fn handler(ctx: Context<OpenChallenge>) -> Result<()> {
    let operation = &mut ctx.accounts.operation;

    operation.open_challenge()?;

    emit!(RoxAnchorChallengeOpened {
        operation_id_hash: operation.operation_id_hash,
    });

    Ok(())
}
