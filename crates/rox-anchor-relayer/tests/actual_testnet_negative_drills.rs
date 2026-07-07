//! RO:WHAT — Tests BUILD_PLAN4 Phase 10 negative-drill receipt validation.
//! RO:WHY — Proves failure receipts are fail-safe, redacted, and cannot claim send/finality/settlement/real ROC mutation.
//! RO:INTERACTS — scripts/check_actual_private_testnet_negative_drills.sh.
//! RO:INVARIANTS — devnet/testnet only; expected failure true; no send authorization; no production settlement; no real ROC mutation.
//! RO:SECURITY — local file checks only; no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-relayer --test actual_testnet_negative_drills.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
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
        .arg(root.join("scripts/check_actual_private_testnet_negative_drills.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("actual private testnet negative drill checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

fn template_receipt(drill_name: &str) -> String {
    let (ok, output) = run_script(&["--template-failure", "testnet", drill_name]);
    assert!(ok, "template should print for {drill_name}:\n{output}");
    output
}

fn write_receipt(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_phase10_negative_drill_{}_{}_{}.json",
        std::process::id(),
        name,
        chrono_free_suffix()
    ));
    fs::write(&path, body).expect("temp receipt should write");
    path
}

fn chrono_free_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
        .to_string()
}

fn check_receipt(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-failure-receipt", &path_arg])
}

#[test]
fn actual_testnet_negative_drill_template_is_redacted_and_non_authorizing() {
    let output = template_receipt("wrong_program_id");

    assert!(output.contains(r#""schema": "rox-anchor.actual-private-testnet-negative-drill.v1""#));
    assert!(output.contains(r#""phase": "BUILD_PLAN4 Phase 10""#));
    assert!(output.contains(r#""receipt_role": "actual_private_testnet_negative_drill_receipt""#));
    assert!(output.contains(r#""drill_outcome": "blocked""#));
    assert!(output.contains(r#""expected_failure": true"#));
    assert!(
        output.contains(r#""failure_reason_redacted": "<redacted-safe-negative-drill-failure>""#)
    );
    assert!(output.contains(r#""transaction_submission": false"#));
    assert!(output.contains(r#""send_authorized": false"#));
    assert!(output.contains(r#""signature_generated": false"#));
    assert!(output.contains(r#""production_bridge_settlement": false"#));
    assert!(output.contains(r#""real_roc_release": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));
    assert!(output.contains(r#""finality_claim": false"#));

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("access_token="));
}

#[test]
fn actual_testnet_negative_drill_checker_accepts_every_required_drill_name() {
    for drill in [
        "wrong_program_id",
        "wrong_mint",
        "wrong_token_account",
        "wrong_authority",
        "missing_config_account",
        "missing_mint_account",
        "stale_readback",
        "under_quorum_rpc_evidence",
        "rpc_provider_disagreement",
        "duplicate_operation_id",
        "duplicate_idempotency_key",
        "nonce_reuse",
        "receipt_tamper",
        "missing_receipt",
        "operator_approval_omitted",
        "send_disabled",
        "cap_exceeded",
        "halt_before_simulation",
        "halt_after_simulation_before_send",
        "halt_after_send_before_readback",
        "recovery_during_pending_operation",
        "readback_missing_after_send",
    ] {
        let receipt = template_receipt(drill);
        let path = write_receipt(drill, &receipt);
        let (ok, output) = check_receipt(&path);

        assert!(ok, "{drill} receipt should pass:\n{output}");
        assert!(output
            .contains("BUILD_PLAN4 Phase 10 actual negative drill failure receipt checks passed"));
        assert!(output.contains("failure reason is redacted"));
    }
}

#[test]
fn actual_testnet_negative_drill_rejects_success_like_outcome() {
    let receipt = template_receipt("wrong_program_id").replace(
        r#""drill_outcome": "blocked""#,
        r#""drill_outcome": "success""#,
    );
    let path = write_receipt("success_like", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(!ok, "success-like outcome must fail:\n{output}");
    assert!(
        output
            .contains("drill_outcome must be blocked, failed_safe, quarantined, or not_performed")
            || output.contains("success-like marker")
    );
}

#[test]
fn actual_testnet_negative_drill_rejects_mainnet_cluster() {
    let receipt = template_receipt("wrong_program_id")
        .replace(r#""cluster": "testnet""#, r#""cluster": "mainnet-beta""#);
    let path = write_receipt("mainnet_cluster", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(!ok, "mainnet cluster must fail:\n{output}");
    assert!(output.contains("cluster must be devnet or testnet"));
}

#[test]
fn actual_testnet_negative_drill_rejects_submission_or_real_roc_claims() {
    for (name, needle) in [
        (
            "transaction_submission",
            (
                r#""transaction_submission": false"#,
                r#""transaction_submission": true"#,
            ),
        ),
        (
            "send_authorized",
            (r#""send_authorized": false"#, r#""send_authorized": true"#),
        ),
        (
            "production_bridge_settlement",
            (
                r#""production_bridge_settlement": false"#,
                r#""production_bridge_settlement": true"#,
            ),
        ),
        (
            "real_roc_mutation",
            (
                r#""real_roc_mutation": false"#,
                r#""real_roc_mutation": true"#,
            ),
        ),
        (
            "finality_claim",
            (r#""finality_claim": false"#, r#""finality_claim": true"#),
        ),
    ] {
        let receipt = template_receipt("wrong_program_id").replace(needle.0, needle.1);
        let path = write_receipt(name, &receipt);
        let (ok, output) = check_receipt(&path);

        assert!(!ok, "{name} true claim must fail:\n{output}");
        assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
    }
}

#[test]
fn actual_testnet_negative_drill_rejects_unredacted_sensitive_text() {
    let receipt = template_receipt("wrong_program_id").replace(
        r#""failure_reason_redacted": "<redacted-safe-negative-drill-failure>""#,
        r#""failure_reason_redacted": "/Users/mymac/pilot-keypairs/keypair.json""#,
    );
    let path = write_receipt("sensitive_text", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(!ok, "unredacted sensitive text must fail:\n{output}");
    assert!(output.contains("unredacted secret/path marker"));
}

#[test]
fn actual_testnet_negative_drill_template_rejects_unknown_drill_name() {
    let (ok, output) = run_script(&["--template-failure", "testnet", "not_a_phase10_drill"]);

    assert!(!ok, "unknown drill should fail:\n{output}");
    assert!(output.contains("unknown drill_name"));
}
