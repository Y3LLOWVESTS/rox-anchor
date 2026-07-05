// RO:WHAT — CLI posture audit display tests.
// RO:WHY — Proves Phase 11 halt/challenge/recovery posture audit output is terminal-visible.
// RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-proof deterministic package review.
// RO:INVARIANTS — challenge, halt, and recovery blockers are reported from real proof review behavior.
// RO:SECURITY — no live RPC, wallet, signing, transaction submission, minting, burning, ROC release, or settlement.
// RO:TEST — cargo test -p rox-anchor-cli --test posture_audit_report.

#![forbid(unsafe_code)]

use rox_anchor_cli::run_from_args;

#[test]
fn audit_posture_command_prints_safe_posture_audit_record() {
    let output =
        run_from_args(["rox-anchor", "audit-posture"]).expect("audit-posture command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("rox-anchor audit-posture"));
    assert!(output.contains("status: proof_posture_audit_report"));
    assert!(output.contains("submission: disabled"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_client: not_enabled"));
    assert!(output.contains("runtime_authority: not_enabled"));

    assert!(output.contains("audit_record=proof-posture-audit-v1"));
    assert!(output.contains("case_count=6"));
    assert!(output.contains("accepted_cases=1"));
    assert!(output.contains("blocked_cases=5"));

    assert!(output.contains("case.0.name=clear"));
    assert!(output.contains("case.0.decision=Accepted"));
    assert!(output.contains("case.0.lifecycle_state=FinalityEligible"));
    assert!(output.contains("case.0.findings=PackageAccepted"));
    assert!(output.contains("case.0.permits_acceptance=true"));

    assert!(output.contains("case.1.name=challenge_open"));
    assert!(output.contains("case.1.challenge_posture=Open"));
    assert!(output.contains("case.1.decision=Blocked"));
    assert!(output.contains("case.1.findings=ChallengeOpen"));

    assert!(output.contains("case.2.name=challenge_accepted"));
    assert!(output.contains("case.2.challenge_posture=Accepted"));
    assert!(output.contains("case.2.decision=Blocked"));
    assert!(output.contains("case.2.findings=ChallengeAccepted"));

    assert!(output.contains("case.3.name=halted"));
    assert!(output.contains("case.3.halt_posture=Halted"));
    assert!(output.contains("case.3.decision=Blocked"));
    assert!(output.contains("case.3.lifecycle_state=Halted"));
    assert!(output.contains("case.3.findings=Halted"));

    assert!(output.contains("case.4.name=recovery_required"));
    assert!(output.contains("case.4.recovery_posture=Required"));
    assert!(output.contains("case.4.decision=Blocked"));
    assert!(output.contains("case.4.lifecycle_state=RecoveryRequired"));
    assert!(output.contains("case.4.findings=RecoveryRequired"));

    assert!(output.contains("case.5.name=recovery_in_review"));
    assert!(output.contains("case.5.recovery_posture=InReview"));
    assert!(output.contains("case.5.decision=Blocked"));
    assert!(output.contains("case.5.findings=RecoveryInReview"));

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
            "posture audit output must not contain runtime authority wording: {forbidden}"
        );
    }
}

#[test]
fn audit_posture_alias_prints_same_report() {
    let primary =
        run_from_args(["rox-anchor", "audit-posture"]).expect("audit-posture command should run");
    let alias =
        run_from_args(["rox-anchor", "posture-audit"]).expect("posture-audit alias should run");

    assert_eq!(primary, alias);
}

#[test]
fn audit_posture_output_is_deterministic() {
    let first = run_from_args(["rox-anchor", "audit-posture"])
        .expect("first audit-posture command should run");
    let second = run_from_args(["rox-anchor", "audit-posture"])
        .expect("second audit-posture command should run");

    assert_eq!(first, second);
}

#[test]
fn help_lists_audit_posture_command() {
    let output = run_from_args(["rox-anchor", "--help"]).expect("help should run");

    assert!(output.contains("audit-posture"));
    assert!(output.contains("show halt/challenge/recovery posture audit report"));
}
