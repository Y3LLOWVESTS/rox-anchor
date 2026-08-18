//! RO:WHAT — Tests BUILD_PLAN4 Phase 4 actual test-only mint readback receipt boundary.
//! RO:WHY — Keeps read-only RPC evidence separate from initialization, submission, public mint availability, and finality.
//! RO:INTERACTS — scripts/check_actual_test_only_mint_initialization.sh readback receipt validation.
//! RO:INVARIANTS — devnet/testnet only; read-only RPC only; test-only labels; tiny caps; redacted evidence; no public/mainnet/real ROC claims.
//! RO:SECURITY — no live RPC, wallet load, signing, initialization, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test actual_test_only_mint_readback.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";

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

    let dir = std::env::temp_dir().join(format!(
        "rox-anchor-actual-test-only-readback-{label}-{nanos}"
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_readback_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-test-only-mint-readback.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_readback_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "readback_outcome": "verified",
  "readback_slot": "123456",
  "program_config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "observed_test_only_mint_label": "test-only-rox-private-testnet",
  "observed_token_account_label": "test-only-rox-token-account-private-testnet",
  "observed_max_supply_units": "1000",
  "observed_max_amount_units_per_operation": "1",
  "observed_mint_authority_redacted": "<redacted-external-mint-authority>",
  "observed_halt_authority_redacted": "<redacted-external-halt-authority>",
  "observed_recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
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

    fs::write(path, receipt).expect("readback receipt fixture should be written");
}

#[test]
fn actual_test_only_mint_readback_template_is_read_only_and_redacted() {
    let (ok, output) = run_script(&["--template-readback", "testnet"]);

    assert!(ok, "readback template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-test-only-mint-readback.v1"));
    assert!(output.contains("test_only_mint_readback_receipt"));
    assert!(output.contains("test-only-rox-private-testnet"));
    assert!(output.contains("<redacted-read-only-rpc-evidence>"));
    assert!(output.contains(r#""read_only_rpc": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""public_mint_available": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
}

#[test]
fn actual_test_only_mint_readback_receipt_accepts_verified_read_only_shape() {
    let dir = unique_temp_dir("verified");
    let receipt = dir.join("readback.json");
    write_readback_receipt(&receipt, &[]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "readback receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 4 readback receipt checks passed"));
    assert!(output.contains("readback is explicitly read-only RPC evidence"));
    assert!(output.contains("observed labels remain test-only/private-testnet"));
    assert!(output.contains("observed caps remain tiny"));
}

#[test]
fn actual_test_only_mint_readback_receipt_rejects_mainnet_cluster() {
    let dir = unique_temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_readback_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet readback receipt should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_test_only_mint_readback_receipt_rejects_public_labels() {
    let dir = unique_temp_dir("public-label");
    let receipt = dir.join("public-label.json");
    write_readback_receipt(
        &receipt,
        &[(
            r#""observed_test_only_mint_label": "test-only-rox-private-testnet""#,
            r#""observed_test_only_mint_label": "public-rox-mainnet""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "public-label readback receipt should fail:\n{output}");
    assert!(output.contains("must stay test-only/private-testnet"));
}

#[test]
fn actual_test_only_mint_readback_receipt_rejects_transaction_submission_claim() {
    let dir = unique_temp_dir("submission");
    let receipt = dir.join("submission.json");
    write_readback_receipt(
        &receipt,
        &[(
            r#""transaction_submission": false"#,
            r#""transaction_submission": true"#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "submission readback receipt should fail:\n{output}");
    assert!(output.contains("forbidden true boolean"));
}

#[test]
fn actual_test_only_mint_readback_receipt_rejects_over_cap_observations() {
    for (field, replacement, expected) in [
        (
            r#""observed_max_supply_units": "1000""#,
            r#""observed_max_supply_units": "1000001""#,
            "observed_max_supply_units exceeds cap",
        ),
        (
            r#""observed_max_amount_units_per_operation": "1""#,
            r#""observed_max_amount_units_per_operation": "1001""#,
            "observed_max_amount_units_per_operation exceeds cap",
        ),
    ] {
        let dir = unique_temp_dir("over-cap");
        let receipt = dir.join("over-cap.json");
        write_readback_receipt(&receipt, &[(field, replacement)]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "over-cap readback receipt should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_test_only_mint_readback_receipt_rejects_unredacted_rpc_or_authority_material() {
    let dir = unique_temp_dir("secret-path");
    let receipt = dir.join("secret-path.json");
    write_readback_receipt(
        &receipt,
        &[(
            r#""rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>""#,
            r#""rpc_evidence_redacted": "/Users/operator/private/provider-token.txt""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unredacted readback receipt should fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn actual_test_only_mint_readback_receipt_rejects_public_or_finality_claims() {
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
        write_readback_receipt(&receipt, &[forbidden]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-readback-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} readback receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}
