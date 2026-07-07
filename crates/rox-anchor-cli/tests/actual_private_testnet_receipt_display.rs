//! RO:WHAT — Tests BUILD_PLAN4 Phase 9 CLI-facing actual private testnet receipt ledger docs/checker.
//! RO:WHY — Keeps operator-visible receipt reconciliation redacted, deterministic, and non-authorizing.
//! RO:INTERACTS — scripts/check_actual_private_testnet_receipts.sh and docs/pilot.
//! RO:INVARIANTS — receipt ledger is private-testnet-only, readback-aware, no production settlement, no real ROC mutation.
//! RO:SECURITY — no live RPC, signer load, signing, submission, mint, burn, settlement, real ROC release, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_receipt_display.

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
        .arg(root.join("scripts/check_actual_private_testnet_receipts.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("actual private testnet receipt checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

#[test]
fn actual_private_testnet_receipt_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(
        output.contains("BUILD_PLAN4 Phase 9 actual receipt ledger documentation checks passed")
    );
    assert!(output.contains("actual private testnet receipt ledger runbook exists"));
    assert!(output.contains("private testnet reconciliation evidence"));
}

#[test]
fn actual_private_testnet_receipt_preflight_is_local_only_and_non_submitting() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass after anchor build:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 9 actual receipt ledger preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, mint, burn, settle, release ROC, or mutate ROC"));
}

#[test]
fn actual_private_testnet_receipt_templates_are_redacted_and_non_authorizing() {
    for command in ["--template-reconciled", "--template-quarantined"] {
        let (ok, output) = run_script(&[command, "testnet"]);

        assert!(ok, "{command} should print:\n{output}");
        assert!(output.contains("BUILD_PLAN4 Phase 9"));
        assert!(output.contains("actual_private_testnet_receipt_ledger"));
        assert!(output.contains("<redacted-"));
        assert!(output.contains(r#""private_testnet_only": true"#));
        assert!(output.contains(r#""test_only_assets_only": true"#));
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
}

#[test]
fn actual_private_testnet_receipt_runbook_links_phase3_through_phase8_without_settlement_claims() {
    let root = repo_root();
    let doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_RECEIPT_LEDGER.md"))
            .expect("actual receipt ledger runbook should be readable");

    assert!(doc.contains("actual_private_testnet_deploy_receipt"));
    assert!(doc.contains("actual_test_only_mint_init_receipt"));
    assert!(doc.contains("private_testnet_read_only_rpc_evidence_receipt"));
    assert!(doc.contains("actual_private_testnet_simulation_receipt"));
    assert!(doc.contains("actual_roc_to_rox_capped_send_receipt"));
    assert!(doc.contains("actual_roc_to_rox_readback_receipt"));
    assert!(doc.contains("actual_rox_to_roc_capped_send_receipt"));
    assert!(doc.contains("actual_rox_to_roc_readback_receipt"));
    assert!(doc.contains("dry_run_internal_roc_release_intent_receipt"));

    assert!(doc.contains("No real ROC release."));
    assert!(doc.contains("No real internal ROC mutation."));
    assert!(doc.contains("No production bridge settlement."));
    assert!(doc.contains("No fake finality."));
    assert!(doc.contains("The ledger is not runtime authorization."));
    assert!(doc.contains("The ledger is not production settlement."));

    assert!(!doc.contains("/Users/"));
    assert!(!doc.contains("/home/"));
    assert!(!doc.contains("api-key="));
    assert!(!doc.contains("access_token="));
}
