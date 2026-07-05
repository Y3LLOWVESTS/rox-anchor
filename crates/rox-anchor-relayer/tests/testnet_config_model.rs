//! RO:WHAT — Tests relayer BUILD_PLAN2 Phase 2 non-secret testnet config wrapper.
//! RO:WHY — Proves relayer can represent external RPC/keypair config without loading secrets.
//! RO:INTERACTS — RelayerTestnetConfig and rox-anchor-core TestnetConfig.
//! RO:INVARIANTS — config validation is local; redaction preserves shape while hiding sensitive material.
//! RO:SECURITY — no RPC, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-relayer --test testnet_config_model.

use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, SubmissionMode, TestnetConfig};
use rox_anchor_relayer::{RelayerConfig, RelayerTestnetConfig};

#[test]
fn relayer_testnet_config_validates_and_redacts() {
    let testnet = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Testnet,
        SubmissionMode::SimulateOnly,
        Some("https://rpc.example.test/secret-path"),
        Some("/Users/operator/.config/solana/relayer-payer.json"),
    )
    .expect("testnet config should validate");

    let config = RelayerTestnetConfig::new(RelayerConfig::new(3, 16), testnet);

    assert!(config.validate().is_ok());

    let report = config.redacted_report().lines().join("\n");
    assert!(report.contains("https://rpc.example.test/<redacted>"));
    assert!(report.contains("<redacted-keypair-path>/relayer-payer.json"));
    assert!(!report.contains("secret-path"));
    assert!(!report.contains("/Users/operator"));
}
