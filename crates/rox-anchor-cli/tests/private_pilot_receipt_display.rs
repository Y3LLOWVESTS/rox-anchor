// RO:WHAT — CLI private-pilot receipt ledger display tests.
// RO:WHY — Proves Phase 9 receipt trail is terminal-visible, deterministic, redacted, and non-settling.
// RO:INTERACTS — rox_anchor_cli::run_from_args and rox-anchor-relayer pilot receipt ledger.
// RO:INVARIANTS — CLI displays receipts only; it does not submit, load keys, or claim settlement.
// RO:SECURITY — no live RPC, wallet, signing, transaction submission, minting, burning, ROC release, or settlement.
// RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_receipt_display.

#![forbid(unsafe_code)]

use rox_anchor_cli::run_from_args;

#[test]
fn receipts_command_prints_private_pilot_receipt_ledger() {
    let output = run_from_args(["rox-anchor", "receipts"]).expect("receipts command should run");

    assert!(output.contains("command: receipts"));
    assert!(output.contains("scope: private_pilot_local_receipt_ledger"));
    assert!(output.contains("network_submission: disabled_in_cli_report"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("settlement_claim: none"));

    assert!(output.contains("pilot_receipt_ledger=pilot-receipt-ledger-v1"));
    assert!(output.contains("entry_count=3"));
    assert!(output.contains("kind=proof_review"));
    assert!(output.contains("kind=transaction_simulation"));
    assert!(output.contains("kind=send_authorization"));
    assert!(output.contains("outcome_label=Authorized"));
    assert!(output.contains("live_submission_default=false"));
    assert!(output.contains("production_settlement_claim=false"));

    let lowered = output.to_ascii_lowercase();
    assert!(!lowered.contains("mainnet"));
    assert!(!lowered.contains("production settlement"));
    assert!(!lowered.contains("wallet_key_loading: enabled"));
    assert!(!lowered.contains("network_submitted=true"));
}

#[test]
fn receipt_ledger_alias_matches_receipts_command() {
    let direct = run_from_args(["rox-anchor", "receipts"]).expect("receipts command should run");
    let alias =
        run_from_args(["rox-anchor", "receipt-ledger"]).expect("receipt-ledger alias should run");

    assert_eq!(direct, alias);
}

#[test]
fn help_lists_receipt_command_without_authorizing_runtime() {
    let output = run_from_args(["rox-anchor", "--help"]).expect("help should render");

    assert!(output.contains("receipts"));
    assert!(output.contains("receipt-ledger"));
    assert!(output.contains("no silent RPC submission"));
    assert!(output.contains("no wallet/key loading"));
    assert!(output.contains("no settlement or finality claim"));
}
