//! RO:WHAT — Tests Phase 14 audit-prep docs and checker.
//! RO:WHY — Keeps audit maps/runbooks tied to real tests instead of loose documentation.
//! RO:INTERACTS — docs/audit and scripts/check_audit_prep.sh.
//! RO:INVARIANTS — audit docs do not authorize public launch, mainnet, production settlement, or fake finality.
//! RO:SECURITY — local read-only test; no RPC, wallet, deployment, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test audit_prep_docs.

use std::{fs, path::PathBuf, process::Command};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve from crate manifest dir")
}

fn run_script(args: &[&str]) -> (bool, String) {
    let root = repo_root();
    let script = root.join("scripts/check_audit_prep.sh");

    let output = Command::new("bash")
        .arg(script)
        .args(args)
        .current_dir(&root)
        .output()
        .expect("audit prep checker should execute");

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    (output.status.success(), combined)
}

#[test]
fn audit_prep_checker_accepts_current_docs() {
    let root = repo_root();
    let root_arg = root.to_string_lossy().to_string();
    let (ok, output) = run_script(&[&root_arg]);

    assert!(ok, "audit prep checker failed:\n{output}");
    assert!(output.contains("Phase 14 audit prep checks passed"));
    assert!(output.contains("docs/audit/INVARIANT_TEST_MAP.md"));
    assert!(output.contains("docs/audit/KNOWN_NON_GOALS.md"));
    assert!(output.contains("docs/audit/KEY_ROTATION_RUNBOOK.md"));
    assert!(output.contains("docs/audit/AUDIT_PREP_INDEX.md"));
    assert!(
        output.contains("this script did not deploy, submit, mint, burn, settle, or load a wallet")
    );
}

#[test]
fn audit_docs_reference_real_test_surfaces_and_non_goals() {
    let root = repo_root();
    let docs = [
        "docs/audit/INVARIANT_TEST_MAP.md",
        "docs/audit/AUTHORITY_MODEL.md",
        "docs/audit/STATE_TRANSITIONS.md",
        "docs/audit/RPC_BOUNDARY.md",
        "docs/audit/RELAYER_BOUNDARY.md",
        "docs/audit/MINT_BURN_BOUNDARY.md",
        "docs/audit/HALT_RECOVERY_RUNBOOK.md",
        "docs/audit/KEY_ROTATION_RUNBOOK.md",
        "docs/audit/TESTNET_DEPLOYMENT_RUNBOOK.md",
        "docs/audit/KNOWN_NON_GOALS.md",
        "docs/audit/AUDIT_PREP_INDEX.md",
    ];

    let mut all = String::new();
    for doc in docs {
        let body = fs::read_to_string(root.join(doc)).unwrap_or_else(|err| {
            panic!("expected audit doc {doc} to be readable: {err}");
        });

        assert!(body.contains("ROX Anchor Phase 14"));
        assert!(body.contains("No public launch authorization."));
        all.push_str(&body);
        all.push('\n');
    }

    for required in [
        "crates/rox-anchor-core/tests/operator_authority_model.rs",
        "crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs",
        "crates/rox-anchor-relayer/tests/capped_testnet_submission.rs",
        "crates/rox-anchor-cli/tests/kill_switch_drill_command.rs",
        "scripts/check_testnet_deploy_drill.sh",
        "programs/rox-anchor/src/state.rs",
        "authority_rotation_intent_rejects_noop_and_requires_activation_slot",
        "KEY_ROTATION_RUNBOOK.md",
        "AUDIT_PREP_INDEX.md",
        "mainnet-beta deployment",
        "fake finality",
    ] {
        assert!(
            all.contains(required),
            "audit docs should reference required surface: {required}"
        );
    }

    for forbidden in [
        "public launch authorized",
        "mainnet launch authorized",
        "production bridge authorized",
        "production settlement authorized",
        "exchange ready",
        "staking ready",
        "liquidity ready",
        "fake finality allowed",
    ] {
        assert!(
            !all.to_lowercase().contains(forbidden),
            "audit docs must not contain authorization wording: {forbidden}"
        );
    }
}
