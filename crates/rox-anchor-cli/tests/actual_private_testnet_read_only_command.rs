//! RO:WHAT — Tests BUILD_PLAN4 Phase 5 CLI-facing read-only evidence runbook/checker boundary.
//! RO:WHY — Keeps actual private testnet read-only evidence non-submitting and non-authorizing.
//! RO:INTERACTS — scripts/check_actual_private_testnet_read_only_evidence.sh and docs/pilot.
//! RO:INVARIANTS — docs/checker/templates must preserve read-only, external-config, redacted, non-mainnet boundaries.
//! RO:SECURITY — no live RPC, wallet load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_read_only_command.

use std::{path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let output = Command::new("bash")
        .arg(root.join("scripts/check_actual_private_testnet_read_only_evidence.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("read-only evidence checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn actual_private_testnet_read_only_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(
        output.contains("BUILD_PLAN4 Phase 5 read-only RPC evidence documentation checks passed")
    );
    assert!(output.contains("actual private testnet read-only RPC evidence runbook exists"));
    assert!(output.contains("read-only, redacted, non-submitting, non-mainnet boundaries"));
    assert!(output.contains("separates read-only evidence from transaction submission"));
}

#[test]
fn actual_private_testnet_read_only_preflight_is_local_only_and_non_submitting() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass after anchor build:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 5 read-only RPC evidence preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a wallet, initialize, mint, burn, settle, or mutate ROC"));
}

#[test]
fn actual_private_testnet_read_only_templates_are_redacted_and_non_authorizing() {
    for command in ["--template-verified", "--template-failed"] {
        let (ok, output) = run_script(&[command, "testnet"]);

        assert!(ok, "{command} should print:\n{output}");
        assert!(output.contains("rox-anchor.actual-private-testnet-read-only-evidence.v1"));
        assert!(output.contains("private_testnet_read_only_rpc_evidence_receipt"));
        assert!(output.contains("<redacted-"));
        assert!(output.contains(r#""read_only_rpc": true"#));
        assert!(output.contains(r#""transaction_submission": false"#));
        assert!(output.contains(r#""wallet_loaded": false"#));
        assert!(output.contains(r#""signature_generated": false"#));
        assert!(output.contains(r#""public_launch_authorized": false"#));
        assert!(output.contains(r#""mainnet_authorized": false"#));
        assert!(output.contains(r#""production_bridge_settlement": false"#));
        assert!(output.contains(r#""public_rox_mint_burn": false"#));
        assert!(output.contains(r#""real_roc_mutation": false"#));
        assert!(output.contains(r#""finality_claim": false"#));

        assert!(!output.contains("/Users/"));
        assert!(!output.contains("/home/"));
        assert!(!output.contains("api-key="));
        assert!(!output.contains("access_token="));
        assert!(!output.contains(r#""transaction_submission": true"#));
        assert!(!output.contains(r#""finality_claim": true"#));
    }
}

#[test]
fn actual_private_testnet_read_only_runbook_uses_external_config_and_receipt_paths() {
    let root = repo_root();
    let doc = std::fs::read_to_string(
        root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_READ_ONLY_EVIDENCE.md"),
    )
    .expect("read-only evidence doc should be readable");

    assert!(doc.contains("cargo run -p rox-anchor-cli -- pilot proof read-only"));
    assert!(doc.contains("--config /external/private/<redacted-private-testnet-config>"));
    assert!(doc.contains("--receipt-out /external/private/<redacted-receipts-dir>/read-only-evidence.pilot-receipt.json"));
    assert!(doc.contains("The repo patch itself does not call RPC."));
    assert!(doc.contains("No transaction submission."));
    assert!(doc.contains("No real internal ROC release."));

    assert!(!doc.contains("/Users/"));
    assert!(!doc.contains("/home/"));
    assert!(!doc.contains("api-key="));
    assert!(!doc.contains("access_token="));
}
