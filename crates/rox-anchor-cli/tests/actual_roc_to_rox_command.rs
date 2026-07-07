//! RO:WHAT — Tests BUILD_PLAN4 Phase 7 CLI-facing ROC-to-ROX runbook/checker boundary.
//! RO:WHY — Keeps actual forward private testnet send explicit, capped, redacted, and non-production.
//! RO:INTERACTS — scripts/check_actual_roc_to_rox_private_testnet_run.sh and docs/pilot.
//! RO:INVARIANTS — external config, explicit approval, one operation, amount cap, shadow ROC only, test-only ROX.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_roc_to_rox_command.

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
        .arg(root.join("scripts/check_actual_roc_to_rox_private_testnet_run.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("ROC-to-ROX checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

#[test]
fn actual_roc_to_rox_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 7 ROC-to-ROX documentation checks passed"));
    assert!(output.contains("actual capped ROC-to-ROX private testnet runbook exists"));
    assert!(
        output.contains("shadow-ROC-only, test-only-ROX, redacted, capped, non-mainnet boundaries")
    );
}

#[test]
fn actual_roc_to_rox_preflight_is_local_only_and_non_submitting() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass after anchor build:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 7 ROC-to-ROX preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, mint, burn, settle, or mutate ROC"));
}

#[test]
fn actual_roc_to_rox_templates_are_redacted_and_non_production() {
    for command in [
        "--template-send-sent",
        "--template-send-blocked",
        "--template-readback",
    ] {
        let (ok, output) = run_script(&[command, "testnet"]);

        assert!(ok, "{command} should print:\n{output}");
        assert!(output.contains("BUILD_PLAN4 Phase 7"));
        assert!(output.contains("<redacted-"));
        assert!(output.contains(r#""direction": "roc_to_rox""#));
        assert!(output.contains(r#""real_roc_burn": false"#));
        assert!(output.contains(r#""real_roc_mutation": false"#));
        assert!(output.contains(r#""finality_claim": false"#));

        assert!(!output.contains("/Users/"));
        assert!(!output.contains("/home/"));
        assert!(!output.contains("api-key="));
        assert!(!output.contains("access_token="));
        assert!(!output.contains(r#""mainnet_authorized": true"#));
        assert!(!output.contains(r#""real_roc_mutation": true"#));
        assert!(!output.contains(r#""finality_claim": true"#));
    }
}

#[test]
fn actual_roc_to_rox_runbook_uses_external_config_explicit_approval_and_caps() {
    let root = repo_root();
    let doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_ROC_TO_ROX_PRIVATE_TESTNET_RUN.md"))
            .expect("ROC-to-ROX runbook should be readable");

    assert!(doc.contains("cargo run -p rox-anchor-cli -- pilot roc-to-rox"));
    assert!(doc.contains("--config /external/private/<redacted-private-testnet-config>"));
    assert!(doc.contains("--receipt-out /external/private/<redacted-receipts-dir>/roc-to-rox-send.pilot-receipt.json"));
    assert!(doc.contains("--operator-approval \"I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND\""));
    assert!(doc.contains("--max-operations 1"));
    assert!(doc.contains("--max-amount-minor 1"));
    assert!(doc.contains("The shadow ROC burn intent is not a real ROC burn."));
    assert!(doc.contains("svc-wallet -> ron-ledger"));
    assert!(doc.contains("No real ROC burn."));
    assert!(doc.contains("No real internal ROC mutation."));

    assert!(!doc.contains("/Users/"));
    assert!(!doc.contains("/home/"));
    assert!(!doc.contains("api-key="));
    assert!(!doc.contains("access_token="));
}
