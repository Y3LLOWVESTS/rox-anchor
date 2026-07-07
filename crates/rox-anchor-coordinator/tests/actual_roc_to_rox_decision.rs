//! RO:WHAT — Tests BUILD_PLAN4 Phase 7 ROC-to-ROX coordinator decision receipt boundaries.
//! RO:WHY — Proves the forward capped-send receipt cannot bypass coordinator/proof/read-only/simulation gates.
//! RO:INTERACTS — scripts/check_actual_roc_to_rox_private_testnet_run.sh.
//! RO:INVARIANTS — coordinator accepted status is required for sent receipts; blocked receipts remain non-submitting.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test actual_roc_to_rox_decision.

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
        "rox-anchor-actual-roc-to-rox-decision-{label}-{nanos}"
    ));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_send_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-roc-to-rox-capped-send.v1",
  "phase": "BUILD_PLAN4 Phase 7",
  "receipt_role": "actual_roc_to_rox_capped_send_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "send_outcome": "sent",
  "operation_id": "actual-roc-to-rox-op-0001",
  "idempotency_key": "actual-roc-to-rox-idem-0001",
  "nonce": "actual-roc-to-rox-nonce-0001",
  "shadow_roc_burn_intent_id": "shadow-roc-burn-intent-0001",
  "shadow_roc_burn_only": true,
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "retry_cap": "1",
  "read_only_evidence_status": "verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_dry_run_status": "accepted",
  "simulation_result": "passed",
  "operator_approval": "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
  "external_signer_used": true,
  "signer_path_redacted": "<redacted-external-signer-path>",
  "receipt_out_redacted": "<redacted-external-receipt-path>",
  "transaction_submission": true,
  "send_authorized": true,
  "signature_generated": true,
  "transaction_signature": "<redacted-testnet-signature>",
  "send_slot": "123456",
  "test_only_rox_delta_minor": "1",
  "readback_required": true,
  "readback_verified": false,
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

    fs::write(path, receipt).expect("send receipt should be written");
}

#[test]
fn actual_roc_to_rox_decision_requires_coordinator_acceptance_for_sent_receipt() {
    let dir = temp_dir("coordinator-blocked");
    let receipt = dir.join("coordinator-blocked.json");
    write_send_receipt(
        &receipt,
        &[(
            r#""coordinator_decision_status": "accepted""#,
            r#""coordinator_decision_status": "blocked""#,
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "coordinator-blocked send should fail:\n{output}");
    assert!(output.contains("coordinator_decision_status expected 'accepted'"));
}

#[test]
fn actual_roc_to_rox_decision_rejects_wrong_direction() {
    let dir = temp_dir("direction");
    let receipt = dir.join("direction.json");
    write_send_receipt(
        &receipt,
        &[(
            r#""direction": "roc_to_rox""#,
            r#""direction": "rox_to_roc""#,
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "wrong direction should fail:\n{output}");
    assert!(output.contains("direction expected 'roc_to_rox'"));
}

#[test]
fn actual_roc_to_rox_decision_rejects_public_or_production_labels() {
    for (label, from, to) in [
        (
            "public",
            r#""test_only_mint_label": "test-only-rox-private-testnet""#,
            r#""test_only_mint_label": "public-rox-mainnet""#,
        ),
        (
            "production",
            r#""test_only_token_account_label": "test-only-rox-token-account-private-testnet""#,
            r#""test_only_token_account_label": "production-rox-token-account""#,
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("bad-label.json");
        write_send_receipt(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-send-receipt", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} label should fail:\n{output}");
        assert!(output.contains("must stay test-only/private-testnet"));
    }
}

#[test]
fn actual_roc_to_rox_decision_requires_shadow_roc_burn_only_marker() {
    let dir = temp_dir("real-roc");
    let receipt = dir.join("real-roc.json");
    write_send_receipt(
        &receipt,
        &[(
            r#""shadow_roc_burn_only": true"#,
            r#""shadow_roc_burn_only": false"#,
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "missing shadow-only marker should fail:\n{output}");
    assert!(output.contains("receipt must set shadow_roc_burn_only true"));
}
