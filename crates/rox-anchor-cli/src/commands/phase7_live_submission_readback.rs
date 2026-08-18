//! BUILD_PLAN4 Phase 7D one-shot ROC-to-ROX submission/readback.
//!
//! This module is deliberately compiled but not reachable from the CLI.
//!
//! It consumes only a Phase 7C prepared transaction, re-simulates that exact
//! signed transaction, verifies simulation left no persistent state, performs
//! exactly one capped Devnet submission, persists the send receipt, then
//! requires strict post-send account readback before writing the verified
//! readback receipt.
//!
//! The send receipt is written before post-send readback. If readback later
//! fails, the already-submitted transaction remains evidenced and must not be
//! resent. The persistent operation PDA also provides the on-chain replay
//! barrier.
//!
//! No real internal ROC is burned or mutated by this module.

#![forbid(unsafe_code)]

use std::{fs::OpenOptions, io::Write, path::Path};

use anchor_client::solana_sdk::signature::Signature;
use anchor_lang::{
    solana_program::{program_option::COption, program_pack::Pack},
    AccountDeserialize,
};
use rox_anchor::{OperationStateCode, RoxAnchorConfig, RoxAnchorOperation};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::{
        phase7_live_capped_sender::{
            PHASE7_AMOUNT_MINOR, PHASE7_IDEMPOTENCY_KEY, PHASE7_MAX_AMOUNT_MINOR,
            PHASE7_MAX_OPERATIONS, PHASE7_NONCE, PHASE7_OPERATION_ID, PHASE7_OPERATOR_APPROVAL,
            PHASE7_PROGRAM_ID, PHASE7_RETRY_CAP, PHASE7_SHADOW_ROC_BURN_INTENT_ID,
        },
        phase7_live_signed_executor::{
            simulate_prepared_phase7_transaction, PreparedPhase7CappedTransaction,
        },
    },
    CliError,
};

const PHASE7_MINT_LABEL: &str = "test-only-rox-private-testnet";

const PHASE7_TOKEN_LABEL: &str = "test-only-rox-token-account-private-testnet";

const REDACTED_SIGNATURE: &str = "<redacted-testnet-signature>";

const REDACTED_SEND_RECEIPT_ID: &str = "<redacted-send-receipt-id>";

const REDACTED_SIGNER_PATH: &str = "<redacted-external-signer-path>";

const REDACTED_RECEIPT_PATH: &str = "<redacted-external-receipt-path>";

const REDACTED_PROGRAM_ACCOUNT: &str = "<redacted-program-account>";

const REDACTED_CONFIG_ACCOUNT: &str = "<redacted-program-config-account>";

const REDACTED_MINT_ACCOUNT: &str = "<redacted-test-only-mint>";

const REDACTED_TOKEN_ACCOUNT: &str = "<redacted-test-only-token-account>";

const REDACTED_RPC_EVIDENCE: &str = "<redacted-read-only-rpc-evidence>";

pub(crate) fn submit_phase7_once_and_readback(
    prepared: &PreparedPhase7CappedTransaction,
    send_receipt_path: &Path,
    readback_receipt_path: &Path,
    operator_approval: &str,
) -> Result<(), CliError> {
    validate_execution_boundary(
        prepared,
        send_receipt_path,
        readback_receipt_path,
        operator_approval,
    )?;

    // Re-simulate the exact signed transaction immediately before the
    // one-shot send boundary.
    simulate_prepared_phase7_transaction(prepared)?;

    // Simulation must remain non-persistent. Also catch any unrelated state
    // drift that occurred between Phase 7C preparation and this send boundary.
    verify_pre_send_state_unchanged(prepared)?;

    let prepared_signature = prepared
        .transaction
        .signatures
        .first()
        .ok_or_else(|| phase7d_error("prepared transaction has no signature"))?;

    if prepared_signature == &Signature::default() {
        return Err(phase7d_error(
            "prepared transaction contains a default signature",
        ));
    }

    // THE ONLY TRANSACTION SUBMISSION CALL IN PHASE 7D.
    let confirmed_signature = prepared
        .rpc
        .send_and_confirm_transaction(&prepared.transaction)
        .map_err(|error| {
            phase7d_error(format!(
                "one-shot Devnet transaction submission failed: {error}",
            ))
        })?;

    if &confirmed_signature != prepared_signature {
        return Err(phase7d_error(
            "confirmed signature does not match the exact prepared transaction",
        ));
    }

    let send_slot = prepared.rpc.get_slot().map_err(|error| {
        phase7d_error(format!(
            "transaction confirmed but post-send slot read failed: {error}",
        ))
    })?;

    let signature_hash = signature_sha256(&confirmed_signature);

    // Persist actual send evidence before mandatory readback. A later
    // readback failure must never be interpreted as permission to resend.
    let send_receipt = build_send_receipt(send_slot, &signature_hash);

    write_new_json(send_receipt_path, &send_receipt)?;

    let (readback_slot, observed_delta) = verify_post_send_readback(prepared)?;

    let readback_receipt = build_readback_receipt(readback_slot, observed_delta, &signature_hash);

    write_new_json(readback_receipt_path, &readback_receipt)?;

    Ok(())
}

fn validate_execution_boundary(
    prepared: &PreparedPhase7CappedTransaction,
    send_receipt_path: &Path,
    readback_receipt_path: &Path,
    operator_approval: &str,
) -> Result<(), CliError> {
    if operator_approval != PHASE7_OPERATOR_APPROVAL {
        return Err(phase7d_error(
            "exact I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND approval is required",
        ));
    }

    if send_receipt_path == readback_receipt_path {
        return Err(phase7d_error(
            "send and readback receipt paths must be distinct",
        ));
    }

    validate_new_receipt_target(send_receipt_path, "send receipt")?;

    validate_new_receipt_target(readback_receipt_path, "readback receipt")?;

    if !prepared.plan.is_exact_phase7_shape() {
        return Err(phase7d_error(
            "prepared transaction no longer matches exact Phase 7A shape",
        ));
    }

    if prepared.plan.program_id.to_string() != PHASE7_PROGRAM_ID {
        return Err(phase7d_error("prepared program ID mismatch"));
    }

    if prepared.workflow_authority != prepared.plan.workflow_authority {
        return Err(phase7d_error("prepared workflow authority mismatch"));
    }

    if prepared.signature_count != 1 || prepared.transaction.signatures.len() != 1 {
        return Err(phase7d_error(
            "Phase 7D requires exactly one transaction signer",
        ));
    }

    if prepared.pre_mint_supply != 0 || prepared.pre_token_amount != 0 {
        return Err(phase7d_error(
            "Phase 7 first capped send requires zero pre-send ROX supply and token balance",
        ));
    }

    if PHASE7_AMOUNT_MINOR != 1
        || PHASE7_MAX_AMOUNT_MINOR != 1
        || PHASE7_MAX_OPERATIONS != 1
        || PHASE7_RETRY_CAP != 1
    {
        return Err(phase7d_error(
            "Phase 7 exact one-unit caps changed unexpectedly",
        ));
    }

    Ok(())
}

fn verify_pre_send_state_unchanged(
    prepared: &PreparedPhase7CappedTransaction,
) -> Result<(), CliError> {
    let accounts = prepared
        .rpc
        .get_multiple_accounts(&[
            prepared.plan.config,
            prepared.plan.test_only_rox_mint,
            prepared.plan.test_only_token_account,
            prepared.plan.operation,
        ])
        .map_err(|error| phase7d_error(format!("pre-send state recheck failed: {error}",)))?;

    if accounts.len() != 4 {
        return Err(phase7d_error(
            "pre-send state recheck returned unexpected account count",
        ));
    }

    let config_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase7d_error("config disappeared before send"))?;

    let mut config_bytes = config_account.data.as_slice();

    let config = RoxAnchorConfig::try_deserialize(&mut config_bytes)
        .map_err(|error| phase7d_error(format!("pre-send config decode failed: {error}",)))?;

    validate_config_binding(prepared, &config)?;

    let mint_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase7d_error("test-only ROX mint disappeared before send"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase7d_error("pre-send mint owner mismatch"));
    }

    let mint = Mint::unpack(&mint_account.data)
        .map_err(|error| phase7d_error(format!("pre-send mint decode failed: {error}",)))?;

    let token_account = accounts[2]
        .as_ref()
        .ok_or_else(|| phase7d_error("test-only token account disappeared before send"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase7d_error("pre-send token-account owner mismatch"));
    }

    let token = SplTokenAccount::unpack(&token_account.data)
        .map_err(|error| phase7d_error(format!("pre-send token decode failed: {error}",)))?;

    if mint.supply != prepared.pre_mint_supply || token.amount != prepared.pre_token_amount {
        return Err(phase7d_error(
            "persistent mint/token state changed after signed simulation or before send",
        ));
    }

    if accounts[3].is_some() {
        return Err(phase7d_error(
            "Phase 7 operation PDA already exists before send; refusing replay",
        ));
    }

    Ok(())
}

fn verify_post_send_readback(
    prepared: &PreparedPhase7CappedTransaction,
) -> Result<(u64, u64), CliError> {
    let accounts = prepared
        .rpc
        .get_multiple_accounts(&[
            prepared.plan.config,
            prepared.plan.test_only_rox_mint,
            prepared.plan.test_only_token_account,
            prepared.plan.operation,
        ])
        .map_err(|error| phase7d_error(format!("mandatory post-send readback failed: {error}",)))?;

    if accounts.len() != 4 {
        return Err(phase7d_error(
            "post-send readback returned unexpected account count",
        ));
    }

    let config_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase7d_error("config missing after confirmed send"))?;

    if config_account.owner != prepared.plan.program_id {
        return Err(phase7d_error("post-send config owner mismatch"));
    }

    let mut config_bytes = config_account.data.as_slice();

    let config = RoxAnchorConfig::try_deserialize(&mut config_bytes)
        .map_err(|error| phase7d_error(format!("post-send config decode failed: {error}",)))?;

    validate_config_binding(prepared, &config)?;

    let mint_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase7d_error("test-only ROX mint missing after confirmed send"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase7d_error("post-send mint owner mismatch"));
    }

    let mint = Mint::unpack(&mint_account.data)
        .map_err(|error| phase7d_error(format!("post-send mint decode failed: {error}",)))?;

    if mint.decimals != 0
        || mint.mint_authority != COption::Some(prepared.plan.mint_authority)
        || mint.freeze_authority != COption::None
    {
        return Err(phase7d_error("post-send mint policy binding mismatch"));
    }

    let token_account = accounts[2]
        .as_ref()
        .ok_or_else(|| phase7d_error("test-only token account missing after confirmed send"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase7d_error("post-send token-account owner mismatch"));
    }

    let token = SplTokenAccount::unpack(&token_account.data).map_err(|error| {
        phase7d_error(format!("post-send token-account decode failed: {error}",))
    })?;

    if token.mint != prepared.plan.test_only_rox_mint || token.owner != prepared.workflow_authority
    {
        return Err(phase7d_error("post-send token-account binding mismatch"));
    }

    let operation_account = accounts[3].as_ref().ok_or_else(|| {
        phase7d_error("confirmed send did not persist the expected Phase 7 operation PDA")
    })?;

    if operation_account.owner != prepared.plan.program_id {
        return Err(phase7d_error("post-send operation owner mismatch"));
    }

    let mut operation_bytes = operation_account.data.as_slice();

    let operation = RoxAnchorOperation::try_deserialize(&mut operation_bytes)
        .map_err(|error| phase7d_error(format!("post-send operation decode failed: {error}",)))?;

    if operation.authority != prepared.workflow_authority
        || operation.operation_id_hash != prepared.plan.operation_id_hash
        || operation.mint != prepared.plan.test_only_rox_mint
        || operation.token_account != prepared.plan.test_only_token_account
        || operation.amount_atoms != PHASE7_AMOUNT_MINOR
        || operation.burn_evidence_hash != prepared.plan.burn_evidence_hash
    {
        return Err(phase7d_error("post-send operation binding mismatch"));
    }

    if !operation.is_roc_to_rox()
        || operation.state_code() != Some(OperationStateCode::Finalized)
        || operation.challenge_open
        || operation.recovery_required
    {
        return Err(phase7d_error(
            "post-send operation is not a clean finalized ROC-to-ROX operation",
        ));
    }

    let mint_delta = mint
        .supply
        .checked_sub(prepared.pre_mint_supply)
        .ok_or_else(|| phase7d_error("post-send mint supply moved backwards"))?;

    let token_delta = token
        .amount
        .checked_sub(prepared.pre_token_amount)
        .ok_or_else(|| phase7d_error("post-send token amount moved backwards"))?;

    if mint_delta != PHASE7_AMOUNT_MINOR
        || token_delta != PHASE7_AMOUNT_MINOR
        || mint_delta != token_delta
    {
        return Err(
            phase7d_error(format!(
                "post-send test-only ROX delta mismatch: mint_delta={mint_delta}, token_delta={token_delta}, expected={PHASE7_AMOUNT_MINOR}",
            )),
        );
    }

    let readback_slot = prepared.rpc.get_slot().map_err(|error| {
        phase7d_error(format!(
            "post-send state verified but readback slot query failed: {error}",
        ))
    })?;

    Ok((readback_slot, token_delta))
}

fn validate_config_binding(
    prepared: &PreparedPhase7CappedTransaction,
    config: &RoxAnchorConfig,
) -> Result<(), CliError> {
    if config.authority != prepared.workflow_authority
        || config.rox_mint != prepared.plan.test_only_rox_mint
        || config.mint_authority != prepared.plan.mint_authority
    {
        return Err(phase7d_error("ROX Anchor config binding mismatch"));
    }

    if !config.test_only_mode
        || config.max_supply_units != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS
        || config.max_amount_units_per_operation
            != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS
    {
        return Err(phase7d_error(
            "ROX Anchor config is not the reviewed private-test-only policy",
        ));
    }

    if config.halted || config.recovery_required {
        return Err(phase7d_error(
            "halt/recovery posture blocks Phase 7 submission or readback acceptance",
        ));
    }

    Ok(())
}

fn build_send_receipt(send_slot: u64, signature_hash: &str) -> Value {
    json!({
        "schema":
            "rox-anchor.actual-roc-to-rox-capped-send.v1",

        "phase":
            "BUILD_PLAN4 Phase 7",

        "receipt_role":
            "actual_roc_to_rox_capped_send_receipt",

        "cluster":
            "devnet",

        "direction":
            "roc_to_rox",

        "program_name":
            "rox_anchor",

        "program_id":
            PHASE7_PROGRAM_ID,

        "send_outcome":
            "sent",

        "operation_id":
            PHASE7_OPERATION_ID,

        "idempotency_key":
            PHASE7_IDEMPOTENCY_KEY,

        "nonce":
            PHASE7_NONCE,

        "shadow_roc_burn_intent_id":
            PHASE7_SHADOW_ROC_BURN_INTENT_ID,

        "shadow_roc_burn_only":
            true,

        "program_account":
            REDACTED_PROGRAM_ACCOUNT,

        "config_account":
            REDACTED_CONFIG_ACCOUNT,

        "test_only_mint":
            REDACTED_MINT_ACCOUNT,

        "test_only_token_account":
            REDACTED_TOKEN_ACCOUNT,

        "test_only_mint_label":
            PHASE7_MINT_LABEL,

        "test_only_token_account_label":
            PHASE7_TOKEN_LABEL,

        "amount_minor":
            PHASE7_AMOUNT_MINOR.to_string(),

        "max_amount_minor":
            PHASE7_MAX_AMOUNT_MINOR.to_string(),

        "max_operations":
            PHASE7_MAX_OPERATIONS.to_string(),

        "retry_cap":
            PHASE7_RETRY_CAP.to_string(),

        "read_only_evidence_status":
            "verified",

        "proof_review_status":
            "accepted",

        "coordinator_decision_status":
            "accepted",

        "relayer_dry_run_status":
            "accepted",

        "simulation_result":
            "passed",

        "operator_approval":
            PHASE7_OPERATOR_APPROVAL,

        "external_signer_used":
            true,

        "signer_path_redacted":
            REDACTED_SIGNER_PATH,

        "receipt_out_redacted":
            REDACTED_RECEIPT_PATH,

        "transaction_submission":
            true,

        "send_authorized":
            true,

        "signature_generated":
            true,

        "transaction_signature":
            REDACTED_SIGNATURE,

        "transaction_signature_sha256":
            signature_hash,

        "send_slot":
            send_slot.to_string(),

        "test_only_rox_delta_minor":
            PHASE7_AMOUNT_MINOR.to_string(),

        "test_only_rox_mint_execution":
            true,

        "readback_required":
            true,

        "readback_verified":
            false,

        "public_mint_available":
            false,

        "public_launch_authorized":
            false,

        "mainnet_authorized":
            false,

        "production_bridge_settlement":
            false,

        "public_rox_mint_burn":
            false,

        "real_roc_burn":
            false,

        "real_roc_mutation":
            false,

        "finality_claim":
            false
    })
}

fn build_readback_receipt(readback_slot: u64, observed_delta: u64, signature_hash: &str) -> Value {
    json!({
        "schema":
            "rox-anchor.actual-roc-to-rox-readback.v1",

        "phase":
            "BUILD_PLAN4 Phase 7",

        "receipt_role":
            "actual_roc_to_rox_readback_receipt",

        "cluster":
            "devnet",

        "direction":
            "roc_to_rox",

        "program_name":
            "rox_anchor",

        "program_id":
            PHASE7_PROGRAM_ID,

        "readback_outcome":
            "verified",

        "operation_id":
            PHASE7_OPERATION_ID,

        "idempotency_key":
            PHASE7_IDEMPOTENCY_KEY,

        "nonce":
            PHASE7_NONCE,

        "transaction_signature":
            REDACTED_SIGNATURE,

        "transaction_signature_sha256":
            signature_hash,

        "send_receipt_id":
            REDACTED_SEND_RECEIPT_ID,

        "program_account":
            REDACTED_PROGRAM_ACCOUNT,

        "config_account":
            REDACTED_CONFIG_ACCOUNT,

        "test_only_mint":
            REDACTED_MINT_ACCOUNT,

        "test_only_token_account":
            REDACTED_TOKEN_ACCOUNT,

        "expected_test_only_rox_delta_minor":
            PHASE7_AMOUNT_MINOR.to_string(),

        "observed_test_only_rox_delta_minor":
            observed_delta.to_string(),

        "rpc_evidence_redacted":
            REDACTED_RPC_EVIDENCE,

        "readback_slot":
            readback_slot.to_string(),

        "operation_state":
            "finalized",

        "read_only_rpc":
            true,

        "transaction_submission":
            false,

        "public_mint_available":
            false,

        "public_launch_authorized":
            false,

        "mainnet_authorized":
            false,

        "production_bridge_settlement":
            false,

        "public_rox_mint_burn":
            false,

        "real_roc_burn":
            false,

        "real_roc_mutation":
            false,

        "finality_claim":
            false
    })
}

fn validate_new_receipt_target(path: &Path, label: &str) -> Result<(), CliError> {
    let display = path.to_string_lossy();

    if !path.is_absolute() && !display.starts_with(".rox-anchor-private-pilot/") {
        return Err(phase7d_error(format!(
            "{label} path must be absolute or inside .rox-anchor-private-pilot",
        )));
    }

    if path.exists() {
        return Err(phase7d_error(format!(
            "{label} already exists; refusing overwrite",
        )));
    }

    let parent = path
        .parent()
        .ok_or_else(|| phase7d_error(format!("{label} has no parent directory",)))?;

    if !parent.is_dir() {
        return Err(phase7d_error(format!(
            "{label} parent directory does not exist",
        )));
    }

    Ok(())
}

fn write_new_json(path: &Path, value: &Value) -> Result<(), CliError> {
    let encoded = serde_json::to_string_pretty(value)
        .map_err(|error| phase7d_error(format!("could not encode receipt JSON: {error}",)))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            phase7d_error(format!(
                "could not create receipt without overwrite: {error}",
            ))
        })?;

    file.write_all(encoded.as_bytes())
        .map_err(|error| phase7d_error(format!("could not write receipt: {error}",)))?;

    file.write_all(b"\n")
        .map_err(|error| phase7d_error(format!("could not terminate receipt: {error}",)))?;

    file.sync_all()
        .map_err(|error| phase7d_error(format!("could not sync receipt: {error}",)))
}

fn signature_sha256(signature: &Signature) -> String {
    let digest = Sha256::digest(signature.to_string().as_bytes());

    let mut output = String::with_capacity(64);

    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }

    output
}

fn phase7d_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase7-one-shot-submission-readback {}",
        message.as_ref(),
    ))
}

// Keep Phase 7D in the normal compile/Clippy graph without creating a CLI or
// runtime invocation path.
const _: fn(&PreparedPhase7CappedTransaction, &Path, &Path, &str) -> Result<(), CliError> =
    submit_phase7_once_and_readback;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase7d_send_receipt_shape_is_exactly_capped_and_non_production() {
        let receipt = build_send_receipt(
            123_456,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );

        assert_eq!(
            receipt["schema"],
            "rox-anchor.actual-roc-to-rox-capped-send.v1",
        );

        assert_eq!(receipt["send_outcome"], "sent",);

        assert_eq!(receipt["amount_minor"], "1",);

        assert_eq!(receipt["max_amount_minor"], "1",);

        assert_eq!(receipt["max_operations"], "1",);

        assert_eq!(receipt["retry_cap"], "1",);

        assert_eq!(receipt["transaction_submission"], true,);

        assert_eq!(receipt["readback_required"], true,);

        assert_eq!(receipt["readback_verified"], false,);

        assert_eq!(receipt["real_roc_burn"], false,);

        assert_eq!(receipt["real_roc_mutation"], false,);

        assert_eq!(receipt["finality_claim"], false,);
    }

    #[test]
    fn phase7d_readback_receipt_requires_exact_observed_delta() {
        let receipt = build_readback_receipt(
            123_457,
            1,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );

        assert_eq!(
            receipt["schema"],
            "rox-anchor.actual-roc-to-rox-readback.v1",
        );

        assert_eq!(receipt["readback_outcome"], "verified",);

        assert_eq!(receipt["expected_test_only_rox_delta_minor"], "1",);

        assert_eq!(receipt["observed_test_only_rox_delta_minor"], "1",);

        assert_eq!(receipt["operation_state"], "finalized",);

        assert_eq!(receipt["read_only_rpc"], true,);

        assert_eq!(receipt["transaction_submission"], false,);
    }

    #[test]
    fn phase7d_wrong_operator_approval_is_not_the_exact_send_phrase() {
        assert_ne!("NOT_APPROVED", PHASE7_OPERATOR_APPROVAL,);

        assert_eq!(
            PHASE7_OPERATOR_APPROVAL,
            "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
        );
    }
}
