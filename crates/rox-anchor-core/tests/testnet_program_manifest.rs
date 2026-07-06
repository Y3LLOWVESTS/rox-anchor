//! RO:WHAT — Tests BUILD_PLAN3 Phase 3 testnet program artifact manifest validation.
//! RO:WHY — Proves deployment metadata is non-secret, redacted, and never treated as finality proof.
//! RO:INTERACTS — TestnetProgramArtifactManifest, ProgramId, AnchorCluster, and core errors.
//! RO:INVARIANTS — mainnet/local clusters reject; program IDs must match Anchor.toml binding; paths redact.
//! RO:SECURITY — no deployment, RPC, wallet loading, submission, mint/burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-core --test testnet_program_manifest.

use rox_anchor_core::{AnchorCoreError, TestnetProgramArtifactManifest};

const ANCHOR_TOML_PROGRAM_ID: &str = "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR";

fn valid_manifest() -> TestnetProgramArtifactManifest {
    TestnetProgramArtifactManifest::from_labels(
        "devnet",
        ANCHOR_TOML_PROGRAM_ID,
        ANCHOR_TOML_PROGRAM_ID,
        "build-hash-private-testnet-0001",
        "idl-hash-private-testnet-0001",
        Some(123_456),
        "private-pilot-program-operator",
        "test-only-rox-program-artifact",
        "/external/pilot-deploy/rox_anchor.so",
        "/external/pilot-deploy/rox_anchor.json",
    )
    .expect("static manifest should validate")
}

#[test]
fn testnet_program_manifest_accepts_matching_anchor_binding_and_redacts_paths() {
    let manifest = valid_manifest();

    assert!(manifest.validate().is_ok());

    let report = manifest.redacted_report().lines().join("\n");

    assert!(report.contains("testnet_program_manifest: redacted_non_secret_artifact_shape"));
    assert!(report.contains("cluster: devnet"));
    assert!(report.contains(&format!("program_id: {ANCHOR_TOML_PROGRAM_ID}")));
    assert!(report.contains(&format!("expected_program_id: {ANCHOR_TOML_PROGRAM_ID}")));
    assert!(report.contains("build_hash: build-hash-private-testnet-0001"));
    assert!(report.contains("idl_hash: idl-hash-private-testnet-0001"));
    assert!(report.contains("deploy_slot: 123456"));
    assert!(report.contains("program_artifact_path: <redacted-external-path>/*.so"));
    assert!(report.contains("idl_artifact_path: <redacted-external-path>/*.json"));
    assert!(report.contains("manifest_is_deployment_proof: false"));
    assert!(report.contains("production_finality_claim: false"));
    assert!(report.contains("public_launch_authorized: false"));

    assert!(!report.contains("/external/pilot-deploy"));
    assert!(!report.contains("deployment_success: true"));
    assert!(!report.contains("production_finality_claim: true"));
    assert!(!report.contains("public_launch_authorized: true"));
}

#[test]
fn testnet_program_manifest_rejects_mainnet_beta_cluster() {
    let err = TestnetProgramArtifactManifest::from_labels(
        "mainnet-beta",
        ANCHOR_TOML_PROGRAM_ID,
        ANCHOR_TOML_PROGRAM_ID,
        "build-hash-private-testnet-0001",
        "idl-hash-private-testnet-0001",
        None,
        "private-pilot-program-operator",
        "test-only-rox-program-artifact",
        "/external/pilot-deploy/rox_anchor.so",
        "/external/pilot-deploy/rox_anchor.json",
    )
    .unwrap_err();

    assert_eq!(err, AnchorCoreError::MainnetBetaClusterForbidden);
}

#[test]
fn testnet_program_manifest_rejects_localnet_cluster_for_private_pilot_artifacts() {
    let err = TestnetProgramArtifactManifest::from_labels(
        "localnet",
        ANCHOR_TOML_PROGRAM_ID,
        ANCHOR_TOML_PROGRAM_ID,
        "build-hash-private-testnet-0001",
        "idl-hash-private-testnet-0001",
        None,
        "private-pilot-program-operator",
        "test-only-rox-program-artifact",
        "/external/pilot-deploy/rox_anchor.so",
        "/external/pilot-deploy/rox_anchor.json",
    )
    .unwrap_err();

    assert_eq!(
        err,
        AnchorCoreError::ClusterNotAllowed {
            cluster: "localnet"
        }
    );
}

#[test]
fn testnet_program_manifest_rejects_empty_program_id() {
    let err = TestnetProgramArtifactManifest::from_labels(
        "devnet",
        "",
        ANCHOR_TOML_PROGRAM_ID,
        "build-hash-private-testnet-0001",
        "idl-hash-private-testnet-0001",
        None,
        "private-pilot-program-operator",
        "test-only-rox-program-artifact",
        "/external/pilot-deploy/rox_anchor.so",
        "/external/pilot-deploy/rox_anchor.json",
    )
    .unwrap_err();

    assert_eq!(err, AnchorCoreError::EmptyIdentifier { kind: "program_id" });
}

#[test]
fn testnet_program_manifest_rejects_mismatched_program_id() {
    let err = TestnetProgramArtifactManifest::from_labels(
        "devnet",
        "DifferentProgram111111111111111111111111111111",
        ANCHOR_TOML_PROGRAM_ID,
        "build-hash-private-testnet-0001",
        "idl-hash-private-testnet-0001",
        None,
        "private-pilot-program-operator",
        "test-only-rox-program-artifact",
        "/external/pilot-deploy/rox_anchor.so",
        "/external/pilot-deploy/rox_anchor.json",
    )
    .unwrap_err();

    assert_eq!(
        err,
        AnchorCoreError::TestnetProgramIdMismatch {
            expected: ANCHOR_TOML_PROGRAM_ID.to_string(),
            actual: "DifferentProgram111111111111111111111111111111".to_string(),
        }
    );
}

#[test]
fn testnet_program_manifest_rejects_public_or_production_labels() {
    let err = TestnetProgramArtifactManifest::from_labels(
        "devnet",
        ANCHOR_TOML_PROGRAM_ID,
        ANCHOR_TOML_PROGRAM_ID,
        "build-hash-private-testnet-0001",
        "idl-hash-private-testnet-0001",
        None,
        "private-pilot-program-operator",
        "public-rox-program-artifact",
        "/external/pilot-deploy/rox_anchor.so",
        "/external/pilot-deploy/rox_anchor.json",
    )
    .unwrap_err();

    assert_eq!(
        err,
        AnchorCoreError::PublicOrProductionTestnetProgramManifestLabel {
            field: "artifact_label",
            label: "public-rox-program-artifact".to_string(),
        }
    );
}
