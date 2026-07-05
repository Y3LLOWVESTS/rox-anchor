// RO:WHAT — CLI relayer audit display tests.
// RO:WHY — Proves Phase 11 relayer/simulation/capped-submit audit output is terminal-visible.
// RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-relayer audit records.
// RO:INVARIANTS — report is deterministic, display-safe, and never claims network submission or settlement.
// RO:SECURITY — no live RPC, wallet, signing, transaction submission, minting, burning, ROC release, or settlement.
// RO:TEST — cargo test -p rox-anchor-cli --test relayer_audit_report.

#![forbid(unsafe_code)]

use rox_anchor_cli::run_from_args;

#[test]
fn audit_relayer_command_prints_safe_relayer_pipeline_audit_record() {
    let output =
        run_from_args(["rox-anchor", "audit-relayer"]).expect("audit-relayer command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("rox-anchor audit-relayer"));
    assert!(output.contains("status: relayer_simulation_capped_audit_report"));
    assert!(output.contains("submission: capped_testnet_report_only"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_client: not_enabled"));
    assert!(output.contains("runtime_authority: not_enabled"));

    assert!(output.contains("audit_record=relayer-testnet-audit-v1"));
    assert!(output.contains("target=audit-cli-relayer-target"));
    assert!(output.contains("relayer_status=DryRunAccepted"));
    assert!(output.contains("proof_decision=Accepted"));
    assert!(output.contains("attempts_used=1"));
    assert!(output.contains("simulation_status=Simulated"));
    assert!(output.contains("instruction_count=1"));
    assert!(output.contains("capped_submission_status=Authorized"));
    assert!(output.contains("requested_attempts=1"));
    assert!(output.contains("requested_operations=1"));
    assert!(output.contains("amount_units=10"));
    assert!(output.contains("receipt_persisted=true"));
    assert!(output.contains("authorized=true"));
    assert!(output.contains("live_submission_permitted=true"));
    assert!(output.contains("live_submission_attempted=false"));
    assert!(output.contains("network_submitted=false"));
    assert!(output.contains("pipeline_consistent=true"));
    assert!(output.contains("display_safe=true"));

    for forbidden in [
        "settlement complete",
        "access granted",
        "rpc submitted",
        "network submitted=true",
        "minted",
        "burned",
        "bridge complete",
        "loaded keypair",
        "loaded wallet",
        "roc released",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "relayer audit output must not contain runtime authority wording: {forbidden}"
        );
    }
}

#[test]
fn audit_relayer_alias_prints_same_report() {
    let primary =
        run_from_args(["rox-anchor", "audit-relayer"]).expect("audit-relayer command should run");
    let alias =
        run_from_args(["rox-anchor", "relayer-audit"]).expect("relayer-audit alias should run");

    assert_eq!(primary, alias);
}

#[test]
fn audit_relayer_output_is_deterministic() {
    let first = run_from_args(["rox-anchor", "audit-relayer"])
        .expect("first audit-relayer command should run");
    let second = run_from_args(["rox-anchor", "audit-relayer"])
        .expect("second audit-relayer command should run");

    assert_eq!(first, second);
}

#[test]
fn help_lists_audit_relayer_command() {
    let output = run_from_args(["rox-anchor", "--help"]).expect("help should run");

    assert!(output.contains("audit-relayer"));
    assert!(output.contains("show relayer simulation and capped submission audit report"));
}
