//! RO:WHAT — Local coordinator binary entry.
//! RO:WHY — Prints readiness from the local coordinator model.
//! RO:INTERACTS — rox_anchor_coordinator readiness/config.
//! RO:INVARIANTS — local model only; no live RPC, wallet, relayer, or settlement side effects.
//! RO:SECURITY — does not submit, mint, burn, bridge, or deploy.
//! RO:TEST — cargo run -p rox-anchor-coordinator.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{review_coordinator_readiness, CoordinatorConfig};

fn main() {
    let readiness = review_coordinator_readiness(CoordinatorConfig::new(2, 100, 32));

    println!(
        "rox-anchor-coordinator local model: ready={}",
        readiness.ready
    );
}
