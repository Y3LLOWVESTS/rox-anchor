//! RO:WHAT — Locks BUILD_PLAN4 Phase 8A exact ROX-to-ROC simulation.
//! RO:WHY — Reverse execution must be proven against the real one-unit Devnet
//! state before a separately approved live burn can exist.
//! RO:SECURITY — source/CLI checks only; no key loading, signing, send, real
//! ROC release, production settlement, or mainnet.

#![forbid(unsafe_code)]

const SOURCE: &str = include_str!("../src/commands/phase8_rox_to_roc_simulation.rs");

const PILOT: &str = include_str!("../src/commands/pilot.rs");

const MODULES: &str = include_str!("../src/commands/mod.rs");

#[test]
fn phase8a_route_is_explicit_simulation_only() {
    assert!(MODULES.contains("pub mod phase8_rox_to_roc_simulation;"));

    assert!(PILOT.contains("\"phase8-simulate-rox-to-roc-burn\""));

    assert!(PILOT.contains("run_phase8_rox_to_roc_simulation"));

    for required in [
        "--simulate-only",
        "--phase7f-closeout",
        "--simulation-receipt-out",
        "--release-intent-receipt-out",
    ] {
        assert!(SOURCE.contains(required));
    }
}

#[test]
fn phase8a_uses_exact_real_devnet_bindings_and_fresh_identity() {
    for required in [
        "PHASE6_PROGRAM_ID",
        "PHASE6_CONFIG_ACCOUNT",
        "PHASE6_ROX_MINT",
        "PHASE6_TOKEN_ACCOUNT",
        "PHASE6_WORKFLOW_AUTHORITY",
        "PHASE6_MINT_AUTHORITY",
        "actual-rox-to-roc-op-0001",
        "actual-rox-to-roc-idem-0001",
        "actual-rox-to-roc-nonce-0001",
        "AnchorTransferDirection::RoxToRoc",
        "AnchorDirection::RoxToRoc",
        "parse_pubkey(PHASE6_PROGRAM_ID",
        "parse_pubkey(PHASE6_CONFIG_ACCOUNT",
        "parse_pubkey(PHASE6_ROX_MINT",
        "parse_pubkey(PHASE6_TOKEN_ACCOUNT",
        "parse_pubkey(PHASE6_WORKFLOW_AUTHORITY",
        "parse_pubkey(PHASE6_MINT_AUTHORITY",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing Phase 8 canonical binding `{required}`",
        );
    }
}

#[test]
fn phase8a_builds_exact_observe_then_real_burn_cpi_candidate() {
    for required in [
        "rox_anchor::accounts::ObserveBurn",
        "rox_anchor::instruction::ObserveBurn",
        "rox_anchor::accounts::FinalizeRoxToRocBurn",
        "rox_anchor::instruction::FinalizeRoxToRocBurn",
        "source_rox_token_account: token",
        "source_rox_token_authority: workflow",
        "amount_atoms: PHASE8_AMOUNT_MINOR",
        "Transaction::new_unsigned",
        "simulate_transaction",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing exact reverse candidate `{required}`",
        );
    }
}

#[test]
fn phase8a_requires_real_two_source_one_unit_post_phase7_state() {
    for required in [
        "get_multiple_accounts_with_context_compat",
        "PHASE6_SOURCE1",
        "PHASE6_SOURCE2",
        "PHASE5B_SOURCE2_RPC_URL",
        "Solana and Uniblock disagree",
        "mint_state.supply != PHASE8_AMOUNT_MINOR",
        "token_state.amount != PHASE8_AMOUNT_MINOR",
        "fresh Phase 8 operation PDA already exists",
        "ExpectedProofBinding::new",
        "CoordinatorReviewRequest::new",
        "review_coordinator_request",
        "RelayerDryRun::new",
        "DryRunAccepted",
        "PrivatePilotSimulationStatus::Simulated",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing Phase 8 gate `{required}`",
        );
    }
}

#[test]
fn phase8a_emits_dry_run_release_intent_without_real_roc_authority() {
    for required in [
        "InternalRocDryRunReleaseIntent::new",
        "test-only-internal-roc-release-intent",
        "svc-wallet -> ron-ledger only",
        "\"real_internal_roc_release\":",
        "\"svc_wallet_call\":",
        "\"ron_ledger_mutation\":",
        "\"receipt_promotable_to_live_burn\":",
    ] {
        assert!(
            SOURCE.contains(required),
            "missing release boundary `{required}`",
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
        "svc_wallet::",
        "ron_ledger::",
    ] {
        assert!(
            !SOURCE.contains(forbidden),
            "Phase 8A must not contain authority API `{forbidden}`",
        );
    }
}
