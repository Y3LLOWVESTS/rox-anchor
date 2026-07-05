//! RO:WHAT — Tests the Phase 7 testnet deployment drill safety script.
//! RO:WHY — Keeps deployment drill checks covered by cargo test without deploying or loading keys.
//! RO:INTERACTS — scripts/check_testnet_deploy_drill.sh, Anchor.toml, .gitignore.
//! RO:INVARIANTS — no mainnet-beta, no committed keypairs, no fake deploy success wording.
//! RO:SECURITY — invokes read-only local script only; no RPC, wallet, deployment, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test testnet_deploy_drill_script.

use std::{path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let script = root.join("scripts/check_testnet_deploy_drill.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("deployment drill script should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn deploy_drill_script_accepts_current_repo_safety_shape() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&root_arg]);

    assert!(ok, "script failed:\n{output}");
    assert!(output.contains("Phase 7 testnet deployment drill safety checks passed"));
    assert!(output.contains("provider cluster is not mainnet-beta"));
    assert!(
        output.contains("this script did not deploy, submit, mint, burn, settle, or load a wallet")
    );
}

#[test]
fn deploy_drill_checklist_is_testnet_only_and_non_success_claiming() {
    let (ok, output) = run_script(&["--checklist"]);

    assert!(ok, "checklist failed:\n{output}");
    assert!(output.contains("anchor build"));
    assert!(output.contains("anchor test"));
    assert!(output.contains("anchor deploy --provider.cluster testnet"));
    assert!(output.contains("Forbidden in this drill"));
    assert!(!output.contains("--provider.cluster mainnet-beta"));
    assert!(!output.contains("settlement complete"));
    assert!(!output.contains("mint complete"));
    assert!(!output.contains("production ready"));
}
