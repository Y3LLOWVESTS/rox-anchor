# ROX Anchor Phase 14 — Halt and Recovery Runbook

No public launch authorization.

This runbook describes local/testnet halt and recovery behavior for audit review. It does not authorize live mainnet operation, public token launch, production bridge settlement, production ROC release, or public mint/burn access.

## When to halt

Use halt posture in drills when any of the following are observed:

```text
binding mismatch
RPC equivocation
RPC outage that prevents quorum confidence
stale/reorg-like evidence
wrong authority attempt
receipt tamper
simulation tamper
unexpected retry storm
unexpected cap pressure
operator key unavailable
challenge accepted
manual safety concern
```

## Expected halt behavior

| Stage | Expected result | Test coverage |
| --- | --- | --- |
| Before proof acceptance | Proof acceptance blocked. | `crates/rox-anchor-core/tests/kill_switch_drills.rs`, `crates/rox-anchor-cli/tests/kill_switch_drill_command.rs` |
| After proof acceptance before simulation | Simulation blocked. | `crates/rox-anchor-relayer/tests/halt_recovery_submit_gate.rs`, `crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs` |
| After simulation before submission | Capped submission blocked. | `crates/rox-anchor-relayer/tests/halt_recovery_submit_gate.rs` |
| Before coordinator finalization | Finalization blocked. | `crates/rox-anchor-coordinator/tests/halt_recovery_finalization_gate.rs` |
| Program state | Finalization blocked while halted or recovery-required. | `programs/rox-anchor/src/state.rs` |

## Recovery path

Recovery must be explicit:

```text
1. Confirm halt authority/recovery authority separation.
2. Confirm no replay or duplicated operation was accepted.
3. Confirm challenge posture is resolved or expired.
4. Confirm RPC evidence is fresh and non-equivocated.
5. Confirm coordinator decision can be recomputed deterministically.
6. Confirm relayer dry-run can resume without live submission.
7. Confirm status/audit output reflects recovery resolved.
```

## Tests of interest

```text
crates/rox-anchor-core/tests/kill_switch_drills.rs
crates/rox-anchor-proof/tests/halt_resume.rs
crates/rox-anchor-coordinator/tests/halt_recovery_finalization_gate.rs
crates/rox-anchor-relayer/tests/halt_recovery_submit_gate.rs
crates/rox-anchor-cli/tests/halt_recovery_status.rs
crates/rox-anchor-cli/tests/kill_switch_drill_command.rs
programs/rox-anchor/src/state.rs
```

## Operator safety note

CLI drill output is local report-only. It must not load wallets, submit RPC transactions, mint, burn, release ROC, settle, or claim public finality.
