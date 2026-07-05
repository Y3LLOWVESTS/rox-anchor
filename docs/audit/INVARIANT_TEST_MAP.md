# ROX Anchor Phase 14 — Invariant-to-Test Map

No public launch authorization.

This document maps ROX Anchor security invariants to the tests that currently prove or guard them. It is an audit-prep index, not a runtime authorization document.

## Scope boundary

ROX Anchor remains testnet/localnet hardening software. This map does not authorize mainnet, public ROX mint/burn, production bridge settlement, production ROC release, public bridge UI, exchange-facing behavior, staking, liquidity, fake finality, or fake success output.

## Core invariant map

| Invariant | Primary tests / files | What the tests prove |
| --- | --- | --- |
| Shared states, IDs, labels, safety profiles, and authority roles come from `rox-anchor-core`. | `crates/rox-anchor-core/tests/testnet_scope_locks.rs`, `crates/rox-anchor-core/tests/operator_authority_model.rs`, `crates/rox-anchor-core/tests/kill_switch_drills.rs` | Mainnet is rejected, default scope is non-submitting, roles are separated, wrong authorities fail, halt/recovery reviews are deterministic. |
| Proof acceptance rejects replay and binding mismatches. | `crates/rox-anchor-proof/src/validate.rs`, `crates/rox-anchor-proof/tests/challenge_griefing.rs`, `crates/rox-anchor-proof/tests/halt_resume.rs`, `tests/unit/replay_rejection.rs`, `tests/unit/proof_package_validation.rs` | Operation IDs, idempotency keys, nonces, domain/direction/cluster/program/mint/token-account bindings, challenge, halt, and recovery posture are enforced before acceptance. |
| RPC evidence is read-only and cannot fabricate quorum. | `crates/rox-anchor-rpc-proof/tests/read_only_rpc_adapter.rs`, `crates/rox-anchor-rpc-proof/tests/rpc_equivocation_chaos.rs`, `crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs`, `tests/unit/rpc_quorum.rs`, `tests/chaos/rpc_equivocation.rs` | Missing, stale, disputed, equivocated, mismatched, and outage-shaped evidence fails closed without live submission. |
| Coordinator cannot turn rejected evidence into finalization. | `crates/rox-anchor-coordinator/tests/coordinator_relayer_boundary.rs`, `crates/rox-anchor-coordinator/tests/halt_recovery_finalization_gate.rs`, `crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs`, `tests/integration/coordinator_relayer_boundary.rs` | Coordinator decisions preserve proof rejection, halt/recovery blockers stop finalization, and testnet shadow flow remains bounded. |
| Relayer dry-run, simulation, and capped submission remain gated. | `crates/rox-anchor-relayer/tests/transaction_simulation.rs`, `crates/rox-anchor-relayer/tests/capped_testnet_submission.rs`, `crates/rox-anchor-relayer/tests/halt_recovery_submit_gate.rs`, `crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs`, `tests/chaos/relayer_retry_storm.rs` | Simulation requires accepted proof/coordinator/relayer state, capped testnet submission requires explicit approval, caps, receipts, and safe testnet scope. |
| Anchor program state transitions own on-chain-shaped rules. | `programs/rox-anchor/src/state.rs`, `programs/rox-anchor/src/instructions/*.rs`, `cargo test -p rox-anchor` | Program config, operation states, challenge, halt, recovery, finalize, mint/burn plan, and token settlement helper rules are compile-tested. |
| CLI output is inspection-only and cannot claim settlement or authority. | `crates/rox-anchor-cli/tests/status_display_boundary.rs`, `crates/rox-anchor-cli/tests/capped_submit_report.rs`, `crates/rox-anchor-cli/tests/kill_switch_drill_command.rs`, `crates/rox-anchor-cli/tests/testnet_deploy_drill_script.rs` | CLI reports deterministic local status, safe audit shape, redacted config, local kill-switch drill results, and no runtime/finality claims. |
| Deployment drill is non-authorizing and external-key only. | `scripts/check_testnet_deploy_drill.sh`, `crates/rox-anchor-cli/tests/testnet_deploy_drill_script.rs` | Mainnet config is rejected, deploy key artifacts stay ignored, and the drill does not deploy, submit, mint, burn, settle, or load a wallet. |

## Audit status

This map is tied to real crate-local and workspace tests. Missing future items should be added only with matching tests or scripts.
