//! RO:WHAT — Tests BUILD_PLAN3 Phase 7 simulation-only pilot runbook boundaries.
//! RO:WHY — Keeps operator-facing simulation-only guidance explicit and non-submitting.
//! RO:INTERACTS — docs/pilot/SIMULATION_ONLY_PILOT_TRANSACTION_PLANS.md.
//! RO:INVARIANTS — docs require proof, coordinator, relayer, read-only RPC, and simulation-only gates.
//! RO:SECURITY — no live RPC, wallet, key loading, transaction send, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_simulation.

use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should resolve")
}

#[test]
fn simulation_only_pilot_runbook_requires_all_gates_and_no_send_path() {
    let doc = fs::read_to_string(
        repo_root().join("docs/pilot/SIMULATION_ONLY_PILOT_TRANSACTION_PLANS.md"),
    )
    .expect("simulation-only pilot runbook exists");

    for required in [
        "simulation-only",
        "private pilot",
        "accepted proof review",
        "accepted coordinator decision",
        "relayer dry-run acceptance",
        "read-only RPC verification",
        "no transaction submission",
        "no wallet loading",
        "no internal ROC mutation",
        "no live mint",
        "no live burn",
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
        "roc released",
    ] {
        assert!(
            !doc.to_ascii_lowercase().contains(forbidden),
            "runbook must not contain unsafe phrase: {forbidden}"
        );
    }
}
