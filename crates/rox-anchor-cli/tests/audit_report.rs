// RO:WHAT — CLI coordinator audit display tests.
// RO:WHY — Proves Phase 11 coordinator audit output is visible through terminal command dispatch.
// RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-coordinator audit records.
// RO:INVARIANTS — report is deterministic, display-safe, and does not claim settlement or submission.
// RO:SECURITY — no live RPC, wallet, signing, transaction submission, minting, burning, ROC release, or settlement.
// RO:TEST — cargo test -p rox-anchor-cli --test audit_report.

#![forbid(unsafe_code)]

use rox_anchor_cli::run_from_args;

#[test]
fn audit_command_prints_safe_coordinator_audit_record() {
    let output = run_from_args(["rox-anchor", "audit"]).expect("audit command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("rox-anchor audit"));
    assert!(output.contains("status: coordinator_audit_report"));
    assert!(output.contains("submission: disabled"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_client: not_enabled"));
    assert!(output.contains("runtime_authority: not_enabled"));

    assert!(output.contains("audit_record=coordinator-testnet-audit-v1"));
    assert!(output.contains("rpc_decision=Agreement"));
    assert!(output.contains("proof_decision=Accepted"));
    assert!(output.contains("coordinator_status=Accepted"));
    assert!(output.contains("permits_simulation=true"));
    assert!(output.contains("status_consistent=true"));
    assert!(output.contains("display_safe=true"));
    assert!(output.contains("rpc_findings=SourceAccepted"));
    assert!(output.contains("proof_findings=Info:PackageAccepted"));

    for forbidden in [
        "settlement complete",
        "access granted",
        "live submission",
        "rpc submitted",
        "minted",
        "burned",
        "bridge complete",
        "loaded keypair",
        "loaded wallet",
        "roc released",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "audit output must not contain runtime authority wording: {forbidden}"
        );
    }
}

#[test]
fn audit_command_output_is_deterministic() {
    let first = run_from_args(["rox-anchor", "audit"]).expect("first audit command should run");
    let second = run_from_args(["rox-anchor", "audit"]).expect("second audit command should run");

    assert_eq!(first, second);
}

#[test]
fn help_lists_audit_command() {
    let output = run_from_args(["rox-anchor", "--help"]).expect("help should run");

    assert!(output.contains("audit"));
    assert!(output.contains("show coordinator audit report"));
}
