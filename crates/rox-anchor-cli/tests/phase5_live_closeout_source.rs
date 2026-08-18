//! RO:WHAT — Focused source-boundary tests for BUILD_PLAN4 Phase 5B2.
//! RO:WHY — Locks loader-v3 metadata, binary, signature, and safety behavior
//! before any live Phase 5 closeout RPC run.
//! RO:INTERACTS — phase5_live_closeout.rs and Phase 5B1 quorum command.
//! RO:INVARIANTS — exact ProgramData/authority/slot/hash; fresh B1 Agreement;
//! fixed independent RPCs; closeout only after metadata verification.
//! RO:SECURITY — source inspection/help only; no live RPC.
//! RO:TEST — cargo test -p rox-anchor-cli --test phase5_live_closeout_source.

#![forbid(unsafe_code)]

use std::{fs, path::PathBuf};

use rox_anchor_cli::run_from_args;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn source(path: &str) -> String {
    fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|error| panic!("could not read {path}: {error}"))
}

#[test]
fn phase5b2_help_exposes_exact_private_devnet_metadata() {
    let output = run_from_args(&[
        "rox-anchor".to_string(),
        "pilot".to_string(),
        "phase5-read-only-closeout".to_string(),
        "--help".to_string(),
    ])
    .expect("Phase 5B2 help should render without RPC");

    for required in [
        "BUILD_PLAN4 Phase 5B2",
        "4JsBSTEXLKtWusJQJAv1DnaRKfAxGnD958WhHPVz84UD",
        "DLQJ1icSQKu5CGsi7FqJgF9ohsiYuYuRkn23EggRDTdJ",
        "484017674",
        "398864",
        "392488",
        "6376",
        "929f1906d497ed22c8e88c8a73bcaae0181271d9001fa1a98a9f8e3c50c45bf1",
        "no arbitrary RPC endpoint flags",
        "no transaction submission",
    ] {
        assert!(
            output.contains(required),
            "Phase 5B2 help missing `{required}`"
        );
    }
}

#[test]
fn phase5b2_missing_inputs_fail_before_live_rpc() {
    let error = run_from_args(&[
        "rox-anchor".to_string(),
        "pilot".to_string(),
        "phase5-read-only-closeout".to_string(),
    ])
    .expect_err("missing Phase 5B2 inputs must fail");

    assert!(error.to_string().contains("requires --init-receipt"));
}

#[test]
fn phase5b2_uses_loader_v3_and_fresh_b1_quorum() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for required in [
        "run_phase5_live_quorum",
        "UpgradeableLoaderState",
        "UpgradeableLoaderState::Program",
        "UpgradeableLoaderState::ProgramData",
        "size_of_programdata_metadata",
        "get_signature_status_with_commitment_and_history",
        "CommitmentConfig::confirmed()",
        "Sha256::digest",
        "PHASE5_STALE_AFTER_SLOTS",
        "\"fresh_state_quorum\": true",
        "\"upgrade_authority_multi_source_verified\": true",
        "\"deployment_metadata_multi_source_verified\": true",
        "\"program_binary_multi_source_verified\": true",
        "\"deploy_signature_multi_source_verified\": true",
        "\"phase5_closeout\": true",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 source missing `{required}`"
        );
    }
}

#[test]
fn phase5b2_is_fixed_to_exact_phase4e_deployment() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for required in [
        "PHASE5_PROGRAM_ID",
        "4JsBSTEXLKtWusJQJAv1DnaRKfAxGnD958WhHPVz84UD",
        "DLQJ1icSQKu5CGsi7FqJgF9ohsiYuYuRkn23EggRDTdJ",
        "484_017_674",
        "398_864",
        "392_488",
        "6_376",
        "929f1906d497ed22c8e88c8a73bcaae0181271d9001fa1a98a9f8e3c50c45bf1",
        "3hcbn13eMpvTHqrwMeJdFVND4jsGy3RpbVUSKNJv4PJinpkgtboAMo5BAbxwhTJstDzA5AKPQxH96Rk7atStm4tT",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 source missing locked Phase 4E binding `{required}`"
        );
    }
}

#[test]
fn phase5b2_has_no_mutating_or_key_loading_api() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for forbidden in [
        "\"--rpc-url\"",
        "\"--source\"",
        "read_keypair_file",
        "Keypair",
        "new_signed_with_payer",
        "send_transaction(",
        "send_and_confirm_transaction(",
        "simulate_transaction(",
        "mint_to(",
        "burn(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 5B2 source unexpectedly contains `{forbidden}`"
        );
    }

    for required in [
        "\"transaction_submission\": false",
        "\"keypair_loading\": false",
        "\"signing\": false",
        "\"simulation\": false",
        "\"rox_mint_performed\": false",
        "\"rox_burn_performed\": false",
        "\"real_roc_mutation\": false",
        "\"production_settlement\": false",
        "\"finality_claim\": false",
        "\"settlement_claim\": false",
        "\"mainnet\": false",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 safety receipt missing `{required}`"
        );
    }
}

#[test]
fn phase5b1_remains_non_closeout_and_two_source() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_quorum.rs");

    for required in [
        "solana-public-devnet-primary",
        "uniblock-devnet-secondary",
        "RpcQuorumDecision::Agreement",
        "\"phase5_closeout\": false",
        "\"upgrade_authority_multi_source_verified\": false",
        "\"deployment_metadata_multi_source_verified\": false",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B1 regression marker missing `{required}`"
        );
    }
}

#[test]
fn phase5b2_keeps_rpc_staleness_on_observation_pairs_not_cross_stage_latency() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for required in [
        "state_quorum_to_metadata_slot_delta",
        "metadata_source_slot_delta",
        "source1.observation_slot",
        "metadata_source_slot_delta > PHASE5_STALE_AFTER_SLOTS",
        "\"state_quorum_fresh_by_rpc_proof\": true",
        "\"cross_stage_slot_delta_policy\": \"telemetry_not_rpc_staleness\"",
        "\"metadata_pair_fresh\": true",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 freshness source missing `{required}`"
        );
    }

    assert!(
        !source.contains(
            "fresh two-source state quorum became stale before metadata closeout"
        ),
        "Phase 5B2 must not reinterpret the RPC observation stale window as a cross-stage workflow timeout"
    );

    assert!(
        !source.contains("\"fresh_state_quorum_age_slots\""),
        "deprecated cross-stage freshness field must be absent"
    );
}

#[test]
fn phase5b2_uses_rpc_context_slot_for_metadata_freshness() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for required in [
        "get_multiple_accounts_with_context_compat",
        "account_response.context_slot",
        "observation_slot",
        "source1.observation_slot < metadata_min_context_slot",
        "source2.observation_slot < metadata_min_context_slot",
        "source1.observation_slot",
        "source2.observation_slot",
        "metadata_source_slot_delta > PHASE5_STALE_AFTER_SLOTS",
        "\"metadata_source_1_not_older_than_fresh_quorum\": true",
        "\"metadata_source_2_not_older_than_fresh_quorum\": true",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 context-slot source missing `{required}`"
        );
    }

    for forbidden in [
        "slot_before",
        "slot_after",
        "collection_window_slots",
        "deployment metadata collection exceeded the Phase 5 freshness window",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 5B2 context-slot source unexpectedly contains `{forbidden}`"
        );
    }
}

#[test]
fn phase5b2_enforces_common_b1_min_context_slot() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for required in [
        "minimum_context_slot: u64",
        "phase5_wire_compat::get_multiple_accounts_with_context_compat",
        "Some(minimum_context_slot)",
        "get_multiple_accounts_with_context_compat",
        "metadata_min_context_slot",
        "quorum.source1_slot.max(quorum.source2_slot)",
        "source1.observation_slot < metadata_min_context_slot",
        "source2.observation_slot < metadata_min_context_slot",
        "\"metadata_min_context_enforced\": true",
        "metadata_source_slot_delta > PHASE5_STALE_AFTER_SLOTS",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 min-context source missing `{required}`"
        );
    }
}

#[test]
fn phase5b2_collects_final_metadata_pair_in_parallel() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_closeout.rs");

    for required in [
        "std::thread::scope",
        "let source1_handle = scope.spawn",
        "let source2_handle = scope.spawn",
        "PHASE5_DEVNET_RPC_URL",
        "PHASE5B_SOURCE2_RPC_URL",
        "metadata_min_context_slot",
        "\"metadata_collection_mode\": \"parallel_common_min_context\"",
        "metadata_source_slot_delta > PHASE5_STALE_AFTER_SLOTS",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B2 parallel metadata source missing `{required}`"
        );
    }
}
