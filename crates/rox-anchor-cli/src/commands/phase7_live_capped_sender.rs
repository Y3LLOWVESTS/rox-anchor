//! RO:WHAT — Builds the exact BUILD_PLAN4 Phase 7 ROC-to-ROX capped-send
//! transaction candidate from the reviewed Devnet bindings.
//! RO:WHY — Phase 7 must simulate and later submit the same operation instead
//! of promoting or reconstructing the older Phase 6 simulation operation.
//! RO:INTERACTS — Phase 6 receipt evidence, external capped-submit config,
//! ROX Anchor observe/finalize instructions, and the classic SPL Token program.
//! RO:INVARIANTS — Devnet only; amount/retry/operation caps are exactly one;
//! Phase 6 evidence must be successful and non-promotable; the Phase 7
//! operation is distinct; the only required transaction signer is the reviewed
//! workflow authority.
//! RO:SECURITY — preparation only. This module reads configuration/evidence
//! files and constructs instructions. It does not load a keypair, contact RPC,
//! sign, simulate, submit, mint, mutate ROC, claim settlement, or claim finality.
//! RO:TEST — crate unit tests plus phase7_live_capped_sender_source.rs.

#![forbid(unsafe_code)]

use std::{collections::BTreeSet, fs, str::FromStr};

use anchor_lang::{
    solana_program::{instruction::Instruction, pubkey::Pubkey},
    InstructionData, ToAccountMetas,
};
use rox_anchor::{AnchorTransferDirection, OperationBindingArgs, RoxAnchorOperation};
use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, PrivatePilotConfig, SubmissionMode};
use serde_json::Value;
use sha2::{Digest, Sha256};
use solana_sdk_ids::system_program;

use crate::CliError;

pub(crate) const PHASE7_PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";
pub(crate) const PHASE7_CONFIG_ACCOUNT: &str = "4RBTypWtrn7mwV47MJkAHtEBMYnvNhd5wdSMAUsxwFeo";
pub(crate) const PHASE7_TEST_ONLY_ROX_MINT: &str = "HfHRJLswuRN3eVsiWnYi7REssDEsxxA8ewU8emhC3XA4";
pub(crate) const PHASE7_TEST_ONLY_TOKEN_ACCOUNT: &str =
    "A3sBYMUf2N7rpkqiCnE7fKZBdnGR5goH3hFmHJvgvqsJ";
pub(crate) const PHASE7_MINT_AUTHORITY_PDA: &str = "C5jTCy4EBY5fKuRMzLv7Lau5Re1SmMXukRXosndk9hJE";
pub(crate) const PHASE7_WORKFLOW_AUTHORITY: &str = "6YYJ43KRJF6pB3jUtRQpvhVHZQHaURTSxJdLpipHU3gs";

pub(crate) const PHASE7_OPERATION_ID: &str = "actual-roc-to-rox-op-0001";
pub(crate) const PHASE7_IDEMPOTENCY_KEY: &str = "actual-roc-to-rox-idem-0001";
pub(crate) const PHASE7_NONCE: &str = "actual-roc-to-rox-nonce-0001";
pub(crate) const PHASE7_SHADOW_ROC_BURN_INTENT_ID: &str = "shadow-roc-burn-intent-0001";
const PHASE7_BURN_EVIDENCE_DOMAIN: &str = "rox-anchor.phase7.shadow-roc-burn.v1";

pub(crate) const PHASE7_AMOUNT_MINOR: u64 = 1;
pub(crate) const PHASE7_MAX_AMOUNT_MINOR: u64 = 1;
pub(crate) const PHASE7_MAX_OPERATIONS: u16 = 1;
pub(crate) const PHASE7_RETRY_CAP: u8 = 1;

pub(crate) const PHASE7_OPERATOR_APPROVAL: &str = "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND";

const PHASE6_RECEIPT_SCHEMA: &str = "rox-anchor.actual-private-testnet-simulation.v1";
const PHASE6_RECEIPT_PHASE: &str = "BUILD_PLAN4 Phase 6";

#[derive(Clone, Debug, Default)]
struct Phase7PrepareArgs {
    config_path: Option<String>,
    phase6_receipt_path: Option<String>,
    operator_approval: Option<String>,
    max_operations: Option<u16>,
    max_amount_minor: Option<u64>,
    retry_cap: Option<u8>,
    prepare_only: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct Phase7CappedRocToRoxPlan {
    pub(crate) program_id: Pubkey,
    pub(crate) config: Pubkey,
    pub(crate) test_only_rox_mint: Pubkey,
    pub(crate) test_only_token_account: Pubkey,
    pub(crate) mint_authority: Pubkey,
    pub(crate) workflow_authority: Pubkey,
    pub(crate) operation: Pubkey,
    pub(crate) operation_id_hash: [u8; 32],
    pub(crate) burn_evidence_hash: [u8; 32],
    pub(crate) required_signers: BTreeSet<Pubkey>,
    pub(crate) instructions: Vec<Instruction>,
}

impl Phase7CappedRocToRoxPlan {
    pub(crate) fn is_exact_phase7_shape(&self) -> bool {
        let expected_signers = BTreeSet::from([self.workflow_authority]);

        self.program_id == rox_anchor::ID
            && self.config != Pubkey::default()
            && self.test_only_rox_mint != Pubkey::default()
            && self.test_only_token_account != Pubkey::default()
            && self.mint_authority != Pubkey::default()
            && self.operation != Pubkey::default()
            && self.operation_id_hash != [0_u8; 32]
            && self.burn_evidence_hash != [0_u8; 32]
            && self.instructions.len() == 2
            && self
                .instructions
                .iter()
                .all(|ix| ix.program_id == self.program_id)
            && self.required_signers == expected_signers
    }
}

pub fn run_phase7_prepare_capped_roc_to_rox(args: &[String]) -> Result<String, CliError> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        return Ok(phase7_help());
    }

    let parsed = parse_args(args)?;

    if !parsed.prepare_only {
        return Err(phase7_error(
            "--prepare-only is required; this Phase 7A command has no send path",
        ));
    }

    require_exact_caps(&parsed)?;

    let config_path = parsed
        .config_path
        .as_deref()
        .ok_or_else(|| phase7_error("--config is required"))?;

    let phase6_receipt_path = parsed
        .phase6_receipt_path
        .as_deref()
        .ok_or_else(|| phase7_error("--phase6-receipt is required"))?;

    require_local_or_external_artifact_path(config_path, "--config")?;
    require_local_or_external_artifact_path(phase6_receipt_path, "--phase6-receipt")?;

    if parsed.operator_approval.as_deref() != Some(PHASE7_OPERATOR_APPROVAL) {
        return Err(phase7_error(
            "exact operator approval I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND is required",
        ));
    }

    let config_text = fs::read_to_string(config_path)
        .map_err(|_| phase7_error("could not read external/ignored config"))?;

    let pilot_config = PrivatePilotConfig::parse_external_config(&config_text)
        .map_err(|error| phase7_error(format!("external config rejected: {error}")))?;

    validate_phase7_config(&pilot_config)?;

    let receipt_text = fs::read_to_string(phase6_receipt_path)
        .map_err(|_| phase7_error("could not read Phase 6 receipt"))?;

    let phase6_receipt: Value = serde_json::from_str(&receipt_text)
        .map_err(|_| phase7_error("Phase 6 receipt is not valid JSON"))?;

    validate_phase6_forward_receipt(&phase6_receipt)?;

    let plan = build_phase7_capped_roc_to_rox_plan()?;

    if !plan.is_exact_phase7_shape() {
        return Err(phase7_error(
            "constructed transaction does not satisfy exact Phase 7 shape",
        ));
    }

    Ok(render_prepare_report(&pilot_config, &plan))
}

pub(crate) fn build_phase7_capped_roc_to_rox_plan() -> Result<Phase7CappedRocToRoxPlan, CliError> {
    let program_id = parse_pubkey(PHASE7_PROGRAM_ID, "program id")?;
    let config = parse_pubkey(PHASE7_CONFIG_ACCOUNT, "config")?;
    let test_only_rox_mint = parse_pubkey(PHASE7_TEST_ONLY_ROX_MINT, "test-only ROX mint")?;
    let test_only_token_account =
        parse_pubkey(PHASE7_TEST_ONLY_TOKEN_ACCOUNT, "test-only token account")?;
    let mint_authority = parse_pubkey(PHASE7_MINT_AUTHORITY_PDA, "mint authority PDA")?;
    let workflow_authority = parse_pubkey(PHASE7_WORKFLOW_AUTHORITY, "workflow authority")?;

    if program_id != rox_anchor::ID {
        return Err(phase7_error(
            "reviewed Phase 7 program id does not match compiled ROX Anchor id",
        ));
    }

    let operation_id_hash = sha256(PHASE7_OPERATION_ID.as_bytes());
    let burn_evidence_hash = phase7_shadow_burn_evidence_hash();

    let (operation, _) =
        RoxAnchorOperation::derive_address(&program_id, &config, &operation_id_hash);

    let binding = OperationBindingArgs {
        operation_id_hash,
        direction: AnchorTransferDirection::RocToRox,
        mint: test_only_rox_mint,
        token_account: test_only_token_account,
        amount_atoms: PHASE7_AMOUNT_MINOR,
        burn_evidence_hash,
    };

    let observe_burn = Instruction {
        program_id,
        accounts: rox_anchor::accounts::ObserveBurn {
            config,
            operation,
            payer: workflow_authority,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
        data: rox_anchor::instruction::ObserveBurn { args: binding }.data(),
    };

    let finalize_roc_to_rox_mint = Instruction {
        program_id,
        accounts: rox_anchor::accounts::FinalizeRocToRoxMint {
            config,
            authority: workflow_authority,
            operation,
            rox_mint: test_only_rox_mint,
            recipient_rox_token_account: test_only_token_account,
            mint_authority,
            token_program: spl_token::id(),
        }
        .to_account_metas(None),
        data: rox_anchor::instruction::FinalizeRocToRoxMint {}.data(),
    };

    let instructions = vec![observe_burn, finalize_roc_to_rox_mint];

    let required_signers: BTreeSet<Pubkey> = instructions
        .iter()
        .flat_map(|instruction| instruction.accounts.iter())
        .filter(|account| account.is_signer)
        .map(|account| account.pubkey)
        .collect();

    let expected_signers = BTreeSet::from([workflow_authority]);

    if required_signers != expected_signers {
        return Err(phase7_error(
            "exact Phase 7 candidate produced unexpected signer requirements",
        ));
    }

    let plan = Phase7CappedRocToRoxPlan {
        program_id,
        config,
        test_only_rox_mint,
        test_only_token_account,
        mint_authority,
        workflow_authority,
        operation,
        operation_id_hash,
        burn_evidence_hash,
        required_signers,
        instructions,
    };

    if !plan.is_exact_phase7_shape() {
        return Err(phase7_error(
            "exact Phase 7 transaction candidate failed self-review",
        ));
    }

    Ok(plan)
}

fn phase7_shadow_burn_evidence_hash() -> [u8; 32] {
    let preimage = format!(
        "{PHASE7_BURN_EVIDENCE_DOMAIN}|operation_id={PHASE7_OPERATION_ID}|idempotency_key={PHASE7_IDEMPOTENCY_KEY}|nonce={PHASE7_NONCE}|intent_id={PHASE7_SHADOW_ROC_BURN_INTENT_ID}|amount_minor={PHASE7_AMOUNT_MINOR}"
    );

    sha256(preimage.as_bytes())
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(bytes);
    let mut output = [0_u8; 32];
    output.copy_from_slice(&digest);
    output
}

fn validate_phase7_config(config: &PrivatePilotConfig) -> Result<(), CliError> {
    config
        .testnet
        .validate()
        .map_err(|error| phase7_error(format!("testnet config rejected: {error}")))?;

    if config.testnet.environment_mode != AnchorEnvironmentMode::TestnetOnly {
        return Err(phase7_error("environment_mode must be testnet_only"));
    }

    if config.testnet.cluster != AnchorCluster::Devnet {
        return Err(phase7_error(
            "Phase 7 current deployment requires cluster=devnet",
        ));
    }

    if config.testnet.submission_mode != SubmissionMode::TestnetSubmitCapped {
        return Err(phase7_error(
            "submission_mode must be testnet_submit_capped",
        ));
    }

    Ok(())
}

pub(crate) fn validate_phase6_forward_receipt(receipt: &Value) -> Result<(), CliError> {
    require_string(receipt, "schema", PHASE6_RECEIPT_SCHEMA)?;
    require_string(receipt, "phase", PHASE6_RECEIPT_PHASE)?;
    require_string(
        receipt,
        "receipt_role",
        "actual_private_testnet_simulation_receipt",
    )?;
    require_string(receipt, "cluster", "devnet")?;
    require_string(receipt, "direction", "roc_to_rox")?;
    require_string(receipt, "program_name", "rox_anchor")?;
    require_string(receipt, "program_id", PHASE7_PROGRAM_ID)?;
    require_string(receipt, "simulation_outcome", "simulated")?;

    require_string(receipt, "amount_minor", "1")?;
    require_string(receipt, "max_amount_minor", "1")?;
    require_string(receipt, "max_operations", "1")?;

    require_string(receipt, "read_only_evidence_status", "verified")?;
    require_string(receipt, "proof_review_status", "accepted")?;
    require_string(receipt, "coordinator_decision_status", "accepted")?;
    require_string(receipt, "relayer_dry_run_status", "accepted")?;
    require_string(receipt, "simulation_result", "passed")?;

    require_bool(receipt, "read_only_evidence_required", true)?;
    require_bool(receipt, "read_only_evidence_verified", true)?;
    require_bool(receipt, "simulate_only", true)?;

    for field in [
        "transaction_submission",
        "send_authorized",
        "wallet_loaded",
        "signature_generated",
        "receipt_promotable_to_send",
        "public_mint_available",
        "public_launch_authorized",
        "mainnet_authorized",
        "production_bridge_settlement",
        "public_rox_mint_burn",
        "real_roc_mutation",
        "finality_claim",
    ] {
        require_bool(receipt, field, false)?;
    }

    let phase6_operation = require_nonempty_string(receipt, "operation_id")?;
    let phase6_idempotency = require_nonempty_string(receipt, "idempotency_key")?;
    let phase6_nonce = require_nonempty_string(receipt, "nonce")?;

    if phase6_operation == PHASE7_OPERATION_ID
        || phase6_idempotency == PHASE7_IDEMPOTENCY_KEY
        || phase6_nonce == PHASE7_NONCE
    {
        return Err(phase7_error(
            "Phase 7 must use a fresh operation/idempotency/nonce tuple distinct from Phase 6",
        ));
    }

    Ok(())
}

fn require_exact_caps(args: &Phase7PrepareArgs) -> Result<(), CliError> {
    if args.max_operations != Some(PHASE7_MAX_OPERATIONS) {
        return Err(phase7_error("--max-operations must be exactly 1"));
    }

    if args.max_amount_minor != Some(PHASE7_MAX_AMOUNT_MINOR) {
        return Err(phase7_error("--max-amount-minor must be exactly 1"));
    }

    if args.retry_cap != Some(PHASE7_RETRY_CAP) {
        return Err(phase7_error("--retry-cap must be exactly 1"));
    }

    Ok(())
}

fn require_local_or_external_artifact_path(value: &str, flag: &str) -> Result<(), CliError> {
    let clean = value.trim();

    if clean.is_empty() {
        return Err(phase7_error(format!("{flag} may not be empty")));
    }

    let ignored_local = clean.starts_with(".rox-anchor-private-pilot/");
    let external_absolute = clean.starts_with("/external/") || clean.contains("/external/");

    if !ignored_local && !external_absolute {
        return Err(phase7_error(format!(
            "{flag} must use the ignored .rox-anchor-private-pilot directory or an external artifact path"
        )));
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<Phase7PrepareArgs, CliError> {
    let mut parsed = Phase7PrepareArgs::default();
    let mut index = 0_usize;

    while index < args.len() {
        match args[index].as_str() {
            "--config" => {
                parsed.config_path = Some(next_value(args, index, "--config")?);
                index += 2;
            }
            "--phase6-receipt" => {
                parsed.phase6_receipt_path = Some(next_value(args, index, "--phase6-receipt")?);
                index += 2;
            }
            "--operator-approval" => {
                parsed.operator_approval = Some(next_value(args, index, "--operator-approval")?);
                index += 2;
            }
            "--max-operations" => {
                let value = next_value(args, index, "--max-operations")?;
                parsed.max_operations = Some(
                    value
                        .parse::<u16>()
                        .map_err(|_| phase7_error("--max-operations must be u16"))?,
                );
                index += 2;
            }
            "--max-amount-minor" => {
                let value = next_value(args, index, "--max-amount-minor")?;
                parsed.max_amount_minor = Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| phase7_error("--max-amount-minor must be u64"))?,
                );
                index += 2;
            }
            "--retry-cap" => {
                let value = next_value(args, index, "--retry-cap")?;
                parsed.retry_cap = Some(
                    value
                        .parse::<u8>()
                        .map_err(|_| phase7_error("--retry-cap must be u8"))?,
                );
                index += 2;
            }
            "--prepare-only" => {
                parsed.prepare_only = true;
                index += 1;
            }
            other => {
                return Err(phase7_error(format!("unknown argument `{other}`")));
            }
        }
    }

    Ok(parsed)
}

fn next_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| phase7_error(format!("{flag} requires a value")))
}

fn require_string(value: &Value, field: &str, expected: &str) -> Result<(), CliError> {
    let actual = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| phase7_error(format!("Phase 6 receipt missing string `{field}`")))?;

    if actual != expected {
        return Err(phase7_error(format!("Phase 6 receipt `{field}` mismatch")));
    }

    Ok(())
}

fn require_nonempty_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, CliError> {
    let actual = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| phase7_error(format!("Phase 6 receipt missing string `{field}`")))?;

    if actual.trim().is_empty() {
        return Err(phase7_error(format!(
            "Phase 6 receipt `{field}` may not be empty"
        )));
    }

    Ok(actual)
}

fn require_bool(value: &Value, field: &str, expected: bool) -> Result<(), CliError> {
    let actual = value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| phase7_error(format!("Phase 6 receipt missing bool `{field}`")))?;

    if actual != expected {
        return Err(phase7_error(format!("Phase 6 receipt `{field}` mismatch")));
    }

    Ok(())
}

fn parse_pubkey(value: &str, label: &str) -> Result<Pubkey, CliError> {
    Pubkey::from_str(value)
        .map_err(|_| phase7_error(format!("{label} is not a valid Solana public key")))
}

fn short_pubkey(value: &Pubkey) -> String {
    let text = value.to_string();

    format!("{}...{}", &text[..4], &text[text.len() - 4..],)
}

fn render_prepare_report(config: &PrivatePilotConfig, plan: &Phase7CappedRocToRoxPlan) -> String {
    [
        "phase7_exact_capped_roc_to_rox_candidate: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 7A".to_string(),
        "cluster: devnet".to_string(),
        "direction: roc_to_rox".to_string(),
        "candidate_mode: prepare_only".to_string(),
        "phase6_forward_receipt: verified_non_promotable".to_string(),
        "external_capped_submit_config: verified".to_string(),
        "operator_approval: exact_match".to_string(),
        format!(
            "rpc_url: {}",
            config.testnet.rpc_url.redacted()
        ),
        format!(
            "future_signer_path: {}",
            config.testnet.payer_keypair_path.redacted()
        ),
        format!(
            "future_receipt_output: {}",
            config.receipt_output_path.redacted()
        ),
        format!("program_id: {}", plan.program_id),
        format!("config_account: {}", plan.config),
        format!("test_only_rox_mint: {}", plan.test_only_rox_mint),
        format!(
            "test_only_token_account: {}",
            plan.test_only_token_account
        ),
        format!("mint_authority_pda: {}", plan.mint_authority),
        format!(
            "workflow_authority: {}",
            plan.workflow_authority
        ),
        format!(
            "operation_pda: {}",
            short_pubkey(&plan.operation)
        ),
        format!("operation_id: {PHASE7_OPERATION_ID}"),
        format!("idempotency_key: {PHASE7_IDEMPOTENCY_KEY}"),
        format!("nonce: {PHASE7_NONCE}"),
        format!(
            "shadow_roc_burn_intent_id: {PHASE7_SHADOW_ROC_BURN_INTENT_ID}"
        ),
        "shadow_roc_burn_only: true".to_string(),
        "burn_evidence_binding: operation_id+idempotency_key+nonce+intent+amount".to_string(),
        format!("amount_minor: {PHASE7_AMOUNT_MINOR}"),
        format!("max_amount_minor: {PHASE7_MAX_AMOUNT_MINOR}"),
        format!("max_operations: {PHASE7_MAX_OPERATIONS}"),
        format!("retry_cap: {PHASE7_RETRY_CAP}"),
        "instruction_count: 2".to_string(),
        "instruction_1: observe_burn".to_string(),
        "instruction_2: finalize_roc_to_rox_mint".to_string(),
        "atomic_transaction_candidate: true".to_string(),
        "required_signer_count: 1".to_string(),
        "required_signer_role: workflow_authority".to_string(),
        "phase7_operation_is_fresh_from_phase6: true".to_string(),
        "exact_candidate_built: true".to_string(),
        "exact_candidate_simulated: false".to_string(),
        "capped_sender_authorized: false".to_string(),
        "live_submission_permitted: false".to_string(),
        "rpc_calls: false".to_string(),
        "keypair_loading: false".to_string(),
        "signature_generated: false".to_string(),
        "transaction_submission: false".to_string(),
        "rox_mint_execution: false".to_string(),
        "real_roc_burn: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
        "finality_claim: false".to_string(),
        "next_action: PHASE7B_SIMULATE_EXACT_PHASE7_TRANSACTION_AND_BIND_EXISTING_SENDER_AUTHORIZATION".to_string(),
    ]
    .join("\n")
}

fn phase7_help() -> String {
    [
        "BUILD_PLAN4 Phase 7A exact capped ROC-to-ROX candidate",
        "",
        "usage:",
        "  rox-anchor pilot phase7-prepare-capped-roc-to-rox \\",
        "    --config <ignored-or-external-config> \\",
        "    --phase6-receipt <ignored-phase6-forward-receipt> \\",
        "    --operator-approval I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND \\",
        "    --max-operations 1 \\",
        "    --max-amount-minor 1 \\",
        "    --retry-cap 1 \\",
        "    --prepare-only",
        "",
        "behavior:",
        "  validates Phase 6 forward simulation evidence",
        "  validates explicit Devnet TestnetSubmitCapped config",
        "  requires the exact operator approval phrase",
        "  creates a fresh Phase 7 operation/idempotency/nonce tuple",
        "  builds observe_burn followed by finalize_roc_to_rox_mint",
        "  requires exactly one workflow-authority signer",
        "  performs no RPC call",
        "  loads no keypair",
        "  signs nothing",
        "  submits nothing",
        "  mints nothing during preparation",
        "  mutates no internal ROC",
        "",
        "Phase 7B must simulate this exact candidate before sender authorization.",
    ]
    .join("\n")
}

fn phase7_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase7-prepare-capped-roc-to-rox {}",
        message.as_ref()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase7_exact_candidate_is_two_instruction_atomic_shape() {
        let plan = build_phase7_capped_roc_to_rox_plan()
            .expect("static reviewed Phase 7 candidate should build");

        assert!(plan.is_exact_phase7_shape());
        assert_eq!(plan.instructions.len(), 2);
        assert_eq!(plan.instructions[0].program_id, rox_anchor::ID);
        assert_eq!(plan.instructions[1].program_id, rox_anchor::ID);
        assert_eq!(
            plan.required_signers,
            BTreeSet::from([plan.workflow_authority])
        );
        assert_ne!(plan.operation, Pubkey::default());
        assert_ne!(plan.operation_id_hash, [0_u8; 32]);
        assert_ne!(plan.burn_evidence_hash, [0_u8; 32]);
    }

    #[test]
    fn phase7_shadow_burn_hash_binds_all_phase7_identity_parts() {
        let expected_preimage = format!(
            "{PHASE7_BURN_EVIDENCE_DOMAIN}|operation_id={PHASE7_OPERATION_ID}|idempotency_key={PHASE7_IDEMPOTENCY_KEY}|nonce={PHASE7_NONCE}|intent_id={PHASE7_SHADOW_ROC_BURN_INTENT_ID}|amount_minor={PHASE7_AMOUNT_MINOR}"
        );

        assert_eq!(
            phase7_shadow_burn_evidence_hash(),
            sha256(expected_preimage.as_bytes())
        );
    }

    #[test]
    fn phase7_caps_are_exactly_one() {
        assert_eq!(PHASE7_AMOUNT_MINOR, 1);
        assert_eq!(PHASE7_MAX_AMOUNT_MINOR, 1);
        assert_eq!(PHASE7_MAX_OPERATIONS, 1);
        assert_eq!(PHASE7_RETRY_CAP, 1);
    }
}
