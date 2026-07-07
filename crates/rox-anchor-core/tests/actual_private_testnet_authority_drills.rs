//! RO:WHAT — Tests BUILD_PLAN4 Phase 11 halt/recovery/authority drill report shapes.
//! RO:WHY — Keeps operator safety states inspectable while preventing runtime, key, settlement, and ROC mutation claims.
//! RO:INTERACTS — scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh.
//! RO:INVARIANTS — devnet/testnet only; redacted reports; wrong authority fails safe; valid recovery can resume clean flow without settlement.
//! RO:SECURITY — local file checks only; no live RPC, wallet load, authority key load, signing, submission, upgrade, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-core --test actual_private_testnet_authority_drills.

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
        .arg(root.join("scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("Phase 11 drill checker should execute");

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

fn write_report(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_phase11_authority_drill_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp report should write");
    path
}

fn template_report(drill_name: &str) -> String {
    let (ok, output) = run_script(&["--template-drill", "testnet", drill_name]);
    assert!(ok, "template should print for {drill_name}:\n{output}");
    output
}

fn check_report(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-drill-report", &path_arg])
}

#[test]
fn phase11_template_matrix_covers_halt_recovery_and_authority_cases() {
    let (ok, output) = run_script(&["--template-matrix", "testnet"]);

    assert!(ok, "matrix template should print:\n{output}");
    for marker in [
        "halt_before_simulation",
        "halt_after_simulation_before_send",
        "halt_after_capped_send_before_readback",
        "valid_recovery_after_halt",
        "clean_flow_after_valid_recovery",
        "wrong_authority_halt_attempt",
        "wrong_authority_recovery_attempt",
        "key_rotation_intent",
        "upgrade_authority_checklist",
        "separated_authority_status",
    ] {
        assert!(output.contains(marker), "matrix missing {marker}");
    }

    assert!(!output.contains(r#""transaction_submission": true"#));
    assert!(!output.contains(r#""authority_key_loaded": true"#));
    assert!(!output.contains(r#""key_rotation_executed": true"#));
    assert!(!output.contains(r#""upgrade_authority_changed": true"#));
    assert!(!output.contains(r#""real_roc_mutation": true"#));
    assert!(!output.contains(r#""finality_claim": true"#));
}

#[test]
fn halt_drill_reports_block_progress_without_submission_or_finality() {
    for drill in [
        "halt_before_simulation",
        "halt_after_simulation_before_send",
        "halt_after_capped_send_before_readback",
    ] {
        let report = template_report(drill);

        assert!(report.contains(r#""halt_status": "active""#));
        assert!(report.contains(r#""recovery_status": "required""#));
        assert!(report.contains(r#""clean_flow_resume_status": "blocked""#));
        assert!(report.contains(r#""transaction_submission": false"#));
        assert!(report.contains(r#""send_authorized": false"#));
        assert!(report.contains(r#""finality_claim": false"#));

        let path = write_report(drill, &report);
        let (ok, output) = check_report(&path);

        assert!(ok, "{drill} report should pass:\n{output}");
        assert!(output.contains("halt/recovery/authority/clean-flow states are inspectable"));
    }
}

#[test]
fn valid_recovery_allows_clean_resume_without_settlement_or_real_roc_mutation() {
    let report = template_report("valid_recovery_after_halt");

    assert!(report.contains(r#""drill_outcome": "recovered""#));
    assert!(report.contains(r#""halt_status": "cleared""#));
    assert!(report.contains(r#""recovery_status": "validated""#));
    assert!(report.contains(r#""authority_status": "validated""#));
    assert!(report.contains(r#""clean_flow_resume_status": "allowed_after_valid_recovery""#));
    assert!(report.contains(r#""production_bridge_settlement": false"#));
    assert!(report.contains(r#""real_roc_release": false"#));
    assert!(report.contains(r#""real_roc_mutation": false"#));

    let path = write_report("valid_recovery", &report);
    let (ok, output) = check_report(&path);

    assert!(ok, "valid recovery report should pass:\n{output}");
    assert!(output.contains("report rejects transaction submission"));
}

#[test]
fn wrong_authority_attempts_are_rejected_and_return_safe_state() {
    for drill in [
        "wrong_authority_halt_attempt",
        "wrong_authority_recovery_attempt",
    ] {
        let report = template_report(drill);

        assert!(report.contains(r#""drill_outcome": "blocked""#));
        assert!(report.contains(r#""authority_status": "rejected""#));
        assert!(report.contains(r#""system_returned_safe_state": true"#));
        assert!(report.contains(r#""authority_key_loaded": false"#));
        assert!(report.contains(r#""signature_generated": false"#));

        let path = write_report(drill, &report);
        let (ok, output) = check_report(&path);

        assert!(ok, "{drill} report should pass:\n{output}");
        assert!(output.contains("report is devnet/testnet only"));
    }
}

#[test]
fn key_rotation_and_upgrade_checklists_are_intent_review_only() {
    for (drill, status) in [
        (
            "key_rotation_intent",
            r#""authority_status": "intent_recorded""#,
        ),
        (
            "upgrade_authority_checklist",
            r#""authority_status": "reviewed""#,
        ),
        (
            "separated_authority_status",
            r#""authority_status": "separated""#,
        ),
    ] {
        let report = template_report(drill);

        assert!(report.contains(status));
        assert!(report.contains(r#""key_rotation_executed": false"#));
        assert!(report.contains(r#""upgrade_authority_changed": false"#));
        assert!(report.contains(r#""authority_key_loaded": false"#));
        assert!(report.contains(r#""operator_report_redacted": true"#));

        let path = write_report(drill, &report);
        let (ok, output) = check_report(&path);

        assert!(ok, "{drill} report should pass:\n{output}");
        assert!(output.contains("report rejects transaction submission"));
    }
}

#[test]
fn phase11_report_rejects_mainnet_or_authority_key_loading_claims() {
    let report = template_report("valid_recovery_after_halt")
        .replace(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#);
    let path = write_report("mainnet", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "mainnet report must fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));

    let report = template_report("key_rotation_intent").replace(
        r#""authority_key_loaded": false"#,
        r#""authority_key_loaded": true"#,
    );
    let path = write_report("authority_key_loaded", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "authority_key_loaded true must fail:\n{output}");
    assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
}

#[test]
fn phase11_report_rejects_unredacted_authority_paths() {
    let report = template_report("wrong_authority_halt_attempt").replace(
        r#""action_reason_redacted": "<redacted-safe-authority-drill-action>""#,
        r#""action_reason_redacted": "/Users/mymac/pilot/keys/halt-authority.json""#,
    );
    let path = write_report("authority_path", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "unredacted authority path must fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}
