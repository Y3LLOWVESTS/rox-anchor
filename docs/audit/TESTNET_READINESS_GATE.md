# ROX Anchor Phase 15 — Testnet Readiness Gate

Public launch remains unauthorized.

Mainnet remains unauthorized.

Production bridge settlement remains unauthorized.

This gate decides whether the current ROX Anchor repo is ready for a private testnet-only pilot. It does not deploy, submit, mint, burn, settle, load a wallet, expose a public bridge, enable exchange-facing behavior, enable liquidity behavior, or enable staking behavior.

## Gate purpose

Phase 15 confirms that the completed local/testnet hardening work is tied to real tests, scripts, and runbooks before any private testnet pilot is considered.

The gate is read-only. It checks repo shape, safety docs, local tests, scripts, and explicit non-goals.

## Required proof surfaces

| Requirement | Required surface |
| --- | --- |
| Local tests pass. | `cargo test --workspace` |
| Workspace compiles. | `cargo check --workspace` |
| Testnet-only guards exist. | `crates/rox-anchor-core/tests/testnet_scope_locks.rs` |
| Mainnet-beta is rejected. | `mainnet_beta_cluster_is_rejected_before_config_can_use_it` |
| Public launch flags do not exist. | `public_launch_flags_are_not_available_modes` |
| Testnet config is explicit and redacted. | `crates/rox-anchor-core/tests/testnet_config_model.rs` |
| Operator authority safety is tested. | `crates/rox-anchor-core/tests/operator_authority_model.rs` |
| Key rotation has a runbook. | `docs/audit/KEY_ROTATION_RUNBOOK.md` |
| Audit maps and runbooks are tied to tests. | `scripts/check_audit_prep.sh` |
| Deployment drill is non-deploying and key-safe. | `scripts/check_testnet_deploy_drill.sh` |
| Capped submit is explicit and gated. | `crates/rox-anchor-relayer/tests/capped_testnet_submission.rs` |
| End-to-end shadow flow remains non-production. | `crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs` |
| Halt/recovery drills are tested. | `crates/rox-anchor-cli/tests/kill_switch_drill_command.rs` |
| Testnet chaos drills fail safely. | `crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs` and `crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs` |
| Anchor state protects authority and token settlement. | `programs/rox-anchor/src/state.rs` |

## Required green commands

Run from repo root:

```bash
cargo fmt --all
bash scripts/check_audit_prep.sh .
bash scripts/check_testnet_deploy_drill.sh .
bash scripts/check_testnet_readiness_gate.sh .
cargo test -p rox-anchor-cli --test testnet_readiness_gate
cargo test --workspace
cargo check --workspace
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

If Anchor tooling is installed:

```bash
anchor build
anchor test
```

## Private pilot boundary

A green Phase 15 gate means:

```text
ROX Anchor is ready to consider a private testnet-only pilot.
```

It does not mean:

```text
public token launch
public bridge
public mint/burn
mainnet deployment
production settlement
production ROC release
staking
liquidity
exchange-facing readiness
```

## Manual operator checklist

Before any private testnet-only pilot:

```text
1. Confirm latest cargo test --workspace output is green.
2. Confirm latest cargo check --workspace output is green.
3. Confirm anchor build and anchor test are green if Anchor tooling is available.
4. Confirm no operator key material is in the repo.
5. Confirm payer, deploy, mint, upgrade, halt, and recovery keys are external.
6. Confirm test-only mint and token account labels are used.
7. Confirm capped submit mode is explicitly enabled only for a tiny private testnet drill.
8. Confirm receipts and audit reports are preserved.
9. Confirm halt authority and recovery authority drills are understood.
10. Confirm rollback/stop procedure is available before any submission attempt.
```

## Final Phase 15 statement

Successful Phase 15 means the testnet-only hardening plan is complete / green / parked for private pilot review. Public launch, mainnet, production bridge, production settlement, public mint/burn, exchange behavior, liquidity behavior, and staking behavior remain outside this plan.
