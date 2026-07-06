//! RO:WHAT — Tests the BUILD_PLAN3 Phase 16 private testnet pilot closeout gate.
//! RO:WHY — Keeps closeout behavior concrete, local, and non-authorizing.
//! RO:INTERACTS — docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md and scripts/check_private_testnet_pilot_closeout.sh.
//! RO:INVARIANTS — no public launch, mainnet, production settlement, ROC release, staking, liquidity, or exchange readiness claims.
//! RO:SECURITY — read-only test; no RPC, wallet load, signing, mint, burn, ROC mutation, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_testnet_pilot_closeout.

use std::{fs, path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

fn run_checker(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let script = root.join("scripts/check_private_testnet_pilot_closeout.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("closeout checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn closeout_checker_accepts_current_repo_safety_shape() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_checker(&[&root_arg]);

    assert!(ok, "closeout checker failed:\n{output}");
    assert!(output.contains("BUILD_PLAN3 Phase 16 private testnet pilot closeout checks passed"));
    assert!(output.contains("closeout completion remains conditional on local green commands"));
    assert!(
        output.contains("no public/mainnet/production/ROC-release/staking/liquidity/exchange authorization is present")
    );
    assert!(output.contains(
        "this script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, sign, or load a wallet"
    ));
}

#[test]
fn closeout_checklist_is_non_launching_and_operator_safe() {
    let (ok, output) = run_checker(&["--checklist"]);

    assert!(ok, "checklist failed:\n{output}");
    assert!(output.contains("ROX Anchor BUILD_PLAN3 Phase 16"));
    assert!(output.contains("cargo test --workspace"));
    assert!(output.contains("anchor build"));
    assert!(output.contains("anchor test"));
    assert!(output.contains("Final Clippy checkpoint"));
    assert!(output.contains("Forbidden by this closeout"));

    assert!(!output.contains("mainnet-beta deployment authorized"));
    assert!(!output.contains("production settlement authorized"));
    assert!(!output.contains("settlement complete"));
    assert!(!output.contains("mint complete"));
    assert!(!output.contains("production ready"));
}

#[test]
fn closeout_doc_is_conditional_and_non_authorizing() {
    let root = repo_root();
    let doc = fs::read_to_string(root.join("docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md"))
        .expect("closeout doc should be readable");

    assert!(doc.contains("ROX Anchor BUILD_PLAN3 Phase 16"));
    assert!(doc.contains("complete / green / parked only after required local commands pass"));
    assert!(doc.contains("This closeout gate does not authorize public launch."));
    assert!(doc.contains("This closeout gate does not authorize mainnet."));
    assert!(doc.contains("This closeout gate does not authorize production bridge settlement."));
    assert!(doc.contains("This closeout gate does not authorize real internal ROC release."));
    assert!(doc.contains("This closeout gate does not authorize exchange-facing behavior."));
    assert!(doc.contains("This closeout gate does not authorize staking."));
    assert!(doc.contains("This closeout gate does not authorize liquidity."));
    assert!(doc.contains(
        "Any future plan after this closeout must be a separate explicitly scoped plan."
    ));

    assert!(!doc.contains("public launch authorized"));
    assert!(!doc.contains("mainnet-beta deployment authorized"));
    assert!(!doc.contains("production settlement authorized"));
    assert!(!doc.contains("settlement complete"));
    assert!(!doc.contains("mint complete"));
    assert!(!doc.contains("production ready"));
}
