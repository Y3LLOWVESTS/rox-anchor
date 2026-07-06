//! RO:WHAT — Tests CrabLink/internal ROC dry-run adapter status output.
//! RO:WHY — BUILD_PLAN3 Phase 11 requires display-safe dry-run handoff surfaces.
//! RO:INTERACTS — rox_anchor_cli status command and core internal ROC intent reports.
//! RO:INVARIANTS — CrabLink-facing status cannot claim backend proof settlement or mutate ROC.
//! RO:SECURITY — no wallet call, ledger mutation, paid unlock, mint/burn execution, or finality claim.
//! RO:TEST — cargo test -p rox-anchor-cli --test crablink_private_pilot_status.

#![forbid(unsafe_code)]

use rox_anchor_cli::run_from_args;

#[test]
fn status_includes_crablink_internal_roc_dry_run_adapter_shape() {
    let output = run_from_args(["rox-anchor", "status"]).expect("status should render");

    assert!(output.contains("crablink_internal_roc_dry_run_surface: display_safe_intent_shapes"));
    assert!(output.contains("internal_roc_burn_intent: dry_run_input"));
    assert!(output.contains("internal_roc_release_intent: dry_run_output"));
    assert!(output.contains("direction: roc_to_rox"));
    assert!(output.contains("direction: rox_to_roc"));
    assert!(output.contains("svc_wallet_call: disabled"));
    assert!(output.contains("ron_ledger_mutation: disabled"));
    assert!(output.contains("paid_content_unlock: disabled"));
    assert!(output.contains("real_internal_roc_burn: disabled"));
    assert!(output.contains("real_internal_roc_release: disabled"));
    assert!(output.contains("future_real_roc_path: svc-wallet -> ron-ledger only"));
    assert!(output.contains("crablink_final_settlement_display: disabled"));
    assert!(output.contains("crablink_internal_roc_adapter_settlement_claim: none"));

    assert!(!output.contains("crablink-status-test-account-0001"));
    assert!(!output.contains("crablink-status-test-account-0002"));
    assert!(!output.contains("paid_content_unlock: enabled"));
    assert!(!output.contains("ron_ledger_mutation: enabled"));
    assert!(!output.contains("settlement_claim: finalized"));
    assert!(!output.contains("final_settlement_display: complete"));
}
