//! RO:WHAT — Tests BUILD_PLAN4 Phase 6 actual private testnet simulation receipt boundary.
//! RO:WHY — Keeps simulation evidence distinct from send authorization, signing, finality, settlement, and public mint availability.
//! RO:INTERACTS — scripts/check_actual_private_testnet_simulation.sh.
//! RO:INVARIANTS — devnet/testnet only; simulate-only true; tiny caps; test-only labels; required gates; no live send claims.
//! RO:SECURITY — no live RPC, wallet load, live simulation, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-relayer --test actual_private_testnet_simulation.

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
        .arg(root.join("scripts/check_actual_private_testnet_simulation.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("simulation checker should execute");

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

    let dir = std::env::temp_dir().join(format!("rox-anchor-actual-simulation-{label}-{nanos}"));
    fs::create_dir_all(&dir).expect("temp dir should be created");
    dir
}

fn write_receipt(path: &Path, overrides: &[(&str, &str)]) {
    let mut receipt = format!(
        r#"{{
  "schema": "rox-anchor.actual-private-testnet-simulation.v1",
  "phase": "BUILD_PLAN4 Phase 6",
  "receipt_role": "actual_private_testnet_simulation_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "simulation_outcome": "simulated",
  "operation_id": "actual-simulation-op-0001",
  "idempotency_key": "actual-simulation-idem-0001",
  "nonce": "actual-simulation-nonce-0001",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "read_only_evidence_status": "verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_dry_run_status": "accepted",
  "simulation_result": "passed",
  "simulation_log_redacted": "<redacted-simulation-log>",
  "read_only_evidence_required": true,
  "read_only_evidence_verified": true,
  "simulate_only": true,
  "transaction_submission": false,
  "send_authorized": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "receipt_promotable_to_send": false,
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

    fs::write(path, receipt).expect("simulation receipt fixture should be written");
}

#[test]
fn actual_private_testnet_simulation_template_is_simulate_only_and_redacted() {
    let (ok, output) = run_script(&["--template-simulated", "roc_to_rox", "testnet"]);

    assert!(ok, "simulated template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-private-testnet-simulation.v1"));
    assert!(output.contains("actual_private_testnet_simulation_receipt"));
    assert!(output.contains("<redacted-program-account>"));
    assert!(output.contains("<redacted-simulation-log>"));
    assert!(output.contains(r#""simulate_only": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""send_authorized": false"#));
    assert!(output.contains(r#""wallet_loaded": false"#));
    assert!(output.contains(r#""signature_generated": false"#));
    assert!(output.contains(r#""receipt_promotable_to_send": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
}

#[test]
fn actual_private_testnet_simulation_receipt_accepts_successful_gate_shape() {
    let dir = unique_temp_dir("success");
    let receipt = dir.join("success.json");
    write_receipt(&receipt, &[]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "successful simulation receipt should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 6 simulation receipt checks passed"));
    assert!(output.contains("receipt simulation_outcome = simulated"));
    assert!(output.contains("simulated receipt satisfies all required gates"));
}

#[test]
fn actual_private_testnet_simulation_receipt_accepts_blocked_non_sendable_shape() {
    let dir = unique_temp_dir("blocked");
    let receipt = dir.join("blocked.json");
    write_receipt(
        &receipt,
        &[
            (
                r#""simulation_outcome": "simulated""#,
                r#""simulation_outcome": "blocked""#,
            ),
            (
                r#""read_only_evidence_status": "verified""#,
                r#""read_only_evidence_status": "missing""#,
            ),
            (
                r#""proof_review_status": "accepted""#,
                r#""proof_review_status": "not_run""#,
            ),
            (
                r#""coordinator_decision_status": "accepted""#,
                r#""coordinator_decision_status": "not_run""#,
            ),
            (
                r#""relayer_dry_run_status": "accepted""#,
                r#""relayer_dry_run_status": "not_run""#,
            ),
            (
                r#""simulation_result": "passed""#,
                r#""simulation_result": "not_run""#,
            ),
            (
                r#""simulation_log_redacted": "<redacted-simulation-log>""#,
                r#""failure_reason_redacted": "<redacted-safe-simulation-blocker>",
  "simulation_log_redacted": "<redacted-simulation-log>""#,
            ),
            (
                r#""read_only_evidence_verified": true"#,
                r#""read_only_evidence_verified": false"#,
            ),
        ],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "blocked simulation receipt should pass:\n{output}");
    assert!(output.contains("receipt simulation_outcome = blocked"));
    assert!(output.contains("non-simulated receipt remains blocked/failed evidence"));
}

#[test]
fn actual_private_testnet_simulation_receipt_rejects_mainnet_cluster() {
    let dir = unique_temp_dir("mainnet");
    let receipt = dir.join("mainnet.json");
    write_receipt(
        &receipt,
        &[(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#)],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "mainnet simulation receipt should fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_private_testnet_simulation_receipt_rejects_send_authorization_or_submission_claims() {
    for (label, forbidden) in [
        (
            "transaction_submission",
            (
                r#""transaction_submission": false"#,
                r#""transaction_submission": true"#,
            ),
        ),
        (
            "send_authorized",
            (r#""send_authorized": false"#, r#""send_authorized": true"#),
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
        (
            "promotable",
            (
                r#""receipt_promotable_to_send": false"#,
                r#""receipt_promotable_to_send": true"#,
            ),
        ),
    ] {
        let dir = unique_temp_dir(label);
        let receipt = dir.join("forbidden.json");
        write_receipt(&receipt, &[forbidden]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}

#[test]
fn actual_private_testnet_simulation_receipt_rejects_missing_required_gates() {
    for (label, replacement, expected) in [
        (
            "read_only",
            (
                r#""read_only_evidence_status": "verified""#,
                r#""read_only_evidence_status": "missing""#,
            ),
            "read_only_evidence_status expected 'verified'",
        ),
        (
            "proof",
            (
                r#""proof_review_status": "accepted""#,
                r#""proof_review_status": "rejected""#,
            ),
            "proof_review_status expected 'accepted'",
        ),
        (
            "coordinator",
            (
                r#""coordinator_decision_status": "accepted""#,
                r#""coordinator_decision_status": "blocked""#,
            ),
            "coordinator_decision_status expected 'accepted'",
        ),
        (
            "dry_run",
            (
                r#""relayer_dry_run_status": "accepted""#,
                r#""relayer_dry_run_status": "blocked""#,
            ),
            "relayer_dry_run_status expected 'accepted'",
        ),
    ] {
        let dir = unique_temp_dir(label);
        let receipt = dir.join("missing-gate.json");
        write_receipt(&receipt, &[replacement]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} gate failure should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_private_testnet_simulation_receipt_rejects_over_cap_amounts() {
    for (field, replacement, expected) in [
        (
            r#""amount_minor": "1""#,
            r#""amount_minor": "1001""#,
            "amount_minor exceeds cap",
        ),
        (
            r#""max_amount_minor": "1""#,
            r#""max_amount_minor": "1001""#,
            "max_amount_minor exceeds cap",
        ),
        (
            r#""max_operations": "1""#,
            r#""max_operations": "11""#,
            "max_operations exceeds cap",
        ),
    ] {
        let dir = unique_temp_dir("over-cap");
        let receipt = dir.join("over-cap.json");
        write_receipt(&receipt, &[(field, replacement)]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "over-cap receipt should fail:\n{output}");
        assert!(output.contains(expected));
    }
}

#[test]
fn actual_private_testnet_simulation_receipt_rejects_unredacted_paths_or_accounts() {
    let dir = unique_temp_dir("secret-path");
    let receipt = dir.join("secret-path.json");
    write_receipt(
        &receipt,
        &[(
            r#""simulation_log_redacted": "<redacted-simulation-log>""#,
            r#""simulation_log_redacted": "/Users/operator/private/provider-token.txt""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unredacted simulation receipt should fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn actual_private_testnet_simulation_receipt_rejects_public_or_finality_claims() {
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
        let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} receipt should fail:\n{output}");
        assert!(output.contains("forbidden true boolean"));
    }
}
