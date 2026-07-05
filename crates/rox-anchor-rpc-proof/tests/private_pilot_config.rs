//! RO:WHAT — Tests RPC proof BUILD_PLAN3 Phase 2 private pilot config wrapper.
//! RO:WHY — Proves RPC proof consumes the core external loader instead of inventing config rules.
//! RO:INTERACTS — RpcProofPrivatePilotConfig and rox-anchor-core PrivatePilotConfig.
//! RO:INVARIANTS — validation is local; reports redact provider paths and signatures.
//! RO:SECURITY — no network, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test private_pilot_config.

use rox_anchor_rpc_proof::{RpcProofConfig, RpcProofPrivatePilotConfig};

fn config_text() -> &'static str {
    r#"
environment_mode = "testnet_only"
cluster = "devnet"
submission_mode = "simulate_only"
rpc_url = "https://rpc-proof.private.invalid/provider-token"
payer_keypair_path = "/external/pilot-keys/rpc-proof-payer.json"
operator_label = "private-pilot-rpc-proof"
asset_label = "test-only-rox-rpc-proof"
receipt_output_path = "/external/pilot-receipts/rpc-proof-receipt.json"
observed_signature = "5JrpcProofPrivatePilotSignature1111222233334444"
"#
}

#[test]
fn rpc_proof_private_pilot_config_validates_and_redacts() {
    let config = RpcProofPrivatePilotConfig::from_external_config_text(
        RpcProofConfig::new(2, 100),
        config_text(),
    )
    .expect("RPC proof private pilot config should parse");

    assert!(config.validate().is_ok());

    let report = config.redacted_report().lines().join("\n");

    assert!(report.contains("private_pilot_config: redacted_external_shape"));
    assert!(report.contains("rpc_url: https://rpc-proof.private.invalid/<redacted>"));
    assert!(report.contains("payer_keypair_path: <redacted-external-path>/*.json"));
    assert!(!report.contains("provider-token"));
    assert!(!report.contains("pilot-keys"));
    assert!(!report.contains("PrivatePilotSignature"));
}
