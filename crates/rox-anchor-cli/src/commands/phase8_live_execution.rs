//! RO:WHAT — Executes exactly one explicitly approved BUILD_PLAN4 Phase 8
//! private-Devnet ROX-to-ROC burn and performs mandatory readback/two-source
//! reconciliation.
//! RO:WHY — Phase 8A already proved the exact one-unit reverse transaction by
//! simulation. This module adds the separately approved live mutation boundary
//! without adding any real internal ROC release authority.
//! RO:INTERACTS — Phase 8A evidence, canonical Phase 6 bindings, ignored local
//! workflow keypair, Solana public Devnet, Uniblock Devnet, Anchor ROX burn.
//! RO:INVARIANTS — exact operation/idempotency/nonce; amount=1; one signer;
//! one transaction submission call; prestate=1/1/op-absent; poststate=0/0/
//! finalized-op; send receipt before readback; no automatic resend.
//! RO:SECURITY — test-only Devnet ROX burn only. No svc-wallet call, no
//! ron-ledger mutation, no real ROC release, no production settlement/mainnet.
//! RO:TEST — phase8_live_execution_source.rs plus existing actual reverse-flow
//! receipt/coordinator/relayer/RPC-proof tests.

#![forbid(unsafe_code)]

use std::{
    fs,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        account::Account,
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
use rox_anchor::{
    AnchorTransferDirection, OperationStateCode, RoxAnchorConfig, RoxAnchorOperation,
};
use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, PrivatePilotConfig, SubmissionMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::{
        phase5_live_quorum::PHASE5B_SOURCE2_RPC_URL,
        phase6_live_simulation::{
            PHASE4_INITIALIZATION_SIGNATURE, PHASE6_CONFIG_ACCOUNT, PHASE6_MINT_AUTHORITY,
            PHASE6_PROGRAM_ID, PHASE6_ROX_MINT, PHASE6_SOURCE1, PHASE6_SOURCE2,
            PHASE6_TOKEN_ACCOUNT, PHASE6_WORKFLOW_AUTHORITY,
        },
        phase8_rox_to_roc_simulation::{
            build_exact_instructions, PHASE8_AMOUNT_MINOR, PHASE8_BURN_EVIDENCE_LABEL,
            PHASE8_IDEMPOTENCY_KEY, PHASE8_NONCE, PHASE8_OPERATION_ID,
        },
    },
    CliError,
};

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

const PHASE8_OPERATOR_APPROVAL: &str = "I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN";

const PHASE8_EXPECTED_OPERATOR_LABEL: &str = "private-phase8-rox-to-roc-operator";

const PHASE8_EXPECTED_ASSET_LABEL: &str = "test-only-rox-private-devnet";

const PHASE8_AUTHORIZATION_AGE_LIMIT_SLOTS: u64 = 100;

#[derive(Debug, Default)]
struct Phase8LiveArgs {
    help: bool,
    execute_live_devnet_burn: bool,
    config: Option<String>,
    simulation_receipt: Option<String>,
    release_intent_receipt: Option<String>,
    send_receipt_out: Option<String>,
    readback_receipt_out: Option<String>,
    closeout_receipt_out: Option<String>,
    operator_approval: Option<String>,
    operation_id: Option<String>,
    idempotency_key: Option<String>,
    nonce: Option<String>,
    max_operations: Option<u64>,
    max_amount_minor: Option<u64>,
    retry_cap: Option<u64>,
}

fn phase8_live_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase8-execute-capped-rox-to-roc-burn {}",
        message.as_ref(),
    ))
}

fn parse_u64(raw: &str, flag: &str) -> Result<u64, CliError> {
    raw.parse::<u64>()
        .map_err(|_| phase8_live_error(format!("{flag} must be an unsigned integer",)))
}

fn take_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    args.get(index + 1)
        .cloned()
        .ok_or_else(|| phase8_live_error(format!("{flag} requires a value",)))
}

fn parse_args(args: &[String]) -> Result<Phase8LiveArgs, CliError> {
    let mut parsed = Phase8LiveArgs::default();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" | "help" => {
                parsed.help = true;
                index += 1;
            }

            "--execute-live-devnet-burn" => {
                parsed.execute_live_devnet_burn = true;
                index += 1;
            }

            "--config" => {
                parsed.config = Some(take_value(args, index, "--config")?);
                index += 2;
            }

            "--simulation-receipt" => {
                parsed.simulation_receipt = Some(take_value(args, index, "--simulation-receipt")?);
                index += 2;
            }

            "--release-intent-receipt" => {
                parsed.release_intent_receipt =
                    Some(take_value(args, index, "--release-intent-receipt")?);
                index += 2;
            }

            "--send-receipt-out" => {
                parsed.send_receipt_out = Some(take_value(args, index, "--send-receipt-out")?);
                index += 2;
            }

            "--readback-receipt-out" => {
                parsed.readback_receipt_out =
                    Some(take_value(args, index, "--readback-receipt-out")?);
                index += 2;
            }

            "--closeout-receipt-out" => {
                parsed.closeout_receipt_out =
                    Some(take_value(args, index, "--closeout-receipt-out")?);
                index += 2;
            }

            "--operator-approval" => {
                parsed.operator_approval = Some(take_value(args, index, "--operator-approval")?);
                index += 2;
            }

            "--operation-id" => {
                parsed.operation_id = Some(take_value(args, index, "--operation-id")?);
                index += 2;
            }

            "--idempotency-key" => {
                parsed.idempotency_key = Some(take_value(args, index, "--idempotency-key")?);
                index += 2;
            }

            "--nonce" => {
                parsed.nonce = Some(take_value(args, index, "--nonce")?);
                index += 2;
            }

            "--max-operations" => {
                let raw = take_value(args, index, "--max-operations")?;

                parsed.max_operations = Some(parse_u64(&raw, "--max-operations")?);

                index += 2;
            }

            "--max-amount-minor" => {
                let raw = take_value(args, index, "--max-amount-minor")?;

                parsed.max_amount_minor = Some(parse_u64(&raw, "--max-amount-minor")?);

                index += 2;
            }

            "--retry-cap" => {
                let raw = take_value(args, index, "--retry-cap")?;

                parsed.retry_cap = Some(parse_u64(&raw, "--retry-cap")?);

                index += 2;
            }

            other => {
                return Err(phase8_live_error(format!("unknown flag `{other}`",)));
            }
        }
    }

    Ok(parsed)
}

fn help_text() -> String {
    [
        "rox-anchor pilot phase8-execute-capped-rox-to-roc-burn",
        "",
        "BUILD_PLAN4 Phase 8 explicitly approved one-shot Devnet burn.",
        "",
        "required:",
        "  --config <private-testnet-config>",
        "  --simulation-receipt <fresh-phase8a-receipt>",
        "  --release-intent-receipt <fresh-dry-run-release-intent>",
        "  --send-receipt-out <new-send-receipt>",
        "  --readback-receipt-out <new-readback-receipt>",
        "  --closeout-receipt-out <new-two-source-closeout>",
        "  --operator-approval I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN",
        "  --operation-id actual-rox-to-roc-op-0001",
        "  --idempotency-key actual-rox-to-roc-idem-0001",
        "  --nonce actual-rox-to-roc-nonce-0001",
        "  --max-operations 1",
        "  --max-amount-minor 1",
        "  --retry-cap 1",
        "  --execute-live-devnet-burn",
        "",
        "live scope:",
        "  exactly one Devnet transaction",
        "  burns exactly one test-only ROX",
        "  no real internal ROC release",
        "  no svc-wallet/ron-ledger mutation",
        "  no production settlement",
        "  no mainnet",
        "",
        "no-rerun:",
        "  after live invocation, reconcile read-only on uncertainty",
        "  never blindly rerun the live command",
    ]
    .join("\n")
}

fn required_arg<'a>(value: &'a Option<String>, flag: &str) -> Result<&'a str, CliError> {
    value
        .as_deref()
        .ok_or_else(|| phase8_live_error(format!("{flag} is required",)))
}

fn validate_exact_scope(args: &Phase8LiveArgs) -> Result<(), CliError> {
    if !args.execute_live_devnet_burn {
        return Err(phase8_live_error(
            "requires explicit --execute-live-devnet-burn",
        ));
    }

    if required_arg(&args.operator_approval, "--operator-approval")? != PHASE8_OPERATOR_APPROVAL {
        return Err(phase8_live_error(
            "exact I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN approval is required",
        ));
    }

    if required_arg(&args.operation_id, "--operation-id")? != PHASE8_OPERATION_ID {
        return Err(phase8_live_error(
            "operation ID does not match the reviewed Phase 8 identity",
        ));
    }

    if required_arg(&args.idempotency_key, "--idempotency-key")? != PHASE8_IDEMPOTENCY_KEY {
        return Err(phase8_live_error(
            "idempotency key does not match the reviewed Phase 8 identity",
        ));
    }

    if required_arg(&args.nonce, "--nonce")? != PHASE8_NONCE {
        return Err(phase8_live_error(
            "nonce does not match the reviewed Phase 8 identity",
        ));
    }

    if args.max_operations != Some(1)
        || args.max_amount_minor != Some(PHASE8_AMOUNT_MINOR)
        || args.retry_cap != Some(1)
    {
        return Err(phase8_live_error(
            "live burn requires max_operations=1, max_amount_minor=1, retry_cap=1",
        ));
    }

    Ok(())
}

fn parse_pubkey(value: &str, label: &str) -> Result<Pubkey, CliError> {
    Pubkey::from_str(value)
        .map_err(|_| phase8_live_error(format!("{label} is not a valid Solana public key",)))
}

fn sha256_array(value: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(value);
    let mut output = [0_u8; 32];
    output.copy_from_slice(&digest);
    output
}

fn sha256_hex(value: &[u8]) -> String {
    let digest = Sha256::digest(value);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn file_sha256_hex(path: &Path) -> Result<String, CliError> {
    let bytes = fs::read(path).map_err(|error| {
        phase8_live_error(format!("could not read receipt for digest: {error}",))
    })?;

    Ok(sha256_hex(&bytes))
}

fn redacted_digest(label: &str, digest: &str) -> String {
    format!("<redacted-{label}-sha256:{digest}>")
}

fn normalize_output_path(path: &Path) -> Result<PathBuf, CliError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    let parent = parent.canonicalize().map_err(|error| {
        phase8_live_error(format!("could not canonicalize receipt parent: {error}",))
    })?;

    let file_name = path
        .file_name()
        .ok_or_else(|| phase8_live_error("receipt path has no file name"))?;

    Ok(parent.join(file_name))
}

fn validate_new_receipt_target(path: &Path, label: &str) -> Result<(), CliError> {
    let display = path.to_string_lossy();

    if !path.is_absolute() && !display.starts_with(".rox-anchor-private-pilot/") {
        return Err(phase8_live_error(format!(
            "{label} must be absolute or inside .rox-anchor-private-pilot",
        )));
    }

    if path.exists() {
        if label == "send receipt" {
            return Err(phase8_live_error(
                "SEND_RECEIPT_EXISTS_DO_NOT_RETRY; perform readback/reconciliation only",
            ));
        }

        return Err(phase8_live_error(format!(
            "{label} already exists; refusing overwrite",
        )));
    }

    let parent = path
        .parent()
        .ok_or_else(|| phase8_live_error(format!("{label} has no parent directory",)))?;

    if !parent.is_dir() {
        return Err(phase8_live_error(format!(
            "{label} parent directory does not exist",
        )));
    }

    Ok(())
}

fn write_new_json(path: &Path, value: &Value) -> Result<(), CliError> {
    let encoded = serde_json::to_string_pretty(value)
        .map_err(|error| phase8_live_error(format!("could not encode receipt: {error}",)))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| phase8_live_error(format!("could not create receipt: {error}",)))?;

    file.write_all(encoded.as_bytes())
        .map_err(|error| phase8_live_error(format!("could not write receipt: {error}",)))?;

    file.write_all(b"\n")
        .map_err(|error| phase8_live_error(format!("could not terminate receipt: {error}",)))?;

    file.sync_all()
        .map_err(|error| phase8_live_error(format!("could not sync receipt: {error}",)))
}

fn read_json(path: &Path, label: &str) -> Result<Value, CliError> {
    let text = fs::read_to_string(path)
        .map_err(|error| phase8_live_error(format!("could not read {label}: {error}",)))?;

    serde_json::from_str(&text)
        .map_err(|_| phase8_live_error(format!("{label} is not valid JSON",)))
}

fn require_json_string(
    value: &Value,
    field: &str,
    expected: &str,
    label: &str,
) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_str) != Some(expected) {
        return Err(phase8_live_error(format!(
            "{label} field `{field}` mismatch",
        )));
    }

    Ok(())
}

fn require_json_bool(
    value: &Value,
    field: &str,
    expected: bool,
    label: &str,
) -> Result<(), CliError> {
    if value.get(field).and_then(Value::as_bool) != Some(expected) {
        return Err(phase8_live_error(format!(
            "{label} field `{field}` must be {expected}",
        )));
    }

    Ok(())
}

fn validate_simulation_receipt(path: &Path) -> Result<u64, CliError> {
    let receipt = read_json(path, "Phase 8A simulation receipt")?;

    for (field, expected) in [
        ("schema", "rox-anchor.phase8-rox-to-roc-simulation.v1"),
        ("phase", "BUILD_PLAN4 Phase 8A"),
        ("cluster", "devnet"),
        ("direction", "rox_to_roc"),
        ("operation_id", PHASE8_OPERATION_ID),
        ("idempotency_key", PHASE8_IDEMPOTENCY_KEY),
        ("nonce", PHASE8_NONCE),
        ("amount_minor", "1"),
        ("pre_mint_supply_minor", "1"),
        ("pre_workflow_token_amount_minor", "1"),
        ("instruction_1", "observe_burn"),
        ("instruction_2", "finalize_rox_to_roc_burn"),
        ("two_source_read_only_state", "Agreement"),
        ("proof_review_status", "accepted"),
        ("coordinator_decision_status", "accepted"),
        ("relayer_dry_run_status", "accepted"),
        ("live_devnet_exact_candidate_simulation", "passed"),
        ("persistent_post_simulation_mint_supply_minor", "1"),
        (
            "persistent_post_simulation_workflow_token_amount_minor",
            "1",
        ),
        ("future_real_roc_path", "svc-wallet -> ron-ledger only"),
    ] {
        require_json_string(&receipt, field, expected, "Phase 8A simulation receipt")?;
    }

    if receipt.get("instruction_count").and_then(Value::as_u64) != Some(2) {
        return Err(phase8_live_error(
            "Phase 8A simulation receipt instruction_count must be 2",
        ));
    }

    for field in [
        "unsigned_transaction",
        "dry_run_internal_roc_release_intent",
        "config_bytes_unchanged",
        "mint_bytes_unchanged",
        "token_account_bytes_unchanged",
    ] {
        require_json_bool(&receipt, field, true, "Phase 8A simulation receipt")?;
    }

    for field in [
        "operation_persisted_after_simulation",
        "keypair_loading",
        "signature_generation",
        "transaction_submission",
        "persistent_rox_burn",
        "real_internal_roc_release",
        "svc_wallet_call",
        "ron_ledger_mutation",
        "production_settlement",
        "mainnet_authorized",
        "receipt_promotable_to_live_burn",
    ] {
        require_json_bool(&receipt, field, false, "Phase 8A simulation receipt")?;
    }

    receipt
        .get("simulation_context_slot")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            phase8_live_error("Phase 8A simulation receipt lacks numeric simulation_context_slot")
        })
}

fn validate_release_intent_receipt(path: &Path) -> Result<(), CliError> {
    let receipt = read_json(path, "Phase 8A release-intent receipt")?;

    for (field, expected) in [
        ("schema", "rox-anchor.phase8-internal-roc-release-intent.v1"),
        ("phase", "BUILD_PLAN4 Phase 8A"),
        ("cluster", "devnet"),
        ("direction", "rox_to_roc"),
        ("operation_id", PHASE8_OPERATION_ID),
        ("idempotency_key", PHASE8_IDEMPOTENCY_KEY),
        ("nonce", PHASE8_NONCE),
        ("test_amount_minor", "1"),
        ("future_real_roc_path", "svc-wallet -> ron-ledger only"),
    ] {
        require_json_string(&receipt, field, expected, "Phase 8A release-intent receipt")?;
    }

    for field in [
        "real_internal_roc_release",
        "svc_wallet_call",
        "ron_ledger_mutation",
        "paid_content_unlock",
        "settlement_claim",
    ] {
        require_json_bool(&receipt, field, false, "Phase 8A release-intent receipt")?;
    }

    Ok(())
}

fn validate_private_config(
    config: &PrivatePilotConfig,
    send_receipt_path: &Path,
) -> Result<(), CliError> {
    if config.testnet.environment_mode != AnchorEnvironmentMode::TestnetOnly {
        return Err(phase8_live_error("config environment must be testnet-only"));
    }

    if config.testnet.cluster != AnchorCluster::Devnet {
        return Err(phase8_live_error("config cluster must be Devnet"));
    }

    if config.testnet.submission_mode != SubmissionMode::TestnetSubmitCapped {
        return Err(phase8_live_error(
            "config submission mode must be testnet-submit-capped",
        ));
    }

    if config.testnet.rpc_url.as_str() != DEVNET_RPC_URL {
        return Err(phase8_live_error(
            "config RPC must be the reviewed Solana public Devnet endpoint",
        ));
    }

    if config.operator_label != PHASE8_EXPECTED_OPERATOR_LABEL {
        return Err(phase8_live_error("config operator label mismatch"));
    }

    if config.asset_label != PHASE8_EXPECTED_ASSET_LABEL {
        return Err(phase8_live_error("config asset label mismatch"));
    }

    if config
        .observed_signature
        .as_ref()
        .map(|signature| signature.as_str())
        != Some(PHASE4_INITIALIZATION_SIGNATURE)
    {
        return Err(phase8_live_error(
            "config deployment signature binding mismatch",
        ));
    }

    let configured = normalize_output_path(Path::new(config.receipt_output_path.as_str()))?;

    let requested = normalize_output_path(send_receipt_path)?;

    if configured != requested {
        return Err(phase8_live_error(
            "CLI send receipt path does not match the externally reviewed config receipt path",
        ));
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct CanonicalKeys {
    program: Pubkey,
    mint: Pubkey,
    token: Pubkey,
    workflow: Pubkey,
    mint_authority: Pubkey,
    accounts: [Pubkey; 4],
}

fn canonical_keys(operation: Pubkey) -> Result<CanonicalKeys, CliError> {
    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program")?;

    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config")?;

    let mint = parse_pubkey(PHASE6_ROX_MINT, "ROX mint")?;

    let token = parse_pubkey(PHASE6_TOKEN_ACCOUNT, "ROX token account")?;

    let workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;

    let mint_authority = parse_pubkey(PHASE6_MINT_AUTHORITY, "mint authority")?;

    Ok(CanonicalKeys {
        program,
        mint,
        token,
        workflow,
        mint_authority,
        accounts: [config, mint, token, operation],
    })
}

fn validate_account_set(
    accounts: &[Option<Account>],
    operation: Pubkey,
    expected_supply: u64,
    expected_token_amount: u64,
    require_finalized_operation: bool,
) -> Result<(), CliError> {
    if accounts.len() != 4 {
        return Err(phase8_live_error(
            "live state read returned unexpected account count",
        ));
    }

    let canonical = canonical_keys(operation)?;

    let config_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase8_live_error("ROX Anchor config account is missing"))?;

    if config_account.owner != canonical.program {
        return Err(phase8_live_error("ROX Anchor config owner mismatch"));
    }

    let mut config_bytes = config_account.data.as_slice();

    let config = RoxAnchorConfig::try_deserialize(&mut config_bytes)
        .map_err(|error| phase8_live_error(format!("ROX Anchor config decode failed: {error}",)))?;

    if config.authority != canonical.workflow
        || config.rox_mint != canonical.mint
        || config.mint_authority != canonical.mint_authority
        || !config.test_only_mode
        || config.max_supply_units != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS
        || config.max_amount_units_per_operation
            != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS
    {
        return Err(phase8_live_error(
            "ROX Anchor config binding/test-only policy mismatch",
        ));
    }

    if config.halted || config.recovery_required {
        return Err(phase8_live_error(
            "halt/recovery posture blocks Phase 8 live execution",
        ));
    }

    let mint_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase8_live_error("test-only ROX mint is missing"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase8_live_error("ROX mint token-program owner mismatch"));
    }

    let mint = Mint::unpack(&mint_account.data)
        .map_err(|error| phase8_live_error(format!("ROX mint decode failed: {error}",)))?;

    if mint.supply != expected_supply
        || mint.decimals != 0
        || mint.mint_authority != COption::Some(canonical.mint_authority)
        || mint.freeze_authority != COption::None
    {
        return Err(phase8_live_error(format!(
            "unexpected ROX mint state; expected supply={expected_supply}",
        )));
    }

    let token_account = accounts[2]
        .as_ref()
        .ok_or_else(|| phase8_live_error("workflow ROX token account is missing"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase8_live_error(
            "ROX token account program owner mismatch",
        ));
    }

    let token = SplTokenAccount::unpack(&token_account.data)
        .map_err(|error| phase8_live_error(format!("ROX token account decode failed: {error}",)))?;

    if token.mint != canonical.mint
        || token.owner != canonical.workflow
        || token.amount != expected_token_amount
    {
        return Err(phase8_live_error(format!(
            "unexpected workflow ROX token state; expected amount={expected_token_amount}",
        )));
    }

    if !require_finalized_operation {
        if accounts[3].is_some() {
            return Err(phase8_live_error(
                "fresh Phase 8 operation PDA already exists",
            ));
        }

        return Ok(());
    }

    let operation_account = accounts[3]
        .as_ref()
        .ok_or_else(|| phase8_live_error("Phase 8 operation PDA is missing after burn"))?;

    if operation_account.owner != canonical.program {
        return Err(phase8_live_error("Phase 8 operation owner mismatch"));
    }

    let mut operation_bytes = operation_account.data.as_slice();

    let operation_state = RoxAnchorOperation::try_deserialize(&mut operation_bytes)
        .map_err(|error| phase8_live_error(format!("Phase 8 operation decode failed: {error}",)))?;

    let expected_operation_hash = sha256_array(PHASE8_OPERATION_ID.as_bytes());

    let expected_burn_hash = sha256_array(PHASE8_BURN_EVIDENCE_LABEL.as_bytes());

    if operation_state.authority != canonical.workflow
        || operation_state.operation_id_hash != expected_operation_hash
        || operation_state.mint != canonical.mint
        || operation_state.token_account != canonical.token
        || operation_state.direction_code() != Some(AnchorTransferDirection::RoxToRoc)
        || operation_state.amount_atoms != PHASE8_AMOUNT_MINOR
        || operation_state.burn_evidence_hash != expected_burn_hash
    {
        return Err(phase8_live_error(
            "finalized Phase 8 operation binding mismatch",
        ));
    }

    if operation_state.state_code() != Some(OperationStateCode::Finalized) {
        return Err(phase8_live_error("Phase 8 operation is not finalized"));
    }

    if operation_state.challenge_open || operation_state.recovery_required {
        return Err(phase8_live_error(
            "Phase 8 finalized operation has unsafe challenge/recovery posture",
        ));
    }

    Ok(())
}

fn read_live_state(
    rpc: &RpcClient,
    operation: Pubkey,
    expected_supply: u64,
    expected_token_amount: u64,
    require_finalized_operation: bool,
) -> Result<(), CliError> {
    let canonical = canonical_keys(operation)?;

    let accounts = rpc
        .get_multiple_accounts(&canonical.accounts)
        .map_err(|error| phase8_live_error(format!("Devnet state read failed: {error}",)))?;

    validate_account_set(
        &accounts,
        operation,
        expected_supply,
        expected_token_amount,
        require_finalized_operation,
    )
}

fn two_source_post_send_closeout(
    operation: Pubkey,
    minimum_context_slot: u64,
) -> Result<(u64, u64), CliError> {
    let canonical = canonical_keys(operation)?;

    let source1 =
        RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), CommitmentConfig::confirmed());

    let source2 = RpcClient::new_with_commitment(
        PHASE5B_SOURCE2_RPC_URL.to_string(),
        CommitmentConfig::confirmed(),
    );

    let source1_response = super::phase5_wire_compat::get_multiple_accounts_with_context_compat(
        &source1,
        PHASE6_SOURCE1,
        &canonical.accounts,
        Some(minimum_context_slot),
    )
    .map_err(|error| phase8_live_error(format!("Solana Phase 8 closeout read failed: {error}",)))?;

    let source2_response = super::phase5_wire_compat::get_multiple_accounts_with_context_compat(
        &source2,
        PHASE6_SOURCE2,
        &canonical.accounts,
        Some(minimum_context_slot),
    )
    .map_err(|error| {
        phase8_live_error(format!("Uniblock Phase 8 closeout read failed: {error}",))
    })?;

    if source1_response.accounts != source2_response.accounts {
        return Err(phase8_live_error(
            "Solana and Uniblock disagree on Phase 8 finalized post-state",
        ));
    }

    validate_account_set(&source1_response.accounts, operation, 0, 0, true)?;

    if source1_response
        .context_slot
        .abs_diff(source2_response.context_slot)
        > PHASE8_AUTHORIZATION_AGE_LIMIT_SLOTS
    {
        return Err(phase8_live_error(
            "Phase 8 post-send provider context slots are too far apart",
        ));
    }

    Ok((source1_response.context_slot, source2_response.context_slot))
}

pub fn run_phase8_live_execution(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help_text());
    }

    validate_exact_scope(&parsed)?;

    let config_path = Path::new(required_arg(&parsed.config, "--config")?);

    let simulation_receipt_path = Path::new(required_arg(
        &parsed.simulation_receipt,
        "--simulation-receipt",
    )?);

    let release_intent_receipt_path = Path::new(required_arg(
        &parsed.release_intent_receipt,
        "--release-intent-receipt",
    )?);

    let send_receipt_path = Path::new(required_arg(
        &parsed.send_receipt_out,
        "--send-receipt-out",
    )?);

    let readback_receipt_path = Path::new(required_arg(
        &parsed.readback_receipt_out,
        "--readback-receipt-out",
    )?);

    let closeout_receipt_path = Path::new(required_arg(
        &parsed.closeout_receipt_out,
        "--closeout-receipt-out",
    )?);

    if send_receipt_path == readback_receipt_path
        || send_receipt_path == closeout_receipt_path
        || readback_receipt_path == closeout_receipt_path
    {
        return Err(phase8_live_error(
            "send/readback/closeout receipt paths must be distinct",
        ));
    }

    // This is the no-rerun marker. It is checked before any RPC/key access.
    validate_new_receipt_target(send_receipt_path, "send receipt")?;

    validate_new_receipt_target(readback_receipt_path, "readback receipt")?;

    validate_new_receipt_target(closeout_receipt_path, "closeout receipt")?;

    let config_text = fs::read_to_string(config_path).map_err(|error| {
        phase8_live_error(format!("could not read capped-submit config: {error}",))
    })?;

    let config = PrivatePilotConfig::parse_external_config(&config_text)
        .map_err(|error| phase8_live_error(format!("capped-submit config rejected: {error}",)))?;

    validate_private_config(&config, send_receipt_path)?;

    let simulation_context_slot = validate_simulation_receipt(simulation_receipt_path)?;

    validate_release_intent_receipt(release_intent_receipt_path)?;

    let rpc = RpcClient::new_with_commitment(
        config.testnet.rpc_url.as_str().to_owned(),
        CommitmentConfig::confirmed(),
    );

    let current_slot = rpc.get_slot().map_err(|error| {
        phase8_live_error(format!("could not read current Devnet slot: {error}",))
    })?;

    if current_slot < simulation_context_slot
        || current_slot - simulation_context_slot > PHASE8_AUTHORIZATION_AGE_LIMIT_SLOTS
    {
        return Err(phase8_live_error(format!(
            "Phase 8A simulation is stale; simulation_slot={simulation_context_slot}, current_slot={current_slot}, limit={PHASE8_AUTHORIZATION_AGE_LIMIT_SLOTS}",
        )));
    }

    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program")?;

    let config_key = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config")?;

    let operation_hash = sha256_array(PHASE8_OPERATION_ID.as_bytes());

    let (operation, _) = RoxAnchorOperation::derive_address(&program, &config_key, &operation_hash);

    // PHASE8_LIVE_PREKEY_PREFLIGHT
    // The exact 1/1 state and fresh operation identity are checked before the
    // ignored local keypair is ever opened.
    read_live_state(&rpc, operation, 1, 1, false)?;

    let workflow = read_keypair_file(config.testnet.payer_keypair_path.as_str())
        .map_err(|_| phase8_live_error("could not load configured workflow-authority keypair"))?;

    let expected_workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;

    if workflow.pubkey() != expected_workflow {
        return Err(phase8_live_error(
            "loaded keypair does not match reviewed workflow authority",
        ));
    }

    let instructions = build_exact_instructions(operation)?;

    if instructions.len() != 2 {
        return Err(phase8_live_error(
            "Phase 8 live candidate must contain exactly two instructions",
        ));
    }

    let blockhash = rpc.get_latest_blockhash().map_err(|error| {
        phase8_live_error(format!("could not fetch recent Devnet blockhash: {error}",))
    })?;

    let signers: [&dyn Signer; 1] = [&workflow];

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&workflow.pubkey()),
        &signers,
        blockhash,
    );

    if transaction.signatures.len() != 1 {
        return Err(phase8_live_error(
            "Phase 8 live transaction must have exactly one signature",
        ));
    }

    let prepared_signature = transaction
        .signatures
        .first()
        .ok_or_else(|| phase8_live_error("Phase 8 transaction has no signature"))?;

    if prepared_signature == &Signature::default() {
        return Err(phase8_live_error(
            "Phase 8 transaction contains a default signature",
        ));
    }

    // PHASE8_SIGNED_RESIMULATION
    let signed_simulation = rpc.simulate_transaction(&transaction).map_err(|error| {
        phase8_live_error(format!("signed Phase 8 simulation RPC failed: {error}",))
    })?;

    if let Some(error) = signed_simulation.value.err.as_ref() {
        return Err(phase8_live_error(format!(
            "signed Phase 8 simulation rejected: {error:?}",
        )));
    }

    // PHASE8_IMMEDIATE_PRESEND_PREFLIGHT
    // Signed simulation must still have persisted nothing and unrelated state
    // must not have drifted before the only send call.
    read_live_state(&rpc, operation, 1, 1, false)?;

    // PHASE8_SINGLE_SUBMISSION
    // THE ONLY TRANSACTION SUBMISSION CALL IN THE PHASE 8 LIVE EXECUTOR.
    let confirmed_signature = rpc
        .send_and_confirm_transaction(&transaction)
        .map_err(|error| {
            phase8_live_error(format!(
                "one-shot Phase 8 Devnet burn submission failed: {error}",
            ))
        })?;

    if &confirmed_signature != prepared_signature {
        return Err(phase8_live_error(
            "confirmed signature does not match exact prepared Phase 8 transaction",
        ));
    }

    let send_slot = rpc.get_slot().map_err(|error| {
        phase8_live_error(format!(
            "transaction confirmed but post-send slot read failed: {error}",
        ))
    })?;

    let signature_digest = sha256_hex(confirmed_signature.to_string().as_bytes());

    let release_intent_digest = file_sha256_hex(release_intent_receipt_path)?;

    let redacted_signature = redacted_digest("testnet-signature", &signature_digest);

    let redacted_release_intent =
        redacted_digest("dry-run-roc-release-intent", &release_intent_digest);

    let send_receipt = json!({
        "schema":
            "rox-anchor.actual-rox-to-roc-capped-send.v1",

        "phase":
            "BUILD_PLAN4 Phase 8",

        "receipt_role":
            "actual_rox_to_roc_capped_send_receipt",

        "cluster":
            "devnet",

        "direction":
            "rox_to_roc",

        "program_name":
            "rox_anchor",

        "program_id":
            PHASE6_PROGRAM_ID,

        "send_outcome":
            "sent",

        "operation_id":
            PHASE8_OPERATION_ID,

        "idempotency_key":
            PHASE8_IDEMPOTENCY_KEY,

        "nonce":
            PHASE8_NONCE,

        "test_only_rox_burn_evidence_id":
            PHASE8_BURN_EVIDENCE_LABEL,

        "test_only_rox_burn_only":
            true,

        "internal_roc_release_intent_only":
            true,

        "dry_run_release_intent_id":
            redacted_release_intent,

        "program_account":
            "<redacted-program-account>",

        "config_account":
            "<redacted-program-config-account>",

        "test_only_mint":
            "<redacted-test-only-mint>",

        "test_only_token_account":
            "<redacted-test-only-token-account>",

        "test_only_mint_label":
            "test-only-rox-private-testnet",

        "test_only_token_account_label":
            "test-only-rox-token-account-private-testnet",

        "amount_minor":
            "1",

        "max_amount_minor":
            "1",

        "max_operations":
            "1",

        "retry_cap":
            "1",

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
            PHASE8_OPERATOR_APPROVAL,

        "external_signer_used":
            true,

        "signer_path_redacted":
            "<redacted-external-signer-path>",

        "receipt_out_redacted":
            "<redacted-external-receipt-path>",

        "transaction_submission":
            true,

        "send_authorized":
            true,

        "signature_generated":
            true,

        "transaction_signature":
            redacted_signature,

        "send_slot":
            send_slot.to_string(),

        "test_only_rox_burn_delta_minor":
            "1",

        "expected_internal_roc_release_intent_minor":
            "1",

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

        "real_roc_release":
            false,

        "real_roc_mutation":
            false,

        "finality_claim":
            false
    });

    // PHASE8_SEND_RECEIPT_BEFORE_READBACK
    // Once this succeeds, any later error is a reconciliation problem only.
    write_new_json(send_receipt_path, &send_receipt)?;

    read_live_state(&rpc, operation, 0, 0, true)?;

    let readback_slot = rpc
        .get_slot()
        .map_err(|error| phase8_live_error(format!("post-burn readback slot failed: {error}",)))?;

    let send_receipt_digest = file_sha256_hex(send_receipt_path)?;

    let redacted_send_receipt = redacted_digest("send-receipt", &send_receipt_digest);

    let readback_receipt = json!({
        "schema":
            "rox-anchor.actual-rox-to-roc-readback.v1",

        "phase":
            "BUILD_PLAN4 Phase 8",

        "receipt_role":
            "actual_rox_to_roc_readback_receipt",

        "cluster":
            "devnet",

        "direction":
            "rox_to_roc",

        "program_name":
            "rox_anchor",

        "program_id":
            PHASE6_PROGRAM_ID,

        "readback_outcome":
            "verified",

        "operation_id":
            PHASE8_OPERATION_ID,

        "idempotency_key":
            PHASE8_IDEMPOTENCY_KEY,

        "nonce":
            PHASE8_NONCE,

        "transaction_signature":
            redacted_digest(
                "testnet-signature",
                &signature_digest,
            ),

        "send_receipt_id":
            redacted_send_receipt,

        "program_account":
            "<redacted-program-account>",

        "config_account":
            "<redacted-program-config-account>",

        "test_only_mint":
            "<redacted-test-only-mint>",

        "test_only_token_account":
            "<redacted-test-only-token-account>",

        "expected_test_only_rox_burn_delta_minor":
            "1",

        "observed_test_only_rox_burn_delta_minor":
            "1",

        "dry_run_release_intent_id":
            redacted_digest(
                "dry-run-roc-release-intent",
                &release_intent_digest,
            ),

        "expected_internal_roc_release_intent_minor":
            "1",

        "observed_internal_roc_release_intent_minor":
            "1",

        "rpc_evidence_redacted":
            "<redacted-read-only-rpc-evidence>",

        "readback_slot":
            readback_slot.to_string(),

        "read_only_rpc":
            true,

        "transaction_submission":
            false,

        "internal_roc_release_intent_only":
            true,

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

        "real_roc_release":
            false,

        "real_roc_mutation":
            false,

        "finality_claim":
            false
    });

    write_new_json(readback_receipt_path, &readback_receipt)?;

    let (source1_context_slot, source2_context_slot) =
        two_source_post_send_closeout(operation, send_slot)?;

    let simulation_digest = file_sha256_hex(simulation_receipt_path)?;

    let readback_digest = file_sha256_hex(readback_receipt_path)?;

    let closeout_receipt = json!({
        "schema":
            "rox-anchor.phase8-post-burn-closeout.v1",

        "phase":
            "BUILD_PLAN4 Phase 8",

        "receipt_role":
            "actual_rox_to_roc_post_burn_two_source_closeout",

        "cluster":
            "devnet",

        "direction":
            "rox_to_roc",

        "operation_id":
            PHASE8_OPERATION_ID,

        "idempotency_key":
            PHASE8_IDEMPOTENCY_KEY,

        "nonce":
            PHASE8_NONCE,

        "source_1":
            PHASE6_SOURCE1,

        "source_2":
            PHASE6_SOURCE2,

        "source_1_context_slot":
            source1_context_slot,

        "source_2_context_slot":
            source2_context_slot,

        "minimum_context_slot":
            send_slot,

        "two_source_account_bytes_agree":
            true,

        "mint_supply_minor":
            "0",

        "workflow_token_amount_minor":
            "0",

        "operation_pda_exists":
            true,

        "operation_binding_verified":
            true,

        "operation_state":
            "finalized",

        "challenge_open":
            false,

        "recovery_required":
            false,

        "test_only_rox_burn_delta_minor":
            "1",

        "dry_run_internal_roc_release_intent_minor":
            "1",

        "real_internal_roc_release":
            false,

        "svc_wallet_call":
            false,

        "ron_ledger_mutation":
            false,

        "production_settlement":
            false,

        "mainnet_authorized":
            false,

        "additional_transaction_submission":
            false,

        "replay_transaction_submitted":
            false,

        "read_only_rpc":
            true,

        "phase8_closeout":
            true,

        "simulation_receipt_sha256":
            simulation_digest,

        "release_intent_receipt_sha256":
            release_intent_digest,

        "send_receipt_sha256":
            send_receipt_digest,

        "readback_receipt_sha256":
            readback_digest,

        "transaction_signature_sha256":
            signature_digest
    });

    write_new_json(closeout_receipt_path, &closeout_receipt)?;

    Ok([
        "rox-anchor pilot".to_string(),
        "command: pilot phase8-execute-capped-rox-to-roc-burn".to_string(),
        "scope: BUILD_PLAN4 Phase 8 explicit private Devnet execution".to_string(),
        "unsafe_defaults: rejected".to_string(),
        "cluster: devnet".to_string(),
        "direction: rox_to_roc".to_string(),
        format!("operation_id: {PHASE8_OPERATION_ID}"),
        format!("idempotency_key: {PHASE8_IDEMPOTENCY_KEY}"),
        format!("nonce: {PHASE8_NONCE}"),
        "amount_minor: 1".to_string(),
        "max_amount_minor: 1".to_string(),
        "max_operations: 1".to_string(),
        "retry_cap: 1".to_string(),
        format!("authorization_age_limit_slots: {PHASE8_AUTHORIZATION_AGE_LIMIT_SLOTS}"),
        "phase8a_simulation: fresh_verified".to_string(),
        "dry_run_roc_release_intent: verified".to_string(),
        "workflow_keypair_loaded: true".to_string(),
        "exact_transaction_signed: true".to_string(),
        "signed_transaction_resimulated: true".to_string(),
        "transaction_submission: true".to_string(),
        "transaction_count: 1".to_string(),
        "test_only_rox_mint_delta: -1".to_string(),
        "test_only_rox_token_delta: -1".to_string(),
        "post_burn_mint_supply_minor: 0".to_string(),
        "post_burn_workflow_token_amount_minor: 0".to_string(),
        "operation_pda_persisted: true".to_string(),
        "operation_state: finalized".to_string(),
        "readback_verified: true".to_string(),
        "two_source_closeout: Agreement".to_string(),
        "send_receipt_persisted: true".to_string(),
        "readback_receipt_persisted: true".to_string(),
        "closeout_receipt_persisted: true".to_string(),
        "dry_run_roc_release_intent_only: true".to_string(),
        "real_internal_roc_release: false".to_string(),
        "svc_wallet_mutation: false".to_string(),
        "ron_ledger_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
        "phase8_reverse_execution: GREEN".to_string(),
        "next_action: VALIDATE_RECEIPTS_AND_PARK_BUILD_PLAN4_PHASE8".to_string(),
    ]
    .join("\n"))
}
