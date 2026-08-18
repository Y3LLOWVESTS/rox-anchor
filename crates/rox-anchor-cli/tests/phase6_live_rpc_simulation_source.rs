#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase6_live_rpc_simulation.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

#[test]
fn phase6b_builds_both_actual_direction_plans() {
    for marker in [
        "SimulationDirection::RocToRox",
        "SimulationDirection::RoxToRoc",
        "AnchorTransferDirection::RocToRox",
        "AnchorTransferDirection::RoxToRoc",
        "AnchorDirection::RocToRox",
        "AnchorDirection::RoxToRoc",
        "rox_anchor::accounts::ObserveBurn",
        "rox_anchor::instruction::ObserveBurn",
        "RoxAnchorOperation::derive_address",
        "PHASE6_CONFIG_ACCOUNT",
        "PHASE6_ROX_MINT",
        "PHASE6_TOKEN_ACCOUNT",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 6B missing actual directional binding marker `{marker}`"
        );
    }
}

#[test]
fn phase6b_uses_unsigned_simulation_without_key_loading_or_submission() {
    for marker in [
        "Transaction::new_unsigned",
        "simulate_transaction",
        "Signature::default",
        "unsigned_transaction",
        "\"signature_generated\":",
        "\"transaction_submission\":",
        "\"send_authorized\":",
        "\"wallet_loaded\":",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 6B missing unsigned simulation marker `{marker}`"
        );
    }

    for forbidden in [
        "read_keypair_file",
        "Keypair::",
        "new_signed_with_payer",
        "partial_sign",
        "try_sign",
        "send_and_confirm_transaction",
        "send_transaction(",
        "send_transaction_with_config",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 6B must not contain signing/submission API `{forbidden}`"
        );
    }
}

#[test]
fn phase6b_requires_fresh_phase5_evidence_and_existing_gate_chain() {
    for marker in [
        "validate_phase5_receipt",
        "verify_phase5_freshness",
        "PHASE6_STALE_AFTER_SLOTS",
        "review_coordinator_request",
        "CoordinatorDecisionStatus::Accepted",
        "ReviewDecision::Accepted",
        "RelayerDryRun::new",
        "RelayerReceiptStatus::DryRunAccepted",
        "PrivatePilotTransactionKind::Observe",
        "PrivatePilotSimulationStatus::Simulated",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 6B missing gate marker `{marker}`"
        );
    }
}

#[test]
fn phase6b_proves_simulation_does_not_persist_state() {
    for marker in [
        "operation PDA already exists before simulation",
        "simulated operation PDA persisted unexpectedly",
        "config bytes changed after simulation",
        "mint bytes changed after simulation",
        "token-account bytes changed after simulation",
        "\"operation_persisted_after_simulation\":",
        "\"config_bytes_unchanged\":",
        "\"mint_bytes_unchanged\":",
        "\"token_account_bytes_unchanged\":",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 6B missing non-persistence marker `{marker}`"
        );
    }
}

#[test]
fn phase6b_writes_non_promotable_receipts_for_both_directions() {
    for marker in [
        "rox-anchor.actual-private-testnet-simulation.v1",
        "actual_private_testnet_simulation_receipt",
        "\"roc_to_rox\"",
        "\"rox_to_roc\"",
        "\"receipt_promotable_to_send\":",
        "\"real_roc_mutation\":",
        "\"public_rox_mint_burn\":",
        "\"finality_claim\":",
        "<redacted-program-account>",
        "<redacted-program-config-account>",
        "<redacted-test-only-mint>",
        "<redacted-test-only-token-account>",
    ] {
        assert!(
            SOURCE.contains(marker),
            "Phase 6B missing receipt boundary `{marker}`"
        );
    }
}

#[test]
fn phase6b_pilot_route_is_explicit_and_separate_from_generic_simulate() {
    assert!(PILOT.contains("\"phase6-live-rpc-simulation\" | \"actual-address-live-simulation\""));

    assert!(PILOT.contains("run_phase6_live_rpc_simulation"));

    assert!(
        PILOT.contains("\"simulate\" | \"simulation\" => run_pilot_simulate(rest)"),
        "existing local simulation route must remain intact"
    );
}
