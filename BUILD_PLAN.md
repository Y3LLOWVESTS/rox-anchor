# ROX Anchor Build Plan

This is an implementation build plan.

The goal is to turn the reduced repo into compile-tested ROX Anchor software.

Rules for every phase:

- Write Rust code.
- Run focused compile/tests.
- Fix failures before moving forward.
- Do not add broad planning docs or decision gates.
- Keep comments about what the code does, validates, rejects, and returns.
- Do not add live deployment, live wallet calls, live RPC submission, or real value movement until the local program and tests are compile-green.

---

## Phase 0 — Workspace Recovery and Compile Spine

Purpose:

Make the reduced repo buildable again after cleanup.

Files:

```text
Cargo.toml
Anchor.toml
crates/*/Cargo.toml
programs/rox-anchor/Cargo.toml
```

Work:

```text
1. Ensure root Cargo.toml includes every crate and program that should compile.
2. Fix crate names, binary names, and lib/bin targets.
3. Add only the minimum dependencies needed for compile.
4. Decide which crates are library crates, binary crates, or both.
5. Run focused cargo checks and repair manifest problems.
```

Commands:

```bash
cargo check -p rox-anchor-core
cargo check -p rox-anchor-proof
cargo check --workspace
```

Exit condition:

```text
cargo check --workspace reaches source-code errors instead of manifest/workspace errors,
then reaches green after the first compile-repair pass.
```

---

## Phase 1 — Core Type Foundation

Purpose:

Make `rox-anchor-core` the shared source of truth.

Files:

```text
crates/rox-anchor-core/Cargo.toml
crates/rox-anchor-core/src/lib.rs
crates/rox-anchor-core/src/errors.rs
crates/rox-anchor-core/src/ids.rs
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-core/src/state.rs
crates/rox-anchor-core/src/labels.rs
```

Work:

```text
1. Define typed wrappers for domains, operation IDs, idempotency keys, nonces, clusters, program IDs, mints, token accounts, and accounts.
2. Define direction types for ROC-to-ROX and ROX-to-ROC.
3. Define lifecycle states for observed, challenged, halted, recovery, finalized, and failed cases.
4. Define challenge, halt, and recovery postures.
5. Define shared error types.
6. Define safe status labels.
7. Re-export the public API from lib.rs.
8. Add unit tests inside the crate.
```

Commands:

```bash
cargo fmt -p rox-anchor-core
cargo test -p rox-anchor-core
```

Exit condition:

```text
rox-anchor-core compiles, tests pass, and no other crate needs to invent duplicate IDs, labels, or states.
```

---

## Phase 2 — Local Proof Engine

Purpose:

Make `rox-anchor-proof` perform deterministic proof package review.

Files:

```text
crates/rox-anchor-proof/Cargo.toml
crates/rox-anchor-proof/src/lib.rs
crates/rox-anchor-proof/src/package.rs
crates/rox-anchor-proof/src/validate.rs
crates/rox-anchor-proof/src/replay.rs
crates/rox-anchor-proof/src/quorum.rs
crates/rox-anchor-proof/src/challenge.rs
crates/rox-anchor-proof/src/recovery.rs
crates/rox-anchor-proof/src/fixtures.rs
```

Work:

```text
1. Use rox-anchor-core types instead of duplicate local enums.
2. Define proof package input shape.
3. Define expected binding shape.
4. Validate required fields.
5. Reject source/target domain mismatch.
6. Reject direction mismatch.
7. Reject cluster mismatch.
8. Reject program ID mismatch.
9. Reject mint mismatch.
10. Reject token account mismatch.
11. Reject replayed nonce/idempotency combinations.
12. Classify incomplete evidence.
13. Classify quorum disagreement.
14. Block acceptance when challenge is open or accepted.
15. Block acceptance when halt or recovery review is required.
16. Return deterministic findings and decisions.
```

Commands:

```bash
cargo fmt -p rox-anchor-proof
cargo test -p rox-anchor-proof
```

Exit condition:

```text
The proof engine returns deterministic decisions for valid, incomplete, rejected, challenge-blocked, halt-blocked, and recovery-blocked packages.
```

---

## Phase 3 — Unit Test Lift

Purpose:

Turn the existing root unit tests into compile-tested Rust tests.

Files:

```text
tests/unit/proof_package_validation.rs
tests/unit/replay_rejection.rs
tests/unit/challenge_window.rs
tests/unit/recovery_cases.rs
tests/unit/rpc_quorum.rs
tests/unit/state_machine_transitions.rs
```

Work:

```text
1. Move tests into crate-level tests if needed.
2. Create shared Rust fixtures instead of relying on deleted JSON vectors.
3. Test valid proof review.
4. Test missing fields.
5. Test replay rejection.
6. Test cluster/program/mint/token-account mismatch.
7. Test challenge-window blocking.
8. Test halt/recovery blocking.
9. Test lifecycle state transitions.
```

Commands:

```bash
cargo test -p rox-anchor-core
cargo test -p rox-anchor-proof
cargo test --workspace
```

Exit condition:

```text
Core/proof tests are real, green, and prove the local validation behavior.
```

---

## Phase 4 — CLI Inspection Tool

Purpose:

Make `rox-anchor-cli` useful from the terminal.

Files:

```text
crates/rox-anchor-cli/Cargo.toml
crates/rox-anchor-cli/src/main.rs
crates/rox-anchor-cli/src/commands/mod.rs
crates/rox-anchor-cli/src/commands/check.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/src/commands/halt.rs
crates/rox-anchor-cli/src/commands/recover.rs
```

Work:

```text
1. Add CLI parsing.
2. Implement `rox-anchor check`.
3. Make check use in-code fixture first.
4. Print deterministic review decision.
5. Print findings.
6. Print status label.
7. Add command smoke tests.
8. Later, add JSON input support after the local engine is green.
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli
cargo run -p rox-anchor-cli -- check
```

Exit condition:

```text
A user can run a local CLI command and see an actual proof review report.
```

---

## Phase 5 — RPC Proof Local Evidence Model

Purpose:

Make `rox-anchor-rpc-proof` classify RPC evidence locally.

Files:

```text
crates/rox-anchor-rpc-proof/Cargo.toml
crates/rox-anchor-rpc-proof/src/main.rs
crates/rox-anchor-rpc-proof/src/config.rs
crates/rox-anchor-rpc-proof/src/rpc.rs
crates/rox-anchor-rpc-proof/src/quorum.rs
crates/rox-anchor-rpc-proof/src/commitment.rs
crates/rox-anchor-rpc-proof/src/readiness.rs
crates/rox-anchor-rpc-proof/src/redaction.rs
```

Work:

```text
1. Define local RPC observation structs.
2. Define agreement/disagreement classification.
3. Define commitment-level classification.
4. Define redacted report output.
5. Do not add live network calls yet.
6. Feed RPC evidence posture into rox-anchor-proof decisions.
7. Add unit tests for agreement, disagreement, missing evidence, stale evidence, and equivocation.
```

Commands:

```bash
cargo fmt -p rox-anchor-rpc-proof
cargo test -p rox-anchor-rpc-proof
```

Exit condition:

```text
RPC evidence can be modeled and classified locally without live RPC calls.
```

---

## Phase 6 — Coordinator Local Model

Purpose:

Make `rox-anchor-coordinator` assemble observations and produce local review decisions.

Files:

```text
crates/rox-anchor-coordinator/Cargo.toml
crates/rox-anchor-coordinator/src/main.rs
crates/rox-anchor-coordinator/src/config.rs
crates/rox-anchor-coordinator/src/observer.rs
crates/rox-anchor-coordinator/src/queue.rs
crates/rox-anchor-coordinator/src/decision.rs
crates/rox-anchor-coordinator/src/readiness.rs
crates/rox-anchor-coordinator/src/redaction.rs
```

Work:

```text
1. Define coordinator config.
2. Define observer input records.
3. Define queue item and queue state.
4. Define decision wrapper around rox-anchor-proof.
5. Define readiness output.
6. Define redacted report output.
7. Add tests for stale evidence, duplicate evidence, rejected evidence, and valid review handoff.
```

Commands:

```bash
cargo fmt -p rox-anchor-coordinator
cargo test -p rox-anchor-coordinator
```

Exit condition:

```text
Coordinator can take local observations and produce deterministic proof-review decisions.
```

---

## Phase 7 — Relayer Local Dry-Run Model

Purpose:

Make `rox-anchor-relayer` model submissions, retries, and receipts without live submission first.

Files:

```text
crates/rox-anchor-relayer/Cargo.toml
crates/rox-anchor-relayer/src/main.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/retry.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/readiness.rs
crates/rox-anchor-relayer/src/redaction.rs
```

Work:

```text
1. Define relayer config.
2. Define local submission request.
3. Define retry policy.
4. Define receipt type.
5. Define readiness output.
6. Define redacted output.
7. Add tests for bounded retries, idempotency, receipt generation, and retry storms.
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer
cargo test -p rox-anchor-relayer
```

Exit condition:

```text
Relayer dry-run behavior is deterministic and tested.
```

---

## Phase 8 — Anchor Program Compile Foundation

Purpose:

Make `programs/rox-anchor` compile as a real Anchor program.

Files:

```text
programs/rox-anchor/Cargo.toml
programs/rox-anchor/src/lib.rs
programs/rox-anchor/src/errors.rs
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/mod.rs
programs/rox-anchor/src/instructions/initialize.rs
programs/rox-anchor/src/instructions/observe_burn.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/instructions/open_challenge.rs
programs/rox-anchor/src/instructions/resolve_challenge.rs
programs/rox-anchor/src/instructions/halt.rs
programs/rox-anchor/src/instructions/recover.rs
```

Work:

```text
1. Add valid Anchor dependencies.
2. Define program ID placeholder.
3. Define program account state.
4. Define operation state account.
5. Define challenge state account.
6. Define config/authority fields.
7. Define program error enum.
8. Define program events.
9. Make initialize compile.
10. Make observe_burn compile.
11. Make open_challenge compile.
12. Make resolve_challenge compile.
13. Make halt compile.
14. Make recover compile.
15. Make finalize compile.
```

Commands:

```bash
cargo fmt -p rox-anchor
cargo check -p rox-anchor
```

If Anchor CLI is installed:

```bash
anchor build
```

Exit condition:

```text
The Anchor program compiles locally.
```

---

## Phase 9 — Anchor Program State Rules

Purpose:

Make the program enforce real state transitions.

Work:

```text
1. Initialize program config.
2. Record burn observation.
3. Reject wrong domain.
4. Reject wrong mint.
5. Reject wrong program binding.
6. Reject replayed operation ID.
7. Open challenge within allowed state.
8. Resolve challenge.
9. Halt transitions.
10. Recover from halted/recovery states.
11. Finalize only when eligible.
```

Commands:

```bash
cargo test -p rox-anchor
anchor test
```

Exit condition:

```text
Program state transitions are enforced by code and tests.
```

---

## Phase 10 — ROX / ROC Mint-Burn Program Logic

Purpose:

Implement the actual mint/burn semantics once the state machine compiles.

Work:

```text
1. Define ROC burn observation fields.
2. Define ROX mint authority model.
3. Define reverse ROX burn observation fields.
4. Define ROC release/recovery model.
5. Enforce mint binding.
6. Enforce token account binding.
7. Enforce replay protection.
8. Enforce challenge window.
9. Enforce halt/recovery controls.
10. Emit events for every successful transition.
```

Commands:

```bash
cargo check -p rox-anchor
anchor build
anchor test
```

Exit condition:

```text
Local validator tests prove the mint/burn state machine works without touching any production environment.
```

---

## Phase 11 — Integration Tests

Purpose:

Prove cross-crate behavior.

Files:

```text
tests/integration/local_nonvalue_roc_to_rox.rs
tests/integration/local_nonvalue_rox_to_roc.rs
tests/integration/coordinator_relayer_boundary.rs
tests/integration/crablink_status_display.rs
```

Work:

```text
1. Wire proof engine to coordinator.
2. Wire coordinator output to relayer dry-run.
3. Wire RPC proof evidence into proof review.
4. Test ROC-to-ROX local path.
5. Test ROX-to-ROC local path.
6. Test status display labels.
```

Commands:

```bash
cargo test --workspace
```

Exit condition:

```text
Core, proof, CLI, RPC proof, coordinator, relayer, and Anchor surfaces agree on the same states and decisions.
```

---

## Phase 12 — Chaos Tests

Purpose:

Prove bad conditions fail cleanly.

Files:

```text
tests/chaos/challenge_griefing.rs
tests/chaos/coordinator_stale_evidence.rs
tests/chaos/halt_resume.rs
tests/chaos/relayer_retry_storm.rs
tests/chaos/rpc_equivocation.rs
```

Work:

```text
1. Challenge griefing remains bounded.
2. Stale coordinator evidence is rejected.
3. Halt/resume stays deterministic.
4. Relayer retry storms are bounded.
5. RPC equivocation is detected.
```

Commands:

```bash
cargo test --workspace
```

Exit condition:

```text
The local model rejects or contains adversarial cases deterministically.
```

---

## Phase 13 — Final Local Green Run

Purpose:

Prove the reduced repo is now a real working implementation.

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
```

If Anchor CLI is installed:

```bash
anchor build
anchor test
```

Exit condition:

```text
The repo is compile-green and test-green locally.
The proof engine works.
The CLI works.
The local service models work.
The Anchor program compiles.
The mint/burn state machine is tested locally.
```

---

## Build order summary

```text
Phase 0  — Workspace recovery
Phase 1  — Core shared types
Phase 2  — Proof engine
Phase 3  — Unit tests
Phase 4  — CLI
Phase 5  — RPC proof model
Phase 6  — Coordinator model
Phase 7  — Relayer dry-run model
Phase 8  — Anchor compile foundation
Phase 9  — Anchor state rules
Phase 10 — ROX/ROC mint-burn logic
Phase 11 — Integration tests
Phase 12 — Chaos tests
Phase 13 — Final local green run
```

## Immediate next command

Start with:

```bash
cargo check --workspace
```

Then fix the first compiler failure.
