//! RO:WHAT — Tests BUILD_PLAN4 Phase 7 actual capped ROC-to-ROX send receipt boundary.
//! RO:WHY — Keeps actual capped forward-flow evidence explicit, tiny-capped, redacted, and separate from real ROC mutation.
//! RO:INTERACTS — scripts/check_actual_roc_to_rox_private_testnet_run.sh.
//! RO:INVARIANTS — devnet/testnet only; shadow ROC burn only; explicit approval; accepted gates; test-only ROX; readback required.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-relayer --test actual_roc_to_rox_capped_send.

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
    let dir = std::env::temp_dir().join(format!("rox-anchor-actual-roc-to-rox-{label}-{nanos}"));
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
fn actual_roc_to_rox_send_template_is_redacted_and_capped() {
    let (ok, output) = run_script(&["--template-send-sent", "testnet"]);

    assert!(ok, "template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-roc-to-rox-capped-send.v1"));
    assert!(output.contains("actual_roc_to_rox_capped_send_receipt"));
    assert!(output.contains("I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND"));
    assert!(output.contains(r#""shadow_roc_burn_only": true"#));
    assert!(output.contains(r#""transaction_submission": true"#));
    assert!(output.contains(r#""send_authorized": true"#));
    assert!(output.contains(r#""readback_required": true"#));
    assert!(output.contains(r#""real_roc_burn": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
}

#[test]
fn actual_roc_to_rox_send_receipt_accepts_sent_shape() {
    let dir = temp_dir("sent");
    let receipt = dir.join("sent.json");
    write_send_receipt(&receipt, &[]);

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "sent receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 7 ROC-to-ROX send receipt checks passed"));
    assert!(output.contains("sent receipt satisfies capped send gates"));
}

#[test]
fn actual_roc_to_rox_send_receipt_accepts_blocked_non_submitting_shape() {
    let dir = temp_dir("blocked");
    let receipt = dir.join("blocked.json");
    write_send_receipt(
        &receipt,
        &[
            (r#""send_outcome": "sent""#, r#""send_outcome": "blocked""#),
            (
                r#""simulation_result": "passed""#,
                r#""simulation_result": "blocked""#,
            ),
            (
                r#""operator_approval": "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND""#,
                r#""operator_approval": "missing""#,
            ),
            (
                r#""external_signer_used": true"#,
                r#""external_signer_used": false"#,
            ),
            (
                r#""transaction_submission": true"#,
                r#""transaction_submission": false"#,
            ),
            (r#""send_authorized": true"#, r#""send_authorized": false"#),
            (
                r#""signature_generated": true"#,
                r#""signature_generated": false"#,
            ),
            (
                r#""transaction_signature": "<redacted-testnet-signature>""#,
                r#""transaction_signature": "none""#,
            ),
            (r#""send_slot": "123456""#, r#""send_slot": "none""#),
            (
                r#""test_only_rox_delta_minor": "1""#,
                r#""test_only_rox_delta_minor": "0""#,
            ),
            (
                r#""readback_required": true"#,
                r#""readback_required": false"#,
            ),
            (
                r#""operator_approval": "missing""#,
                r#""failure_reason_redacted": "<redacted-safe-capped-send-blocker>",
  "operator_approval": "missing""#,
            ),
        ],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "blocked receipt should pass:\n{output}");
    assert!(output.contains("blocked/failed receipt remains non-submitting evidence"));
}

#[test]
fn actual_roc_to_rox_send_receipt_rejects_mainnet_cluster() {
    let dir = temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_send_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet receipt should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_roc_to_rox_send_receipt_rejects_missing_required_gate() {
    for (label, from, to, expected) in [
        (
            "read_only",
            r#""read_only_evidence_status": "verified""#,
            r#""read_only_evidence_status": "missing""#,
            "read_only_evidence_status expected 'verified'",
        ),
        (
            "proof",
            r#""proof_review_status": "accepted""#,
            r#""proof_review_status": "rejected""#,
            "proof_review_status expected 'accepted'",
        ),
        (
            "coordinator",
            r#""coordinator_decision_status": "accepted""#,
            r#""coordinator_decision_status": "blocked""#,
            "coordinator_decision_status expected 'accepted'",
        ),
        (
            "simulation",
            r#""simulation_result": "passed""#,
            r#""simulation_result": "blocked""#,
            "simulation_result expected 'passed'",
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("gate.json");
        write_send_receipt(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-send-receipt", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} gate should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_roc_to_rox_send_receipt_rejects_missing_approval_or_signature() {
    for (label, from, to, expected) in [
        (
            "approval",
            r#""operator_approval": "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND""#,
            r#""operator_approval": "missing""#,
            "operator_approval expected 'I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND'",
        ),
        (
            "signature",
            r#""transaction_signature": "<redacted-testnet-signature>""#,
            r#""transaction_signature": "none""#,
            "sent receipt requires transaction_signature",
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("approval.json");
        write_send_receipt(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-send-receipt", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_roc_to_rox_send_receipt_rejects_real_roc_or_finality_claims() {
    for (label, from, to) in [
        (
            "real_burn",
            r#""real_roc_burn": false"#,
            r#""real_roc_burn": true"#,
        ),
        (
            "real_mutation",
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
        write_send_receipt(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-send-receipt", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_roc_to_rox_send_receipt_rejects_unredacted_paths() {
    let dir = temp_dir("secret-path");
    let receipt = dir.join("secret-path.json");
    write_send_receipt(
        &receipt,
        &[(
            r#""signer_path_redacted": "<redacted-external-signer-path>""#,
            r#""signer_path_redacted": "/Users/operator/private/payer.json""#,
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-send-receipt", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unredacted receipt should fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}
