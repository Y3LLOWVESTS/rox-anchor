//! RO:WHAT — Tests CLI BUILD_PLAN3 Phase 3 testnet program manifest status surface.
//! RO:WHY — Proves status output exposes redacted deployment artifact metadata without finality claims.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-core manifest report.
//! RO:INVARIANTS — CLI reports manifest shape only; it does not deploy, call RPC, or assert production success.
//! RO:SECURITY — no wallet, RPC, key loading, deployment, submission, mint/burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test testnet_program_manifest.

use rox_anchor_cli::run_from_args;

const ANCHOR_TOML_PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";

#[test]
fn status_output_includes_redacted_testnet_program_manifest_without_finality_claims() {
    let output = run_from_args(["rox-anchor", "status"]).expect("status command should run");

    assert!(output.contains("testnet_program_manifest_surface: redacted_non_secret_artifact_shape"));
    assert!(output.contains("testnet_program_manifest: redacted_non_secret_artifact_shape"));
    assert!(output.contains("cluster: devnet"));
    assert!(output.contains(&format!("program_id: {ANCHOR_TOML_PROGRAM_ID}")));
    assert!(output.contains(&format!("expected_program_id: {ANCHOR_TOML_PROGRAM_ID}")));
    assert!(output.contains("build_hash: build-hash-private-status-0001"));
    assert!(output.contains("idl_hash: idl-hash-private-status-0001"));
    assert!(output.contains("deploy_slot: 123456"));
    assert!(output.contains("program_artifact_path: <redacted-external-path>/*.so"));
    assert!(output.contains("idl_artifact_path: <redacted-external-path>/*.json"));
    assert!(output.contains("manifest_is_deployment_proof: false"));
    assert!(output.contains("private_pilot_finality_claim: false"));
    assert!(output.contains("public_launch_authorized: false"));
    assert!(output.contains("testnet_program_manifest_runtime_effects: disabled"));
    assert!(output.contains("testnet_program_manifest_deployment_claims: disabled"));

    assert!(!output.contains("/external/pilot-deploy"));
    assert!(!output.contains("deployment_success: true"));
    assert!(!output.contains("manifest_is_deployment_proof: true"));
    assert!(!output.contains("private_pilot_finality_claim: true"));
    assert!(!output.contains("public_launch_authorized: true"));
    assert!(!output.contains("mainnet-beta"));
}
