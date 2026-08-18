//! BUILD_PLAN4 Phase 7D one-shot submission/readback source boundary.
//!
//! This test compiles and inspects the live-capable module without invoking it.
//! No RPC, signer load, signing, transaction submission, mint, or ROC mutation
//! occurs in this test target.

#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase7_live_submission_readback.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

const MODULES: &str = include_str!("../src/commands/mod.rs");

#[test]
fn phase7d_is_compiled_but_not_directly_cli_reachable() {
    assert!(MODULES.contains("pub mod phase7_live_submission_readback;"));

    for forbidden in [
        "phase7_live_submission_readback",
        "phase7-one-shot-submission-readback",
        "submit_phase7_once_and_readback",
    ] {
        assert!(
            !PILOT.contains(forbidden),
            "Phase 7D must not be directly CLI-routed through `{forbidden}`"
        );
    }

    assert!(
        PILOT.contains("\"phase7-execute-capped-roc-to-rox\" | \"execute-actual-roc-to-rox-send\""),
        "the guarded Phase 7E live route should now exist"
    );

    assert!(
        PILOT.contains("phase7_live_manual_execution::run_phase7_live_manual_execution"),
        "the live route must terminate at Phase 7E rather than Phase 7D"
    );
}

#[test]
fn phase7d_has_exactly_one_submission_call() {
    assert_eq!(
        SOURCE.matches(".send_and_confirm_transaction(").count(),
        1,
        "Phase 7D must contain exactly one transaction submission call"
    );
}

#[test]
fn phase7d_resimulates_before_the_one_shot_send() {
    let simulate = SOURCE
        .find("simulate_prepared_phase7_transaction")
        .expect("signed simulation call must exist");

    let pre_send = SOURCE
        .find("verify_pre_send_state_unchanged")
        .expect("non-persistence recheck must exist");

    let send = SOURCE
        .find(".send_and_confirm_transaction(")
        .expect("one-shot send call must exist");

    assert!(simulate < pre_send);

    assert!(pre_send < send);
}

#[test]
fn phase7d_does_not_reload_or_resign_the_keypair() {
    for forbidden in [
        "read_keypair_file",
        "Keypair::",
        "new_signed_with_payer",
        "partial_sign",
        "try_sign",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 7D must consume the exact Phase 7C signed candidate, not create a second signer path: {forbidden}"
        );
    }
}

#[test]
fn phase7d_persists_send_evidence_before_mandatory_readback() {
    let send_call = SOURCE.find(".send_and_confirm_transaction(").unwrap();

    let send_receipt = SOURCE.find("build_send_receipt(").unwrap();

    let send_write = SOURCE[send_receipt..]
        .find("write_new_json(")
        .map(|offset| send_receipt + offset)
        .unwrap();

    let readback = SOURCE.find("verify_post_send_readback(").unwrap();

    assert!(send_call < send_receipt);

    assert!(send_receipt < send_write);

    assert!(send_write < readback);
}

#[test]
fn phase7d_post_send_readback_is_strict() {
    for marker in [
        "RoxAnchorConfig::try_deserialize",
        "RoxAnchorOperation::try_deserialize",
        "OperationStateCode::Finalized",
        "operation.is_roc_to_rox()",
        "operation.operation_id_hash",
        "operation.amount_atoms",
        "operation.burn_evidence_hash",
        "Mint::unpack",
        "SplTokenAccount::unpack",
        "mint_delta",
        "token_delta",
        "post-send test-only ROX delta mismatch",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing strict readback invariant `{marker}`"
        );
    }
}

#[test]
fn phase7d_receipts_match_existing_phase7_schema_boundaries() {
    for marker in [
        "rox-anchor.actual-roc-to-rox-capped-send.v1",
        "actual_roc_to_rox_capped_send_receipt",
        "rox-anchor.actual-roc-to-rox-readback.v1",
        "actual_roc_to_rox_readback_receipt",
        "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
        "shadow_roc_burn_only",
        "expected_test_only_rox_delta_minor",
        "observed_test_only_rox_delta_minor",
        "transaction_signature_sha256",
        "readback_required",
        "read_only_rpc",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7D receipt contract missing `{marker}`"
        );
    }
}

#[test]
fn phase7d_never_claims_real_roc_or_finality() {
    for marker in [
        r#""real_roc_burn":"#,
        r#""real_roc_mutation":"#,
        r#""production_bridge_settlement":"#,
        r#""mainnet_authorized":"#,
        r#""finality_claim":"#,
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing negative authority field `{marker}`"
        );
    }
}

#[test]
fn phase7d_is_compile_anchored_without_runtime_route() {
    assert!(SOURCE.contains("submit_phase7_once_and_readback;"));

    assert!(SOURCE.contains("const _: fn("));

    assert!(!SOURCE.contains("allow(dead_code)"));

    assert!(!SOURCE.contains("expect(dead_code)"));
}
