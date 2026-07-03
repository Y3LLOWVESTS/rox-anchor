//! RO:WHAT — Commitment-level comparison for local RPC evidence.
//! RO:WHY — Lets quorum review reject observations below the expected local review threshold.
//! RO:INTERACTS — rpc.rs observations and quorum.rs classification.
//! RO:INVARIANTS — commitment is local evidence posture, not finality.
//! RO:SECURITY — no live RPC calls or settlement behavior.
//! RO:TEST — covered by quorum review tests.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RpcCommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

impl RpcCommitmentLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Processed => "processed",
            Self::Confirmed => "confirmed",
            Self::Finalized => "finalized",
        }
    }

    pub fn meets_minimum(self, minimum: Self) -> bool {
        self >= minimum
    }
}
