//! RO:WHAT — BUILD_PLAN4 Phase 4 preparation plus explicit non-submitting live simulation command.
//! RO:WHY — Gives operators a fail-closed CLI surface before live mint/config initialization.
//! RO:INTERACTS — local ignored Phase 4 config and pilot command dispatch.
//! RO:INVARIANTS — devnet/testnet only; FiUY program binding; tiny caps; test-only labels;
//! separated operator artifact paths; exact operator approval.
//! RO:SECURITY — prepare-only is inert; --simulate-live may read RPC and load/sign with local pilot keys but never submits, creates, mints, or initializes.
//! initialization, ROC mutation, settlement, or mainnet behavior.
//! RO:TEST — phase4_test_only_init_cli.rs.

#![forbid(unsafe_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use crate::CliError;

pub const PHASE4_OPERATOR_APPROVAL: &str = "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT";

pub const PHASE4_PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";

const MAX_ALLOWED_SUPPLY_UNITS: u64 = 1_000;
const MAX_ALLOWED_AMOUNT_PER_OPERATION: u64 = 10;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PrepareArgs {
    config: Option<String>,
    receipt_out: Option<String>,
    operator_approval: Option<String>,
    prepare_only: bool,
    simulate_live: bool,
    execute_live: bool,
    help: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Phase4InitConfig {
    pub(crate) cluster: String,
    pub(crate) program_id: String,
    pub(crate) payer_keypair_path: String,
    pub(crate) program_config_keypair_path: String,
    pub(crate) test_only_mint_keypair_path: String,
    pub(crate) halt_authority_path: String,
    pub(crate) recovery_authority_path: String,
    pub(crate) upgrade_authority_path: String,
    pub(crate) test_only_mint_label: String,
    pub(crate) test_only_token_account_label: String,
    pub(crate) max_supply_units: u64,
    pub(crate) max_amount_units_per_operation: u64,
    pub(crate) require_operator_approval: bool,
}

pub fn run_initialize_test_only_mint(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help_text());
    }

    let selected_mode_count = [
        parsed.prepare_only,
        parsed.simulate_live,
        parsed.execute_live,
    ]
    .into_iter()
    .filter(|selected| *selected)
    .count();

    if selected_mode_count == 0 {
        return Err(pilot_error(
            "requires exactly one explicit mode: --prepare-only, --simulate-live, or --execute-live",
        ));
    }

    if selected_mode_count != 1 {
        return Err(pilot_error(
            "--prepare-only, --simulate-live, and --execute-live are mutually exclusive",
        ));
    }

    let approval = parsed.operator_approval.as_deref().ok_or_else(|| {
        pilot_error("requires --operator-approval with the exact Phase 4 approval phrase")
    })?;

    if approval != PHASE4_OPERATOR_APPROVAL {
        return Err(pilot_error(
            "operator approval phrase does not match Phase 4 approval",
        ));
    }

    let config_path = parsed
        .config
        .as_deref()
        .ok_or_else(|| pilot_error("requires --config <local-ignored-config>"))?;

    let receipt_out = parsed
        .receipt_out
        .as_deref()
        .ok_or_else(|| pilot_error("requires --receipt-out <local-ignored-receipt-path>"))?;

    validate_local_artifact_path(config_path, "config", false)?;

    validate_local_artifact_path(receipt_out, "receipt-out", true)?;

    let input = fs::read_to_string(config_path)
        .map_err(|_| pilot_error("could not read the Phase 4 config file"))?;

    let config = Phase4InitConfig::parse(&input)?;
    config.validate()?;

    if parsed.prepare_only {
        return Ok(config.redacted_prepare_report());
    }

    if parsed.simulate_live {
        return crate::commands::phase4_live_executor::run_phase4_live_simulation(&config);
    }

    crate::commands::phase4_live_submit::run_phase4_live_submission(&config, receipt_out)
}

fn parse_args(args: &[String]) -> Result<PrepareArgs, CliError> {
    let mut parsed = PrepareArgs::default();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                parsed.help = true;
                index += 1;
            }
            "--prepare-only" => {
                if parsed.prepare_only {
                    return Err(pilot_error("duplicate --prepare-only flag"));
                }

                parsed.prepare_only = true;
                index += 1;
            }
            "--simulate-live" => {
                if parsed.simulate_live {
                    return Err(pilot_error("duplicate --simulate-live flag"));
                }

                parsed.simulate_live = true;
                index += 1;
            }
            "--execute-live" => {
                if parsed.execute_live {
                    return Err(pilot_error("duplicate --execute-live flag"));
                }

                parsed.execute_live = true;
                index += 1;
            }
            "--config" => {
                let value = value_after(args, index, "--config")?;

                set_once(&mut parsed.config, value, "--config")?;

                index += 2;
            }
            "--receipt-out" => {
                let value = value_after(args, index, "--receipt-out")?;

                set_once(&mut parsed.receipt_out, value, "--receipt-out")?;

                index += 2;
            }
            "--operator-approval" => {
                let value = value_after(args, index, "--operator-approval")?;

                set_once(&mut parsed.operator_approval, value, "--operator-approval")?;

                index += 2;
            }
            other => {
                return Err(pilot_error(&format!(
                    "unknown flag `{other}`; expected --config, --receipt-out, --operator-approval, --prepare-only, --simulate-live, --execute-live, or --help"
                )));
            }
        }
    }

    Ok(parsed)
}

fn value_after(args: &[String], index: usize, flag: &'static str) -> Result<String, CliError> {
    args.get(index + 1)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .ok_or_else(|| pilot_error(&format!("{flag} requires a value")))
}

fn set_once(
    target: &mut Option<String>,
    value: String,
    flag: &'static str,
) -> Result<(), CliError> {
    if target.is_some() {
        return Err(pilot_error(&format!("duplicate {flag} flag")));
    }

    *target = Some(value);
    Ok(())
}

impl Phase4InitConfig {
    fn parse(input: &str) -> Result<Self, CliError> {
        let pairs = parse_key_value_config(input)?;

        Ok(Self {
            cluster: required(&pairs, "cluster")?,
            program_id: required(&pairs, "program_id")?,
            payer_keypair_path: required(&pairs, "payer_keypair_path")?,
            program_config_keypair_path: required(&pairs, "program_config_keypair_path")?,
            test_only_mint_keypair_path: required(&pairs, "test_only_mint_keypair_path")?,
            halt_authority_path: required(&pairs, "halt_authority_path")?,
            recovery_authority_path: required(&pairs, "recovery_authority_path")?,
            upgrade_authority_path: required(&pairs, "upgrade_authority_path")?,
            test_only_mint_label: required(&pairs, "test_only_mint_label")?,
            test_only_token_account_label: required(&pairs, "test_only_token_account_label")?,
            max_supply_units: parse_u64(&pairs, "max_supply_units")?,
            max_amount_units_per_operation: parse_u64(&pairs, "max_amount_units_per_operation")?,
            require_operator_approval: parse_bool(&pairs, "require_operator_approval")?,
        })
    }

    fn validate(&self) -> Result<(), CliError> {
        match self.cluster.as_str() {
            "devnet" | "testnet" => {}
            _ => {
                return Err(pilot_error("cluster must be devnet or testnet"));
            }
        }

        if self.program_id != PHASE4_PROGRAM_ID {
            return Err(pilot_error(
                "program_id does not match the reviewed FiUY deployment",
            ));
        }

        validate_test_only_label(&self.test_only_mint_label, "test_only_mint_label")?;

        validate_test_only_label(
            &self.test_only_token_account_label,
            "test_only_token_account_label",
        )?;

        if self.max_supply_units != MAX_ALLOWED_SUPPLY_UNITS {
            return Err(pilot_error(
                "max_supply_units must equal the on-chain private-pilot cap of 1000",
            ));
        }

        if self.max_amount_units_per_operation != MAX_ALLOWED_AMOUNT_PER_OPERATION {
            return Err(pilot_error(
                "max_amount_units_per_operation must equal the on-chain private-pilot cap of 10",
            ));
        }

        if self.max_amount_units_per_operation > self.max_supply_units {
            return Err(pilot_error("per-operation cap cannot exceed supply cap"));
        }

        if !self.require_operator_approval {
            return Err(pilot_error("require_operator_approval must be true"));
        }

        let role_paths = [
            ("payer_keypair_path", self.payer_keypair_path.as_str()),
            (
                "program_config_keypair_path",
                self.program_config_keypair_path.as_str(),
            ),
            (
                "test_only_mint_keypair_path",
                self.test_only_mint_keypair_path.as_str(),
            ),
            ("halt_authority_path", self.halt_authority_path.as_str()),
            (
                "recovery_authority_path",
                self.recovery_authority_path.as_str(),
            ),
            (
                "upgrade_authority_path",
                self.upgrade_authority_path.as_str(),
            ),
        ];

        for (field, value) in role_paths {
            validate_local_artifact_path(value, field, true)?;
        }

        let unique: BTreeSet<&str> = role_paths.iter().map(|(_, value)| *value).collect();

        if unique.len() != role_paths.len() {
            return Err(pilot_error(
                "payer/config/mint/halt/recovery/upgrade artifact paths must be distinct",
            ));
        }

        Ok(())
    }

    fn redacted_prepare_report(&self) -> String {
        [
            "phase4_test_only_initialization: prepare_only".to_string(),
            "phase: BUILD_PLAN4 Phase 4".to_string(),
            format!("cluster: {}", self.cluster),
            format!("program_id: {}", self.program_id),
            "config_path: <redacted-phase4-config>".to_string(),
            "receipt_out: <redacted-phase4-receipt>".to_string(),
            format!("test_only_mint_label: {}", self.test_only_mint_label),
            format!(
                "test_only_token_account_label: {}",
                self.test_only_token_account_label
            ),
            format!("max_supply_units: {}", self.max_supply_units),
            format!(
                "max_amount_units_per_operation: {}",
                self.max_amount_units_per_operation
            ),
            "mint_authority_model: program_derived_pda".to_string(),
            "workflow_authority_model: payer_signer".to_string(),
            "halt_authority_model: separated_signer".to_string(),
            "recovery_authority_model: separated_signer".to_string(),
            "upgrade_authority_model: separated_external".to_string(),
            "authority_artifact_paths: distinct".to_string(),
            "authority_pubkey_separation: deferred_to_live_backend".to_string(),
            "operator_approval: verified".to_string(),
            "wallet_key_loading: disabled".to_string(),
            "rpc_calls: disabled".to_string(),
            "signing: disabled".to_string(),
            "transaction_submission: disabled".to_string(),
            "test_only_mint_creation: disabled".to_string(),
            "program_config_initialization: disabled".to_string(),
            "rox_mint_execution: disabled".to_string(),
            "rox_burn_execution: disabled".to_string(),
            "real_roc_mutation: disabled".to_string(),
            "production_settlement: disabled".to_string(),
            "public_launch_authorized: false".to_string(),
            "mainnet_authorized: false".to_string(),
            "prepare_gate: GREEN".to_string(),
        ]
        .join("\n")
    }
}

fn parse_key_value_config(input: &str) -> Result<BTreeMap<String, String>, CliError> {
    let mut pairs = BTreeMap::new();

    for raw_line in input.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(pilot_error("config contains a non key=value line"));
        };

        let key = key.trim();

        if key.is_empty() {
            return Err(pilot_error("config contains an empty key"));
        }

        let value = unquote(value.trim());

        if value.is_empty() {
            return Err(pilot_error("config contains an empty value"));
        }

        if pairs.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(pilot_error("config contains a duplicate key"));
        }
    }

    Ok(pairs)
}

fn unquote(value: &str) -> &str {
    if value.len() >= 2 {
        let bytes = value.as_bytes();

        if (bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\'')
        {
            return &value[1..value.len() - 1];
        }
    }

    value
}

fn required(pairs: &BTreeMap<String, String>, field: &'static str) -> Result<String, CliError> {
    pairs
        .get(field)
        .cloned()
        .ok_or_else(|| pilot_error(&format!("config is missing required field `{field}`")))
}

fn parse_u64(pairs: &BTreeMap<String, String>, field: &'static str) -> Result<u64, CliError> {
    required(pairs, field)?.parse::<u64>().map_err(|_| {
        pilot_error(&format!(
            "config field `{field}` must be an unsigned integer"
        ))
    })
}

fn parse_bool(pairs: &BTreeMap<String, String>, field: &'static str) -> Result<bool, CliError> {
    match required(pairs, field)?.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(pilot_error(&format!(
            "config field `{field}` must be true or false"
        ))),
    }
}

fn validate_test_only_label(value: &str, field: &'static str) -> Result<(), CliError> {
    let lower = value.to_ascii_lowercase();

    if !(lower.contains("test-only") || lower.contains("test_only")) {
        return Err(pilot_error(&format!(
            "{field} must explicitly contain test-only"
        )));
    }

    for forbidden in [
        "production",
        "mainnet",
        "public-mint",
        "public_mint",
        "public launch",
        "public-launch",
    ] {
        if lower.contains(forbidden) {
            return Err(pilot_error(&format!(
                "{field} contains forbidden production/public wording"
            )));
        }
    }

    Ok(())
}

fn validate_local_artifact_path(
    value: &str,
    field: &'static str,
    json_required: bool,
) -> Result<(), CliError> {
    let clean = value.trim();
    let lower = clean.to_ascii_lowercase();

    if clean.is_empty()
        || clean.contains('<')
        || clean.contains('>')
        || clean.contains('\n')
        || clean.contains('\r')
    {
        return Err(pilot_error(&format!(
            "{field} is not a concrete local artifact path"
        )));
    }

    if lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.contains("mainnet-beta")
    {
        return Err(pilot_error(&format!(
            "{field} must be a local devnet/testnet artifact path"
        )));
    }

    if json_required && !lower.ends_with(".json") {
        return Err(pilot_error(&format!(
            "{field} must reference a JSON artifact"
        )));
    }

    Ok(())
}

fn pilot_error(message: &str) -> CliError {
    CliError::UnknownPilotFlag(format!("initialize-test-only-mint {message}"))
}

fn help_text() -> String {
    [
        "rox-anchor pilot initialize-test-only-mint",
        "",
        "BUILD_PLAN4 Phase 4 test-only initialization preparation and live simulation.",
        "",
        "usage:",
        "  prepare only:",
        "    rox-anchor pilot initialize-test-only-mint \\",
        "      --config <ignored-local-config> \\",
        "      --receipt-out <ignored-local-receipt.json> \\",
        "      --operator-approval I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT \\",
        "      --prepare-only",
        "",
        "  explicit devnet/testnet simulation:",
        "    rox-anchor pilot initialize-test-only-mint \\",
        "      --config <ignored-local-config> \\",
        "      --receipt-out <ignored-local-receipt.json> \\",
        "      --operator-approval I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT \\",
        "      --simulate-live",
        "",
        "  explicit devnet/testnet submission:",
        "    rox-anchor pilot initialize-test-only-mint \\",
        "      --config <ignored-local-config> \\",
        "      --receipt-out <ignored-local-receipt.json> \\",
        "      --operator-approval I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT \\",
        "      --execute-live",
        "",
        "prepare-only guarantees:",
        "  validates devnet/testnet scope",
        "  validates FiUY program binding",
        "  validates test-only asset labels",
        "  validates exact 1000 / 10 private-pilot caps",
        "  validates distinct authority artifact paths",
        "  RPC calls: disabled in prepare-only",
        "  keypair loading: disabled in prepare-only",
        "",
        "simulate-live behavior:",
        "  loads only explicit ignored Phase 4 operator keypairs",
        "  performs devnet/testnet RPC preflight",
        "  requires program executable and config/mint/ATA absence",
        "  fetches rent and recent blockhash",
        "  builds the exact four-instruction atomic transaction",
        "  signs locally for simulation only",
        "  calls simulateTransaction without broadcasting",
        "",
        "execute-live behavior:",
        "  uses the same devnet/testnet bindings and preflight",
        "  requires config/mint/ATA absence before submission",
        "  simulates the exact signed transaction first",
        "  submits only after successful simulation",
        "  waits for confirmed transaction result",
        "  reads back config, mint, and payer ATA",
        "  requires zero initial ROX supply and zero payer ATA amount",
        "  writes a redacted local receipt only after verified readback",
        "",
        "always disabled outside explicit --execute-live:",
        "  transaction submission",
        "  test-only mint creation on ledger",
        "  program config initialization on ledger",
        "always disabled in every Phase 4 mode:",
        "  ROX mint/burn execution",
        "  internal ROC mutation",
        "  production settlement",
        "  mainnet/public behavior",
    ]
    .join("\n")
}
