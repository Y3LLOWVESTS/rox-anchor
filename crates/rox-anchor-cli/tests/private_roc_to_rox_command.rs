//! RO:WHAT — Tests BUILD_PLAN3 Phase 12 private ROC-to-ROX CLI command.
//! RO:WHY — Ensures forward pilot reports are explicit, test-only, receipt-aware, and display safe.
//! RO:INTERACTS — rox_anchor_cli pilot command group, proof review, simulation, and capped submit report.
//! RO:INVARIANTS — command requires --simulate-only and never claims real burn, public mint, or settlement.
//! RO:SECURITY — no wallet loading, signing, RPC submission, live send attempt, ROC mutation, or access unlock.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_roc_to_rox_command.

#![forbid(unsafe_code)]

use rox_anchor_cli::{run_from_args, CliError};

#[test]
fn pilot_roc_to_rox_command_reports_forward_flow_without_unsafe_effects() {
    let output = run_from_args([
        "rox-anchor",
        "pilot",
        "roc-to-rox",
        "--simulate-only",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
    ])
    .expect("private ROC-to-ROX pilot command should render");

    assert!(output.contains("command: pilot roc-to-rox"));
    assert!(output.contains("scope: private_roc_to_rox_testnet_pilot"));
    assert!(output.contains("proof_decision: Accepted"));
    assert!(output.contains("internal_roc_burn_intent: dry_run_input"));
    assert!(output.contains("real_internal_roc_burn: disabled"));
    assert!(output.contains("test_rox_mint_path: simulation_or_explicit_capped_testnet_only"));
    assert!(output.contains("read_only_rpc_gate_fixture: verified"));
    assert!(output.contains("status: Simulated"));
    assert!(output.contains("capped_submit_status: Authorized"));
    assert!(output.contains("authorized: true"));
    assert!(output.contains("live_submission_attempted: false"));
    assert!(output.contains("network_submitted: false"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("settlement_claim: none"));
    assert!(output.contains("public_launch_authorization: none"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "transaction sent",
        "mint complete",
        "burn complete",
        "public launch authorized",
        "settlement complete",
        "access granted",
        "roc released",
    ] {
        assert!(
            !output.to_ascii_lowercase().contains(forbidden),
            "output must not contain unsafe phrase: {forbidden}\n{output}"
        );
    }
}

#[test]
fn pilot_roc_to_rox_requires_explicit_simulate_only() {
    let error = run_from_args(["rox-anchor", "pilot", "roc-to-rox"]).unwrap_err();

    assert_eq!(
        error,
        CliError::UnknownPilotFlag(
            "pilot roc-to-rox requires explicit --simulate-only".to_string()
        )
    );
}

#[test]
fn pilot_roc_to_rox_surfaces_read_only_rpc_gate_failure() {
    let output = run_from_args([
        "rox-anchor",
        "pilot",
        "roc-to-rox",
        "--simulate-only",
        "--missing-read-only-rpc",
    ])
    .expect("missing read-only RPC fixture should render safely");

    assert!(output.contains("read_only_rpc_gate_fixture: missing"));
    assert!(output.contains("status: ReadOnlyRpcNotVerified"));
    assert!(output.contains("simulated: false"));
    assert!(output.contains("capped_submit_authorization: not_requested"));
    assert!(output.contains("network_submission: disabled_in_cli_report"));
    assert!(output.contains("settlement_claim: none"));
}

#[test]
fn pilot_roc_to_rox_unknown_or_public_flags_fail_closed() {
    let error = run_from_args([
        "rox-anchor",
        "pilot",
        "roc-to-rox",
        "--simulate-only",
        "--mainnet",
    ])
    .unwrap_err();

    assert!(matches!(error, CliError::UnknownPilotFlag(_)));
}
