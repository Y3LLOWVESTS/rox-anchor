//! RO:WHAT — Tests BUILD_PLAN3 Phase 15 private pilot authority drill reports.
//! RO:WHY — Phase 15 requires authority, upgrade, halt, recovery, wrong-authority, and rotation drills.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and core authority / kill-switch review behavior.
//! RO:INVARIANTS — reports are local-only, redacted, and make no production safety or settlement claims.
//! RO:SECURITY — no private keys, wallet loading, RPC submission, signing, mint/burn, ROC mutation, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_drill_reports.

use rox_anchor_cli::run_from_args;

#[test]
fn phase15_authority_drill_report_is_operator_readable_and_local_only() {
    let output = run_from_args(["rox-anchor", "drill", "phase15"])
        .expect("phase15 drill report should render");

    for required in [
        "rox-anchor phase15 authority drill",
        "command: drill phase15",
        "scope: authority_upgrade_halt_recovery_operational_drills",
        "source: rox-anchor-core authority map review",
        "source: rox-anchor-core kill-switch review",
        "mode: local_report_only",
        "drill_count: 8",
        "strict_authority_separation_valid: true",
        "explicit_test_only_shared_authority_valid: true",
        "phase15_authority_checklist: local_only",
        "upgrade_authority_checklist: external_key_required",
        "mint_authority_checklist: separated_or_explicit_test_only",
        "halt_authority_checklist: can_block_acceptance_simulation_submission_finalization",
        "recovery_authority_checklist: can_only_recover_from_halted_recovery_required_state",
        "operator_role: upgrade_authority critical=true",
        "operator_role: mint_authority critical=true",
        "operator_role: halt_authority critical=true",
        "operator_role: recovery_authority critical=true",
        "phase15_wrong_authority_rejection_drill: local_only",
        "status: WrongAuthority",
        "phase15_halted_system_read_only_status_drill: local_only",
        "status: HaltAccepted",
        "read_only_status: available",
        "phase15_recovery_from_halt_drill: local_only",
        "status: RecoveryAccepted",
        "phase15_key_rotation_intent_drill: local_only",
        "rotation_scope: intent_only",
        "old_upgrade_authority: redacted_authority_key",
        "new_upgrade_authority: redacted_authority_key",
        "private_key_material: not_loaded",
        "keypair_path: redacted",
        "runtime: disabled",
        "wallet_key_loading: disabled",
        "rpc_submission: disabled",
        "signing: disabled",
        "mint_burn_execution: disabled",
        "internal_roc_mutation: disabled",
        "production_safety_claim: none",
        "settlement_claim: none",
        "public_bridge_authorization: none",
        "phase15_summary: authority_drills_are_local_redacted_and_operator_readable",
    ] {
        assert!(
            output.contains(required),
            "phase15 drill output missing `{required}`\n{output}"
        );
    }
}

#[test]
fn phase15_authority_drill_report_does_not_leak_or_claim_runtime_power() {
    let output = run_from_args(["rox-anchor", "drill", "phase15"])
        .expect("phase15 drill report should render");
    let lowered = output.to_ascii_lowercase();

    for forbidden in [
        "loaded keypair",
        "private key:",
        "secret",
        "seed phrase",
        "mnemonic",
        "rpc submitted",
        "network submitted=true",
        "signature:",
        "signed transaction",
        "upgrade complete",
        "minted",
        "burned",
        "roc released",
        "ron-ledger mutated",
        "settlement complete",
        "production safe",
        "mainnet",
        "public launch",
        "staking",
        "liquidity",
        "exchange",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "phase15 drill output must not contain unsafe phrase: {forbidden}\n{output}"
        );
    }
}

#[test]
fn drill_help_mentions_phase15_authority_drills() {
    let output =
        run_from_args(["rox-anchor", "drill", "--help"]).expect("drill help should render");

    assert!(output.contains("rox-anchor drill phase15"));
    assert!(output.contains("phase15 authority drills use core authority and kill-switch review"));
}
