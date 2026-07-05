//! RO:WHAT — Tests BUILD_PLAN2 Phase 1 testnet scope locks in rox-anchor-core.
//! RO:WHY — Proves mainnet/public launch labels are not representable as safe local/testnet config.
//! RO:INTERACTS — AnchorEnvironmentMode, AnchorCluster, ClusterAllowlist, SubmissionMode, AnchorSafetyProfile.
//! RO:INVARIANTS — mainnet-beta rejected; public launch flags absent; default mode does not submit.
//! RO:SECURITY — validation-only; no wallet, RPC, key, transaction, mint, burn, or settlement behavior.
//! RO:TEST — run with cargo test -p rox-anchor-core --test testnet_scope_locks.

use rox_anchor_core::{
    AnchorCluster, AnchorCoreError, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist,
    SubmissionMode,
};

#[test]
fn mainnet_beta_cluster_is_rejected_before_config_can_use_it() {
    assert_eq!(
        AnchorCluster::from_label("mainnet-beta"),
        Err(AnchorCoreError::MainnetBetaClusterForbidden)
    );

    assert_eq!(
        AnchorCluster::from_label("solana-mainnet"),
        Err(AnchorCoreError::MainnetBetaClusterForbidden)
    );
}

#[test]
fn public_launch_flags_are_not_available_modes() {
    assert_eq!(
        AnchorEnvironmentMode::from_label("public-launch"),
        Err(AnchorCoreError::PublicLaunchFlagUnavailable {
            flag: "public-launch".to_owned(),
        })
    );

    assert_eq!(
        SubmissionMode::from_label("public-submit"),
        Err(AnchorCoreError::PublicLaunchFlagUnavailable {
            flag: "public-submit".to_owned(),
        })
    );
}

#[test]
fn default_safety_profile_is_non_submitting() {
    let profile = AnchorSafetyProfile::default();

    assert_eq!(
        profile.environment_mode,
        AnchorEnvironmentMode::ProductionDisabled
    );
    assert_eq!(profile.cluster, AnchorCluster::Localnet);
    assert!(profile.submission_mode.is_non_submitting());
    assert!(profile.validate().is_ok());
}

#[test]
fn local_only_profile_rejects_non_local_clusters_even_when_allowlisted() {
    let profile = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::DryRunOnly,
    );

    assert!(matches!(
        profile.validate(),
        Err(AnchorCoreError::UnsafeModeCluster {
            environment: "local_only",
            cluster: "devnet",
        })
    ));
}

#[test]
fn capped_testnet_submission_requires_testnet_mode_and_testnet_cluster() {
    let local_submit = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );

    assert!(matches!(
        local_submit.validate(),
        Err(AnchorCoreError::UnsafeSubmissionMode {
            environment: "local_only",
            cluster: "localnet",
            submission: "testnet_submit_capped",
        })
    ));

    let devnet_submit = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    );

    assert!(devnet_submit.validate().is_ok());
}
