//! RO:WHAT — Tests BUILD_PLAN2 Phase 2 non-secret testnet configuration model.
//! RO:WHY — Proves config requires explicit mode, external RPC/keypair inputs, and redacted display.
//! RO:INTERACTS — TestnetConfig, ExternalRpcUrl, ExternalKeypairPath, and AnchorSafetyProfile.
//! RO:INVARIANTS — mainnet endpoints reject; paths/URL tokens do not leak in reports.
//! RO:SECURITY — no key loading, no RPC calls, no wallet calls, no submission, no settlement.
//! RO:TEST — run with cargo test -p rox-anchor-core --test testnet_config_model.

use rox_anchor_core::{
    AnchorCluster, AnchorCoreError, AnchorEnvironmentMode, SubmissionMode, TestnetConfig,
};

#[test]
fn testnet_config_requires_explicit_mode() {
    let err = TestnetConfig::require_explicit(
        None,
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://api.devnet.solana.com"),
        Some("/Users/operator/.config/solana/testnet-payer.json"),
    )
    .unwrap_err();

    assert_eq!(err, AnchorCoreError::MissingExplicitMode);
}

#[test]
fn testnet_config_requires_external_rpc_url() {
    let err = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        None,
        Some("/Users/operator/.config/solana/testnet-payer.json"),
    )
    .unwrap_err();

    assert_eq!(err, AnchorCoreError::MissingRpcUrl);
}

#[test]
fn testnet_config_requires_external_keypair_path() {
    let err = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://api.devnet.solana.com"),
        None,
    )
    .unwrap_err();

    assert_eq!(err, AnchorCoreError::MissingPayerKeypairPath);
}

#[test]
fn testnet_config_rejects_mainnet_beta_rpc_endpoint() {
    let err = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://api.mainnet-beta.solana.com"),
        Some("/Users/operator/.config/solana/testnet-payer.json"),
    )
    .unwrap_err();

    assert_eq!(err, AnchorCoreError::MainnetBetaEndpointForbidden);
}

#[test]
fn redacted_report_hides_rpc_path_tokens_and_full_keypair_path() {
    let config = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://rpc.example.dev/secret-token?api_key=do-not-print"),
        Some("/Users/operator/.config/solana/testnet-payer.json"),
    )
    .expect("devnet testnet config should validate");

    let report = config.redacted_report();
    let rendered = report.lines().join("\n");

    assert!(rendered.contains("https://rpc.example.dev/<redacted>"));
    assert!(rendered.contains("<redacted-keypair-path>/testnet-payer.json"));
    assert!(!rendered.contains("secret-token"));
    assert!(!rendered.contains("api_key"));
    assert!(!rendered.contains("/Users/operator"));
}
