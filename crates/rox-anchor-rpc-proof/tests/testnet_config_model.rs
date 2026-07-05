//! RO:WHAT — Tests RPC proof BUILD_PLAN2 Phase 2 non-secret testnet config wrapper.
//! RO:WHY — Proves RPC proof can carry explicit, redacted testnet config without live calls.
//! RO:INTERACTS — RpcProofTestnetConfig and rox-anchor-core TestnetConfig.
//! RO:INVARIANTS — config validation is local and mainnet endpoints stay rejected.
//! RO:SECURITY — no network, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-rpc-proof --test testnet_config_model.

use rox_anchor_core::{AnchorCluster, AnchorEnvironmentMode, SubmissionMode, TestnetConfig};
use rox_anchor_rpc_proof::{RpcProofConfig, RpcProofTestnetConfig};

#[test]
fn rpc_proof_testnet_config_validates_and_redacts() {
    let testnet = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://rpc.example.dev/private-token"),
        Some("/Users/operator/.config/solana/rpc-proof-payer.json"),
    )
    .expect("devnet config should validate");

    let config = RpcProofTestnetConfig::new(RpcProofConfig::new(2, 100), testnet);

    assert!(config.validate().is_ok());

    let report = config.redacted_report().lines().join("\n");
    assert!(report.contains("https://rpc.example.dev/<redacted>"));
    assert!(report.contains("<redacted-keypair-path>/rpc-proof-payer.json"));
    assert!(!report.contains("private-token"));
    assert!(!report.contains("/Users/operator"));
}
