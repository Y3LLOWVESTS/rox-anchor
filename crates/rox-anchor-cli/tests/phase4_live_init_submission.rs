//! Focused BUILD_PLAN4 Phase 4H submission-surface tests.
//!
//! Nothing in this test file loads the real pilot keys, calls RPC, or invokes
//! --execute-live.

#![forbid(unsafe_code)]

use rox_anchor_cli::{commands::test_only_init::PHASE4_OPERATOR_APPROVAL, run_from_args};

#[test]
fn phase4_help_exposes_execute_live_as_a_distinct_explicit_mode() {
    let output = run_from_args(["rox-anchor", "pilot", "initialize-test-only-mint", "--help"])
        .expect("Phase 4 help should render");

    assert!(output.contains("--prepare-only"));

    assert!(output.contains("--simulate-live"));

    assert!(output.contains("--execute-live"));

    assert!(output.contains("simulates the exact signed transaction first"));

    assert!(output.contains("waits for confirmed transaction result"));
}

#[test]
fn all_phase4_modes_remain_mutually_exclusive() {
    for arguments in [
        vec!["--prepare-only", "--simulate-live"],
        vec!["--prepare-only", "--execute-live"],
        vec!["--simulate-live", "--execute-live"],
    ] {
        let mut command = vec!["rox-anchor", "pilot", "initialize-test-only-mint"];

        command.extend(arguments);

        let error = run_from_args(command).expect_err("multiple Phase 4 modes must fail closed");

        assert!(error.to_string().contains("mutually exclusive"));
    }
}

#[test]
fn execute_live_requires_exact_operator_approval_before_file_or_rpc_access() {
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
        "--execute-live",
    ])
    .expect_err("wrong approval must fail before Phase 4 file or RPC access");

    assert!(error.to_string().contains("operator approval phrase"));

    assert_eq!(
        PHASE4_OPERATOR_APPROVAL,
        "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT"
    );
}

#[test]
fn submission_is_isolated_from_the_simulation_only_module() {
    let simulation_source = include_str!("../src/commands/phase4_live_executor.rs");

    let submission_source = include_str!("../src/commands/phase4_live_submit.rs");

    assert!(simulation_source.contains("simulate_transaction"));

    assert!(!simulation_source.contains("send_and_confirm_transaction"));

    assert!(submission_source.contains("send_and_confirm_transaction"));

    assert!(submission_source.contains("simulate_prepared_transaction"));
}

#[test]
fn confirmed_submission_requires_strict_post_transaction_readback() {
    let source = include_str!("../src/commands/phase4_live_submit.rs");

    for marker in [
        "RoxAnchorConfig::try_deserialize",
        "state.authority",
        "state.halt_authority",
        "state.recovery_authority",
        "state.rox_mint",
        "state.mint_authority",
        "state.test_only_mode",
        "state.max_supply_units",
        "state.max_amount_units_per_operation",
        "mint.supply != 0",
        "mint.freeze_authority",
        "token_state.amount != 0",
        "confirmed_readback: GREEN",
    ] {
        assert!(
            source.contains(marker),
            "submission source is missing readback invariant: {marker}"
        );
    }
}

#[test]
fn phase4_submission_receipt_cannot_claim_rox_or_roc_value_movement() {
    let source = include_str!("../src/commands/phase4_live_submit.rs");

    for marker in [
        r#"\"rox_mint_performed\": false"#,
        r#"\"rox_burn_performed\": false"#,
        r#"\"real_roc_mutation\": false"#,
        r#"\"production_settlement\": false"#,
        r#"\"mainnet\": false"#,
        r#"\"initial_mint_supply\": 0"#,
        r#"\"initial_token_account_amount\": 0"#,
    ] {
        assert!(
            source.contains(marker),
            "submission receipt is missing safety field: {marker}"
        );
    }
}
