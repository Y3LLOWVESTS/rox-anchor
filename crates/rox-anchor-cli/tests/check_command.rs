//! RO:WHAT — CLI command smoke tests for local ROX Anchor proof inspection.
//! RO:WHY — Verifies `rox-anchor check` is backed by proof/core behavior, not fake success strings.
//! RO:INTERACTS — rox_anchor_cli::run_from_args.
//! RO:INVARIANTS — deterministic decisions, status labels, and findings.
//! RO:SECURITY — tests local command rendering only; no network/wallet/RPC/value movement.
//! RO:TEST — cargo test -p rox-anchor-cli --test check_command.

use rox_anchor_cli::run_from_args;

#[test]
fn check_default_fixture_prints_valid_review() {
    let output = run_from_args(["rox-anchor", "check"]).unwrap();

    assert!(output.contains("command: check"));
    assert!(output.contains("fixture: valid"));
    assert!(output.contains("decision: ValidForLocalReview"));
    assert!(output.contains("status_label: Finality eligible"));
    assert!(output.contains("findings:"));
    assert!(output.contains("- PackageAccepted [Info]"));
}

#[test]
fn check_halt_fixture_prints_halt_blocked_review() {
    let output = run_from_args(["rox-anchor", "check", "--fixture", "halt"]).unwrap();

    assert!(output.contains("fixture: halt"));
    assert!(output.contains("decision: HaltBlocked"));
    assert!(output.contains("status_label: Halted"));
    assert!(output.contains("- Halted [Block]"));
}

#[test]
fn check_mismatch_fixture_prints_rejection_findings() {
    let output = run_from_args(["rox-anchor", "check", "--fixture=mismatch"]).unwrap();

    assert!(output.contains("fixture: mismatch"));
    assert!(output.contains("decision: Rejected"));
    assert!(output.contains("status_label: Failed"));
    assert!(output.contains("- SourceDomainMismatch [Reject]"));
    assert!(output.contains("- TokenAccountMismatch [Reject]"));
    assert!(output.contains("- NonceMismatch [Reject]"));
}
