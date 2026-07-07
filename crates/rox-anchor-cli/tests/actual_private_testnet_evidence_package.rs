//! RO:WHAT — Tests BUILD_PLAN4 Phase 14 actual private testnet evidence package boundary.
//! RO:WHY — Keeps the final private testnet evidence package audit-ready, redacted, local-only, and non-authorizing.
//! RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md and scripts/check_actual_private_testnet_evidence_package.sh.
//! RO:INVARIANTS — evidence package only; no runtime authorization; no wallet/ledger/bridge authority; no settlement; no real ROC mutation; no finality.
//! RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, transaction submission, mint, burn, settlement, paid unlock, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_evidence_package.

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
        .arg(root.join("scripts/check_actual_private_testnet_evidence_package.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("Phase 14 evidence package checker should execute");

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

fn write_package(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_phase14_evidence_package_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp package should write");
    path
}

fn template_package() -> String {
    let (ok, output) = run_script(&["--template-package", "testnet"]);
    assert!(ok, "template should print:\n{output}");
    output
}

fn check_package(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-package", &path_arg])
}

#[test]
fn phase14_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output.contains(
        "BUILD_PLAN4 Phase 14 actual private testnet evidence package documentation checks passed"
    ));
    assert!(output.contains("redacted, evidence-index-only, private-testnet"));
    assert!(output.contains("runtime authorization"));
    assert!(output.contains("real ROC burn/release/mutation"));
}

#[test]
fn phase14_preflight_is_local_only_and_non_authorizing() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass:\n{output}");
    assert!(output
        .contains("BUILD_PLAN4 Phase 14 actual private testnet evidence package preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, load authority keys, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, mutate ROC, unlock paid content, or grant runtime authority"));
}

#[test]
fn phase14_template_package_covers_required_surfaces_without_runtime_claims() {
    let output = template_package();

    assert!(output.contains("BUILD_PLAN4 Phase 14"));
    assert!(output.contains("actual_private_testnet_evidence_package"));
    assert!(output.contains("rox-anchor.actual-private-testnet-evidence-package.v1"));
    assert!(output.contains(r#""evidence_index_status": "audit_ready""#));
    assert!(output.contains(r#""build_artifact_manifest_status": "linked""#));
    assert!(output.contains(r#""deploy_receipt_status": "linked_or_not_performed""#));
    assert!(output.contains(r#""test_only_mint_init_status": "linked_or_not_performed""#));
    assert!(output.contains(r#""read_only_rpc_evidence_status": "linked_or_not_performed""#));
    assert!(output.contains(r#""simulation_receipts_status": "linked_or_not_performed""#));
    assert!(output.contains(r#""roc_to_rox_receipts_status": "linked_or_not_performed""#));
    assert!(output.contains(r#""rox_to_roc_receipts_status": "linked_or_not_performed""#));
    assert!(output.contains(r#""receipt_ledger_status": "linked""#));
    assert!(output.contains(r#""negative_drill_receipts_status": "linked""#));
    assert!(output.contains(r#""halt_recovery_reports_status": "linked""#));
    assert!(output.contains(r#""authority_reports_status": "linked""#));
    assert!(output.contains(r#""rustyonions_handoff_status": "linked""#));
    assert!(output.contains(r#""crablink_display_status": "linked""#));
    assert!(output.contains(r#""redaction_status": "redacted""#));
    assert!(output.contains(r#""private_testnet_only": true"#));
    assert!(output.contains(r#""test_only_assets_only": true"#));
    assert!(output.contains(r#""evidence_package_only": true"#));

    for forbidden_true in [
        r#""runtime_authorization": true"#,
        r#""wallet_authority": true"#,
        r#""ledger_authority": true"#,
        r#""bridge_authority": true"#,
        r#""transaction_submission": true"#,
        r#""public_launch_authorized": true"#,
        r#""mainnet_authorized": true"#,
        r#""production_bridge_settlement": true"#,
        r#""public_rox_mint_burn": true"#,
        r#""real_roc_burn": true"#,
        r#""real_roc_release": true"#,
        r#""real_roc_mutation": true"#,
        r#""final_settlement": true"#,
        r#""finality_claim": true"#,
    ] {
        assert!(
            !output.contains(forbidden_true),
            "template contains {forbidden_true}"
        );
    }
}

#[test]
fn phase14_package_checker_accepts_audit_ready_evidence_index() {
    let package = template_package();
    let path = write_package("audit_ready", &package);
    let (ok, output) = check_package(&path);

    assert!(ok, "audit-ready package should pass:\n{output}");
    assert!(output.contains("package is evidence-index-only and redacted"));
    assert!(output.contains("package covers build, deploy, initialization"));
    assert!(output.contains("package validates operation ID, idempotency key, receipt ID"));
    assert!(output.contains("package rejects runtime authorization"));
}

#[test]
fn phase14_package_checker_accepts_incomplete_or_quarantined_non_success_index() {
    let package = template_package()
        .replace(
            r#""evidence_index_status": "audit_ready""#,
            r#""evidence_index_status": "incomplete""#,
        )
        .replace(
            r#""deploy_receipt_status": "linked_or_not_performed""#,
            r#""deploy_receipt_status": "failed_safe""#,
        )
        .replace(
            r#""read_only_rpc_evidence_status": "linked_or_not_performed""#,
            r#""read_only_rpc_evidence_status": "missing""#,
        )
        .replace(
            r#""operation_id_linkage_status": "validated""#,
            r#""operation_id_linkage_status": "incomplete""#,
        )
        .replace(
            r#""redaction_status": "redacted""#,
            r#""redaction_status": "incomplete""#,
        );
    let path = write_package("incomplete", &package);
    let (ok, output) = check_package(&path);

    assert!(ok, "incomplete non-success package should pass:\n{output}");
    assert!(output.contains("evidence_index_status is valid: incomplete"));
    assert!(output.contains("deploy_receipt_status is valid: failed_safe"));
    assert!(output.contains("redaction_status is valid: incomplete"));

    let package = template_package()
        .replace(
            r#""evidence_index_status": "audit_ready""#,
            r#""evidence_index_status": "quarantined""#,
        )
        .replace(
            r#""receipt_ledger_status": "linked""#,
            r#""receipt_ledger_status": "quarantined""#,
        )
        .replace(
            r#""negative_drill_receipts_status": "linked""#,
            r#""negative_drill_receipts_status": "quarantined""#,
        )
        .replace(
            r#""receipt_id_linkage_status": "validated""#,
            r#""receipt_id_linkage_status": "quarantined""#,
        );
    let path = write_package("quarantined", &package);
    let (ok, output) = check_package(&path);

    assert!(ok, "quarantined package should pass:\n{output}");
    assert!(output.contains("evidence_index_status is valid: quarantined"));
    assert!(output.contains("receipt_ledger_status is valid: quarantined"));
}

#[test]
fn phase14_package_rejects_mainnet_runtime_authority_settlement_or_real_roc_mutation() {
    let package =
        template_package().replace(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#);
    let path = write_package("mainnet", &package);
    let (ok, output) = check_package(&path);

    assert!(!ok, "mainnet package must fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));

    for (name, from, to) in [
        (
            "runtime_authorization",
            r#""runtime_authorization": false"#,
            r#""runtime_authorization": true"#,
        ),
        (
            "wallet_authority",
            r#""wallet_authority": false"#,
            r#""wallet_authority": true"#,
        ),
        (
            "transaction_submission",
            r#""transaction_submission": false"#,
            r#""transaction_submission": true"#,
        ),
        (
            "production_settlement",
            r#""production_bridge_settlement": false"#,
            r#""production_bridge_settlement": true"#,
        ),
        (
            "real_roc_mutation",
            r#""real_roc_mutation": false"#,
            r#""real_roc_mutation": true"#,
        ),
        (
            "finality",
            r#""finality_claim": false"#,
            r#""finality_claim": true"#,
        ),
    ] {
        let package = template_package().replace(from, to);
        let path = write_package(name, &package);
        let (ok, output) = check_package(&path);

        assert!(!ok, "{name} true must fail:\n{output}");
        assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
    }
}

#[test]
fn phase14_package_rejects_unredacted_paths_or_secret_markers() {
    let package = template_package().replace(
        r#""package_id": "actual-private-testnet-evidence-package-0001""#,
        r#""package_id": "/Users/mymac/private/evidence-package.json""#,
    );
    let path = write_package("path_leak", &package);
    let (ok, output) = check_package(&path);

    assert!(!ok, "unredacted path package must fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));

    let package = template_package().replace(
        r#""package_id": "actual-private-testnet-evidence-package-0001""#,
        r#""package_id": "api-key=abc123""#,
    );
    let path = write_package("token_leak", &package);
    let (ok, output) = check_package(&path);

    assert!(!ok, "provider token package must fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn phase14_doc_keeps_evidence_package_as_audit_index_only() {
    let root = repo_root();
    let doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md"))
            .expect("evidence package doc should be readable");

    for marker in [
        "ROX Anchor BUILD_PLAN4 Phase 14",
        "Actual Private Testnet Evidence Package",
        "build artifact manifest",
        "deployment receipt or safe failed-deployment receipt",
        "test-only mint/config initialization receipt",
        "read-only RPC evidence receipt",
        "negative drill failure receipts",
        "RustyOnions dry-run handoff report",
        "CrabLink display-only status report",
        "No runtime authorization.",
        "No transaction submission.",
        "No production bridge settlement.",
        "No real internal ROC mutation.",
        "The actual private testnet evidence package is an audit index only.",
    ] {
        assert!(doc.contains(marker), "doc missing marker: {marker}");
    }

    for forbidden in [
        r#""runtime_authorization": true"#,
        r#""wallet_authority": true"#,
        r#""ledger_authority": true"#,
        r#""bridge_authority": true"#,
        r#""transaction_submission": true"#,
        r#""production_bridge_settlement": true"#,
        r#""real_roc_mutation": true"#,
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
