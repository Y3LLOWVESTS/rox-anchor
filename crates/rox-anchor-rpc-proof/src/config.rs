//! RO:WHAT — Configuration for local RPC proof quorum review.
//! RO:WHY — Keeps thresholds explicit instead of hard-coded in quorum logic.
//! RO:INTERACTS — readiness.rs and quorum.rs.
//! RO:INVARIANTS — config is local review policy only; it does not authorize live RPC or finality.
//! RO:SECURITY — no network endpoints or credentials are used.
//! RO:TEST — covered by readiness tests.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RpcProofConfig {
    pub required_observations: u16,
    pub stale_after_slots: u64,
}

impl RpcProofConfig {
    pub fn new(required_observations: u16, stale_after_slots: u64) -> Self {
        Self {
            required_observations,
            stale_after_slots,
        }
    }
}
