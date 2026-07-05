//! RO:WHAT — Readiness review for local relayer dry-run and authority-aware testnet posture.
//! RO:WHY — Fails closed when retry, receipt, scope, or operator authority settings are unusable.
//! RO:INTERACTS — RelayerConfig, rox-anchor-core safety validation, AuthorityMap, and future CLI/service output.
//! RO:INVARIANTS — config-only readiness is local dry-run; authority-aware readiness is required before testnet hardening.
//! RO:SECURITY — no network calls, key loading, wallet calls, transaction submission, mint/burn, or settlement.
//! RO:TEST — covered by readiness, scope-lock, and authority-readiness tests.

use rox_anchor_core::{AuthorityMap, OperatorRole};

use crate::RelayerConfig;

const RELAYER_REQUIRED_AUTHORITY_ROLES: [OperatorRole; 3] = [
    OperatorRole::Relayer,
    OperatorRole::HaltAuthority,
    OperatorRole::RecoveryAuthority,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelayerReadinessFinding {
    Ready,
    MissingRetryLimit,
    MissingReceiptCapacity,
    UnsafeScope,
    MissingAuthorityModel,
    AuthorityModelInvalid,
    MissingRequiredAuthority(OperatorRole),
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
    review_relayer_readiness_inner(config, None, false)
}

pub fn review_relayer_readiness_with_authorities(
    config: RelayerConfig,
    authorities: Option<&AuthorityMap>,
) -> RelayerReadiness {
    review_relayer_readiness_inner(config, authorities, true)
}

fn review_relayer_readiness_inner(
    config: RelayerConfig,
    authorities: Option<&AuthorityMap>,
    require_authorities: bool,
) -> RelayerReadiness {
    let mut findings = Vec::new();

    if config.max_attempts == 0 {
        findings.push(RelayerReadinessFinding::MissingRetryLimit);
    }

    if config.max_receipts == 0 {
        findings.push(RelayerReadinessFinding::MissingReceiptCapacity);
    }

    if config.safety.validate().is_err() {
        findings.push(RelayerReadinessFinding::UnsafeScope);
    }

    if require_authorities {
        review_relayer_authorities(authorities, &mut findings);
    }

    if findings.is_empty() {
        findings.push(RelayerReadinessFinding::Ready);
    }

    RelayerReadiness {
        ready: findings == vec![RelayerReadinessFinding::Ready],
        findings,
    }
}

fn review_relayer_authorities(
    authorities: Option<&AuthorityMap>,
    findings: &mut Vec<RelayerReadinessFinding>,
) {
    let Some(authorities) = authorities else {
        findings.push(RelayerReadinessFinding::MissingAuthorityModel);
        return;
    };

    if authorities.validate_critical_authorities().is_err() {
        findings.push(RelayerReadinessFinding::AuthorityModelInvalid);
    }

    for role in RELAYER_REQUIRED_AUTHORITY_ROLES {
        if authorities.authority_for_role(role).is_none() {
            findings.push(RelayerReadinessFinding::MissingRequiredAuthority(role));
        }
    }
}
