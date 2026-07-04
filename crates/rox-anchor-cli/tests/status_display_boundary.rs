// RO:WHAT — Active status-display boundary test for downstream UI/CrabLink-style surfaces.
// RO:WHY — Proves display text comes from rox-anchor-core safe labels and CLI dispatch, not invented finality.
// RO:INTERACTS — rox-anchor-cli status command and rox-anchor-core safe status labels.
// RO:INVARIANTS — status output is display-only; it does not claim settlement, paid unlock, live submission, mint, or burn.
// RO:SECURITY — no live RPC, wallet, deployment, minting, burning, bridge settlement, staking, liquidity, or exchange behavior.
// RO:TEST — cargo test -p rox-anchor-cli --test status_display_boundary.

#![forbid(unsafe_code)]

use rox_anchor_cli::run_from_args;
use rox_anchor_core::{SAFE_STATUS_LABELS, STATUS_FINALITY_ELIGIBLE};

#[test]
fn status_command_uses_core_safe_labels() {
    let output = run_from_args(["rox-anchor", "status"]).unwrap();

    assert!(output.contains("rox-anchor status labels"));
    assert!(output.contains(&format!(
        "finality_candidate_label: {STATUS_FINALITY_ELIGIBLE}"
    )));
    assert!(output.contains("safe_labels:"));

    for label in SAFE_STATUS_LABELS {
        assert!(
            output.contains(&format!("  - {label}")),
            "missing safe status label: {label}"
        );
    }
}

#[test]
fn status_command_does_not_claim_runtime_authority_or_settlement() {
    let output = run_from_args(["rox-anchor", "status"]).unwrap();
    let lowered = output.to_ascii_lowercase();

    for forbidden in [
        "settled",
        "settlement complete",
        "paid unlocked",
        "access granted",
        "live submission",
        "production",
        "deployed",
        "wallet call",
        "rpc submitted",
        "minted",
        "burned",
        "bridge complete",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "status output must not contain authority/settlement wording: {forbidden}"
        );
    }
}

#[test]
fn status_command_is_deterministic() {
    let first = run_from_args(["rox-anchor", "status"]).unwrap();
    let second = run_from_args(["rox-anchor", "status"]).unwrap();

    assert_eq!(first, second);
}
