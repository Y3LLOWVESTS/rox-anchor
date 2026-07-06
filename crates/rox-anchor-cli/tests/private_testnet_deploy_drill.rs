//! RO:WHAT — Tests BUILD_PLAN3 Phase 4 private testnet deployment drill checker.
//! RO:WHY — Keeps private deployment preparation test-covered without deploying or loading keys.
//! RO:INTERACTS — scripts/check_private_testnet_deploy.sh, Anchor.toml, .gitignore, pilot runbook.
//! RO:INVARIANTS — external keys only; no mainnet-beta; no committed private deploy outputs.
//! RO:SECURITY — invokes a read-only local script only; no RPC, wallet, deploy, submit, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_testnet_deploy_drill.

use std::{path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let script = root.join("scripts/check_private_testnet_deploy.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("private deployment drill script should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn private_testnet_deploy_drill_accepts_current_repo_safety_shape() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&root_arg]);

    assert!(ok, "script failed:\n{output}");
    assert!(output.contains("BUILD_PLAN3 Phase 4 private testnet deployment drill checks passed"));
    assert!(output.contains("Anchor program bindings are devnet/testnet scoped"));
    assert!(output.contains("mainnet-beta is rejected by local inspection"));
    assert!(output.contains("external deploy keypair path is required by checklist"));
    assert!(output.contains("external payer path is required by checklist"));
    assert!(output.contains("external upgrade authority path is required by checklist"));
    assert!(output.contains("deployment output remains local/ignored/redacted"));
    assert!(output.contains(
        "this script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, or load a wallet"
    ));

    assert!(!output.contains("deployment_success: true"));
    assert!(!output.contains("public_launch_authorized: true"));
    assert!(!output.contains("settlement complete"));
    assert!(!output.contains("mint complete"));
}

#[test]
fn private_testnet_deploy_checklist_is_external_key_only_and_non_launching() {
    let (ok, output) = run_script(&["--checklist"]);

    assert!(ok, "checklist failed:\n{output}");
    assert!(output.contains("cargo test --workspace"));
    assert!(output.contains("anchor build"));
    assert!(output.contains("anchor test"));
    assert!(output.contains("ROX_ANCHOR_PRIVATE_TESTNET_PAYER"));
    assert!(output.contains("ROX_ANCHOR_PRIVATE_TESTNET_PROGRAM_KEYPAIR"));
    assert!(output.contains("ROX_ANCHOR_PRIVATE_TESTNET_UPGRADE_AUTHORITY"));
    assert!(output.contains("anchor deploy --provider.cluster testnet"));
    assert!(output.contains("Forbidden in this drill"));

    assert!(!output.contains("--provider.cluster mainnet-beta"));
    assert!(!output.contains("public launch authorized"));
    assert!(!output.contains("settlement complete"));
    assert!(!output.contains("mint complete"));
}
