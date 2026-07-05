//! RO:WHAT — Tests relayer BUILD_PLAN2 Phase 3 authority-aware readiness.
//! RO:WHY — Proves testnet hardening readiness rejects missing, incomplete, and unsafe authority models.
//! RO:INTERACTS — RelayerConfig, relayer readiness, AuthorityMap, and OperatorRole.
//! RO:INVARIANTS — relayer testnet readiness requires explicit authority model before later RPC/submission work.
//! RO:SECURITY — identifier-only authority checks; no key loading, RPC, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-relayer --test authority_readiness.

use rox_anchor_core::{
    AuthorityAssignment, AuthorityKeyId, AuthorityMap, AuthoritySeparationMode, OperatorRole,
};
use rox_anchor_relayer::{
    review_relayer_readiness, review_relayer_readiness_with_authorities, RelayerConfig,
    RelayerReadinessFinding,
};

fn key(label: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(label).expect("authority key id should validate")
}

fn complete_authorities() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::Relayer, key("relayer-key-00000001")),
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000002")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000003")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000004")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000005"),
            ),
        ],
    )
}

#[test]
fn config_only_relayer_readiness_remains_local_dry_run_ready() {
    let readiness = review_relayer_readiness(RelayerConfig::new(3, 16));

    assert!(readiness.ready);
    assert!(readiness.has_finding(RelayerReadinessFinding::Ready));
}

#[test]
fn authority_aware_relayer_readiness_rejects_missing_authority_model() {
    let readiness = review_relayer_readiness_with_authorities(RelayerConfig::new(3, 16), None);

    assert!(!readiness.ready);
    assert!(readiness.has_finding(RelayerReadinessFinding::MissingAuthorityModel));
}

#[test]
fn authority_aware_relayer_readiness_rejects_missing_relayer_role() {
    let authorities = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000002")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000003")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000004")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000005"),
            ),
        ],
    );

    let readiness =
        review_relayer_readiness_with_authorities(RelayerConfig::new(3, 16), Some(&authorities));

    assert!(!readiness.ready);
    assert!(
        readiness.has_finding(RelayerReadinessFinding::MissingRequiredAuthority(
            OperatorRole::Relayer
        ))
    );
}

#[test]
fn authority_aware_relayer_readiness_rejects_unsafe_shared_critical_authority() {
    let shared = key("shared-critical-authority-key-00000001");
    let authorities = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::Relayer, key("relayer-key-00000001")),
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::MintAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::RecoveryAuthority, shared),
        ],
    );

    let readiness =
        review_relayer_readiness_with_authorities(RelayerConfig::new(3, 16), Some(&authorities));

    assert!(!readiness.ready);
    assert!(readiness.has_finding(RelayerReadinessFinding::AuthorityModelInvalid));
}

#[test]
fn authority_aware_relayer_readiness_accepts_complete_separated_authority_model() {
    let authorities = complete_authorities();
    let readiness =
        review_relayer_readiness_with_authorities(RelayerConfig::new(3, 16), Some(&authorities));

    assert!(readiness.ready);
    assert!(readiness.has_finding(RelayerReadinessFinding::Ready));
}
