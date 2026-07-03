//! RO:WHAT — Readiness review for local RPC proof configuration.
//! RO:WHY — Fails closed when quorum thresholds are unusable.
//! RO:INTERACTS — config.rs and future CLI/coordinator readiness output.
//! RO:INVARIANTS — readiness is local configuration posture, not runtime authority.
//! RO:SECURITY — no network calls, wallet calls, or settlement behavior.
//! RO:TEST — covered by readiness unit tests.

use crate::RpcProofConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcProofReadinessFinding {
    Ready,
    MissingRequiredObservations,
    MissingStaleSlotWindow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcProofReadiness {
    pub ready: bool,
    pub findings: Vec<RpcProofReadinessFinding>,
}

impl RpcProofReadiness {
    pub fn has_finding(&self, finding: RpcProofReadinessFinding) -> bool {
        self.findings.contains(&finding)
    }
}

pub fn review_rpc_proof_readiness(config: RpcProofConfig) -> RpcProofReadiness {
    let mut findings = Vec::new();

    if config.required_observations == 0 {
        findings.push(RpcProofReadinessFinding::MissingRequiredObservations);
    }

    if config.stale_after_slots == 0 {
        findings.push(RpcProofReadinessFinding::MissingStaleSlotWindow);
    }

    if findings.is_empty() {
        findings.push(RpcProofReadinessFinding::Ready);
    }

    RpcProofReadiness {
        ready: findings == vec![RpcProofReadinessFinding::Ready],
        findings,
    }
}
