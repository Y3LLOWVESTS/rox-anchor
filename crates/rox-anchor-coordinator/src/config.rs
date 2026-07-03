//! RO:WHAT — Local coordinator configuration.
//! RO:WHY — Keeps queue capacity and RPC evidence thresholds explicit.
//! RO:INTERACTS — queue, readiness, and decision review.
//! RO:INVARIANTS — config is local review posture only, not runtime authority.
//! RO:SECURITY — no endpoints, secrets, wallet calls, or submission toggles.
//! RO:TEST — covered by coordinator readiness and queue tests.

use rox_anchor_rpc_proof::RpcProofConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoordinatorConfig {
    pub rpc: RpcProofConfig,
    pub max_queue_items: usize,
}

impl CoordinatorConfig {
    pub fn new(required_observations: u16, stale_after_slots: u64, max_queue_items: usize) -> Self {
        Self {
            rpc: RpcProofConfig::new(required_observations, stale_after_slots),
            max_queue_items,
        }
    }
}
