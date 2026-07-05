//! RO:WHAT — Tests the CLI capped testnet submission authorization report.
//! RO:WHY — BUILD_PLAN2 Phase 8 requires explicit CLI/config gates before capped submit is reachable.
//! RO:INTERACTS — rox_anchor_cli command dispatch and rox-anchor-relayer capped submit model.
//! RO:INVARIANTS — command can authorize only in report form; it never attempts or claims network submission.
//! RO:SECURITY — no RPC, wallet/key loading, mint, burn, ROC release, settlement, or finality.
//! RO:TEST — cargo test -p rox-anchor-cli --test capped_submit_report.

use rox_anchor_cli::{run_from_args, CliError};

#[test]
fn capped_submit_report_requires_explicit_operator_approval() {
    let output = run_from_args(["rox-anchor", "submit-capped"]).expect("report should render");

    assert!(output.contains("command: submit-capped"));
    assert!(output.contains("mode: TestnetSubmitCapped"));
    assert!(output.contains("explicit_operator_approval: false"));
    assert!(output.contains("receipt_persisted: false"));
    assert!(output.contains("proof_decision: Accepted"));
    assert!(output.contains("relayer_status: DryRunAccepted"));
    assert!(output.contains("simulation_status: Simulated"));
    assert!(output.contains("capped_submit_status: MissingExplicitOperatorApproval"));
    assert!(output.contains("authorized: false"));
    assert!(output.contains("live_submission_permitted: false"));
    assert!(output.contains("live_submission_attempted: false"));
    assert!(output.contains("network_submitted: false"));
}

#[test]
fn capped_submit_report_authorizes_only_as_non_executing_report() {
    let output = run_from_args([
        "rox-anchor",
        "submit-capped",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
    ])
    .expect("authorized report should render");

    assert!(output.contains("explicit_operator_approval: true"));
    assert!(output.contains("receipt_persisted: true"));
    assert!(output.contains("capped_submit_status: Authorized"));
    assert!(output.contains("authorized: true"));
    assert!(output.contains("live_submission_permitted: true"));
    assert!(output.contains("live_submission_attempted: false"));
    assert!(output.contains("network_submitted: false"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("rpc_submission: disabled_in_cli_report"));
    assert!(output.contains("mint_burn_execution: disabled_in_cli_report"));
    assert!(output.contains("roc_release: disabled_in_cli_report"));
    assert!(output.contains("finality_claim: none"));
    assert!(output.contains("settlement_claim: none"));
}

#[test]
fn capped_submit_report_surfaces_blocked_proof_as_not_authorized() {
    let output = run_from_args([
        "rox-anchor",
        "submit-capped",
        "--fixture=blocked",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
    ])
    .expect("blocked report should render");

    assert!(output.contains("fixture: blocked"));
    assert!(output.contains("proof_decision: Blocked"));
    assert!(output.contains("relayer_status: ProofBlocked"));
    assert!(output.contains("simulation_status: ProofNotAccepted"));
    assert!(output.contains("capped_submit_status: SimulationNotAccepted"));
    assert!(output.contains("authorized: false"));
    assert!(output.contains("network_submitted: false"));
}

#[test]
fn capped_submit_report_exposes_cap_rejections() {
    let retry_output = run_from_args([
        "rox-anchor",
        "submit-capped",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
        "--attempts=3",
    ])
    .expect("retry cap report should render");

    assert!(retry_output.contains("capped_submit_status: RetryCapExceeded"));
    assert!(retry_output.contains("network_submitted: false"));

    let amount_output = run_from_args([
        "rox-anchor",
        "submit-capped",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
        "--amount-units=101",
    ])
    .expect("amount cap report should render");

    assert!(amount_output.contains("capped_submit_status: AmountCapExceeded"));
    assert!(amount_output.contains("network_submitted: false"));
}

#[test]
fn capped_submit_unknown_flag_is_rejected() {
    let error = run_from_args(["rox-anchor", "submit-capped", "--live-send-now"]).unwrap_err();

    assert_eq!(
        error,
        CliError::UnknownSubmitFlag("--live-send-now".to_string())
    );
}
