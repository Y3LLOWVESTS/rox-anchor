//! RO:WHAT — Tests BUILD_PLAN3 Phase 6 pilot read-only RPC docs and CLI boundary.
//! RO:WHY — Keeps private testnet readback operator-facing material explicit, read-only, and non-submitting.
//! RO:INTERACTS — docs/pilot/PRIVATE_TESTNET_READ_ONLY_RPC.md and rox-anchor proof command.
//! RO:INVARIANTS — readback is explicit, testnet/private-pilot only, and never claims send/finality/settlement.
//! RO:SECURITY — no live RPC, key loading, wallet, transaction, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_testnet_read_only_rpc.

use std::{fs, path::PathBuf};

use rox_anchor_cli::run_from_args;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

#[test]
fn private_testnet_read_only_rpc_runbook_is_read_only_and_non_submitting() {
    let doc_path = repo_root().join("docs/pilot/PRIVATE_TESTNET_READ_ONLY_RPC.md");
    let doc = fs::read_to_string(&doc_path).expect("private read-only RPC runbook exists");

    for required in [
        "read-only RPC",
        "private testnet",
        "external config",
        "program account status",
        "config account status",
        "mint account status",
        "token account status",
        "signature status",
        "no transaction submission",
        "no wallet loading",
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
    ] {
        assert!(
            !doc.to_ascii_lowercase().contains(forbidden),
            "runbook must not contain unsafe wording: {forbidden}"
        );
    }
}

#[test]
fn proof_command_remains_read_only_after_private_testnet_readback_patch() {
    let output = run_from_args(["rox-anchor", "proof"]).expect("proof command should run");
    let lowered = output.to_ascii_lowercase();

    assert!(output.contains("rox-anchor proof"));
    assert!(output.contains("status: read_only_rpc_adapter_shape"));
    assert!(output.contains("submission: disabled"));
    assert!(output.contains("wallet_key_loading: disabled"));
    assert!(output.contains("network_client: not_enabled"));
    assert!(output.contains("quorum_decision: Agreement"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "mint complete",
        "burn complete",
        "settlement complete",
        "access granted",
        "roc released",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "proof command must remain display-safe: {forbidden}"
        );
    }
}
