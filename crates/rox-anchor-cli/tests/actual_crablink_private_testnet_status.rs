//! RO:WHAT — Tests BUILD_PLAN4 Phase 13 CrabLink display-only private testnet status.
//! RO:WHY — Keeps CrabLink-facing private testnet status display-only and non-authorizing.
//! RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md and scripts/check_actual_crablink_private_testnet_status.sh.
//! RO:INVARIANTS — no client authority; no wallet/ledger/bridge authority; no Solana submit; no ROX mint/burn authority; no paid content unlock; no real ROC mutation.
//! RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, transaction submission, mint, burn, settlement, paid unlock, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_crablink_private_testnet_status.

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
        .arg(root.join("scripts/check_actual_crablink_private_testnet_status.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("Phase 13 CrabLink status checker should execute");

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

fn write_status(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_phase13_crablink_status_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp status should write");
    path
}

fn template_status() -> String {
    let (ok, output) = run_script(&["--template-status", "testnet"]);
    assert!(ok, "template should print:\n{output}");
    output
}

fn check_status(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-status", &path_arg])
}

#[test]
fn phase13_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output
        .contains("BUILD_PLAN4 Phase 13 CrabLink display-only status documentation checks passed"));
    assert!(output.contains("backend-derived, display-only, test-only"));
    assert!(output.contains("client authority"));
    assert!(output.contains("paid content unlock"));
    assert!(output.contains("real ROC mutation"));
}

#[test]
fn phase13_preflight_is_local_only_and_non_authorizing() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 13 CrabLink display-only status preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, mutate ROC, unlock paid content, or grant client authority"));
}

#[test]
fn phase13_template_status_is_backend_derived_display_only_and_non_authorizing() {
    let output = template_status();

    assert!(output.contains("BUILD_PLAN4 Phase 13"));
    assert!(output.contains("actual_crablink_private_testnet_display_status"));
    assert!(output.contains("rox-anchor.actual-crablink-private-testnet-status.v1"));
    assert!(output.contains(r#""display_status": "display_only""#));
    assert!(output.contains(r#""backend_derived": true"#));
    assert!(output.contains(r#""display_only": true"#));
    assert!(output.contains(r#""private_testnet_only": true"#));
    assert!(output.contains(r#""test_only_assets_only": true"#));
    assert!(output.contains(r#""dry_run_only": true"#));
    assert!(output.contains(r#""operator_report_redacted": true"#));
    assert!(output.contains(r#""test_only_asset_label": "TEST-ONLY ROX""#));
    assert!(output.contains(r#""private_testnet_label": "PRIVATE TESTNET STATUS""#));

    for forbidden_true in [
        r#""client_authority": true"#,
        r#""wallet_authority": true"#,
        r#""ledger_authority": true"#,
        r#""bridge_authority": true"#,
        r#""solana_submit_command_available": true"#,
        r#""rox_mint_burn_authority": true"#,
        r#""paid_content_unlock": true"#,
        r#""real_roc_burn": true"#,
        r#""real_roc_release": true"#,
        r#""real_roc_mutation": true"#,
        r#""production_bridge_settlement": true"#,
        r#""final_settlement": true"#,
        r#""finality_claim": true"#,
    ] {
        assert!(
            !output.contains(forbidden_true),
            "template contains {forbidden_true}"
        );
    }

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
}

#[test]
fn phase13_status_checker_accepts_display_only_status() {
    let status = template_status();
    let path = write_status("display_only", &status);
    let (ok, output) = check_status(&path);

    assert!(ok, "display-only status should pass:\n{output}");
    assert!(output.contains("status is backend-derived and display-only"));
    assert!(output.contains("status labels test-only assets and private testnet evidence"));
    assert!(output.contains("status rejects client authority"));
}

#[test]
fn phase13_status_checker_accepts_blocked_or_unavailable_display_without_authority() {
    let status = template_status()
        .replace(
            r#""display_status": "display_only""#,
            r#""display_status": "blocked""#,
        )
        .replace(
            r#""proof_status": "accepted""#,
            r#""proof_status": "blocked""#,
        )
        .replace(
            r#""read_only_rpc_status": "verified""#,
            r#""read_only_rpc_status": "unavailable""#,
        )
        .replace(
            r#""receipt_status": "linked""#,
            r#""receipt_status": "quarantined""#,
        )
        .replace(
            r#""rustyonions_handoff_status": "dry_run_recorded""#,
            r#""rustyonions_handoff_status": "blocked""#,
        );
    let path = write_status("blocked", &status);
    let (ok, output) = check_status(&path);

    assert!(ok, "blocked display-only status should pass:\n{output}");
    assert!(output.contains("display_status is valid: blocked"));
    assert!(output.contains("rustyonions_handoff_status is valid: blocked"));
    assert!(!status.contains(r#""client_authority": true"#));
    assert!(!status.contains(r#""finality_claim": true"#));
}

#[test]
fn phase13_status_rejects_mainnet_client_authority_or_paid_unlock() {
    let status =
        template_status().replace(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#);
    let path = write_status("mainnet", &status);
    let (ok, output) = check_status(&path);

    assert!(!ok, "mainnet status must fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));

    for (name, from, to) in [
        (
            "client_authority",
            r#""client_authority": false"#,
            r#""client_authority": true"#,
        ),
        (
            "solana_submit",
            r#""solana_submit_command_available": false"#,
            r#""solana_submit_command_available": true"#,
        ),
        (
            "paid_unlock",
            r#""paid_content_unlock": false"#,
            r#""paid_content_unlock": true"#,
        ),
        (
            "real_roc_mutation",
            r#""real_roc_mutation": false"#,
            r#""real_roc_mutation": true"#,
        ),
    ] {
        let status = template_status().replace(from, to);
        let path = write_status(name, &status);
        let (ok, output) = check_status(&path);

        assert!(!ok, "{name} true must fail:\n{output}");
        assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
    }
}

#[test]
fn phase13_doc_keeps_crablink_status_as_display_only_boundary() {
    let root = repo_root();
    let doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md"))
            .expect("CrabLink status doc should be readable");

    for marker in [
        "ROX Anchor BUILD_PLAN4 Phase 13",
        "CrabLink Display-Only Private Testnet Status",
        "backend-derived",
        "display-only",
        "No Solana submit commands in CrabLink.",
        "No ROX mint/burn authority in CrabLink.",
        "No paid content unlock from private testnet status.",
        "No wallet authority.",
        "No ledger authority.",
        "No bridge authority.",
        "No real ROC mutation.",
        "No production bridge settlement.",
        "No final settlement.",
        "CrabLink status remains display-only.",
    ] {
        assert!(doc.contains(marker), "doc missing marker: {marker}");
    }

    for forbidden in [
        r#""backend_derived": false"#,
        r#""display_only": false"#,
        r#""client_authority": true"#,
        r#""wallet_authority": true"#,
        r#""ledger_authority": true"#,
        r#""bridge_authority": true"#,
        r#""solana_submit_command_available": true"#,
        r#""rox_mint_burn_authority": true"#,
        r#""paid_content_unlock": true"#,
        r#""real_roc_mutation": true"#,
        r#""production_bridge_settlement": true"#,
        r#""final_settlement": true"#,
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
