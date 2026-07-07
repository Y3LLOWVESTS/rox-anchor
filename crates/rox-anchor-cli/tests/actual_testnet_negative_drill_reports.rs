//! RO:WHAT — Tests BUILD_PLAN4 Phase 10 CLI-facing negative-drill docs/checker output.
//! RO:WHY — Keeps operator-visible failure reports redacted, deterministic, and non-authorizing.
//! RO:INTERACTS — scripts/check_actual_private_testnet_negative_drills.sh and docs/pilot.
//! RO:INVARIANTS — negative drills are private-testnet-only, expected failures, no production settlement, no real ROC mutation.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, real ROC release, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_testnet_negative_drill_reports.

use std::{path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let output = Command::new("bash")
        .arg(root.join("scripts/check_actual_private_testnet_negative_drills.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("actual private testnet negative drill checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

#[test]
fn actual_testnet_negative_drill_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(
        output.contains("BUILD_PLAN4 Phase 10 actual negative drill documentation checks passed")
    );
    assert!(output.contains("actual private testnet negative drill runbook exists"));
    assert!(output.contains("failure receipts from finality"));
    assert!(output.contains("real ROC mutation"));
}

#[test]
fn actual_testnet_negative_drill_preflight_is_local_only_and_non_submitting() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 10 actual negative drill preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, mint, burn, settle, release ROC, or mutate ROC"));
}

#[test]
fn actual_testnet_negative_drill_failure_template_is_redacted_and_non_authorizing() {
    let (ok, output) = run_script(&["--template-failure", "testnet", "wrong_program_id"]);

    assert!(ok, "template should print:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 10"));
    assert!(output.contains("actual_private_testnet_negative_drill_receipt"));
    assert!(output.contains("rox-anchor.actual-private-testnet-negative-drill.v1"));
    assert!(output.contains("<redacted-"));
    assert!(output.contains(r#""expected_failure": true"#));
    assert!(output.contains(r#""private_testnet_only": true"#));
    assert!(output.contains(r#""test_only_assets_only": true"#));
    assert!(output.contains(r#""system_returned_safe_state": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""send_authorized": false"#));
    assert!(output.contains(r#""signature_generated": false"#));
    assert!(output.contains(r#""production_bridge_settlement": false"#));
    assert!(output.contains(r#""real_roc_release": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
    assert!(!output.contains(r#""production_bridge_settlement": true"#));
    assert!(!output.contains(r#""real_roc_mutation": true"#));
    assert!(!output.contains(r#""finality_claim": true"#));
}

#[test]
fn actual_testnet_negative_drill_runbook_covers_required_matrix_without_settlement_claims() {
    let root = repo_root();
    let doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md"))
            .expect("actual negative drill runbook should be readable");

    for marker in [
        "wrong_program_id",
        "wrong_mint",
        "wrong_token_account",
        "wrong_authority",
        "missing_config_account",
        "missing_mint_account",
        "stale_readback",
        "under_quorum_rpc_evidence",
        "rpc_provider_disagreement",
        "duplicate_operation_id",
        "duplicate_idempotency_key",
        "nonce_reuse",
        "receipt_tamper",
        "missing_receipt",
        "operator_approval_omitted",
        "send_disabled",
        "cap_exceeded",
        "halt_before_simulation",
        "halt_after_simulation_before_send",
        "halt_after_send_before_readback",
        "recovery_during_pending_operation",
        "readback_missing_after_send",
    ] {
        assert!(
            doc.contains(marker),
            "runbook missing drill marker: {marker}"
        );
    }

    assert!(doc.contains("No real ROC release."));
    assert!(doc.contains("No real internal ROC mutation."));
    assert!(doc.contains("No production bridge settlement."));
    assert!(doc.contains("No fake finality."));
    assert!(doc.contains("The negative drill receipt is not runtime authorization."));
    assert!(doc.contains("The negative drill receipt is not production settlement."));

    assert!(!doc.contains("/Users/"));
    assert!(!doc.contains("/home/"));
    assert!(!doc.contains("api-key="));
    assert!(!doc.contains("access_token="));
    assert!(!doc.contains(r#""production_bridge_settlement": true"#));
    assert!(!doc.contains(r#""real_roc_mutation": true"#));
    assert!(!doc.contains(r#""finality_claim": true"#));
}
