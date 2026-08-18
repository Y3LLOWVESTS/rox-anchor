//! RO:WHAT — Focused source-boundary tests for BUILD_PLAN4 Phase 5B.
//! RO:WHY — Proves the live quorum command is fixed to two independent
//! providers and delegates agreement to rox-anchor-rpc-proof.
//! RO:INTERACTS — phase5_live_quorum.rs, phase5_live_read_only.rs, pilot.rs.
//! RO:INVARIANTS — fixed provider endpoints, quorum=2, no arbitrary RPC flags,
//! no submission/key loading/mint/burn behavior, Phase 5 remains open here.
//! RO:SECURITY — source inspection and help only; no live RPC.
//! RO:TEST — cargo test -p rox-anchor-cli --test phase5_live_quorum_source.

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
fn phase5b_help_exposes_fixed_two_source_quorum() {
    let output = run_from_args(&[
        "rox-anchor".to_string(),
        "pilot".to_string(),
        "phase5-read-only-quorum".to_string(),
        "--help".to_string(),
    ])
    .expect("Phase 5B help should render without RPC");

    for required in [
        "BUILD_PLAN4 Phase 5B",
        "solana-public-devnet-primary",
        "https://api.devnet.solana.com",
        "uniblock-devnet-secondary",
        "https://api.uniblock.dev/uni/v1/json-rpc?chainId=solana-devnet",
        "requires two distinct matching observations",
        "requires rpc-proof Agreement",
        "no arbitrary RPC endpoint flags",
    ] {
        assert!(
            output.contains(required),
            "Phase 5B help missing `{required}`"
        );
    }
}

#[test]
fn phase5b_command_rejects_missing_receipt_arguments_before_rpc() {
    let error = run_from_args(&[
        "rox-anchor".to_string(),
        "pilot".to_string(),
        "phase5-read-only-quorum".to_string(),
    ])
    .expect_err("missing Phase 5B inputs must fail");

    assert!(error.to_string().contains("requires --init-receipt"));
}

#[test]
fn phase5b_source_is_fixed_to_distinct_providers_and_existing_quorum_engine() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_quorum.rs");

    for required in [
        "https://api.devnet.solana.com",
        "https://api.uniblock.dev/uni/v1/json-rpc?chainId=solana-devnet",
        "solana-public-devnet-primary",
        "uniblock-devnet-secondary",
        "collect_single_source_evidence",
        "review_rpc_observations",
        "RpcProofConfig::new",
        "RpcQuorumDecision::Agreement",
        "PHASE5_REQUIRED_OBSERVATIONS",
        "\"phase5_closeout\": false",
        "\"upgrade_authority_multi_source_verified\": false",
        "\"deployment_metadata_multi_source_verified\": false",
    ] {
        assert!(
            source.contains(required),
            "Phase 5B source missing `{required}`"
        );
    }

    for forbidden in [
        "\"--rpc-url\"",
        "\"--source\"",
        "read_keypair_file",
        "send_transaction",
        "send_and_confirm_transaction",
        "simulate_transaction",
        "Keypair",
        "mint_to",
        "burn(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 5B source unexpectedly contains `{forbidden}`"
        );
    }
}

#[test]
fn phase5a_history_repair_and_single_source_policy_are_preserved() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_read_only.rs");

    for required in [
        "get_signature_status_with_commitment_and_history",
        "CommitmentConfig::confirmed()",
        "true",
        "RpcQuorumDecision::MissingEvidence",
        "\"under_quorum_rejected\": true",
        "\"phase5_closeout\": false",
        "\"rpc_endpoint_class\": endpoint_class",
        "\"explicit-official-devnet\"",
    ] {
        assert!(
            source.contains(required),
            "Phase 5A source missing preserved marker `{required}`"
        );
    }
}

#[test]
fn phase5b_does_not_claim_full_phase5_closeout_before_metadata_review() {
    let source = source("crates/rox-anchor-cli/src/commands/phase5_live_quorum.rs");

    assert!(source
        .contains("\"next_action\": \"VERIFY_PHASE5B_UPGRADE_AUTHORITY_AND_DEPLOYMENT_METADATA\""));

    assert!(source.contains("\"phase5_closeout\": false"));

    assert!(!source.contains("\"phase5_closeout\": true"));
}
