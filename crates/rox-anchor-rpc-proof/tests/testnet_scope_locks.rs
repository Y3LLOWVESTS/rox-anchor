//! RO:WHAT — Tests BUILD_PLAN2 Phase 1 RPC proof safety configuration.
//! RO:WHY — Proves RPC proof config stays local/testnet scoped and non-submitting by default.
//! RO:INTERACTS — RpcProofConfig, RPC proof readiness, and rox-anchor-core safety types.
//! RO:INVARIANTS — unsafe capped-submit scope is rejected before any live RPC adapter exists.
//! RO:SECURITY — readiness only; no network, keys, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-rpc-proof --test testnet_scope_locks.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_rpc_proof::{review_rpc_proof_readiness, RpcProofConfig, RpcProofReadinessFinding};

#[test]
fn rpc_proof_defaults_to_non_submitting_scope() {
    let config = RpcProofConfig::new(2, 100);
    let readiness = review_rpc_proof_readiness(config);

    assert!(config.safety.submission_mode.is_non_submitting());
    assert!(readiness.ready);
    assert!(readiness.has_finding(RpcProofReadinessFinding::Ready));
}

#[test]
fn rpc_proof_readiness_rejects_capped_submit_outside_testnet_scope() {
    let unsafe_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );
    let config = RpcProofConfig::new_with_safety(2, 100, unsafe_safety);
    let readiness = review_rpc_proof_readiness(config);

    assert!(!readiness.ready);
    assert!(readiness.has_finding(RpcProofReadinessFinding::UnsafeScope));
}
