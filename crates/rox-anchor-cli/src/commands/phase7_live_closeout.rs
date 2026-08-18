//! BUILD_PLAN4 Phase 7F post-send two-source reconciliation.
//!
//! Re-reads the finalized Phase 7 ROC-to-ROX state from both reviewed Devnet
//! providers, validates exact config/mint/token/operation bindings, reconciles
//! the persisted send/readback receipts, and proves the consumed
//! operation/idempotency/nonce tuple is rejected by the deterministic replay
//! engine. This command is read-only: it has no signer or submission path.

#![forbid(unsafe_code)]

use std::{
    fmt::Debug,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::commitment_config::CommitmentConfig,
};
use anchor_lang::{
    solana_program::{program_option::COption, program_pack::Pack},
    AccountDeserialize,
};
use rox_anchor::{OperationStateCode, RoxAnchorConfig, RoxAnchorOperation};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorDirection, ChallengePosture, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture, TokenAccountId,
};
use rox_anchor_proof::{
    review_proof_package, EvidenceBundle, ProofFindingCode, ProofPackage, ReplaySet, ReviewDecision,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::{
        phase5_live_quorum::{
            PHASE5B_SOURCE1_LABEL, PHASE5B_SOURCE2_LABEL, PHASE5B_SOURCE2_RPC_URL,
        },
        phase5_live_read_only::{
            phase5_read_only_rpc_retry, PHASE5_DEVNET_RPC_URL, PHASE5_STALE_AFTER_SLOTS,
        },
        phase6_live_simulation::validate_phase5_receipt,
        phase7_live_capped_sender::{
            build_phase7_capped_roc_to_rox_plan, validate_phase6_forward_receipt,
            Phase7CappedRocToRoxPlan, PHASE7_AMOUNT_MINOR, PHASE7_IDEMPOTENCY_KEY, PHASE7_NONCE,
            PHASE7_OPERATION_ID, PHASE7_PROGRAM_ID, PHASE7_TEST_ONLY_ROX_MINT,
            PHASE7_TEST_ONLY_TOKEN_ACCOUNT,
        },
        phase7_live_signed_executor::validate_phase7b_authorization_receipt,
    },
    CliError,
};

const PHASE7_SEND_SCHEMA: &str = "rox-anchor.actual-roc-to-rox-capped-send.v1";
const PHASE7_READBACK_SCHEMA: &str = "rox-anchor.actual-roc-to-rox-readback.v1";
const PHASE7F_SCHEMA: &str = "rox-anchor.phase7-post-send-closeout.v1";

#[derive(Clone, Debug, Default)]
struct Phase7FArgs {
    phase5_receipt: Option<String>,
    phase6_receipt: Option<String>,
    phase7b_authorization_receipt: Option<String>,
    send_receipt: Option<String>,
    readback_receipt: Option<String>,
    closeout_receipt_out: Option<String>,
    read_only_closeout: bool,
}

#[derive(Clone, Debug)]
struct ReceiptLinkage {
    send_slot: u64,
    readback_slot: u64,
    signature_sha256: String,
    send_receipt_sha256: String,
    readback_receipt_sha256: String,
}

#[derive(Clone, Debug)]
struct PostSendObservation {
    source: &'static str,
    context_slot: u64,
    config_data: Vec<u8>,
    mint_data: Vec<u8>,
    token_data: Vec<u8>,
    operation_data: Vec<u8>,
}

pub fn run_phase7_post_send_closeout(args: &[String]) -> Result<String, CliError> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        return Ok(help_text());
    }

    let args = parse_args(args)?;

    if !args.read_only_closeout {
        return Err(phase7f_error("--read-only-closeout is required"));
    }

    let phase5_path = required_arg(args.phase5_receipt.as_deref(), "--phase5-receipt")?;
    let phase6_path = required_arg(args.phase6_receipt.as_deref(), "--phase6-receipt")?;
    let phase7b_path = required_arg(
        args.phase7b_authorization_receipt.as_deref(),
        "--phase7b-authorization-receipt",
    )?;
    let send_path = required_arg(args.send_receipt.as_deref(), "--send-receipt")?;
    let readback_path = required_arg(args.readback_receipt.as_deref(), "--readback-receipt")?;
    let closeout_path = required_arg(
        args.closeout_receipt_out.as_deref(),
        "--closeout-receipt-out",
    )?;

    require_ignored_or_absolute_path(phase5_path, "--phase5-receipt")?;
    require_ignored_or_absolute_path(phase6_path, "--phase6-receipt")?;
    require_ignored_or_absolute_path(phase7b_path, "--phase7b-authorization-receipt")?;
    require_ignored_or_absolute_path(send_path, "--send-receipt")?;
    require_ignored_or_absolute_path(readback_path, "--readback-receipt")?;
    require_ignored_or_absolute_path(closeout_path, "--closeout-receipt-out")?;

    let closeout_path = Path::new(closeout_path);
    validate_new_receipt_target(closeout_path)?;

    validate_phase5_receipt(Path::new(phase5_path))?;

    let phase5_bytes = fs::read(phase5_path)
        .map_err(|error| phase7f_error(format!("could not read Phase 5 receipt: {error}")))?;
    let phase6_bytes = fs::read(phase6_path)
        .map_err(|error| phase7f_error(format!("could not read Phase 6 receipt: {error}")))?;
    let phase7b_bytes = fs::read(phase7b_path)
        .map_err(|error| phase7f_error(format!("could not read Phase 7B receipt: {error}")))?;

    let phase6: Value = serde_json::from_slice(&phase6_bytes)
        .map_err(|error| phase7f_error(format!("Phase 6 receipt is not valid JSON: {error}")))?;
    validate_phase6_forward_receipt(&phase6)?;

    let phase7b: Value = serde_json::from_slice(&phase7b_bytes)
        .map_err(|error| phase7f_error(format!("Phase 7B receipt is not valid JSON: {error}")))?;
    validate_phase7b_authorization_receipt(&phase7b)?;

    let phase5_receipt_sha256 = sha256_hex(&phase5_bytes);
    let phase6_receipt_sha256 = sha256_hex(&phase6_bytes);
    let phase7b_receipt_sha256 = sha256_hex(&phase7b_bytes);

    let send_bytes = fs::read(send_path)
        .map_err(|error| phase7f_error(format!("could not read send receipt: {error}")))?;
    let readback_bytes = fs::read(readback_path)
        .map_err(|error| phase7f_error(format!("could not read readback receipt: {error}")))?;

    let send: Value = serde_json::from_slice(&send_bytes)
        .map_err(|error| phase7f_error(format!("send receipt is not valid JSON: {error}")))?;
    let readback: Value = serde_json::from_slice(&readback_bytes)
        .map_err(|error| phase7f_error(format!("readback receipt is not valid JSON: {error}")))?;

    let linkage = validate_receipt_linkage(&send, &readback, &send_bytes, &readback_bytes)?;
    let plan = build_phase7_capped_roc_to_rox_plan()?;

    let minimum_context_slot = linkage.send_slot.max(linkage.readback_slot);

    let (source1_join, source2_join) = std::thread::scope(|scope| {
        let source1_handle = scope.spawn(|| {
            collect_post_send_observation(
                PHASE5_DEVNET_RPC_URL,
                PHASE5B_SOURCE1_LABEL,
                minimum_context_slot,
                &plan,
            )
            .map_err(|error| format!("{error:?}"))
        });

        let source2_handle = scope.spawn(|| {
            collect_post_send_observation(
                PHASE5B_SOURCE2_RPC_URL,
                PHASE5B_SOURCE2_LABEL,
                minimum_context_slot,
                &plan,
            )
            .map_err(|error| format!("{error:?}"))
        });

        (source1_handle.join(), source2_handle.join())
    });

    let source1 = source1_join
        .map_err(|_| phase7f_error("source 1 post-send readback worker panicked"))?
        .map_err(|message| {
            phase7f_error(format!("source 1 post-send readback failed: {message}"))
        })?;

    let source2 = source2_join
        .map_err(|_| phase7f_error("source 2 post-send readback worker panicked"))?
        .map_err(|message| {
            phase7f_error(format!("source 2 post-send readback failed: {message}"))
        })?;

    require_observation_agreement(&source1, &source2)?;

    let source_slot_delta = source1.context_slot.abs_diff(source2.context_slot);
    if source_slot_delta > PHASE5_STALE_AFTER_SLOTS {
        return Err(phase7f_error(
            "post-send provider context slots exceed the reviewed freshness window",
        ));
    }

    prove_replay_rejection()?;

    let receipt = json!({
        "schema": PHASE7F_SCHEMA,
        "phase": "BUILD_PLAN4 Phase 7F",
        "closeout_scope": "post_send_two_source_replay_reconciliation",
        "cluster": "devnet",
        "direction": "roc_to_rox",
        "program_id": PHASE7_PROGRAM_ID,
        "operation_id": PHASE7_OPERATION_ID,
        "idempotency_key": PHASE7_IDEMPOTENCY_KEY,
        "nonce": PHASE7_NONCE,
        "phase5_receipt_sha256": phase5_receipt_sha256,
        "phase6_receipt_sha256": phase6_receipt_sha256,
        "phase7b_authorization_receipt_sha256": phase7b_receipt_sha256,
        "phase5_receipt_verified": true,
        "phase6_forward_receipt_verified": true,
        "phase7b_authorization_receipt_verified": true,
        "send_receipt_sha256": linkage.send_receipt_sha256,
        "readback_receipt_sha256": linkage.readback_receipt_sha256,
        "transaction_signature_sha256": linkage.signature_sha256,
        "send_readback_linkage_verified": true,
        "minimum_context_slot": minimum_context_slot,
        "source_1": source1.source,
        "source_1_context_slot": source1.context_slot,
        "source_2": source2.source,
        "source_2_context_slot": source2.context_slot,
        "source_context_slot_delta": source_slot_delta,
        "two_source_account_bytes_agree": true,
        "config_binding_verified": true,
        "mint_supply_minor": PHASE7_AMOUNT_MINOR.to_string(),
        "workflow_token_amount_minor": PHASE7_AMOUNT_MINOR.to_string(),
        "operation_pda_exists": true,
        "operation_binding_verified": true,
        "operation_state": "finalized",
        "challenge_open": false,
        "recovery_required": false,
        "replay_operation_id_rejected": true,
        "replay_idempotency_key_rejected": true,
        "replay_nonce_rejected": true,
        "replay_transaction_submitted": false,
        "read_only_rpc": true,
        "keypair_loading": false,
        "signing": false,
        "transaction_submission": false,
        "additional_rox_mint": false,
        "real_roc_burn": false,
        "real_roc_mutation": false,
        "production_settlement": false,
        "mainnet_authorized": false,
        "finality_claim": false,
        "phase7_closeout": true,
        "next_action": "BEGIN_BUILD_PLAN4_PHASE8_ACTUAL_CAPPED_ROX_TO_ROC"
    });

    write_new_json(closeout_path, &receipt)?;

    Ok([
        "phase7f_post_send_closeout: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 7F".to_string(),
        "cluster: devnet".to_string(),
        "direction: roc_to_rox".to_string(),
        "phase5_receipt: verified".to_string(),
        "phase6_forward_receipt: verified".to_string(),
        "phase7b_authorization_receipt: verified".to_string(),
        "send_readback_linkage: GREEN".to_string(),
        format!("minimum_context_slot: {minimum_context_slot}"),
        format!("source_1_context_slot: {}", source1.context_slot),
        format!("source_2_context_slot: {}", source2.context_slot),
        format!("source_context_slot_delta: {source_slot_delta}"),
        "two_source_account_bytes: Agreement".to_string(),
        "mint_supply_minor: 1".to_string(),
        "workflow_token_amount_minor: 1".to_string(),
        "operation_pda_exists: true".to_string(),
        "operation_binding: GREEN".to_string(),
        "operation_state: finalized".to_string(),
        "challenge_open: false".to_string(),
        "recovery_required: false".to_string(),
        "replay_operation_id: REJECTED".to_string(),
        "replay_idempotency_key: REJECTED".to_string(),
        "replay_nonce: REJECTED".to_string(),
        "replay_transaction_submitted: false".to_string(),
        "keypair_loading: false".to_string(),
        "signing: false".to_string(),
        "transaction_submission: false".to_string(),
        "additional_rox_mint: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
        format!("closeout_receipt: {}", redact_path(closeout_path)),
        "phase7_closeout: GREEN".to_string(),
        "next_action: BEGIN_BUILD_PLAN4_PHASE8_ACTUAL_CAPPED_ROX_TO_ROC".to_string(),
    ]
    .join("\n"))
}

fn collect_post_send_observation(
    rpc_url: &str,
    source: &'static str,
    minimum_context_slot: u64,
    plan: &Phase7CappedRocToRoxPlan,
) -> Result<PostSendObservation, CliError> {
    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let batch = phase5_read_only_rpc_retry(source, "Phase 7F post-send account readback", || {
        super::phase5_wire_compat::get_multiple_accounts_with_context_compat(
            &rpc,
            source,
            &[
                plan.config,
                plan.test_only_rox_mint,
                plan.test_only_token_account,
                plan.operation,
            ],
            Some(minimum_context_slot),
        )
    })
    .map_err(|error| phase7f_error(format!("{source} post-send account read failed: {error}")))?;

    if batch.context_slot < minimum_context_slot {
        return Err(phase7f_error(format!(
            "{source} returned context slot older than the send/readback evidence"
        )));
    }

    if batch.accounts.len() != 4 {
        return Err(phase7f_error(format!(
            "{source} returned unexpected post-send account count"
        )));
    }

    let config_account = batch.accounts[0]
        .as_ref()
        .ok_or_else(|| phase7f_error(format!("{source} config account is missing")))?;

    if config_account.owner != plan.program_id {
        return Err(phase7f_error(format!("{source} config owner mismatch")));
    }

    let mut config_bytes = config_account.data.as_slice();
    let config = RoxAnchorConfig::try_deserialize(&mut config_bytes)
        .map_err(|error| phase7f_error(format!("{source} config decode failed: {error}")))?;

    if config.authority != plan.workflow_authority
        || config.rox_mint != plan.test_only_rox_mint
        || config.mint_authority != plan.mint_authority
        || !config.test_only_mode
        || config.max_supply_units != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS
        || config.max_amount_units_per_operation
            != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS
        || config.halted
        || config.recovery_required
    {
        return Err(phase7f_error(format!(
            "{source} config binding/posture mismatch"
        )));
    }

    let mint_account = batch.accounts[1]
        .as_ref()
        .ok_or_else(|| phase7f_error(format!("{source} test-only ROX mint is missing")))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase7f_error(format!("{source} mint owner mismatch")));
    }

    let mint = Mint::unpack(&mint_account.data)
        .map_err(|error| phase7f_error(format!("{source} mint decode failed: {error}")))?;

    if mint.decimals != 0
        || mint.supply != PHASE7_AMOUNT_MINOR
        || mint.mint_authority != COption::Some(plan.mint_authority)
        || mint.freeze_authority != COption::None
    {
        return Err(phase7f_error(format!("{source} mint state mismatch")));
    }

    let token_account = batch.accounts[2]
        .as_ref()
        .ok_or_else(|| phase7f_error(format!("{source} workflow token account is missing")))?;

    if token_account.owner != spl_token::id() {
        return Err(phase7f_error(format!(
            "{source} token-account program owner mismatch"
        )));
    }

    let token = SplTokenAccount::unpack(&token_account.data)
        .map_err(|error| phase7f_error(format!("{source} token-account decode failed: {error}")))?;

    if token.mint != plan.test_only_rox_mint
        || token.owner != plan.workflow_authority
        || token.amount != PHASE7_AMOUNT_MINOR
    {
        return Err(phase7f_error(format!(
            "{source} workflow token-account state mismatch"
        )));
    }

    let operation_account = batch.accounts[3]
        .as_ref()
        .ok_or_else(|| phase7f_error(format!("{source} Phase 7 operation PDA is missing")))?;

    if operation_account.owner != plan.program_id {
        return Err(phase7f_error(format!("{source} operation owner mismatch")));
    }

    let mut operation_bytes = operation_account.data.as_slice();
    let operation = RoxAnchorOperation::try_deserialize(&mut operation_bytes)
        .map_err(|error| phase7f_error(format!("{source} operation decode failed: {error}")))?;

    if operation.authority != plan.workflow_authority
        || operation.operation_id_hash != plan.operation_id_hash
        || operation.mint != plan.test_only_rox_mint
        || operation.token_account != plan.test_only_token_account
        || operation.amount_atoms != PHASE7_AMOUNT_MINOR
        || operation.burn_evidence_hash != plan.burn_evidence_hash
        || !operation.is_roc_to_rox()
        || operation.state_code() != Some(OperationStateCode::Finalized)
        || operation.challenge_open
        || operation.recovery_required
    {
        return Err(phase7f_error(format!(
            "{source} finalized operation binding/posture mismatch"
        )));
    }

    Ok(PostSendObservation {
        source,
        context_slot: batch.context_slot,
        config_data: config_account.data.clone(),
        mint_data: mint_account.data.clone(),
        token_data: token_account.data.clone(),
        operation_data: operation_account.data.clone(),
    })
}

fn require_observation_agreement(
    source1: &PostSendObservation,
    source2: &PostSendObservation,
) -> Result<(), CliError> {
    if source1.config_data != source2.config_data
        || source1.mint_data != source2.mint_data
        || source1.token_data != source2.token_data
        || source1.operation_data != source2.operation_data
    {
        return Err(phase7f_error(
            "independent providers disagree on post-send account bytes",
        ));
    }

    Ok(())
}

fn prove_replay_rejection() -> Result<(), CliError> {
    let binding = AnchorBinding::new(
        core_id(
            DomainId::new("internal-roc-private-pilot-test"),
            "source domain",
        )?,
        core_id(
            DomainId::new("solana-devnet-rox-private-pilot-test"),
            "target domain",
        )?,
        AnchorDirection::RocToRox,
        core_id(ClusterId::new("devnet"), "cluster")?,
        core_id(ProgramId::new(PHASE7_PROGRAM_ID), "program id")?,
        core_id(MintId::new(PHASE7_TEST_ONLY_ROX_MINT), "mint")?,
        core_id(
            TokenAccountId::new(PHASE7_TEST_ONLY_TOKEN_ACCOUNT),
            "token account",
        )?,
    );

    let package = ProofPackage::new(
        binding,
        core_id(OperationId::new(PHASE7_OPERATION_ID), "operation id")?,
        core_id(
            IdempotencyKey::new(PHASE7_IDEMPOTENCY_KEY),
            "idempotency key",
        )?,
        core_id(Nonce::new(PHASE7_NONCE), "nonce")?,
        core_id(
            AccountId::new("shadow-roc-burn-source-phase7"),
            "source account",
        )?,
        core_id(
            AccountId::new("actual-private-rox-recipient-phase7"),
            "target account",
        )?,
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );

    let expected = package.expected_binding_snapshot();
    let replay = ReplaySet::from_package(&package);
    let review = review_proof_package(&package, &expected, &replay);

    let codes = review
        .findings
        .iter()
        .map(|finding| finding.code)
        .collect::<Vec<_>>();

    let expected_codes = vec![
        ProofFindingCode::ReplayOperationId,
        ProofFindingCode::ReplayIdempotencyKey,
        ProofFindingCode::ReplayNonce,
    ];

    if review.decision != ReviewDecision::Rejected || codes != expected_codes {
        return Err(phase7f_error(format!(
            "consumed Phase 7 identity was not deterministically rejected as replay: decision={:?}, findings={codes:?}",
            review.decision,
        )));
    }

    Ok(())
}

fn validate_receipt_linkage(
    send: &Value,
    readback: &Value,
    send_bytes: &[u8],
    readback_bytes: &[u8],
) -> Result<ReceiptLinkage, CliError> {
    require_string(send, "schema", PHASE7_SEND_SCHEMA)?;
    require_string(send, "phase", "BUILD_PLAN4 Phase 7")?;
    require_string(
        send,
        "receipt_role",
        "actual_roc_to_rox_capped_send_receipt",
    )?;
    require_string(send, "cluster", "devnet")?;
    require_string(send, "direction", "roc_to_rox")?;
    require_string(send, "program_id", PHASE7_PROGRAM_ID)?;
    require_string(send, "send_outcome", "sent")?;
    require_phase7_identity(send)?;
    require_string(send, "amount_minor", "1")?;
    require_bool(send, "shadow_roc_burn_only", true)?;
    require_bool(send, "transaction_submission", true)?;
    require_bool(send, "real_roc_burn", false)?;
    require_bool(send, "real_roc_mutation", false)?;

    require_string(readback, "schema", PHASE7_READBACK_SCHEMA)?;
    require_string(readback, "phase", "BUILD_PLAN4 Phase 7")?;
    require_string(
        readback,
        "receipt_role",
        "actual_roc_to_rox_readback_receipt",
    )?;
    require_string(readback, "cluster", "devnet")?;
    require_string(readback, "direction", "roc_to_rox")?;
    require_string(readback, "program_id", PHASE7_PROGRAM_ID)?;
    require_string(readback, "readback_outcome", "verified")?;
    require_phase7_identity(readback)?;
    require_string(readback, "expected_test_only_rox_delta_minor", "1")?;
    require_string(readback, "observed_test_only_rox_delta_minor", "1")?;
    require_string(readback, "operation_state", "finalized")?;
    require_bool(readback, "read_only_rpc", true)?;
    require_bool(readback, "transaction_submission", false)?;
    require_bool(readback, "real_roc_burn", false)?;
    require_bool(readback, "real_roc_mutation", false)?;

    let send_signature = require_nonempty_string(send, "transaction_signature_sha256")?;
    let readback_signature = require_nonempty_string(readback, "transaction_signature_sha256")?;

    if send_signature != readback_signature {
        return Err(phase7f_error(
            "send/readback transaction signature hashes do not match",
        ));
    }

    let send_slot = require_u64_string(send, "send_slot")?;
    let readback_slot = require_u64_string(readback, "readback_slot")?;

    if readback_slot < send_slot {
        return Err(phase7f_error("readback slot predates confirmed send slot"));
    }

    Ok(ReceiptLinkage {
        send_slot,
        readback_slot,
        signature_sha256: send_signature.to_string(),
        send_receipt_sha256: sha256_hex(send_bytes),
        readback_receipt_sha256: sha256_hex(readback_bytes),
    })
}

fn require_phase7_identity(value: &Value) -> Result<(), CliError> {
    require_string(value, "operation_id", PHASE7_OPERATION_ID)?;
    require_string(value, "idempotency_key", PHASE7_IDEMPOTENCY_KEY)?;
    require_string(value, "nonce", PHASE7_NONCE)
}

fn require_string(value: &Value, field: &str, expected: &str) -> Result<(), CliError> {
    let actual = value.get(field).and_then(Value::as_str);
    if actual != Some(expected) {
        return Err(phase7f_error(format!(
            "receipt field {field} must be exactly {expected}"
        )));
    }
    Ok(())
}

fn require_nonempty_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, CliError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| phase7f_error(format!("receipt field {field} must be a non-empty string")))
}

fn require_bool(value: &Value, field: &str, expected: bool) -> Result<(), CliError> {
    let actual = value.get(field).and_then(Value::as_bool);
    if actual != Some(expected) {
        return Err(phase7f_error(format!(
            "receipt field {field} must be exactly {expected}"
        )));
    }
    Ok(())
}

fn require_u64_string(value: &Value, field: &str) -> Result<u64, CliError> {
    require_nonempty_string(value, field)?
        .parse::<u64>()
        .map_err(|_| phase7f_error(format!("receipt field {field} must contain a u64 string")))
}

fn core_id<T, E: Debug>(value: Result<T, E>, label: &str) -> Result<T, CliError> {
    value.map_err(|error| phase7f_error(format!("invalid {label}: {error:?}")))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(64);
    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn write_new_json(path: &Path, value: &Value) -> Result<(), CliError> {
    let encoded = serde_json::to_vec_pretty(value)
        .map_err(|error| phase7f_error(format!("could not encode closeout receipt: {error}")))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| phase7f_error(format!("could not create closeout receipt: {error}")))?;

    file.write_all(&encoded)
        .map_err(|error| phase7f_error(format!("could not write closeout receipt: {error}")))?;
    file.write_all(b"\n")
        .map_err(|error| phase7f_error(format!("could not terminate closeout receipt: {error}")))?;
    file.sync_all()
        .map_err(|error| phase7f_error(format!("could not sync closeout receipt: {error}")))
}

fn validate_new_receipt_target(path: &Path) -> Result<(), CliError> {
    if path.exists() {
        return Err(phase7f_error(
            "closeout receipt already exists; refusing overwrite",
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| phase7f_error("closeout receipt has no parent directory"))?;
    if !parent.is_dir() {
        return Err(phase7f_error(
            "closeout receipt parent directory does not exist",
        ));
    }
    Ok(())
}

fn require_ignored_or_absolute_path(value: &str, flag: &str) -> Result<(), CliError> {
    let path = Path::new(value);
    if !path.is_absolute() && !value.starts_with(".rox-anchor-private-pilot/") {
        return Err(phase7f_error(format!(
            "{flag} must be absolute or inside .rox-anchor-private-pilot"
        )));
    }
    Ok(())
}

fn redact_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("<redacted-local-path>/{name}"))
        .unwrap_or_else(|| "<redacted-local-path>".to_string())
}

fn required_arg<'a>(value: Option<&'a str>, flag: &str) -> Result<&'a str, CliError> {
    value.ok_or_else(|| phase7f_error(format!("{flag} is required")))
}

fn parse_args(args: &[String]) -> Result<Phase7FArgs, CliError> {
    let mut parsed = Phase7FArgs::default();
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--phase5-receipt" => {
                parsed.phase5_receipt = Some(next_value(args, index, "--phase5-receipt")?);
                index += 2;
            }
            "--phase6-receipt" => {
                parsed.phase6_receipt = Some(next_value(args, index, "--phase6-receipt")?);
                index += 2;
            }
            "--phase7b-authorization-receipt" => {
                parsed.phase7b_authorization_receipt =
                    Some(next_value(args, index, "--phase7b-authorization-receipt")?);
                index += 2;
            }
            "--send-receipt" => {
                parsed.send_receipt = Some(next_value(args, index, "--send-receipt")?);
                index += 2;
            }
            "--readback-receipt" => {
                parsed.readback_receipt = Some(next_value(args, index, "--readback-receipt")?);
                index += 2;
            }
            "--closeout-receipt-out" => {
                parsed.closeout_receipt_out =
                    Some(next_value(args, index, "--closeout-receipt-out")?);
                index += 2;
            }
            "--read-only-closeout" => {
                parsed.read_only_closeout = true;
                index += 1;
            }
            other => return Err(phase7f_error(format!("unknown argument `{other}`"))),
        }
    }

    Ok(parsed)
}

fn next_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| phase7f_error(format!("{flag} requires a value")))
}

fn help_text() -> String {
    [
        "BUILD_PLAN4 Phase 7F post-send two-source replay/reconciliation closeout",
        "",
        "required:",
        "  --phase5-receipt <fresh-phase5-closeout>",
        "  --phase6-receipt <forward-phase6-simulation>",
        "  --phase7b-authorization-receipt <phase7b-authorization>",
        "  --send-receipt <phase7-send-receipt>",
        "  --readback-receipt <phase7-readback-receipt>",
        "  --closeout-receipt-out <new-ignored-local-receipt>",
        "  --read-only-closeout",
        "",
        "effects:",
        "  Solana public Devnet read-only RPC: YES",
        "  Uniblock Devnet read-only RPC: YES",
        "  exact finalized operation binding verification: YES",
        "  deterministic replay rejection proof: YES",
        "  keypair loading: NO",
        "  signing: NO",
        "  transaction submission: NO",
        "  additional ROX mint: NO",
        "  real ROC mutation: NO",
    ]
    .join("\n")
}

fn phase7f_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!("phase7-post-send-closeout {}", message.as_ref()))
}
