//! RO:WHAT — Performs BUILD_PLAN4 Phase 5B2 read-only deployment metadata
//! verification after obtaining a fresh two-provider Phase 5B1 state quorum.
//! RO:WHY — Phase 5 cannot close merely because config/mint state agrees;
//! loader-v3 ProgramData, upgrade authority, deployed binary, and the final
//! Phase 4E deployment signature must also agree across independent RPCs.
//! RO:INTERACTS — Phase 5B1 quorum command, Solana public Devnet, Uniblock
//! Devnet, loader-v3 Program/ProgramData state, SHA-256, and Phase 4E bindings.
//! RO:INVARIANTS — fixed providers only; fresh B1 Agreement required;
//! exact ProgramData pointer/authority/slot/binary hash/capacity/padding;
//! exact known Phase 4E deployment signature must be successful on both.
//! RO:SECURITY — read-only RPC only; no keys, signing, simulation,
//! transaction submission, ROX mint/burn, ROC mutation, settlement, or mainnet.
//! RO:TEST — phase5_live_closeout_source.rs plus B1 regression tests.

#![forbid(unsafe_code)]

use std::{fs, str::FromStr};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature},
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use solana_loader_v3_interface::state::UpgradeableLoaderState;

use crate::{
    commands::{
        phase5_live_quorum::{
            run_phase5_live_quorum, PHASE5B_SOURCE1_ENDPOINT_CLASS, PHASE5B_SOURCE1_LABEL,
            PHASE5B_SOURCE2_ENDPOINT_CLASS, PHASE5B_SOURCE2_LABEL, PHASE5B_SOURCE2_RPC_URL,
        },
        phase5_live_read_only::{
            phase5_read_only_rpc_retry, PHASE5_DEVNET_RPC_URL, PHASE5_PROGRAM_ID,
            PHASE5_PROGRAM_OWNER, PHASE5_REQUIRED_OBSERVATIONS, PHASE5_STALE_AFTER_SLOTS,
        },
    },
    CliError,
};

const PHASE5B2_PROGRAMDATA: &str = "4JsBSTEXLKtWusJQJAv1DnaRKfAxGnD958WhHPVz84UD";

const PHASE5B2_UPGRADE_AUTHORITY: &str = "DLQJ1icSQKu5CGsi7FqJgF9ohsiYuYuRkn23EggRDTdJ";

const PHASE5B2_DEPLOYMENT_SLOT: u64 = 484_017_674;

const PHASE5B2_PROGRAMDATA_PAYLOAD_BYTES: usize = 398_864;

const PHASE5B2_MEANINGFUL_PROGRAM_BYTES: usize = 392_488;

const PHASE5B2_TRAILING_ZERO_PADDING_BYTES: usize = 6_376;

const PHASE5B2_PROGRAM_PREFIX_SHA256: &str =
    "929f1906d497ed22c8e88c8a73bcaae0181271d9001fa1a98a9f8e3c50c45bf1";

const PHASE5B2_DEPLOY_SIGNATURE: &str =
    "3hcbn13eMpvTHqrwMeJdFVND4jsGy3RpbVUSKNJv4PJinpkgtboAMo5BAbxwhTJstDzA5AKPQxH96Rk7atStm4tT";

#[derive(Default)]
struct Phase5B2Args {
    init_receipt: Option<String>,
    receipt_out: Option<String>,
    help: bool,
}

#[derive(Debug, Clone, Copy)]
struct FreshQuorum {
    source1_slot: u64,
    source2_slot: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeploymentObservation {
    source: String,
    endpoint_class: String,
    observation_slot: u64,
    programdata_slot: u64,
    programdata_payload_bytes: usize,
    meaningful_program_bytes: usize,
    trailing_zero_padding_bytes: usize,
    program_prefix_sha256: String,
}

pub fn run_phase5_live_closeout(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help());
    }

    let init_receipt = parsed
        .init_receipt
        .ok_or_else(|| phase5b2_error("requires --init-receipt"))?;

    let receipt_out = parsed
        .receipt_out
        .ok_or_else(|| phase5b2_error("requires --receipt-out"))?;

    let fresh_quorum_path = format!("{receipt_out}.quorum.json");

    let quorum_args = vec![
        "--init-receipt".to_string(),
        init_receipt,
        "--receipt-out".to_string(),
        fresh_quorum_path.clone(),
    ];

    run_phase5_live_quorum(&quorum_args).map_err(|error| {
        phase5b2_error(&format!(
            "fresh Phase 5B1 quorum failed before deployment metadata review: {error}"
        ))
    })?;

    let quorum = load_fresh_quorum(&fresh_quorum_path)?;

    verify_deploy_signature_history(PHASE5_DEVNET_RPC_URL, PHASE5B_SOURCE1_LABEL)?;

    verify_deploy_signature_history(PHASE5B_SOURCE2_RPC_URL, PHASE5B_SOURCE2_LABEL)?;

    let metadata_min_context_slot = quorum.source1_slot.max(quorum.source2_slot);

    let (source1_join, source2_join) = std::thread::scope(|scope| {
        let source1_handle = scope.spawn(|| {
            collect_deployment_observation(
                PHASE5_DEVNET_RPC_URL,
                PHASE5B_SOURCE1_LABEL,
                PHASE5B_SOURCE1_ENDPOINT_CLASS,
                metadata_min_context_slot,
            )
            .map_err(|error| error.to_string())
        });

        let source2_handle = scope.spawn(|| {
            collect_deployment_observation(
                PHASE5B_SOURCE2_RPC_URL,
                PHASE5B_SOURCE2_LABEL,
                PHASE5B_SOURCE2_ENDPOINT_CLASS,
                metadata_min_context_slot,
            )
            .map_err(|error| error.to_string())
        });

        (source1_handle.join(), source2_handle.join())
    });

    let source1 = source1_join
        .map_err(|_| phase5b2_error("source 1 deployment metadata worker panicked"))?
        .map_err(|message| {
            phase5b2_error(&format!(
                "source 1 parallel deployment metadata read failed: {message}"
            ))
        })?;

    let source2 = source2_join
        .map_err(|_| phase5b2_error("source 2 deployment metadata worker panicked"))?
        .map_err(|message| {
            phase5b2_error(&format!(
                "source 2 parallel deployment metadata read failed: {message}"
            ))
        })?;

    require_matching_deployment_metadata(&source1, &source2)?;

    if source1.observation_slot < metadata_min_context_slot {
        return Err(phase5b2_error(
            "source 1 deployment metadata response did not satisfy the common minimum context slot",
        ));
    }

    if source2.observation_slot < metadata_min_context_slot {
        return Err(phase5b2_error(
            "source 2 deployment metadata response did not satisfy the common minimum context slot",
        ));
    }

    let source1_state_to_metadata_slot_delta = source1.observation_slot - quorum.source1_slot;

    let source2_state_to_metadata_slot_delta = source2.observation_slot - quorum.source2_slot;

    let quorum_max_slot = quorum.source1_slot.max(quorum.source2_slot);

    let metadata_max_slot = source1.observation_slot.max(source2.observation_slot);

    let state_quorum_to_metadata_slot_delta = metadata_max_slot - quorum_max_slot;

    let metadata_source_slot_delta = source1.observation_slot.abs_diff(source2.observation_slot);

    if metadata_source_slot_delta > PHASE5_STALE_AFTER_SLOTS {
        return Err(phase5b2_error(
            "independent deployment metadata observations exceed the Phase 5 freshness window",
        ));
    }

    let deploy_signature_redacted = redact_signature(PHASE5B2_DEPLOY_SIGNATURE);

    let receipt = json!({
        "schema": "rox-anchor.phase5-read-only-closeout.v1",
        "phase": "BUILD_PLAN4 Phase 5B2",
        "cluster": "devnet",
        "closeout_scope": "state_quorum_plus_loader_v3_deployment_metadata",
        "rpc_source_1": PHASE5B_SOURCE1_LABEL,
        "rpc_source_1_endpoint_class": PHASE5B_SOURCE1_ENDPOINT_CLASS,
        "rpc_source_1_metadata_slot": source1.observation_slot,
        "rpc_source_2": PHASE5B_SOURCE2_LABEL,
        "rpc_source_2_endpoint_class": PHASE5B_SOURCE2_ENDPOINT_CLASS,
        "rpc_source_2_metadata_slot": source2.observation_slot,
        "rpc_sources_distinct": true,
        "rpc_proof_observation_count": PHASE5_REQUIRED_OBSERVATIONS,
        "rpc_proof_required_observations": PHASE5_REQUIRED_OBSERVATIONS,
        "rpc_proof_decision": "Agreement",
        "fresh_state_quorum": true,
        "state_quorum_fresh_by_rpc_proof": true,
        "metadata_min_context_slot": metadata_min_context_slot,
        "metadata_min_context_enforced": true,
        "metadata_collection_mode": "parallel_common_min_context",
        "metadata_source_1_not_older_than_fresh_quorum": true,
        "metadata_source_2_not_older_than_fresh_quorum": true,
        "source1_state_to_metadata_slot_delta": source1_state_to_metadata_slot_delta,
        "source2_state_to_metadata_slot_delta": source2_state_to_metadata_slot_delta,
        "state_quorum_to_metadata_slot_delta": state_quorum_to_metadata_slot_delta,
        "cross_stage_slot_delta_policy": "telemetry_not_rpc_staleness",
        "metadata_source_slot_delta": metadata_source_slot_delta,
        "metadata_pair_fresh": true,
        "stale_after_slots": PHASE5_STALE_AFTER_SLOTS,
        "program_id": PHASE5_PROGRAM_ID,
        "program_loader": "BPFLoaderUpgradeab1e11111111111111111111111",
        "programdata": PHASE5B2_PROGRAMDATA,
        "programdata_pointer_multi_source_verified": true,
        "upgrade_authority": PHASE5B2_UPGRADE_AUTHORITY,
        "upgrade_authority_multi_source_verified": true,
        "deployment_slot": PHASE5B2_DEPLOYMENT_SLOT,
        "deployment_metadata_multi_source_verified": true,
        "programdata_payload_bytes": PHASE5B2_PROGRAMDATA_PAYLOAD_BYTES,
        "meaningful_program_bytes": PHASE5B2_MEANINGFUL_PROGRAM_BYTES,
        "trailing_zero_padding_bytes": PHASE5B2_TRAILING_ZERO_PADDING_BYTES,
        "program_prefix_sha256": PHASE5B2_PROGRAM_PREFIX_SHA256,
        "program_binary_multi_source_verified": true,
        "deploy_signature_redacted": deploy_signature_redacted,
        "deploy_signature_status": "confirmed_success_or_better",
        "deploy_signature_multi_source_verified": true,
        "initialization_signature_multi_source_verified": true,
        "state_readback_both_sources": true,
        "phase5_closeout": true,
        "read_only_rpc": true,
        "transaction_submission": false,
        "keypair_loading": false,
        "signing": false,
        "simulation": false,
        "rox_mint_performed": false,
        "rox_burn_performed": false,
        "real_roc_mutation": false,
        "production_settlement": false,
        "finality_claim": false,
        "settlement_claim": false,
        "mainnet": false,
        "next_action": "BEGIN_BUILD_PLAN4_PHASE6_ACTUAL_ADDRESS_SIMULATION"
    });

    let bytes = serde_json::to_vec_pretty(&receipt).map_err(|error| {
        phase5b2_error(&format!(
            "could not encode Phase 5 closeout receipt: {error}"
        ))
    })?;

    fs::write(&receipt_out, [bytes.as_slice(), b"\n"].concat()).map_err(|error| {
        phase5b2_error(&format!(
            "could not write Phase 5 closeout receipt: {error}"
        ))
    })?;

    Ok([
        "phase5_live_closeout: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 5B2".to_string(),
        "cluster: devnet".to_string(),
        "rpc_proof_decision: Agreement".to_string(),
        "rpc_proof_observation_count: 2".to_string(),
        "rpc_proof_required_observations: 2".to_string(),
        "fresh_state_quorum: true".to_string(),
        "state_quorum_fresh_by_rpc_proof: true".to_string(),
        format!("metadata_min_context_slot: {metadata_min_context_slot}"),
        "metadata_min_context_enforced: true".to_string(),
        "metadata_collection_mode: parallel_common_min_context".to_string(),
        "metadata_source_1_not_older_than_fresh_quorum: true".to_string(),
        "metadata_source_2_not_older_than_fresh_quorum: true".to_string(),
        format!("source1_state_to_metadata_slot_delta: {source1_state_to_metadata_slot_delta}"),
        format!("source2_state_to_metadata_slot_delta: {source2_state_to_metadata_slot_delta}"),
        format!("state_quorum_to_metadata_slot_delta: {state_quorum_to_metadata_slot_delta}"),
        "cross_stage_slot_delta_policy: telemetry_not_rpc_staleness".to_string(),
        format!("metadata_source_slot_delta: {metadata_source_slot_delta}"),
        "metadata_pair_fresh: true".to_string(),
        format!("programdata: {PHASE5B2_PROGRAMDATA}"),
        "programdata_pointer_multi_source_verified: true".to_string(),
        format!("upgrade_authority: {PHASE5B2_UPGRADE_AUTHORITY}"),
        "upgrade_authority_multi_source_verified: true".to_string(),
        format!("deployment_slot: {PHASE5B2_DEPLOYMENT_SLOT}"),
        "deployment_metadata_multi_source_verified: true".to_string(),
        format!("programdata_payload_bytes: {PHASE5B2_PROGRAMDATA_PAYLOAD_BYTES}"),
        format!("meaningful_program_bytes: {PHASE5B2_MEANINGFUL_PROGRAM_BYTES}"),
        format!("trailing_zero_padding_bytes: {PHASE5B2_TRAILING_ZERO_PADDING_BYTES}"),
        format!("program_prefix_sha256: {PHASE5B2_PROGRAM_PREFIX_SHA256}"),
        "program_binary_multi_source_verified: true".to_string(),
        "deploy_signature_multi_source_verified: true".to_string(),
        "phase5_closeout: true".to_string(),
        "read_only_rpc: true".to_string(),
        "transaction_submission: disabled".to_string(),
        "keypair_loading: disabled".to_string(),
        "signing: disabled".to_string(),
        "simulation: disabled".to_string(),
        "rox_mint_execution: false".to_string(),
        "rox_burn_execution: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "finality_claim: false".to_string(),
        "settlement_claim: false".to_string(),
        "mainnet_authorized: false".to_string(),
        format!("receipt_out: {receipt_out}"),
        "next_action: BEGIN_BUILD_PLAN4_PHASE6_ACTUAL_ADDRESS_SIMULATION".to_string(),
    ]
    .join("\n"))
}

fn verify_deploy_signature_history(rpc_url: &str, source: &str) -> Result<(), CliError> {
    let deploy_signature = Signature::from_str(PHASE5B2_DEPLOY_SIGNATURE)
        .map_err(|_| phase5b2_error("locked Phase 4E deployment signature is invalid"))?;

    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let deploy_status =
        phase5_read_only_rpc_retry(source, "Phase 4E deployment signature history read", || {
            rpc.get_signature_status_with_commitment_and_history(
                &deploy_signature,
                CommitmentConfig::confirmed(),
                true,
            )
            .map_err(|error| error.to_string())
        })
        .map_err(|message| phase5b2_error(&message))?;

    match deploy_status {
        Some(Ok(())) => Ok(()),
        Some(Err(error)) => Err(phase5b2_error(&format!(
            "Phase 4E deployment signature has an on-chain error: {error}"
        ))),
        None => Err(phase5b2_error(
            "Phase 4E deployment signature was not found in transaction history",
        )),
    }
}

fn collect_deployment_observation(
    rpc_url: &str,
    source: &str,
    endpoint_class: &str,
    minimum_context_slot: u64,
) -> Result<DeploymentObservation, CliError> {
    let program_id = parse_pubkey(PHASE5_PROGRAM_ID, "program ID")?;

    let expected_programdata = parse_pubkey(PHASE5B2_PROGRAMDATA, "ProgramData account")?;

    let expected_upgrade_authority = parse_pubkey(PHASE5B2_UPGRADE_AUTHORITY, "upgrade authority")?;

    let loader_id = parse_pubkey(PHASE5_PROGRAM_OWNER, "loader-v3 program ID")?;

    let rpc = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let account_response =
        phase5_read_only_rpc_retry(source, "deployment metadata account read", || {
            super::phase5_wire_compat::get_multiple_accounts_with_context_compat(
                &rpc,
                source,
                &[program_id, expected_programdata],
                Some(minimum_context_slot),
            )
        })
        .map_err(|message| phase5b2_error(&message))?;

    let observation_slot = account_response.context_slot;

    let accounts = account_response.accounts;

    if accounts.len() != 2 {
        return Err(phase5b2_error(
            "deployment metadata RPC returned an unexpected account count",
        ));
    }

    let program_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase5b2_error("deployed Program account is missing"))?;

    let programdata_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase5b2_error("ProgramData account is missing"))?;

    if program_account.owner != loader_id {
        return Err(phase5b2_error("Program account is not owned by loader-v3"));
    }

    if !program_account.executable {
        return Err(phase5b2_error("Program account is not executable"));
    }

    let program_state: UpgradeableLoaderState = bincode::deserialize(&program_account.data)
        .map_err(|error| {
            phase5b2_error(&format!(
                "Program loader state could not be decoded: {error}"
            ))
        })?;

    match program_state {
        UpgradeableLoaderState::Program {
            programdata_address,
        } if programdata_address == expected_programdata => {}
        UpgradeableLoaderState::Program { .. } => {
            return Err(phase5b2_error(
                "Program points to an unexpected ProgramData account",
            ));
        }
        _ => {
            return Err(phase5b2_error(
                "deployed Program account does not contain loader-v3 Program state",
            ));
        }
    }

    if programdata_account.owner != loader_id {
        return Err(phase5b2_error(
            "ProgramData account is not owned by loader-v3",
        ));
    }

    if programdata_account.executable {
        return Err(phase5b2_error(
            "ProgramData account unexpectedly claims executable=true",
        ));
    }

    let metadata_len = UpgradeableLoaderState::size_of_programdata_metadata();

    if programdata_account.data.len() < metadata_len {
        return Err(phase5b2_error(
            "ProgramData account is shorter than loader-v3 metadata",
        ));
    }

    let programdata_state: UpgradeableLoaderState =
        bincode::deserialize(&programdata_account.data[..metadata_len]).map_err(|error| {
            phase5b2_error(&format!(
                "ProgramData loader metadata could not be decoded: {error}"
            ))
        })?;

    let (programdata_slot, observed_upgrade_authority) = match programdata_state {
        UpgradeableLoaderState::ProgramData {
            slot,
            upgrade_authority_address,
        } => (slot, upgrade_authority_address),
        _ => {
            return Err(phase5b2_error(
                "ProgramData account does not contain loader-v3 ProgramData state",
            ));
        }
    };

    if programdata_slot != PHASE5B2_DEPLOYMENT_SLOT {
        return Err(phase5b2_error(
            "ProgramData deployment slot does not match the locked Phase 4E slot",
        ));
    }

    if observed_upgrade_authority != Some(expected_upgrade_authority) {
        return Err(phase5b2_error(
            "ProgramData upgrade authority does not match the dedicated Phase 4 authority",
        ));
    }

    let payload = &programdata_account.data[metadata_len..];

    if payload.len() != PHASE5B2_PROGRAMDATA_PAYLOAD_BYTES {
        return Err(phase5b2_error(
            "ProgramData payload capacity does not match the locked Phase 4E capacity",
        ));
    }

    if PHASE5B2_MEANINGFUL_PROGRAM_BYTES > payload.len() {
        return Err(phase5b2_error(
            "locked meaningful program size exceeds ProgramData payload",
        ));
    }

    let meaningful = &payload[..PHASE5B2_MEANINGFUL_PROGRAM_BYTES];

    let padding = &payload[PHASE5B2_MEANINGFUL_PROGRAM_BYTES..];

    if padding.len() != PHASE5B2_TRAILING_ZERO_PADDING_BYTES {
        return Err(phase5b2_error(
            "ProgramData trailing padding length does not match the locked Phase 4E value",
        ));
    }

    if !padding.iter().all(|byte| *byte == 0) {
        return Err(phase5b2_error(
            "ProgramData trailing padding contains non-zero bytes",
        ));
    }

    let digest = Sha256::digest(meaningful);

    let program_prefix_sha256 = format!("{digest:x}");

    if program_prefix_sha256 != PHASE5B2_PROGRAM_PREFIX_SHA256 {
        return Err(phase5b2_error(
            "deployed meaningful program binary hash does not match Phase 4E",
        ));
    }

    Ok(DeploymentObservation {
        source: source.to_string(),
        endpoint_class: endpoint_class.to_string(),
        observation_slot,
        programdata_slot,
        programdata_payload_bytes: payload.len(),
        meaningful_program_bytes: PHASE5B2_MEANINGFUL_PROGRAM_BYTES,
        trailing_zero_padding_bytes: padding.len(),
        program_prefix_sha256,
    })
}

fn require_matching_deployment_metadata(
    source1: &DeploymentObservation,
    source2: &DeploymentObservation,
) -> Result<(), CliError> {
    if source1.source == source2.source {
        return Err(phase5b2_error(
            "deployment metadata sources are not distinct",
        ));
    }

    if source1.endpoint_class == source2.endpoint_class {
        return Err(phase5b2_error(
            "deployment metadata endpoint classes are not distinct",
        ));
    }

    if source1.programdata_slot != source2.programdata_slot
        || source1.programdata_payload_bytes != source2.programdata_payload_bytes
        || source1.meaningful_program_bytes != source2.meaningful_program_bytes
        || source1.trailing_zero_padding_bytes != source2.trailing_zero_padding_bytes
        || source1.program_prefix_sha256 != source2.program_prefix_sha256
    {
        return Err(phase5b2_error(
            "independent providers disagree on deployment metadata",
        ));
    }

    Ok(())
}

fn load_fresh_quorum(path: &str) -> Result<FreshQuorum, CliError> {
    let bytes = fs::read(path).map_err(|error| {
        phase5b2_error(&format!(
            "could not read fresh Phase 5B1 quorum receipt: {error}"
        ))
    })?;

    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        phase5b2_error(&format!(
            "fresh Phase 5B1 quorum receipt is not valid JSON: {error}"
        ))
    })?;

    require_string(&value, "schema", "rox-anchor.phase5-read-only-quorum.v1")?;

    require_string(&value, "phase", "BUILD_PLAN4 Phase 5B")?;

    require_string(&value, "rpc_proof_decision", "Agreement")?;

    require_string(&value, "rpc_source_1", PHASE5B_SOURCE1_LABEL)?;

    require_string(&value, "rpc_source_2", PHASE5B_SOURCE2_LABEL)?;

    require_bool(&value, "rpc_sources_distinct", true)?;

    require_bool(&value, "state_readback_both_sources", true)?;

    require_bool(
        &value,
        "initialization_signature_multi_source_verified",
        true,
    )?;

    require_bool(&value, "phase5_closeout", false)?;

    require_bool(&value, "transaction_submission", false)?;

    require_bool(&value, "keypair_loading", false)?;

    require_bool(&value, "signing", false)?;

    require_bool(&value, "real_roc_mutation", false)?;

    let observation_count = require_u64(&value, "rpc_proof_observation_count")?;

    let required_observations = require_u64(&value, "rpc_proof_required_observations")?;

    if observation_count != u64::from(PHASE5_REQUIRED_OBSERVATIONS)
        || required_observations != u64::from(PHASE5_REQUIRED_OBSERVATIONS)
    {
        return Err(phase5b2_error(
            "fresh Phase 5B1 receipt does not contain the required 2-of-2 quorum",
        ));
    }

    Ok(FreshQuorum {
        source1_slot: require_u64(&value, "rpc_source_1_slot")?,
        source2_slot: require_u64(&value, "rpc_source_2_slot")?,
    })
}

fn parse_args(args: &[String]) -> Result<Phase5B2Args, CliError> {
    let mut parsed = Phase5B2Args::default();

    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                parsed.help = true;
                index += 1;
            }
            "--init-receipt" => {
                parsed.init_receipt = Some(require_value(args, index, "--init-receipt")?);

                index += 2;
            }
            "--receipt-out" => {
                parsed.receipt_out = Some(require_value(args, index, "--receipt-out")?);

                index += 2;
            }
            flag => {
                return Err(phase5b2_error(&format!("unknown flag `{flag}`")));
            }
        }
    }

    Ok(parsed)
}

fn require_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    let value = args
        .get(index + 1)
        .filter(|value| !value.starts_with('-'))
        .ok_or_else(|| phase5b2_error(&format!("{flag} requires a value")))?;

    Ok(value.clone())
}

fn parse_pubkey(value: &str, field: &str) -> Result<Pubkey, CliError> {
    Pubkey::from_str(value)
        .map_err(|_| phase5b2_error(&format!("locked {field} is not a valid public key")))
}

fn require_string(value: &Value, field: &str, expected: &str) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(phase5b2_error(&format!(
            "fresh quorum field `{field}` does not match expected value"
        )))
    }
}

fn require_bool(value: &Value, field: &str, expected: bool) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_bool) == Some(expected) {
        Ok(())
    } else {
        Err(phase5b2_error(&format!(
            "fresh quorum field `{field}` does not match expected boolean"
        )))
    }
}

fn require_u64(value: &Value, field: &str) -> Result<u64, CliError> {
    value.get(field).and_then(Value::as_u64).ok_or_else(|| {
        phase5b2_error(&format!(
            "fresh quorum field `{field}` is missing or not an integer"
        ))
    })
}

fn redact_signature(signature: &str) -> String {
    if signature.len() <= 14 {
        return "<redacted-signature>".to_string();
    }

    format!(
        "{}...{}",
        &signature[..10],
        &signature[signature.len() - 4..]
    )
}

fn help() -> String {
    [
        "BUILD_PLAN4 Phase 5B2 live read-only deployment metadata closeout",
        "",
        "usage:",
        "  rox-anchor pilot phase5-read-only-closeout \\",
        "    --init-receipt <phase4-receipt.json> \\",
        "    --receipt-out <phase5-closeout.json>",
        "",
        "behavior:",
        "  obtains a fresh fixed two-provider Phase 5B1 Agreement",
        "  validates loader-v3 Program to ProgramData pointer",
        "  validates ProgramData deployment slot and upgrade authority",
        "  validates meaningful deployed program SHA-256",
        "  validates ProgramData capacity and trailing zero padding",
        "  validates the final Phase 4E deployment signature through both providers",
        "  closes Phase 5 only after all checks agree",
        "",
        "fixed program data:",
        "  ProgramData: 4JsBSTEXLKtWusJQJAv1DnaRKfAxGnD958WhHPVz84UD",
        "  upgrade authority: DLQJ1icSQKu5CGsi7FqJgF9ohsiYuYuRkn23EggRDTdJ",
        "  deployment slot: 484017674",
        "  payload bytes: 398864",
        "  meaningful bytes: 392488",
        "  trailing zero padding bytes: 6376",
        "  SHA-256: 929f1906d497ed22c8e88c8a73bcaae0181271d9001fa1a98a9f8e3c50c45bf1",
        "",
        "security:",
        "  read-only RPC only",
        "  fixed Solana and Uniblock Devnet providers",
        "  no arbitrary RPC endpoint flags",
        "  no wallet or key loading",
        "  no signing",
        "  no simulation",
        "  no transaction submission",
        "  no ROX mint or burn",
        "  no internal ROC mutation",
        "  no settlement or finality claim",
        "  no mainnet",
    ]
    .join("\n")
}

fn phase5b2_error(message: &str) -> CliError {
    CliError::UnknownPilotFlag(format!("phase5-read-only-closeout {message}"))
}
