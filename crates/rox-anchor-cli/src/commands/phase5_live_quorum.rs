//! RO:WHAT — Collects two fixed independent BUILD_PLAN4 Phase 5B devnet
//! observations and requires agreement through rox-anchor-rpc-proof.
//! RO:WHY — Turns the proven Phase 5A single-source collector into genuine
//! two-provider evidence without creating a second quorum ruleset.
//! RO:INTERACTS — Phase 5A collector, Solana public Devnet, Uniblock Devnet,
//! Phase 4 initialization receipt, and rox-anchor-rpc-proof.
//! RO:INVARIANTS — source endpoints and labels are fixed and distinct;
//! both sources independently pass Phase 5A state validation; quorum is 2/2;
//! disagreement, stale evidence, or binding mismatch cannot close this step.
//! RO:SECURITY — read-only RPC only; no keys, signing, simulation,
//! transaction submission, mint, burn, ROC mutation, settlement, or mainnet.
//! RO:TEST — phase5_live_quorum_source.rs plus rpc-proof quorum tests.

#![forbid(unsafe_code)]

use std::fs;

use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    review_rpc_observations, ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation,
    RpcProofAuditRecord, RpcProofConfig, RpcQuorumDecision,
};
use serde_json::{json, Value};

use crate::{
    commands::phase5_live_read_only::{
        collect_single_source_evidence, load_and_validate_phase4_receipt, PHASE5_DEVNET_RPC_URL,
        PHASE5_OPERATION_ID, PHASE5_PROGRAM_ID, PHASE5_REQUIRED_OBSERVATIONS,
        PHASE5_STALE_AFTER_SLOTS, PHASE5_TEST_ONLY_MINT, PHASE5_TEST_ONLY_TOKEN_ACCOUNT,
    },
    CliError,
};

pub(super) const PHASE5B_SOURCE2_RPC_URL: &str =
    "https://api.uniblock.dev/uni/v1/json-rpc?chainId=solana-devnet";

pub(super) const PHASE5B_SOURCE1_LABEL: &str = "solana-public-devnet-primary";
pub(super) const PHASE5B_SOURCE2_LABEL: &str = "uniblock-devnet-secondary";

pub(super) const PHASE5B_SOURCE1_ENDPOINT_CLASS: &str = "explicit-official-devnet";
pub(super) const PHASE5B_SOURCE2_ENDPOINT_CLASS: &str = "independent-uniblock-devnet";

#[derive(Default)]
struct Phase5BArgs {
    init_receipt: Option<String>,
    receipt_out: Option<String>,
    help: bool,
}

#[derive(Clone, Copy)]
struct SourceReceipt {
    slot_after: u64,
}

pub fn run_phase5_live_quorum(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help());
    }

    let init_receipt = parsed
        .init_receipt
        .ok_or_else(|| phase5b_error("requires --init-receipt"))?;

    let receipt_out = parsed
        .receipt_out
        .ok_or_else(|| phase5b_error("requires --receipt-out"))?;

    let phase4 = load_and_validate_phase4_receipt(&init_receipt)?;

    let source1_receipt_path = format!("{receipt_out}.source1.json");
    let source2_receipt_path = format!("{receipt_out}.source2.json");

    collect_single_source_evidence(
        &phase4,
        &source1_receipt_path,
        PHASE5_DEVNET_RPC_URL,
        PHASE5B_SOURCE1_LABEL,
        PHASE5B_SOURCE1_ENDPOINT_CLASS,
    )?;

    collect_single_source_evidence(
        &phase4,
        &source2_receipt_path,
        PHASE5B_SOURCE2_RPC_URL,
        PHASE5B_SOURCE2_LABEL,
        PHASE5B_SOURCE2_ENDPOINT_CLASS,
    )?;

    let source1 = load_source_receipt(
        &source1_receipt_path,
        PHASE5B_SOURCE1_LABEL,
        PHASE5B_SOURCE1_ENDPOINT_CLASS,
    )?;

    let source2 = load_source_receipt(
        &source2_receipt_path,
        PHASE5B_SOURCE2_LABEL,
        PHASE5B_SOURCE2_ENDPOINT_CLASS,
    )?;

    let current_slot = source1.slot_after.max(source2.slot_after);

    let expected = expected_binding()?;

    let observations = [
        observation(
            PHASE5B_SOURCE1_LABEL,
            &phase4.transaction_signature,
            source1.slot_after,
        )?,
        observation(
            PHASE5B_SOURCE2_LABEL,
            &phase4.transaction_signature,
            source2.slot_after,
        )?,
    ];

    let review = review_rpc_observations(
        &observations,
        &expected,
        RpcProofConfig::new(PHASE5_REQUIRED_OBSERVATIONS, PHASE5_STALE_AFTER_SLOTS),
        current_slot,
    );

    if review.decision != RpcQuorumDecision::Agreement {
        return Err(phase5b_error(
            "two-source observations did not reach rpc-proof Agreement",
        ));
    }

    if review.accepted_observations != PHASE5_REQUIRED_OBSERVATIONS {
        return Err(phase5b_error(
            "two-source observations did not satisfy the required observation count",
        ));
    }

    if review.required_observations != PHASE5_REQUIRED_OBSERVATIONS {
        return Err(phase5b_error(
            "rpc-proof returned an unexpected required observation count",
        ));
    }

    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, current_slot);

    if !audit.is_safe_for_display() {
        return Err(phase5b_error(
            "two-source rpc-proof audit projection is not display-safe",
        ));
    }

    let source_slot_delta = source1.slot_after.abs_diff(source2.slot_after);

    let receipt = json!({
        "schema": "rox-anchor.phase5-read-only-quorum.v1",
        "phase": "BUILD_PLAN4 Phase 5B",
        "closeout_scope": "two_source_rpc_quorum_foundation",
        "cluster": "devnet",
        "program_id": PHASE5_PROGRAM_ID,
        "test_only_rox_mint": PHASE5_TEST_ONLY_MINT,
        "test_only_token_account": PHASE5_TEST_ONLY_TOKEN_ACCOUNT,
        "rpc_proof_operation_id": PHASE5_OPERATION_ID,
        "rpc_proof_minimum_commitment": "confirmed",
        "rpc_source_1": PHASE5B_SOURCE1_LABEL,
        "rpc_source_1_endpoint_class": PHASE5B_SOURCE1_ENDPOINT_CLASS,
        "rpc_source_1_slot": source1.slot_after,
        "rpc_source_2": PHASE5B_SOURCE2_LABEL,
        "rpc_source_2_endpoint_class": PHASE5B_SOURCE2_ENDPOINT_CLASS,
        "rpc_source_2_slot": source2.slot_after,
        "rpc_source_slot_delta": source_slot_delta,
        "rpc_sources_distinct": true,
        "rpc_proof_observation_count": review.accepted_observations,
        "rpc_proof_required_observations": review.required_observations,
        "rpc_proof_decision": "Agreement",
        "rpc_disputed_sources_count": 0,
        "under_quorum_rejected": false,
        "state_readback_both_sources": true,
        "initialization_signature_multi_source_verified": true,
        "upgrade_authority_multi_source_verified": false,
        "deployment_metadata_multi_source_verified": false,
        "deploy_signature_multi_source_verified": false,
        "phase5_closeout": false,
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
        "next_action": "VERIFY_PHASE5B_UPGRADE_AUTHORITY_AND_DEPLOYMENT_METADATA"
    });

    let bytes = serde_json::to_vec_pretty(&receipt).map_err(|error| {
        phase5b_error(&format!(
            "could not encode two-source quorum receipt: {error}"
        ))
    })?;

    fs::write(&receipt_out, [bytes.as_slice(), b"\n"].concat()).map_err(|error| {
        phase5b_error(&format!(
            "could not write two-source quorum receipt: {error}"
        ))
    })?;

    Ok([
        "phase5_live_quorum: two_source_agreement".to_string(),
        "phase: BUILD_PLAN4 Phase 5B".to_string(),
        "cluster: devnet".to_string(),
        format!("rpc_source_1: {PHASE5B_SOURCE1_LABEL}"),
        format!("rpc_source_1_slot: {}", source1.slot_after),
        format!("rpc_source_2: {PHASE5B_SOURCE2_LABEL}"),
        format!("rpc_source_2_slot: {}", source2.slot_after),
        format!("rpc_source_slot_delta: {source_slot_delta}"),
        "rpc_sources_distinct: true".to_string(),
        "rpc_proof_observation_count: 2".to_string(),
        "rpc_proof_required_observations: 2".to_string(),
        "rpc_proof_decision: Agreement".to_string(),
        "state_readback_both_sources: true".to_string(),
        "initialization_signature_multi_source_verified: true".to_string(),
        "upgrade_authority_multi_source_verified: false".to_string(),
        "deployment_metadata_multi_source_verified: false".to_string(),
        "deploy_signature_multi_source_verified: false".to_string(),
        "phase5_closeout: false".to_string(),
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
        "next_action: VERIFY_PHASE5B_UPGRADE_AUTHORITY_AND_DEPLOYMENT_METADATA".to_string(),
    ]
    .join("\n"))
}

fn expected_binding() -> Result<ExpectedRpcBinding, CliError> {
    Ok(ExpectedRpcBinding::new(
        ClusterId::new("devnet").map_err(|error| {
            phase5b_error(&format!("could not construct cluster binding: {error}"))
        })?,
        ProgramId::new(PHASE5_PROGRAM_ID).map_err(|error| {
            phase5b_error(&format!("could not construct program binding: {error}"))
        })?,
        MintId::new(PHASE5_TEST_ONLY_MINT).map_err(|error| {
            phase5b_error(&format!("could not construct mint binding: {error}"))
        })?,
        TokenAccountId::new(PHASE5_TEST_ONLY_TOKEN_ACCOUNT).map_err(|error| {
            phase5b_error(&format!(
                "could not construct token-account binding: {error}"
            ))
        })?,
        OperationId::new(PHASE5_OPERATION_ID).map_err(|error| {
            phase5b_error(&format!("could not construct operation binding: {error}"))
        })?,
        RpcCommitmentLevel::Confirmed,
    ))
}

fn observation(source: &str, signature: &str, slot: u64) -> Result<RpcObservation, CliError> {
    Ok(RpcObservation::new(
        source,
        ClusterId::new("devnet").map_err(|error| {
            phase5b_error(&format!("could not construct observation cluster: {error}"))
        })?,
        ProgramId::new(PHASE5_PROGRAM_ID).map_err(|error| {
            phase5b_error(&format!("could not construct observation program: {error}"))
        })?,
        MintId::new(PHASE5_TEST_ONLY_MINT).map_err(|error| {
            phase5b_error(&format!("could not construct observation mint: {error}"))
        })?,
        TokenAccountId::new(PHASE5_TEST_ONLY_TOKEN_ACCOUNT).map_err(|error| {
            phase5b_error(&format!(
                "could not construct observation token account: {error}"
            ))
        })?,
        OperationId::new(PHASE5_OPERATION_ID).map_err(|error| {
            phase5b_error(&format!(
                "could not construct observation operation: {error}"
            ))
        })?,
        signature,
        slot,
        RpcCommitmentLevel::Confirmed,
    ))
}

fn load_source_receipt(
    path: &str,
    expected_source: &str,
    expected_endpoint_class: &str,
) -> Result<SourceReceipt, CliError> {
    let bytes = fs::read(path).map_err(|error| {
        phase5b_error(&format!("could not read generated source receipt: {error}"))
    })?;

    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        phase5b_error(&format!(
            "generated source receipt is not valid JSON: {error}"
        ))
    })?;

    require_string(&value, "schema", "rox-anchor.phase5-read-only-source.v1")?;

    require_string(&value, "phase", "BUILD_PLAN4 Phase 5A")?;
    require_string(&value, "cluster", "devnet")?;
    require_string(&value, "rpc_source", expected_source)?;

    require_string(&value, "rpc_endpoint_class", expected_endpoint_class)?;

    require_string(
        &value,
        "initialization_signature_status",
        "confirmed_success",
    )?;

    require_string(&value, "rpc_proof_decision", "MissingEvidence")?;

    require_bool(&value, "program_executable", true)?;
    require_bool(&value, "test_only_mode", true)?;
    require_bool(&value, "halted", false)?;
    require_bool(&value, "recovery_required", false)?;
    require_bool(&value, "under_quorum_rejected", true)?;
    require_bool(&value, "phase5_closeout", false)?;
    require_bool(&value, "transaction_submission", false)?;
    require_bool(&value, "keypair_loading", false)?;
    require_bool(&value, "signing", false)?;
    require_bool(&value, "simulation", false)?;
    require_bool(&value, "rox_mint_performed", false)?;
    require_bool(&value, "rox_burn_performed", false)?;
    require_bool(&value, "real_roc_mutation", false)?;
    require_bool(&value, "production_settlement", false)?;
    require_bool(&value, "mainnet", false)?;

    let observation_count = value
        .get("rpc_proof_observation_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            phase5b_error("generated source receipt is missing rpc_proof_observation_count")
        })?;

    if observation_count != 1 {
        return Err(phase5b_error(
            "generated source receipt did not remain one-source evidence",
        ));
    }

    let required_observations = value
        .get("rpc_proof_required_observations")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            phase5b_error("generated source receipt is missing rpc_proof_required_observations")
        })?;

    if required_observations != u64::from(PHASE5_REQUIRED_OBSERVATIONS) {
        return Err(phase5b_error(
            "generated source receipt changed the required quorum",
        ));
    }

    let slot_after = value
        .get("slot_after")
        .and_then(Value::as_u64)
        .ok_or_else(|| phase5b_error("generated source receipt is missing slot_after"))?;

    Ok(SourceReceipt { slot_after })
}

fn parse_args(args: &[String]) -> Result<Phase5BArgs, CliError> {
    let mut parsed = Phase5BArgs::default();
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
                return Err(phase5b_error(&format!("unknown flag `{flag}`")));
            }
        }
    }

    Ok(parsed)
}

fn require_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    let value = args
        .get(index + 1)
        .filter(|value| !value.starts_with('-'))
        .ok_or_else(|| phase5b_error(&format!("{flag} requires a value")))?;

    Ok(value.clone())
}

fn require_string(value: &Value, field: &str, expected: &str) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(phase5b_error(&format!(
            "generated source receipt field `{field}` does not match expected value"
        )))
    }
}

fn require_bool(value: &Value, field: &str, expected: bool) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_bool) == Some(expected) {
        Ok(())
    } else {
        Err(phase5b_error(&format!(
            "generated source receipt field `{field}` does not match expected boolean"
        )))
    }
}

fn help() -> String {
    [
        "BUILD_PLAN4 Phase 5B live two-source read-only quorum",
        "",
        "usage:",
        "  rox-anchor pilot phase5-read-only-quorum \\",
        "    --init-receipt <ignored-phase4-receipt.json> \\",
        "    --receipt-out <ignored-phase5b-quorum-receipt.json>",
        "",
        "fixed source 1:",
        "  provider: Solana public Devnet",
        "  source: solana-public-devnet-primary",
        "  endpoint: https://api.devnet.solana.com",
        "",
        "fixed source 2:",
        "  provider: Uniblock Devnet",
        "  source: uniblock-devnet-secondary",
        "  endpoint: https://api.uniblock.dev/uni/v1/json-rpc?chainId=solana-devnet",
        "",
        "behavior:",
        "  independently validates the initialized state through both providers",
        "  feeds both observations into rox-anchor-rpc-proof",
        "  requires two distinct matching observations",
        "  requires rpc-proof Agreement",
        "  leaves Phase 5 open for ProgramData/upgrade-authority verification",
        "",
        "security:",
        "  read-only RPC only",
        "  no arbitrary RPC endpoint flags",
        "  no operator key loading",
        "  no signing",
        "  no simulation",
        "  no transaction submission",
        "  no ROX mint or burn",
        "  no internal ROC mutation",
        "  no settlement or finality claim",
    ]
    .join("\n")
}

fn phase5b_error(message: &str) -> CliError {
    CliError::UnknownPilotFlag(format!("phase5-read-only-quorum {message}"))
}
