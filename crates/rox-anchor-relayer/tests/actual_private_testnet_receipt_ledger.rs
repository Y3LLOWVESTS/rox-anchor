//! RO:WHAT — Tests BUILD_PLAN4 Phase 9 actual private testnet receipt ledger validation.
//! RO:WHY — Ensures actual private-testnet evidence can be linked without claiming settlement, finality, or real ROC mutation.
//! RO:INTERACTS — scripts/check_actual_private_testnet_receipts.sh.
//! RO:INVARIANTS — receipt IDs unique; operation/idempotency/nonce bindings match; redacted evidence; no production/mainnet/real-ROC claims.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, real ROC release, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-relayer --test actual_private_testnet_receipt_ledger.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let output = Command::new("bash")
        .arg(root.join("scripts/check_actual_private_testnet_receipts.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("actual private testnet receipt checker should execute");

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
    let dir =
        std::env::temp_dir().join(format!("rox-anchor-actual-receipt-ledger-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_ledger(path: &Path, overrides: &[(&str, &str)]) {
    let mut ledger = r#"{
  "schema": "rox-anchor.actual-private-testnet-receipt-ledger.v1",
  "phase": "BUILD_PLAN4 Phase 9",
  "receipt_role": "actual_private_testnet_receipt_ledger",
  "cluster": "testnet",
  "ledger_id": "<redacted-ledger-id>",
  "ledger_outcome": "reconciled",
  "reconciliation_status": "reconciled",
  "operation_id": "actual-private-testnet-op-0001",
  "idempotency_key": "actual-private-testnet-idem-0001",
  "nonce": "actual-private-testnet-nonce-0001",
  "receipt_ids": "deploy-0001,init-0001,read-only-0001,simulation-0001,roc-to-rox-send-0001,roc-to-rox-readback-0001,rox-to-roc-send-0001,rox-to-roc-readback-0001",
  "receipt_operation_ids": "actual-private-testnet-op-0001",
  "receipt_idempotency_keys": "actual-private-testnet-idem-0001",
  "receipt_nonces": "actual-private-testnet-nonce-0001",
  "deploy_receipt_status": "verified",
  "initialization_receipt_status": "verified",
  "read_only_evidence_status": "verified",
  "simulation_receipt_status": "verified",
  "roc_to_rox_send_status": "verified",
  "roc_to_rox_readback_status": "verified",
  "rox_to_roc_send_status": "verified",
  "rox_to_roc_readback_status": "verified",
  "dry_run_release_intent_status": "verified",
  "receipt_chain_status": "linked",
  "operation_binding_status": "matched",
  "idempotency_binding_status": "matched",
  "nonce_binding_status": "matched",
  "signature_binding_status": "redacted",
  "readback_binding_status": "verified",
  "transaction_signatures_redacted": "<redacted-signature-list>",
  "readback_evidence_redacted": "<redacted-readback-evidence>",
  "operator_report_redacted": "<redacted-operator-report>",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "readback_verified": true,
  "duplicate_receipts_detected": false,
  "operation_id_mismatch_detected": false,
  "idempotency_key_mismatch_detected": false,
  "nonce_mismatch_detected": false,
  "live_submission_without_signature_detected": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "finality_claim": false
}"#
    .to_string();

    for (from, to) in overrides {
        ledger = ledger.replace(from, to);
    }

    fs::write(path, ledger).expect("ledger should be written");
}

#[test]
fn actual_private_testnet_receipt_ledger_template_is_redacted_and_non_authorizing() {
    let (ok, output) = run_script(&["--template-reconciled", "testnet"]);

    assert!(ok, "template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-private-testnet-receipt-ledger.v1"));
    assert!(output.contains("actual_private_testnet_receipt_ledger"));
    assert!(output.contains(r#""ledger_outcome": "reconciled""#));
    assert!(output.contains(r#""private_testnet_only": true"#));
    assert!(output.contains(r#""test_only_assets_only": true"#));
    assert!(output.contains(r#""production_bridge_settlement": false"#));
    assert!(output.contains(r#""real_roc_release": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
}

#[test]
fn actual_private_testnet_receipt_ledger_accepts_reconciled_shape() {
    let dir = temp_dir("reconciled");
    let receipt = dir.join("ledger.json");
    write_ledger(&receipt, &[]);

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-ledger", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "reconciled ledger should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 9 actual receipt ledger checks passed"));
    assert!(output.contains("reconciled ledger satisfies receipt linkage gates"));
}

#[test]
fn actual_private_testnet_receipt_ledger_accepts_quarantined_non_success_shape() {
    let dir = temp_dir("quarantined");
    let receipt = dir.join("ledger.json");
    write_ledger(
        &receipt,
        &[
            (
                r#""ledger_outcome": "reconciled""#,
                r#""ledger_outcome": "quarantined""#,
            ),
            (
                r#""reconciliation_status": "reconciled""#,
                r#""reconciliation_status": "quarantined""#,
            ),
            (
                r#""simulation_receipt_status": "verified""#,
                r#""simulation_receipt_status": "blocked""#,
            ),
            (
                r#""roc_to_rox_send_status": "verified""#,
                r#""roc_to_rox_send_status": "not_performed""#,
            ),
            (
                r#""roc_to_rox_readback_status": "verified""#,
                r#""roc_to_rox_readback_status": "not_performed""#,
            ),
            (
                r#""rox_to_roc_send_status": "verified""#,
                r#""rox_to_roc_send_status": "not_performed""#,
            ),
            (
                r#""rox_to_roc_readback_status": "verified""#,
                r#""rox_to_roc_readback_status": "not_performed""#,
            ),
            (
                r#""dry_run_release_intent_status": "verified""#,
                r#""dry_run_release_intent_status": "not_performed""#,
            ),
            (
                r#""readback_binding_status": "verified""#,
                r#""readback_binding_status": "not_performed""#,
            ),
            (
                r#""readback_verified": true"#,
                r#""readback_verified": false"#,
            ),
            (
                r#""transaction_signatures_redacted": "<redacted-signature-list>""#,
                r#""quarantine_reason_redacted": "<redacted-reconciliation-blocker>",
  "transaction_signatures_redacted": "<redacted-signature-list>""#,
            ),
        ],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-ledger", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "quarantined ledger should pass:\n{output}");
    assert!(output.contains("incomplete/quarantined ledger remains non-success evidence"));
}

#[test]
fn actual_private_testnet_receipt_ledger_rejects_duplicate_receipt_ids() {
    let dir = temp_dir("duplicate");
    let receipt = dir.join("ledger.json");
    write_ledger(
        &receipt,
        &[(
            "deploy-0001,init-0001,read-only-0001,simulation-0001",
            "deploy-0001,init-0001,init-0001,simulation-0001",
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-ledger", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "duplicate receipt IDs should fail:\n{output}");
    assert!(output.contains("receipt_ids contain duplicate values"));
}

#[test]
fn actual_private_testnet_receipt_ledger_rejects_binding_mismatch() {
    for (label, from, to, expected) in [
        (
            "operation",
            r#""operation_binding_status": "matched""#,
            r#""operation_binding_status": "mismatched""#,
            "operation_binding_status expected 'matched'",
        ),
        (
            "idempotency",
            r#""idempotency_binding_status": "matched""#,
            r#""idempotency_binding_status": "mismatched""#,
            "idempotency_binding_status expected 'matched'",
        ),
        (
            "nonce",
            r#""nonce_binding_status": "matched""#,
            r#""nonce_binding_status": "mismatched""#,
            "nonce_binding_status expected 'matched'",
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("ledger.json");
        write_ledger(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-ledger", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} mismatch should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_private_testnet_receipt_ledger_rejects_runtime_or_real_roc_claims() {
    for (label, from, to) in [
        (
            "live_without_signature",
            r#""live_submission_without_signature_detected": false"#,
            r#""live_submission_without_signature_detected": true"#,
        ),
        (
            "production",
            r#""production_bridge_settlement": false"#,
            r#""production_bridge_settlement": true"#,
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
        (
            "finality",
            r#""finality_claim": false"#,
            r#""finality_claim": true"#,
        ),
    ] {
        let dir = temp_dir(label);
        let receipt = dir.join("ledger.json");
        write_ledger(&receipt, &[(from, to)]);

        let arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-ledger", &arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} claim should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_private_testnet_receipt_ledger_rejects_unredacted_paths() {
    let dir = temp_dir("secret-path");
    let receipt = dir.join("ledger.json");
    write_ledger(
        &receipt,
        &[(
            r#""operator_report_redacted": "<redacted-operator-report>""#,
            r#""operator_report_redacted": "/Users/operator/private/receipt.json""#,
        )],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-ledger", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unredacted ledger should fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn actual_private_testnet_receipt_ledger_rejects_mainnet_cluster() {
    let dir = temp_dir("mainnet");
    let receipt = dir.join("ledger.json");
    write_ledger(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-ledger", &arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet ledger should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}
