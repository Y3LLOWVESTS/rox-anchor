//! RO:WHAT — Tests CLI status output for BUILD_PLAN2 Phase 2 redacted config display.
//! RO:WHY — Ensures terminal status can show config shape without leaking paths or URL tokens.
//! RO:INTERACTS — status command and rox-anchor-core TestnetConfig redaction.
//! RO:INVARIANTS — status output remains local inspection only and does not claim runtime authority.
//! RO:SECURITY — no RPC, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-cli --test testnet_config_status.

use rox_anchor_cli::run_from_args;

#[test]
fn status_output_includes_redacted_testnet_config_shape() {
    let output = run_from_args(["rox-anchor", "status"]).expect("status should run");

    assert!(output.contains("testnet_config_surface: redacted_non_secret_shape"));
    assert!(output.contains("environment_mode: testnet_only"));
    assert!(output.contains("cluster: devnet"));
    assert!(output.contains("submission_mode: simulate_only"));
    assert!(output.contains("https://api.devnet.solana.com/<redacted>"));
    assert!(output.contains("<redacted-keypair-path>/testnet-payer.json"));
    assert!(!output.contains("example-token"));
    assert!(!output.contains("/Users/operator"));
    assert!(!output.contains("settlement complete"));
}
