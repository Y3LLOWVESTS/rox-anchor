//! RO:WHAT — Builds the exact BUILD_PLAN4 Phase 4 atomic initialization transaction plan.
//! RO:WHY — Proves mint/config/ATA initialization can be assembled before any live signer or RPC executor exists.
//! RO:INTERACTS — ROX Anchor Initialize instruction, classic SPL Token mint initialization,
//! Associated Token Account creation, and System Program mint-account creation.
//! RO:INVARIANTS — devnet/testnet pilot profile is exactly 1000 total units and 10 units per
//! operation; mint decimals are zero; mint authority is the ROX Anchor PDA; freeze authority is
//! absent; critical operator identities are pairwise distinct.
//! RO:SECURITY — instruction construction only; no key loading, RPC, signing, submission, minting,
//! burning, ROC mutation, settlement, or mainnet behavior.
//! RO:TEST — cargo test -p rox-anchor-cli --test phase4_live_init_plan.

#![forbid(unsafe_code)]

use std::{collections::BTreeSet, error::Error, fmt};

use anchor_lang::{
    solana_program::{instruction::Instruction, program_pack::Pack, pubkey::Pubkey},
    InstructionData, ToAccountMetas,
};
use rox_anchor::{InitializeConfigArgs, RoxAnchorConfig};
use solana_sdk_ids::system_program;
use solana_system_interface::instruction as system_instruction;
use spl_token::state::Mint;

pub const PHASE4_PRIVATE_MAX_SUPPLY_UNITS: u64 = 1_000;
pub const PHASE4_PRIVATE_MAX_AMOUNT_UNITS_PER_OPERATION: u64 = 10;
pub const PHASE4_TEST_ONLY_MINT_DECIMALS: u8 = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Phase4LiveInitRequest {
    pub program_id: Pubkey,
    pub payer: Pubkey,
    pub config: Pubkey,
    pub test_only_mint: Pubkey,
    pub halt_authority: Pubkey,
    pub recovery_authority: Pubkey,
    pub upgrade_authority: Pubkey,
    pub mint_rent_lamports: u64,
    pub max_supply_units: u64,
    pub max_amount_units_per_operation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Phase4LiveInitPlan {
    pub program_id: Pubkey,
    pub payer: Pubkey,
    pub config: Pubkey,
    pub test_only_mint: Pubkey,
    pub test_only_token_account: Pubkey,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub halt_authority: Pubkey,
    pub recovery_authority: Pubkey,
    pub upgrade_authority: Pubkey,
    pub mint_rent_lamports: u64,
    pub mint_decimals: u8,
    pub max_supply_units: u64,
    pub max_amount_units_per_operation: u64,
    pub required_signers: BTreeSet<Pubkey>,
    pub instructions: Vec<Instruction>,
}

impl Phase4LiveInitPlan {
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_atomic_phase4_shape(&self) -> bool {
        self.instructions.len() == 4
            && self.instructions[0].program_id == system_program::id()
            && self.instructions[1].program_id == spl_token::id()
            && self.instructions[2].program_id == spl_associated_token_account::id()
            && self.instructions[3].program_id == self.program_id
    }

    pub fn redacted_summary(&self) -> String {
        [
            "phase4_live_initialization_backend: transaction_plan".to_string(),
            format!("program_id: {}", self.program_id),
            "cluster_scope: devnet_or_testnet_only".to_string(),
            "instruction_count: 4".to_string(),
            "instruction_1: create_test_only_spl_mint_account".to_string(),
            "instruction_2: initialize_zero_decimal_mint_with_program_pda".to_string(),
            "instruction_3: create_payer_associated_token_account_idempotent".to_string(),
            "instruction_4: initialize_rox_anchor_config".to_string(),
            format!("mint_decimals: {}", self.mint_decimals),
            format!("max_supply_units: {}", self.max_supply_units),
            format!(
                "max_amount_units_per_operation: {}",
                self.max_amount_units_per_operation
            ),
            "mint_authority_model: program_derived_pda".to_string(),
            "freeze_authority: none".to_string(),
            "required_signers: payer_config_mint_halt_recovery".to_string(),
            "upgrade_authority_signer_required_for_initialization: false".to_string(),
            "transaction_atomicity: single_transaction_plan".to_string(),
            "rpc_calls: disabled".to_string(),
            "keypair_loading: disabled".to_string(),
            "signing: disabled".to_string(),
            "transaction_submission: disabled".to_string(),
            "rox_mint_execution: disabled".to_string(),
            "rox_burn_execution: disabled".to_string(),
            "real_roc_mutation: disabled".to_string(),
            "mainnet_authorized: false".to_string(),
        ]
        .join("\n")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase4LiveInitPlanError {
    WrongProgramId,
    InvalidPilotCaps,
    ZeroMintRent,
    DefaultCriticalIdentity,
    SharedCriticalIdentity,
    DerivedMintAuthorityCollision,
    AnchorBindingRejected,
    TokenInstructionRejected,
    UnexpectedSignerSet,
}

impl fmt::Display for Phase4LiveInitPlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::WrongProgramId => "program id does not match the reviewed ROX Anchor deployment",
            Self::InvalidPilotCaps => {
                "private pilot caps must be exactly 1000 total and 10 per operation"
            }
            Self::ZeroMintRent => "mint rent lamports must be fetched and nonzero before execution",
            Self::DefaultCriticalIdentity => "critical Phase 4 identities must be non-default",
            Self::SharedCriticalIdentity => {
                "payer/config/mint/halt/recovery/upgrade identities must be pairwise distinct"
            }
            Self::DerivedMintAuthorityCollision => {
                "derived mint authority must remain distinct from operator identities"
            }
            Self::AnchorBindingRejected => {
                "ROX Anchor rejected derived initialization binding construction"
            }
            Self::TokenInstructionRejected => {
                "SPL Token rejected mint initialization instruction construction"
            }
            Self::UnexpectedSignerSet => {
                "atomic initialization plan produced an unexpected signer set"
            }
        };

        f.write_str(message)
    }
}

impl Error for Phase4LiveInitPlanError {}

pub fn build_phase4_live_init_plan(
    request: Phase4LiveInitRequest,
) -> Result<Phase4LiveInitPlan, Phase4LiveInitPlanError> {
    if request.program_id != rox_anchor::ID {
        return Err(Phase4LiveInitPlanError::WrongProgramId);
    }

    if request.max_supply_units != PHASE4_PRIVATE_MAX_SUPPLY_UNITS
        || request.max_amount_units_per_operation != PHASE4_PRIVATE_MAX_AMOUNT_UNITS_PER_OPERATION
    {
        return Err(Phase4LiveInitPlanError::InvalidPilotCaps);
    }

    if request.mint_rent_lamports == 0 {
        return Err(Phase4LiveInitPlanError::ZeroMintRent);
    }

    let critical_identities = [
        request.payer,
        request.config,
        request.test_only_mint,
        request.halt_authority,
        request.recovery_authority,
        request.upgrade_authority,
    ];

    if critical_identities
        .iter()
        .any(|key| *key == Pubkey::default())
    {
        return Err(Phase4LiveInitPlanError::DefaultCriticalIdentity);
    }

    let unique_critical: BTreeSet<Pubkey> = critical_identities.into_iter().collect();

    if unique_critical.len() != critical_identities.len() {
        return Err(Phase4LiveInitPlanError::SharedCriticalIdentity);
    }

    let args: InitializeConfigArgs = RoxAnchorConfig::derived_initialize_args(
        &request.program_id,
        &request.config,
        request.test_only_mint,
    )
    .map_err(|_| Phase4LiveInitPlanError::AnchorBindingRejected)?;

    if unique_critical.contains(&args.mint_authority) {
        return Err(Phase4LiveInitPlanError::DerivedMintAuthorityCollision);
    }

    let test_only_token_account = spl_associated_token_account::get_associated_token_address(
        &request.payer,
        &request.test_only_mint,
    );

    let create_mint_account = system_instruction::create_account(
        &request.payer,
        &request.test_only_mint,
        request.mint_rent_lamports,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint = spl_token::instruction::initialize_mint2(
        &spl_token::id(),
        &request.test_only_mint,
        &args.mint_authority,
        None,
        PHASE4_TEST_ONLY_MINT_DECIMALS,
    )
    .map_err(|_| Phase4LiveInitPlanError::TokenInstructionRejected)?;

    let create_payer_ata =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            &request.payer,
            &request.payer,
            &request.test_only_mint,
            &spl_token::id(),
        );

    let initialize_anchor_config = Instruction {
        program_id: request.program_id,
        accounts: rox_anchor::accounts::Initialize {
            config: request.config,
            payer: request.payer,
            halt_authority: request.halt_authority,
            recovery_authority: request.recovery_authority,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
        data: rox_anchor::instruction::Initialize { args }.data(),
    };

    let instructions = vec![
        create_mint_account,
        initialize_mint,
        create_payer_ata,
        initialize_anchor_config,
    ];

    let required_signers: BTreeSet<Pubkey> = instructions
        .iter()
        .flat_map(|instruction| instruction.accounts.iter())
        .filter(|account| account.is_signer)
        .map(|account| account.pubkey)
        .collect();

    let expected_signers: BTreeSet<Pubkey> = [
        request.payer,
        request.config,
        request.test_only_mint,
        request.halt_authority,
        request.recovery_authority,
    ]
    .into_iter()
    .collect();

    if required_signers != expected_signers {
        return Err(Phase4LiveInitPlanError::UnexpectedSignerSet);
    }

    Ok(Phase4LiveInitPlan {
        program_id: request.program_id,
        payer: request.payer,
        config: request.config,
        test_only_mint: request.test_only_mint,
        test_only_token_account,
        mint_authority: args.mint_authority,
        mint_authority_bump: args.mint_authority_bump,
        halt_authority: request.halt_authority,
        recovery_authority: request.recovery_authority,
        upgrade_authority: request.upgrade_authority,
        mint_rent_lamports: request.mint_rent_lamports,
        mint_decimals: PHASE4_TEST_ONLY_MINT_DECIMALS,
        max_supply_units: request.max_supply_units,
        max_amount_units_per_operation: request.max_amount_units_per_operation,
        required_signers,
        instructions,
    })
}
