//! RO:WHAT — Tests BUILD_PLAN3 Phase 5 pilot runbook and display boundary.
//! RO:WHY — Keeps test-only mint initialization operator-facing docs tied to safe behavior.
//! RO:INTERACTS — docs/pilot/TEST_ONLY_MINT_INITIALIZATION.md and core initialization intent review.
//! RO:INVARIANTS — explicit testnet-only, tiny caps, separated authorities, no public labels, no live behavior.
//! RO:SECURITY — no wallet, RPC, deploy, live mint initialization, mint, burn, settlement, or internal ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test test_only_mint_initialization.

use std::{fs, path::PathBuf};

use rox_anchor_core::{
    AuthorityAssignment, AuthorityKeyId, AuthorityMap, AuthoritySeparationMode, OperatorRole,
    TestOnlyMintInitializationIntent,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn key(label: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(label).expect("authority key id should validate")
}

fn separated_authorities() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000001")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000002")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000003")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000004"),
            ),
        ],
    )
}

#[test]
fn test_only_mint_initialization_runbook_is_private_testnet_only() {
    let doc_path = repo_root().join("docs/pilot/TEST_ONLY_MINT_INITIALIZATION.md");
    let doc = fs::read_to_string(&doc_path).expect("test-only mint initialization runbook exists");

    for required in [
        "explicit testnet mode",
        "explicit test-only mint label",
        "tiny supply cap",
        "mint authority separation",
        "halt authority",
        "recovery authority",
        "not a launch",
        "no real internal ROC mutation",
        "no live mint initialization",
        "No command in this runbook may load a wallet",
    ] {
        assert!(
            doc.contains(required),
            "runbook missing required phrase `{required}`"
        );
    }

    assert!(!doc.contains("public launch authorized"));
    assert!(!doc.contains("mainnet-beta"));
    assert!(!doc.contains("mint complete"));
    assert!(!doc.contains("settlement complete"));
}

#[test]
fn test_only_mint_initialization_report_is_display_safe() {
    let intent =
        TestOnlyMintInitializationIntent::devnet_fixture_with_authorities(separated_authorities());
    let report = intent.redacted_report_lines().join("\n");

    assert!(report.contains("ready: true"));
    assert!(report.contains("live_mint_initialization: disabled"));
    assert!(report.contains("wallet_loading: disabled"));
    assert!(report.contains("rpc_calls: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));

    for forbidden in [
        "public_launch_authorized: true",
        "mainnet",
        "mint complete",
        "burn complete",
        "settlement complete",
        "wallet loaded",
        "rpc submitted",
        "private-key",
        "seed phrase",
    ] {
        assert!(
            !report.to_ascii_lowercase().contains(forbidden),
            "report must not contain unsafe wording: {forbidden}\n{report}"
        );
    }
}
