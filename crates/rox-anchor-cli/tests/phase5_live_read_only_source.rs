//! BUILD_PLAN4 Phase 5A command-boundary tests.
//!
//! These tests never invoke live RPC. They prove the live collector is
//! explicitly read-only and cannot falsely promote one source to quorum.

#![forbid(unsafe_code)]

use std::fs;

use rox_anchor_cli::run_from_args;

#[test]
fn phase5_help_exposes_explicit_read_only_live_surface() {
    let report = run_from_args(["rox-anchor", "pilot", "phase5-read-only-live", "--help"])
        .expect("Phase 5A help should render without RPC");

    for marker in [
        "--init-receipt",
        "--receipt-out",
        "--rpc-url",
        "--source",
        "live read-only devnet RPC",
        "loads no operator keypairs",
        "signs nothing",
        "simulates nothing",
        "submits nothing",
        "requires two distinct observations for quorum",
        "Phase 5 is not closed by one source",
    ] {
        assert!(
            report.contains(marker,),
            "missing Phase 5A help marker: {marker}",
        );
    }
}

#[test]
fn phase5_missing_arguments_fail_before_any_rpc_access() {
    let error = run_from_args(["rox-anchor", "pilot", "phase5-read-only-live"])
        .expect_err("Phase 5A must fail closed without explicit inputs");

    assert!(error.to_string().contains("requires --init-receipt",),);
}

#[test]
fn phase5_live_source_contains_no_mutation_or_key_loading_api() {
    let source = include_str!("../src/commands/phase5_live_read_only.rs");

    for forbidden in [
        "read_keypair_file",
        "new_signed_with_payer",
        "simulate_transaction(",
        "send_transaction(",
        "send_and_confirm_transaction(",
        "send_with_spinner",
        "mint_to(",
        "burn(",
    ] {
        assert!(
            !source.contains(forbidden,),
            "read-only Phase 5 source contains forbidden API: {forbidden}",
        );
    }

    assert!(source.contains("get_multiple_accounts",),);

    assert!(source.contains("get_signature_status_with_commitment_and_history",),);
}

#[test]
fn phase5_single_source_is_explicitly_bound_to_missing_evidence() {
    let source = include_str!("../src/commands/phase5_live_read_only.rs");

    for marker in [
        "PHASE5_REQUIRED_OBSERVATIONS: u16 = 2",
        "RpcQuorumDecision::MissingEvidence",
        "review_rpc_observations",
        "RpcProofAuditRecord::from_review",
        "\"under_quorum_rejected\": true",
        "\"phase5_closeout\": false",
    ] {
        assert!(
            source.contains(marker,),
            "Phase 5A source missing quorum boundary: {marker}",
        );
    }
}

#[test]
fn phase5_source_is_locked_to_actual_phase4_devnet_bindings() {
    let source = include_str!("../src/commands/phase5_live_read_only.rs");

    for marker in [
        "https://api.devnet.solana.com",
        "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
        "4RBTypWtrn7mwV47MJkAHtEBMYnvNhd5wdSMAUsxwFeo",
        "HfHRJLswuRN3eVsiWnYi7REssDEsxxA8ewU8emhC3XA4",
        "A3sBYMUf2N7rpkqiCnE7fKZBdnGR5goH3hFmHJvgvqsJ",
        "C5jTCy4EBY5fKuRMzLv7Lau5Re1SmMXukRXosndk9hJE",
        "PHASE5_STALE_AFTER_SLOTS",
    ] {
        assert!(
            source.contains(marker,),
            "Phase 5A source missing actual binding: {marker}",
        );
    }

    assert!(!source.contains("api.mainnet-beta.solana.com",),);
}

#[test]
fn phase5_receipt_schema_is_redacted_and_non_authoritative() {
    let source = fs::read_to_string("src/commands/phase5_live_read_only.rs")
        .expect("Phase 5A source should be readable");

    for marker in [
        "rox-anchor.phase5-read-only-source.v1",
        "initialization_signature_redacted",
        "\"transaction_submission\": false",
        "\"keypair_loading\": false",
        "\"signing\": false",
        "\"simulation\": false",
        "\"rox_mint_performed\": false",
        "\"rox_burn_performed\": false",
        "\"real_roc_mutation\": false",
        "\"production_settlement\": false",
        "\"mainnet\": false",
    ] {
        assert!(
            source.contains(marker,),
            "Phase 5A receipt boundary missing: {marker}",
        );
    }
}

#[test]
fn phase5_read_only_retry_is_bounded_paced_and_non_mutating() {
    let source = include_str!("../src/commands/phase5_live_read_only.rs");

    for required in [
        "PHASE5_READ_ONLY_RPC_MAX_ATTEMPTS: u8 = 4",
        "PHASE5_UNIBLOCK_SUCCESS_PACING_MS",
        "PHASE5_READ_ONLY_RPC_BASE_BACKOFF_MS",
        "PHASE5_READ_ONLY_RPC_RATE_LIMIT_BACKOFF_MS",
        "\"429\"",
        "\"too many requests\"",
        "\"error sending request\"",
        "std::thread::sleep",
        "phase5_read_only_rpc_retry",
        "get_multiple_accounts",
        "get_signature_status_with_commitment_and_history",
    ] {
        assert!(
            source.contains(required),
            "Phase 5 read-only retry source missing `{required}`",
        );
    }

    for forbidden in [
        "read_keypair_file",
        "new_signed_with_payer",
        "send_transaction(",
        "send_and_confirm_transaction(",
        "mint_to(",
        "burn(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 5 read-only retry path contains forbidden API `{forbidden}`",
        );
    }
}
