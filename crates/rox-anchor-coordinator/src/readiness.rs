//! RO:WHAT — Readiness review for the local coordinator model.
//! RO:WHY — Fails closed when queue capacity or RPC proof settings are unusable.
//! RO:INTERACTS — CoordinatorConfig and rox-anchor-rpc-proof readiness.
//! RO:INVARIANTS — readiness is local configuration posture, not runtime authority.
//! RO:SECURITY — no network calls, wallet calls, transaction submission, or settlement.
//! RO:TEST — covered by readiness unit tests.

use rox_anchor_rpc_proof::review_rpc_proof_readiness;

use crate::CoordinatorConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorReadinessFinding {
    Ready,
    MissingQueueCapacity,
    RpcProofNotReady,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorReadiness {
    pub ready: bool,
    pub findings: Vec<CoordinatorReadinessFinding>,
}

impl CoordinatorReadiness {
    pub fn has_finding(&self, finding: CoordinatorReadinessFinding) -> bool {
        self.findings.contains(&finding)
    }
}

pub fn review_coordinator_readiness(config: CoordinatorConfig) -> CoordinatorReadiness {
    let mut findings = Vec::new();

    if config.max_queue_items == 0 {
        findings.push(CoordinatorReadinessFinding::MissingQueueCapacity);
    }

    let rpc_readiness = review_rpc_proof_readiness(config.rpc);
    if !rpc_readiness.ready {
        findings.push(CoordinatorReadinessFinding::RpcProofNotReady);
    }

    if findings.is_empty() {
        findings.push(CoordinatorReadinessFinding::Ready);
    }

    CoordinatorReadiness {
        ready: findings == vec![CoordinatorReadinessFinding::Ready],
        findings,
    }
}
