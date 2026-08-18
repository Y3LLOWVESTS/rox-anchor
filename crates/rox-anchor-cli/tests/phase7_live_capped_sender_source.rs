//! RO:WHAT — Locks the BUILD_PLAN4 Phase 7A exact ROC-to-ROX candidate boundary.
//! RO:WHY — The first live mutation phase must not gain a hidden send path while
//! its exact observe+finalize transaction candidate is being built.
//! RO:INTERACTS — phase7_live_capped_sender.rs, pilot.rs, and compiled Anchor
//! client instruction types.
//! RO:INVARIANTS — actual Devnet bindings, fresh Phase 7 identity tuple,
//! exact one-unit caps, Phase 6 gate, and exact operator approval are required.
//! RO:SECURITY — source/CLI tests only; no RPC, key loading, signing, submission,
//! mint execution, ROC mutation, settlement, or mainnet.
//! RO:TEST — cargo test -p rox-anchor-cli --test phase7_live_capped_sender_source.

use std::{fs, path::PathBuf};

use rox_anchor_cli::run_from_args;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn source(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative))
        .unwrap_or_else(|error| panic!("failed to read {relative}: {error}"))
}

#[test]
fn phase7a_pilot_route_is_explicit_and_prepare_only() {
    let pilot = source("crates/rox-anchor-cli/src/commands/pilot.rs");
    let modules = source("crates/rox-anchor-cli/src/commands/mod.rs");

    assert!(modules.contains("pub mod phase7_live_capped_sender;"));
    assert!(pilot.contains(r#""phase7-prepare-capped-roc-to-rox""#));
    assert!(pilot.contains(r#""prepare-actual-roc-to-rox-send""#));
    assert!(
        pilot.contains("commands::phase7_live_capped_sender::run_phase7_prepare_capped_roc_to_rox")
    );

    let help = run_from_args([
        "rox-anchor",
        "pilot",
        "phase7-prepare-capped-roc-to-rox",
        "--help",
    ])
    .expect("Phase 7 help should route");

    assert!(help.contains("BUILD_PLAN4 Phase 7A"));
    assert!(help.contains("--prepare-only"));
    assert!(help.contains("--phase6-receipt"));
    assert!(help.contains("--operator-approval"));
    assert!(help.contains("--max-operations 1"));
    assert!(help.contains("--max-amount-minor 1"));
    assert!(help.contains("--retry-cap 1"));
}

#[test]
fn phase7a_binds_actual_deployed_devnet_accounts() {
    let source = source("crates/rox-anchor-cli/src/commands/phase7_live_capped_sender.rs");

    for required in [
        "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
        "4RBTypWtrn7mwV47MJkAHtEBMYnvNhd5wdSMAUsxwFeo",
        "HfHRJLswuRN3eVsiWnYi7REssDEsxxA8ewU8emhC3XA4",
        "A3sBYMUf2N7rpkqiCnE7fKZBdnGR5goH3hFmHJvgvqsJ",
        "C5jTCy4EBY5fKuRMzLv7Lau5Re1SmMXukRXosndk9hJE",
        "6YYJ43KRJF6pB3jUtRQpvhVHZQHaURTSxJdLpipHU3gs",
    ] {
        assert!(
            source.contains(required),
            "Phase 7 source missing reviewed binding {required}"
        );
    }
}

#[test]
fn phase7a_builds_exact_observe_then_mint_finalize_candidate() {
    let source = source("crates/rox-anchor-cli/src/commands/phase7_live_capped_sender.rs");

    for required in [
        "rox_anchor::accounts::ObserveBurn",
        "rox_anchor::instruction::ObserveBurn",
        "rox_anchor::accounts::FinalizeRocToRoxMint",
        "rox_anchor::instruction::FinalizeRocToRoxMint",
        "AnchorTransferDirection::RocToRox",
        "OperationBindingArgs",
        "RoxAnchorOperation::derive_address",
        "instruction_count: 2",
        "instruction_1: observe_burn",
        "instruction_2: finalize_roc_to_rox_mint",
        "required_signer_count: 1",
        "required_signer_role: workflow_authority",
    ] {
        assert!(
            source.contains(required),
            "Phase 7 exact transaction component missing: {required}"
        );
    }
}

#[test]
fn phase7a_uses_fresh_identity_and_exact_caps() {
    let source = source("crates/rox-anchor-cli/src/commands/phase7_live_capped_sender.rs");

    for required in [
        "actual-roc-to-rox-op-0001",
        "actual-roc-to-rox-idem-0001",
        "actual-roc-to-rox-nonce-0001",
        "shadow-roc-burn-intent-0001",
        "rox-anchor.phase7.shadow-roc-burn.v1",
        "PHASE7_AMOUNT_MINOR: u64 = 1",
        "PHASE7_MAX_AMOUNT_MINOR: u64 = 1",
        "PHASE7_MAX_OPERATIONS: u16 = 1",
        "PHASE7_RETRY_CAP: u8 = 1",
        "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
    ] {
        assert!(
            source.contains(required),
            "Phase 7 identity/cap invariant missing: {required}"
        );
    }

    assert!(source.contains(
        "Phase 7 must use a fresh operation/idempotency/nonce tuple distinct from Phase 6"
    ));
}

#[test]
fn phase7a_requires_successful_non_promotable_phase6_forward_evidence() {
    let source = source("crates/rox-anchor-cli/src/commands/phase7_live_capped_sender.rs");

    for required in [
        "rox-anchor.actual-private-testnet-simulation.v1",
        "BUILD_PLAN4 Phase 6",
        r#""direction", "roc_to_rox""#,
        r#""simulation_result", "passed""#,
        r#""read_only_evidence_status", "verified""#,
        r#""proof_review_status", "accepted""#,
        r#""coordinator_decision_status", "accepted""#,
        r#""relayer_dry_run_status", "accepted""#,
        r#""receipt_promotable_to_send""#,
        r#""transaction_submission""#,
        r#""send_authorized""#,
    ] {
        assert!(
            source.contains(required),
            "Phase 7 Phase 6 prerequisite missing: {required}"
        );
    }
}

#[test]
fn phase7a_contains_no_rpc_key_sign_or_send_api() {
    let source = source("crates/rox-anchor-cli/src/commands/phase7_live_capped_sender.rs");

    for forbidden in [
        "RpcClient",
        "read_keypair_file",
        "Keypair::",
        "send_transaction",
        "send_and_confirm_transaction",
        "process_transaction",
        "Transaction::new",
        "Transaction::new_signed",
    ] {
        assert!(
            !source.contains(forbidden),
            "Phase 7A candidate module must not contain live API {forbidden}"
        );
    }

    for required in [
        "rpc_calls: false",
        "keypair_loading: false",
        "signature_generated: false",
        "transaction_submission: false",
        "rox_mint_execution: false",
        "real_roc_mutation: false",
        "capped_sender_authorized: false",
        "live_submission_permitted: false",
    ] {
        assert!(
            source.contains(required),
            "Phase 7A safety report missing {required}"
        );
    }
}
