//! RO:WHAT — Tests coordinator BUILD_PLAN2 Phase 2 non-secret testnet config wrapper.
//! RO:WHY — Proves coordinator can carry explicit testnet config without secrets or separate rules.
//! RO:INTERACTS — CoordinatorTestnetConfig, RpcProofConfig, and core TestnetConfig.
//! RO:INVARIANTS — coordinator remains orchestration only and inherits shared config validation.
//! RO:SECURITY — no RPC, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-coordinator --test testnet_config_model.

use rox_anchor_coordinator::{CoordinatorConfig, CoordinatorTestnetConfig};
use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, SubmissionMode, TestnetConfig};

#[test]
fn coordinator_testnet_config_validates_and_redacts() {
    let testnet = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://rpc.example.dev/coordinator-token"),
        Some("/Users/operator/.config/solana/coordinator-payer.json"),
    )
    .expect("devnet coordinator config should validate");

    let config = CoordinatorTestnetConfig::new(CoordinatorConfig::new(2, 100, 4), testnet);

    assert!(config.validate().is_ok());

    let report = config.redacted_report().lines().join("\n");
    assert!(report.contains("https://rpc.example.dev/<redacted>"));
    assert!(report.contains("<redacted-keypair-path>/coordinator-payer.json"));
    assert!(!report.contains("coordinator-token"));
    assert!(!report.contains("/Users/operator"));
}
