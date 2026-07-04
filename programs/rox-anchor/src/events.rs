//! RO:WHAT — Anchor events for ROX Anchor state transitions.
//! RO:WHY — Makes local program transitions observable once validator tests are added.
//! RO:INTERACTS — instruction handlers.
//! RO:INVARIANTS — events mirror state transitions and do not authorize settlement by themselves.
//! RO:SECURITY — event emission only; no hidden value movement.
//! RO:TEST — compile-tested through cargo check -p rox-anchor.

use anchor_lang::prelude::*;

use crate::state::{
    AnchorFinalizePlan, AnchorTokenSettlementExecutionReceipt, OperationStateCode,
    RoxAnchorOperation,
};

#[event]
pub struct RoxAnchorInitialized {
    pub authority: Pubkey,
    pub rox_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
}

#[event]
pub struct RoxAnchorBurnObserved {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
}

#[event]
pub struct RoxAnchorChallengeOpened {
    pub operation_id_hash: [u8; 32],
}

#[event]
pub struct RoxAnchorChallengeResolved {
    pub operation_id_hash: [u8; 32],
    pub accepted: bool,
}

#[event]
pub struct RoxAnchorHalted {
    pub authority: Pubkey,
}

#[event]
pub struct RoxAnchorRecovered {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
}

#[event]
pub struct RoxAnchorFinalized {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub settlement_action: u8,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
    pub requires_rox_mint_output: bool,
    pub requires_internal_roc_release: bool,
}

#[event]
pub struct RoxAnchorTokenSettlementPlanned {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub execution_kind: u8,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub amount_atoms: u64,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub used_mint_authority_pda: bool,
    pub token_mint_cpi_planned: bool,
    pub internal_roc_release_planned: bool,
    pub live_value_moved: bool,
}

impl RoxAnchorTokenSettlementPlanned {
    pub fn from_execution_receipt(
        authority: Pubkey,
        operation: &RoxAnchorOperation,
        receipt: AnchorTokenSettlementExecutionReceipt,
    ) -> Result<Self> {
        require!(
            operation.state_code() == Some(OperationStateCode::Finalized),
            crate::RoxAnchorError::InvalidStateTransition
        );
        require!(
            operation.authority == authority,
            crate::RoxAnchorError::AuthorityMismatch
        );
        require!(
            operation.direction_code().is_some(),
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            receipt.operation_id_hash == operation.operation_id_hash,
            crate::RoxAnchorError::OperationBindingMismatch
        );
        require!(
            receipt.direction == operation.direction,
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            receipt.mint == operation.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            receipt.token_account == operation.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            receipt.amount_atoms == operation.amount_atoms,
            crate::RoxAnchorError::AmountBindingMismatch
        );
        require!(
            receipt.used_mint_authority_pda,
            crate::RoxAnchorError::MintAuthorityMismatch
        );
        require!(
            receipt.token_account_owner != Pubkey::default(),
            crate::RoxAnchorError::InvalidBinding
        );
        require!(
            !receipt.live_value_moved,
            crate::RoxAnchorError::InvalidStateTransition
        );
        require!(
            receipt.is_roc_to_rox_mint_receipt() || receipt.is_rox_to_roc_release_receipt(),
            crate::RoxAnchorError::InvalidStateTransition
        );

        Ok(Self {
            authority,
            operation_id_hash: receipt.operation_id_hash,
            execution_kind: receipt.execution_kind,
            direction: receipt.direction,
            mint: receipt.mint,
            token_account: receipt.token_account,
            token_account_owner: receipt.token_account_owner,
            amount_atoms: receipt.amount_atoms,
            mint_authority: receipt.mint_authority,
            mint_authority_bump: receipt.mint_authority_bump,
            used_mint_authority_pda: receipt.used_mint_authority_pda,
            token_mint_cpi_planned: receipt.token_mint_cpi_planned,
            internal_roc_release_planned: receipt.internal_roc_release_planned,
            live_value_moved: receipt.live_value_moved,
        })
    }

    pub fn is_local_plan_only(&self) -> bool {
        !self.live_value_moved
    }
}

impl RoxAnchorFinalized {
    pub fn from_operation_plan(
        authority: Pubkey,
        operation: &RoxAnchorOperation,
        plan: AnchorFinalizePlan,
    ) -> Result<Self> {
        require!(
            operation.state_code() == Some(OperationStateCode::Finalized),
            crate::RoxAnchorError::InvalidStateTransition
        );
        require!(
            operation.authority == authority,
            crate::RoxAnchorError::AuthorityMismatch
        );
        plan.require_matches_operation(operation)?;

        Ok(Self {
            authority,
            operation_id_hash: plan.operation_id_hash,
            settlement_action: plan.settlement_action.as_u8(),
            direction: plan.direction,
            mint: plan.mint,
            token_account: plan.token_account,
            amount_atoms: plan.amount_atoms,
            burn_evidence_hash: plan.burn_evidence_hash,
            requires_rox_mint_output: plan.requires_rox_mint_output,
            requires_internal_roc_release: plan.requires_internal_roc_release,
        })
    }
}
