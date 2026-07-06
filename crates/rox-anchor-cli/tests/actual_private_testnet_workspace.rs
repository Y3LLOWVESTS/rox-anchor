//! RO:WHAT — Tests BUILD_PLAN4 Phase 1 actual private testnet workspace checker.
//! RO:WHY — Keeps real private testnet operator config/key/RPC/receipt artifacts external, ignored, and redacted.
//! RO:INTERACTS — scripts/check_actual_private_testnet_workspace.sh, .gitignore, docs/pilot.
//! RO:INVARIANTS — no tracked keypairs, no tokenized RPC URLs, no unredacted local paths, and no launch/finality claims.
//! RO:SECURITY — invokes a read-only local checker; no RPC, wallet load, deployment, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_workspace.

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
    let script = root.join("scripts/check_actual_private_testnet_workspace.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("actual private testnet workspace checker should execute");

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

    let root = std::env::temp_dir().join(format!("rox-anchor-actual-private-{label}-{nanos}"));
    fs::create_dir_all(root.join("docs/pilot")).expect("temp docs dir should be created");
    root
}

fn write_minimal_safe_shape(root: &Path) {
    fs::write(
        root.join(".gitignore"),
        concat!(
            ".rox-anchor-pilot/\n",
            ".rox-anchor-private-pilot/\n",
            "private-pilot/\n",
            "pilot-artifacts/\n",
            "pilot-rpc/\n",
            "pilot-keys/\n",
            "pilot-keypairs/\n",
            "pilot-wallets/\n",
            "pilot-secrets/\n",
            "pilot-receipts/\n",
            "pilot-audit/\n",
            "pilot-deploy/\n",
            "pilot-ledger/\n",
            "pilot-tmp/\n",
            "*.pilot-config.local.toml\n",
            "*.pilot-config.local.json\n",
            "*.pilot-rpc.txt\n",
            "*.pilot-provider.txt\n",
            "*.pilot-keypair.json\n",
            "*.pilot-wallet.json\n",
            "*.pilot-authority.json\n",
            "*.pilot-payer.json\n",
            "*.pilot-receipt.json\n",
            "*.pilot-audit.json\n",
            "*.pilot-deploy-output.json\n",
            "*.pilot-ledger.json\n",
            "private-testnet.toml\n",
            "actual-private-testnet.toml\n",
            "*.private-testnet.local.toml\n",
        ),
    )
    .expect("temp .gitignore should be written");

    fs::write(
        root.join("docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"),
        concat!(
            "# ROX Anchor BUILD_PLAN4 Phase 1\n",
            "external-only / ignored / redacted\n",
            "<external-private-workspace>\n",
            "private-testnet.toml\n",
            "ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_CONFIG\n",
            "No public launch authorization.\n",
            "No mainnet-beta deployment.\n",
            "No real internal ROC release.\n",
            "scripts/check_actual_private_testnet_workspace.sh\n",
        ),
    )
    .expect("temp actual private testnet doc should be written");
}

#[test]
fn actual_private_testnet_workspace_accepts_current_repo_safety_shape() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&root_arg]);

    assert!(
        ok,
        "actual private testnet workspace checker failed:\n{output}"
    );
    assert!(output.contains("BUILD_PLAN4 Phase 1 actual private testnet workspace checks passed"));
    assert!(output.contains("external operator workspace shape is documented"));
    assert!(output.contains(
        ".gitignore covers actual private testnet config, key, RPC, receipt, deploy, audit, ledger, and tmp artifacts"
    ));
    assert!(output.contains("source paths contain no key-shaped actual private testnet files"));
    assert!(output.contains("source paths contain no raw tokenized RPC/provider URLs"));
    assert!(output.contains(
        "this script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, sign, or load a wallet"
    ));
}

#[test]
fn actual_private_testnet_checklist_is_external_only_and_non_launching() {
    let (ok, output) = run_script(&["--checklist"]);

    assert!(ok, "checklist failed:\n{output}");
    assert!(output.contains("ROX Anchor BUILD_PLAN4 Phase 1"));
    assert!(output.contains("cargo check --workspace"));
    assert!(output.contains("cargo test --workspace"));
    assert!(output.contains("<external-private-workspace>"));
    assert!(output.contains("private-testnet.toml"));
    assert!(output.contains("Forbidden in Phase 1"));

    assert!(!output.contains("mainnet-beta deployment authorized"));
    assert!(!output.contains("public launch authorized"));
    assert!(!output.contains("settlement complete"));
    assert!(!output.contains("mint complete"));
}

#[test]
fn actual_private_testnet_template_is_redacted_and_capped() {
    let (ok, output) = run_script(&["--template"]);

    assert!(ok, "template failed:\n{output}");
    assert!(output.contains("<external-private-workspace>/private-testnet.toml"));
    assert!(output.contains("cluster = \"testnet\""));
    assert!(output.contains("require_operator_approval = true"));
    assert!(output.contains("max_operation_count = 1"));
    assert!(output.contains("max_retry_count = 0"));
    assert!(output.contains("max_test_only_amount_minor"));

    assert!(!output.contains("/Users/"));
    assert!(!output.contains("/home/"));
    assert!(!output.contains("api-key="));
    assert!(!output.contains("apikey="));
    assert!(!output.contains("access_token="));
}

#[test]
fn actual_private_testnet_workspace_rejects_local_config_in_source_paths() {
    let temp = unique_temp_root("local-config");
    write_minimal_safe_shape(&temp);

    fs::write(temp.join("private-testnet.toml"), "cluster = \"testnet\"\n")
        .expect("temp forbidden local config should be written");

    let temp_arg = temp.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&temp_arg]);
    let _ = fs::remove_dir_all(&temp);

    assert!(
        !ok,
        "checker should reject actual private testnet config in source paths:\n{output}"
    );
    assert!(output.contains("private-testnet.toml"));
    assert!(output.contains("remove, relocate, or ignore actual private testnet operator material"));
}

#[test]
fn actual_private_testnet_workspace_allows_ignored_local_workspace() {
    let temp = unique_temp_root("ignored-workspace");
    write_minimal_safe_shape(&temp);

    let ignored = temp.join(".rox-anchor-private-pilot");
    fs::create_dir_all(ignored.join("rpc")).expect("ignored workspace should be created");
    fs::write(
        ignored.join("private-testnet.toml"),
        "cluster = \"testnet\"\n",
    )
    .expect("ignored private config should be written");

    let raw_provider_url = [
        "https://pilot-rpc.invalid/rpc?",
        "api",
        "-",
        "key",
        "=",
        "abcdef1234567890",
        "\n",
    ]
    .concat();

    fs::write(ignored.join("rpc/provider.pilot-rpc.txt"), raw_provider_url)
        .expect("ignored RPC token file should be written");

    let temp_arg = temp.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&temp_arg]);
    let _ = fs::remove_dir_all(&temp);

    assert!(
        ok,
        "checker should allow ignored local-only workspace artifacts:\n{output}"
    );
    assert!(output.contains("BUILD_PLAN4 Phase 1 actual private testnet workspace checks passed"));
}

#[test]
fn actual_private_testnet_workspace_rejects_raw_rpc_provider_token_urls() {
    let temp = unique_temp_root("rpc-token");
    write_minimal_safe_shape(&temp);

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
