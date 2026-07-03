//! RO:WHAT — Bounded retry policy for local relayer dry-runs.
//! RO:WHY — Prevents retry storms before live submission behavior exists.
//! RO:INTERACTS — RelayerConfig and submit.rs.
//! RO:INVARIANTS — requested attempts are capped by configured max_attempts.
//! RO:SECURITY — no sleeping, network calls, submission, wallet, mint, burn, or settlement.
//! RO:TEST — covered by retry policy tests.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    pub max_attempts: u8,
}

impl RetryPolicy {
    pub fn new(max_attempts: u8) -> Self {
        Self { max_attempts }
    }

    pub fn plan_attempts(self, requested_attempts: u8) -> RetryPlan {
        RetryPlan {
            requested_attempts,
            allowed_attempts: requested_attempts.min(self.max_attempts),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPlan {
    pub requested_attempts: u8,
    pub allowed_attempts: u8,
}

impl RetryPlan {
    pub fn was_capped(self) -> bool {
        self.allowed_attempts < self.requested_attempts
    }
}
