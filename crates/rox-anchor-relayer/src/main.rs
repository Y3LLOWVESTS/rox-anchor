//! RO:WHAT — Local relayer dry-run binary entry.
//! RO:WHY — Prints readiness from the local relayer model.
//! RO:INTERACTS — rox_anchor_relayer readiness/config.
//! RO:INVARIANTS — dry-run model only; no live submission or value movement.
//! RO:SECURITY — does not call wallet/RPC, mint, burn, bridge, or deploy.
//! RO:TEST — cargo run -p rox-anchor-relayer.

#![forbid(unsafe_code)]

use rox_anchor_relayer::{review_relayer_readiness, RelayerConfig};

fn main() {
    let readiness = review_relayer_readiness(RelayerConfig::new(3, 128));

    println!(
        "rox-anchor-relayer dry-run model: ready={}",
        readiness.ready
    );
}
