//! RO:WHAT — Minimal local RPC proof binary entry.
//! RO:WHY — Prints readiness from the local RPC evidence model.
//! RO:INTERACTS — rox_anchor_rpc_proof readiness/config.
//! RO:INVARIANTS — local evidence model only; no live RPC calls or transaction submission.
//! RO:SECURITY — does not submit, mint, burn, bridge, deploy, or settle.
//! RO:TEST — cargo run -p rox-anchor-rpc-proof.

#![forbid(unsafe_code)]

use rox_anchor_rpc_proof::{review_rpc_proof_readiness, RpcProofConfig};

fn main() {
    let readiness = review_rpc_proof_readiness(RpcProofConfig::new(2, 100));

    println!(
        "rox-anchor-rpc-proof local evidence model: ready={}",
        readiness.ready
    );
}
