//! RO:WHAT — Tests BUILD_PLAN2 Phase 1 relayer safety configuration.
//! RO:WHY — Proves the relayer is non-submitting by default and flags unsafe capped-submit scope.
//! RO:INTERACTS — RelayerConfig, relayer readiness, and rox-anchor-core safety types.
//! RO:INVARIANTS — no implicit submission; capped submit requires a valid testnet-only profile.
//! RO:SECURITY — dry-run/readiness only; no RPC, keys, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-relayer --test testnet_scope_locks.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_relayer::{review_relayer_readiness, RelayerConfig, RelayerReadinessFinding};

#[test]
fn relayer_defaults_to_non_submitting_scope() {
    let config = RelayerConfig::new(3, 16);
    let readiness = review_relayer_readiness(config);

    assert!(config.safety.submission_mode.is_non_submitting());
    assert!(readiness.ready);
    assert!(readiness.has_finding(RelayerReadinessFinding::Ready));
}

#[test]
fn relayer_readiness_rejects_capped_submit_outside_testnet_scope() {
    let unsafe_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );
    let config = RelayerConfig::new_with_safety(3, 16, unsafe_safety);
    let readiness = review_relayer_readiness(config);

    assert!(!readiness.ready);
    assert!(readiness.has_finding(RelayerReadinessFinding::UnsafeScope));
}
