# ROX Anchor Phase 14 — Audit Prep Index

No public launch authorization.

This index lists the Phase 14 audit-prep documents and the implementation surfaces they are tied to. It is not a launch checklist and does not authorize mainnet, public mint/burn, production bridge settlement, production ROC release, staking, liquidity, exchange-facing behavior, or public bridge access.

## Audit documents

| Document | Purpose |
| --- | --- |
| `INVARIANT_TEST_MAP.md` | Maps security invariants to real tests and scripts. |
| `AUTHORITY_MODEL.md` | Summarizes authority roles, separation rules, wrong-authority rejection, and readiness coverage. |
| `STATE_TRANSITIONS.md` | Summarizes local/proof/coordinator/relayer/program state transition rules. |
| `RPC_BOUNDARY.md` | Defines read-only RPC evidence trust boundaries and fail-closed cases. |
| `RELAYER_BOUNDARY.md` | Defines dry-run, simulation, and capped testnet submission boundaries. |
| `MINT_BURN_BOUNDARY.md` | Defines local/testnet-only ROC ↔ ROX mint/burn-shaped boundaries. |
| `HALT_RECOVERY_RUNBOOK.md` | Defines halt/recovery safety procedure and test coverage. |
| `KEY_ROTATION_RUNBOOK.md` | Defines testnet/localnet key rotation procedure and authority checks. |
| `TESTNET_DEPLOYMENT_RUNBOOK.md` | Defines deployment drill and rollback boundaries. |
| `KNOWN_NON_GOALS.md` | Defines explicit forbidden behavior and allowed testnet/localnet scope. |

## Checker coverage

The audit prep index is checked by:

```text
scripts/check_audit_prep.sh
crates/rox-anchor-cli/tests/audit_prep_docs.rs
```

## Required green commands

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
bash scripts/check_audit_prep.sh .
bash scripts/check_testnet_deploy_drill.sh .
```

If Anchor tooling is installed:

```bash
anchor build
anchor test
```

## Audit conclusion boundary

Successful Phase 14 means the repo is audit-prep ready for a self-directed or external security review. It does not mean public launch, mainnet readiness, production settlement, exchange readiness, staking readiness, liquidity readiness, or public bridge readiness.

## Index self-reference

This audit index file is:

```text
AUDIT_PREP_INDEX.md
```

The filename is intentionally recorded so audit-prep tests can verify that the index itself is included in the checked documentation set.

## Phase 15 readiness gate

The Phase 15 private testnet-readiness gate is documented in:

```text
TESTNET_READINESS_GATE.md
```

The checker for that gate is:

```text
scripts/check_testnet_readiness_gate.sh
```

This readiness gate is read-only and does not authorize public launch, mainnet, production settlement, public mint/burn, staking, liquidity, or exchange-facing behavior.
