//! RO:WHAT — Tests BUILD_PLAN4 Phase 7 ROC-to-ROX readback receipt boundary.
//! RO:WHY — Proves test-only ROX readback evidence remains read-only, redacted, and matched to expected delta.
//! RO:INTERACTS — scripts/check_actual_roc_to_rox_private_testnet_run.sh.
//! RO:INVARIANTS — read-only RPC only; observed test-only ROX delta must equal expected delta; no real ROC/finality claims.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test actual_roc_to_rox_readback.

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
        .expect("repo root should resolve")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let output = Command::new("bash")
        .arg(root.join("scripts/check_actual_roc_to_rox_private_testnet_run.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("ROC-to-ROX checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

fn temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should be valid")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "rox-anchor-actual-roc-to-rox-readback-{label}-{nanos}"
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_readback_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-roc-to-rox-readback.v1",
  "phase": "BUILD_PLAN4 Phase 7",
  "receipt_role": "actual_roc_to_rox_readback_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "readback_outcome": "verified",
  "operation_id": "actual-roc-to-rox-op-0001",
  "idempotency_key": "actual-roc-to-rox-idem-0001",
  "nonce": "actual-roc-to-rox-nonce-0001",
  "transaction_signature": "<redacted-testnet-signature>",
  "send_receipt_id": "<redacted-send-receipt-id>",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "expected_test_only_rox_delta_minor": "1",
  "observed_test_only_rox_delta_minor": "1",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}}"#
    );

    for (from, to) in overrides {
        receipt = receipt.replace(from, to);
    }

    fs::write(path, receipt).expect("readback receipt should be written");
}

#[test]
fn actual_roc_to_rox_readback_template_is_read_only_and_redacted() {
    let (ok, output) = run_script(&["--template-readback", "testnet"]);

    assert!(ok, "readback template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-roc-to-rox-readback.v1"));
    assert!(output.contains("actual_roc_to_rox_readback_receipt"));
    assert!(output.contains("<redacted-read-only-rpc-evidence>"));
    assert!(output.contains(r#""read_only_rpc": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(!output.contains("/Users/"));
    assert!(!output.contains("api-key="));
}

#[test]
fn actual_roc_to_rox_readback_receipt_accepts_verified_delta() {
    let dir = temp_dir("verified");
    let receipt = dir.join("verified.json");
    write_readback_receipt(&receipt, &[]);

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "readback receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 7 ROC-to-ROX readback receipt checks passed"));
    assert!(output.contains("readback delta matches expected delta"));
}

#[test]
fn actual_roc_to_rox_readback_receipt_rejects_delta_mismatch() {
    let dir = temp_dir("mismatch");
    let receipt = dir.join("mismatch.json");
    write_readback_receipt(
        &receipt,
        &[(
            r#""observed_test_only_rox_delta_minor": "1""#,
            r#""observed_test_only_rox_delta_minor": "2""#,
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "delta mismatch should fail:\n{output}");
    assert!(output.contains("observed test-only ROX delta must match expected delta"));
}

#[test]
fn actual_roc_to_rox_readback_receipt_rejects_submission_or_finality_claim() {
    for (label, from, to) in [
        (
            "submission",
            r#""transaction_submission": false"#,
            r#""transaction_submission": true"#,
        ),
        (
            "real_roc",
            r#""real_roc_mutation": false"#,
            r#""real_roc_mutation": true"#,
        ),
        (
            "finality",
            r#""finality_claim": false"#,
            r#""finality_claim": true"#,
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("forbidden.json");
        write_readback_receipt(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-readback-receipt", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_roc_to_rox_readback_receipt_rejects_mainnet_cluster() {
    let dir = temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_readback_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet readback should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}
