//! RO:WHAT — Local coordinator observation wrapper.
//! RO:WHY — Gives coordinator code an owned observation input type before service intake exists.
//! RO:INTERACTS — rox-anchor-rpc-proof RpcObservation.
//! RO:INVARIANTS — observation records are evidence inputs only, not finality or settlement.
//! RO:SECURITY — no live RPC calls or wallet calls.
//! RO:TEST — indirectly covered through coordinator decision tests.

use rox_anchor_rpc_proof::RpcObservation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorObservation {
    pub rpc: RpcObservation,
}

impl CoordinatorObservation {
    pub fn new(rpc: RpcObservation) -> Self {
        Self { rpc }
    }
}
