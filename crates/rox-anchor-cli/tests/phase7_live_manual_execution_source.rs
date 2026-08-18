//! BUILD_PLAN4 Phase 7E explicit live execution boundary.
//!
//! Tests only help/fail-closed paths and source ordering. The live execution
//! flag is never supplied together with complete live arguments here.

#![forbid(unsafe_code)]

use rox_anchor_cli::{run_from_args, CliError};

const SOURCE: &str = include_str!("../src/commands/phase7_live_manual_execution.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

#[test]
fn phase7e_help_truthfully_warns_that_the_command_is_live() {
    let output = run_from_args([
        "rox-anchor",
        "pilot",
        "phase7-execute-capped-roc-to-rox",
        "--help",
    ])
    .expect("Phase 7E help should be safe");

    assert!(output.contains("LIVE DEVNET ROC-to-ROX execution"));

    assert!(output.contains("THIS COMMAND SUBMITS ONE REAL DEVNET TRANSACTION"));

    assert!(output.contains("--execute-live-devnet-send"));

    assert!(output.contains("DO NOT RETRY"));
}

#[test]
fn phase7e_route_fails_before_io_without_live_execution_flag() {
    let error =
        run_from_args(["rox-anchor", "pilot", "phase7-execute-capped-roc-to-rox"]).unwrap_err();

    assert!(matches!(error, CliError::UnknownPilotFlag(_)));

    assert!(format!("{error:?}").contains("--execute-live-devnet-send is required"));
}

#[test]
fn phase7e_execution_flag_guard_precedes_file_and_rpc_access() {
    let function_start = SOURCE
        .find("pub(crate) fn run_phase7_live_manual_execution")
        .expect("Phase 7E execution function must exist");

    let function_end = SOURCE[function_start..]
        .find("\nfn require_exact_operator_binding")
        .map(|offset| function_start + offset)
        .expect("Phase 7E execution function boundary must exist");

    let body = &SOURCE[function_start..function_end];

    let live_guard = body
        .find("if !args.execute_live_devnet_send")
        .expect("live execution guard must exist in runtime body");

    let first_file_read = body
        .find("fs::read_to_string")
        .expect("authorization/config file read must exist in runtime body");

    let freshness_rpc = body
        .find("RpcClient::new_with_commitment")
        .expect("freshness RPC construction must exist in runtime body");

    let phase7c_prepare = body
        .find("prepare_phase7_signed_transaction(")
        .expect("Phase 7C preparation call must exist in runtime body");

    let phase7d_submit = body
        .find("submit_phase7_once_and_readback(")
        .expect("Phase 7D submission/readback call must exist in runtime body");

    assert!(
        live_guard < first_file_read,
        "live execution flag must precede file IO"
    );

    assert!(
        first_file_read < freshness_rpc,
        "authorization/config review must precede freshness RPC"
    );

    assert!(
        freshness_rpc < phase7c_prepare,
        "freshness verification must precede Phase 7C key loading and signing"
    );

    assert!(
        phase7c_prepare < phase7d_submit,
        "signed preparation must precede the isolated one-shot submission/readback"
    );
}

#[test]
fn phase7e_requires_exact_operator_identity_and_caps() {
    for marker in [
        "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
        "PHASE7_OPERATION_ID",
        "PHASE7_IDEMPOTENCY_KEY",
        "PHASE7_NONCE",
        "PHASE7_MAX_OPERATIONS",
        "PHASE7_MAX_AMOUNT_MINOR",
        "PHASE7_RETRY_CAP",
        "--max-operations must be exactly 1",
        "--max-amount-minor must be exactly 1",
        "--retry-cap must be exactly 1",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing live execution binding `{marker}`"
        );
    }
}

#[test]
fn phase7e_requires_fresh_phase7b_authorization_before_key_loading() {
    for marker in [
        "validate_phase7b_authorization_receipt",
        "phase7_review_slot",
        "live_simulation_context_slot",
        "PHASE7E_AUTH_MAX_AGE_SLOTS",
        "require_fresh_slot",
        "prepare_phase7_signed_transaction",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing authorization freshness marker `{marker}`"
        );
    }
}

#[test]
fn phase7e_uses_phase7c_then_phase7d_without_second_execution_model() {
    assert!(SOURCE.contains("prepare_phase7_signed_transaction"));

    assert!(SOURCE.contains("submit_phase7_once_and_readback"));

    for forbidden in [
        "read_keypair_file",
        "Transaction::new_signed_with_payer",
        "send_and_confirm_transaction",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 7E must orchestrate Phase 7C/7D rather than duplicate `{forbidden}`"
        );
    }
}

#[test]
fn phase7e_explicitly_blocks_resend_after_send_receipt_exists() {
    for marker in [
        "SEND_RECEIPT_EXISTS_DO_NOT_RETRY",
        "send_receipt_path.exists()",
        "perform readback/reconciliation only",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing no-resend marker `{marker}`"
        );
    }
}

#[test]
fn phase7e_live_route_does_not_use_the_generic_non_live_wrapper() {
    assert!(
        PILOT.contains("\"phase7-execute-capped-roc-to-rox\" | \"execute-actual-roc-to-rox-send\"")
    );

    let route = PILOT.find("\"phase7-execute-capped-roc-to-rox\"").unwrap();

    let next_route = PILOT[route..]
        .find("\"proof\" | \"read-only-proof\"")
        .map(|offset| route + offset)
        .unwrap();

    let block = &PILOT[route..next_route];

    assert!(block.contains("run_phase7_live_manual_execution"));

    assert!(
        !block.contains("wrap_pilot_report"),
        "live execution must not inherit the generic wrapper's disabled wallet/signing claims"
    );
}
