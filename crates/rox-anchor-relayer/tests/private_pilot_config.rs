//! RO:WHAT — Tests relayer BUILD_PLAN3 Phase 2 private pilot config wrapper.
//! RO:WHY — Proves relayer consumes the core external loader and keeps config report-only.
//! RO:INTERACTS — RelayerPrivatePilotConfig and rox-anchor-core PrivatePilotConfig.
//! RO:INVARIANTS — validation is local; reports redact provider paths and signatures.
//! RO:SECURITY — no RPC, key loading, wallet, live submission, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-relayer --test private_pilot_config.

use rox_anchor_relayer::{RelayerConfig, RelayerPrivatePilotConfig};

fn config_text() -> &'static str {
    r#"
environment_mode = "testnet_only"
cluster = "testnet"
submission_mode = "simulate_only"
rpc_url = "https://relayer.private.invalid/provider-token"
payer_keypair_path = "/external/pilot-keys/relayer-payer.json"
operator_label = "private-pilot-relayer"
asset_label = "test-only-rox-relayer"
receipt_output_path = "/external/pilot-receipts/relayer-receipt.json"
observed_signature = "5JrelayerPrivatePilotSignature1111222233334444"
"#
}

#[test]
fn relayer_private_pilot_config_validates_and_redacts() {
    let config = RelayerPrivatePilotConfig::from_external_config_text(
        RelayerConfig::new(3, 16),
        config_text(),
    )
    .expect("relayer private pilot config should parse");

    assert!(config.validate().is_ok());

    let report = config.redacted_report().lines().join("\n");

    assert!(report.contains("private_pilot_config: redacted_external_shape"));
    assert!(report.contains("rpc_url: https://relayer.private.invalid/<redacted>"));
    assert!(report.contains("receipt_output_path: <redacted-external-path>/*.json"));
    assert!(!report.contains("provider-token"));
    assert!(!report.contains("pilot-receipts"));
    assert!(!report.contains("PrivatePilotSignature"));
}
