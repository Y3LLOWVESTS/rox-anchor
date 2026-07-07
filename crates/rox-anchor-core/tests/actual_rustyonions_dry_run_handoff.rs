//! RO:WHAT — Tests BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff report shape.
//! RO:WHY — Preserves the svc-wallet -> ron-ledger real ROC boundary while allowing ROX Anchor to report dry-run intent/status.
//! RO:INTERACTS — scripts/check_actual_rustyonions_dry_run_handoff.sh.
//! RO:INVARIANTS — dry-run only; no svc-wallet mutation; no ron-ledger mutation; no real ROC burn/release/mutation.
//! RO:SECURITY — local file checks only; no live RPC, wallet call, ledger call, signer load, signing, submission, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-core --test actual_rustyonions_dry_run_handoff.

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
        "rox_anchor_phase12_rustyonions_handoff_{}_{}_{}.json",
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
fn roc_to_rox_handoff_is_shadow_burn_intent_only_without_wallet_or_ledger_mutation() {
    let report = template_report("roc_to_rox");

    assert!(report.contains(r#""direction": "roc_to_rox""#));
    assert!(report.contains(r#""shadow_roc_burn_intent_only": true"#));
    assert!(report.contains(r#""internal_roc_release_intent_only": false"#));
    assert!(report.contains(r#""rustyonions_target_boundary": "svc-wallet -> ron-ledger""#));
    assert!(report.contains(r#""dry_run_only": true"#));
    assert!(report.contains(r#""svc_wallet_mutation": false"#));
    assert!(report.contains(r#""ron_ledger_mutation": false"#));
    assert!(report.contains(r#""real_roc_burn": false"#));
    assert!(report.contains(r#""real_roc_mutation": false"#));

    let path = write_report("roc_to_rox", &report);
    let (ok, output) = check_report(&path);

    assert!(ok, "roc_to_rox handoff report should pass:\n{output}");
    assert!(output.contains("svc-wallet -> ron-ledger remains the target boundary"));
}

#[test]
fn rox_to_roc_handoff_is_release_intent_only_without_real_roc_release() {
    let report = template_report("rox_to_roc");

    assert!(report.contains(r#""direction": "rox_to_roc""#));
    assert!(report.contains(r#""shadow_roc_burn_intent_only": false"#));
    assert!(report.contains(r#""internal_roc_release_intent_only": true"#));
    assert!(report.contains(r#""dry_run_only": true"#));
    assert!(report.contains(r#""real_roc_release": false"#));
    assert!(report.contains(r#""real_roc_mutation": false"#));
    assert!(report.contains(r#""production_bridge_settlement": false"#));

    let path = write_report("rox_to_roc", &report);
    let (ok, output) = check_report(&path);

    assert!(ok, "rox_to_roc handoff report should pass:\n{output}");
    assert!(output.contains("handoff is dry-run only and redacted"));
}

#[test]
fn handoff_rejects_mainnet_wallet_ledger_or_real_roc_mutation_claims() {
    let report = template_report("roc_to_rox")
        .replace(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#);
    let path = write_report("mainnet", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "mainnet handoff must fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));

    for (name, needle) in [
        ("svc_wallet_mutation", r#""svc_wallet_mutation": true"#),
        ("ron_ledger_mutation", r#""ron_ledger_mutation": true"#),
        ("real_roc_mutation", r#""real_roc_mutation": true"#),
    ] {
        let report =
            template_report("rox_to_roc").replace(&needle.replace(": true", ": false"), needle);
        let path = write_report(name, &report);
        let (ok, output) = check_report(&path);

        assert!(!ok, "{name} true must fail:\n{output}");
        assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
    }
}

#[test]
fn handoff_rejects_unknown_direction_or_wrong_target_boundary() {
    let (ok, output) = run_script(&["--template-report", "testnet", "sideways"]);

    assert!(!ok, "unknown direction should fail:\n{output}");
    assert!(output.contains("direction must be roc_to_rox or rox_to_roc"));

    let report = template_report("roc_to_rox").replace(
        r#""rustyonions_target_boundary": "svc-wallet -> ron-ledger""#,
        r#""rustyonions_target_boundary": "rox-anchor -> ron-ledger""#,
    );
    let path = write_report("wrong_boundary", &report);
    let (ok, output) = check_report(&path);

    assert!(!ok, "wrong target boundary must fail:\n{output}");
    assert!(output.contains("rustyonions_target_boundary expected"));
}
