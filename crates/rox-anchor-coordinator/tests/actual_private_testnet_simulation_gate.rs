//! RO:WHAT — Tests BUILD_PLAN4 Phase 6 coordinator-side simulation gate receipt boundary.
//! RO:WHY — Proves simulation cannot bypass read-only evidence, proof review, coordinator acceptance, or relayer dry-run gates.
//! RO:INTERACTS — scripts/check_actual_private_testnet_simulation.sh.
//! RO:INVARIANTS — successful simulation requires all gates accepted; blocked simulations remain non-sendable.
//! RO:SECURITY — no live RPC, wallet load, live simulation, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test actual_private_testnet_simulation_gate.

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

    let dir =
        std::env::temp_dir().join(format!("rox-anchor-actual-simulation-gate-{label}-{nanos}"));
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
  "direction": "rox_to_roc",
  "program_name": "rox_anchor",
  "program_id": "{PROGRAM_ID}",
  "simulation_outcome": "simulated",
  "operation_id": "actual-simulation-op-0002",
  "idempotency_key": "actual-simulation-idem-0002",
  "nonce": "actual-simulation-nonce-0002",
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

    fs::write(path, receipt).expect("simulation gate receipt fixture should be written");
}

#[test]
fn actual_private_testnet_simulation_gate_accepts_reverse_flow_shape() {
    let dir = unique_temp_dir("reverse");
    let receipt = dir.join("reverse.json");
    write_receipt(&receipt, &[]);

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(ok, "reverse simulation receipt should pass:\n{output}");
    assert!(output.contains("receipt direction is rox_to_roc"));
    assert!(output.contains("simulated receipt satisfies all required gates"));
}

#[test]
fn actual_private_testnet_simulation_gate_rejects_unknown_direction() {
    let dir = unique_temp_dir("direction");
    let receipt = dir.join("direction.json");
    write_receipt(
        &receipt,
        &[(
            r#""direction": "rox_to_roc""#,
            r#""direction": "public_bridge""#,
        )],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "unknown direction should fail:\n{output}");
    assert!(output.contains("direction must be roc_to_rox or rox_to_roc"));
}

#[test]
fn actual_private_testnet_simulation_gate_rejects_public_or_production_labels() {
    for (label, replacement) in [
        (
            "public",
            (
                r#""test_only_mint_label": "test-only-rox-private-testnet""#,
                r#""test_only_mint_label": "public-rox-mainnet""#,
            ),
        ),
        (
            "production",
            (
                r#""test_only_token_account_label": "test-only-rox-token-account-private-testnet""#,
                r#""test_only_token_account_label": "production-rox-token-account""#,
            ),
        ),
    ] {
        let dir = unique_temp_dir(label);
        let receipt = dir.join("bad-label.json");
        write_receipt(&receipt, &[replacement]);

        let receipt_arg = receipt.to_string_lossy().to_string();
        let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
        let _ = fs::remove_dir_all(&dir);

        assert!(!ok, "{label} label should fail:\n{output}");
        assert!(output.contains("must stay test-only/private-testnet"));
    }
}

#[test]
fn actual_private_testnet_simulation_gate_rejects_amount_above_declared_max() {
    let dir = unique_temp_dir("amount-relation");
    let receipt = dir.join("amount-relation.json");
    write_receipt(
        &receipt,
        &[
            (r#""amount_minor": "1""#, r#""amount_minor": "2""#),
            (r#""max_amount_minor": "1""#, r#""max_amount_minor": "1""#),
        ],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "amount above declared max should fail:\n{output}");
    assert!(output.contains("amount_minor cannot exceed max_amount_minor"));
}

#[test]
fn actual_private_testnet_simulation_gate_rejects_failed_receipt_that_claims_passed_result() {
    let dir = unique_temp_dir("failed-passed");
    let receipt = dir.join("failed-passed.json");
    write_receipt(
        &receipt,
        &[
            (
                r#""simulation_outcome": "simulated""#,
                r#""simulation_outcome": "failed""#,
            ),
            (
                r#""read_only_evidence_verified": true"#,
                r#""read_only_evidence_verified": false"#,
            ),
            (
                r#""simulation_log_redacted": "<redacted-simulation-log>""#,
                r#""failure_reason_redacted": "<redacted-safe-simulation-failure>",
  "simulation_log_redacted": "<redacted-simulation-log>""#,
            ),
        ],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(
        !ok,
        "failed receipt claiming passed result should fail:\n{output}"
    );
    assert!(output.contains("non-simulated receipt must not claim passed simulation_result"));
}

#[test]
fn actual_private_testnet_simulation_gate_rejects_non_simulate_only_receipt() {
    let dir = unique_temp_dir("simulate-only");
    let receipt = dir.join("simulate-only.json");
    write_receipt(
        &receipt,
        &[(r#""simulate_only": true"#, r#""simulate_only": false"#)],
    );

    let receipt_arg = receipt.to_string_lossy().to_string();
    let (ok, output) = run_script(&["--check-simulation-receipt", &receipt_arg]);
    let _ = fs::remove_dir_all(&dir);

    assert!(!ok, "non-simulate-only receipt should fail:\n{output}");
    assert!(output.contains("receipt must set simulate_only true"));
}
