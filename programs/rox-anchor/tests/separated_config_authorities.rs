//! Focused tests for separated live config authorities.
//!
//! These tests are local only. They prove the Anchor state model can represent
//! distinct workflow, halt, recovery, and program-derived mint authorities
//! before any devnet config initialization is attempted.

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
        max_supply_units: u64::MAX,
        max_amount_units_per_operation: u64::MAX,
        mint_authority_bump: 0,
        halted: false,
        recovery_required: false,
    }
}

fn derived_args(config_key: &Pubkey, rox_mint: Pubkey) -> InitializeConfigArgs {
    RoxAnchorConfig::derived_initialize_args(&rox_anchor::ID, config_key, rox_mint)
        .expect("derived test mint authority should be valid")
}

#[test]
fn strict_initialize_stores_separated_authorities() {
    let authority = Pubkey::new_unique();
    let halt_authority = Pubkey::new_unique();
    let recovery_authority = Pubkey::new_unique();
    let config_key = Pubkey::new_unique();
    let rox_mint = Pubkey::new_unique();

    let args = derived_args(&config_key, rox_mint);

    let mut config = blank_config();

    config
        .initialize_with_separated_authorities(authority, halt_authority, recovery_authority, args)
        .expect("separated authorities should initialize");

    assert_eq!(config.authority, authority,);
    assert_eq!(config.halt_authority, halt_authority,);
    assert_eq!(config.recovery_authority, recovery_authority,);
    assert_eq!(config.rox_mint, rox_mint,);
    assert_eq!(config.mint_authority, args.mint_authority,);
    assert_ne!(config.mint_authority, authority,);
    assert_ne!(config.mint_authority, halt_authority,);
    assert_ne!(config.mint_authority, recovery_authority,);
}

#[test]
fn strict_initialize_rejects_duplicate_critical_roles() {
    let authority = Pubkey::new_unique();
    let halt_authority = Pubkey::new_unique();
    let recovery_authority = Pubkey::new_unique();
    let config_key = Pubkey::new_unique();
    let args = derived_args(&config_key, Pubkey::new_unique());

    let mut config = blank_config();

    assert!(config
        .initialize_with_separated_authorities(authority, authority, recovery_authority, args,)
        .is_err());

    assert!(config
        .initialize_with_separated_authorities(authority, halt_authority, authority, args,)
        .is_err());

    assert!(config
        .initialize_with_separated_authorities(authority, halt_authority, halt_authority, args,)
        .is_err());
}

#[test]
fn halt_and_recovery_roles_cannot_impersonate_each_other() {
    let authority = Pubkey::new_unique();
    let halt_authority = Pubkey::new_unique();
    let recovery_authority = Pubkey::new_unique();
    let config_key = Pubkey::new_unique();
    let args = derived_args(&config_key, Pubkey::new_unique());

    let mut config = blank_config();

    config
        .initialize_with_separated_authorities(authority, halt_authority, recovery_authority, args)
        .expect("strict initialization should succeed");

    config
        .halt(halt_authority)
        .expect("halt authority should halt");

    assert!(config.halted);

    assert!(config.recover(halt_authority).is_err());

    config
        .recover(recovery_authority)
        .expect("recovery authority should recover");

    assert!(!config.halted);

    assert!(config.halt(recovery_authority).is_err());
}

#[test]
fn workflow_authority_cannot_halt_or_recover_strict_config() {
    let authority = Pubkey::new_unique();
    let halt_authority = Pubkey::new_unique();
    let recovery_authority = Pubkey::new_unique();
    let config_key = Pubkey::new_unique();
    let args = derived_args(&config_key, Pubkey::new_unique());

    let mut config = blank_config();

    config
        .initialize_with_separated_authorities(authority, halt_authority, recovery_authority, args)
        .expect("strict initialization should succeed");

    assert!(config.halt(authority).is_err());

    assert!(config.recover(authority).is_err());

    assert!(config.require_authority(authority).is_ok());
}

#[test]
fn config_space_includes_all_separated_roles() {
    assert_eq!(RoxAnchorConfig::SPACE, 8 + (32 * 5) + 1 + 8 + 8 + 1 + 1 + 1);
}

#[test]
fn legacy_fixture_initializer_does_not_define_live_policy() {
    let authority = Pubkey::new_unique();
    let config_key = Pubkey::new_unique();
    let args = derived_args(&config_key, Pubkey::new_unique());

    let mut config = blank_config();

    config
        .initialize(authority, args)
        .expect("legacy local fixture initialization should remain valid");

    assert_eq!(config.authority, authority,);
    assert_eq!(config.halt_authority, authority,);
    assert_eq!(config.recovery_authority, authority,);
}
