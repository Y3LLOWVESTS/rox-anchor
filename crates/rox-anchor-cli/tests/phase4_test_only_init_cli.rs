//! Focused BUILD_PLAN4 Phase 4 CLI preparation tests.

#![forbid(unsafe_code)]

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use rox_anchor_cli::{
    commands::test_only_init::{PHASE4_OPERATOR_APPROVAL, PHASE4_PROGRAM_ID},
    run_from_args, CliError,
};

static TEMP_DIR_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn temp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();

    let sequence = TEMP_DIR_SEQUENCE.fetch_add(1, Ordering::Relaxed);

    let process_id = std::process::id();

    let dir = std::env::temp_dir().join(format!(
        "rox-anchor-phase4-cli-{process_id}-{timestamp}-{sequence}"
    ));

    fs::create_dir_all(&dir).expect("temporary directory should be created");

    dir
}

fn config_text(cluster: &str, max_supply: u64, max_operation: u64) -> String {
    format!(
        r#"
cluster = "{cluster}"
program_id = "{program_id}"

payer_keypair_path = ".rox-anchor-private-pilot/devnet-payer.json"
program_config_keypair_path = ".rox-anchor-private-pilot/devnet-config.json"
test_only_mint_keypair_path = ".rox-anchor-private-pilot/devnet-test-only-mint.json"
halt_authority_path = ".rox-anchor-private-pilot/devnet-halt-authority.json"
recovery_authority_path = ".rox-anchor-private-pilot/devnet-recovery-authority.json"
upgrade_authority_path = ".rox-anchor-private-pilot/devnet-upgrade-authority.json"

test_only_mint_label = "test-only-rox-private-devnet"
test_only_token_account_label = "test-only-rox-token-private-devnet"

max_supply_units = "{max_supply}"
max_amount_units_per_operation = "{max_operation}"
require_operator_approval = true
"#,
        program_id = PHASE4_PROGRAM_ID,
    )
}

fn write_config(dir: &Path, text: &str) -> PathBuf {
    let path = dir.join("phase4-init.local.toml");

    fs::write(&path, text).expect("temporary config should be written");

    path
}

fn prepare_args(config: &Path, receipt: &Path) -> Vec<String> {
    vec![
        "rox-anchor".to_string(),
        "pilot".to_string(),
        "initialize-test-only-mint".to_string(),
        "--config".to_string(),
        config.display().to_string(),
        "--receipt-out".to_string(),
        receipt.display().to_string(),
        "--operator-approval".to_string(),
        PHASE4_OPERATOR_APPROVAL.to_string(),
        "--prepare-only".to_string(),
    ]
}

#[test]
fn phase4_help_exposes_prepare_only_without_live_execution() {
    let output = run_from_args(["rox-anchor", "pilot", "initialize-test-only-mint", "--help"])
        .expect("Phase 4 help should render");

    assert!(output.contains("initialize-test-only-mint"));
    assert!(output.contains("--prepare-only"));
    assert!(output.contains("RPC calls"));
    assert!(output.contains("keypair loading"));
    assert!(output.contains("transaction submission"));
}

#[test]
fn phase4_prepare_accepts_devnet_tiny_caps_and_redacts_paths() {
    let dir = temp_dir();
    let config = write_config(&dir, &config_text("devnet", 1_000, 10));
    let receipt = dir.join("phase4-init-receipt.json");

    let output = run_from_args(prepare_args(&config, &receipt))
        .expect("valid prepare-only config should pass");

    assert!(output.contains("phase4_test_only_initialization: prepare_only"));
    assert!(output.contains("cluster: devnet"));
    assert!(output.contains(&format!("program_id: {}", PHASE4_PROGRAM_ID)));
    assert!(output.contains("mint_authority_model: program_derived_pda"));
    assert!(output.contains("authority_artifact_paths: distinct"));
    assert!(output.contains("operator_approval: verified"));
    assert!(output.contains("transaction_submission: disabled"));
    assert!(output.contains("test_only_mint_creation: disabled"));
    assert!(output.contains("program_config_initialization: disabled"));
    assert!(output.contains("real_roc_mutation: disabled"));
    assert!(output.contains("mainnet_authorized: false"));

    let config_string = config.display().to_string();
    let receipt_string = receipt.display().to_string();

    assert!(!output.contains(&config_string));
    assert!(!output.contains(&receipt_string));
    assert!(output.contains("<redacted-phase4-config>"));
    assert!(output.contains("<redacted-phase4-receipt>"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn phase4_prepare_requires_exact_operator_approval() {
    let dir = temp_dir();
    let config = write_config(&dir, &config_text("devnet", 1_000, 10));
    let receipt = dir.join("phase4-init-receipt.json");

    let mut args = prepare_args(&config, &receipt);
    let approval_index = args
        .iter()
        .position(|value| value == PHASE4_OPERATOR_APPROVAL)
        .expect("approval argument should exist");

    args[approval_index] = "NOT_APPROVED".to_string();

    let error = run_from_args(args).expect_err("wrong approval must fail closed");

    assert!(matches!(error, CliError::UnknownPilotFlag(_)));
    assert!(error.to_string().contains("operator approval phrase"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn phase4_command_refuses_execution_without_explicit_mode() {
    let dir = temp_dir();
    let config = write_config(&dir, &config_text("devnet", 1_000, 10));
    let receipt = dir.join("phase4-init-receipt.json");

    let mut args = prepare_args(&config, &receipt);

    args.retain(|value| value != "--prepare-only");

    let error = run_from_args(args).expect_err("command without prepare-only must fail closed");

    assert!(error
        .to_string()
        .contains("requires exactly one explicit mode"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn phase4_prepare_rejects_mainnet_and_wrong_program() {
    let dir = temp_dir();

    let mainnet_config = write_config(&dir, &config_text("mainnet-beta", 1_000, 10));

    let receipt = dir.join("phase4-init-receipt.json");

    let error = run_from_args(prepare_args(&mainnet_config, &receipt))
        .expect_err("mainnet must fail closed");

    assert!(error
        .to_string()
        .contains("cluster must be devnet or testnet"));

    let wrong_program = config_text("devnet", 1_000, 10).replace(
        PHASE4_PROGRAM_ID,
        "WrongProgram111111111111111111111111111111",
    );

    let wrong_program_config = dir.join("wrong-program.local.toml");

    fs::write(&wrong_program_config, wrong_program)
        .expect("wrong program config should be written");

    let error = run_from_args(prepare_args(&wrong_program_config, &receipt))
        .expect_err("wrong program must fail closed");

    assert!(error
        .to_string()
        .contains("does not match the reviewed FiUY"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn phase4_prepare_rejects_over_cap_values() {
    let dir = temp_dir();
    let receipt = dir.join("phase4-init-receipt.json");

    let over_supply = write_config(&dir, &config_text("devnet", 1_000_001, 10));

    let error = run_from_args(prepare_args(&over_supply, &receipt))
        .expect_err("over-cap supply must reject");

    assert!(error.to_string().contains("max_supply_units"));

    let over_operation = dir.join("over-operation.local.toml");

    fs::write(&over_operation, config_text("devnet", 1_000, 11))
        .expect("over-operation config should be written");

    let error = run_from_args(prepare_args(&over_operation, &receipt))
        .expect_err("over-cap operation must reject");

    assert!(error.to_string().contains("max_amount_units_per_operation"));

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn pilot_help_lists_phase4_prepare_command() {
    let output =
        run_from_args(["rox-anchor", "pilot", "--help"]).expect("pilot help should render");

    assert!(output.contains("initialize-test-only-mint --prepare-only"));
    assert!(output.contains("no wallet/key loading"));
    assert!(output.contains("no silent RPC submission"));
}

#[test]
fn phase4_prepare_rejects_caps_below_onchain_policy() {
    let dir = temp_dir();
    let receipt = dir.join("phase4-init-receipt.json");

    let lower_supply = write_config(&dir, &config_text("devnet", 999, 10));

    let error = run_from_args(prepare_args(&lower_supply, &receipt))
        .expect_err("lower supply value must not misrepresent on-chain policy");

    assert!(error
        .to_string()
        .contains("on-chain private-pilot cap of 1000"));

    let lower_amount = dir.join("lower-amount.local.toml");

    fs::write(&lower_amount, config_text("devnet", 1_000, 9))
        .expect("lower amount config should be written");

    let error = run_from_args(prepare_args(&lower_amount, &receipt))
        .expect_err("lower per-operation value must not misrepresent on-chain policy");

    assert!(error
        .to_string()
        .contains("on-chain private-pilot cap of 10"));

    let _ = fs::remove_dir_all(dir);
}
