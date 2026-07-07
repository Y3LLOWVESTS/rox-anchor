//! RO:WHAT — Tests BUILD_PLAN4 Phase 10 coordinator negative-drill decision boundaries.
//! RO:WHY — Ensures unsafe deployed-testnet evidence shapes are rejected/blocked before relayer authorization.
//! RO:INTERACTS — scripts/check_actual_private_testnet_negative_drills.sh.
//! RO:INVARIANTS — devnet/testnet only; expected failure true; coordinator never turns negative drills into acceptance/finality.
//! RO:SECURITY — local file checks only; no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test actual_testnet_negative_drills.

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

fn suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
        .to_string()
}

fn write_receipt(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_coordinator_phase10_negative_drill_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp receipt should write");
    path
}

fn template_receipt(drill_name: &str) -> String {
    let (ok, output) = run_script(&["--template-failure", "testnet", drill_name]);
    assert!(ok, "template should print for {drill_name}:\n{output}");
    output
}

fn check_receipt(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-failure-receipt", &path_arg])
}

#[test]
fn coordinator_negative_drills_reject_binding_mismatches_before_authorization() {
    for drill in [
        "wrong_program_id",
        "wrong_mint",
        "wrong_token_account",
        "wrong_authority",
    ] {
        let receipt = template_receipt(drill);

        assert!(receipt.contains(r#""proof_review_status": "rejected""#));
        assert!(receipt.contains(r#""coordinator_decision_status": "rejected""#));
        assert!(receipt.contains(r#""relayer_status": "blocked""#));
        assert!(receipt.contains(r#""send_authorized": false"#));
        assert!(receipt.contains(r#""transaction_submission": false"#));

        let path = write_receipt(drill, &receipt);
        let (ok, output) = check_receipt(&path);

        assert!(
            ok,
            "{drill} coordinator negative receipt should pass:\n{output}"
        );
        assert!(output.contains("receipt rejects submission"));
    }
}

#[test]
fn coordinator_negative_drills_block_missing_evidence_without_success_labels() {
    for drill in [
        "missing_config_account",
        "missing_mint_account",
        "missing_receipt",
    ] {
        let receipt = template_receipt(drill);

        assert!(receipt.contains(r#""proof_review_status": "missing_evidence""#));
        assert!(receipt.contains(r#""coordinator_decision_status": "blocked""#));
        assert!(receipt.contains(r#""readback_status": "not_performed""#));
        assert!(receipt.contains(r#""system_returned_safe_state": true"#));
        assert!(!receipt.contains(r#""coordinator_decision_status": "accepted""#));

        let path = write_receipt(drill, &receipt);
        let (ok, output) = check_receipt(&path);

        assert!(
            ok,
            "{drill} missing-evidence receipt should pass:\n{output}"
        );
        assert!(output.contains("receipt is devnet/testnet only"));
    }
}

#[test]
fn coordinator_negative_drills_block_operator_and_cap_failures_before_send() {
    for drill in ["operator_approval_omitted", "send_disabled", "cap_exceeded"] {
        let receipt = template_receipt(drill);

        assert!(receipt.contains(r#""proof_review_status": "blocked""#));
        assert!(receipt.contains(r#""coordinator_decision_status": "blocked""#));
        assert!(receipt.contains(r#""relayer_status": "not_authorized""#));
        assert!(receipt.contains(r#""send_authorized": false"#));
        assert!(receipt.contains(r#""signature_generated": false"#));

        let path = write_receipt(drill, &receipt);
        let (ok, output) = check_receipt(&path);

        assert!(
            ok,
            "{drill} operator/cap failure receipt should pass:\n{output}"
        );
        assert!(output.contains("receipt rejects submission"));
    }
}

#[test]
fn coordinator_negative_drills_block_halt_and_recovery_state_transitions() {
    for drill in [
        "halt_before_simulation",
        "halt_after_simulation_before_send",
        "halt_after_send_before_readback",
        "recovery_during_pending_operation",
    ] {
        let receipt = template_receipt(drill);

        assert!(receipt.contains(r#""proof_review_status": "blocked""#));
        assert!(receipt.contains(r#""coordinator_decision_status": "blocked""#));
        assert!(receipt.contains(r#""private_testnet_only": true"#));
        assert!(receipt.contains(r#""test_only_assets_only": true"#));
        assert!(receipt.contains(r#""real_roc_release": false"#));
        assert!(receipt.contains(r#""real_roc_mutation": false"#));

        let path = write_receipt(drill, &receipt);
        let (ok, output) = check_receipt(&path);

        assert!(ok, "{drill} halt/recovery receipt should pass:\n{output}");
        assert!(output.contains("statuses are fail-safe statuses"));
    }
}

#[test]
fn coordinator_negative_drills_reject_acceptance_or_settlement_claims() {
    let receipt = template_receipt("wrong_mint").replace(
        r#""coordinator_decision_status": "rejected""#,
        r#""coordinator_decision_status": "accepted""#,
    );
    let path = write_receipt("accepted_coordinator", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(
        !ok,
        "coordinator acceptance must fail for negative drill:\n{output}"
    );
    assert!(
        output.contains("coordinator_decision_status must be rejected or blocked")
            || output.contains("success-like marker")
    );

    let receipt = template_receipt("wrong_token_account").replace(
        r#""production_bridge_settlement": false"#,
        r#""production_bridge_settlement": true"#,
    );
    let path = write_receipt("production_settlement", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(!ok, "production settlement claim must fail:\n{output}");
    assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
}
