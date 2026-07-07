//! RO:WHAT — Tests BUILD_PLAN4 Phase 12 CLI-facing RustyOnions dry-run handoff documentation and templates.
//! RO:WHY — Ensures user/operator status remains displayable without implying wallet truth, ledger truth, settlement, or real ROC mutation.
//! RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md and scripts/check_actual_rustyonions_dry_run_handoff.sh.
//! RO:INVARIANTS — dry-run only; svc-wallet -> ron-ledger remains the future real ROC mutation boundary.
//! RO:SECURITY — local file checks only; no live RPC, wallet call, ledger call, signer load, signing, submission, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_rustyonions_dry_run_status.

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

#[test]
fn phase12_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output
        .contains("BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff documentation checks passed"));
    assert!(output.contains("svc-wallet -> ron-ledger"));
    assert!(output.contains("wallet mutation"));
    assert!(output.contains("ledger mutation"));
    assert!(output.contains("real ROC burn/release"));
}

#[test]
fn phase12_preflight_is_local_only_and_non_mutating() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, load authority keys, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, or mutate ROC"));
}

#[test]
fn phase12_templates_are_redacted_dry_run_only_and_directional() {
    for (direction, required_true, required_false) in [
        (
            "roc_to_rox",
            r#""shadow_roc_burn_intent_only": true"#,
            r#""internal_roc_release_intent_only": false"#,
        ),
        (
            "rox_to_roc",
            r#""internal_roc_release_intent_only": true"#,
            r#""shadow_roc_burn_intent_only": false"#,
        ),
    ] {
        let (ok, output) = run_script(&["--template-report", "testnet", direction]);

        assert!(ok, "template should print for {direction}:\n{output}");
        assert!(output.contains("BUILD_PLAN4 Phase 12"));
        assert!(output.contains("actual_rustyonions_dry_run_handoff_report"));
        assert!(output.contains("rox-anchor.actual-rustyonions-dry-run-handoff.v1"));
        assert!(output.contains(r#""rustyonions_target_boundary": "svc-wallet -> ron-ledger""#));
        assert!(output.contains(r#""dry_run_only": true"#));
        assert!(output.contains(required_true));
        assert!(output.contains(required_false));
        assert!(output.contains(r#""svc_wallet_mutation": false"#));
        assert!(output.contains(r#""ron_ledger_mutation": false"#));
        assert!(output.contains(r#""real_roc_burn": false"#));
        assert!(output.contains(r#""real_roc_release": false"#));
        assert!(output.contains(r#""real_roc_mutation": false"#));
        assert!(output.contains(r#""production_bridge_settlement": false"#));
        assert!(output.contains(r#""finality_claim": false"#));

        assert!(!output.contains("/Users/"));
        assert!(!output.contains("/home/"));
        assert!(!output.contains("api-key="));
        assert!(!output.contains("access_token="));
        assert!(!output.contains(r#""svc_wallet_mutation": true"#));
        assert!(!output.contains(r#""ron_ledger_mutation": true"#));
        assert!(!output.contains(r#""real_roc_mutation": true"#));
    }
}

#[test]
fn phase12_doc_keeps_crablink_and_rustyonions_boundaries_clear() {
    let root = repo_root();
    let doc = std::fs::read_to_string(
        root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md"),
    )
    .expect("RustyOnions handoff doc should be readable");

    for marker in [
        "ROX Anchor BUILD_PLAN4 Phase 12",
        "RustyOnions Dry-Run Handoff Evidence",
        "svc-wallet -> ron-ledger",
        "No real ROC burn.",
        "No real ROC release.",
        "No real internal ROC mutation.",
        "No svc-wallet mutation.",
        "No ron-ledger mutation.",
        "No production bridge settlement.",
        "The RustyOnions handoff remains dry-run only.",
    ] {
        assert!(doc.contains(marker), "doc missing marker: {marker}");
    }

    for forbidden in [
        r#""dry_run_only": false"#,
        r#""svc_wallet_mutation": true"#,
        r#""ron_ledger_mutation": true"#,
        r#""real_roc_burn": true"#,
        r#""real_roc_release": true"#,
        r#""real_roc_mutation": true"#,
        r#""production_bridge_settlement": true"#,
        r#""public_rox_mint_burn": true"#,
        r#""mainnet_authorized": true"#,
        r#""public_launch_authorized": true"#,
        r#""finality_claim": true"#,
        "/Users/",
        "/home/",
        "api-key=",
        "access_token=",
    ] {
        assert!(
            !doc.contains(forbidden),
            "doc contains forbidden marker: {forbidden}"
        );
    }
}
