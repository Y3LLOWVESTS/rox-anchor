//! RO:WHAT — Tests BUILD_PLAN4 Phase 11 CLI-facing halt/recovery/authority drill reports.
//! RO:WHY — Keeps operator reports redacted, deterministic, local-only, and non-authorizing.
//! RO:INTERACTS — docs/pilot Phase 11 runbooks and scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh.
//! RO:INVARIANTS — no mainnet, no public launch, no key loading, no key rotation execution, no upgrade authority change, no real ROC mutation.
//! RO:SECURITY — local file checks only; no live RPC, wallet load, authority key load, signing, submission, upgrade, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_drill_reports.

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
        .arg(root.join("scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("Phase 11 drill checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

#[test]
fn phase11_docs_checker_accepts_current_repo_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(
        output.contains("BUILD_PLAN4 Phase 11 halt/recovery/authority documentation checks passed")
    );
    assert!(output.contains("halt/recovery and authority drill runbooks exist"));
    assert!(output.contains("key loading"));
    assert!(output.contains("real ROC mutation"));
}

#[test]
fn phase11_preflight_is_local_only_and_non_submitting() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--preflight", &root_arg, "testnet"]);

    assert!(ok, "preflight should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 11 halt/recovery/authority preflight passed"));
    assert!(output.contains("this preflight did not call RPC, submit, sign, load a signer, load authority keys, rotate keys, upgrade authority, mint, burn, settle, release ROC, or mutate ROC"));
}

#[test]
fn phase11_template_report_is_redacted_and_non_authorizing() {
    let (ok, output) = run_script(&[
        "--template-drill",
        "testnet",
        "wrong_authority_halt_attempt",
    ]);

    assert!(ok, "template should print:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 11"));
    assert!(output.contains("actual_private_testnet_authority_drill_report"));
    assert!(output.contains("rox-anchor.actual-private-testnet-authority-drill.v1"));
    assert!(output.contains("<redacted-safe-authority-drill-action>"));
    assert!(output.contains(r#""expected_drill": true"#));
    assert!(output.contains(r#""operator_report_redacted": true"#));
    assert!(output.contains(r#""private_testnet_only": true"#));
    assert!(output.contains(r#""test_only_assets_only": true"#));
    assert!(output.contains(r#""system_returned_safe_state": true"#));
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""send_authorized": false"#));
    assert!(output.contains(r#""wallet_loaded": false"#));
    assert!(output.contains(r#""signature_generated": false"#));
    assert!(output.contains(r#""authority_key_loaded": false"#));
    assert!(output.contains(r#""key_rotation_executed": false"#));
    assert!(output.contains(r#""upgrade_authority_changed": false"#));
    assert!(output.contains(r#""production_bridge_settlement": false"#));
    assert!(output.contains(r#""real_roc_release": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
    assert!(!output.contains(r#""key_rotation_executed": true"#));
    assert!(!output.contains(r#""upgrade_authority_changed": true"#));
    assert!(!output.contains(r#""real_roc_mutation": true"#));
}

#[test]
fn phase11_docs_cover_operator_safety_without_runtime_or_settlement_claims() {
    let root = repo_root();
    let halt_doc = std::fs::read_to_string(
        root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md"),
    )
    .expect("halt/recovery drill doc should be readable");
    let authority_doc =
        std::fs::read_to_string(root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md"))
            .expect("authority drill doc should be readable");

    for marker in [
        "halt_before_simulation",
        "halt_after_simulation_before_send",
        "halt_after_capped_send_before_readback",
        "valid_recovery_after_halt",
        "clean_flow_after_valid_recovery",
    ] {
        assert!(halt_doc.contains(marker), "halt doc missing {marker}");
    }

    for marker in [
        "wrong_authority_halt_attempt",
        "wrong_authority_recovery_attempt",
        "key_rotation_intent",
        "upgrade_authority_checklist",
        "separated_authority_status",
    ] {
        assert!(
            authority_doc.contains(marker),
            "authority doc missing {marker}"
        );
    }

    for doc in [&halt_doc, &authority_doc] {
        assert!(doc.contains("No real internal ROC mutation."));
        assert!(
            doc.contains("No production bridge settlement.")
                || doc.contains("not production settlement")
        );
        assert!(!doc.contains("/Users/"));
        assert!(!doc.contains("/home/"));
        assert!(!doc.contains("api-key="));
        assert!(!doc.contains("access_token="));
        assert!(!doc.contains(r#""wallet_loaded": true"#));
        assert!(!doc.contains(r#""authority_key_loaded": true"#));
        assert!(!doc.contains(r#""key_rotation_executed": true"#));
        assert!(!doc.contains(r#""upgrade_authority_changed": true"#));
        assert!(!doc.contains(r#""production_bridge_settlement": true"#));
        assert!(!doc.contains(r#""real_roc_mutation": true"#));
        assert!(!doc.contains(r#""finality_claim": true"#));
    }
}

#[test]
fn phase11_template_rejects_unknown_drill_name() {
    let (ok, output) = run_script(&["--template-drill", "testnet", "not_a_phase11_drill"]);

    assert!(!ok, "unknown drill should fail:\n{output}");
    assert!(output.contains("unknown drill_name"));
}
