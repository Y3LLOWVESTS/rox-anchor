//! RO:WHAT — Tests CLI proof output for BUILD_PLAN2 Phase 4 read-only RPC adapter shape.
//! RO:WHY — Ensures proof command exposes real read-only RPC proof review without claiming submission or settlement.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-rpc-proof read-only adapter path.
//! RO:INVARIANTS — CLI proof remains local/read-only and display-safe.
//! RO:SECURITY — no live RPC, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-cli --test proof_read_only_rpc.

use rox_anchor_cli::run_from_args;

#[test]
fn proof_command_reports_read_only_rpc_shape_without_submission_claims() {
    let output = run_from_args(["rox-anchor", "proof"]).expect("proof command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("rox-anchor proof"));
    assert!(output.contains("status: read_only_rpc_adapter_shape"));
    assert!(output.contains("submission: disabled"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_client: not_enabled"));
    assert!(output.contains("current_slot: 51"));
    assert!(output.contains("observations_checked: 2"));
    assert!(output.contains("quorum_decision: Agreement"));

    for forbidden in [
        "settlement complete",
        "access granted",
        "live submission",
        "rpc submitted",
        "minted",
        "burned",
        "bridge complete",
        "loaded keypair",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "proof output must not contain runtime authority wording: {forbidden}"
        );
    }
}

#[test]
fn proof_command_includes_safe_rpc_audit_record() {
    let output = run_from_args(["rox-anchor", "proof"]).expect("proof command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("audit:"));
    assert!(output.contains("audit_record=rpc-proof-audit-v1"));
    assert!(output.contains("expected_cluster=devnet"));
    assert!(output.contains("minimum_commitment=Confirmed"));
    assert!(output.contains("observation_count=2"));
    assert!(output.contains("accepted_observations=2"));
    assert!(output.contains("required_observations=2"));
    assert!(output.contains("decision=Agreement"));
    assert!(output.contains("findings=SourceAccepted"));
    assert!(output.contains("evidence_consistent=true"));
    assert!(output.contains("display_safe=true"));
    assert!(output.contains("observation.0.source=rpc-a"));
    assert!(output.contains("observation.1.source=rpc-b"));
    assert!(output.contains("observation.0.signature=sig-proo...1111"));
    assert!(!output.contains("sig-proof-command-readonly-111111111111"));

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
    ] {
        assert!(
            !lowered.contains(forbidden),
            "proof audit output must not contain runtime authority wording: {forbidden}"
        );
    }
}

#[test]
fn proof_command_audit_output_is_deterministic() {
    let first = run_from_args(["rox-anchor", "proof"]).expect("first proof command should run");
    let second = run_from_args(["rox-anchor", "proof"]).expect("second proof command should run");

    assert_eq!(first, second);
}
