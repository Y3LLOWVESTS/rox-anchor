//! RO:WHAT — Tests BUILD_PLAN2 Phase 1 coordinator safety configuration.
//! RO:WHY — Proves coordinator readiness inherits core/RPC testnet scope locks.
//! RO:INTERACTS — CoordinatorConfig, coordinator readiness, RPC proof config, and core safety types.
//! RO:INVARIANTS — no public/mainnet scope; capped submit must be valid testnet-only before readiness.
//! RO:SECURITY — readiness only; no RPC, keys, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-coordinator --test testnet_scope_locks.

use rox_anchor_coordinator::{
    review_coordinator_readiness, CoordinatorConfig, CoordinatorReadinessFinding,
};
use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};

#[test]
fn coordinator_defaults_to_non_submitting_scope() {
    let config = CoordinatorConfig::new(2, 100, 4);
    let readiness = review_coordinator_readiness(config);

    assert!(config.safety.submission_mode.is_non_submitting());
    assert!(config.rpc.safety.submission_mode.is_non_submitting());
    assert!(readiness.ready);
    assert!(readiness.has_finding(CoordinatorReadinessFinding::Ready));
}

#[test]
fn coordinator_readiness_rejects_capped_submit_outside_testnet_scope() {
    let unsafe_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );
    let config = CoordinatorConfig::new_with_safety(2, 100, 4, unsafe_safety);
    let readiness = review_coordinator_readiness(config);

    assert!(!readiness.ready);
    assert!(readiness.has_finding(CoordinatorReadinessFinding::UnsafeScope));
    assert!(readiness.has_finding(CoordinatorReadinessFinding::RpcProofNotReady));
}
