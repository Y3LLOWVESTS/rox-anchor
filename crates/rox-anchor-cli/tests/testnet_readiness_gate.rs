//! RO:WHAT — Tests the Phase 15 private testnet readiness gate checker.
//! RO:WHY — Keeps the final readiness gate tied to real scripts, docs, tests, and non-launch boundaries.
//! RO:INTERACTS — docs/audit/TESTNET_READINESS_GATE.md and scripts/check_testnet_readiness_gate.sh.
//! RO:INVARIANTS — readiness is private testnet-only; no public launch, mainnet, settlement, or wallet side effects.
//! RO:SECURITY — local read-only test; no RPC, wallet load, deployment, submission, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test testnet_readiness_gate.

use std::{fs, path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let script = root.join("scripts/check_testnet_readiness_gate.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("testnet readiness checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn readiness_gate_accepts_current_repo_safety_shape() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&root_arg]);

    assert!(ok, "testnet readiness gate failed:\n{output}");
    assert!(output.contains("Phase 15 testnet readiness gate checks passed"));
    assert!(output.contains("private testnet-only pilot review surface is present"));
    assert!(output.contains("docs/audit/TESTNET_READINESS_GATE.md"));
    assert!(output.contains("scripts/check_testnet_readiness_gate.sh"));
    assert!(output.contains("Phase 14 audit prep checker remains green"));
    assert!(output.contains("testnet deployment drill checker remains green"));
    assert!(output.contains(
        "this script did not deploy, submit, mint, burn, settle, call RPC, or load a wallet"
    ));
}

#[test]
fn readiness_doc_is_tied_to_real_surfaces_and_non_launch_boundaries() {
    let root = repo_root();

    let docs = [
        "docs/audit/TESTNET_READINESS_GATE.md",
        "docs/audit/AUDIT_PREP_INDEX.md",
        "docs/audit/KNOWN_NON_GOALS.md",
    ];

    let mut combined = String::new();
    for doc in docs {
        let path = root.join(doc);
        let body =
            fs::read_to_string(&path).unwrap_or_else(|err| panic!("{doc} should read: {err}"));
        combined.push_str(&body);
        combined.push('\n');
    }

    for required in [
        "cargo test --workspace",
        "cargo check --workspace",
        "scripts/check_audit_prep.sh",
        "scripts/check_testnet_deploy_drill.sh",
        "scripts/check_testnet_readiness_gate.sh",
        "crates/rox-anchor-core/tests/testnet_scope_locks.rs",
        "crates/rox-anchor-relayer/tests/capped_testnet_submission.rs",
        "crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs",
        "programs/rox-anchor/src/state.rs",
        "TESTNET_READINESS_GATE.md",
    ] {
        assert!(
            combined.contains(required),
            "readiness docs should reference required surface: {required}"
        );
    }

    let lower = combined.to_lowercase();
    for forbidden in [
        "public launch authorized",
        "mainnet launch authorized",
        "mainnet-beta authorized",
        "production bridge authorized",
        "production settlement authorized",
        "exchange ready",
        "staking ready",
        "liquidity ready",
        "fake finality allowed",
    ] {
        assert!(
            !lower.contains(forbidden),
            "readiness docs must not contain authorization wording: {forbidden}"
        );
    }
}
