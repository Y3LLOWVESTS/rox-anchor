//! RO:WHAT — Tests BUILD_PLAN4 Phase 2 actual Anchor build artifact manifest capture.
//! RO:WHY — Proves program/IDL hashes can be captured as build evidence without deployment or finality claims.
//! RO:INTERACTS — scripts/capture_actual_private_testnet_build_artifacts.sh, Anchor.toml, target/deploy, target/idl.
//! RO:INVARIANTS — devnet/testnet only; redacted paths; build manifest is not deployment proof.
//! RO:SECURITY — no RPC, wallet load, signing, deployment, submission, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-core --test actual_testnet_artifact_manifest.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn script_path() -> PathBuf {
    repo_root().join("scripts/capture_actual_private_testnet_build_artifacts.sh")
}

fn run_script(args: &[String]) -> (bool, String) {
    let output = Command::new("bash")
        .arg(script_path())
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("build artifact capture script should execute");

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

    let root = std::env::temp_dir().join(format!("rox-anchor-artifact-manifest-{label}-{nanos}"));
    fs::create_dir_all(root.join("target/deploy")).expect("temp deploy dir should be created");
    fs::create_dir_all(root.join("target/idl")).expect("temp idl dir should be created");
    root
}

fn write_fake_anchor_build(root: &Path) {
    fs::write(
        root.join("Anchor.toml"),
        format!(
            r#"[programs.devnet]
rox_anchor = "{PROGRAM_ID}"

[programs.testnet]
rox_anchor = "{PROGRAM_ID}"
"#
        ),
    )
    .expect("temp Anchor.toml should be written");

    fs::write(
        root.join("target/deploy/rox_anchor.so"),
        b"fake-but-deterministic-program-binary-for-build-manifest-test\n",
    )
    .expect("fake program binary should be written");

    fs::write(
        root.join("target/idl/rox_anchor.json"),
        br#"{"version":"0.1.0","name":"rox_anchor","instructions":[]}"#,
    )
    .expect("fake IDL should be written");
}

#[test]
fn actual_build_artifact_manifest_captures_hashes_and_redacts_paths() {
    let temp = unique_temp_root("capture");
    write_fake_anchor_build(&temp);

    let output_path = temp.join("actual-private-testnet-build-artifacts.local.json");

    let args = vec![
        "--capture".to_owned(),
        temp.to_string_lossy().to_string(),
        output_path.to_string_lossy().to_string(),
        "devnet".to_owned(),
    ];

    let (ok, output) = run_script(&args);
    assert!(ok, "capture should pass:\n{output}");

    let manifest =
        fs::read_to_string(&output_path).expect("capture should write a redacted manifest");

    let _ = fs::remove_dir_all(&temp);

    assert!(
        manifest.contains(r#""schema": "rox-anchor.actual-private-testnet-build-artifacts.v1""#)
    );
    assert!(manifest.contains(r#""phase": "BUILD_PLAN4 Phase 2""#));
    assert!(manifest.contains(r#""artifact_role": "anchor_build_metadata_only""#));
    assert!(manifest.contains(r#""cluster": "devnet""#));
    assert!(manifest.contains(r#""program_name": "rox_anchor""#));
    assert!(manifest.contains(&format!(r#""program_id": "{PROGRAM_ID}""#)));
    assert!(manifest.contains(r#""expected_program_id_source": "Anchor.toml [programs.devnet]""#));
    assert!(manifest.contains(r#""program_binary_sha256": ""#));
    assert!(manifest.contains(r#""idl_sha256": ""#));
    assert!(manifest.contains(r#""program_binary_size_bytes": "#));
    assert!(manifest.contains(r#""idl_size_bytes": "#));
    assert!(manifest
        .contains(r#""program_artifact_path": "<redacted-anchor-build-path>/rox_anchor.so""#));
    assert!(
        manifest.contains(r#""idl_artifact_path": "<redacted-anchor-build-path>/rox_anchor.json""#)
    );
    assert!(manifest.contains(r#""build_manifest_is_deployment_proof": false"#));
    assert!(manifest.contains(r#""deployment_claim": false"#));
    assert!(manifest.contains(r#""finality_claim": false"#));
    assert!(manifest.contains(r#""runtime_authority": false"#));
    assert!(manifest.contains(r#""public_launch_authorized": false"#));
    assert!(manifest.contains(r#""mainnet_authorized": false"#));
    assert!(manifest.contains(r#""real_roc_mutation": false"#));

    assert!(!manifest.contains(temp.to_string_lossy().as_ref()));
    assert!(!manifest.contains("/Users/"));
    assert!(!manifest.contains("/home/"));
    assert!(!manifest.contains("deployment_success"));
    assert!(!manifest.contains(r#""build_manifest_is_deployment_proof": true"#));
    assert!(!manifest.contains(r#""finality_claim": true"#));
}

#[test]
fn actual_build_artifact_capture_can_print_to_stdout_without_writing_source_artifacts() {
    let temp = unique_temp_root("stdout");
    write_fake_anchor_build(&temp);

    let args = vec![
        "--capture".to_owned(),
        temp.to_string_lossy().to_string(),
        "-".to_owned(),
        "testnet".to_owned(),
    ];

    let (ok, output) = run_script(&args);
    let _ = fs::remove_dir_all(&temp);

    assert!(ok, "stdout capture should pass:\n{output}");
    assert!(output.contains(r#""cluster": "testnet""#));
    assert!(output.contains(r#""expected_program_id_source": "Anchor.toml [programs.testnet]""#));
    assert!(output.contains(r#""build_manifest_is_deployment_proof": false"#));
    assert!(output.contains("manifest is not deployment proof"));
}

#[test]
fn actual_build_artifact_capture_rejects_mainnet_and_localnet_clusters() {
    let temp = unique_temp_root("bad-cluster");
    write_fake_anchor_build(&temp);

    for cluster in ["mainnet-beta", "mainnet", "localnet"] {
        let args = vec![
            "--capture".to_owned(),
            temp.to_string_lossy().to_string(),
            "-".to_owned(),
            cluster.to_owned(),
        ];

        let (ok, output) = run_script(&args);
        assert!(
            !ok,
            "capture should reject forbidden cluster {cluster}:\n{output}"
        );
        assert!(output.contains("cluster must be devnet or testnet"));
    }

    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn actual_build_artifact_capture_rejects_missing_anchor_build_outputs() {
    let temp = unique_temp_root("missing-output");

    fs::write(
        temp.join("Anchor.toml"),
        format!(
            r#"[programs.devnet]
rox_anchor = "{PROGRAM_ID}"
"#
        ),
    )
    .expect("temp Anchor.toml should be written");

    let args = vec![
        "--capture".to_owned(),
        temp.to_string_lossy().to_string(),
        "-".to_owned(),
        "devnet".to_owned(),
    ];

    let (ok, output) = run_script(&args);
    let _ = fs::remove_dir_all(&temp);

    assert!(!ok, "capture should reject missing build output:\n{output}");
    assert!(output.contains("program binary missing"));
    assert!(output.contains("run anchor build first"));
}

#[test]
fn actual_build_artifact_docs_checker_accepts_current_repo_boundaries() {
    let args = vec![
        "--check-docs".to_owned(),
        repo_root().to_string_lossy().to_string(),
    ];
    let (ok, output) = run_script(&args);

    assert!(ok, "doc checker should pass:\n{output}");
    assert!(output.contains("BUILD_PLAN4 Phase 2 build artifact documentation checks passed"));
    assert!(output.contains("build-only/non-deployment/non-finality boundaries"));
}
