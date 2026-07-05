//! RO:WHAT — Tests BUILD_PLAN3 Phase 2 external private pilot config loader.
//! RO:WHY — Proves private pilot config is explicit, external, redacted, and non-production.
//! RO:INTERACTS — PrivatePilotConfig, TestnetConfig, safety labels, and core error vocabulary.
//! RO:INVARIANTS — missing mode/mainnet/public labels reject; reports redact URLs, paths, and signatures.
//! RO:SECURITY — no RPC, key loading, wallet, deployment, transaction, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-core --test private_pilot_config.

use rox_anchor_core::{
    AnchorCluster, AnchorCoreError, AnchorEnvironmentMode, PrivatePilotConfig, SubmissionMode,
};

fn valid_config_text() -> &'static str {
    r#"
environment_mode = "testnet_only"
cluster = "devnet"
submission_mode = "simulate_only"
rpc_url = "https://private-devnet.invalid/provider-token"
payer_keypair_path = "/external/pilot-keys/devnet-payer.json"
operator_label = "private-pilot-operator"
asset_label = "test-only-rox-private-pilot"
receipt_output_path = "/external/pilot-receipts/devnet-receipt.json"
observed_signature = "5JprivatePilotSignatureForRedaction111111222222333333"
"#
}

#[test]
fn private_pilot_config_parses_valid_external_shape_and_redacts_sensitive_values() {
    let config = PrivatePilotConfig::parse_external_config(valid_config_text())
        .expect("valid private pilot config should parse");

    assert_eq!(
        config.testnet.environment_mode,
        AnchorEnvironmentMode::TestnetOnly
    );
    assert_eq!(config.testnet.cluster, AnchorCluster::Devnet);
    assert_eq!(config.testnet.submission_mode, SubmissionMode::SimulateOnly);
    assert_eq!(config.operator_label, "private-pilot-operator");
    assert_eq!(config.asset_label, "test-only-rox-private-pilot");

    let report = config.redacted_report().lines().join("\n");

    assert!(report.contains("private_pilot_config: redacted_external_shape"));
    assert!(report.contains("environment_mode: testnet_only"));
    assert!(report.contains("cluster: devnet"));
    assert!(report.contains("submission_mode: simulate_only"));
    assert!(report.contains("rpc_url: https://private-devnet.invalid/<redacted>"));
    assert!(report.contains("payer_keypair_path: <redacted-external-path>/*.json"));
    assert!(report.contains("receipt_output_path: <redacted-external-path>/*.json"));
    assert!(report.contains("observed_signature: 5Jprivat…3333"));

    assert!(!report.contains("provider-token"));
    assert!(!report.contains("pilot-keys"));
    assert!(!report.contains("pilot-receipts"));
    assert!(!report.contains("SignatureForRedaction"));
    assert!(!report.contains("111111222222"));
}

#[test]
fn private_pilot_config_requires_explicit_environment_mode() {
    let config = valid_config_text().replace("environment_mode = \"testnet_only\"\n", "");
    let err = PrivatePilotConfig::parse_external_config(&config).unwrap_err();

    assert_eq!(err, AnchorCoreError::MissingExplicitMode);
}

#[test]
fn private_pilot_config_rejects_mainnet_beta_cluster() {
    let config = valid_config_text().replace("cluster = \"devnet\"", "cluster = \"mainnet-beta\"");
    let err = PrivatePilotConfig::parse_external_config(&config).unwrap_err();

    assert_eq!(err, AnchorCoreError::MainnetBetaClusterForbidden);
}

#[test]
fn private_pilot_config_rejects_public_or_production_labels() {
    let config = valid_config_text().replace(
        "asset_label = \"test-only-rox-private-pilot\"",
        "asset_label = \"public-rox-mainnet\"",
    );

    let err = PrivatePilotConfig::parse_external_config(&config).unwrap_err();

    assert_eq!(
        err,
        AnchorCoreError::PublicOrProductionPrivatePilotLabel {
            field: "asset_label",
            label: "public-rox-mainnet".to_string()
        }
    );
}

#[test]
fn private_pilot_config_rejects_duplicate_fields() {
    let config = format!("{}\ncluster = \"testnet\"\n", valid_config_text());
    let err = PrivatePilotConfig::parse_external_config(&config).unwrap_err();

    assert_eq!(
        err,
        AnchorCoreError::DuplicatePrivatePilotConfigField {
            field: "cluster".to_string()
        }
    );
}
