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

#[test]
fn drill_phase14_renders_coordinator_incident_drills_without_runtime_claims() {
    let output = run_from_args(["rox-anchor", "drill", "phase14"])
        .expect("phase14 drill report should render");

    assert!(output.contains("command: drill phase14"));
    assert!(output.contains("scope: live_testnet_chaos_and_incident_drills"));
    assert!(output.contains("source: rox-anchor-coordinator incident review"));
    assert!(output.contains("source: rox-anchor-relayer incident receipt review"));
    assert!(output.contains("source: rox-anchor-relayer capped sender authorization review"));
    assert!(output.contains("source: rox-anchor-rpc-proof readback review"));
    assert!(output.contains("mode: local_report_only"));
    assert!(output.contains("drill_count: 19"));
    assert!(output.contains("coordinator_drill_count: 7"));
    assert!(output.contains("relayer_receipt_drill_count: 7"));
    assert!(output.contains("relayer_sender_drill_count: 2"));
    assert!(output.contains("rpc_readback_drill_count: 3"));

    for required in [
        "drill: halt_before_simulation",
        "drill: halt_after_simulation_before_submit",
        "drill: halt_after_capped_submit",
        "drill: recovery_during_pending_operation",
        "drill: operator_approval_omitted",
        "drill: wrong_authority_attempted",
        "drill: readback_missing_after_send",
        "phase14_coordinator_incident_drill: local_only",
        "phase14_incident_receipt_review: local_only",
        "phase14_readback_after_send_review: local_only",
        "relayer_receipt_drill: missing_receipt_file",
        "relayer_receipt_drill: receipt_tamper",
        "relayer_receipt_drill: duplicate_receipt",
        "relayer_receipt_drill: duplicate_operation_id",
        "relayer_receipt_drill: duplicate_idempotency_key",
        "relayer_receipt_drill: nonce_reuse",
        "relayer_sender_drill: simulation_passes_but_send_disabled",
        "relayer_sender_drill: send_enabled_but_cap_exceeded",
        "phase14_sender_authorization_review: local_only",
        "scenario: simulation_passes_but_send_disabled",
        "scenario: send_enabled_but_cap_exceeded",
        "simulation_status: Simulated",
        "simulation_passed: true",
        "simulation_live_submission: false",
        "status: UnsafeExternalConfig",
        "status: CappedAuthorizationRejected",
        "capped_submit_status: AmountCapExceeded",
        "rpc_submission: disabled_in_local_authorization_model",
        "mint_burn_execution: disabled_in_local_authorization_model",
        "rpc_readback_drill: rpc_disagreement_during_readback",
        "rpc_readback_drill: rpc_stale_readback_after_send",
        "rpc_readback_drill: rpc_readback_missing_after_send",
        "status: FinalizationBlocked",
        "status: OperatorApprovalOmitted",
        "status: WrongAuthorityAttempted",
        "status: MissingReadbackAfterSend",
        "status: MissingReceiptFile",
        "status: ReceiptTamper",
        "status: DuplicateReceipt",
        "status: DuplicateOperationId",
        "status: DuplicateIdempotencyKey",
        "status: NonceReuse",
        "status: DisputedReadback",
        "status: RejectedReadback",
        "operator_action: halt_or_recover_before_retry",
        "transaction_submission: not_performed_by_rpc_proof",
        "wallet_key_loading: disabled",
        "rpc_submission: disabled",
        "signing: disabled",
        "mint_burn_execution: disabled",
        "internal_roc_mutation: disabled",
        "settlement_claim: none",
        "public_bridge_authorization: none",
        "phase14_summary: incidents_fail_safe_without_runtime_finality_or_settlement_claims",
    ] {
        assert!(
            output.contains(required),
            "phase14 drill output missing `{required}`\n{output}"
        );
    }

    for forbidden in [
        "settlement complete",
        "finality: confirmed",
        "mint complete",
        "burn complete",
        "access granted",
        "roc released",
        "loaded wallet",
        "loaded keypair",
        "public launch authorized",
    ] {
        assert!(
            !output.to_ascii_lowercase().contains(forbidden),
            "phase14 drill output must not contain unsafe phrase: {forbidden}\n{output}"
        );
    }
}

#[test]
fn drill_help_mentions_phase14_incident_drills() {
    let output =
        run_from_args(["rox-anchor", "drill", "--help"]).expect("drill help should render");

    assert!(output.contains("rox-anchor drill phase14"));
    assert!(output.contains(
        "phase14 incident drills use coordinator, relayer, and rpc-proof incident review"
    ));
}
