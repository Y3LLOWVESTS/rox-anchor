//! BUILD_PLAN4 Phase 7C signed ROC-to-ROX executor preparation.
//!
//! This module is intentionally compiled but not routed from the CLI.
//!
//! It reuses the exact Phase 7A transaction candidate and requires the
//! successful Phase 7B simulation/authorization receipt before it can:
//!
//! 1. validate the explicit capped-submit private-pilot config,
//! 2. re-read the actual deployed Devnet program/config/mint/token state,
//! 3. prove the Phase 7 operation PDA is still absent,
//! 4. load the externally configured workflow-authority keypair,
//! 5. require that keypair to match the reviewed workflow authority,
//! 6. sign the exact two-instruction Phase 7 candidate,
//! 7. simulate that signed candidate.
//!
//! There is deliberately no transaction-send API in this module.
//! Phase 7D will build the separately gated submit/readback surface.
//!
//! Merely compiling or testing this module does not load a keypair, contact
//! RPC, sign a transaction, mint ROX, or mutate any state.

#![forbid(unsafe_code)]

use std::{fs, path::Path};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Signature, Signer},
        transaction::Transaction,
    },
};
use anchor_lang::{
    solana_program::{program_option::COption, program_pack::Pack},
    AccountDeserialize,
};
use rox_anchor::{RoxAnchorConfig, RoxAnchorOperation};
use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, PrivatePilotConfig, SubmissionMode};
use serde_json::Value;
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::phase7_live_capped_sender::{
        build_phase7_capped_roc_to_rox_plan, Phase7CappedRocToRoxPlan, PHASE7_AMOUNT_MINOR,
        PHASE7_IDEMPOTENCY_KEY, PHASE7_MAX_AMOUNT_MINOR, PHASE7_MAX_OPERATIONS, PHASE7_NONCE,
        PHASE7_OPERATION_ID, PHASE7_PROGRAM_ID, PHASE7_RETRY_CAP,
    },
    CliError,
};

const PHASE7B_AUTH_SCHEMA: &str = "rox-anchor.phase7-simulation-authorization.v1";

const PHASE7B_AUTH_PHASE: &str = "BUILD_PLAN4 Phase 7B";

const PHASE7_EXPECTED_RPC_URL: &str = "https://api.devnet.solana.com";

const PHASE7_EXPECTED_OPERATOR_LABEL: &str = "private-phase7-roc-to-rox-operator";

const PHASE7_EXPECTED_ASSET_LABEL: &str = "test-only-rox-private-devnet";

const PHASE4_INITIALIZATION_SIGNATURE: &str =
    "5J8cjGr3idqUff4Mh5FeMSfEDoXpn5QAh6bxyWQrmpT1q4PeFCouRbThH5JN2dtLaHC1kC4QPcMApownmXeimyK5";

pub(crate) struct PreparedPhase7CappedTransaction {
    pub(crate) rpc: RpcClient,
    pub(crate) transaction: Transaction,
    pub(crate) plan: Phase7CappedRocToRoxPlan,
    pub(crate) workflow_authority: Pubkey,
    pub(crate) pre_mint_supply: u64,
    pub(crate) pre_token_amount: u64,
    pub(crate) operation_rent_lamports: u64,
    pub(crate) payer_balance_lamports: u64,
    pub(crate) signature_count: usize,
}

pub(crate) fn prepare_phase7_signed_transaction(
    config_path: &Path,
    phase7b_authorization_receipt_path: &Path,
) -> Result<PreparedPhase7CappedTransaction, CliError> {
    let config_text = fs::read_to_string(config_path)
        .map_err(|_| phase7c_error("could not read capped-submit config"))?;

    let config = PrivatePilotConfig::parse_external_config(&config_text)
        .map_err(|error| phase7c_error(format!("capped-submit config rejected: {error}",)))?;

    validate_phase7c_config(&config)?;

    let authorization_text = fs::read_to_string(phase7b_authorization_receipt_path)
        .map_err(|_| phase7c_error("could not read Phase 7B authorization receipt"))?;

    let authorization: Value = serde_json::from_str(&authorization_text)
        .map_err(|_| phase7c_error("Phase 7B authorization receipt is not valid JSON"))?;

    validate_phase7b_authorization_receipt(&authorization)?;

    let plan = build_phase7_capped_roc_to_rox_plan()?;

    if !plan.is_exact_phase7_shape() {
        return Err(phase7c_error(
            "Phase 7A candidate failed exact-shape revalidation",
        ));
    }

    if plan.instructions.len() != 2 {
        return Err(phase7c_error(
            "Phase 7 signed candidate must contain exactly two instructions",
        ));
    }

    let rpc = RpcClient::new_with_commitment(
        config.testnet.rpc_url.as_str().to_owned(),
        CommitmentConfig::confirmed(),
    );

    let state = read_and_validate_live_preflight(&rpc, &plan)?;

    let workflow = read_keypair_file(config.testnet.payer_keypair_path.as_str())
        .map_err(|_| phase7c_error("could not load configured workflow-authority keypair"))?;

    if workflow.pubkey() != plan.workflow_authority {
        return Err(phase7c_error(
            "loaded keypair does not match reviewed workflow authority",
        ));
    }

    let blockhash = rpc.get_latest_blockhash().map_err(|error| {
        phase7c_error(format!("could not fetch recent Devnet blockhash: {error}",))
    })?;

    let signers: [&dyn Signer; 1] = [&workflow];

    let transaction = Transaction::new_signed_with_payer(
        &plan.instructions,
        Some(&workflow.pubkey()),
        &signers,
        blockhash,
    );

    if transaction.signatures.len() != 1 {
        return Err(phase7c_error(
            "Phase 7 signed transaction must contain exactly one signature",
        ));
    }

    if transaction.signatures[0] == Signature::default() {
        return Err(phase7c_error(
            "Phase 7 signed transaction contains a default signature",
        ));
    }

    Ok(PreparedPhase7CappedTransaction {
        rpc,
        transaction,
        plan,
        workflow_authority: workflow.pubkey(),
        pre_mint_supply: state.mint_supply,
        pre_token_amount: state.token_amount,
        operation_rent_lamports: state.operation_rent_lamports,
        payer_balance_lamports: state.payer_balance_lamports,
        signature_count: 1,
    })
}

pub(crate) fn simulate_prepared_phase7_transaction(
    prepared: &PreparedPhase7CappedTransaction,
) -> Result<(), CliError> {
    let simulation = prepared
        .rpc
        .simulate_transaction(&prepared.transaction)
        .map_err(|error| {
            phase7c_error(format!("signed Phase 7 simulation request failed: {error}",))
        })?;

    if let Some(error) = simulation.value.err.as_ref() {
        return Err(phase7c_error(format!(
            "signed Phase 7 simulation rejected: {error:?}",
        )));
    }

    Ok(())
}

struct LivePreflightState {
    mint_supply: u64,
    token_amount: u64,
    operation_rent_lamports: u64,
    payer_balance_lamports: u64,
}

fn read_and_validate_live_preflight(
    rpc: &RpcClient,
    plan: &Phase7CappedRocToRoxPlan,
) -> Result<LivePreflightState, CliError> {
    let accounts = rpc
        .get_multiple_accounts(&[
            plan.program_id,
            plan.config,
            plan.test_only_rox_mint,
            plan.test_only_token_account,
            plan.operation,
        ])
        .map_err(|error| {
            phase7c_error(format!(
                "Phase 7 live preflight account read failed: {error}",
            ))
        })?;

    if accounts.len() != 5 {
        return Err(phase7c_error(
            "Phase 7 live preflight returned unexpected account count",
        ));
    }

    let program = accounts[0]
        .as_ref()
        .ok_or_else(|| phase7c_error("deployed ROX Anchor program account is missing"))?;

    if !program.executable {
        return Err(phase7c_error(
            "deployed ROX Anchor program account is not executable",
        ));
    }

    let config_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase7c_error("ROX Anchor config account is missing"))?;

    if config_account.owner != plan.program_id {
        return Err(phase7c_error("ROX Anchor config owner mismatch"));
    }

    let mut config_bytes = config_account.data.as_slice();

    let config = RoxAnchorConfig::try_deserialize(&mut config_bytes)
        .map_err(|error| phase7c_error(format!("ROX Anchor config decode failed: {error}",)))?;

    if config.authority != plan.workflow_authority {
        return Err(phase7c_error("workflow authority binding mismatch"));
    }

    if config.rox_mint != plan.test_only_rox_mint {
        return Err(phase7c_error("ROX mint binding mismatch"));
    }

    if config.mint_authority != plan.mint_authority {
        return Err(phase7c_error("ROX mint-authority PDA binding mismatch"));
    }

    if !config.test_only_mode
        || config.max_supply_units != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS
        || config.max_amount_units_per_operation
            != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS
    {
        return Err(phase7c_error(
            "deployed config is not the exact private-test-only policy",
        ));
    }

    if config.halted || config.recovery_required {
        return Err(phase7c_error(
            "halt/recovery posture blocks Phase 7 signed preparation",
        ));
    }

    let mint_account = accounts[2]
        .as_ref()
        .ok_or_else(|| phase7c_error("test-only ROX mint is missing"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase7c_error("test-only ROX mint program owner mismatch"));
    }

    let mint = Mint::unpack(&mint_account.data)
        .map_err(|error| phase7c_error(format!("test-only ROX mint decode failed: {error}",)))?;

    if mint.decimals != 0 {
        return Err(phase7c_error(
            "test-only ROX mint decimals must remain zero",
        ));
    }

    if mint.supply != 0 {
        return Err(phase7c_error(
            "Phase 7 first capped send requires pre-send mint supply zero",
        ));
    }

    if mint.mint_authority != COption::Some(plan.mint_authority) {
        return Err(phase7c_error("test-only ROX mint authority mismatch"));
    }

    if mint.freeze_authority != COption::None {
        return Err(phase7c_error(
            "test-only ROX mint must not have a freeze authority",
        ));
    }

    let token_account = accounts[3]
        .as_ref()
        .ok_or_else(|| phase7c_error("test-only ROX token account is missing"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase7c_error(
            "test-only token account program owner mismatch",
        ));
    }

    let token = SplTokenAccount::unpack(&token_account.data).map_err(|error| {
        phase7c_error(format!("test-only token account decode failed: {error}",))
    })?;

    if token.mint != plan.test_only_rox_mint {
        return Err(phase7c_error("test-only token account mint mismatch"));
    }

    if token.owner != plan.workflow_authority {
        return Err(phase7c_error("test-only token account owner mismatch"));
    }

    if token.amount != 0 {
        return Err(phase7c_error(
            "Phase 7 first capped send requires pre-send token amount zero",
        ));
    }

    if accounts[4].is_some() {
        return Err(
            phase7c_error(
                "Phase 7 operation PDA already exists; refusing replay or duplicate execution preparation",
            ),
        );
    }

    let operation_rent_lamports = rpc
        .get_minimum_balance_for_rent_exemption(RoxAnchorOperation::SPACE)
        .map_err(|error| {
            phase7c_error(format!(
                "could not fetch operation rent requirement: {error}",
            ))
        })?;

    let payer_balance_lamports = rpc.get_balance(&plan.workflow_authority).map_err(|error| {
        phase7c_error(format!(
            "could not read workflow-authority balance: {error}",
        ))
    })?;

    if payer_balance_lamports <= operation_rent_lamports {
        return Err(phase7c_error(
            "workflow authority balance is not above the operation rent requirement",
        ));
    }

    Ok(LivePreflightState {
        mint_supply: mint.supply,
        token_amount: token.amount,
        operation_rent_lamports,
        payer_balance_lamports,
    })
}

fn validate_phase7c_config(config: &PrivatePilotConfig) -> Result<(), CliError> {
    config.validate().map_err(|error| {
        phase7c_error(format!("private-pilot config validation failed: {error}",))
    })?;

    if config.testnet.environment_mode != AnchorEnvironmentMode::TestnetOnly {
        return Err(phase7c_error("environment_mode must be testnet_only"));
    }

    if config.testnet.cluster != AnchorCluster::Devnet {
        return Err(phase7c_error("Phase 7 current deployment requires devnet"));
    }

    if config.testnet.submission_mode != SubmissionMode::TestnetSubmitCapped {
        return Err(phase7c_error(
            "submission_mode must be testnet_submit_capped",
        ));
    }

    if config.testnet.rpc_url.as_str() != PHASE7_EXPECTED_RPC_URL {
        return Err(phase7c_error(
            "Phase 7C requires the reviewed official Devnet RPC endpoint",
        ));
    }

    if config.operator_label != PHASE7_EXPECTED_OPERATOR_LABEL {
        return Err(phase7c_error("Phase 7 operator label mismatch"));
    }

    if config.asset_label != PHASE7_EXPECTED_ASSET_LABEL {
        return Err(phase7c_error("Phase 7 asset label mismatch"));
    }

    let observed_signature = config.observed_signature.as_ref().ok_or_else(|| {
        phase7c_error("Phase 7 config requires observed initialization signature")
    })?;

    if observed_signature.as_str() != PHASE4_INITIALIZATION_SIGNATURE {
        return Err(phase7c_error(
            "Phase 7 observed initialization signature mismatch",
        ));
    }

    Ok(())
}

pub(crate) fn validate_phase7b_authorization_receipt(receipt: &Value) -> Result<(), CliError> {
    require_string(receipt, "schema", PHASE7B_AUTH_SCHEMA)?;

    require_string(receipt, "phase", PHASE7B_AUTH_PHASE)?;

    require_string(
        receipt,
        "receipt_role",
        "simulation_and_sender_authorization_evidence",
    )?;

    require_string(receipt, "cluster", "devnet")?;

    require_string(receipt, "direction", "roc_to_rox")?;

    require_string(receipt, "program_id", PHASE7_PROGRAM_ID)?;

    require_string(receipt, "operation_id", PHASE7_OPERATION_ID)?;

    require_string(receipt, "idempotency_key", PHASE7_IDEMPOTENCY_KEY)?;

    require_string(receipt, "nonce", PHASE7_NONCE)?;

    require_string(receipt, "amount_minor", &PHASE7_AMOUNT_MINOR.to_string())?;

    require_string(
        receipt,
        "max_amount_minor",
        &PHASE7_MAX_AMOUNT_MINOR.to_string(),
    )?;

    require_string(
        receipt,
        "max_operations",
        &PHASE7_MAX_OPERATIONS.to_string(),
    )?;

    require_string(receipt, "retry_cap", &PHASE7_RETRY_CAP.to_string())?;

    require_u64(receipt, "instruction_count", 2)?;

    require_string(receipt, "phase5_read_only_evidence", "fresh_verified")?;

    require_string(
        receipt,
        "phase6_forward_simulation_evidence",
        "verified_non_promotable",
    )?;

    require_string(receipt, "phase7_local_proof_review", "accepted")?;

    require_string(receipt, "phase7_coordinator_decision", "accepted")?;

    require_string(receipt, "phase7_relayer_dry_run", "accepted")?;

    require_string(receipt, "phase7_live_devnet_simulation", "passed")?;

    let sequence = receipt
        .get("instruction_sequence")
        .and_then(Value::as_array)
        .ok_or_else(|| phase7c_error("authorization receipt missing instruction_sequence"))?;

    if sequence.len() != 2
        || sequence[0].as_str() != Some("observe_burn")
        || sequence[1].as_str() != Some("finalize_roc_to_rox_mint")
    {
        return Err(phase7c_error(
            "authorization receipt instruction sequence mismatch",
        ));
    }

    for field in [
        "persistent_operation_after_simulation",
        "persistent_config_change_after_simulation",
        "persistent_mint_change_after_simulation",
        "persistent_token_account_change_after_simulation",
    ] {
        require_bool(receipt, field, false)?;
    }

    require_bool(receipt, "sender_authorized_by_existing_model", true)?;

    require_bool(receipt, "live_submission_permitted_by_existing_model", true)?;

    require_bool(receipt, "approval_translation_explicit", true)?;

    for field in [
        "live_submission_attempted",
        "network_submitted",
        "wallet_key_loading",
        "signing",
        "send_receipt",
        "transaction_submission",
        "signature_generated",
        "rox_mint_persisted",
        "real_roc_burn",
        "real_roc_mutation",
        "production_settlement",
        "public_launch_authorized",
        "mainnet_authorized",
        "finality_claim",
    ] {
        require_bool(receipt, field, false)?;
    }

    Ok(())
}

fn require_string(value: &Value, field: &str, expected: &str) -> Result<(), CliError> {
    let actual = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| phase7c_error(format!("authorization receipt missing string `{field}`",)))?;

    if actual != expected {
        return Err(phase7c_error(format!(
            "authorization receipt `{field}` mismatch",
        )));
    }

    Ok(())
}

fn require_u64(value: &Value, field: &str, expected: u64) -> Result<(), CliError> {
    let actual = value.get(field).and_then(Value::as_u64).ok_or_else(|| {
        phase7c_error(format!("authorization receipt missing integer `{field}`",))
    })?;

    if actual != expected {
        return Err(phase7c_error(format!(
            "authorization receipt `{field}` mismatch",
        )));
    }

    Ok(())
}

fn require_bool(value: &Value, field: &str, expected: bool) -> Result<(), CliError> {
    let actual = value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| phase7c_error(format!("authorization receipt missing bool `{field}`",)))?;

    if actual != expected {
        return Err(phase7c_error(format!(
            "authorization receipt `{field}` mismatch",
        )));
    }

    Ok(())
}

fn phase7c_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!("phase7-live-signed-executor {}", message.as_ref(),))
}

// Phase 7D consumes this prepared-state evidence after its own one-shot
// submission/readback boundary exists. Keep every field type-checked and
// intentionally referenced now without creating a runtime execution path.
fn phase7c_compile_prepared_field_contract(prepared: &PreparedPhase7CappedTransaction) {
    let _ = (
        &prepared.rpc,
        &prepared.transaction,
        &prepared.plan,
        &prepared.workflow_authority,
        &prepared.pre_mint_supply,
        &prepared.pre_token_amount,
        &prepared.operation_rent_lamports,
        &prepared.payer_balance_lamports,
        &prepared.signature_count,
    );
}

const _: fn(&PreparedPhase7CappedTransaction) = phase7c_compile_prepared_field_contract;

// Keep the deliberately unrouted Phase 7C executor in the normal compile
// graph without creating a runtime call path. Phase 7D will consume these
// entrypoints after its own submission/readback gates are built.
const _: fn(&Path, &Path) -> Result<PreparedPhase7CappedTransaction, CliError> =
    prepare_phase7_signed_transaction;

const _: fn(&PreparedPhase7CappedTransaction) -> Result<(), CliError> =
    simulate_prepared_phase7_transaction;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid_authorization_receipt() -> Value {
        json!({
            "schema":
                "rox-anchor.phase7-simulation-authorization.v1",

            "phase":
                "BUILD_PLAN4 Phase 7B",

            "receipt_role":
                "simulation_and_sender_authorization_evidence",

            "cluster":
                "devnet",

            "direction":
                "roc_to_rox",

            "program_id":
                PHASE7_PROGRAM_ID,

            "operation_id":
                PHASE7_OPERATION_ID,

            "idempotency_key":
                PHASE7_IDEMPOTENCY_KEY,

            "nonce":
                PHASE7_NONCE,

            "amount_minor":
                "1",

            "max_amount_minor":
                "1",

            "max_operations":
                "1",

            "retry_cap":
                "1",

            "instruction_count":
                2,

            "instruction_sequence": [
                "observe_burn",
                "finalize_roc_to_rox_mint"
            ],

            "phase5_read_only_evidence":
                "fresh_verified",

            "phase6_forward_simulation_evidence":
                "verified_non_promotable",

            "phase7_local_proof_review":
                "accepted",

            "phase7_coordinator_decision":
                "accepted",

            "phase7_relayer_dry_run":
                "accepted",

            "phase7_live_devnet_simulation":
                "passed",

            "persistent_operation_after_simulation":
                false,

            "persistent_config_change_after_simulation":
                false,

            "persistent_mint_change_after_simulation":
                false,

            "persistent_token_account_change_after_simulation":
                false,

            "sender_authorized_by_existing_model":
                true,

            "live_submission_permitted_by_existing_model":
                true,

            "approval_translation_explicit":
                true,

            "live_submission_attempted":
                false,

            "network_submitted":
                false,

            "wallet_key_loading":
                false,

            "signing":
                false,

            "send_receipt":
                false,

            "transaction_submission":
                false,

            "signature_generated":
                false,

            "rox_mint_persisted":
                false,

            "real_roc_burn":
                false,

            "real_roc_mutation":
                false,

            "production_settlement":
                false,

            "public_launch_authorized":
                false,

            "mainnet_authorized":
                false,

            "finality_claim":
                false
        })
    }

    #[test]
    fn phase7c_accepts_exact_phase7b_authorization_shape() {
        let receipt = valid_authorization_receipt();

        validate_phase7b_authorization_receipt(&receipt)
            .expect("exact Phase 7B authorization should be accepted");
    }

    #[test]
    fn phase7c_rejects_authorization_that_already_claims_submission() {
        let mut receipt = valid_authorization_receipt();

        receipt["network_submitted"] = Value::Bool(true);

        assert!(validate_phase7b_authorization_receipt(&receipt,).is_err());
    }

    #[test]
    fn phase7c_rejects_authorization_without_sender_permission() {
        let mut receipt = valid_authorization_receipt();

        receipt["live_submission_permitted_by_existing_model"] = Value::Bool(false);

        assert!(validate_phase7b_authorization_receipt(&receipt,).is_err());
    }

    #[test]
    fn phase7c_reuses_exact_phase7a_candidate_shape() {
        let plan =
            build_phase7_capped_roc_to_rox_plan().expect("reviewed static candidate should build");

        assert!(plan.is_exact_phase7_shape());

        assert_eq!(plan.instructions.len(), 2,);

        assert_eq!(plan.required_signers.len(), 1,);

        assert!(plan.required_signers.contains(&plan.workflow_authority,));
    }
}
