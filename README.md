# ROX Anchor

> **Build status:** ROX Anchor is an active work-in-progress implementation. The repo is being built in phases, with a QuickChain-style balance of real Rust code, focused tests, compile checks, and fixes before moving forward. Some crates may be incomplete while their phase is not yet finished. Current priority is making the workspace compile, then building `rox-anchor-core`, `rox-anchor-proof`, the CLI, local service models, and the Anchor program in order.
>
> **Development posture:** This repo should be treated as implementation-first. Avoid broad planning churn. Every meaningful change should make code compile, add behavior, add tests, or fix failures.

ROX Anchor is the Rust and Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.

The repo is intentionally small and implementation-first. The current goal is to build working Rust crates, tests, local proof validation, CLI inspection, coordinator/relayer/RPC evidence models, and the Anchor program in a balanced code-and-test cadence.

## Current build rule

Every phase should produce one or more of:

```text
- compiling Rust code
- deterministic validation behavior
- focused unit tests
- local CLI output
- local service/model behavior
- Anchor program code that compiles
```

Avoid broad planning docs, repeated scope disclaimers, or placeholder-only work.

## Repo layout

```text
crates/
  rox-anchor-core/          shared IDs, states, labels, errors, and domain types
  rox-anchor-proof/         deterministic local proof validation
  rox-anchor-cli/           local inspection commands
  rox-anchor-rpc-proof/     local RPC evidence and quorum model
  rox-anchor-coordinator/   local observation queue and review coordinator
  rox-anchor-relayer/       local relayer dry-run, retry, and receipt model

programs/
  rox-anchor/               Solana/Anchor program

tests/
  unit/                     focused behavior tests
  integration/              cross-crate flow tests
  chaos/                    adversarial/failure tests
```

## Build order

```text
Phase 0  — workspace recovery and compile spine
Phase 1  — rox-anchor-core shared types
Phase 2  — rox-anchor-proof validation engine
Phase 3  — focused unit tests
Phase 4  — rox-anchor-cli local inspection
Phase 5  — RPC proof evidence model
Phase 6  — coordinator local model
Phase 7  — relayer dry-run model
Phase 8  — Anchor program compile foundation
Phase 9  — Anchor state rules
Phase 10 — ROC ↔ ROX mint/burn logic
Phase 11 — integration tests
Phase 12 — chaos tests
Phase 13 — final local green run
```

## Development workflow

Use a QuickChain-style balance:

```text
1. Write the smallest useful Rust behavior.
2. Add focused tests for that behavior.
3. Run cargo fmt.
4. Run focused cargo test/check.
5. Fix failures before moving to the next file.
```

Preferred commands:

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

For focused work:

```bash
cargo check -p rox-anchor-core
cargo test -p rox-anchor-core

cargo check -p rox-anchor-proof
cargo test -p rox-anchor-proof
```

For the Anchor program:

```bash
cargo check -p rox-anchor
```

If Anchor CLI is installed:

```bash
anchor build
anchor test
```

## Current priority

Start with the compile spine:

```bash
cargo check --workspace
```

Then fix the first compiler failure.

After the workspace compiles, the next target is `rox-anchor-core`, then `rox-anchor-proof`, with tests added alongside each real behavior.
