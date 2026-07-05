# ROX Anchor Phase 14 — State Transition Map

No public launch authorization.

This document summarizes the state transition surface for audit review. The Anchor program owns on-chain-shaped state transitions. Local crates model and preflight those transitions but must not invent separate finality rules.

## Main state surfaces

| Surface | Files | Test coverage |
| --- | --- | --- |
| Shared lifecycle/posture states | `crates/rox-anchor-core/src/state.rs`, `crates/rox-anchor-core/src/operations.rs` | `crates/rox-anchor-core/tests/kill_switch_drills.rs` |
| Proof review states | `crates/rox-anchor-proof/src/validate.rs`, `crates/rox-anchor-proof/src/challenge.rs`, `crates/rox-anchor-proof/src/recovery.rs` | `crates/rox-anchor-proof/tests/challenge_griefing.rs`, `crates/rox-anchor-proof/tests/halt_resume.rs` |
| Coordinator finalization gate | `crates/rox-anchor-coordinator/src/decision.rs` | `crates/rox-anchor-coordinator/tests/halt_recovery_finalization_gate.rs` |
| Relayer dry-run/simulation/capped submit gate | `crates/rox-anchor-relayer/src/submit.rs`, `crates/rox-anchor-relayer/src/receipts.rs` | `crates/rox-anchor-relayer/tests/transaction_simulation.rs`, `crates/rox-anchor-relayer/tests/capped_testnet_submission.rs` |
| Anchor program config and operation state | `programs/rox-anchor/src/state.rs`, `programs/rox-anchor/src/instructions/*.rs` | `cargo test -p rox-anchor` |

## Transition expectations

```text
observed -> challenged
observed -> halted
observed -> recovery required
observed -> finalized only when clear and eligible
challenged -> resolved or blocked
halted -> recovery path only
recovery required -> recovery resolved before unsafe progress
finalized -> cannot reopen
```

## Required blockers

| Blocker | Expected behavior |
| --- | --- |
| Challenge open or accepted | Blocks proof acceptance/finality paths. |
| Halted posture | Blocks acceptance, simulation, submission, and finalization. |
| Recovery required or in review | Blocks unsafe acceptance, submission, and finalization. |
| Replay | Blocks acceptance before coordinator/relayer attempt. |
| Binding mismatch | Blocks acceptance before simulation or submission. |
| Wrong authority | Blocks halt/recover/config transitions. |

## Program state tests of interest

The program crate currently has focused unit coverage for:

```text
operation_can_finalize_only_when_clear
accepted_challenge_blocks_finalization
challenge_resolution_controls_finalization
halt_and_recovery_are_explicit_blockers
wrong_authority_cannot_halt_or_recover_config
finalized_operations_cannot_be_reopened
config_rox_mint_mismatch_blocks_finalize
corrupt_mint_burn_binding_blocks_finalize
token_settlement_* helpers and receipt/event tamper checks
```

These tests are local and do not authorize live value movement.
