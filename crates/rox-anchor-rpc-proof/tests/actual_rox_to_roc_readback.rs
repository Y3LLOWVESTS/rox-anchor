//! RO:WHAT — Tests BUILD_PLAN4 Phase 8 ROX-to-ROC readback receipt boundary.
//! RO:WHY — Proves test-only ROX burn readback remains read-only and only produces dry-run ROC release intent.
//! RO:INTERACTS — scripts/check_actual_rox_to_roc_private_testnet_run.sh.
//! RO:INVARIANTS — read-only RPC only; observed burn delta must match dry-run release intent; no real ROC release.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, real ROC release, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test actual_rox_to_roc_readback.

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
        .arg(root.join("scripts/check_actual_rox_to_roc_private_testnet_run.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("ROX-to-ROC checker should execute");

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
        "rox-anchor-actual-rox-to-roc-readback-{label}-{nanos}"
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_readback_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-rox-to-roc-readback.v1",
  "phase": "BUILD_PLAN4 Phase 8",
  "receipt_role": "actual_rox_to_roc_readback_receipt",
  "cluster": "testnet",
  "direction": "rox_to_roc",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "readback_outcome": "verified",
  "operation_id": "actual-rox-to-roc-op-0001",
  "idempotency_key": "actual-rox-to-roc-idem-0001",
  "nonce": "actual-rox-to-roc-nonce-0001",
  "transaction_signature": "<redacted-testnet-signature>",
  "send_receipt_id": "<redacted-send-receipt-id>",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "expected_test_only_rox_burn_delta_minor": "1",
  "observed_test_only_rox_burn_delta_minor": "1",
  "dry_run_release_intent_id": "<redacted-dry-run-roc-release-intent-id>",
  "expected_internal_roc_release_intent_minor": "1",
  "observed_internal_roc_release_intent_minor": "1",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "internal_roc_release_intent_only": true,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_release": false,
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
fn actual_rox_to_roc_readback_template_is_read_only_and_redacted() {
    let (ok, output) = run_script(&["--template-readback", "testnet"]);

    assert!(ok, "readback template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-rox-to-roc-readback.v1"));
    assert!(output.contains("actual_rox_to_roc_readback_receipt"));
    assert!(output.contains("<redacted-read-only-rpc-evidence>"));
    assert!(output.contains(r#""read_only_rpc": true"#));
    assert!(output.contains(r#""internal_roc_release_intent_only": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""real_roc_release": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(!output.contains("/Users/"));
    assert!(!output.contains("api-key="));
}

#[test]
fn actual_rox_to_roc_readback_receipt_accepts_verified_delta() {
    let dir = temp_dir("verified");
    let receipt = dir.join("verified.json");
    write_readback_receipt(&receipt, &[]);

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-readback-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "readback receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 8 ROX-to-ROC readback receipt checks passed"));
    assert!(output.contains("readback burn and dry-run release-intent deltas match"));
}

#[test]
fn actual_rox_to_roc_readback_receipt_rejects_burn_or_release_mismatch() {
    for (label, from, to, expected) in [
        (
            "burn",
            r#""observed_test_only_rox_burn_delta_minor": "1""#,
            r#""observed_test_only_rox_burn_delta_minor": "2""#,
            "observed test-only ROX burn delta must match expected delta",
        ),
        (
            "release",
            r#""observed_internal_roc_release_intent_minor": "1""#,
            r#""observed_internal_roc_release_intent_minor": "2""#,
            "observed dry-run release intent must match expected amount",
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("mismatch.json");
        write_readback_receipt(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-readback-receipt", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} mismatch should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_rox_to_roc_readback_receipt_rejects_submission_or_real_roc_claim() {
    for (label, from, to) in [
        (
            "submission",
            r#""transaction_submission": false"#,
            r#""transaction_submission": true"#,
        ),
        (
            "real_release",
            r#""real_roc_release": false"#,
            r#""real_roc_release": true"#,
        ),
        (
            "real_mutation",
            r#""real_roc_mutation": false"#,
            r#""real_roc_mutation": true"#,
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
