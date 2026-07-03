//! RO:WHAT — Halt posture inspection notes for the ROX Anchor CLI.
//! RO:WHY — Keeps halt reporting explicit and local before coordinator/relayer behavior expands.
//! RO:INTERACTS — rox-anchor-core HaltPosture.
//! RO:INVARIANTS — halted states block acceptance; CLI does not operate halt authority.
//! RO:SECURITY — no live runtime authority, wallet, RPC, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — covered through CLI command dispatch tests.

use rox_anchor_core::HaltPosture;

pub fn halt_report() -> String {
    [
        "rox-anchor halt posture",
        &format!(
            "active_blocks_acceptance: {}",
            HaltPosture::Active.blocks_acceptance()
        ),
        &format!(
            "halted_blocks_acceptance: {}",
            HaltPosture::Halted.blocks_acceptance()
        ),
        "authority: local inspection only",
    ]
    .join("\n")
}
