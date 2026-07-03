//! RO:WHAT — Recovery posture inspection notes for the ROX Anchor CLI.
//! RO:WHY — Keeps recovery reporting explicit and local before recovery handlers expand.
//! RO:INTERACTS — rox-anchor-core RecoveryPosture.
//! RO:INVARIANTS — recovery-required states block acceptance until resolved by explicit code.
//! RO:SECURITY — no live recovery authority, wallet, RPC, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — covered through CLI command dispatch tests.

use rox_anchor_core::RecoveryPosture;

pub fn recovery_report() -> String {
    [
        "rox-anchor recovery posture",
        &format!(
            "not_required_blocks_acceptance: {}",
            RecoveryPosture::NotRequired.blocks_acceptance()
        ),
        &format!(
            "required_blocks_acceptance: {}",
            RecoveryPosture::Required.blocks_acceptance()
        ),
        "authority: local inspection only",
    ]
    .join("\n")
}
