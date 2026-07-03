//! RO:WHAT — Configuration for the local relayer dry-run model.
//! RO:WHY — Keeps retry and receipt limits explicit.
//! RO:INTERACTS — readiness, retry, submit, and receipts modules.
//! RO:INVARIANTS — config controls local dry-run behavior only.
//! RO:SECURITY — no RPC endpoints, secrets, keypairs, or live submission toggles.
//! RO:TEST — covered by readiness and retry tests.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelayerConfig {
    pub max_attempts: u8,
    pub max_receipts: usize,
}

impl RelayerConfig {
    pub fn new(max_attempts: u8, max_receipts: usize) -> Self {
        Self {
            max_attempts,
            max_receipts,
        }
    }
}
