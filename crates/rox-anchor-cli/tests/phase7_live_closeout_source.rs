//! BUILD_PLAN4 Phase 7F post-send closeout source contract.
//!
//! These tests prove the Phase 7F command is a read-only two-source
//! reconciliation/replay closeout and cannot become another send path.

#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase7_live_closeout.rs");
const MODULES: &str = include_str!("../src/commands/mod.rs");
const PILOT: &str = include_str!("../src/commands/pilot.rs");

#[test]
fn phase7f_is_registered_as_explicit_read_only_closeout() {
    assert!(MODULES.contains("pub mod phase7_live_closeout;"));
    assert!(PILOT.contains("\"phase7-post-send-closeout\" | \"closeout-actual-roc-to-rox-send\""));
    assert!(PILOT.contains("phase7_live_closeout::run_phase7_post_send_closeout"));

    for marker in [
        "--phase5-receipt",
        "--phase6-receipt",
        "--phase7b-authorization-receipt",
        "--send-receipt",
        "--readback-receipt",
        "--closeout-receipt-out",
        "--read-only-closeout",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7F missing CLI marker `{marker}`"
        );
    }
}

#[test]
fn phase7f_reuses_existing_evidence_validators_and_receipt_linkage() {
    for marker in [
        "validate_phase5_receipt",
        "validate_phase6_forward_receipt",
        "validate_phase7b_authorization_receipt",
        "rox-anchor.actual-roc-to-rox-capped-send.v1",
        "rox-anchor.actual-roc-to-rox-readback.v1",
        "transaction_signature_sha256",
        "send_readback_linkage_verified",
        "phase5_receipt_sha256",
        "phase6_receipt_sha256",
        "phase7b_authorization_receipt_sha256",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7F missing evidence marker `{marker}`"
        );
    }
}

#[test]
fn phase7f_requires_two_source_post_send_state_agreement() {
    for marker in [
        "PHASE5_DEVNET_RPC_URL",
        "PHASE5B_SOURCE2_RPC_URL",
        "PHASE5B_SOURCE1_LABEL",
        "PHASE5B_SOURCE2_LABEL",
        "phase5_read_only_rpc_retry",
        "get_multiple_accounts_with_context_compat",
        "Some(minimum_context_slot)",
        "PHASE5_STALE_AFTER_SLOTS",
        "independent providers disagree on post-send account bytes",
        "two_source_account_bytes_agree",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7F missing two-source marker `{marker}`"
        );
    }
}

#[test]
fn phase7f_proves_exact_finalized_one_unit_post_state() {
    for marker in [
        "build_phase7_capped_roc_to_rox_plan",
        "RoxAnchorConfig::try_deserialize",
        "Mint::unpack",
        "SplTokenAccount::unpack",
        "RoxAnchorOperation::try_deserialize",
        "mint.supply != PHASE7_AMOUNT_MINOR",
        "token.amount != PHASE7_AMOUNT_MINOR",
        "OperationStateCode::Finalized",
        "operation.challenge_open",
        "operation.recovery_required",
        "operation_id_hash != plan.operation_id_hash",
        "operation.burn_evidence_hash != plan.burn_evidence_hash",
        "operation.is_roc_to_rox()",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7F missing post-state marker `{marker}`"
        );
    }
}

#[test]
fn phase7f_rejects_consumed_identity_as_replay_without_resubmission() {
    for marker in [
        "ReplaySet::from_package",
        "review_proof_package",
        "ReviewDecision::Rejected",
        "ProofFindingCode::ReplayOperationId",
        "ProofFindingCode::ReplayIdempotencyKey",
        "ProofFindingCode::ReplayNonce",
        "replay_transaction_submitted\": false",
        "transaction_submission\": false",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 7F missing replay marker `{marker}`"
        );
    }

    for forbidden in [
        "read_keypair_file",
        "Keypair::",
        "Transaction::new_signed_with_payer",
        "send_and_confirm_transaction",
        ".send_transaction(",
        ".send_and_confirm_transaction(",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 7F must not contain live authority API `{forbidden}`"
        );
    }
}
