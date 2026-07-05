# ROX Anchor Phase 14 — Relayer Submission Boundary

No public launch authorization.

This document summarizes the relayer dry-run, simulation, and capped testnet submission boundary. The relayer is not a production bridge, not a public mint/burn executor, and not an exchange/liquidity/staking surface.

## Relayer boundary files

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/retry.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-relayer/src/audit.rs
crates/rox-anchor-relayer/src/redaction.rs
```

## Required gates

| Gate | Test coverage |
| --- | --- |
| Proof review must be accepted. | `crates/rox-anchor-relayer/tests/transaction_simulation.rs` |
| Coordinator decision must be accepted before simulation. | `crates/rox-anchor-coordinator/tests/transaction_simulation_gate.rs` |
| Relayer dry-run must be accepted before simulation. | `crates/rox-anchor-relayer/tests/transaction_simulation.rs` |
| Capped submit requires explicit operator approval. | `crates/rox-anchor-relayer/tests/capped_testnet_submission.rs` |
| Capped submit requires safe testnet scope. | `crates/rox-anchor-relayer/tests/testnet_scope_locks.rs` |
| Capped submit requires retry, operation, and amount caps. | `crates/rox-anchor-relayer/tests/capped_testnet_submission.rs`, `crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs` |
| Capped submit requires receipt persistence when configured. | `crates/rox-anchor-relayer/tests/capped_testnet_submission.rs` |
| Halt/recovery/challenge postures block unsafe submit paths. | `crates/rox-anchor-relayer/tests/halt_recovery_submit_gate.rs` |
| Retry storms remain bounded. | `crates/rox-anchor-relayer/tests/relayer_retry_storm.rs` |

## Explicit non-claims

Relayer reports must not claim:

```text
production settlement
public bridge completion
public ROX mint completion
public ROX burn completion
production ROC release
mainnet submission
exchange readiness
staking readiness
liquidity readiness
```

## Audit note

Even when capped testnet authorization returns an accepted local report, the model must distinguish authorization, attempted submission, and network-submitted state. Default mode remains non-submitting.
