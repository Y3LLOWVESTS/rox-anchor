//! RO:WHAT — Collects one live BUILD_PLAN4 Phase 5 read-only RPC observation
//! against the initialized private devnet accounts.
//! RO:WHY — Binds actual deployed state into the existing rpc-proof model
//! without inventing RPC finality, quorum, or a second acceptance ruleset.
//! RO:INTERACTS — Phase 4 initialization receipt, Solana RpcClient,
//! RoxAnchorConfig, classic SPL Token state, and rox-anchor-rpc-proof.
//! RO:INVARIANTS — exact devnet/FIUY/config/mint/ATA/PDA bindings; exact
//! 1000/10 private-test policy; zero initial ROX supply; zero ATA amount;
//! clear halt/recovery posture; one RPC source remains under the two-source
//! quorum policy.
//! RO:SECURITY — read-only RPC only. No keypair loading, signing, simulation,
//! transaction submission, mint, burn, ROC mutation, settlement, or mainnet.
//! RO:TEST — phase5_live_read_only_source.rs plus rpc-proof quorum tests.

#![forbid(unsafe_code)]

use std::{
    fs,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature},
};
use anchor_lang::{
    solana_program::{program_option::COption, program_pack::Pack},
    AccountDeserialize,
};
use rox_anchor::RoxAnchorConfig;
use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    review_rpc_observations, ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation,
    RpcProofAuditRecord, RpcProofConfig, RpcQuorumDecision,
};
use serde_json::{json, Value};
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::CliError;

pub(super) const PHASE5_DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

pub(super) const PHASE5_PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";

const PHASE5_CONFIG_ACCOUNT: &str = "4RBTypWtrn7mwV47MJkAHtEBMYnvNhd5wdSMAUsxwFeo";

pub(super) const PHASE5_TEST_ONLY_MINT: &str = "HfHRJLswuRN3eVsiWnYi7REssDEsxxA8ewU8emhC3XA4";

pub(super) const PHASE5_TEST_ONLY_TOKEN_ACCOUNT: &str =
    "A3sBYMUf2N7rpkqiCnE7fKZBdnGR5goH3hFmHJvgvqsJ";

const PHASE5_MINT_AUTHORITY_PDA: &str = "C5jTCy4EBY5fKuRMzLv7Lau5Re1SmMXukRXosndk9hJE";

const PHASE5_WORKFLOW_AUTHORITY: &str = "6YYJ43KRJF6pB3jUtRQpvhVHZQHaURTSxJdLpipHU3gs";

const PHASE5_HALT_AUTHORITY: &str = "3aAvoLEAsCCte4gow6rheJQ3F4zeoCuMvERqyFBobGgz";

const PHASE5_RECOVERY_AUTHORITY: &str = "74upNee16zSKS2hSuovDaioWVsadFf8Za4CCRwJW5fqe";

const PHASE5_UPGRADE_AUTHORITY: &str = "DLQJ1icSQKu5CGsi7FqJgF9ohsiYuYuRkn23EggRDTdJ";

pub(super) const PHASE5_PROGRAM_OWNER: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

pub(super) const PHASE5_OPERATION_ID: &str = "op-phase4-initialization-0001";

pub(super) const PHASE5_REQUIRED_OBSERVATIONS: u16 = 2;

pub(super) const PHASE5_STALE_AFTER_SLOTS: u64 = 100;

const PHASE5_MAX_COLLECTION_WINDOW_SLOTS: u64 = 100;

const PHASE5_READ_ONLY_RPC_MAX_ATTEMPTS: u8 = 4;

const PHASE5_UNIBLOCK_SUCCESS_PACING_MS: u64 = 5_000;

const PHASE5_READ_ONLY_RPC_BASE_BACKOFF_MS: u64 = 5_000;

const PHASE5_READ_ONLY_RPC_RATE_LIMIT_BACKOFF_MS: u64 = 12_000;

/// Retry transient failures only for Phase 5 read-only RPC collection.
///
/// The retry budget never applies to signing, submission, minting, burning,
/// settlement, or any other mutating behavior. Uniblock successes are paced
/// because the qualified public endpoint demonstrated burst rate limiting.
pub(super) fn phase5_read_only_rpc_retry<T, F>(
    source: &str,
    action: &str,
    mut call: F,
) -> Result<T, String>
where
    F: FnMut() -> Result<T, String>,
{
    for attempt in 1..=PHASE5_READ_ONLY_RPC_MAX_ATTEMPTS {
        match call() {
            Ok(value) => {
                if source.contains("uniblock") {
                    std::thread::sleep(std::time::Duration::from_millis(
                        PHASE5_UNIBLOCK_SUCCESS_PACING_MS,
                    ));
                }

                return Ok(value);
            }

            Err(error) => {
                let normalized = error.to_ascii_lowercase();

                let transient = [
                    "429",
                    "too many requests",
                    "timed out",
                    "timeout",
                    "dns",
                    "resolve",
                    "connection",
                    "transport",
                    "error sending request",
                    "request error",
                    "502",
                    "503",
                    "504",
                    "service unavailable",
                    "server error",
                ]
                .iter()
                .any(|marker| normalized.contains(marker));

                if !transient || attempt == PHASE5_READ_ONLY_RPC_MAX_ATTEMPTS {
                    return Err(format!(
                        "{source} {action} failed after {attempt} read-only attempt(s): {error}",
                    ));
                }

                let delay_ms =
                    if normalized.contains("429") || normalized.contains("too many requests") {
                        PHASE5_READ_ONLY_RPC_RATE_LIMIT_BACKOFF_MS
                    } else {
                        PHASE5_READ_ONLY_RPC_BASE_BACKOFF_MS * u64::from(attempt)
                    };

                std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            }
        }
    }

    Err(format!(
        "{source} {action} retry loop exhausted unexpectedly",
    ))
}

#[derive(Default)]
struct Phase5Args {
    init_receipt: Option<String>,
    receipt_out: Option<String>,
    rpc_url: Option<String>,
    source: Option<String>,
    help: bool,
}

pub(super) struct Phase4Receipt {
    pub(super) transaction_signature: String,
}

pub fn run_phase5_live_read_only(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_phase5_args(args)?;

    if parsed.help {
        return Ok(phase5_help());
    }

    let init_receipt = parsed
        .init_receipt
        .ok_or_else(|| phase5_error("requires --init-receipt"))?;

    let receipt_out = parsed
        .receipt_out
        .ok_or_else(|| phase5_error("requires --receipt-out"))?;

    let rpc_url = parsed
        .rpc_url
        .ok_or_else(|| phase5_error("requires explicit --rpc-url"))?;

    let source = parsed
        .source
        .ok_or_else(|| phase5_error("requires explicit --source"))?;

    if rpc_url != PHASE5_DEVNET_RPC_URL {
        return Err(phase5_error(
            "Phase 5A currently allows only the explicit official devnet RPC endpoint",
        ));
    }

    validate_source_label(&source)?;

    let phase4 = load_and_validate_phase4_receipt(&init_receipt)?;

    collect_single_source_evidence(
        &phase4,
        &receipt_out,
        &rpc_url,
        &source,
        "explicit-official-devnet",
    )
}

pub(super) fn collect_single_source_evidence(
    phase4: &Phase4Receipt,
    receipt_out: &str,
    rpc_url: &str,
    source: &str,
    endpoint_class: &str,
) -> Result<String, CliError> {
    let program_id = parse_pubkey(PHASE5_PROGRAM_ID, "program ID")?;

    let config_id = parse_pubkey(PHASE5_CONFIG_ACCOUNT, "config account")?;

    let mint_id = parse_pubkey(PHASE5_TEST_ONLY_MINT, "test-only mint")?;

    let token_account_id = parse_pubkey(PHASE5_TEST_ONLY_TOKEN_ACCOUNT, "test-only token account")?;

    let mint_authority = parse_pubkey(PHASE5_MINT_AUTHORITY_PDA, "mint-authority PDA")?;

    let workflow_authority = parse_pubkey(PHASE5_WORKFLOW_AUTHORITY, "workflow authority")?;

    let halt_authority = parse_pubkey(PHASE5_HALT_AUTHORITY, "halt authority")?;

    let recovery_authority = parse_pubkey(PHASE5_RECOVERY_AUTHORITY, "recovery authority")?;

    let signature = Signature::from_str(&phase4.transaction_signature)
        .map_err(|_| phase5_error("Phase 4 receipt transaction signature is invalid"))?;

    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let slot_before = phase5_read_only_rpc_retry(source, "slot-before readback", || {
        rpc.get_slot().map_err(|error| error.to_string())
    })
    .map_err(|message| phase5_error(&message))?;

    let accounts = phase5_read_only_rpc_retry(source, "live account readback", || {
        super::phase5_wire_compat::get_multiple_accounts_compat(
            &rpc,
            source,
            &[program_id, config_id, mint_id, token_account_id],
        )
    })
    .map_err(|message| phase5_error(&message))?;

    if accounts.len() != 4 {
        return Err(phase5_error(
            "live account query returned an unexpected account count",
        ));
    }

    let slot_after = phase5_read_only_rpc_retry(source, "slot-after readback", || {
        rpc.get_slot().map_err(|error| error.to_string())
    })
    .map_err(|message| phase5_error(&message))?;

    if slot_after < slot_before {
        return Err(phase5_error(
            "RPC slot moved backwards during the evidence collection window",
        ));
    }

    let collection_window = slot_after.saturating_sub(slot_before);

    if collection_window > PHASE5_MAX_COLLECTION_WINDOW_SLOTS {
        return Err(phase5_error(
            "read-only RPC evidence collection window exceeded the freshness limit",
        ));
    }

    let program_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase5_error("deployed ROX Anchor program account is missing"))?;

    if !program_account.executable {
        return Err(phase5_error("ROX Anchor program account is not executable"));
    }

    if program_account.owner.to_string() != PHASE5_PROGRAM_OWNER {
        return Err(phase5_error(
            "ROX Anchor program account has the wrong loader owner",
        ));
    }

    let config_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase5_error("ROX Anchor config account is missing"))?;

    if config_account.owner != program_id {
        return Err(phase5_error(
            "ROX Anchor config account has the wrong owner",
        ));
    }

    let mut config_data = config_account.data.as_slice();

    let state = RoxAnchorConfig::try_deserialize(&mut config_data).map_err(|error| {
        phase5_error(&format!(
            "ROX Anchor config readback could not be decoded: {error}"
        ))
    })?;

    if state.authority != workflow_authority
        || state.halt_authority != halt_authority
        || state.recovery_authority != recovery_authority
        || state.rox_mint != mint_id
        || state.mint_authority != mint_authority
    {
        return Err(phase5_error(
            "live config authority or mint binding mismatch",
        ));
    }

    if !state.test_only_mode
        || state.max_supply_units != 1000
        || state.max_amount_units_per_operation != 10
    {
        return Err(phase5_error("live config private-test policy mismatch"));
    }

    if state.halted || state.recovery_required {
        return Err(phase5_error("live config is halted or recovery-blocked"));
    }

    let mint_account = accounts[2]
        .as_ref()
        .ok_or_else(|| phase5_error("test-only ROX mint account is missing"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase5_error(
            "test-only ROX mint is not owned by classic SPL Token",
        ));
    }

    let mint = Mint::unpack(&mint_account.data).map_err(|error| {
        phase5_error(&format!("test-only ROX mint could not be decoded: {error}"))
    })?;

    if !mint.is_initialized
        || mint.decimals != 0
        || mint.supply != 0
        || mint.mint_authority != COption::Some(mint_authority)
        || mint.freeze_authority != COption::None
    {
        return Err(phase5_error("live test-only ROX mint state mismatch"));
    }

    let token_account = accounts[3]
        .as_ref()
        .ok_or_else(|| phase5_error("test-only payer token account is missing"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase5_error(
            "test-only token account is not owned by classic SPL Token",
        ));
    }

    let token_state = SplTokenAccount::unpack(&token_account.data).map_err(|error| {
        phase5_error(&format!(
            "test-only token account could not be decoded: {error}"
        ))
    })?;

    if token_state.owner != workflow_authority
        || token_state.mint != mint_id
        || token_state.amount != 0
    {
        return Err(phase5_error(
            "live token account owner/mint/zero-balance binding mismatch",
        ));
    }

    let signature_status =
        phase5_read_only_rpc_retry(source, "initialization signature history read", || {
            rpc.get_signature_status_with_commitment_and_history(
                &signature,
                CommitmentConfig::confirmed(),
                true,
            )
            .map_err(|error| error.to_string())
        })
        .map_err(|message| phase5_error(&message))?;

    match signature_status {
        Some(Ok(())) => {}
        Some(Err(error)) => {
            return Err(phase5_error(&format!(
                "initialization transaction has an on-chain error: {error}"
            )));
        }
        None => {
            return Err(phase5_error(
                "initialization signature was not found by the selected RPC source",
            ));
        }
    }

    let expected = ExpectedRpcBinding::new(
        ClusterId::new("devnet").map_err(|error| {
            phase5_error(&format!(
                "could not construct devnet cluster binding: {error}"
            ))
        })?,
        ProgramId::new(PHASE5_PROGRAM_ID).map_err(|error| {
            phase5_error(&format!("could not construct program binding: {error}"))
        })?,
        MintId::new(PHASE5_TEST_ONLY_MINT)
            .map_err(|error| phase5_error(&format!("could not construct mint binding: {error}")))?,
        TokenAccountId::new(PHASE5_TEST_ONLY_TOKEN_ACCOUNT).map_err(|error| {
            phase5_error(&format!(
                "could not construct token-account binding: {error}"
            ))
        })?,
        OperationId::new(PHASE5_OPERATION_ID).map_err(|error| {
            phase5_error(&format!("could not construct operation binding: {error}"))
        })?,
        RpcCommitmentLevel::Confirmed,
    );

    let observation = RpcObservation::new(
        source,
        ClusterId::new("devnet").map_err(|error| {
            phase5_error(&format!("could not construct observation cluster: {error}"))
        })?,
        ProgramId::new(PHASE5_PROGRAM_ID).map_err(|error| {
            phase5_error(&format!("could not construct observation program: {error}"))
        })?,
        MintId::new(PHASE5_TEST_ONLY_MINT).map_err(|error| {
            phase5_error(&format!("could not construct observation mint: {error}"))
        })?,
        TokenAccountId::new(PHASE5_TEST_ONLY_TOKEN_ACCOUNT).map_err(|error| {
            phase5_error(&format!(
                "could not construct observation token account: {error}"
            ))
        })?,
        OperationId::new(PHASE5_OPERATION_ID).map_err(|error| {
            phase5_error(&format!(
                "could not construct observation operation: {error}"
            ))
        })?,
        phase4.transaction_signature.clone(),
        slot_after,
        RpcCommitmentLevel::Confirmed,
    );

    let observations = [observation];

    let review = review_rpc_observations(
        &observations,
        &expected,
        RpcProofConfig::new(PHASE5_REQUIRED_OBSERVATIONS, PHASE5_STALE_AFTER_SLOTS),
        slot_after,
    );

    if review.decision != RpcQuorumDecision::MissingEvidence
        || review.accepted_observations != 1
        || review.required_observations != PHASE5_REQUIRED_OBSERVATIONS
    {
        return Err(phase5_error(
            "single-source evidence did not remain explicitly under quorum",
        ));
    }

    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, slot_after);

    if !audit.is_safe_for_display() {
        return Err(phase5_error(
            "rpc-proof audit projection is not display-safe",
        ));
    }

    let observed_at_unix_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| phase5_error("system clock is before UNIX epoch"))?
        .as_secs();

    let redacted_signature = redact_signature(&phase4.transaction_signature);

    let evidence = json!({
        "schema": "rox-anchor.phase5-read-only-source.v1",
        "phase": "BUILD_PLAN4 Phase 5A",
        "cluster": "devnet",
        "rpc_source": source,
        "rpc_endpoint_class": endpoint_class,
        "observed_at_unix_seconds": observed_at_unix_seconds,
        "slot_before": slot_before,
        "slot_after": slot_after,
        "collection_window_slots": collection_window,
        "stale_after_slots": PHASE5_STALE_AFTER_SLOTS,
        "program_id": PHASE5_PROGRAM_ID,
        "program_owner": PHASE5_PROGRAM_OWNER,
        "program_executable": true,
        "program_config": PHASE5_CONFIG_ACCOUNT,
        "workflow_authority": PHASE5_WORKFLOW_AUTHORITY,
        "halt_authority": PHASE5_HALT_AUTHORITY,
        "recovery_authority": PHASE5_RECOVERY_AUTHORITY,
        "upgrade_authority_expected": PHASE5_UPGRADE_AUTHORITY,
        "upgrade_authority_live_metadata": "deferred_to_phase5_multi_source_closeout",
        "test_only_rox_mint": PHASE5_TEST_ONLY_MINT,
        "test_only_token_account": PHASE5_TEST_ONLY_TOKEN_ACCOUNT,
        "mint_authority_pda": PHASE5_MINT_AUTHORITY_PDA,
        "test_only_mode": true,
        "mint_decimals": 0,
        "max_supply_units": 1000,
        "max_amount_units_per_operation": 10,
        "mint_supply": 0,
        "token_account_amount": 0,
        "halted": false,
        "recovery_required": false,
        "initialization_signature_redacted": redacted_signature,
        "initialization_signature_status": "confirmed_success",
        "rpc_proof_operation_id": PHASE5_OPERATION_ID,
        "rpc_proof_minimum_commitment": "confirmed",
        "rpc_proof_observation_count": 1,
        "rpc_proof_required_observations": PHASE5_REQUIRED_OBSERVATIONS,
        "rpc_proof_decision": "MissingEvidence",
        "under_quorum_rejected": true,
        "phase5_closeout": false,
        "transaction_submission": false,
        "keypair_loading": false,
        "signing": false,
        "simulation": false,
        "rox_mint_performed": false,
        "rox_burn_performed": false,
        "real_roc_mutation": false,
        "production_settlement": false,
        "mainnet": false
    });

    let bytes = serde_json::to_vec_pretty(&evidence).map_err(|error| {
        phase5_error(&format!(
            "could not encode redacted read-only evidence: {error}"
        ))
    })?;

    fs::write(receipt_out, [bytes.as_slice(), b"\n"].concat()).map_err(|error| {
        phase5_error(&format!(
            "could not write read-only evidence receipt: {error}"
        ))
    })?;

    Ok([
        "phase5_live_read_only: source_observation".to_string(),
        "phase: BUILD_PLAN4 Phase 5A".to_string(),
        "cluster: devnet".to_string(),
        format!("rpc_source: {source}"),
        format!("slot_before: {slot_before}"),
        format!("slot_after: {slot_after}"),
        format!("collection_window_slots: {collection_window}"),
        format!("program_id: {PHASE5_PROGRAM_ID}"),
        "program_executable: true".to_string(),
        format!("program_config: {PHASE5_CONFIG_ACCOUNT}"),
        format!("test_only_rox_mint: {PHASE5_TEST_ONLY_MINT}"),
        format!("test_only_token_account: {PHASE5_TEST_ONLY_TOKEN_ACCOUNT}"),
        format!("mint_authority_pda: {PHASE5_MINT_AUTHORITY_PDA}"),
        "test_only_mode: true".to_string(),
        "mint_decimals: 0".to_string(),
        "max_supply_units: 1000".to_string(),
        "max_amount_units_per_operation: 10".to_string(),
        "mint_supply: 0".to_string(),
        "token_account_amount: 0".to_string(),
        "halted: false".to_string(),
        "recovery_required: false".to_string(),
        format!("initialization_signature: {redacted_signature}"),
        "initialization_signature_status: confirmed_success".to_string(),
        "state_readback: GREEN".to_string(),
        "rpc_proof_observation_count: 1".to_string(),
        format!("rpc_proof_required_observations: {PHASE5_REQUIRED_OBSERVATIONS}"),
        "rpc_proof_decision: MissingEvidence".to_string(),
        "under_quorum_rejected: true".to_string(),
        "phase5_closeout: false".to_string(),
        "transaction_submission: disabled".to_string(),
        "keypair_loading: disabled".to_string(),
        "signing: disabled".to_string(),
        "simulation: disabled".to_string(),
        "rox_mint_execution: false".to_string(),
        "rox_burn_execution: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
        "next_action: ADD_SECOND_DISTINCT_RPC_SOURCE".to_string(),
    ]
    .join("\n"))
}

pub(super) fn load_and_validate_phase4_receipt(file_path: &str) -> Result<Phase4Receipt, CliError> {
    let bytes = fs::read(file_path).map_err(|error| {
        phase5_error(&format!(
            "could not read Phase 4 initialization receipt: {error}"
        ))
    })?;

    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        phase5_error(&format!(
            "Phase 4 initialization receipt is not valid JSON: {error}"
        ))
    })?;

    require_string(&value, "schema", "rox-anchor.phase4-live-initialization.v1")?;

    require_string(&value, "cluster", "devnet")?;

    require_string(&value, "program_id", PHASE5_PROGRAM_ID)?;

    require_string(&value, "program_config", PHASE5_CONFIG_ACCOUNT)?;

    require_string(&value, "test_only_rox_mint", PHASE5_TEST_ONLY_MINT)?;

    require_string(
        &value,
        "test_only_token_account",
        PHASE5_TEST_ONLY_TOKEN_ACCOUNT,
    )?;

    require_string(&value, "mint_authority_pda", PHASE5_MINT_AUTHORITY_PDA)?;

    require_u64(&value, "mint_decimals", 0)?;

    require_u64(&value, "max_supply_units", 1000)?;

    require_u64(&value, "max_amount_units_per_operation", 10)?;

    require_u64(&value, "initial_mint_supply", 0)?;

    require_u64(&value, "initial_token_account_amount", 0)?;

    for field in [
        "test_only_mode",
        "simulation_before_submission",
        "transaction_confirmed",
        "confirmed_readback",
    ] {
        require_bool(&value, field, true)?;
    }

    for field in [
        "rox_mint_performed",
        "rox_burn_performed",
        "real_roc_mutation",
        "production_settlement",
        "mainnet",
    ] {
        require_bool(&value, field, false)?;
    }

    let transaction_signature = value
        .get("transaction_signature")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| phase5_error("Phase 4 receipt is missing transaction_signature"))?
        .to_string();

    Ok(Phase4Receipt {
        transaction_signature,
    })
}

fn require_string(value: &Value, field: &str, expected: &str) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_str) == Some(expected) {
        return Ok(());
    }

    Err(phase5_error(&format!(
        "Phase 4 receipt field `{field}` does not match the expected private-devnet binding"
    )))
}

fn require_u64(value: &Value, field: &str, expected: u64) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_u64) == Some(expected) {
        return Ok(());
    }

    Err(phase5_error(&format!(
        "Phase 4 receipt field `{field}` does not match the expected numeric policy"
    )))
}

fn require_bool(value: &Value, field: &str, expected: bool) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_bool) == Some(expected) {
        return Ok(());
    }

    Err(phase5_error(&format!(
        "Phase 4 receipt field `{field}` does not match the expected safety posture"
    )))
}

fn parse_phase5_args(args: &[String]) -> Result<Phase5Args, CliError> {
    let mut parsed = Phase5Args::default();

    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                parsed.help = true;
                index += 1;
            }
            "--init-receipt" => {
                parsed.init_receipt = Some(next_value(args, index, "--init-receipt")?);
                index += 2;
            }
            "--receipt-out" => {
                parsed.receipt_out = Some(next_value(args, index, "--receipt-out")?);
                index += 2;
            }
            "--rpc-url" => {
                parsed.rpc_url = Some(next_value(args, index, "--rpc-url")?);
                index += 2;
            }
            "--source" => {
                parsed.source = Some(next_value(args, index, "--source")?);
                index += 2;
            }
            other => {
                return Err(phase5_error(&format!("unknown argument `{other}`")));
            }
        }
    }

    Ok(parsed)
}

fn next_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| phase5_error(&format!("{flag} requires a value")))
}

fn validate_source_label(source: &str) -> Result<(), CliError> {
    if source.is_empty()
        || !source
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_' | '.'))
    {
        return Err(phase5_error(
            "RPC source label must be non-empty ASCII alphanumeric/hyphen/underscore/dot text",
        ));
    }

    let lower = source.to_ascii_lowercase();

    for blocked in [
        "secret",
        "keypair",
        "wallet",
        "mnemonic",
        "seed",
        "credential",
        "password",
        "rpc-url",
    ] {
        if lower.contains(blocked) {
            return Err(phase5_error(
                "RPC source label contains a sensitive-value hint",
            ));
        }
    }

    Ok(())
}

fn parse_pubkey(value: &str, label: &str) -> Result<Pubkey, CliError> {
    Pubkey::from_str(value)
        .map_err(|_| phase5_error(&format!("{label} is not a valid Solana public key")))
}

fn redact_signature(signature: &str) -> String {
    if signature.len() <= 16 {
        return "<redacted-signature>".to_string();
    }

    format!(
        "{}...{}",
        &signature[..8],
        &signature[signature.len() - 4..],
    )
}

fn phase5_help() -> String {
    [
        "BUILD_PLAN4 Phase 5A live read-only evidence",
        "",
        "usage:",
        "  rox-anchor pilot phase5-read-only-live \\",
        "    --init-receipt <ignored-phase4-receipt.json> \\",
        "    --receipt-out <ignored-phase5-receipt.json> \\",
        "    --rpc-url https://api.devnet.solana.com \\",
        "    --source solana-public-devnet-primary",
        "",
        "behavior:",
        "  reads Phase 4 public initialization bindings",
        "  performs live read-only devnet RPC",
        "  loads no operator keypairs",
        "  signs nothing",
        "  simulates nothing",
        "  submits nothing",
        "  verifies program/config/mint/token-account state",
        "  queries the Phase 4 initialization signature status",
        "  converts the result into rox-anchor-rpc-proof",
        "  requires two distinct observations for quorum",
        "  one source must remain MissingEvidence/under-quorum",
        "  writes a redacted source evidence receipt",
        "",
        "Phase 5 is not closed by one source.",
    ]
    .join("\n")
}

fn phase5_error(message: &str) -> CliError {
    CliError::UnknownPilotFlag(format!("phase5-read-only-live {message}"))
}
