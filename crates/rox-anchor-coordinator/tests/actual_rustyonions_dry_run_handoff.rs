//! RO:WHAT — Tests BUILD_PLAN4 Phase 12 coordinator-facing RustyOnions dry-run handoff status.
//! RO:WHY — Ensures accepted private testnet evidence can become only dry-run RustyOnions handoff status, never wallet/ledger mutation.
//! RO:INTERACTS — scripts/check_actual_rustyonions_dry_run_handoff.sh.
//! RO:INVARIANTS — accepted coordinator status is still dry-run-only; blocked/quarantined handoffs remain non-mutating.
//! RO:SECURITY — local file checks only; no live RPC, wallet call, ledger call, signer load, signing, submission, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test actual_rustyonions_dry_run_handoff.

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
        .arg(root.join("scripts/check_actual_rustyonions_dry_run_handoff.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("Phase 12 RustyOnions dry-run handoff checker should execute");

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
        "rox_anchor_coordinator_phase12_handoff_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp report should write");
    path
}

fn template_report(direction: &str) -> String {
    let (ok, output) = run_script(&["--template-report", "testnet", direction]);
    assert!(ok, "template should print for {direction}:\n{output}");
    output
}

fn check_report(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-report", &path_arg])
}

#[test]
fn accepted_coordinator_handoff_remains_dry_run_only() {
    for direction in ["roc_to_rox", "rox_to_roc"] {
        let report = template_report(direction);

        assert!(report.contains(r#""proof_review_status": "accepted""#));
        assert!(report.contains(r#""coordinator_decision_status": "accepted""#));
        assert!(report.contains(r#""relayer_status": "dry_run_only""#));
        assert!(report.contains(r#""rustyonions_handoff_status": "dry_run_recorded""#));
        assert!(report.contains(r#""dry_run_only": true"#));
        assert!(report.contains(r#""svc_wallet_mutation": false"#));
        assert!(report.contains(r#""ron_ledger_mutation": false"#));

        let path = write_report(direction, &report);
        let (ok, output) = check_report(&path);

        assert!(
            ok,
            "{direction} accepted dry-run handoff should pass:\n{output}"
        );
        assert!(output.contains("report rejects wallet mutation"));
    }
}

#[test]
fn blocked_coordinator_handoff_can_be_reported_without_mutation_or_finality() {
    let report = template_report("roc_to_rox")
        .replace(
            r#""coordinator_decision_status": "accepted""#,
            r#""coordinator_decision_status": "blocked""#,
        )
        .replace(
            r#""relayer_status": "dry_run_only""#,
            r#""relayer_status": "blocked""#,
        )
        .replace(
            r#""rustyonions_handoff_status": "dry_run_recorded""#,
            r#""rustyonions_handoff_status": "blocked""#,
        );
    let path = write_report("blocked", &report);
    let (ok, output) = check_report(&path);

    assert!(ok, "blocked handoff report should pass:\n{output}");
    assert!(output.contains("handoff is dry-run only and redacted"));
    assert!(!report.contains(r#""finality_claim": true"#));
    assert!(!report.contains(r#""real_roc_mutation": true"#));
}

#[test]
fn quarantined_receipt_ledger_handoff_remains_non_mutating() {
    let report = template_report("rox_to_roc")
        .replace(
            r#""source_receipt_ledger_status": "linked""#,
            r#""source_receipt_ledger_status": "quarantined""#,
        )
        .replace(
            r#""source_private_testnet_receipts_status": "redacted_verified""#,
            r#""source_private_testnet_receipts_status": "redacted_quarantined""#,
        )
        .replace(
            r#""rustyonions_handoff_status": "dry_run_recorded""#,
            r#""rustyonions_handoff_status": "quarantined""#,
        );
    let path = write_report("quarantined", &report);
    let (ok, output) = check_report(&path);

    assert!(ok, "quarantined handoff report should pass:\n{output}");
    assert!(output.contains("direction is rox_to_roc"));
    assert!(output.contains("report rejects wallet mutation"));
}

#[test]
fn coordinator_handoff_rejects_promoted_settlement_or_finality_claims() {
    let report = template_report("roc_to_rox").replace(
        r#""production_bridge_settlement": false"#,
        r#""production_bridge_settlement": true"#,
    );
    let path = write_report("settlement", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "settlement claim must fail:\n{output}");
    assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));

    let report = template_report("rox_to_roc")
        .replace(r#""finality_claim": false"#, r#""finality_claim": true"#);
    let path = write_report("finality", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "finality claim must fail:\n{output}");
    assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
}
