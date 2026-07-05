//! RO:WHAT — Readiness review for the local coordinator model and authority-aware testnet posture.
//! RO:WHY — Fails closed when queue capacity, RPC proof settings, scope, or operator authority settings are unusable.
//! RO:INTERACTS — CoordinatorConfig, rox-anchor-rpc-proof readiness, core safety validation, and AuthorityMap.
//! RO:INVARIANTS — config-only readiness is local dry-run; authority-aware readiness is required before testnet hardening.
//! RO:SECURITY — no network calls, key loading, wallet calls, transaction submission, mint/burn, or settlement.
//! RO:TEST — covered by readiness, scope-lock, and authority-readiness tests.

use rox_anchor_core::{AuthorityMap, OperatorRole};
use rox_anchor_rpc_proof::review_rpc_proof_readiness;

use crate::CoordinatorConfig;

const COORDINATOR_REQUIRED_AUTHORITY_ROLES: [OperatorRole; 4] = [
    OperatorRole::Observer,
    OperatorRole::Coordinator,
    OperatorRole::HaltAuthority,
    OperatorRole::RecoveryAuthority,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorReadinessFinding {
    Ready,
    MissingQueueCapacity,
    RpcProofNotReady,
    UnsafeScope,
    MissingAuthorityModel,
    AuthorityModelInvalid,
    MissingRequiredAuthority(OperatorRole),
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
    review_coordinator_readiness_inner(config, None, false)
}

pub fn review_coordinator_readiness_with_authorities(
    config: CoordinatorConfig,
    authorities: Option<&AuthorityMap>,
) -> CoordinatorReadiness {
    review_coordinator_readiness_inner(config, authorities, true)
}

fn review_coordinator_readiness_inner(
    config: CoordinatorConfig,
    authorities: Option<&AuthorityMap>,
    require_authorities: bool,
) -> CoordinatorReadiness {
    let mut findings = Vec::new();

    if config.max_queue_items == 0 {
        findings.push(CoordinatorReadinessFinding::MissingQueueCapacity);
    }

    if config.safety.validate().is_err() {
        findings.push(CoordinatorReadinessFinding::UnsafeScope);
    }

    let rpc_readiness = review_rpc_proof_readiness(config.rpc);
    if !rpc_readiness.ready {
        findings.push(CoordinatorReadinessFinding::RpcProofNotReady);
    }

    if require_authorities {
        review_coordinator_authorities(authorities, &mut findings);
    }

    if findings.is_empty() {
        findings.push(CoordinatorReadinessFinding::Ready);
    }

    CoordinatorReadiness {
        ready: findings == vec![CoordinatorReadinessFinding::Ready],
        findings,
    }
}

fn review_coordinator_authorities(
    authorities: Option<&AuthorityMap>,
    findings: &mut Vec<CoordinatorReadinessFinding>,
) {
    let Some(authorities) = authorities else {
        findings.push(CoordinatorReadinessFinding::MissingAuthorityModel);
        return;
    };

    if authorities.validate_critical_authorities().is_err() {
        findings.push(CoordinatorReadinessFinding::AuthorityModelInvalid);
    }

    for role in COORDINATOR_REQUIRED_AUTHORITY_ROLES {
        if authorities.authority_for_role(role).is_none() {
            findings.push(CoordinatorReadinessFinding::MissingRequiredAuthority(role));
        }
    }
}
