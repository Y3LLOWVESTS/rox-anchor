//! RO:WHAT — Submits the BUILD_PLAN4 Phase 4 atomic test-only initialization
//! transaction after the shared live preflight and simulation succeed.
//! RO:WHY — Provides one tightly gated devnet/testnet mutation path rather than
//! letting operator scripts reconstruct mint/config initialization ad hoc.
//! RO:INTERACTS — phase4_live_executor, Solana RPC, ROX Anchor config,
//! classic SPL Token mint, and payer associated token account.
//! RO:INVARIANTS — submission reuses the exact prepared four-instruction
//! transaction, simulation must pass first, confirmed readback must match the
//! separated authorities and exact 1000/10 policy, initial mint supply and ATA
//! amount must both remain zero.
//! RO:SECURITY — this is the only Phase 4 module containing a transaction send
//! API. It is reachable only through explicit --execute-live. It does not mint
//! ROX, burn ROX, mutate internal ROC, perform settlement, or authorize mainnet.
//! RO:TEST — phase4_live_init_submission.rs compiles and inspects the gated path;
//! live invocation requires separate operator authorization outside this patch.

#![forbid(unsafe_code)]

use std::fs;

use anchor_lang::{
    solana_program::{program_option::COption, program_pack::Pack},
    AccountDeserialize,
};
use rox_anchor::RoxAnchorConfig;
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::{
        phase4_live_executor::{
            prepare_phase4_live_transaction, simulate_prepared_transaction,
            PreparedPhase4Transaction,
        },
        test_only_init::Phase4InitConfig,
    },
    CliError,
};

pub(crate) fn run_phase4_live_submission(
    config: &Phase4InitConfig,
    receipt_out: &str,
) -> Result<String, CliError> {
    let prepared = prepare_phase4_live_transaction(config)?;

    simulate_prepared_transaction(&prepared)?;

    let signature = prepared
        .rpc
        .send_and_confirm_transaction(&prepared.transaction)
        .map_err(|error| {
            submission_error(&format!("confirmed transaction submission failed: {error}"))
        })?;

    verify_confirmed_initialization(&prepared)?;

    write_redacted_receipt(config, &prepared, receipt_out, &signature.to_string())?;

    Ok(render_confirmed_report(
        config,
        &prepared,
        receipt_out,
        &signature.to_string(),
    ))
}

fn verify_confirmed_initialization(prepared: &PreparedPhase4Transaction) -> Result<(), CliError> {
    let accounts = prepared
        .rpc
        .get_multiple_accounts(&[
            prepared.plan.config,
            prepared.plan.test_only_mint,
            prepared.plan.test_only_token_account,
        ])
        .map_err(|error| {
            submission_error(&format!("confirmed account readback failed: {error}"))
        })?;

    if accounts.len() != 3 {
        return Err(submission_error(
            "confirmed readback returned an unexpected account count",
        ));
    }

    let config_account = accounts[0]
        .as_ref()
        .ok_or_else(|| submission_error("confirmed program config account is missing"))?;

    if config_account.owner != prepared.plan.program_id {
        return Err(submission_error(
            "confirmed program config owner does not match ROX Anchor",
        ));
    }

    let mut config_data = config_account.data.as_slice();

    let state = RoxAnchorConfig::try_deserialize(&mut config_data).map_err(|error| {
        submission_error(&format!(
            "confirmed program config could not be decoded: {error}"
        ))
    })?;

    if state.authority != prepared.payer
        || state.halt_authority != prepared.halt_authority
        || state.recovery_authority != prepared.recovery_authority
        || state.rox_mint != prepared.plan.test_only_mint
        || state.mint_authority != prepared.plan.mint_authority
        || state.mint_authority_bump != prepared.plan.mint_authority_bump
    {
        return Err(submission_error(
            "confirmed program config authority or mint binding mismatch",
        ));
    }

    if !state.test_only_mode
        || state.max_supply_units != prepared.plan.max_supply_units
        || state.max_amount_units_per_operation != prepared.plan.max_amount_units_per_operation
    {
        return Err(submission_error(
            "confirmed program config private-test-only policy mismatch",
        ));
    }

    if state.halted || state.recovery_required {
        return Err(submission_error(
            "newly initialized config entered an unsafe halt or recovery posture",
        ));
    }

    let mint_account = accounts[1]
        .as_ref()
        .ok_or_else(|| submission_error("confirmed test-only ROX mint account is missing"))?;

    if mint_account.owner != spl_token::id() {
        return Err(submission_error(
            "confirmed test-only ROX mint is not owned by classic SPL Token",
        ));
    }

    let mint = Mint::unpack(&mint_account.data).map_err(|error| {
        submission_error(&format!(
            "confirmed test-only ROX mint could not be decoded: {error}"
        ))
    })?;

    if !mint.is_initialized
        || mint.decimals != prepared.plan.mint_decimals
        || mint.supply != 0
        || mint.mint_authority != COption::Some(prepared.plan.mint_authority)
        || mint.freeze_authority != COption::None
    {
        return Err(submission_error(
            "confirmed test-only ROX mint state mismatch",
        ));
    }

    let token_account = accounts[2]
        .as_ref()
        .ok_or_else(|| submission_error("confirmed payer token account is missing"))?;

    if token_account.owner != spl_token::id() {
        return Err(submission_error(
            "confirmed payer token account is not owned by classic SPL Token",
        ));
    }

    let token_state = SplTokenAccount::unpack(&token_account.data).map_err(|error| {
        submission_error(&format!(
            "confirmed payer token account could not be decoded: {error}"
        ))
    })?;

    if token_state.owner != prepared.payer
        || token_state.mint != prepared.plan.test_only_mint
        || token_state.amount != 0
    {
        return Err(submission_error(
            "confirmed payer token account binding or zero-balance invariant mismatch",
        ));
    }

    Ok(())
}

fn write_redacted_receipt(
    config: &Phase4InitConfig,
    prepared: &PreparedPhase4Transaction,
    receipt_out: &str,
    signature: &str,
) -> Result<(), CliError> {
    let receipt = format!(
        concat!(
            "{{\n",
            "  \"schema\": \"rox-anchor.phase4-live-initialization.v1\",\n",
            "  \"cluster\": \"{}\",\n",
            "  \"program_id\": \"{}\",\n",
            "  \"transaction_signature\": \"{}\",\n",
            "  \"program_config\": \"{}\",\n",
            "  \"test_only_rox_mint\": \"{}\",\n",
            "  \"test_only_token_account\": \"{}\",\n",
            "  \"mint_authority_pda\": \"{}\",\n",
            "  \"mint_decimals\": {},\n",
            "  \"max_supply_units\": {},\n",
            "  \"max_amount_units_per_operation\": {},\n",
            "  \"test_only_mode\": true,\n",
            "  \"initial_mint_supply\": 0,\n",
            "  \"initial_token_account_amount\": 0,\n",
            "  \"simulation_before_submission\": true,\n",
            "  \"transaction_confirmed\": true,\n",
            "  \"confirmed_readback\": true,\n",
            "  \"rox_mint_performed\": false,\n",
            "  \"rox_burn_performed\": false,\n",
            "  \"real_roc_mutation\": false,\n",
            "  \"production_settlement\": false,\n",
            "  \"mainnet\": false\n",
            "}}\n"
        ),
        config.cluster,
        prepared.plan.program_id,
        signature,
        prepared.plan.config,
        prepared.plan.test_only_mint,
        prepared.plan.test_only_token_account,
        prepared.plan.mint_authority,
        prepared.plan.mint_decimals,
        prepared.plan.max_supply_units,
        prepared.plan.max_amount_units_per_operation,
    );

    fs::write(receipt_out, receipt).map_err(|error| {
        submission_error(&format!(
            "could not write redacted Phase 4 receipt: {error}"
        ))
    })
}

fn render_confirmed_report(
    config: &Phase4InitConfig,
    prepared: &PreparedPhase4Transaction,
    receipt_out: &str,
    signature: &str,
) -> String {
    [
        "phase4_live_initialization_executor: confirmed_submission".to_string(),
        "phase: BUILD_PLAN4 Phase 4".to_string(),
        format!("cluster: {}", config.cluster),
        format!("program_id: {}", prepared.plan.program_id),
        format!("transaction_signature: {signature}"),
        format!("program_config: {}", prepared.plan.config),
        format!("test_only_rox_mint: {}", prepared.plan.test_only_mint),
        format!(
            "test_only_token_account: {}",
            prepared.plan.test_only_token_account
        ),
        format!("mint_authority_pda: {}", prepared.plan.mint_authority),
        format!("workflow_payer: {}", prepared.payer),
        format!("halt_authority: {}", prepared.halt_authority),
        format!("recovery_authority: {}", prepared.recovery_authority),
        format!("upgrade_authority: {}", prepared.upgrade_authority),
        "simulation_before_submission: GREEN".to_string(),
        "transaction_submission: CONFIRMED".to_string(),
        "confirmed_readback: GREEN".to_string(),
        "test_only_mode: true".to_string(),
        format!("mint_decimals: {}", prepared.plan.mint_decimals),
        format!("max_supply_units: {}", prepared.plan.max_supply_units),
        format!(
            "max_amount_units_per_operation: {}",
            prepared.plan.max_amount_units_per_operation
        ),
        "initial_mint_supply: 0".to_string(),
        "initial_token_account_amount: 0".to_string(),
        "freeze_authority: none".to_string(),
        "mint_authority_model: program_derived_pda".to_string(),
        "required_signers: payer_config_mint_halt_recovery".to_string(),
        "upgrade_authority_signs_initialization: false".to_string(),
        format!("receipt_out: {receipt_out}"),
        "rox_mint_execution: false".to_string(),
        "rox_burn_execution: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
    ]
    .join("\n")
}

fn submission_error(message: &str) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "initialize-test-only-mint live submission {message}"
    ))
}
