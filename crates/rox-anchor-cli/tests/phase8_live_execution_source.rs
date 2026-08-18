//! BUILD_PLAN4 Phase 8 live one-shot execution source boundary.
//!
//! Source inspection compiles the executor but never invokes RPC, loads a
//! keypair, signs, submits, burns, or releases ROC.

#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase8_live_execution.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

const MODULES: &str = include_str!("../src/commands/mod.rs");

#[test]
fn phase8_live_route_is_explicit_and_not_generic_wrapped() {
    assert!(MODULES.contains("pub mod phase8_live_execution;"));

    assert!(PILOT.contains("\"phase8-execute-capped-rox-to-roc-burn\""));

    assert!(PILOT.contains("run_phase8_live_execution"));

    assert!(SOURCE.contains("I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN"));
}

#[test]
fn phase8_live_executor_has_exactly_one_submission_call() {
    assert_eq!(
        SOURCE.matches(".send_and_confirm_transaction(").count(),
        1,
        "Phase 8 live executor must contain exactly one send call",
    );
}

#[test]
fn phase8_live_order_is_preflight_sign_resimulate_recheck_send() {
    let prekey = SOURCE.find("PHASE8_LIVE_PREKEY_PREFLIGHT").unwrap();

    let signer = SOURCE[prekey..]
        .find("read_keypair_file(")
        .map(|offset| prekey + offset)
        .unwrap();

    let signed_sim = SOURCE[signer..]
        .find("PHASE8_SIGNED_RESIMULATION")
        .map(|offset| signer + offset)
        .unwrap();

    let recheck = SOURCE[signed_sim..]
        .find("PHASE8_IMMEDIATE_PRESEND_PREFLIGHT")
        .map(|offset| signed_sim + offset)
        .unwrap();

    let send = SOURCE[recheck..]
        .find("PHASE8_SINGLE_SUBMISSION")
        .map(|offset| recheck + offset)
        .unwrap();

    assert!(prekey < signer);
    assert!(signer < signed_sim);
    assert!(signed_sim < recheck);
    assert!(recheck < send);
}

#[test]
fn phase8_live_persists_send_receipt_before_readback() {
    let send = SOURCE.find(".send_and_confirm_transaction(").unwrap();

    let receipt_marker = SOURCE.find("PHASE8_SEND_RECEIPT_BEFORE_READBACK").unwrap();

    let receipt_write = SOURCE[receipt_marker..]
        .find("write_new_json(")
        .map(|offset| receipt_marker + offset)
        .unwrap();

    let post_readback = SOURCE[receipt_write..]
        .find("read_live_state(")
        .map(|offset| receipt_write + offset)
        .unwrap();

    assert!(send < receipt_marker);
    assert!(receipt_marker < receipt_write);
    assert!(receipt_write < post_readback);
}

#[test]
fn phase8_live_reuses_exact_phase8a_candidate_and_canonical_bindings() {
    for required in [
        "build_exact_instructions",
        "PHASE8_OPERATION_ID",
        "PHASE8_IDEMPOTENCY_KEY",
        "PHASE8_NONCE",
        "PHASE8_BURN_EVIDENCE_LABEL",
        "PHASE6_PROGRAM_ID",
        "PHASE6_CONFIG_ACCOUNT",
        "PHASE6_ROX_MINT",
        "PHASE6_TOKEN_ACCOUNT",
        "PHASE6_WORKFLOW_AUTHORITY",
        "PHASE6_MINT_AUTHORITY",
        "AnchorTransferDirection::RoxToRoc",
        "OperationStateCode::Finalized",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing canonical Phase 8 binding `{required}`",
        );
    }
}

#[test]
fn phase8_live_requires_fresh_evidence_and_exact_one_to_zero_state() {
    for required in [
        "PHASE8_AUTHORIZATION_AGE_LIMIT_SLOTS",
        "simulation_context_slot",
        "read_live_state(",
        "expected_supply",
        "expected_token_amount",
        "fresh Phase 8 operation PDA already exists",
        "Phase 8 operation is not finalized",
        "two_source_post_send_closeout",
        "get_multiple_accounts_with_context_compat",
        "Solana and Uniblock disagree",
        "\"mint_supply_minor\":",
        "\"workflow_token_amount_minor\":",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing Phase 8 freshness/readback marker `{required}`",
        );
    }
}

#[test]
fn phase8_live_has_hard_no_rerun_and_config_receipt_binding() {
    for required in [
        "SEND_RECEIPT_EXISTS_DO_NOT_RETRY",
        "perform readback/reconciliation only",
        "receipt_output_path",
        "CLI send receipt path does not match the externally reviewed config receipt path",
        "SubmissionMode::TestnetSubmitCapped",
        "AnchorEnvironmentMode::TestnetOnly",
        "AnchorCluster::Devnet",
        "https://api.devnet.solana.com",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing Phase 8 execution boundary `{required}`",
        );
    }
}

#[test]
fn phase8_live_contains_no_real_internal_roc_release_api() {
    for forbidden in [
        "svc_wallet::",
        "svc_wallet.",
        "ron_ledger::",
        "ron_ledger.",
        "release_real_roc(",
        "credit_real_roc(",
        "production_settle(",
        "production_settlement::",
        "mainnet-submit",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 8 executor must not contain authority API `{forbidden}`",
        );
    }

    for required_safety_marker in [
        "\"real_roc_release\":",
        "\"real_roc_mutation\":",
        "\"production_settlement\":",
        "\"mainnet_authorized\":",
        "\"svc_wallet_call\":",
        "\"ron_ledger_mutation\":",
    ] {
        assert!(
            SOURCE.contains(required_safety_marker),
            "missing explicit negative safety marker `{required_safety_marker}`",
        );
    }
}
