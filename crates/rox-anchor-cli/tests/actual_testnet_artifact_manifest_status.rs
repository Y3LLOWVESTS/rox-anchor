//! RO:WHAT — Tests BUILD_PLAN4 Phase 2 build artifact status/doc surfaces.
//! RO:WHY — Proves operator-facing build metadata remains redacted and does not claim deployment/finality.
//! RO:INTERACTS — scripts/capture_actual_private_testnet_build_artifacts.sh and docs/pilot.
//! RO:INVARIANTS — template/doc output is build evidence only; no mainnet, public launch, finality, or runtime claims.
//! RO:SECURITY — no RPC, wallet load, signing, deployment, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_testnet_artifact_manifest_status.

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
        .arg(root.join("scripts/capture_actual_private_testnet_build_artifacts.sh"))
        .args(args)
        .current_dir(&root)
        .output()
        .expect("build artifact capture script should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn actual_build_artifact_template_is_redacted_and_non_deploying() {
    let (ok, output) = run_script(&["--template"]);

    assert!(ok, "template should print:\n{output}");
    assert!(output.contains("rox-anchor.actual-private-testnet-build-artifacts.v1"));
    assert!(output.contains("BUILD_PLAN4 Phase 2"));
    assert!(output.contains("anchor_build_metadata_only"));
    assert!(output.contains("<sha256>"));
    assert!(output.contains("<redacted-anchor-build-path>/rox_anchor.so"));
    assert!(output.contains("<redacted-anchor-build-path>/rox_anchor.json"));
    assert!(output.contains(r#""build_manifest_is_deployment_proof": false"#));
    assert!(output.contains(r#""deployment_claim": false"#));
    assert!(output.contains(r#""finality_claim": false"#));
    assert!(output.contains(r#""runtime_authority": false"#));
    assert!(output.contains(r#""public_launch_authorized": false"#));
    assert!(output.contains(r#""mainnet_authorized": false"#));
    assert!(output.contains(r#""real_roc_mutation": false"#));

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("deployment_success"));
    assert!(!output.contains(r#""build_manifest_is_deployment_proof": true"#));
    assert!(!output.contains(r#""finality_claim": true"#));
}

#[test]
fn actual_build_artifact_docs_checker_reports_operator_safe_boundaries() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();

    let (ok, output) = run_script(&["--check-docs", &root_arg]);

    assert!(ok, "docs checker should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 2 build artifact documentation checks passed"));
    assert!(output.contains("actual Anchor build artifact capture is documented"));
    assert!(output.contains("local manifest artifact names are ignored"));
    assert!(output.contains("build-only/non-deployment/non-finality boundaries"));
    assert!(output.contains("redacted paths"));
}

#[test]
fn actual_build_artifact_runbook_is_build_only_and_non_authorizing() {
    let doc = std::fs::read_to_string(
        repo_root().join("docs/pilot/ACTUAL_PRIVATE_TESTNET_BUILD_ARTIFACTS.md"),
    )
    .expect("actual build artifact doc should exist");

    assert!(doc.contains("anchor build"));
    assert!(doc.contains("target/deploy/rox_anchor.so"));
    assert!(doc.contains("target/idl/rox_anchor.json"));
    assert!(doc.contains("program binary SHA-256"));
    assert!(doc.contains("IDL SHA-256"));
    assert!(doc.contains("What this does not prove"));
    assert!(doc.contains("deployment success"));
    assert!(doc.contains("program account existence"));
    assert!(doc.contains("transaction submission success"));
    assert!(doc.contains("No deployment proof."));
    assert!(doc.contains("No finality proof."));
    assert!(doc.contains("No public launch authorization."));
    assert!(doc.contains("No mainnet-beta authorization."));
    assert!(doc.contains("No real internal ROC release."));

    assert!(!doc.contains("deployment_success: true"));
    assert!(!doc.contains("build_manifest_is_deployment_proof: true"));
    assert!(!doc.contains("finality_claim: true"));
    assert!(!doc.contains("public_launch_authorized: true"));
    assert!(!doc.contains("mainnet_authorized: true"));
    assert!(!doc.contains("/Users/"));
    assert!(!doc.contains("/home/"));
}
