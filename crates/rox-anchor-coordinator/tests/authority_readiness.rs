//! RO:WHAT — Tests coordinator BUILD_PLAN2 Phase 3 authority-aware readiness.
//! RO:WHY — Proves coordinator testnet hardening rejects missing, incomplete, and unsafe authority models.
//! RO:INTERACTS — CoordinatorConfig, coordinator readiness, AuthorityMap, OperatorRole, and RPC proof readiness.
//! RO:INVARIANTS — coordinator testnet readiness requires explicit authority model before later RPC work.
//! RO:SECURITY — identifier-only authority checks; no key loading, RPC, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-coordinator --test authority_readiness.

use rox_anchor_coordinator::{
    review_coordinator_readiness, review_coordinator_readiness_with_authorities, CoordinatorConfig,
    CoordinatorReadinessFinding,
};
use rox_anchor_core::{
    AuthorityAssignment, AuthorityKeyId, AuthorityMap, AuthoritySeparationMode, OperatorRole,
};

fn key(label: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(label).expect("authority key id should validate")
}

fn complete_authorities() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::Observer, key("observer-key-00000001")),
            AuthorityAssignment::new(OperatorRole::Coordinator, key("coordinator-key-00000002")),
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000003")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000004")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000005")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000006"),
            ),
        ],
    )
}

#[test]
fn config_only_coordinator_readiness_remains_local_dry_run_ready() {
    let readiness = review_coordinator_readiness(CoordinatorConfig::new(2, 100, 4));

    assert!(readiness.ready);
    assert!(readiness.has_finding(CoordinatorReadinessFinding::Ready));
}

#[test]
fn authority_aware_coordinator_readiness_rejects_missing_authority_model() {
    let readiness =
        review_coordinator_readiness_with_authorities(CoordinatorConfig::new(2, 100, 4), None);

    assert!(!readiness.ready);
    assert!(readiness.has_finding(CoordinatorReadinessFinding::MissingAuthorityModel));
}

#[test]
fn authority_aware_coordinator_readiness_rejects_missing_observer_role() {
    let authorities = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::Coordinator, key("coordinator-key-00000002")),
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000003")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000004")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000005")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000006"),
            ),
        ],
    );

    let readiness = review_coordinator_readiness_with_authorities(
        CoordinatorConfig::new(2, 100, 4),
        Some(&authorities),
    );

    assert!(!readiness.ready);
    assert!(
        readiness.has_finding(CoordinatorReadinessFinding::MissingRequiredAuthority(
            OperatorRole::Observer
        ))
    );
}

#[test]
fn authority_aware_coordinator_readiness_rejects_unsafe_shared_critical_authority() {
    let shared = key("shared-critical-authority-key-00000001");
    let authorities = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::Observer, key("observer-key-00000001")),
            AuthorityAssignment::new(OperatorRole::Coordinator, key("coordinator-key-00000002")),
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::MintAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::RecoveryAuthority, shared),
        ],
    );

    let readiness = review_coordinator_readiness_with_authorities(
        CoordinatorConfig::new(2, 100, 4),
        Some(&authorities),
    );

    assert!(!readiness.ready);
    assert!(readiness.has_finding(CoordinatorReadinessFinding::AuthorityModelInvalid));
}

#[test]
fn authority_aware_coordinator_readiness_accepts_complete_separated_authority_model() {
    let authorities = complete_authorities();
    let readiness = review_coordinator_readiness_with_authorities(
        CoordinatorConfig::new(2, 100, 4),
        Some(&authorities),
    );

    assert!(readiness.ready);
    assert!(readiness.has_finding(CoordinatorReadinessFinding::Ready));
}
