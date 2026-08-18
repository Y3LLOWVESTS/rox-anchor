use std::{fs, path::Path};

fn source(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);

    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("could not read {}: {error}", path.display(),))
}

#[test]
fn phase6a_binds_exact_live_devnet_accounts() {
    let source = source("src/commands/phase6_live_simulation.rs");

    for required in [
        "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
        "4RBTypWtrn7mwV47MJkAHtEBMYnvNhd5wdSMAUsxwFeo",
        "HfHRJLswuRN3eVsiWnYi7REssDEsxxA8ewU8emhC3XA4",
        "A3sBYMUf2N7rpkqiCnE7fKZBdnGR5goH3hFmHJvgvqsJ",
        "C5jTCy4EBY5fKuRMzLv7Lau5Re1SmMXukRXosndk9hJE",
        "6YYJ43KRJF6pB3jUtRQpvhVHZQHaURTSxJdLpipHU3gs",
        "3aAvoLEAsCCte4gow6rheJQ3F4zeoCuMvERqyFBobGgz",
        "74upNee16zSKS2hSuovDaioWVsadFf8Za4CCRwJW5fqe",
    ] {
        assert!(
            source.contains(required),
            "Phase 6A source missing actual binding `{required}`"
        );
    }
}

#[test]
fn phase6a_requires_real_phase5_closeout_and_simulate_only() {
    let source = source("src/commands/phase6_live_simulation.rs");

    for required in [
        "--simulate-only",
        "--phase5-receipt",
        "--receipt-out",
        "rox-anchor.phase5-read-only-closeout.v1",
        "BUILD_PLAN4 Phase 5B2",
        "solana-public-devnet-primary",
        "uniblock-devnet-secondary",
        "rpc_proof_decision",
        "Agreement",
        "phase5_closeout",
        "metadata_source_slot_delta",
        "PHASE6_STALE_AFTER_SLOTS",
    ] {
        assert!(
            source.contains(required),
            "Phase 6A source missing Phase 5 gate `{required}`"
        );
    }
}

#[test]
fn phase6a_reuses_existing_proof_coordinator_and_relayer_models() {
    let source = source("src/commands/phase6_live_simulation.rs");

    for required in [
        "ProofPackage::new",
        "EvidenceBundle::satisfied",
        "ExpectedProofBinding::new",
        "ExpectedRpcBinding::new",
        "RpcObservation::new",
        "CoordinatorReviewRequest::new",
        "review_coordinator_request",
        "decision.permits_transaction_simulation()",
        "RelayerDryRun::new",
        "RelayerSubmissionRequest::new",
        "TransactionSimulationPlan::from_dry_run_receipt",
        "PrivatePilotSimulationPlan::new",
        "PrivatePilotTransactionKind::Halt",
        "simulate_private_pilot_transaction_plan",
        "PrivatePilotSimulationStatus::Simulated",
    ] {
        assert!(
            source.contains(required),
            "Phase 6A source missing existing gate reuse `{required}`"
        );
    }
}

#[test]
fn phase6a_keeps_live_runtime_effects_deferred_and_non_sendable() {
    let source = source("src/commands/phase6_live_simulation.rs");

    for required in [
        "live_rpc_simulation: deferred_until_phase6b",
        "final_phase6_receipt_written: false",
        "receipt_promotable_to_send: false",
        "transaction_submission: false",
        "keypair_loading: false",
        "signing: false",
        "rox_mint_execution: false",
        "rox_burn_execution: false",
        "real_roc_mutation: false",
        "production_settlement: false",
        "finality_claim: false",
        "mainnet_authorized: false",
    ] {
        assert!(
            source.contains(required),
            "Phase 6A source missing safety marker `{required}`"
        );
    }

    for forbidden in [
        "read_keypair_file",
        "Keypair::",
        "send_transaction(",
        "send_and_confirm_transaction",
        "send_transaction_with_config",
        "sign_message",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 6A must not contain runtime authority API `{forbidden}`"
        );
    }
}

#[test]
fn phase6a_pilot_route_is_registered_without_send_aliases() {
    let pilot = source("src/commands/pilot.rs");

    assert!(
        pilot.contains(
            "\"phase6-actual-address-simulation-gate\" | \"actual-address-simulation-gate\""
        ),
        "Phase 6A pilot route missing"
    );

    assert!(
        pilot.contains("run_phase6_actual_address_simulation_gate"),
        "Phase 6A pilot route must call real gate implementation"
    );
}
