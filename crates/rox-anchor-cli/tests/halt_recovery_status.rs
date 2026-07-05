//! RO:WHAT — Tests CLI status output for Phase 12 halt/recovery drill posture.
//! RO:WHY — BUILD_PLAN2 Phase 12 requires status output to reflect halt and recovery blockers.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and core AnchorOperationalPosture.
//! RO:INVARIANTS — status is display-only and must not claim finality, settlement, or live submission.
//! RO:SECURITY — no RPC, keypair loading, wallet, transaction, mint, burn, staking, liquidity, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test halt_recovery_status.

use rox_anchor_cli::run_from_args;

#[test]
fn status_reports_halt_and_recovery_blockers_without_runtime_claims() {
    let output = run_from_args(["rox-anchor", "status"]).expect("status command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("phase12_kill_switch_surface: local_drill_only"));
    assert!(output.contains("halted_blocks_acceptance: true"));
    assert!(output.contains("halted_blocks_simulation: true"));
    assert!(output.contains("halted_blocks_submission: true"));
    assert!(output.contains("halted_blocks_finalization: true"));
    assert!(output.contains("recovery_resolved_blocks_submission: false"));
    assert!(output.contains("recovery_resolved_blocks_finalization: false"));

    for forbidden in [
        "settlement complete",
        "network submitted=true",
        "rpc submitted",
        "loaded keypair",
        "minted",
        "burned",
        "bridge complete",
        "finalized on chain",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "status output must not claim runtime authority: {forbidden}"
        );
    }
}
