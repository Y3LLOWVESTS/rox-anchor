//! RO:WHAT — Readiness review for local RPC proof configuration.
//! RO:WHY — Fails closed when quorum thresholds or testnet scope settings are unusable.
//! RO:INTERACTS — config.rs, quorum.rs, and rox-anchor-core safety validation.
//! RO:INVARIANTS — readiness is local/testnet configuration posture, not runtime authority.
//! RO:SECURITY — no network calls, wallet calls, live RPC submission, or settlement behavior.
//! RO:TEST — covered by readiness and testnet scope-lock tests.

use crate::RpcProofConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RpcProofReadinessFinding {
    Ready,
    MissingRequiredObservations,
    MissingStaleSlotWindow,
    UnsafeScope,
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

    if config.safety.validate().is_err() {
        findings.push(RpcProofReadinessFinding::UnsafeScope);
    }

    if findings.is_empty() {
        findings.push(RpcProofReadinessFinding::Ready);
    }

    RpcProofReadiness {
        ready: findings == vec![RpcProofReadinessFinding::Ready],
        findings,
    }
}
