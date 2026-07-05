//! RO:WHAT — Tests BUILD_PLAN2 Phase 6 test-only mint/token-account harness.
//! RO:WHY — Proves test assets stay explicitly testnet-only, capped, and binding-checked.
//! RO:INTERACTS — AnchorSafetyProfile, MintId, TokenAccountId, and TestOnlyAssetHarness.
//! RO:INVARIANTS — production/public labels, over-cap amounts, and token-account mint mismatches fail closed.
//! RO:SECURITY — no live mint, burn, token account creation, RPC, wallet, keypair, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-core --test test_only_asset_harness.

use rox_anchor_core::{
    AnchorCluster, AnchorSafetyProfile, MintId, TestOnlyAssetHarness, TestOnlyAssetHarnessFinding,
    TestOnlyMintFixture, TestOnlyTokenAccountFixture, TokenAccountId,
};

#[test]
fn test_only_asset_harness_accepts_explicit_testnet_simulation_fixture() {
    let harness = TestOnlyAssetHarness::devnet_simulation_fixture();
    let review = harness.review_amount(25);

    assert!(review.ready);
    assert_eq!(review.requested_amount_units, 25);
    assert_eq!(review.max_amount_units, 10_000);
    assert!(review.has_finding(TestOnlyAssetHarnessFinding::Ready));
}

#[test]
fn test_only_asset_harness_requires_explicit_testnet_mode() {
    let mint = TestOnlyMintFixture::devnet_fixture();
    let token_account = TestOnlyTokenAccountFixture::devnet_fixture_for_mint(mint.mint.clone());
    let harness =
        TestOnlyAssetHarness::new(AnchorSafetyProfile::local_dry_run(), mint, token_account);

    let review = harness.review_amount(25);

    assert!(!review.ready);
    assert!(review.has_finding(TestOnlyAssetHarnessFinding::ExplicitTestnetModeRequired));
}

#[test]
fn test_only_asset_harness_rejects_public_or_production_mint_labels() {
    let mint = TestOnlyMintFixture::new(
        "public-production-rox-mint",
        MintId::new("RoxTestMint1111111111111111111111111111").unwrap(),
        10_000,
    );
    let token_account = TestOnlyTokenAccountFixture::devnet_fixture_for_mint(mint.mint.clone());
    let harness = TestOnlyAssetHarness::new(
        AnchorSafetyProfile::testnet_simulation(AnchorCluster::Devnet),
        mint,
        token_account,
    );

    let review = harness.review_amount(25);

    assert!(!review.ready);
    assert!(review.has_finding(TestOnlyAssetHarnessFinding::PublicOrProductionMintLabelRejected));
}

#[test]
fn test_only_asset_harness_rejects_token_account_mint_mismatch() {
    let mint = TestOnlyMintFixture::devnet_fixture();
    let token_account = TestOnlyTokenAccountFixture::new(
        "test-only-mismatched-token-account",
        MintId::new("DifferentRoxTestMint11111111111111111111").unwrap(),
        TokenAccountId::new("RoxTestTokenAccount111111111111111111").unwrap(),
    );
    let harness = TestOnlyAssetHarness::new(
        AnchorSafetyProfile::testnet_simulation(AnchorCluster::Devnet),
        mint,
        token_account,
    );

    let review = harness.review_amount(25);

    assert!(!review.ready);
    assert!(review.has_finding(TestOnlyAssetHarnessFinding::TokenAccountMintMismatch));
}

#[test]
fn test_only_asset_harness_rejects_zero_and_over_cap_amounts() {
    let mint = TestOnlyMintFixture::new(
        "test-only-low-cap-rox-mint",
        MintId::new("RoxTestMint1111111111111111111111111111").unwrap(),
        25,
    );
    let token_account = TestOnlyTokenAccountFixture::devnet_fixture_for_mint(mint.mint.clone());
    let harness = TestOnlyAssetHarness::new(
        AnchorSafetyProfile::testnet_simulation(AnchorCluster::Devnet),
        mint,
        token_account,
    );

    let zero = harness.review_amount(0);
    assert!(!zero.ready);
    assert!(zero.has_finding(TestOnlyAssetHarnessFinding::ZeroAmount));

    let over_cap = harness.review_amount(26);
    assert!(!over_cap.ready);
    assert!(over_cap.has_finding(TestOnlyAssetHarnessFinding::AmountCapExceeded));
}
