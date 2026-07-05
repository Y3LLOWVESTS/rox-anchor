//! RO:WHAT — CLI tests for Phase 12 kill-switch drill output.
//! RO:WHY — Proves halt/recovery drills are operator-visible without runtime side effects.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-core kill-switch review.
//! RO:INVARIANTS — accepted halt blocks unsafe progress; recovery requires correct authority and halted state.
//! RO:SECURITY — no RPC, wallet/key loading, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test kill_switch_drill_command.

use rox_anchor_cli::{run_from_args, CliError};

#[test]
fn help_lists_kill_switch_drill_command() {
    let output = run_from_args(["rox-anchor", "--help"]).expect("help should render");

    assert!(output.contains("drill"));
    assert!(output.contains("run local halt/recovery kill-switch drill"));
}

#[test]
fn drill_default_halt_is_accepted_and_blocks_all_unsafe_stages() {
    let output = run_from_args(["rox-anchor", "drill"]).expect("default drill should run");

    assert!(output.contains("command: drill"));
    assert!(output.contains("stage: before_proof_acceptance"));
    assert!(output.contains("action: halt"));
    assert!(output.contains("posture_fixture: clear"));
    assert!(output.contains("presented_authority: halt_authority"));
    assert!(output.contains("status=HaltAccepted"));
    assert!(output.contains("action_permitted=true"));
    assert!(output.contains("blocks_acceptance=true"));
    assert!(output.contains("blocks_simulation=true"));
    assert!(output.contains("blocks_submission=true"));
    assert!(output.contains("blocks_finalization=true"));
    assert!(output.contains("network_submitted: false"));
    assert!(output.contains("settlement_claim: none"));
}

#[test]
fn drill_recovery_from_halted_state_is_accepted_and_unblocks_progress() {
    let output = run_from_args([
        "rox-anchor",
        "drill",
        "--stage",
        "after-submit",
        "--action",
        "recover",
        "--posture",
        "halted-recovery-required",
        "--authority",
        "recovery",
    ])
    .expect("recovery drill should run");

    assert!(output.contains("stage: after_capped_testnet_submission"));
    assert!(output.contains("action: recover"));
    assert!(output.contains("posture_fixture: halted_recovery_required"));
    assert!(output.contains("presented_authority: recovery_authority"));
    assert!(output.contains("status=RecoveryAccepted"));
    assert!(output.contains("action_permitted=true"));
    assert!(output.contains("blocks_acceptance=false"));
    assert!(output.contains("blocks_simulation=false"));
    assert!(output.contains("blocks_submission=false"));
    assert!(output.contains("blocks_finalization=false"));
    assert!(output.contains("rpc_submission: disabled"));
}

#[test]
fn drill_wrong_authority_is_rejected_before_runtime_claims() {
    let output = run_from_args([
        "rox-anchor",
        "drill",
        "--stage=after-simulation",
        "--action=halt",
        "--posture=clear",
        "--authority=wrong",
    ])
    .expect("wrong authority drill should still render a refusal report");

    assert!(output.contains("stage: after_simulation_before_submission"));
    assert!(output.contains("status=WrongAuthority"));
    assert!(output.contains("action_permitted=false"));
    assert!(output.contains("review_accepted: false"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_submitted: false"));
    assert!(output.contains("public_bridge_authorization: none"));
}

#[test]
fn drill_recovery_without_halt_is_rejected() {
    let output = run_from_args([
        "rox-anchor",
        "drill",
        "--action",
        "recover",
        "--posture",
        "clear",
        "--authority",
        "recovery",
    ])
    .expect("invalid recovery drill should render a refusal report");

    assert!(output.contains("status=RecoveryRequiresHaltedState"));
    assert!(output.contains("action_permitted=false"));
    assert!(output.contains("review_accepted: false"));
}

#[test]
fn drill_unknown_flag_is_rejected() {
    let error = run_from_args(["rox-anchor", "drill", "--definitely-not-a-drill"]).unwrap_err();

    assert_eq!(
        error,
        CliError::UnknownDrillFlag("--definitely-not-a-drill".to_string())
    );
}
