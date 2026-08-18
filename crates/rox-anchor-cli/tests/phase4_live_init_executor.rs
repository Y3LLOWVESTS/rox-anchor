//! Focused BUILD_PLAN4 Phase 4G executor surface tests.
//!
//! These tests never load real keypairs and never call RPC.

#![forbid(unsafe_code)]

use rox_anchor_cli::{commands::test_only_init::PHASE4_OPERATOR_APPROVAL, run_from_args, CliError};

#[test]
fn phase4_help_exposes_explicit_live_simulation_without_send() {
    let output = run_from_args(["rox-anchor", "pilot", "initialize-test-only-mint", "--help"])
        .expect("Phase 4 help should render");

    assert!(output.contains("--simulate-live"));

    assert!(output.contains("calls simulateTransaction without broadcasting"));

    assert!(output.contains("transaction submission"));

    assert!(output.contains("always disabled outside explicit --execute-live:"));
}

#[test]
fn phase4_command_still_fails_closed_without_explicit_mode() {
    let error = run_from_args(["rox-anchor", "pilot", "initialize-test-only-mint"])
        .expect_err("Phase 4 command without a mode must fail");

    assert!(error
        .to_string()
        .contains("requires exactly one explicit mode"));

    assert!(error.to_string().contains("--simulate-live"));
}

#[test]
fn prepare_and_live_simulation_modes_are_mutually_exclusive() {
    let error = run_from_args([
        "rox-anchor",
        "pilot",
        "initialize-test-only-mint",
        "--prepare-only",
        "--simulate-live",
    ])
    .expect_err("ambiguous Phase 4 mode must fail");

    assert_eq!(
        error,
        CliError::UnknownPilotFlag(
            "initialize-test-only-mint --prepare-only, --simulate-live, and --execute-live are mutually exclusive"
                .to_string()
        )
    );
}

#[test]
fn live_simulation_requires_exact_operator_approval_before_key_loading() {
    let error = run_from_args([
        "rox-anchor",
        "pilot",
        "initialize-test-only-mint",
        "--config",
        ".rox-anchor-private-pilot/nonexistent-phase4.local.toml",
        "--receipt-out",
        ".rox-anchor-private-pilot/nonexistent-phase4-receipt.local.json",
        "--operator-approval",
        "NOT_APPROVED",
        "--simulate-live",
    ])
    .expect_err("wrong approval must fail before file or RPC access");

    assert!(error.to_string().contains("operator approval phrase"));
}

#[test]
fn exact_phase4_approval_constant_is_preserved() {
    assert_eq!(
        PHASE4_OPERATOR_APPROVAL,
        "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT"
    );
}

#[test]
fn executor_source_contains_simulation_but_no_submission_api() {
    let source = include_str!("../src/commands/phase4_live_executor.rs");

    assert!(source.contains("simulate_transaction"));

    let forbidden = [
        ["send_", "transaction"].concat(),
        ["send_and_confirm_", "transaction"].concat(),
        ["send_with_", "spinner"].concat(),
    ];

    for marker in forbidden {
        assert!(
            !source.contains(&marker),
            "executor must not contain transaction submission API: {marker}"
        );
    }
}

#[test]
fn cli_manifest_uses_matching_anchor_client_generation() {
    let manifest = include_str!("../Cargo.toml");

    assert!(manifest.contains("anchor-client = \"0.31.1\""));

    assert!(manifest.contains("anchor-lang = \"0.31.1\""));
}
