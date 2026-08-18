//! RO:WHAT — Prepares and simulates the exact BUILD_PLAN4 Phase 4 atomic
//! initialization transaction.
//! RO:WHY — Centralizes live key loading, RPC preflight, instruction construction,
//! local signing, and simulation so the submission path cannot invent a second
//! initialization policy.
//! RO:INTERACTS — Phase4LiveInitPlan, local ignored keypairs, Solana JSON-RPC,
//! classic SPL Token, Associated Token Program, and ROX Anchor.
//! RO:INVARIANTS — FiUY only; devnet/testnet only; exact 1000/10 caps;
//! program executable; config/mint/ATA absent; five transaction signers;
//! upgrade authority validation-only.
//! RO:SECURITY — this module has no transaction submission API. It prepares and
//! simulates transactions only.
//! RO:TEST — phase4_live_init_executor.rs and phase4_live_init_submission.rs.

#![forbid(unsafe_code)]

use std::{collections::BTreeSet, str::FromStr};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    },
};
use anchor_lang::solana_program::program_pack::Pack;
use rox_anchor::RoxAnchorConfig;
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::{
        phase4_live_init::{
            build_phase4_live_init_plan, Phase4LiveInitPlan, Phase4LiveInitRequest,
        },
        test_only_init::Phase4InitConfig,
    },
    CliError,
};

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

const TESTNET_RPC_URL: &str = "https://api.testnet.solana.com";

pub(crate) struct PreparedPhase4Transaction {
    pub(crate) rpc: RpcClient,
    pub(crate) transaction: Transaction,
    pub(crate) plan: Phase4LiveInitPlan,
    pub(crate) payer: Pubkey,
    pub(crate) halt_authority: Pubkey,
    pub(crate) recovery_authority: Pubkey,
    pub(crate) upgrade_authority: Pubkey,
    pub(crate) payer_balance: u64,
    pub(crate) mint_rent: u64,
    pub(crate) config_rent: u64,
    pub(crate) token_account_rent: u64,
    pub(crate) required_rent: u64,
    pub(crate) signature_count: usize,
}

pub(crate) fn run_phase4_live_simulation(config: &Phase4InitConfig) -> Result<String, CliError> {
    let prepared = prepare_phase4_live_transaction(config)?;

    simulate_prepared_transaction(&prepared)?;

    Ok(render_green_report(config, &prepared))
}

pub(crate) fn prepare_phase4_live_transaction(
    config: &Phase4InitConfig,
) -> Result<PreparedPhase4Transaction, CliError> {
    let rpc_url = rpc_url_for_cluster(&config.cluster)?;

    let program_id = Pubkey::from_str(&config.program_id)
        .map_err(|_| phase4_error("program_id is not a valid Solana public key"))?;

    if program_id != rox_anchor::ID {
        return Err(phase4_error(
            "program_id does not match compiled ROX Anchor ID",
        ));
    }

    let payer = load_keypair(&config.payer_keypair_path, "payer")?;

    let program_config = load_keypair(&config.program_config_keypair_path, "program config")?;

    let test_only_mint = load_keypair(&config.test_only_mint_keypair_path, "test-only mint")?;

    let halt_authority = load_keypair(&config.halt_authority_path, "halt authority")?;

    let recovery_authority = load_keypair(&config.recovery_authority_path, "recovery authority")?;

    let upgrade_authority = load_keypair(&config.upgrade_authority_path, "upgrade authority")?;

    let operator_pubkeys = [
        payer.pubkey(),
        program_config.pubkey(),
        test_only_mint.pubkey(),
        halt_authority.pubkey(),
        recovery_authority.pubkey(),
        upgrade_authority.pubkey(),
    ];

    let unique: BTreeSet<Pubkey> = operator_pubkeys.into_iter().collect();

    if unique.len() != operator_pubkeys.len() {
        return Err(phase4_error(
            "loaded payer/config/mint/halt/recovery/upgrade public keys must be pairwise distinct",
        ));
    }

    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let mint_rent = rpc
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .map_err(|_| phase4_error("RPC failed to fetch SPL mint rent"))?;

    let config_rent = rpc
        .get_minimum_balance_for_rent_exemption(RoxAnchorConfig::SPACE)
        .map_err(|_| phase4_error("RPC failed to fetch ROX Anchor config rent"))?;

    let token_account_rent = rpc
        .get_minimum_balance_for_rent_exemption(SplTokenAccount::LEN)
        .map_err(|_| phase4_error("RPC failed to fetch SPL token-account rent"))?;

    let plan = build_phase4_live_init_plan(Phase4LiveInitRequest {
        program_id,
        payer: payer.pubkey(),
        config: program_config.pubkey(),
        test_only_mint: test_only_mint.pubkey(),
        halt_authority: halt_authority.pubkey(),
        recovery_authority: recovery_authority.pubkey(),
        upgrade_authority: upgrade_authority.pubkey(),
        mint_rent_lamports: mint_rent,
        max_supply_units: config.max_supply_units,
        max_amount_units_per_operation: config.max_amount_units_per_operation,
    })
    .map_err(|error| {
        phase4_error(&format!(
            "atomic initialization plan rejected live bindings: {error}"
        ))
    })?;

    let accounts = rpc
        .get_multiple_accounts(&[
            program_id,
            program_config.pubkey(),
            test_only_mint.pubkey(),
            plan.test_only_token_account,
        ])
        .map_err(|_| phase4_error("RPC account preflight failed"))?;

    if accounts.len() != 4 {
        return Err(phase4_error(
            "RPC account preflight returned an unexpected account count",
        ));
    }

    let program_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase4_error("reviewed ROX Anchor program account is missing"))?;

    if !program_account.executable {
        return Err(phase4_error(
            "reviewed ROX Anchor program account is not executable",
        ));
    }

    if accounts[1].is_some() {
        return Err(phase4_error(
            "program config account already exists; initialization is not safe to repeat",
        ));
    }

    if accounts[2].is_some() {
        return Err(phase4_error(
            "test-only ROX mint account already exists; initialization is not safe to repeat",
        ));
    }

    if accounts[3].is_some() {
        return Err(
            phase4_error(
                "derived payer associated token account already exists before test-only mint initialization",
            ),
        );
    }

    let payer_balance = rpc
        .get_balance(&payer.pubkey())
        .map_err(|_| phase4_error("RPC failed to read payer balance"))?;

    let required_rent = mint_rent
        .checked_add(config_rent)
        .and_then(|value| value.checked_add(token_account_rent))
        .ok_or_else(|| phase4_error("rent requirement overflowed u64"))?;

    if payer_balance <= required_rent {
        return Err(phase4_error(
            "payer balance is not above the combined mint/config/token-account rent requirement",
        ));
    }

    let blockhash = rpc
        .get_latest_blockhash()
        .map_err(|_| phase4_error("RPC failed to fetch a recent blockhash"))?;

    let signers: [&dyn Signer; 5] = [
        &payer,
        &program_config,
        &test_only_mint,
        &halt_authority,
        &recovery_authority,
    ];

    let transaction = Transaction::new_signed_with_payer(
        &plan.instructions,
        Some(&payer.pubkey()),
        &signers,
        blockhash,
    );

    let signature_count = transaction.signatures.len();

    if signature_count != 5 {
        return Err(phase4_error(
            "signed Phase 4 transaction did not contain the expected five signatures",
        ));
    }

    Ok(PreparedPhase4Transaction {
        rpc,
        transaction,
        plan,
        payer: payer.pubkey(),
        halt_authority: halt_authority.pubkey(),
        recovery_authority: recovery_authority.pubkey(),
        upgrade_authority: upgrade_authority.pubkey(),
        payer_balance,
        mint_rent,
        config_rent,
        token_account_rent,
        required_rent,
        signature_count,
    })
}

pub(crate) fn simulate_prepared_transaction(
    prepared: &PreparedPhase4Transaction,
) -> Result<(), CliError> {
    let simulation = prepared
        .rpc
        .simulate_transaction(&prepared.transaction)
        .map_err(|_| phase4_error("Solana RPC simulation request failed"))?;

    if let Some(error) = simulation.value.err.as_ref() {
        return Err(phase4_error(&format!(
            "atomic initialization simulation rejected: {error:?}"
        )));
    }

    Ok(())
}

fn rpc_url_for_cluster(cluster: &str) -> Result<&'static str, CliError> {
    match cluster {
        "devnet" => Ok(DEVNET_RPC_URL),
        "testnet" => Ok(TESTNET_RPC_URL),
        _ => Err(phase4_error(
            "live Phase 4 cluster must be devnet or testnet",
        )),
    }
}

fn load_keypair(file_path: &str, role: &'static str) -> Result<Keypair, CliError> {
    read_keypair_file(file_path)
        .map_err(|_| phase4_error(&format!("could not load the configured {role} keypair")))
}

fn render_green_report(config: &Phase4InitConfig, prepared: &PreparedPhase4Transaction) -> String {
    [
        "phase4_live_initialization_executor: simulation_only".to_string(),
        "phase: BUILD_PLAN4 Phase 4".to_string(),
        format!("cluster: {}", config.cluster),
        format!("program_id: {}", prepared.plan.program_id),
        format!("workflow_payer: {}", prepared.payer),
        format!("program_config: {}", prepared.plan.config),
        format!("test_only_rox_mint: {}", prepared.plan.test_only_mint),
        format!(
            "test_only_token_account: {}",
            prepared.plan.test_only_token_account
        ),
        format!("mint_authority_pda: {}", prepared.plan.mint_authority),
        format!("halt_authority: {}", prepared.halt_authority),
        format!("recovery_authority: {}", prepared.recovery_authority),
        format!("upgrade_authority: {}", prepared.upgrade_authority),
        "operator_pubkey_separation: verified".to_string(),
        "program_account_executable: true".to_string(),
        "program_config_account_exists: false".to_string(),
        "test_only_mint_account_exists: false".to_string(),
        "test_only_token_account_exists: false".to_string(),
        format!("mint_decimals: {}", prepared.plan.mint_decimals),
        format!("max_supply_units: {}", prepared.plan.max_supply_units),
        format!(
            "max_amount_units_per_operation: {}",
            prepared.plan.max_amount_units_per_operation
        ),
        format!("mint_rent_lamports: {}", prepared.mint_rent),
        format!("config_rent_lamports: {}", prepared.config_rent),
        format!(
            "token_account_rent_lamports: {}",
            prepared.token_account_rent
        ),
        format!("combined_rent_lamports: {}", prepared.required_rent),
        format!("payer_balance_lamports: {}", prepared.payer_balance),
        format!("instruction_count: {}", prepared.plan.instruction_count()),
        format!(
            "local_simulation_signature_count: {}",
            prepared.signature_count
        ),
        "required_signers: payer_config_mint_halt_recovery".to_string(),
        "upgrade_authority_signs_initialization: false".to_string(),
        "mint_authority_model: program_derived_pda".to_string(),
        "freeze_authority: none".to_string(),
        "transaction_atomicity: single_transaction".to_string(),
        "keypair_loading: explicit_phase4_simulation_only".to_string(),
        "signing: local_simulation_only".to_string(),
        "rpc_reads: enabled".to_string(),
        "rpc_simulation: enabled_without_broadcast".to_string(),
        "simulation_status: GREEN".to_string(),
        "transaction_submission: disabled_no_submission_api".to_string(),
        "persistent_account_creation: false".to_string(),
        "program_config_initialization: false".to_string(),
        "rox_mint_execution: false".to_string(),
        "rox_burn_execution: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
    ]
    .join("\n")
}

fn phase4_error(message: &str) -> CliError {
    CliError::UnknownPilotFlag(format!("initialize-test-only-mint live Phase 4 {message}"))
}
