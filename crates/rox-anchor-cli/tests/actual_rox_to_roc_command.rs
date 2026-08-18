//! RO:WHAT — Tests BUILD_PLAN4 Phase 8 CLI-facing ROX-to-ROC runbook/checker boundary.
//! RO:WHY — Keeps actual reverse private testnet action explicit, capped, redacted, and non-production.
//! RO:INTERACTS — scripts/check_actual_rox_to_roc_private_testnet_run.sh and docs/pilot.
//! RO:INVARIANTS — external config, explicit approval, one operation, amount cap, test-only ROX, dry-run ROC release intent.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, real ROC release, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_rox_to_roc_command.

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
        .arg(root.join("scripts/check_actual_rox_to_roc_private_testnet_run.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("ROX-to-ROC checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

#[test]
fn actual_rox_to_roc_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 8 ROX-to-ROC documentation checks passed"));
    assert!(output.contains("actual capped ROX-to-ROC private testnet runbook exists"));
    assert!(output.contains("dry-run-ROC-release-intent-only"));
}

#[test]
fn actual_rox_to_roc_preflight_is_local_only_and_non_submitting() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass after anchor build:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 8 ROX-to-ROC preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, mint, burn, settle, release ROC, or mutate ROC"));
}

#[test]
fn actual_rox_to_roc_templates_are_redacted_and_non_production() {
    for command in [
        "--template-send-sent",
        "--template-send-blocked",
        "--template-readback",
    ] {
        let (ok, output) = run_script(&[command, "testnet"]);

        assert!(ok, "{command} should print:\n{output}");
        assert!(output.contains("BUILD_PLAN4 Phase 8"));
        assert!(output.contains("<redacted-"));
        assert!(output.contains(r#""direction": "rox_to_roc""#));
        assert!(output.contains(r#""internal_roc_release_intent_only": true"#));
        assert!(output.contains(r#""real_roc_release": false"#));
        assert!(output.contains(r#""real_roc_mutation": false"#));
        assert!(output.contains(r#""finality_claim": false"#));

        assert!(!output.contains("/Users/"));
        assert!(!output.contains("/home/"));
        assert!(!output.contains("api-key="));
        assert!(!output.contains("access_token="));
        assert!(!output.contains(r#""mainnet_authorized": true"#));
        assert!(!output.contains(r#""real_roc_release": true"#));
        assert!(!output.contains(r#""real_roc_mutation": true"#));
    }
}

#[test]
fn actual_rox_to_roc_runbook_uses_external_config_explicit_approval_and_caps() {
    let root = repo_root();
    let doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_ROX_TO_ROC_PRIVATE_TESTNET_RUN.md"))
            .expect("ROX-to-ROC runbook should be readable");

    assert!(doc.contains("cargo run -p rox-anchor-cli -- pilot rox-to-roc"));
    assert!(doc.contains("--config /external/private/<redacted-private-testnet-config>"));
    assert!(doc.contains(
        "--receipt-out /external/private/<redacted-receipts-dir>/rox-to-roc.pilot-receipt.json"
    ));
    assert!(
        doc.contains("--operator-approval \"I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN\"")
    );
    assert!(doc.contains("--max-operations 1"));
    assert!(doc.contains("--max-amount-minor 1"));
    assert!(doc.contains("The internal ROC release intent is dry-run only."));
    assert!(doc.contains("ROX Anchor must not release real ROC."));
    assert!(doc.contains("svc-wallet -> ron-ledger"));
    assert!(doc.contains("No real ROC release."));
    assert!(doc.contains("No real internal ROC mutation."));

    assert!(!doc.contains("/Users/"));
    assert!(!doc.contains("/home/"));
    assert!(!doc.contains("api-key="));
    assert!(!doc.contains("access_token="));
}
