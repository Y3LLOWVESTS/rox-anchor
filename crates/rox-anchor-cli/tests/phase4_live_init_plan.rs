#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use anchor_lang::solana_program::{program_option::COption, pubkey::Pubkey};
use rox_anchor::RoxAnchorConfig;
use rox_anchor_cli::commands::phase4_live_init::{
    build_phase4_live_init_plan, Phase4LiveInitPlanError, Phase4LiveInitRequest,
    PHASE4_PRIVATE_MAX_AMOUNT_UNITS_PER_OPERATION, PHASE4_PRIVATE_MAX_SUPPLY_UNITS,
    PHASE4_TEST_ONLY_MINT_DECIMALS,
};
use solana_sdk_ids::system_program;
use spl_token::instruction::TokenInstruction;

fn request() -> Phase4LiveInitRequest {
    Phase4LiveInitRequest {
        program_id: rox_anchor::ID,
        payer: Pubkey::new_unique(),
        config: Pubkey::new_unique(),
        test_only_mint: Pubkey::new_unique(),
        halt_authority: Pubkey::new_unique(),
        recovery_authority: Pubkey::new_unique(),
        upgrade_authority: Pubkey::new_unique(),
        mint_rent_lamports: 1_461_600,
        max_supply_units: PHASE4_PRIVATE_MAX_SUPPLY_UNITS,
        max_amount_units_per_operation: PHASE4_PRIVATE_MAX_AMOUNT_UNITS_PER_OPERATION,
    }
}

#[test]
fn phase4_plan_is_exact_four_instruction_atomic_shape() {
    let plan = build_phase4_live_init_plan(request()).expect("valid Phase 4 request");

    assert_eq!(plan.instruction_count(), 4);
    assert!(plan.is_atomic_phase4_shape());

    assert_eq!(plan.instructions[0].program_id, system_program::id());

    assert_eq!(plan.instructions[1].program_id, spl_token::id());

    assert_eq!(
        plan.instructions[2].program_id,
        spl_associated_token_account::id()
    );

    assert_eq!(plan.instructions[3].program_id, rox_anchor::ID);
}

#[test]
fn phase4_plan_binds_zero_decimal_mint_directly_to_program_pda() {
    let request = request();

    let plan = build_phase4_live_init_plan(request).expect("valid Phase 4 request");

    let expected = RoxAnchorConfig::derived_initialize_args(
        &request.program_id,
        &request.config,
        request.test_only_mint,
    )
    .expect("derived Anchor initialization args");

    assert_eq!(plan.mint_authority, expected.mint_authority);

    assert_eq!(plan.mint_authority_bump, expected.mint_authority_bump);

    assert_eq!(plan.mint_decimals, PHASE4_TEST_ONLY_MINT_DECIMALS);

    let token_instruction = TokenInstruction::unpack(&plan.instructions[1].data)
        .expect("InitializeMint2 should decode");

    match token_instruction {
        TokenInstruction::InitializeMint2 {
            decimals,
            mint_authority,
            freeze_authority,
        } => {
            assert_eq!(decimals, 0);
            assert_eq!(mint_authority, plan.mint_authority);
            assert_eq!(freeze_authority, COption::None);
        }
        other => {
            panic!("expected InitializeMint2, got {other:?}");
        }
    }
}

#[test]
fn phase4_plan_derives_payer_associated_token_account() {
    let request = request();

    let plan = build_phase4_live_init_plan(request).expect("valid Phase 4 request");

    assert_eq!(
        plan.test_only_token_account,
        spl_associated_token_account::get_associated_token_address(
            &request.payer,
            &request.test_only_mint,
        )
    );
}

#[test]
fn phase4_plan_requires_only_execution_signers_not_pda_or_upgrade_authority() {
    let request = request();

    let plan = build_phase4_live_init_plan(request).expect("valid Phase 4 request");

    let expected: BTreeSet<Pubkey> = [
        request.payer,
        request.config,
        request.test_only_mint,
        request.halt_authority,
        request.recovery_authority,
    ]
    .into_iter()
    .collect();

    assert_eq!(plan.required_signers, expected);

    assert!(!plan.required_signers.contains(&plan.mint_authority));

    assert!(!plan.required_signers.contains(&request.upgrade_authority));
}

#[test]
fn phase4_plan_rejects_shared_critical_authority() {
    let mut request = request();

    request.recovery_authority = request.halt_authority;

    let error =
        build_phase4_live_init_plan(request).expect_err("shared halt/recovery authority must fail");

    assert_eq!(error, Phase4LiveInitPlanError::SharedCriticalIdentity);
}

#[test]
fn phase4_plan_rejects_policy_drift_and_missing_rent() {
    let mut wrong_supply = request();

    wrong_supply.max_supply_units = 999;

    assert_eq!(
        build_phase4_live_init_plan(wrong_supply,).expect_err("wrong supply policy must fail",),
        Phase4LiveInitPlanError::InvalidPilotCaps
    );

    let mut wrong_amount = request();

    wrong_amount.max_amount_units_per_operation = 11;

    assert_eq!(
        build_phase4_live_init_plan(wrong_amount,).expect_err("wrong operation policy must fail",),
        Phase4LiveInitPlanError::InvalidPilotCaps
    );

    let mut no_rent = request();

    no_rent.mint_rent_lamports = 0;

    assert_eq!(
        build_phase4_live_init_plan(no_rent).expect_err("missing RPC rent preflight must fail",),
        Phase4LiveInitPlanError::ZeroMintRent
    );
}

#[test]
fn phase4_plan_summary_cannot_claim_live_execution() {
    let plan = build_phase4_live_init_plan(request()).expect("valid Phase 4 request");

    let summary = plan.redacted_summary();

    assert!(summary.contains("transaction_atomicity: single_transaction_plan"));

    assert!(summary.contains("mint_authority_model: program_derived_pda"));

    assert!(summary.contains("freeze_authority: none"));

    assert!(summary.contains("transaction_submission: disabled"));

    assert!(summary.contains("real_roc_mutation: disabled"));

    assert!(summary.contains("mainnet_authorized: false"));
}
