//! RO:WHAT — Tests BUILD_PLAN4 Phase 15 closeout gate.
//! RO:WHY — Lets BUILD_PLAN4 park cleanly while preserving BUILD_PLAN5 as the future production/mainnet/real-ROC plan.
//! RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md and scripts/check_actual_private_testnet_closeout.sh.
//! RO:INVARIANTS — closeout gate only; no runtime authorization; no wallet/ledger/bridge authority; no public/mainnet/production/real-ROC behavior.
//! RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, transaction submission, mint, burn, settlement, paid unlock, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_closeout.

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
        .arg(root.join("scripts/check_actual_private_testnet_closeout.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("Phase 15 closeout checker should execute");

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

fn write_closeout(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_phase15_closeout_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp closeout should write");
    path
}

fn template_closeout() -> String {
    let (ok, output) = run_script(&["--template-closeout", "testnet"]);
    assert!(ok, "template should print:\n{output}");
    output
}

fn check_closeout(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-closeout", &path_arg])
}

#[test]
fn phase15_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 15 closeout documentation checks passed"));
    assert!(output.contains("closeout-gate-only"));
    assert!(output.contains("BUILD_PLAN5-separate"));
    assert!(output.contains("real ROC burn/release/mutation"));
}

#[test]
fn phase15_preflight_sees_all_prior_actual_phase_checkers() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass:\n{output}");
    assert!(
        output.contains("BUILD_PLAN4 Phase 15 actual private testnet closeout preflight passed")
    );
    assert!(output.contains("Phase 1 through Phase 14 actual private testnet checkers exist"));
    assert!(output.contains("BUILD_PLAN5 exists and remains separate/future"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, load authority keys, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, mutate ROC, unlock paid content, or grant runtime authority"));
}

#[test]
fn phase15_template_closeout_parks_build_plan4_without_authorizing_build_plan5_behavior() {
    let output = template_closeout();

    assert!(output.contains("BUILD_PLAN4 Phase 15"));
    assert!(output.contains("actual_private_testnet_closeout_gate"));
    assert!(output.contains("rox-anchor.actual-private-testnet-closeout.v1"));
    assert!(output.contains(r#""closeout_status": "complete_green_parked""#));
    assert!(output.contains(r#""build_plan4_status": "complete_green_parked""#));
    assert!(output.contains(r#""build_plan5_status": "separate_future_plan""#));
    assert!(output.contains(r#""rustyonions_handoff_status": "dry_run_only""#));
    assert!(output.contains(r#""crablink_display_status": "display_only""#));
    assert!(output.contains(r#""tracked_key_material_status": "none_tracked""#));
    assert!(output.contains(r#""mainnet_behavior_status": "absent""#));
    assert!(output.contains(r#""public_launch_status": "absent""#));
    assert!(output.contains(r#""production_settlement_status": "absent""#));
    assert!(output.contains(r#""real_internal_roc_mutation_status": "absent""#));
    assert!(output.contains(r#""exchange_staking_liquidity_status": "absent""#));
    assert!(output.contains(r#""future_build_plan5_required": true"#));

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
fn phase15_closeout_checker_accepts_complete_green_parked_report() {
    let closeout = template_closeout();
    let path = write_closeout("complete", &closeout);
    let (ok, output) = check_closeout(&path);

    assert!(ok, "complete closeout should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 is allowed to be complete/green/parked"));
    assert!(output.contains("BUILD_PLAN5 remains separate/future"));
    assert!(output.contains("RustyOnions handoff remains dry-run only"));
    assert!(output.contains("closeout rejects runtime authorization"));
}

#[test]
fn phase15_closeout_checker_accepts_incomplete_or_quarantined_non_success_report() {
    let closeout = template_closeout()
        .replace(
            r#""closeout_status": "complete_green_parked""#,
            r#""closeout_status": "incomplete""#,
        )
        .replace(
            r#""build_plan4_status": "complete_green_parked""#,
            r#""build_plan4_status": "incomplete""#,
        )
        .replace(
            r#""actual_private_testnet_checks_status": "passed""#,
            r#""actual_private_testnet_checks_status": "incomplete""#,
        )
        .replace(
            r#""deploy_receipt_status": "linked_or_not_performed""#,
            r#""deploy_receipt_status": "failed_safe""#,
        )
        .replace(
            r#""known_pilot_failures_status": "none_observed_or_documented""#,
            r#""known_pilot_failures_status": "documented""#,
        );
    let path = write_closeout("incomplete", &closeout);
    let (ok, output) = check_closeout(&path);

    assert!(ok, "incomplete non-success closeout should pass:\n{output}");
    assert!(output.contains("closeout_status is valid: incomplete"));
    assert!(output.contains("known_pilot_failures_status is valid: documented"));

    let closeout = template_closeout()
        .replace(
            r#""closeout_status": "complete_green_parked""#,
            r#""closeout_status": "quarantined""#,
        )
        .replace(
            r#""build_plan4_status": "complete_green_parked""#,
            r#""build_plan4_status": "quarantined""#,
        )
        .replace(
            r#""readback_receipts_status": "linked_or_not_performed""#,
            r#""readback_receipts_status": "quarantined""#,
        )
        .replace(
            r#""negative_drill_failure_receipts_status": "linked""#,
            r#""negative_drill_failure_receipts_status": "quarantined""#,
        );
    let path = write_closeout("quarantined", &closeout);
    let (ok, output) = check_closeout(&path);

    assert!(ok, "quarantined closeout should pass:\n{output}");
    assert!(output.contains("closeout_status is valid: quarantined"));
    assert!(output.contains("negative_drill_failure_receipts_status is valid: quarantined"));
}

#[test]
fn phase15_closeout_rejects_mainnet_runtime_authority_or_real_roc_claims() {
    let closeout =
        template_closeout().replace(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#);
    let path = write_closeout("mainnet", &closeout);
    let (ok, output) = check_closeout(&path);

    assert!(!ok, "mainnet closeout must fail:\n{output}");
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
        let closeout = template_closeout().replace(from, to);
        let path = write_closeout(name, &closeout);
        let (ok, output) = check_closeout(&path);

        assert!(!ok, "{name} true must fail:\n{output}");
        assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
    }
}

#[test]
fn phase15_closeout_rejects_unredacted_paths_or_secret_markers() {
    let closeout = template_closeout().replace(
        r#""known_pilot_failures_status": "none_observed_or_documented""#,
        r#""known_pilot_failures_status": "/Users/mymac/private/closeout.json""#,
    );
    let path = write_closeout("path_leak", &closeout);
    let (ok, output) = check_closeout(&path);

    assert!(!ok, "unredacted path closeout must fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));

    let closeout = template_closeout().replace(
        r#""known_pilot_failures_status": "none_observed_or_documented""#,
        r#""known_pilot_failures_status": "api-key=abc123""#,
    );
    let path = write_closeout("token_leak", &closeout);
    let (ok, output) = check_closeout(&path);

    assert!(!ok, "provider token closeout must fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn phase15_doc_keeps_build_plan4_closeout_separate_from_build_plan5() {
    let root = repo_root();
    let doc = std::fs::read_to_string(root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md"))
        .expect("closeout doc should be readable");

    for marker in [
        "ROX Anchor BUILD_PLAN4 Phase 15",
        "BUILD_PLAN4 Closeout Gate",
        "complete / green / parked",
        "Those require BUILD_PLAN5.",
        "No runtime authorization.",
        "No transaction submission.",
        "No production bridge settlement.",
        "No real internal ROC mutation.",
        "No final settlement.",
        "No fake finality.",
        "BUILD_PLAN5 remains separate and future.",
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
