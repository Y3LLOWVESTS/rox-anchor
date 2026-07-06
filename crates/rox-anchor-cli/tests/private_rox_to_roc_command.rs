//! RO:WHAT — Tests BUILD_PLAN3 Phase 13 private ROX-to-ROC CLI command.
//! RO:WHY — Ensures reverse pilot reports are explicit, test-only, release-intent-only, and display safe.
//! RO:INTERACTS — rox_anchor_cli pilot command group, proof review, simulation, and release intent report.
//! RO:INVARIANTS — command requires --simulate-only and never claims real ROC release or settlement.
//! RO:SECURITY — no wallet loading, signing, RPC submission, live send, ROC mutation, or access unlock.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_rox_to_roc_command.

#![forbid(unsafe_code)]

use rox_anchor_cli::{run_from_args, CliError};

#[test]
fn pilot_rox_to_roc_command_reports_reverse_flow_without_unsafe_effects() {
    let output = run_from_args(["rox-anchor", "pilot", "rox-to-roc", "--simulate-only"])
        .expect("private ROX-to-ROC pilot command should render");

    assert!(output.contains("command: pilot rox-to-roc"));
    assert!(output.contains("scope: private_rox_to_roc_testnet_pilot"));
    assert!(output.contains("proof_decision: Accepted"));
    assert!(output.contains("test_rox_burn_evidence: read_only_rpc_verified_or_simulated_fixture"));
    assert!(output.contains("internal_roc_release_intent: dry_run_output"));
    assert!(output.contains("real_internal_roc_release: disabled"));
    assert!(output.contains("future_real_roc_path: svc-wallet -> ron-ledger only"));
    assert!(output.contains("svc_wallet_call: disabled"));
    assert!(output.contains("ron_ledger_mutation: disabled"));
    assert!(output.contains("read_only_rpc_gate_fixture: verified"));
    assert!(output.contains("status: Simulated"));
    assert!(output.contains("capped_submit_authorization: not_applicable_to_internal_roc_release"));
    assert!(output.contains("live_submission: false"));
    assert!(output.contains("network_submission: disabled_in_cli_report"));
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
        "ron ledger mutated",
    ] {
        assert!(
            !output.to_ascii_lowercase().contains(forbidden),
            "output must not contain unsafe phrase: {forbidden}\n{output}"
        );
    }
}

#[test]
fn pilot_rox_to_roc_requires_explicit_simulate_only() {
    let error = run_from_args(["rox-anchor", "pilot", "rox-to-roc"]).unwrap_err();

    assert_eq!(
        error,
        CliError::UnknownPilotFlag(
            "pilot rox-to-roc requires explicit --simulate-only".to_string()
        )
    );
}

#[test]
fn pilot_rox_to_roc_surfaces_read_only_rpc_gate_failure() {
    let output = run_from_args([
        "rox-anchor",
        "pilot",
        "rox-to-roc",
        "--simulate-only",
        "--missing-read-only-rpc",
    ])
    .expect("missing read-only RPC fixture should render safely");

    assert!(output.contains("read_only_rpc_gate_fixture: missing"));
    assert!(output.contains("status: ReadOnlyRpcNotVerified"));
    assert!(output.contains("simulated: false"));
    assert!(output.contains("capped_submit_authorization: not_applicable_to_internal_roc_release"));
    assert!(output.contains("network_submission: disabled_in_cli_report"));
    assert!(output.contains("real_internal_roc_release: disabled"));
    assert!(output.contains("settlement_claim: none"));
}

#[test]
fn pilot_rox_to_roc_unknown_or_public_flags_fail_closed() {
    let error = run_from_args([
        "rox-anchor",
        "pilot",
        "rox-to-roc",
        "--simulate-only",
        "--mainnet",
    ])
    .unwrap_err();

    assert!(matches!(error, CliError::UnknownPilotFlag(_)));

    let submit_flag = run_from_args([
        "rox-anchor",
        "pilot",
        "rox-to-roc",
        "--simulate-only",
        "--authorize-testnet-submit-capped",
    ])
    .unwrap_err();

    assert!(matches!(submit_flag, CliError::UnknownPilotFlag(_)));
}
