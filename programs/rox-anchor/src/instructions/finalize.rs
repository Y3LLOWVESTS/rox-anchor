//! RO:WHAT — Finalize eligible ROX Anchor operations and execute explicit local SPL mint/burn CPI paths.
//! RO:WHY — Terminal state transition after local proof/challenge/recovery blockers are clear.
//! RO:INTERACTS — config, authority signer, PDA-bound operation record, token mint/account constraints, SPL token program, and events.
//! RO:INVARIANTS — only configured authority can finalize; halted/challenged/recovery-required/finalized/mismatched operations cannot finalize or mint.
//! RO:SECURITY — local token CPI is explicit and constrained; mint uses PDA authority, burn uses the source token owner signer.
//! RO:TEST — cargo test -p rox-anchor.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

use crate::{
    AnchorTokenAccountConstraintSnapshot, AnchorTokenCpiExecutionReceipt, AnchorTokenCpiReadiness,
    AnchorTokenSettlementBinding, AnchorTokenSettlementExecutionPlan,
    AnchorTokenSettlementExecutionReceipt, AnchorTransferDirection, RoxAnchorConfig,
    RoxAnchorFinalized, RoxAnchorOperation, RoxAnchorTokenSettlementExecuted,
    RoxAnchorTokenSettlementPlanned,
};

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

#[derive(Accounts)]
pub struct FinalizeRocToRoxMint<'info> {
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
    #[account(
        mut,
        address = config.rox_mint @ crate::RoxAnchorError::MintBindingMismatch
    )]
    pub rox_mint: Account<'info, Mint>,
    #[account(
        mut,
        address = operation.token_account @ crate::RoxAnchorError::TokenAccountBindingMismatch,
        constraint = recipient_rox_token_account.mint == config.rox_mint @ crate::RoxAnchorError::MintBindingMismatch
    )]
    pub recipient_rox_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            RoxAnchorConfig::MINT_AUTHORITY_SEED_PREFIX,
            config.key().as_ref(),
            config.rox_mint.as_ref(),
        ],
        bump = config.mint_authority_bump,
        address = config.mint_authority @ crate::RoxAnchorError::MintAuthorityMismatch
    )]
    /// CHECK: PDA-only mint authority; no account data is read, and seeds/address are enforced above.
    pub mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FinalizeRoxToRocReleaseReview<'info> {
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
    #[account(
        address = config.rox_mint @ crate::RoxAnchorError::MintBindingMismatch
    )]
    pub rox_mint: Account<'info, Mint>,
    #[account(
        address = operation.token_account @ crate::RoxAnchorError::TokenAccountBindingMismatch,
        constraint = source_rox_token_account.mint == config.rox_mint @ crate::RoxAnchorError::MintBindingMismatch,
        constraint = source_rox_token_account.amount >= operation.amount_atoms @ crate::RoxAnchorError::AmountBindingMismatch
    )]
    pub source_rox_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            RoxAnchorConfig::MINT_AUTHORITY_SEED_PREFIX,
            config.key().as_ref(),
            config.rox_mint.as_ref(),
        ],
        bump = config.mint_authority_bump,
        address = config.mint_authority @ crate::RoxAnchorError::MintAuthorityMismatch
    )]
    /// CHECK: PDA-only mint authority binding; no account data is read, and no CPI is executed in this review path.
    pub mint_authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct FinalizeRoxToRocBurn<'info> {
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
    #[account(
        mut,
        address = config.rox_mint @ crate::RoxAnchorError::MintBindingMismatch
    )]
    pub rox_mint: Account<'info, Mint>,
    #[account(
        mut,
        address = operation.token_account @ crate::RoxAnchorError::TokenAccountBindingMismatch,
        constraint = source_rox_token_account.mint == config.rox_mint @ crate::RoxAnchorError::MintBindingMismatch,
        constraint = source_rox_token_account.owner == source_rox_token_authority.key() @ crate::RoxAnchorError::AuthorityMismatch,
        constraint = source_rox_token_account.amount >= operation.amount_atoms @ crate::RoxAnchorError::AmountBindingMismatch
    )]
    pub source_rox_token_account: Account<'info, TokenAccount>,
    pub source_rox_token_authority: Signer<'info>,
    #[account(
        seeds = [
            RoxAnchorConfig::MINT_AUTHORITY_SEED_PREFIX,
            config.key().as_ref(),
            config.rox_mint.as_ref(),
        ],
        bump = config.mint_authority_bump,
        address = config.mint_authority @ crate::RoxAnchorError::MintAuthorityMismatch
    )]
    /// CHECK: PDA-only mint authority binding; burn CPI uses source_rox_token_authority, not this PDA.
    pub mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
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

pub fn handler_roc_to_rox_mint(ctx: Context<FinalizeRocToRoxMint>) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();
    let config_key = ctx.accounts.config.key();
    let rox_mint_key = ctx.accounts.rox_mint.key();

    config.require_authority(authority)?;
    config.require_derived_mint_authority(ctx.program_id, &config_key)?;
    config.require_rox_mint(rox_mint_key)?;
    config.require_mint_authority(ctx.accounts.mint_authority.key())?;

    require!(
        ctx.accounts.rox_mint.mint_authority == COption::Some(config.mint_authority),
        crate::RoxAnchorError::MintAuthorityMismatch
    );

    let operation_account_key = ctx.accounts.operation.key();
    let operation = &mut ctx.accounts.operation;
    operation.require_derived_address(ctx.program_id, &config_key, operation_account_key)?;
    operation.require_roc_to_rox()?;

    let finalize_plan =
        operation.finalize_for_direction(config, AnchorTransferDirection::RocToRox)?;
    let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
        config,
        ctx.program_id,
        &config_key,
        finalize_plan,
    )?;

    let snapshot = AnchorTokenAccountConstraintSnapshot {
        mint: rox_mint_key,
        token_account: ctx.accounts.recipient_rox_token_account.key(),
        token_account_mint: ctx.accounts.recipient_rox_token_account.mint,
        token_account_owner: ctx.accounts.recipient_rox_token_account.owner,
        token_account_amount_atoms: ctx.accounts.recipient_rox_token_account.amount,
        mint_authority: ctx.accounts.mint_authority.key(),
    };

    let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
        config,
        ctx.program_id,
        &config_key,
        &settlement,
        &finalize_plan,
        &snapshot,
    )?;

    let planned_receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
        operation,
        &finalize_plan,
        &execution_plan,
    )?;

    let planned_event = RoxAnchorTokenSettlementPlanned::from_execution_receipt(
        authority,
        operation,
        planned_receipt,
    )?;

    let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
        config,
        ctx.program_id,
        &config_key,
        &planned_receipt,
        &planned_event,
    )?;

    require!(
        readiness.is_ready_for_local_token_mint_cpi(),
        crate::RoxAnchorError::InvalidStateTransition
    );

    let pre_mint_amount_atoms = ctx.accounts.recipient_rox_token_account.amount;
    let mint_authority_bump = readiness.mint_authority_bump_bytes();
    let signer_seeds = RoxAnchorConfig::mint_authority_signer_seeds(
        &config_key,
        &rox_mint_key,
        &mint_authority_bump,
    );
    let signer = &[&signer_seeds[..]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.rox_mint.to_account_info(),
                to: ctx.accounts.recipient_rox_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer,
        ),
        readiness.amount_atoms,
    )?;

    ctx.accounts.recipient_rox_token_account.reload()?;

    let executed_receipt = AnchorTokenCpiExecutionReceipt::from_readiness_and_amounts(
        &readiness,
        pre_mint_amount_atoms,
        ctx.accounts.recipient_rox_token_account.amount,
    )?;

    let finalized_event =
        RoxAnchorFinalized::from_operation_plan(authority, operation, finalize_plan)?;
    let executed_event =
        RoxAnchorTokenSettlementExecuted::from_cpi_receipt(authority, operation, executed_receipt)?;

    emit!(finalized_event);
    emit!(planned_event);
    emit!(executed_event);

    Ok(())
}

pub fn handler_rox_to_roc_release_review(
    ctx: Context<FinalizeRoxToRocReleaseReview>,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();
    let config_key = ctx.accounts.config.key();
    let rox_mint_key = ctx.accounts.rox_mint.key();

    config.require_authority(authority)?;
    config.require_derived_mint_authority(ctx.program_id, &config_key)?;
    config.require_rox_mint(rox_mint_key)?;
    config.require_mint_authority(ctx.accounts.mint_authority.key())?;

    require!(
        ctx.accounts.rox_mint.mint_authority == COption::Some(config.mint_authority),
        crate::RoxAnchorError::MintAuthorityMismatch
    );

    let operation_account_key = ctx.accounts.operation.key();
    let operation = &mut ctx.accounts.operation;
    operation.require_derived_address(ctx.program_id, &config_key, operation_account_key)?;
    operation.require_rox_to_roc()?;

    let finalize_plan =
        operation.finalize_for_direction(config, AnchorTransferDirection::RoxToRoc)?;
    let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
        config,
        ctx.program_id,
        &config_key,
        finalize_plan,
    )?;

    let snapshot = AnchorTokenAccountConstraintSnapshot {
        mint: rox_mint_key,
        token_account: ctx.accounts.source_rox_token_account.key(),
        token_account_mint: ctx.accounts.source_rox_token_account.mint,
        token_account_owner: ctx.accounts.source_rox_token_account.owner,
        token_account_amount_atoms: ctx.accounts.source_rox_token_account.amount,
        mint_authority: ctx.accounts.mint_authority.key(),
    };

    let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
        config,
        ctx.program_id,
        &config_key,
        &settlement,
        &finalize_plan,
        &snapshot,
    )?;

    let planned_receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
        operation,
        &finalize_plan,
        &execution_plan,
    )?;

    let planned_event = RoxAnchorTokenSettlementPlanned::from_execution_receipt(
        authority,
        operation,
        planned_receipt,
    )?;

    let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
        config,
        ctx.program_id,
        &config_key,
        &planned_receipt,
        &planned_event,
    )?;

    require!(
        readiness.is_ready_for_internal_roc_release_review(),
        crate::RoxAnchorError::InvalidStateTransition
    );

    let finalized_event =
        RoxAnchorFinalized::from_operation_plan(authority, operation, finalize_plan)?;

    emit!(finalized_event);
    emit!(planned_event);

    Ok(())
}

pub fn handler_rox_to_roc_burn(ctx: Context<FinalizeRoxToRocBurn>) -> Result<()> {
    let config = &ctx.accounts.config;
    let authority = ctx.accounts.authority.key();
    let token_authority = ctx.accounts.source_rox_token_authority.key();
    let config_key = ctx.accounts.config.key();
    let rox_mint_key = ctx.accounts.rox_mint.key();

    config.require_authority(authority)?;
    config.require_derived_mint_authority(ctx.program_id, &config_key)?;
    config.require_rox_mint(rox_mint_key)?;
    config.require_mint_authority(ctx.accounts.mint_authority.key())?;

    require!(
        ctx.accounts.rox_mint.mint_authority == COption::Some(config.mint_authority),
        crate::RoxAnchorError::MintAuthorityMismatch
    );
    require!(
        ctx.accounts.source_rox_token_account.owner == token_authority,
        crate::RoxAnchorError::AuthorityMismatch
    );

    let operation_account_key = ctx.accounts.operation.key();
    let operation = &mut ctx.accounts.operation;
    operation.require_derived_address(ctx.program_id, &config_key, operation_account_key)?;
    operation.require_rox_to_roc()?;

    let finalize_plan =
        operation.finalize_for_direction(config, AnchorTransferDirection::RoxToRoc)?;
    let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
        config,
        ctx.program_id,
        &config_key,
        finalize_plan,
    )?;

    let snapshot = AnchorTokenAccountConstraintSnapshot {
        mint: rox_mint_key,
        token_account: ctx.accounts.source_rox_token_account.key(),
        token_account_mint: ctx.accounts.source_rox_token_account.mint,
        token_account_owner: ctx.accounts.source_rox_token_account.owner,
        token_account_amount_atoms: ctx.accounts.source_rox_token_account.amount,
        mint_authority: ctx.accounts.mint_authority.key(),
    };

    let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
        config,
        ctx.program_id,
        &config_key,
        &settlement,
        &finalize_plan,
        &snapshot,
    )?;

    let planned_receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
        operation,
        &finalize_plan,
        &execution_plan,
    )?;

    let planned_event = RoxAnchorTokenSettlementPlanned::from_execution_receipt(
        authority,
        operation,
        planned_receipt,
    )?;

    let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
        config,
        ctx.program_id,
        &config_key,
        &planned_receipt,
        &planned_event,
    )?;

    require!(
        readiness.is_ready_for_local_rox_burn_cpi(),
        crate::RoxAnchorError::InvalidStateTransition
    );

    let pre_burn_amount_atoms = ctx.accounts.source_rox_token_account.amount;

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.rox_mint.to_account_info(),
                from: ctx.accounts.source_rox_token_account.to_account_info(),
                authority: ctx.accounts.source_rox_token_authority.to_account_info(),
            },
        ),
        readiness.amount_atoms,
    )?;

    ctx.accounts.source_rox_token_account.reload()?;

    let executed_receipt = AnchorTokenCpiExecutionReceipt::from_rox_burn_readiness_and_amounts(
        &readiness,
        pre_burn_amount_atoms,
        ctx.accounts.source_rox_token_account.amount,
    )?;

    let finalized_event =
        RoxAnchorFinalized::from_operation_plan(authority, operation, finalize_plan)?;
    let executed_event =
        RoxAnchorTokenSettlementExecuted::from_cpi_receipt(authority, operation, executed_receipt)?;

    emit!(finalized_event);
    emit!(planned_event);
    emit!(executed_event);

    Ok(())
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
