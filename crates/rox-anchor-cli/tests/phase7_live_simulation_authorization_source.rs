#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase7_live_simulation_authorization.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

#[test]
fn phase7b_reuses_exact_phase7a_candidate() {
    for marker in [
        "build_phase7_capped_roc_to_rox_plan",
        "is_exact_phase7_shape",
        "PHASE7_OPERATION_ID",
        "PHASE7_IDEMPOTENCY_KEY",
        "PHASE7_NONCE",
        "candidate_instruction_count: 2",
        "candidate_instruction_1: observe_burn",
        "candidate_instruction_2: finalize_roc_to_rox_mint",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing exact candidate marker `{marker}`"
        );
    }
}

#[test]
fn phase7b_requires_fresh_prior_evidence_and_real_gate_chain() {
    for marker in [
        "validate_phase5_receipt",
        "verify_phase5_freshness",
        "validate_phase6_forward_receipt",
        "ExpectedProofBinding::new",
        "CoordinatorReviewRequest::new",
        "review_coordinator_request",
        "RelayerDryRun::new",
        "PrivatePilotTransactionKind::Observe",
        "PrivatePilotTransactionKind::Finalize",
        "PrivatePilotSimulationStatus::Simulated",
    ] {
        assert!(SOURCE.contains(marker), "missing gate marker `{marker}`");
    }
}

#[test]
fn phase7b_performs_live_unsigned_simulation_only() {
    for marker in [
        "RpcClient",
        "Transaction::new_unsigned",
        "simulate_transaction",
        "Signature::default",
        "persistent_operation_after_simulation: false",
        "persistent_mint_change_after_simulation: false",
        "persistent_token_account_change_after_simulation: false",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing live simulation marker `{marker}`"
        );
    }

    for forbidden in [
        "read_keypair_file",
        "Keypair::",
        "partial_sign",
        "try_sign",
        "new_signed_with_payer",
        "send_transaction(",
        "send_transaction_with_config",
        "send_and_confirm_transaction",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 7B must not contain execution API `{forbidden}`"
        );
    }
}

#[test]
fn phase7b_uses_existing_sender_authorization_without_executing_it() {
    for marker in [
        "authorize_private_testnet_sender",
        "PrivateTestnetSenderRequest::new",
        "CappedTestnetSubmissionLimits::new",
        "PRIVATE_TESTNET_CAPPED_SEND_APPROVAL",
        "PrivateTestnetSenderStatus::Authorized",
        "live_submission_attempted",
        "network_submitted",
        "wallet_key_loading",
        "signing",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing sender authorization marker `{marker}`"
        );
    }
}

#[test]
fn phase7b_keeps_both_operator_approval_vocabularies_explicit() {
    assert!(SOURCE.contains("I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND"));

    assert!(SOURCE.contains("I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT"));

    assert!(SOURCE.contains("approval_translation_explicit"));
}

#[test]
fn phase7b_writes_authorization_evidence_not_a_send_receipt() {
    for marker in [
        "rox-anchor.phase7-simulation-authorization.v1",
        "simulation_and_sender_authorization_evidence",
        "\"send_receipt\":",
        "\"transaction_submission\":",
        "\"signature_generated\":",
        "\"real_roc_burn\":",
        "\"real_roc_mutation\":",
        "\"finality_claim\":",
        "PHASE7C_BUILD_LIVE_SIGNED_EXECUTOR_WITHOUT_RUNNING_IT",
    ] {
        assert!(
            SOURCE.contains(marker),
            "missing authorization receipt boundary `{marker}`"
        );
    }
}

#[test]
fn phase7b_pilot_route_is_explicit() {
    assert!(PILOT.contains(
        "\"phase7-simulate-and-authorize-roc-to-rox\" | \"phase7-live-simulation-authorization\""
    ));

    assert!(PILOT.contains("run_phase7_simulate_and_authorize"));
}
