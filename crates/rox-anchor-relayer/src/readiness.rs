//! RO:WHAT — Readiness review for local relayer dry-run configuration.
//! RO:WHY — Fails closed when retry or receipt limits are unusable.
//! RO:INTERACTS — RelayerConfig and future CLI/service output.
//! RO:INVARIANTS — readiness is local configuration posture, not live authority.
//! RO:SECURITY — no network calls, wallet calls, transaction submission, or settlement.
//! RO:TEST — covered by readiness unit tests.

use crate::RelayerConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelayerReadinessFinding {
    Ready,
    MissingRetryLimit,
    MissingReceiptCapacity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerReadiness {
    pub ready: bool,
    pub findings: Vec<RelayerReadinessFinding>,
}

impl RelayerReadiness {
    pub fn has_finding(&self, finding: RelayerReadinessFinding) -> bool {
        self.findings.contains(&finding)
    }
}

pub fn review_relayer_readiness(config: RelayerConfig) -> RelayerReadiness {
    let mut findings = Vec::new();

    if config.max_attempts == 0 {
        findings.push(RelayerReadinessFinding::MissingRetryLimit);
    }

    if config.max_receipts == 0 {
        findings.push(RelayerReadinessFinding::MissingReceiptCapacity);
    }

    if findings.is_empty() {
        findings.push(RelayerReadinessFinding::Ready);
    }

    RelayerReadiness {
        ready: findings == vec![RelayerReadinessFinding::Ready],
        findings,
    }
}
