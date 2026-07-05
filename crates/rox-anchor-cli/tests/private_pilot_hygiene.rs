//! RO:WHAT — Tests the BUILD_PLAN3 Phase 1 private pilot hygiene checker.
//! RO:WHY — Keeps operator keys, RPC URLs, receipts, and pilot outputs out of tracked source.
//! RO:INTERACTS — scripts/check_private_pilot_hygiene.sh, .gitignore, docs/pilot.
//! RO:INVARIANTS — no tracked keypairs, no raw RPC/provider tokens, and no fake launch/success wording.
//! RO:SECURITY — invokes a read-only local checker; no RPC, wallet load, deployment, submission, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_hygiene.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let script = root.join("scripts/check_private_pilot_hygiene.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("private pilot hygiene checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

fn unique_temp_root(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after UNIX_EPOCH")
        .as_nanos();

    let root = std::env::temp_dir().join(format!("rox-anchor-{label}-{nanos}"));
    fs::create_dir_all(root.join("docs/pilot")).expect("temp docs dir should be created");
    root
}

fn write_minimal_safe_shape(root: &Path) {
    fs::write(
        root.join(".gitignore"),
        concat!(
            ".rox-anchor-pilot/\n",
            "pilot-rpc/\n",
            "pilot-keys/\n",
            "pilot-receipts/\n",
            "pilot-deploy/\n",
            "pilot-artifacts/\n",
            "pilot-ledger/\n",
            "*.pilot-keypair.json\n",
            "*.pilot-rpc.txt\n",
            "*.pilot-receipt.json\n",
        ),
    )
    .expect("temp .gitignore should be written");

    fs::write(
        root.join("docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"),
        concat!(
            "# ROX Anchor BUILD_PLAN3 Phase 1\n",
            "No public launch authorization.\n",
            "No mainnet-beta deployment.\n",
            "No real internal ROC release.\n",
            "scripts/check_private_pilot_hygiene.sh\n",
            "local-only / ignored / external\n",
        ),
    )
    .expect("temp pilot doc should be written");
}

#[test]
fn private_pilot_hygiene_accepts_current_repo_safety_shape() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&root_arg]);

    assert!(ok, "private pilot hygiene checker failed:\n{output}");
    assert!(output.contains("BUILD_PLAN3 Phase 1 private pilot hygiene checks passed"));
    assert!(output.contains("private pilot local workspace layout is documented"));
    assert!(output.contains("no raw RPC/provider token URLs found in source paths"));
    assert!(output.contains(
        "this script did not deploy, submit, mint, burn, settle, call RPC, or load a wallet"
    ));
}

#[test]
fn private_pilot_hygiene_checklist_is_non_launching_and_operator_safe() {
    let (ok, output) = run_script(&["--checklist"]);

    assert!(ok, "checklist failed:\n{output}");
    assert!(output.contains("ROX Anchor BUILD_PLAN3 Phase 1"));
    assert!(output.contains(".rox-anchor-pilot/"));
    assert!(output.contains("explicit operator approval"));
    assert!(output.contains("Forbidden from this hygiene phase"));
    assert!(!output.contains("mainnet-beta deployment authorized"));
    assert!(!output.contains("production settlement authorized"));
    assert!(!output.contains("mint complete"));
    assert!(!output.contains("settlement complete"));
}

#[test]
fn private_pilot_hygiene_rejects_key_shaped_files_in_source_paths() {
    let temp = unique_temp_root("key-shaped");
    write_minimal_safe_shape(&temp);
    fs::write(temp.join("operator.pilot-keypair.json"), "[1,2,3]")
        .expect("temp forbidden file should be written");

    let temp_arg = temp.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&temp_arg]);
    let _ = fs::remove_dir_all(&temp);

    assert!(!ok, "checker should reject pilot key material:\n{output}");
    assert!(output.contains("operator.pilot-keypair.json"));
    assert!(output.contains("remove, relocate, or ignore private pilot key/RPC material"));
}

#[test]
fn private_pilot_hygiene_rejects_raw_rpc_provider_token_urls() {
    let temp = unique_temp_root("rpc-token");
    write_minimal_safe_shape(&temp);

    // Keep the tracked Rust source itself clean while still writing a raw
    // tokenized URL into the temporary fixture at runtime.
    let raw_provider_url = [
        "pilot endpoint: ",
        "https://pilot-rpc.invalid/rpc?",
        "api",
        "-",
        "key",
        "=",
        "abcdef1234567890",
        "\n",
    ]
    .concat();

    fs::write(temp.join("notes.md"), raw_provider_url)
        .expect("temp forbidden token URL file should be written");

    let temp_arg = temp.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&temp_arg]);
    let _ = fs::remove_dir_all(&temp);

    assert!(!ok, "checker should reject raw RPC token URLs:\n{output}");
    assert!(output.contains("notes.md"));
    assert!(output.contains("redact RPC/provider URLs before committing"));
}
