//! RO:WHAT — Tests BUILD_PLAN3 Phase 10 private-pilot CLI command surface.
//! RO:WHY — Proves status/proof/simulation/submit/receipts/drill are usable without unsafe defaults.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and pilot command wrappers.
//! RO:INVARIANTS — non-read-only paths require explicit flags; unknown or ambiguous flags fail closed.
//! RO:SECURITY — no live RPC, wallet/key loading, signing, mint/burn, ROC release, settlement, or public launch.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_cli.

#![forbid(unsafe_code)]

use rox_anchor_cli::{run_from_args, CliError};

#[test]
fn pilot_help_lists_safe_subcommands_and_no_launch_modes() {
    let output =
        run_from_args(["rox-anchor", "pilot", "--help"]).expect("pilot help should render");

    assert!(output.contains("rox-anchor pilot"));
    assert!(output.contains("status"));
    assert!(output.contains("read-only-proof"));
    assert!(output.contains("simulate --simulate-only"));
    assert!(output.contains("submit-capped"));
    assert!(output.contains("receipts"));
    assert!(output.contains("drill"));

    assert!(output.contains("no default send path"));
    assert!(output.contains("no mainnet mode"));
    assert!(output.contains("no public launch mode"));
    assert!(output.contains("no production settlement mode"));
    assert!(output.contains("non-read-only pilot paths require explicit flags"));

    assert!(!output.contains("mainnet-beta deployment"));
    assert!(!output.contains("public ROX minting"));
    assert!(!output.contains("production bridge settlement"));
}

#[test]
fn root_help_lists_pilot_command() {
    let output = run_from_args(["rox-anchor", "--help"]).expect("root help should render");

    assert!(output.contains("pilot"));
    assert!(output.contains("private pilot command group"));
    assert!(output.contains("no silent RPC submission"));
    assert!(output.contains("no wallet/key loading"));
    assert!(output.contains("no settlement or finality claim"));
}

#[test]
fn pilot_status_wraps_existing_status_report() {
    let output =
        run_from_args(["rox-anchor", "pilot", "status"]).expect("pilot status should render");

    assert!(output.contains("command: pilot status"));
    assert!(output.contains("scope: private_pilot_command_surface"));
    assert!(output.contains("pilot_subcommand: status"));
    assert!(output.contains("private_pilot_config_surface"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("internal_roc_mutation: disabled"));
    assert!(output.contains("settlement_claim: none"));
}

#[test]
fn pilot_read_only_proof_wraps_existing_read_only_report() {
    let output = run_from_args(["rox-anchor", "pilot", "read-only-proof"])
        .expect("pilot read-only proof should render");

    assert!(output.contains("command: pilot read-only-proof"));
    assert!(output.contains("scope: private_pilot_command_surface"));
    assert!(output.contains("pilot_subcommand: read-only-proof"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_submission"));
    assert!(!output.contains("network_submitted: true"));
    assert!(!output.contains("settlement_claim: final"));
}

#[test]
fn pilot_simulate_requires_explicit_simulate_only_flag() {
    let error = run_from_args(["rox-anchor", "pilot", "simulate"]).unwrap_err();

    assert_eq!(
        error,
        CliError::UnknownPilotFlag("pilot simulate requires explicit --simulate-only".to_string())
    );
}

#[test]
fn pilot_simulate_runs_only_after_explicit_flag() {
    let output = run_from_args(["rox-anchor", "pilot", "simulate", "--simulate-only"])
        .expect("pilot simulate should render");

    assert!(output.contains("command: pilot simulate"));
    assert!(output.contains("explicit_simulate_only: true"));
    assert!(output.contains("read_only_rpc_gate_fixture: verified"));
    assert!(output.contains("private_pilot_simulation: local_only"));
    assert!(output.contains("status: Simulated"));
    assert!(output.contains("simulated: true"));
    assert!(output.contains("live_submission: false"));
    assert!(output.contains("wallet_key_loading: false"));
    assert!(output.contains("internal_roc_mutation: false"));
    assert!(output.contains("network_submission: disabled"));
    assert!(output.contains("settlement_claim: none"));
    assert!(!output.contains("network_submitted: true"));
}

#[test]
fn pilot_simulate_surfaces_read_only_rpc_gate_failure() {
    let output = run_from_args([
        "rox-anchor",
        "pilot",
        "simulate",
        "--simulate-only",
        "--missing-read-only-rpc",
    ])
    .expect("pilot simulate missing read-only rpc fixture should render");

    assert!(output.contains("read_only_rpc_gate_fixture: missing"));
    assert!(output.contains("status: ReadOnlyRpcNotVerified"));
    assert!(output.contains("simulated: false"));
    assert!(output.contains("live_submission: false"));
    assert!(output.contains("network_submission: disabled"));
}

#[test]
fn pilot_submit_capped_routes_to_existing_safe_authorization_report() {
    let output = run_from_args([
        "rox-anchor",
        "pilot",
        "submit-capped",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
    ])
    .expect("pilot submit-capped should render");

    assert!(output.contains("command: pilot submit-capped"));
    assert!(output.contains("scope: private_pilot_command_surface"));
    assert!(output.contains("pilot_subcommand: submit-capped"));
    assert!(output.contains("capped_submit_status: Authorized"));
    assert!(output.contains("authorized: true"));
    assert!(output.contains("live_submission_permitted: true"));
    assert!(output.contains("live_submission_attempted: false"));
    assert!(output.contains("network_submitted: false"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("settlement_claim: none"));
}

#[test]
fn pilot_receipts_and_drill_are_accessible() {
    let receipts =
        run_from_args(["rox-anchor", "pilot", "receipts"]).expect("pilot receipts should render");
    assert!(receipts.contains("command: pilot receipts"));
    assert!(receipts.contains("pilot_receipt_ledger=pilot-receipt-ledger-v1"));
    assert!(receipts.contains("production_settlement_claim=false"));

    let drill = run_from_args(["rox-anchor", "pilot", "drill", "--stage=after-simulation"])
        .expect("pilot drill should render");
    assert!(drill.contains("command: pilot drill"));
    assert!(drill.contains("command: drill"));
    assert!(drill.contains("stage: after_simulation_before_submission"));
    assert!(drill.contains("network_submitted: false"));
    assert!(drill.contains("settlement_claim: none"));
}

#[test]
fn pilot_unknown_subcommands_and_ambiguous_flags_fail_closed() {
    let unknown = run_from_args(["rox-anchor", "pilot", "mainnet"]).unwrap_err();
    assert!(matches!(unknown, CliError::UnknownPilotFlag(_)));

    let extra = run_from_args(["rox-anchor", "pilot", "status", "--mainnet"]).unwrap_err();
    assert_eq!(
        extra,
        CliError::UnknownPilotFlag("pilot status does not accept `--mainnet`".to_string())
    );

    let ambiguous = run_from_args([
        "rox-anchor",
        "pilot",
        "simulate",
        "--simulate-only",
        "--authorize-testnet-submit-capped",
    ])
    .unwrap_err();
    assert!(matches!(ambiguous, CliError::UnknownPilotFlag(_)));
}
