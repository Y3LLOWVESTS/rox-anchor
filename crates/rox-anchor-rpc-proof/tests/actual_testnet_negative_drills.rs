//! RO:WHAT — Tests BUILD_PLAN4 Phase 10 RPC-proof negative-drill receipt handling.
//! RO:WHY — Ensures readback/RPC disagreement failure receipts are classified without accepting unsafe evidence.
//! RO:INTERACTS — scripts/check_actual_private_testnet_negative_drills.sh.
//! RO:INVARIANTS — devnet/testnet only; expected failure true; no send authorization; no finality; no settlement; no real ROC mutation.
//! RO:SECURITY — local file checks only; no live RPC, signer load, signing, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test actual_testnet_negative_drills.

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

fn suffix() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
        .to_string()
}

fn write_receipt(name: &str, body: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "rox_anchor_rpc_phase10_negative_drill_{}_{}_{}.json",
        std::process::id(),
        name,
        suffix()
    ));
    fs::write(&path, body).expect("temp receipt should write");
    path
}

fn template_receipt(drill_name: &str) -> String {
    let (ok, output) = run_script(&["--template-failure", "testnet", drill_name]);
    assert!(ok, "template should print for {drill_name}:\n{output}");
    output
}

fn check_receipt(path: &Path) -> (bool, String) {
    let path_arg = path.to_string_lossy().to_string();
    run_script(&["--check-failure-receipt", &path_arg])
}

#[test]
fn rpc_negative_drills_classify_stale_readback_as_fail_safe_missing_readback() {
    let receipt = template_receipt("stale_readback");

    assert!(receipt.contains(r#""proof_review_status": "disputed""#));
    assert!(receipt.contains(r#""readback_status": "missing""#));
    assert!(receipt.contains(r#""transaction_submission": false"#));
    assert!(receipt.contains(r#""finality_claim": false"#));
    assert!(receipt.contains(r#""production_bridge_settlement": false"#));
    assert!(receipt.contains(r#""real_roc_mutation": false"#));

    let path = write_receipt("stale_readback", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(ok, "stale readback receipt should pass:\n{output}");
    assert!(output.contains("proof/coordinator/relayer/readback statuses are fail-safe statuses"));
}

#[test]
fn rpc_negative_drills_classify_under_quorum_rpc_evidence_as_disputed() {
    let receipt = template_receipt("under_quorum_rpc_evidence");

    assert!(receipt.contains(r#""proof_review_status": "disputed""#));
    assert!(receipt.contains(r#""readback_status": "disputed""#));
    assert!(receipt.contains(r#""send_authorized": false"#));
    assert!(receipt.contains(r#""signature_generated": false"#));

    let path = write_receipt("under_quorum", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(
        ok,
        "under-quorum RPC evidence receipt should pass:\n{output}"
    );
    assert!(output.contains("receipt is devnet/testnet only"));
}

#[test]
fn rpc_negative_drills_classify_provider_disagreement_as_disputed() {
    let receipt = template_receipt("rpc_provider_disagreement");

    assert!(receipt.contains(r#""proof_review_status": "disputed""#));
    assert!(receipt.contains(r#""readback_status": "disputed""#));
    assert!(receipt.contains(r#""public_launch_authorized": false"#));
    assert!(receipt.contains(r#""mainnet_authorized": false"#));

    let path = write_receipt("provider_disagreement", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(
        ok,
        "RPC provider disagreement receipt should pass:\n{output}"
    );
    assert!(output.contains("receipt rejects submission"));
}

#[test]
fn rpc_negative_drills_reject_verified_readback_or_finality_claims() {
    let receipt = template_receipt("rpc_provider_disagreement").replace(
        r#""readback_status": "disputed""#,
        r#""readback_status": "verified""#,
    );
    let path = write_receipt("verified_readback", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(
        !ok,
        "verified readback must fail for negative drill:\n{output}"
    );
    assert!(
        output.contains("readback_status must be missing, rejected, disputed, or not_performed")
            || output.contains("success-like marker")
    );

    let receipt = template_receipt("stale_readback")
        .replace(r#""finality_claim": false"#, r#""finality_claim": true"#);
    let path = write_receipt("finality_claim", &receipt);
    let (ok, output) = check_receipt(&path);

    assert!(
        !ok,
        "finality claim must fail for negative drill:\n{output}"
    );
    assert!(output.contains("forbidden true boolean") || output.contains("forbidden"));
}

#[test]
fn rpc_negative_drills_template_matrix_mentions_all_rpc_readback_failure_modes() {
    let (ok, output) = run_script(&["--template-matrix", "testnet"]);

    assert!(ok, "matrix template should print:\n{output}");
    assert!(output.contains(r#""drill_name": "stale_readback""#));
    assert!(output.contains(r#""drill_name": "under_quorum_rpc_evidence""#));
    assert!(output.contains(r#""drill_name": "rpc_provider_disagreement""#));
    assert!(output.contains(r#""drill_name": "readback_missing_after_send""#));
    assert!(!output.contains(r#""finality_claim": true"#));
    assert!(!output.contains(r#""real_roc_mutation": true"#));
}
