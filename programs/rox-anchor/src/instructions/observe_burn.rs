//! RO:WHAT — Record a local burn observation operation.
//! RO:WHY — Creates operation state before challenge/finalize transitions.
//! RO:INTERACTS — config, operation record, OperationBindingArgs, and burn-observed event.
//! RO:INVARIANTS — halted/recovery-required config blocks observation; operation account is PDA-bound to config + operation hash.
//! RO:SECURITY — records observation metadata only; no token mint/burn behavior.
//! RO:TEST — cargo test -p rox-anchor.

use anchor_lang::prelude::*;

use crate::{OperationBindingArgs, RoxAnchorBurnObserved, RoxAnchorConfig, RoxAnchorOperation};

#[derive(Accounts)]
#[instruction(args: OperationBindingArgs)]
pub struct ObserveBurn<'info> {
    #[account(mut)]
    pub config: Account<'info, RoxAnchorConfig>,
    #[account(
        init,
        payer = payer,
        space = RoxAnchorOperation::SPACE,
        seeds = [
            RoxAnchorOperation::SEED_PREFIX,
            config.key().as_ref(),
            args.operation_id_hash.as_ref(),
        ],
        bump
    )]
    pub operation: Account<'info, RoxAnchorOperation>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ObserveBurn>, args: OperationBindingArgs) -> Result<()> {
    let config = &ctx.accounts.config;
    config.require_observation_open()?;

    let operation = &mut ctx.accounts.operation;
    operation.initialize_with_bump(ctx.accounts.payer.key(), args, ctx.bumps.operation)?;

    emit!(RoxAnchorBurnObserved {
        authority: ctx.accounts.payer.key(),
        operation_id_hash: operation.operation_id_hash,
        direction: operation.direction,
        mint: operation.mint,
        token_account: operation.token_account,
        amount_atoms: operation.amount_atoms,
        burn_evidence_hash: operation.burn_evidence_hash,
    });

    Ok(())
}
