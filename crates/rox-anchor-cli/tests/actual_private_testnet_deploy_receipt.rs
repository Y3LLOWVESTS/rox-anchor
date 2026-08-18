//! RO:WHAT — Tests BUILD_PLAN4 Phase 3 actual private devnet/testnet deploy receipt checker.
//! RO:WHY — Keeps manual deployment receipts redacted, devnet/testnet-only, and non-finality/non-settlement.
//! RO:INTERACTS — scripts/check_actual_private_testnet_deploy_receipt.sh and docs/pilot.
//! RO:INVARIANTS — receipts may evidence succeeded/failed attempts but never public launch, mainnet, finality, or real ROC mutation.
//! RO:SECURITY — no RPC, wallet load, signing, deployment, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_deploy_receipt.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
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
        .arg(root.join("scripts/check_actual_private_testnet_deploy_receipt.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("deploy receipt checker should execute");

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

    let dir = std::env::temp_dir().join(format!("rox-anchor-deploy-receipt-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-private-testnet-deploy-receipt.v1",
  "phase": "BUILD_PLAN4 Phase 3",
  "receipt_role": "private_testnet_deployment_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "deployment_outcome": "succeeded",
  "deploy_signature": "redacted-signature-1111111111111111111111111111111111111111111111111111111111111111",
  "deploy_slot": "123456",
  "program_binary_sha256": "{HASH_A}",
  "idl_sha256": "{HASH_B}",
  "build_manifest_path": "<redacted-local-build-manifest>",
  "payer_redacted": "<redacted-external-payer>",
  "deploy_authority_redacted": "<redacted-external-deploy-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "failure_reason_redacted": "not_applicable",
  "deploy_command_was_manual": true,
  "preflight_passed": true,
  "program_account_readback_verified": false,
  "idl_account_readback_verified": false,
  "deployment_success_claim_scope": "private_devnet_testnet_only",
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false
}}"#
    );

    for (from, to) in overrides {
        receipt = receipt.replace(from, to);
    }

    fs::write(path, receipt).expect("receipt fixture should be written");
}

#[test]
fn actual_deploy_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 3 deployment documentation checks passed"));
    assert!(output.contains("actual private devnet/testnet deployment runbook exists"));
    assert!(output.contains("manual-only, external-key-only, non-mainnet boundaries"));
    assert!(output.contains(
        "separates deployment receipt evidence from readback/finality/settlement evidence"
    ));
}

#[test]
fn actual_deploy_preflight_is_read_only_and_requires_anchor_outputs() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass after anchor build:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 3 deployment preflight passed"));
    assert!(output.contains("this preflight did not deploy, submit, call RPC, sign, mint, burn, settle, mutate ROC, or load a wallet"));
}

#[test]
fn actual_deploy_templates_are_redacted_and_non_authorizing() {
    for command in ["--template-success", "--template-failure"] {
        let (ok, output) = run_script(&[command, "testnet"]);

        assert!(ok, "{command} should print:\n{output}");
        assert!(output.contains("rox-anchor.actual-private-testnet-deploy-receipt.v1"));
        assert!(output.contains("BUILD_PLAN4 Phase 3"));
        assert!(output.contains("private_testnet_deployment_receipt"));
        assert!(output.contains("<redacted-external-payer>"));
        assert!(output.contains("<redacted-external-deploy-authority>"));
        assert!(output.contains(r#""program_account_readback_verified": false"#));
        assert!(output.contains(r#""idl_account_readback_verified": false"#));
        assert!(output.contains(r#""finality_claim": false"#));
        assert!(output.contains(r#""runtime_authority": false"#));
        assert!(output.contains(r#""public_launch_authorized": false"#));
        assert!(output.contains(r#""mainnet_authorized": false"#));
        assert!(output.contains(r#""production_bridge_settlement": false"#));
        assert!(output.contains(r#""public_rox_mint_burn": false"#));
        assert!(output.contains(r#""real_roc_mutation": false"#));

        assert!(!output.contains("/Users/"));
        assert!(!output.contains("/home/"));
        assert!(!output.contains("api-key="));
        assert!(!output.contains("access_token="));
        assert!(!output.contains(r#""finality_claim": true"#));
        assert!(!output.contains(r#""public_launch_authorized": true"#));
        assert!(!output.contains(r#""mainnet_authorized": true"#));
    }
}

#[test]
fn actual_deploy_receipt_accepts_redacted_success_shape() {
    let dir = unique_temp_dir("success");
    let receipt = dir.join("success.json");
    write_receipt(&receipt, &[]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "success receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 3 deploy receipt checks passed"));
    assert!(output.contains("receipt deployment_outcome = succeeded"));
    assert!(output.contains("deployment_success_claim_scope = private_devnet_testnet_only"));
    assert!(output.contains("receipt does not claim readback, finality, runtime authority, public launch, mainnet, production settlement, public ROX mint/burn, or real ROC mutation"));
}

#[test]
fn actual_deploy_receipt_accepts_redacted_failed_shape() {
    let dir = unique_temp_dir("failed");
    let receipt = dir.join("failed.json");
    write_receipt(
        &receipt,
        &[
            (
                r#""deployment_outcome": "succeeded""#,
                r#""deployment_outcome": "failed""#,
            ),
            (
                r#""deploy_signature": "redacted-signature-1111111111111111111111111111111111111111111111111111111111111111""#,
                r#""deploy_signature": "none""#,
            ),
            (r#""deploy_slot": "123456""#, r#""deploy_slot": "none""#),
            (
                r#""failure_reason_redacted": "not_applicable""#,
                r#""failure_reason_redacted": "airdrop_unavailable_redacted""#,
            ),
            (
                r#""deployment_success_claim_scope": "private_devnet_testnet_only""#,
                r#""deployment_success_claim_scope": "none""#,
            ),
        ],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "failed receipt should pass:\n{output}");
    assert!(output.contains("receipt deployment_outcome = failed"));
    assert!(output.contains("deployment_success_claim_scope = none"));
}

#[test]
fn actual_deploy_receipt_rejects_mainnet_cluster() {
    let dir = unique_temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet receipt should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_deploy_receipt_rejects_unredacted_paths_or_provider_tokens() {
    let dir = unique_temp_dir("secrets");
    let receipt = dir.join("secrets.json");
    write_receipt(
        &receipt,
        &[(
            r#""failure_reason_redacted": "not_applicable""#,
            r#""failure_reason_redacted": "failed at /Users/operator/.config/solana/payer.json api-key=abcdef1234567890""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unredacted receipt should fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn actual_deploy_receipt_rejects_forbidden_runtime_claims() {
    for (label, forbidden) in [
        (
            "finality",
            (r#""finality_claim": false"#, r#""finality_claim": true"#),
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
            "roc",
            (
                r#""real_roc_mutation": false"#,
                r#""real_roc_mutation": true"#,
            ),
        ),
    ] {
        let dir = unique_temp_dir(label);
        let receipt = dir.join("forbidden.json");
        write_receipt(&receipt, &[forbidden]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_deploy_receipt_rejects_placeholder_hashes() {
    let dir = unique_temp_dir("placeholder-hash");
    let receipt = dir.join("placeholder.json");
    write_receipt(&receipt, &[(HASH_A, "<sha256>")]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "placeholder hash receipt should fail:\n{output}");
    assert!(output.contains("program_binary_sha256 must be 64 lowercase hex characters"));
}
