//! RO:WHAT — Tests BUILD_PLAN3 Phase 5 test-only mint initialization intent review.
//! RO:WHY — Proves private pilot initialization is testnet-only, capped, authority-separated, and non-live.
//! RO:INTERACTS — TestOnlyMintInitializationIntent, TestOnlyAssetHarness, AuthorityMap.
//! RO:INVARIANTS — public labels, missing authorities, shared critical keys, zero supply, and cap overflows fail closed.
//! RO:SECURITY — no wallet, RPC, deploy, live mint initialization, mint, burn, settlement, or internal ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-core --test test_only_mint_initialization.

use rox_anchor_core::{
    AnchorSafetyProfile, AuthorityAssignment, AuthorityKeyId, AuthorityMap,
    AuthoritySeparationMode, OperatorRole, TestOnlyAssetHarness, TestOnlyMintFixture,
    TestOnlyMintInitializationFinding, TestOnlyMintInitializationIntent,
    TestOnlyTokenAccountFixture,
};

fn key(label: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(label).expect("authority key id should validate")
}

fn separated_authorities() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000001")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000002")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000003")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000004"),
            ),
        ],
    )
}

#[test]
fn test_only_mint_initialization_accepts_explicit_testnet_fixture() {
    let intent =
        TestOnlyMintInitializationIntent::devnet_fixture_with_authorities(separated_authorities());
    let review = intent.review();

    assert!(review.ready);
    assert_eq!(review.requested_initial_supply_units, 100);
    assert_eq!(review.max_initial_supply_units, 10_000);
    assert!(review.has_finding(TestOnlyMintInitializationFinding::Ready));

    let report = intent.redacted_report_lines().join("\n");

    assert!(report.contains("test_only_mint_initialization_surface: redacted_intent"));
    assert!(report.contains("ready: true"));
    assert!(report.contains("initialization_label: test-only-rox-mint-initialization"));
    assert!(report.contains("safety_environment_mode: testnet_only"));
    assert!(report.contains("safety_submission_mode: simulate_only"));
    assert!(report.contains("live_mint_initialization: disabled"));
    assert!(report.contains("wallet_loading: disabled"));
    assert!(report.contains("rpc_calls: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
    assert!(report.contains("mint…0002"));
    assert!(report.contains("halt…0003"));
    assert!(report.contains("reco…0004"));

    assert!(!report.contains("mint-key-00000002"));
    assert!(!report.contains("halt-key-00000003"));
    assert!(!report.contains("recovery-key-00000004"));
    assert!(!report.contains("mint complete"));
    assert!(!report.contains("settlement complete"));
}

#[test]
fn test_only_mint_initialization_rejects_public_or_production_labels() {
    let intent = TestOnlyMintInitializationIntent::new(
        "public-rox-mainnet-initialization",
        100,
        TestOnlyAssetHarness::devnet_simulation_fixture(),
        separated_authorities(),
    );

    let review = intent.review();

    assert!(!review.ready);
    assert!(review.has_finding(
        TestOnlyMintInitializationFinding::PublicOrProductionInitializationLabelRejected
    ));
}

#[test]
fn test_only_mint_initialization_rejects_zero_and_over_cap_supply() {
    let zero = TestOnlyMintInitializationIntent::new(
        "test-only-zero-supply",
        0,
        TestOnlyAssetHarness::devnet_simulation_fixture(),
        separated_authorities(),
    );

    let zero_review = zero.review();

    assert!(!zero_review.ready);
    assert!(zero_review.has_finding(TestOnlyMintInitializationFinding::ZeroInitialSupply));
    assert!(zero_review.has_finding(TestOnlyMintInitializationFinding::TestOnlyAssetHarnessBlocked));

    let over_cap = TestOnlyMintInitializationIntent::new(
        "test-only-over-cap-supply",
        10_001,
        TestOnlyAssetHarness::devnet_simulation_fixture(),
        separated_authorities(),
    );

    let over_cap_review = over_cap.review();

    assert!(!over_cap_review.ready);
    assert!(over_cap_review.has_finding(TestOnlyMintInitializationFinding::SupplyCapExceeded));
    assert!(
        over_cap_review.has_finding(TestOnlyMintInitializationFinding::TestOnlyAssetHarnessBlocked)
    );
}

#[test]
fn test_only_mint_initialization_rejects_missing_required_authorities() {
    let authorities = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000001")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000002")),
        ],
    );

    let intent = TestOnlyMintInitializationIntent::devnet_fixture_with_authorities(authorities);
    let review = intent.review();

    assert!(!review.ready);
    assert!(review.has_finding(TestOnlyMintInitializationFinding::MissingHaltAuthority));
    assert!(review.has_finding(TestOnlyMintInitializationFinding::MissingRecoveryAuthority));
    assert!(review.has_finding(TestOnlyMintInitializationFinding::UnsafeAuthoritySeparation));
}

#[test]
fn test_only_mint_initialization_rejects_shared_critical_authorities_in_strict_mode() {
    let shared = key("shared-critical-key-00000001");
    let authorities = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::MintAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::RecoveryAuthority, shared),
        ],
    );

    let intent = TestOnlyMintInitializationIntent::devnet_fixture_with_authorities(authorities);
    let review = intent.review();

    assert!(!review.ready);
    assert!(review.has_finding(TestOnlyMintInitializationFinding::UnsafeAuthoritySeparation));
}

#[test]
fn test_only_mint_initialization_rejects_asset_harness_that_is_not_testnet_only() {
    let mint = TestOnlyMintFixture::devnet_fixture();
    let token_account = TestOnlyTokenAccountFixture::devnet_fixture_for_mint(mint.mint.clone());
    let harness =
        TestOnlyAssetHarness::new(AnchorSafetyProfile::local_dry_run(), mint, token_account);

    let intent = TestOnlyMintInitializationIntent::new(
        "test-only-local-mode-rejected",
        100,
        harness,
        separated_authorities(),
    );

    let review = intent.review();

    assert!(!review.ready);
    assert!(review.has_finding(TestOnlyMintInitializationFinding::TestOnlyAssetHarnessBlocked));
}
