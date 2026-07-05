//! RO:WHAT — Tests CLI BUILD_PLAN3 Phase 2 private pilot config status surface.
//! RO:WHY — Proves status output uses the core external loader and redacts pilot config material.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and commands::status.
//! RO:INVARIANTS — CLI reports config shape only; it does not perform RPC, wallet loading, or settlement.
//! RO:SECURITY — no RPC, key loading, deployment, submission, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_config.

use rox_anchor_cli::run_from_args;

#[test]
fn status_output_includes_redacted_private_pilot_config_loader_shape() {
    let output = run_from_args(["rox-anchor", "status"]).expect("status command should run");

    assert!(output.contains("private_pilot_config_surface: redacted_external_config_loader"));
    assert!(output.contains("private_pilot_config: redacted_external_shape"));
    assert!(output.contains("private_pilot_config_runtime_effects: disabled"));
    assert!(output.contains("private_pilot_config_wallet_loading: disabled"));
    assert!(output.contains("private_pilot_config_rpc_calls: disabled"));
    assert!(output.contains("rpc_url: https://private-devnet.invalid/<redacted>"));
    assert!(output.contains("payer_keypair_path: <redacted-external-path>/*.json"));
    assert!(output.contains("receipt_output_path: <redacted-external-path>/*.json"));
    assert!(output.contains("observed_signature: 5Jstatus…4444"));

    assert!(!output.contains("status-provider-token"));
    assert!(!output.contains("/external/pilot-keys"));
    assert!(!output.contains("/external/pilot-receipts"));
    assert!(!output.contains("PrivatePilotSignature"));
    assert!(!output.contains("rpc_submission: enabled"));
    assert!(!output.contains("wallet_loading: enabled"));
    assert!(!output.contains("settlement complete"));
}
