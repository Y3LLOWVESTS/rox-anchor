//! RO:WHAT — Tests BUILD_PLAN4 Phase 4 actual test-only mint/config initialization receipt boundary.
//! RO:WHY — Keeps manual initialization receipts redacted, capped, test-only, and non-public.
//! RO:INTERACTS — scripts/check_actual_test_only_mint_initialization.sh and docs/pilot.
//! RO:INVARIANTS — devnet/testnet only; test-only labels; tiny caps; separated redacted authorities; no public/mainnet/finality/real ROC claims.
//! RO:SECURITY — no RPC, wallet load, signing, initialization, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_test_only_mint_initialization.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const PROGRAM_ID: &str = "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let output = Command::new("bash")
        .arg(root.join("scripts/check_actual_test_only_mint_initialization.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("test-only mint initialization checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after UNIX_EPOCH")
        .as_nanos();

    let dir =
        std::env::temp_dir().join(format!("rox-anchor-actual-test-only-init-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_init_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-test-only-mint-initialization.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_initialization_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "initialization_outcome": "succeeded",
  "operation_id": "actual-test-only-init-0001",
  "idempotency_key": "actual-test-only-init-idem-0001",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "program_config_account": "<redacted-program-config-account>",
  "max_supply_units": "1000",
  "max_amount_units_per_operation": "1",
  "mint_authority_redacted": "<redacted-external-mint-authority>",
  "halt_authority_redacted": "<redacted-external-halt-authority>",
  "recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "init_signature": "redacted-init-signature-111111111111111111111111111111111111111111111111111",
  "init_slot": "123456",
  "failure_reason_redacted": "not_applicable",
  "operator_approval": "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT",
  "manual_operator_action": true,
  "preflight_passed": true,
  "readback_required": true,
  "readback_verified": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}}"#
    );

    for (from, to) in overrides {
        receipt = receipt.replace(from, to);
    }

    fs::write(path, receipt).expect("initialization receipt fixture should be written");
}

#[test]
fn actual_test_only_mint_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output
        .contains("BUILD_PLAN4 Phase 4 test-only mint initialization documentation checks passed"));
    assert!(output.contains("actual test-only mint/config initialization runbook exists"));
    assert!(output.contains("test-only, tiny-cap, external-authority, non-mainnet boundaries"));
    assert!(output.contains("separates initialization attempt evidence from readback evidence"));
}

#[test]
fn actual_test_only_mint_preflight_is_read_only_and_requires_anchor_outputs() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass after anchor build:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 4 test-only mint initialization preflight passed"));
    assert!(output.contains("this preflight did not initialize, submit, call RPC, sign, mint, burn, settle, mutate ROC, or load a wallet"));
}

#[test]
fn actual_test_only_mint_templates_are_redacted_and_non_authorizing() {
    for command in [
        "--template-init-success",
        "--template-init-failure",
        "--template-readback",
    ] {
        let (ok, output) = run_script(&[command, "testnet"]);

        assert!(ok, "{command} should print:\n{output}");
        assert!(output.contains("BUILD_PLAN4 Phase 4"));
        assert!(output.contains("test-only-rox-private-testnet"));
        assert!(output.contains("<redacted-"));
        assert!(output.contains(r#""public_mint_available": false"#));
        assert!(output.contains(r#""public_launch_authorized": false"#));
        assert!(output.contains(r#""mainnet_authorized": false"#));
        assert!(output.contains(r#""production_bridge_settlement": false"#));
        assert!(output.contains(r#""public_rox_mint_burn": false"#));
        assert!(output.contains(r#""real_roc_mutation": false"#));
        assert!(output.contains(r#""finality_claim": false"#));

        assert!(!output.contains("/Users/"));
        assert!(!output.contains("/home/"));
        assert!(!output.contains("api-key="));
        assert!(!output.contains("access_token="));
        assert!(!output.contains("public-rox-mainnet"));
        assert!(!output.contains(r#""public_mint_available": true"#));
        assert!(!output.contains(r#""real_roc_mutation": true"#));
    }
}

#[test]
fn actual_test_only_mint_init_receipt_accepts_redacted_success_shape() {
    let dir = unique_temp_dir("success");
    let receipt = dir.join("success.json");
    write_init_receipt(&receipt, &[]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "success receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 4 initialization receipt checks passed"));
    assert!(output.contains("receipt initialization_outcome = succeeded"));
    assert!(output.contains("labels are test-only/private-testnet"));
    assert!(output.contains("supply and per-operation caps are tiny"));
}

#[test]
fn actual_test_only_mint_init_receipt_accepts_redacted_failed_shape() {
    let dir = unique_temp_dir("failed");
    let receipt = dir.join("failed.json");

    write_init_receipt(
        &receipt,
        &[
            (
                r#""initialization_outcome": "succeeded""#,
                r#""initialization_outcome": "failed""#,
            ),
            (
                r#""init_signature": "redacted-init-signature-111111111111111111111111111111111111111111111111111""#,
                r#""init_signature": "none""#,
            ),
            (r#""init_slot": "123456""#, r#""init_slot": "none""#),
            (
                r#""failure_reason_redacted": "not_applicable""#,
                r#""failure_reason_redacted": "airdrop_unavailable_redacted""#,
            ),
            (
                r#""readback_required": true"#,
                r#""readback_required": false"#,
            ),
        ],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "failed receipt should pass:\n{output}");
    assert!(output.contains("receipt initialization_outcome = failed"));
}

#[test]
fn actual_test_only_mint_init_receipt_rejects_mainnet_cluster() {
    let dir = unique_temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_init_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet receipt should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_test_only_mint_init_receipt_rejects_public_or_production_labels() {
    for label in ["public-rox-mainnet", "production-rox-release"] {
        let dir = unique_temp_dir("bad-label");
        let receipt = dir.join("bad-label.json");
        write_init_receipt(
            &receipt,
            &[(
                r#""test_only_mint_label": "test-only-rox-private-testnet""#,
                &format!(r#""test_only_mint_label": "{label}""#),
            )],
        );

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("must stay test-only/private-testnet"));
    }
}

#[test]
fn actual_test_only_mint_init_receipt_rejects_over_cap_supply_or_amount() {
    for (field, replacement, expected) in [
        (
            r#""max_supply_units": "1000""#,
            r#""max_supply_units": "1000001""#,
            "max_supply_units exceeds cap",
        ),
        (
            r#""max_amount_units_per_operation": "1""#,
            r#""max_amount_units_per_operation": "1001""#,
            "max_amount_units_per_operation exceeds cap",
        ),
    ] {
        let dir = unique_temp_dir("over-cap");
        let receipt = dir.join("over-cap.json");
        write_init_receipt(&receipt, &[(field, replacement)]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "over-cap receipt should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_test_only_mint_init_receipt_rejects_unredacted_authority_or_secret_path() {
    let dir = unique_temp_dir("secret-path");
    let receipt = dir.join("secret-path.json");
    write_init_receipt(
        &receipt,
        &[(
            r#""failure_reason_redacted": "not_applicable""#,
            r#""failure_reason_redacted": "operator path leaked /Users/operator/private/payer.json""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unredacted receipt should fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn actual_test_only_mint_init_receipt_rejects_forbidden_public_runtime_claims() {
    for (label, forbidden) in [
        (
            "public_mint",
            (
                r#""public_mint_available": false"#,
                r#""public_mint_available": true"#,
            ),
        ),
        (
            "public_launch",
            (
                r#""public_launch_authorized": false"#,
                r#""public_launch_authorized": true"#,
            ),
        ),
        (
            "mainnet",
            (
                r#""mainnet_authorized": false"#,
                r#""mainnet_authorized": true"#,
            ),
        ),
        (
            "real_roc",
            (
                r#""real_roc_mutation": false"#,
                r#""real_roc_mutation": true"#,
            ),
        ),
        (
            "finality",
            (r#""finality_claim": false"#, r#""finality_claim": true"#),
        ),
    ] {
        let dir = unique_temp_dir(label);
        let receipt = dir.join("forbidden.json");
        write_init_receipt(&receipt, &[forbidden]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-init-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}
