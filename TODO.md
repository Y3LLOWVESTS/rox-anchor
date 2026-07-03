# ROX Anchor TODO and File Map

This repo has been reduced to the active Rust/Anchor implementation surface.

Current priority:

1. Make the workspace compile.
2. Make `rox-anchor-core` the shared type foundation.
3. Make `rox-anchor-proof` perform deterministic local proof validation.
4. Bring the CLI online for local inspection.
5. Bring coordinator, RPC proof, and relayer crates online as local services/models.
6. Bring `programs/rox-anchor` online as the Solana/Anchor program.
7. Add tests as each surface becomes real.

---

## Current file tree

.
  .gitattributes
  .gitignore
  Anchor.toml
  Cargo.toml
  LICENSE
  NOTICE
  TODO.md

  crates/
    rox-anchor-cli/
      Cargo.toml
      src/
        main.rs
        commands/
          check.rs
          halt.rs
          mod.rs
          proof.rs
          recover.rs
          status.rs

    rox-anchor-coordinator/
      Cargo.toml
      src/
        config.rs
        decision.rs
        main.rs
        observer.rs
        queue.rs
        readiness.rs
        redaction.rs

    rox-anchor-core/
      Cargo.toml
      src/
        errors.rs
        ids.rs
        labels.rs
        lib.rs
        state.rs
        types.rs

    rox-anchor-proof/
      Cargo.toml
      src/
        challenge.rs
        fixtures.rs
        lib.rs
        package.rs
        quorum.rs
        recovery.rs
        replay.rs
        validate.rs

    rox-anchor-relayer/
      Cargo.toml
      src/
        config.rs
        main.rs
        readiness.rs
        receipts.rs
        redaction.rs
        retry.rs
        submit.rs

    rox-anchor-rpc-proof/
      Cargo.toml
      src/
        commitment.rs
        config.rs
        main.rs
        quorum.rs
        readiness.rs
        redaction.rs
        rpc.rs

  programs/
    rox-anchor/
      Cargo.toml
      src/
        errors.rs
        events.rs
        lib.rs
        state.rs
        instructions/
          finalize.rs
          halt.rs
          initialize.rs
          mod.rs
          observe_burn.rs
          open_challenge.rs
          recover.rs
          resolve_challenge.rs

  tests/
    chaos/
      challenge_griefing.rs
      coordinator_stale_evidence.rs
      halt_resume.rs
      relayer_retry_storm.rs
      rpc_equivocation.rs

    integration/
      coordinator_relayer_boundary.rs
      crablink_status_display.rs
      local_nonvalue_roc_to_rox.rs
      local_nonvalue_rox_to_roc.rs

    unit/
      challenge_window.rs
      proof_package_validation.rs
      recovery_cases.rs
      replay_rejection.rs
      rpc_quorum.rs
      state_machine_transitions.rs

---

## Root files

### `.gitattributes`

Git attribute rules for the repo. Keep this for line-ending normalization and future repository hygiene.

### `.gitignore`

Git ignore rules for build outputs, local editor files, secrets, and transient artifacts. Keep this updated as Cargo, Anchor, and local tooling generate new output folders.

### `Anchor.toml`

Anchor project configuration for the Solana program under `programs/rox-anchor`. This becomes important once the Anchor program is compile-tested and later run under local validator/dev tooling.

### `Cargo.toml`

Root Cargo workspace manifest. This decides which crates and programs are included in `cargo check --workspace`.

### `LICENSE`

Project license file. Keep this as repo metadata.

### `NOTICE`

Project notice/attribution file. Keep this as repo metadata.

### `TODO.md`

This file. It tracks the current reduced repo shape and what each source file is supposed to do.

---

## `crates/rox-anchor-core`

Shared type foundation for the whole project. This crate should stay dependency-light and should define common IDs, states, labels, errors, and proof-review domain concepts used by all other crates.

### `crates/rox-anchor-core/Cargo.toml`

Cargo manifest for the shared core crate. It should expose `rox_anchor_core` as a library and avoid unnecessary dependencies.

### `crates/rox-anchor-core/src/lib.rs`

Library root for `rox-anchor-core`. It should re-export the public shared types used by proof, CLI, coordinator, relayer, RPC proof, and the Anchor program where appropriate.

### `crates/rox-anchor-core/src/errors.rs`

Shared error types for core validation and typed wrapper construction. This should contain small, deterministic errors instead of service-specific failures.

### `crates/rox-anchor-core/src/ids.rs`

Typed ID wrappers for anchor domains, operation IDs, idempotency keys, nonces, clusters, program IDs, mints, and token accounts. This file should prevent every crate from inventing raw string identity rules.

### `crates/rox-anchor-core/src/labels.rs`

Shared display/status labels for safe review states. This keeps CLI, coordinator, UI-facing services, and proof review from using conflicting wording.

### `crates/rox-anchor-core/src/state.rs`

Shared state machine labels for anchor lifecycle status. This should define the allowed states and helper methods for status classification.

### `crates/rox-anchor-core/src/types.rs`

Shared domain enums and data structures such as direction, challenge posture, halt posture, recovery posture, and proof package skeletons. This file should be the first place other crates look before creating new type names.

---

## `crates/rox-anchor-proof`

Deterministic local proof-review engine. This crate should validate proof packages, reject replay, detect domain/cluster/program/mint mismatches, classify challenge/recovery/halt posture, and return review decisions.

### `crates/rox-anchor-proof/Cargo.toml`

Cargo manifest for the local proof validation crate. It should depend on `rox-anchor-core` and only add dependencies that are needed for real validation.

### `crates/rox-anchor-proof/src/lib.rs`

Library root for the proof engine. It should re-export the proof package types, review functions, findings, and decisions used by tests, CLI, coordinator, and later services.

### `crates/rox-anchor-proof/src/package.rs`

Proof package shape and expected binding definitions. This file should own how incoming proof material is represented before validation.

### `crates/rox-anchor-proof/src/validate.rs`

Main proof-review logic. This file should combine package validation, binding checks, evidence posture, quorum posture, replay posture, challenge status, halt status, and recovery status into a deterministic decision.

### `crates/rox-anchor-proof/src/replay.rs`

Replay rejection helpers. This file should detect reused nonces/idempotency bindings and return findings that prevent repeated proof packages from passing review.

### `crates/rox-anchor-proof/src/quorum.rs`

Quorum/evidence posture definitions and helpers. This file should classify whether observations are present, incomplete, disputed, or otherwise unusable for acceptance.

### `crates/rox-anchor-proof/src/challenge.rs`

Challenge-window review helpers. This file should classify whether a challenge is open, accepted, rejected, expired, or otherwise blocking proof acceptance.

### `crates/rox-anchor-proof/src/recovery.rs`

Halt and recovery review helpers. This file should classify recovery-required, halted, queued, or resolved cases and prevent unsafe acceptance when recovery review is still needed.

### `crates/rox-anchor-proof/src/fixtures.rs`

In-code fixtures for proof-review tests. Since JSON vectors were removed during cleanup, this file can hold small deterministic test packages until fixture files are intentionally reintroduced.

---

## `crates/rox-anchor-cli`

Local command-line inspection surface. This crate should become the developer/operator tool for checking proof packages, printing status, reviewing halt/recovery cases, and inspecting local evidence.

### `crates/rox-anchor-cli/Cargo.toml`

Cargo manifest for the CLI crate. It should depend on core/proof and later add CLI parsing dependencies if needed.

### `crates/rox-anchor-cli/src/main.rs`

CLI entry point. This should parse commands and dispatch into the command modules.

### `crates/rox-anchor-cli/src/commands/mod.rs`

Command module registry. This file should expose the individual CLI command handlers.

### `crates/rox-anchor-cli/src/commands/check.rs`

Implementation for a local `check` command. This should eventually read a proof package and print deterministic review output.

### `crates/rox-anchor-cli/src/commands/proof.rs`

Implementation for proof inspection commands. This should focus on package details, evidence posture, and validation findings.

### `crates/rox-anchor-cli/src/commands/status.rs`

Implementation for status inspection commands. This should print local status labels and lifecycle state without inventing finality.

### `crates/rox-anchor-cli/src/commands/halt.rs`

Implementation for halt inspection commands. This should show halt posture and explain why a package or case is blocked.

### `crates/rox-anchor-cli/src/commands/recover.rs`

Implementation for recovery inspection commands. This should review recovery cases and explain required next steps.

---

## `crates/rox-anchor-coordinator`

Local coordinator model/service. This crate should eventually assemble observations, queue review work, decide local review status, and prepare evidence for relayer/RPC proof layers.

### `crates/rox-anchor-coordinator/Cargo.toml`

Cargo manifest for the coordinator crate. It should depend on core/proof before adding any service dependencies.

### `crates/rox-anchor-coordinator/src/main.rs`

Coordinator executable entry point. This should start the local coordinator process once the crate becomes active.

### `crates/rox-anchor-coordinator/src/config.rs`

Coordinator configuration types. This should define local paths, review settings, queue settings, and redaction options.

### `crates/rox-anchor-coordinator/src/observer.rs`

Observer/evidence input model. This should describe how local observations are represented before proof review.

### `crates/rox-anchor-coordinator/src/queue.rs`

Queue model for pending review work. This should keep ordering, retry, and idempotency behavior explicit.

### `crates/rox-anchor-coordinator/src/decision.rs`

Coordinator decision logic. This should use `rox-anchor-proof` decisions instead of inventing separate acceptance rules.

### `crates/rox-anchor-coordinator/src/readiness.rs`

Coordinator readiness model. This should report whether config, queue, proof engine, and observer input are usable.

### `crates/rox-anchor-coordinator/src/redaction.rs`

Redaction helpers for logs and reports. This should prevent sensitive values from being printed raw.

---

## `crates/rox-anchor-rpc-proof`

Local RPC evidence/quorum model. This crate should compare RPC observations, classify disagreement, and provide evidence to the proof engine or coordinator.

### `crates/rox-anchor-rpc-proof/Cargo.toml`

Cargo manifest for the RPC proof crate. It should start as local evidence modeling before adding any live network behavior.

### `crates/rox-anchor-rpc-proof/src/main.rs`

Executable entry point for RPC proof tooling or service mode. This should remain small and call into library-style modules.

### `crates/rox-anchor-rpc-proof/src/config.rs`

RPC proof configuration. This should describe endpoints, quorum thresholds, timeouts, redaction, and local mode behavior.

### `crates/rox-anchor-rpc-proof/src/rpc.rs`

RPC observation model. This should define request/response evidence shapes and avoid hidden mutation behavior.

### `crates/rox-anchor-rpc-proof/src/quorum.rs`

RPC quorum logic. This should classify agreement, disagreement, missing evidence, and equivocation-style cases.

### `crates/rox-anchor-rpc-proof/src/commitment.rs`

Commitment-level review helpers. This should classify whether observed RPC evidence has enough commitment for local proof review.

### `crates/rox-anchor-rpc-proof/src/readiness.rs`

Readiness model for the RPC proof component. This should report whether configured sources and local proof settings are usable.

### `crates/rox-anchor-rpc-proof/src/redaction.rs`

Redaction helpers for RPC proof reports. This should strip or shorten sensitive/verbose fields before display.

---

## `crates/rox-anchor-relayer`

Relayer code surface. This crate should eventually prepare and submit reviewed evidence, but before live submission it should model receipts, retries, readiness, and redacted local reports.

### `crates/rox-anchor-relayer/Cargo.toml`

Cargo manifest for the relayer crate. It should start with local models and only add live dependencies when intentionally needed.

### `crates/rox-anchor-relayer/src/main.rs`

Relayer executable entry point. This should start the relayer process or local dry-run mode.

### `crates/rox-anchor-relayer/src/config.rs`

Relayer configuration. This should hold local submission settings, retry limits, receipt output paths, and safety toggles.

### `crates/rox-anchor-relayer/src/submit.rs`

Submission model. This should eventually own the transition from reviewed local evidence to a submission request.

### `crates/rox-anchor-relayer/src/retry.rs`

Retry logic for relayer attempts. This should bound retries, backoff, and idempotency behavior.

### `crates/rox-anchor-relayer/src/receipts.rs`

Receipt model for relayer outcomes. This should record what happened during local or future submission attempts.

### `crates/rox-anchor-relayer/src/readiness.rs`

Relayer readiness model. This should report whether config, queue, proof input, and receipt output are usable.

### `crates/rox-anchor-relayer/src/redaction.rs`

Redaction helpers for relayer logs and receipts. This should avoid leaking raw sensitive values.

---

## `programs/rox-anchor`

Solana/Anchor program surface. This is where the actual on-chain program code lives once we compile-test the Anchor path.

### `programs/rox-anchor/Cargo.toml`

Cargo manifest for the Anchor program. This should include Anchor dependencies and build as the on-chain program crate.

### `programs/rox-anchor/src/lib.rs`

Anchor program root. This should declare the program module and route instruction handlers.

### `programs/rox-anchor/src/errors.rs`

Anchor program error definitions. This should map invalid states, mismatches, challenge violations, halt violations, and recovery violations into program errors.

### `programs/rox-anchor/src/events.rs`

Anchor event definitions. This should emit structured events for initialization, observations, finalization, challenge actions, halt actions, and recovery actions.

### `programs/rox-anchor/src/state.rs`

Anchor account state definitions. This should define program-owned accounts, config/state records, challenge state, halt state, and recovery state.

### `programs/rox-anchor/src/instructions/mod.rs`

Instruction module registry. This should expose the individual Anchor instruction handlers.

### `programs/rox-anchor/src/instructions/initialize.rs`

Initialize instruction. This should create or configure the program state account and set initial parameters.

### `programs/rox-anchor/src/instructions/observe_burn.rs`

Observe-burn instruction. This should record or verify a burn observation according to program state rules.

### `programs/rox-anchor/src/instructions/finalize.rs`

Finalize instruction. This should transition an eligible reviewed operation into the program’s finalized state when all checks pass.

### `programs/rox-anchor/src/instructions/open_challenge.rs`

Open-challenge instruction. This should create a challenge record or move an operation into a challenge-open state.

### `programs/rox-anchor/src/instructions/resolve_challenge.rs`

Resolve-challenge instruction. This should accept or reject a challenge and update operation state accordingly.

### `programs/rox-anchor/src/instructions/halt.rs`

Halt instruction. This should pause or halt sensitive program transitions according to authority and state rules.

### `programs/rox-anchor/src/instructions/recover.rs`

Recover instruction. This should handle approved recovery transitions after halt/challenge/failure cases.

---

## `tests/unit`

Focused Rust unit tests. These should compile and run as soon as the crates they touch are buildable.

### `tests/unit/proof_package_validation.rs`

Tests valid and invalid proof package review outcomes. This should prove that package validation accepts good local cases and rejects malformed or mismatched ones.

### `tests/unit/replay_rejection.rs`

Tests replay prevention. This should prove that reused nonce/idempotency/domain bindings cannot pass review.

### `tests/unit/challenge_window.rs`

Tests challenge posture behavior. This should prove that open or accepted challenges block acceptance and that safe terminal challenge states are handled correctly.

### `tests/unit/recovery_cases.rs`

Tests halt and recovery review behavior. This should prove that recovery-required or halted states block unsafe acceptance.

### `tests/unit/rpc_quorum.rs`

Tests RPC quorum/evidence behavior. This should prove that agreement, disagreement, missing evidence, and stale evidence classify correctly.

### `tests/unit/state_machine_transitions.rs`

Tests lifecycle state transitions. This should prove that the allowed state machine paths are deterministic and invalid transitions are rejected.

---

## `tests/integration`

Cross-crate integration tests. These should be activated only after the relevant crates compile.

### `tests/integration/local_nonvalue_roc_to_rox.rs`

Integration test for the ROC-to-ROX local review path. This should validate the end-to-end local proof path without live settlement.

### `tests/integration/local_nonvalue_rox_to_roc.rs`

Integration test for the ROX-to-ROC local review path. This should validate the reverse local proof path.

### `tests/integration/coordinator_relayer_boundary.rs`

Integration test for coordinator-to-relayer handoff. This should prove that only reviewed evidence moves forward.

### `tests/integration/crablink_status_display.rs`

Integration test for display-safe status output. This should prove that downstream status labels remain backend-derived and unambiguous.

---

## `tests/chaos`

Failure and adversarial behavior tests. These should be activated after the core local model and services exist.

### `tests/chaos/challenge_griefing.rs`

Chaos test for repeated or abusive challenge behavior. This should prove challenge handling remains bounded and deterministic.

### `tests/chaos/coordinator_stale_evidence.rs`

Chaos test for stale coordinator evidence. This should prove stale observations do not become accepted review material.

### `tests/chaos/halt_resume.rs`

Chaos test for halt and resume paths. This should prove halt/recovery transitions behave safely under repeated attempts.

### `tests/chaos/relayer_retry_storm.rs`

Chaos test for relayer retry storms. This should prove retries are bounded and idempotent.

### `tests/chaos/rpc_equivocation.rs`

Chaos test for conflicting RPC observations. This should prove RPC disagreement is detected and classified instead of silently accepted.

---

## Immediate TODO

### 1. Workspace compile

Make `cargo check --workspace` pass.

Likely tasks:

- Confirm root `Cargo.toml` includes every active crate.
- Fix placeholder crates that do not compile.
- Remove old comment language from source files.
- Ensure every crate has a clear `main.rs` or `lib.rs` surface as needed.

### 2. Core crate

Make `rox-anchor-core` the shared source of truth for:

- IDs
- domains
- nonces
- directions
- lifecycle states
- challenge/halt/recovery postures
- safe status labels
- reusable errors

### 3. Proof crate

Make `rox-anchor-proof` prove real local behavior:

- valid package acceptance for local review
- missing field rejection
- replay rejection
- cluster mismatch rejection
- program mismatch rejection
- mint mismatch rejection
- token account mismatch rejection
- quorum disagreement classification
- challenge-open blocking
- recovery-required blocking
- halt blocking

### 4. CLI crate

Bring up a minimal command:

- `rox-anchor check`
- it should call into `rox-anchor-proof`
- it should print review findings deterministically

### 5. Anchor program

Bring `programs/rox-anchor` into compile-tested shape:

- valid Anchor manifest
- program root
- account state
- error enum
- event enum
- initialize instruction
- observation instruction
- challenge/halt/recovery/finalize instruction shells that compile

### 6. Tests

Move from placeholder tests to compile-tested tests:

- start with core/proof unit tests
- then CLI smoke tests
- then coordinator/RPC proof/relayer tests
- then Anchor program tests

---

## Current guiding rule

Every next patch should make code compile, add behavior, or add tests.

Avoid adding broad docs, decision gates, or placeholder scaffolds unless they directly support compiling and testing the current Rust surface.
