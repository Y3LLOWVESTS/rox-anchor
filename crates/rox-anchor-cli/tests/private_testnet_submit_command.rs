//! RO:WHAT — Tests BUILD_PLAN3 Phase 8 CLI command shape for explicit capped private testnet submit authorization.
//! RO:WHY — Ensures CLI exposes only report/authorization behavior and no default send path.
//! RO:INTERACTS — rox_anchor_cli::run_from_args and submit command dispatch.
//! RO:INVARIANTS — nested submit command is explicit; unknown submit subcommands fail closed.
//! RO:SECURITY — no RPC, wallet, key loading, transaction send, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_testnet_submit_command.

use std::{fs, path::PathBuf};

use rox_anchor_cli::{run_from_args, CliError};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

#[test]
fn submit_group_help_has_no_default_send_path() {
    let output = run_from_args(["rox-anchor", "submit"]).expect("submit help should render");

    assert!(output.contains("rox-anchor submit"));
    assert!(output.contains("capped-testnet"));
    assert!(output.contains("no default send path"));
    assert!(output.contains("no RPC submission"));
    assert!(output.contains("no wallet/key loading"));
}

#[test]
fn nested_capped_testnet_submit_command_uses_existing_safe_report() {
    let output = run_from_args([
        "rox-anchor",
        "submit",
        "capped-testnet",
        "--authorize-testnet-submit-capped",
        "--receipt-persisted",
    ])
    .expect("nested capped-testnet command should render");

    assert!(output.contains("command: submit-capped"));
    assert!(output.contains("scope: testnet_only"));
    assert!(output.contains("capped_submit_status: Authorized"));
    assert!(output.contains("authorized: true"));
    assert!(output.contains("live_submission_permitted: true"));
    assert!(output.contains("live_submission_attempted: false"));
    assert!(output.contains("network_submitted: false"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("rpc_submission: disabled_in_cli_report"));
    assert!(output.contains("settlement_claim: none"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "transaction sent",
        "mint complete",
        "burn complete",
        "settlement complete",
        "access granted",
        "roc released",
        "public launch authorized",
    ] {
        assert!(
            !output.to_ascii_lowercase().contains(forbidden),
            "submit output must not contain unsafe phrase: {forbidden}\n{output}"
        );
    }
}

#[test]
fn unknown_submit_subcommand_fails_closed() {
    let error = run_from_args(["rox-anchor", "submit", "public-mainnet"]).unwrap_err();

    assert_eq!(
        error,
        CliError::UnknownSubmitFlag(
            "submit subcommand `public-mainnet`; expected capped-testnet".to_string()
        )
    );
}

#[test]
fn private_testnet_sender_runbook_is_explicit_capped_and_non_executing() {
    let doc = fs::read_to_string(
        repo_root().join("docs/pilot/EXPLICIT_CAPPED_PRIVATE_TESTNET_SENDER.md"),
    )
    .expect("phase 8 runbook should exist");

    for required in [
        "explicit capped private testnet sender",
        "external config",
        "operator approval",
        "receipt output path",
        "successful simulation",
        "read-only RPC verification",
        "no default send path",
        "no wallet loading",
        "no signing",
        "no internal ROC mutation",
    ] {
        assert!(
            doc.contains(required),
            "runbook missing phrase `{required}`"
        );
    }

    for forbidden in [
        "mainnet-beta",
        "public launch authorized",
        "mint complete",
        "burn complete",
        "settlement complete",
        "access granted",
        "roc released",
    ] {
        assert!(
            !doc.to_ascii_lowercase().contains(forbidden),
            "runbook must not contain unsafe phrase: {forbidden}"
        );
    }
}
