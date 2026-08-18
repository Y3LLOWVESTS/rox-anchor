//! BUILD_PLAN4 Phase 7C signed-executor source boundary.
//!
//! These tests inspect and compile the executor without invoking it.
//! No keypair is loaded, no RPC request is made, and no transaction is signed
//! or submitted by this test target.

#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase7_live_signed_executor.rs");

const MODULES: &str = include_str!("../src/commands/mod.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

#[test]
fn phase7c_module_is_compiled_but_not_directly_cli_routed() {
    assert!(MODULES.contains("pub mod phase7_live_signed_executor;"));

    for forbidden in [
        "phase7_live_signed_executor",
        "phase7-live-signed-executor",
        "prepare_phase7_signed_transaction",
        "simulate_prepared_phase7_transaction",
    ] {
        assert!(
            !PILOT.contains(forbidden),
            "Phase 7C must not be directly CLI-routed through `{forbidden}`"
        );
    }

    assert!(
        PILOT.contains("\"phase7-execute-capped-roc-to-rox\" | \"execute-actual-roc-to-rox-send\""),
        "the guarded Phase 7E live route should now exist"
    );

    assert!(
        PILOT.contains("phase7_live_manual_execution::run_phase7_live_manual_execution"),
        "the live route must terminate at the Phase 7E orchestrator"
    );
}

#[test]
fn phase7c_reuses_exact_phase7a_candidate_and_phase7b_authorization() {
    for marker in [
        "build_phase7_capped_roc_to_rox_plan",
        "is_exact_phase7_shape",
        "rox-anchor.phase7-simulation-authorization.v1",
        "BUILD_PLAN4 Phase 7B",
        "simulation_and_sender_authorization_evidence",
        "PHASE7_OPERATION_ID",
        "PHASE7_IDEMPOTENCY_KEY",
        "PHASE7_NONCE",
        "sender_authorized_by_existing_model",
        "live_submission_permitted_by_existing_model",
        "phase7_live_devnet_simulation",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7C missing prerequisite marker `{marker}`"
        );
    }
}

#[test]
fn phase7c_has_real_external_key_loading_and_exact_one_signer_transaction() {
    for marker in [
        "read_keypair_file",
        "payer_keypair_path",
        "workflow.pubkey()",
        "plan.workflow_authority",
        "let signers: [&dyn Signer; 1]",
        "Transaction::new_signed_with_payer",
        "transaction.signatures.len() != 1",
        "Signature::default()",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7C missing signed-transaction marker `{marker}`"
        );
    }
}

#[test]
fn phase7c_revalidates_actual_devnet_state_before_signing() {
    for marker in [
        "get_multiple_accounts",
        "program.executable",
        "RoxAnchorConfig::try_deserialize",
        "config.authority",
        "config.rox_mint",
        "config.mint_authority",
        "config.test_only_mode",
        "PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS",
        "PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS",
        "config.halted",
        "config.recovery_required",
        "Mint::unpack",
        "mint.supply != 0",
        "SplTokenAccount::unpack",
        "token.amount != 0",
        "Phase 7 operation PDA already exists",
        "get_minimum_balance_for_rent_exemption",
        "get_balance",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7C missing live-preflight marker `{marker}`"
        );
    }
}

#[test]
fn phase7c_signed_candidate_must_simulate_before_any_future_send() {
    for marker in [
        "simulate_prepared_phase7_transaction",
        "simulate_transaction",
        "signed Phase 7 simulation rejected",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7C missing signed simulation marker `{marker}`"
        );
    }
}

#[test]
fn phase7c_contains_no_transaction_submission_api() {
    for forbidden in [
        "send_and_confirm_transaction",
        "send_transaction(",
        "send_transaction_with_config",
        "send_raw_transaction",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 7C must not contain transaction-send API `{forbidden}`"
        );
    }
}

#[test]
fn phase7c_keeps_exact_caps_and_test_only_boundary() {
    for marker in [
        "PHASE7_AMOUNT_MINOR",
        "PHASE7_MAX_AMOUNT_MINOR",
        "PHASE7_MAX_OPERATIONS",
        "PHASE7_RETRY_CAP",
        "AnchorEnvironmentMode::TestnetOnly",
        "AnchorCluster::Devnet",
        "SubmissionMode::TestnetSubmitCapped",
        "test-only-rox-private-devnet",
        "https://api.devnet.solana.com",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7C missing safety marker `{marker}`"
        );
    }
}

#[test]
fn phase7c_uses_zero_runtime_compile_anchors_instead_of_dead_code_suppression() {
    assert!(SOURCE.contains("prepare_phase7_signed_transaction;"));

    assert!(SOURCE.contains("simulate_prepared_phase7_transaction;"));

    assert!(SOURCE.contains("const _: fn("));

    assert!(
        !SOURCE.contains("allow(dead_code)"),
        "Phase 7C should stay in the compile graph without globally suppressing dead-code linting"
    );

    assert!(
        !SOURCE.contains("expect(dead_code)"),
        "Phase 7C should use real compile references rather than lint suppression"
    );
}

#[test]
fn phase7c_prepared_fields_are_compile_anchored_for_phase7d() {
    assert!(SOURCE.contains("phase7c_compile_prepared_field_contract"));

    for marker in [
        "&prepared.rpc",
        "&prepared.transaction",
        "&prepared.plan",
        "&prepared.workflow_authority",
        "&prepared.pre_mint_supply",
        "&prepared.pre_token_amount",
        "&prepared.operation_rent_lamports",
        "&prepared.payer_balance_lamports",
        "&prepared.signature_count",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7C prepared field contract missing `{marker}`"
        );
    }

    assert!(!SOURCE.contains("allow(dead_code)"));

    assert!(!SOURCE.contains("expect(dead_code)"));
}
