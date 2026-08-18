//! Focused BUILD_PLAN4 private-pilot on-chain cap behavior.

#![forbid(unsafe_code)]

use anchor_lang::prelude::Pubkey;
use rox_anchor::{InitializeConfigArgs, RoxAnchorConfig};

fn blank_config() -> RoxAnchorConfig {
    RoxAnchorConfig {
        authority: Pubkey::default(),
        halt_authority: Pubkey::default(),
        recovery_authority: Pubkey::default(),
        rox_mint: Pubkey::default(),
        mint_authority: Pubkey::default(),
        test_only_mode: false,
        max_supply_units: 0,
        max_amount_units_per_operation: 0,
        mint_authority_bump: 0,
        halted: false,
        recovery_required: false,
    }
}

fn strict_config() -> RoxAnchorConfig {
    let program_id = Pubkey::new_unique();
    let config_key = Pubkey::new_unique();
    let mint = Pubkey::new_unique();

    let workflow = Pubkey::new_unique();
    let halt = Pubkey::new_unique();
    let recovery = Pubkey::new_unique();

    let args = RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, mint)
        .expect("derived mint authority binding should succeed");

    let mut config = blank_config();

    config
        .initialize_with_separated_authorities(workflow, halt, recovery, args)
        .expect("strict private-pilot config should initialize");

    config
}

#[test]
fn strict_initializer_records_private_test_only_caps() {
    let config = strict_config();

    assert!(config.test_only_mode);

    assert_eq!(
        config.max_supply_units,
        RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS
    );

    assert_eq!(
        config.max_amount_units_per_operation,
        RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS
    );

    assert!(config.private_test_only_policy_is_valid());
}

#[test]
fn strict_policy_is_exactly_1000_supply_and_10_per_operation() {
    assert_eq!(RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS, 1_000);

    assert_eq!(RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS, 10);

    let config = strict_config();

    assert_eq!(config.max_supply_units, 1_000);

    assert_eq!(config.max_amount_units_per_operation, 10);
}

#[test]
fn per_operation_cap_accepts_boundary_and_rejects_overage() {
    let config = strict_config();

    assert!(config.require_test_only_amount_cap(1).is_ok());

    assert!(config.require_test_only_amount_cap(10).is_ok());

    assert!(config.require_test_only_amount_cap(0).is_err());

    assert!(config.require_test_only_amount_cap(11).is_err());
}

#[test]
fn cumulative_supply_cap_accepts_exact_ceiling() {
    let config = strict_config();

    assert!(config.require_test_only_mint_supply_cap(990, 10,).is_ok());

    assert!(config.require_test_only_mint_supply_cap(991, 10,).is_err());

    assert!(config.require_test_only_mint_supply_cap(1_000, 1,).is_err());
}

#[test]
fn cumulative_supply_cap_rejects_integer_overflow() {
    let config = strict_config();

    assert!(config
        .require_test_only_mint_supply_cap(u64::MAX, 1,)
        .is_err());
}

#[test]
fn malformed_private_policy_fails_closed() {
    let mut config = strict_config();

    config.max_amount_units_per_operation = 11;

    assert!(config.require_private_test_only_policy().is_err());

    assert!(!config.test_only_amount_allowed(1));

    assert!(config.require_test_only_amount_cap(1).is_err());
}

#[test]
fn legacy_fixture_initializer_preserves_non_test_only_behavior() {
    let authority = Pubkey::new_unique();

    let args = InitializeConfigArgs {
        rox_mint: Pubkey::new_unique(),
        mint_authority: Pubkey::new_unique(),
        mint_authority_bump: 1,
    };

    let mut config = blank_config();

    config
        .initialize(authority, args)
        .expect("legacy fixture initialization should remain usable");

    assert!(!config.test_only_mode);

    assert_eq!(config.max_supply_units, u64::MAX);

    assert_eq!(config.max_amount_units_per_operation, u64::MAX);

    assert!(config.test_only_amount_allowed(u64::MAX));
}

#[test]
fn config_space_includes_private_policy_fields() {
    assert_eq!(RoxAnchorConfig::SPACE, 8 + (32 * 5) + 1 + 8 + 8 + 1 + 1 + 1);
}
