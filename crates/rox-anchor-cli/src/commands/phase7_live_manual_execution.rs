//! BUILD_PLAN4 Phase 7E explicit manual ROC-to-ROX execution gate.
//!
//! This is the first CLI-reachable Phase 7 path that may load the external
//! Devnet workflow keypair, sign, and submit a transaction.
//!
//! It remains fail-closed unless the operator explicitly supplies:
//! - the live-execution flag,
//! - the exact approval phrase,
//! - the exact Phase 7 operation/idempotency/nonce tuple,
//! - exact one-unit amount/operation/retry caps,
//! - a validated and still-fresh Phase 7B authorization receipt,
//! - distinct new send/readback receipt paths.
//!
//! Freshness and authorization are checked before Phase 7C is allowed to load
//! the keypair. Phase 7D then owns the only send call and mandatory readback.
//!
//! If a send receipt exists after an error, the command explicitly tells the
//! operator not to retry. The operation PDA is the on-chain replay barrier.
//!
//! This command never burns or mutates real internal ROC.

#![forbid(unsafe_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::commitment_config::CommitmentConfig,
};
use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, PrivatePilotConfig, SubmissionMode};
use serde_json::Value;

use crate::{
    commands::{
        phase7_live_capped_sender::{
            PHASE7_IDEMPOTENCY_KEY, PHASE7_MAX_AMOUNT_MINOR, PHASE7_MAX_OPERATIONS, PHASE7_NONCE,
            PHASE7_OPERATION_ID, PHASE7_OPERATOR_APPROVAL, PHASE7_RETRY_CAP,
        },
        phase7_live_signed_executor::{
            prepare_phase7_signed_transaction, validate_phase7b_authorization_receipt,
        },
        phase7_live_submission_readback::submit_phase7_once_and_readback,
    },
    CliError,
};

const PHASE7E_RPC_URL: &str = "https://api.devnet.solana.com";

const PHASE7E_AUTH_MAX_AGE_SLOTS: u64 = 100;

#[derive(Clone, Debug, Default)]
struct Phase7EArgs {
    config_path: Option<String>,
    authorization_receipt_path: Option<String>,
    send_receipt_out: Option<String>,
    readback_receipt_out: Option<String>,
    operator_approval: Option<String>,
    operation_id: Option<String>,
    idempotency_key: Option<String>,
    nonce: Option<String>,
    max_operations: Option<u16>,
    max_amount_minor: Option<u64>,
    retry_cap: Option<u8>,
    execute_live_devnet_send: bool,
}

pub(crate) fn run_phase7_live_manual_execution(args: &[String]) -> Result<String, CliError> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        return Ok(help_text());
    }

    let args = parse_args(args)?;

    // This check intentionally occurs before any file read, RPC client,
    // keypair load, signing, or submission.
    if !args.execute_live_devnet_send {
        return Err(phase7e_error("--execute-live-devnet-send is required"));
    }

    require_exact_operator_binding(&args)?;

    let config_path = PathBuf::from(required_arg(args.config_path.as_deref(), "--config")?);

    let authorization_path = PathBuf::from(required_arg(
        args.authorization_receipt_path.as_deref(),
        "--phase7b-authorization-receipt",
    )?);

    let send_receipt_path = PathBuf::from(required_arg(
        args.send_receipt_out.as_deref(),
        "--send-receipt-out",
    )?);

    let readback_receipt_path = PathBuf::from(required_arg(
        args.readback_receipt_out.as_deref(),
        "--readback-receipt-out",
    )?);

    require_existing_input(&config_path, "config")?;

    require_existing_input(&authorization_path, "Phase 7B authorization receipt")?;

    require_new_output(&send_receipt_path, "send receipt")?;

    require_new_output(&readback_receipt_path, "readback receipt")?;

    if absolute_path(&send_receipt_path)? == absolute_path(&readback_receipt_path)? {
        return Err(phase7e_error(
            "send and readback receipt paths must be distinct",
        ));
    }

    // Validate the prior authorization shape before any key loading.
    let authorization_text = fs::read_to_string(&authorization_path)
        .map_err(|_| phase7e_error("could not read Phase 7B authorization receipt"))?;

    let authorization: Value = serde_json::from_str(&authorization_text)
        .map_err(|_| phase7e_error("Phase 7B authorization receipt is not valid JSON"))?;

    validate_phase7b_authorization_receipt(&authorization)?;

    // Validate config before any key loading.
    let config_text = fs::read_to_string(&config_path)
        .map_err(|_| phase7e_error("could not read Phase 7 capped-submit config"))?;

    let config = PrivatePilotConfig::parse_external_config(&config_text).map_err(|error| {
        phase7e_error(format!("Phase 7 capped-submit config rejected: {error}",))
    })?;

    config.validate().map_err(|error| {
        phase7e_error(format!(
            "Phase 7 capped-submit config validation failed: {error}",
        ))
    })?;

    if config.testnet.environment_mode != AnchorEnvironmentMode::TestnetOnly
        || config.testnet.cluster != AnchorCluster::Devnet
        || config.testnet.submission_mode != SubmissionMode::TestnetSubmitCapped
    {
        return Err(phase7e_error(
            "Phase 7E requires testnet_only/devnet/testnet_submit_capped",
        ));
    }

    if config.testnet.rpc_url.as_str() != PHASE7E_RPC_URL {
        return Err(phase7e_error(
            "Phase 7E requires the reviewed official Devnet RPC endpoint",
        ));
    }

    let configured_send_receipt = absolute_path(Path::new(config.receipt_output_path.as_str()))?;

    if configured_send_receipt != absolute_path(&send_receipt_path)? {
        return Err(phase7e_error(
            "CLI send receipt path does not match the externally reviewed config receipt path",
        ));
    }

    // Freshness is checked using live read-only RPC before the keypair can be
    // loaded by Phase 7C.
    let freshness_rpc = RpcClient::new_with_commitment(
        config.testnet.rpc_url.as_str().to_owned(),
        CommitmentConfig::confirmed(),
    );

    let current_slot = freshness_rpc.get_slot().map_err(|error| {
        phase7e_error(format!(
            "could not read current Devnet slot before live execution: {error}",
        ))
    })?;

    let review_slot = require_receipt_u64(&authorization, "phase7_review_slot")?;

    let simulation_slot = require_receipt_u64(&authorization, "live_simulation_context_slot")?;

    require_fresh_slot("phase7_review_slot", current_slot, review_slot)?;

    require_fresh_slot(
        "live_simulation_context_slot",
        current_slot,
        simulation_slot,
    )?;

    // From here forward, key loading/signing becomes possible.
    let prepared = prepare_phase7_signed_transaction(&config_path, &authorization_path)?;

    let result = submit_phase7_once_and_readback(
        &prepared,
        &send_receipt_path,
        &readback_receipt_path,
        PHASE7_OPERATOR_APPROVAL,
    );

    if let Err(error) = result {
        if send_receipt_path.exists() {
            return Err(
                phase7e_error(format!(
                    "SEND_RECEIPT_EXISTS_DO_NOT_RETRY; the transaction may already be confirmed; inspect the send receipt and perform readback/reconciliation only; source_error={error:?}",
                )),
            );
        }

        return Err(error);
    }

    if !send_receipt_path.is_file() || !readback_receipt_path.is_file() {
        return Err(phase7e_error(
            "execution returned success without both required receipt artifacts",
        ));
    }

    Ok([
        "rox-anchor pilot".to_string(),
        "command: pilot phase7-execute-capped-roc-to-rox".to_string(),
        "scope: BUILD_PLAN4 Phase 7E explicit private Devnet execution".to_string(),
        "unsafe_defaults: rejected".to_string(),
        "cluster: devnet".to_string(),
        "direction: roc_to_rox".to_string(),
        format!("operation_id: {PHASE7_OPERATION_ID}"),
        format!("idempotency_key: {PHASE7_IDEMPOTENCY_KEY}"),
        format!("nonce: {PHASE7_NONCE}"),
        "amount_minor: 1".to_string(),
        "max_amount_minor: 1".to_string(),
        "max_operations: 1".to_string(),
        "retry_cap: 1".to_string(),
        format!("authorization_age_limit_slots: {PHASE7E_AUTH_MAX_AGE_SLOTS}"),
        "phase7b_authorization: fresh_verified".to_string(),
        "workflow_keypair_loaded: true".to_string(),
        "exact_transaction_signed: true".to_string(),
        "signed_transaction_resimulated: true".to_string(),
        "transaction_submission: true".to_string(),
        "transaction_count: 1".to_string(),
        "test_only_rox_mint_delta: 1".to_string(),
        "test_only_rox_token_delta: 1".to_string(),
        "operation_pda_persisted: true".to_string(),
        "operation_state: finalized".to_string(),
        "readback_verified: true".to_string(),
        "send_receipt_persisted: true".to_string(),
        "readback_receipt_persisted: true".to_string(),
        "shadow_roc_burn_only: true".to_string(),
        "real_roc_burn: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
        "finality_claim: false".to_string(),
        "phase7_forward_execution: GREEN".to_string(),
        "next_action: VALIDATE_PHASE7_SEND_AND_READBACK_RECEIPTS_THEN_CLOSE_PHASE7".to_string(),
    ]
    .join("\n"))
}

fn require_exact_operator_binding(args: &Phase7EArgs) -> Result<(), CliError> {
    if args.operator_approval.as_deref() != Some(PHASE7_OPERATOR_APPROVAL) {
        return Err(phase7e_error(
            "exact I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND approval is required",
        ));
    }

    if args.operation_id.as_deref() != Some(PHASE7_OPERATION_ID) {
        return Err(phase7e_error("exact Phase 7 operation ID is required"));
    }

    if args.idempotency_key.as_deref() != Some(PHASE7_IDEMPOTENCY_KEY) {
        return Err(phase7e_error("exact Phase 7 idempotency key is required"));
    }

    if args.nonce.as_deref() != Some(PHASE7_NONCE) {
        return Err(phase7e_error("exact Phase 7 nonce is required"));
    }

    if args.max_operations != Some(PHASE7_MAX_OPERATIONS) {
        return Err(phase7e_error("--max-operations must be exactly 1"));
    }

    if args.max_amount_minor != Some(PHASE7_MAX_AMOUNT_MINOR) {
        return Err(phase7e_error("--max-amount-minor must be exactly 1"));
    }

    if args.retry_cap != Some(PHASE7_RETRY_CAP) {
        return Err(phase7e_error("--retry-cap must be exactly 1"));
    }

    Ok(())
}

fn require_fresh_slot(label: &str, current_slot: u64, evidence_slot: u64) -> Result<(), CliError> {
    let delta = current_slot.abs_diff(evidence_slot);

    if delta > PHASE7E_AUTH_MAX_AGE_SLOTS {
        return Err(
            phase7e_error(format!(
                "{label} is stale for live execution: slot_delta={delta}, limit={PHASE7E_AUTH_MAX_AGE_SLOTS}",
            )),
        );
    }

    Ok(())
}

fn require_receipt_u64(receipt: &Value, field: &str) -> Result<u64, CliError> {
    receipt.get(field).and_then(Value::as_u64).ok_or_else(|| {
        phase7e_error(format!(
            "Phase 7B authorization receipt missing integer `{field}`",
        ))
    })
}

fn require_existing_input(path: &Path, label: &str) -> Result<(), CliError> {
    if !path.is_file() {
        return Err(phase7e_error(format!("{label} does not exist",)));
    }

    Ok(())
}

fn require_new_output(path: &Path, label: &str) -> Result<(), CliError> {
    if path.exists() {
        return Err(phase7e_error(format!(
            "{label} already exists; refusing possible replay or overwrite",
        )));
    }

    let parent = path
        .parent()
        .ok_or_else(|| phase7e_error(format!("{label} has no parent directory",)))?;

    if !parent.is_dir() {
        return Err(phase7e_error(format!(
            "{label} parent directory does not exist",
        )));
    }

    Ok(())
}

fn absolute_path(path: &Path) -> Result<PathBuf, CliError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    std::env::current_dir()
        .map(|cwd| cwd.join(path))
        .map_err(|error| phase7e_error(format!("could not resolve local path: {error}",)))
}

fn required_arg<'a>(value: Option<&'a str>, flag: &str) -> Result<&'a str, CliError> {
    value.ok_or_else(|| phase7e_error(format!("{flag} is required",)))
}

fn parse_args(args: &[String]) -> Result<Phase7EArgs, CliError> {
    let mut parsed = Phase7EArgs::default();

    let mut index = 0_usize;

    while index < args.len() {
        match args[index].as_str() {
            "--config" => {
                parsed.config_path = Some(next_value(args, index, "--config")?);
                index += 2;
            }

            "--phase7b-authorization-receipt" => {
                parsed.authorization_receipt_path =
                    Some(next_value(args, index, "--phase7b-authorization-receipt")?);
                index += 2;
            }

            "--send-receipt-out" => {
                parsed.send_receipt_out = Some(next_value(args, index, "--send-receipt-out")?);
                index += 2;
            }

            "--readback-receipt-out" => {
                parsed.readback_receipt_out =
                    Some(next_value(args, index, "--readback-receipt-out")?);
                index += 2;
            }

            "--operator-approval" => {
                parsed.operator_approval = Some(next_value(args, index, "--operator-approval")?);
                index += 2;
            }

            "--operation-id" => {
                parsed.operation_id = Some(next_value(args, index, "--operation-id")?);
                index += 2;
            }

            "--idempotency-key" => {
                parsed.idempotency_key = Some(next_value(args, index, "--idempotency-key")?);
                index += 2;
            }

            "--nonce" => {
                parsed.nonce = Some(next_value(args, index, "--nonce")?);
                index += 2;
            }

            "--max-operations" => {
                parsed.max_operations = Some(
                    next_value(args, index, "--max-operations")?
                        .parse::<u16>()
                        .map_err(|_| phase7e_error("--max-operations must be u16"))?,
                );
                index += 2;
            }

            "--max-amount-minor" => {
                parsed.max_amount_minor = Some(
                    next_value(args, index, "--max-amount-minor")?
                        .parse::<u64>()
                        .map_err(|_| phase7e_error("--max-amount-minor must be u64"))?,
                );
                index += 2;
            }

            "--retry-cap" => {
                parsed.retry_cap = Some(
                    next_value(args, index, "--retry-cap")?
                        .parse::<u8>()
                        .map_err(|_| phase7e_error("--retry-cap must be u8"))?,
                );
                index += 2;
            }

            "--execute-live-devnet-send" => {
                parsed.execute_live_devnet_send = true;
                index += 1;
            }

            other => {
                return Err(phase7e_error(format!("unknown argument `{other}`",)));
            }
        }
    }

    Ok(parsed)
}

fn next_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| phase7e_error(format!("{flag} requires a value",)))
}

fn help_text() -> String {
    [
        "BUILD_PLAN4 Phase 7E LIVE DEVNET ROC-to-ROX execution",
        "",
        "THIS COMMAND SUBMITS ONE REAL DEVNET TRANSACTION.",
        "It mints exactly one unit of the private test-only ROX mint.",
        "It does not burn or mutate real internal ROC.",
        "",
        "required:",
        "  --config <reviewed-capped-submit-config>",
        "  --phase7b-authorization-receipt <fresh-phase7b-receipt>",
        "  --send-receipt-out <new-path>",
        "  --readback-receipt-out <new-path>",
        "  --operator-approval I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
        "  --operation-id actual-roc-to-rox-op-0001",
        "  --idempotency-key actual-roc-to-rox-idem-0001",
        "  --nonce actual-roc-to-rox-nonce-0001",
        "  --max-operations 1",
        "  --max-amount-minor 1",
        "  --retry-cap 1",
        "  --execute-live-devnet-send",
        "",
        "The Phase 7B authorization and live simulation slots must be within",
        "100 slots of the current Devnet slot.",
        "",
        "If a send receipt exists after any error, DO NOT RETRY the send.",
    ]
    .join("\n")
}

fn phase7e_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase7-execute-capped-roc-to-rox {}",
        message.as_ref(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase7e_slot_freshness_accepts_boundary_and_rejects_over_boundary() {
        require_fresh_slot("test", 1_000, 900).expect("100-slot delta should pass");

        assert!(require_fresh_slot("test", 1_000, 899,).is_err());

        require_fresh_slot("test", 900, 1_000).expect("bounded provider-forward skew should pass");
    }

    #[test]
    fn phase7e_exact_caps_remain_one() {
        assert_eq!(PHASE7_MAX_OPERATIONS, 1,);

        assert_eq!(PHASE7_MAX_AMOUNT_MINOR, 1,);

        assert_eq!(PHASE7_RETRY_CAP, 1,);
    }

    #[test]
    fn phase7e_exact_identity_tuple_is_locked() {
        assert_eq!(PHASE7_OPERATION_ID, "actual-roc-to-rox-op-0001",);

        assert_eq!(PHASE7_IDEMPOTENCY_KEY, "actual-roc-to-rox-idem-0001",);

        assert_eq!(PHASE7_NONCE, "actual-roc-to-rox-nonce-0001",);
    }
}
