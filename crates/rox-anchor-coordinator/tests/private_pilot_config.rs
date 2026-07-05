//! RO:WHAT — Tests coordinator BUILD_PLAN3 Phase 2 private pilot config wrapper.
//! RO:WHY — Proves coordinator consumes core-owned private pilot config validation.
//! RO:INTERACTS — CoordinatorPrivatePilotConfig, RpcProofConfig, and rox-anchor-core PrivatePilotConfig.
//! RO:INVARIANTS — coordinator remains orchestration only and inherits shared config/redaction rules.
//! RO:SECURITY — no RPC, key loading, wallet, transaction, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test private_pilot_config.

use rox_anchor_coordinator::{CoordinatorConfig, CoordinatorPrivatePilotConfig};

fn config_text() -> &'static str {
    r#"
environment_mode = "testnet_only"
cluster = "devnet"
submission_mode = "simulate_only"
rpc_url = "https://coordinator.private.invalid/provider-token"
payer_keypair_path = "/external/pilot-keys/coordinator-payer.json"
operator_label = "private-pilot-coordinator"
asset_label = "test-only-rox-coordinator"
receipt_output_path = "/external/pilot-receipts/coordinator-receipt.json"
observed_signature = "5JcoordinatorPrivatePilotSignature1111222233334444"
"#
}

#[test]
fn coordinator_private_pilot_config_validates_and_redacts() {
    let config = CoordinatorPrivatePilotConfig::from_external_config_text(
        CoordinatorConfig::new(2, 100, 4),
        config_text(),
    )
    .expect("coordinator private pilot config should parse");

    assert!(config.validate().is_ok());

    let report = config.redacted_report().lines().join("\n");

    assert!(report.contains("private_pilot_config: redacted_external_shape"));
    assert!(report.contains("rpc_url: https://coordinator.private.invalid/<redacted>"));
    assert!(report.contains("payer_keypair_path: <redacted-external-path>/*.json"));
    assert!(!report.contains("provider-token"));
    assert!(!report.contains("pilot-keys"));
    assert!(!report.contains("PrivatePilotSignature"));
}
