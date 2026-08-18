//! RO:WHAT — Tests BUILD_PLAN4 Phase 5 actual private testnet read-only RPC evidence receipt boundary.
//! RO:WHY — Keeps live readback evidence distinct from submission, signing, finality, settlement, and public mint availability.
//! RO:INTERACTS — scripts/check_actual_private_testnet_read_only_evidence.sh.
//! RO:INVARIANTS — devnet/testnet only; read-only RPC true; transaction submission false; redacted account/provider evidence; bounded quorum.
//! RO:SECURITY — no live RPC, wallet load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test actual_private_testnet_read_only_rpc.

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
        .arg(root.join("scripts/check_actual_private_testnet_read_only_evidence.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("read-only evidence checker should execute");

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
        "rox-anchor-actual-read-only-evidence-{label}-{nanos}"
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-private-testnet-read-only-evidence.v1",
  "phase": "BUILD_PLAN4 Phase 5",
  "receipt_role": "private_testnet_read_only_rpc_evidence_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "evidence_outcome": "verified",
  "current_slot": "1000",
  "program_account": "<redacted-program-account>",
  "program_account_status": "exists-executable",
  "program_account_slot": "1000",
  "config_account": "<redacted-program-config-account>",
  "config_account_status": "exists",
  "config_account_slot": "1000",
  "test_only_mint": "<redacted-test-only-mint>",
  "mint_account_status": "exists",
  "mint_account_slot": "1000",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "token_account_status": "exists",
  "token_account_slot": "1000",
  "deploy_signature_status": "confirmed",
  "initialization_signature_status": "confirmed",
  "rpc_sources_count": "2",
  "rpc_quorum_threshold": "2",
  "rpc_matching_sources_count": "2",
  "rpc_disputed_sources_count": "0",
  "max_observation_lag_slots": "150",
  "rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "wallet_loaded": false,
  "signature_generated": false,
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

    fs::write(path, receipt).expect("read-only evidence receipt fixture should be written");
}

#[test]
fn actual_private_testnet_read_only_template_is_redacted_and_non_submitting() {
    let (ok, output) = run_script(&["--template-verified", "testnet"]);

    assert!(ok, "verified template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-private-testnet-read-only-evidence.v1"));
    assert!(output.contains("private_testnet_read_only_rpc_evidence_receipt"));
    assert!(output.contains("<redacted-program-account>"));
    assert!(output.contains("<redacted-rpc-provider-labels>"));
    assert!(output.contains(r#""read_only_rpc": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""wallet_loaded": false"#));
    assert!(output.contains(r#""signature_generated": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
}

#[test]
fn actual_private_testnet_read_only_receipt_accepts_verified_quorum_shape() {
    let dir = unique_temp_dir("verified");
    let receipt = dir.join("verified.json");
    write_receipt(&receipt, &[]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "verified evidence receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 5 read-only RPC evidence receipt checks passed"));
    assert!(output.contains("receipt evidence_outcome = verified"));
    assert!(output.contains("verified evidence satisfies account and quorum requirements"));
}

#[test]
fn actual_private_testnet_read_only_receipt_accepts_failed_non_quorum_shape() {
    let dir = unique_temp_dir("failed");
    let receipt = dir.join("failed.json");
    write_receipt(
        &receipt,
        &[
            (
                r#""evidence_outcome": "verified""#,
                r#""evidence_outcome": "failed""#,
            ),
            (
                r#""program_account_status": "exists-executable""#,
                r#""program_account_status": "missing""#,
            ),
            (
                r#""config_account_status": "exists""#,
                r#""config_account_status": "not_checked""#,
            ),
            (
                r#""mint_account_status": "exists""#,
                r#""mint_account_status": "not_checked""#,
            ),
            (
                r#""token_account_status": "exists""#,
                r#""token_account_status": "not_checked""#,
            ),
            (
                r#""deploy_signature_status": "confirmed""#,
                r#""deploy_signature_status": "not_checked""#,
            ),
            (
                r#""initialization_signature_status": "confirmed""#,
                r#""initialization_signature_status": "not_checked""#,
            ),
            (
                r#""rpc_matching_sources_count": "2""#,
                r#""rpc_matching_sources_count": "0""#,
            ),
            (
                r#""rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>""#,
                r#""failure_reason_redacted": "<redacted-safe-read-only-failure-reason>",
  "rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>""#,
            ),
        ],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "failed evidence receipt should pass:\n{output}");
    assert!(output.contains("receipt evidence_outcome = failed"));
    assert!(output.contains("failure_reason_redacted is redacted"));
}

#[test]
fn actual_private_testnet_read_only_receipt_rejects_mainnet_cluster() {
    let dir = unique_temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet evidence receipt should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_private_testnet_read_only_receipt_rejects_verified_without_quorum() {
    let dir = unique_temp_dir("no-quorum");
    let receipt = dir.join("no-quorum.json");
    write_receipt(
        &receipt,
        &[(
            r#""rpc_matching_sources_count": "2""#,
            r#""rpc_matching_sources_count": "1""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "verified without quorum should fail:\n{output}");
    assert!(output.contains("verified evidence requires matching sources to meet quorum threshold"));
}

#[test]
fn actual_private_testnet_read_only_receipt_rejects_verified_missing_program_or_mint() {
    for (field, replacement, expected) in [
        (
            r#""program_account_status": "exists-executable""#,
            r#""program_account_status": "missing""#,
            "verified evidence requires executable program account",
        ),
        (
            r#""mint_account_status": "exists""#,
            r#""mint_account_status": "missing""#,
            "verified evidence requires test-only mint account",
        ),
    ] {
        let dir = unique_temp_dir("missing-account");
        let receipt = dir.join("missing-account.json");
        write_receipt(&receipt, &[(field, replacement)]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "missing account receipt should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_private_testnet_read_only_receipt_rejects_submission_or_signing_claims() {
    for (label, forbidden) in [
        (
            "transaction_submission",
            (
                r#""transaction_submission": false"#,
                r#""transaction_submission": true"#,
            ),
        ),
        (
            "wallet_loaded",
            (r#""wallet_loaded": false"#, r#""wallet_loaded": true"#),
        ),
        (
            "signature_generated",
            (
                r#""signature_generated": false"#,
                r#""signature_generated": true"#,
            ),
        ),
    ] {
        let dir = unique_temp_dir(label);
        let receipt = dir.join("forbidden.json");
        write_receipt(&receipt, &[forbidden]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_private_testnet_read_only_receipt_rejects_public_or_finality_claims() {
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
        write_receipt(&receipt, &[forbidden]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_private_testnet_read_only_receipt_rejects_unredacted_rpc_or_paths() {
    let dir = unique_temp_dir("secret-path");
    let receipt = dir.join("secret-path.json");
    write_receipt(
        &receipt,
        &[(
            r#""rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>""#,
            r#""rpc_provider_labels_redacted": "/Users/operator/private/provider-token.txt""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-evidence-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(
        !ok,
        "unredacted read-only evidence receipt should fail:\n{output}"
    );
    assert!(output.contains("unredacted secret/path marker"));
}
