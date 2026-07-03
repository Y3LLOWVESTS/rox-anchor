---
title: ROX Anchor IDB
version: 0.1.0
status: draft
last-updated: 2026-07-03
audience: contributors, operators, auditors
---

# ROX Anchor IDB

ROX Anchor is the Rust and Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.

This document defines the invariants for building the project as real software. It is not a substitute for code, tests, or compiler output.

The project should advance through a balanced QuickChain-style cadence:

```text
real Rust behavior
+ focused tests
+ compile checks
+ fixes before next phase
```

---

## 1. Invariants (MUST)

### [I-1] The repo must build toward real implementation

Every phase must produce at least one of:

```text
compiling Rust code
deterministic validation behavior
unit tests
integration tests
CLI output
local service/model behavior
Anchor program code that compiles
```

Placeholder-only work is not progress.

### [I-2] `rox-anchor-core` owns shared domain truth

Shared IDs, domains, directions, states, labels, and posture enums must live in `rox-anchor-core`.

Other crates must not invent competing versions of:

```text
operation IDs
idempotency keys
nonces
domains
clusters
program IDs
mints
token accounts
anchor directions
challenge states
halt states
recovery states
status labels
```

### [I-3] `rox-anchor-proof` owns deterministic local review

The proof crate must decide, from local inputs, whether a package is:

```text
valid for local review
incomplete
rejected
blocked by challenge
blocked by halt
blocked by recovery
```

The same input must always produce the same decision and findings.

### [I-4] Replay must be rejected

Reused nonce, operation ID, or idempotency binding must not pass review.

Replay rejection must be tested directly.

### [I-5] Binding mismatches must be rejected

The system must reject mismatched:

```text
source domain
target domain
direction
cluster
program ID
mint
token account
operation ID
idempotency key
nonce
```

### [I-6] Challenge, halt, and recovery states must block unsafe transitions

Open challenges, accepted challenges, halted states, and recovery-required states must prevent acceptance/finalization until resolved by explicit state transition code.

### [I-7] The CLI must expose real local behavior

`rox-anchor-cli` must call the proof engine and print deterministic review output.

The CLI should not fake success or print decorative status that is not backed by code.

### [I-8] Coordinator, RPC proof, and relayer crates must use the proof engine

Local service/model crates must call into `rox-anchor-core` and `rox-anchor-proof`.

They must not create separate acceptance rules.

### [I-9] The Anchor program owns on-chain state transitions

`programs/rox-anchor` must define the Solana/Anchor state machine for:

```text
initialize
observe burn
open challenge
resolve challenge
halt
recover
finalize
```

Anchor state transitions must be enforced in program code, not only in off-chain proof code.

### [I-10] Mint/burn behavior must be state-machine protected

ROC ↔ ROX mint/burn logic must enforce:

```text
correct mint binding
correct token account binding
replay protection
challenge window rules
halt rules
recovery rules
authority rules
event emission
```

### [I-11] Tests must grow alongside code

Every new behavior should have a focused test close to the implementation phase.

Preferred balance:

```text
small behavior patch
small test patch
cargo fmt
focused cargo test
fix failures
```

### [I-12] No hidden live behavior

Live wallet calls, live RPC submission, live deployment, production minting, production burning, or production settlement must be explicit code paths, not side effects hidden inside local validation, CLI inspection, tests, or dry-run models.

This invariant must not block local compile-tested Anchor program development.

---

## 2. Design Principles (SHOULD)

### [P-1] Build inside-out

Recommended order:

```text
core types
proof validation
unit tests
CLI
RPC evidence model
coordinator
relayer dry-run
Anchor compile foundation
Anchor state rules
mint/burn logic
integration tests
chaos tests
```

### [P-2] Prefer typed wrappers over raw strings

Use typed domain wrappers wherever possible.

Bad:

```rust
pub mint: String
```

Better:

```rust
pub mint: MintId
```

### [P-3] Keep validation deterministic

Avoid time, randomness, network calls, process calls, or environment reads in validation logic.

Time-sensitive rules should receive explicit input values.

### [P-4] Keep local proof logic separate from on-chain state mutation

The proof engine should review evidence.

The Anchor program should enforce on-chain state transitions.

The relayer/coordinator should connect reviewed evidence to later action.

### [P-5] Keep comments useful

Source comments should describe:

```text
what the code does
what input it validates
what errors/findings it returns
what tests prove it
```

Do not fill code files with repeated boilerplate disclaimers.

### [P-6] Favor boring Rust first

Use simple structs, enums, match statements, and deterministic helpers before adding heavy abstractions.

### [P-7] Tests should prove behavior, not wording

Tests should focus on decisions, findings, state transitions, replay rejection, and mismatch rejection.

---

## 3. Implementation (HOW)

### [C-1] Workspace compile first

Start every phase by keeping the workspace buildable.

```bash
cargo check --workspace
```

When the full workspace is noisy, use focused checks:

```bash
cargo check -p rox-anchor-core
cargo check -p rox-anchor-proof
cargo check -p rox-anchor-cli
cargo check -p rox-anchor-rpc-proof
cargo check -p rox-anchor-coordinator
cargo check -p rox-anchor-relayer
cargo check -p rox-anchor
```

### [C-2] Core crate API pattern

`rox-anchor-core/src/lib.rs` should re-export the shared API:

```rust
pub mod errors;
pub mod ids;
pub mod labels;
pub mod state;
pub mod types;

pub use errors::*;
pub use ids::*;
pub use labels::*;
pub use state::*;
pub use types::*;
```

### [C-3] Proof review pattern

Proof review should return a structured report:

```rust
pub struct ProofReview {
    pub decision: ProofDecision,
    pub findings: Vec<ProofFinding>,
}

pub enum ProofDecision {
    ValidForLocalReview,
    EvidenceIncomplete,
    Rejected,
    ChallengeBlocked,
    HaltBlocked,
    RecoveryBlocked,
}
```

### [C-4] Finding pattern

Findings should be explicit and testable:

```rust
pub enum ProofFinding {
    MissingField(&'static str),
    SourceDomainMismatch,
    TargetDomainMismatch,
    DirectionMismatch,
    ClusterMismatch,
    ProgramMismatch,
    MintMismatch,
    TokenAccountMismatch,
    ReplayDetected,
    QuorumDisputed,
    ChallengeOpen,
    Halted,
    RecoveryRequired,
}
```

### [C-5] CLI pattern

The first useful CLI behavior should be:

```bash
cargo run -p rox-anchor-cli -- check
```

Expected output should include:

```text
decision
findings
status label
```

### [C-6] Anchor instruction pattern

Each Anchor instruction file should own one instruction context and one handler.

Example shape:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    // accounts
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    // state transition
    Ok(())
}
```

### [C-7] Test placement pattern

Use crate-local tests first:

```text
crates/rox-anchor-core/tests/
crates/rox-anchor-proof/tests/
```

Use root integration/chaos tests only after the crate APIs are stable enough to call.

---

## 4. Acceptance Gates (PROOF)

These are proof steps, not paperwork gates.

### [G-1] Workspace compile

```bash
cargo check --workspace
```

### [G-2] Core tests

```bash
cargo test -p rox-anchor-core
```

Must prove:

```text
typed ID validation
state classification
status labels
shared posture enums
error behavior
```

### [G-3] Proof tests

```bash
cargo test -p rox-anchor-proof
```

Must prove:

```text
valid package accepted for local review
missing field rejected/incomplete
replay rejected
cluster mismatch rejected
program mismatch rejected
mint mismatch rejected
token account mismatch rejected
challenge-open blocks acceptance
halt blocks acceptance
recovery-required blocks acceptance
quorum disagreement classified
```

### [G-4] CLI smoke

```bash
cargo run -p rox-anchor-cli -- check
```

Must print a real review decision from the proof engine.

### [G-5] RPC proof tests

```bash
cargo test -p rox-anchor-rpc-proof
```

Must prove:

```text
agreement
disagreement
missing evidence
stale evidence
equivocation classification
```

### [G-6] Coordinator tests

```bash
cargo test -p rox-anchor-coordinator
```

Must prove:

```text
observation intake
queue behavior
duplicate evidence handling
stale evidence rejection
proof-engine handoff
```

### [G-7] Relayer tests

```bash
cargo test -p rox-anchor-relayer
```

Must prove:

```text
bounded retry
idempotent dry-run submission
receipt generation
redaction
```

### [G-8] Anchor compile

```bash
cargo check -p rox-anchor
```

If Anchor CLI is available:

```bash
anchor build
```

### [G-9] Anchor state tests

Must prove:

```text
initialize works
observe burn validates binding
replay is rejected
challenge opens only in allowed state
challenge resolution updates state
halt blocks sensitive transitions
recovery transitions are explicit
finalize only succeeds from eligible state
```

### [G-10] Final local green run

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

If Anchor CLI is available:

```bash
anchor build
anchor test
```

---

## 5. Anti-Scope (Forbidden)

The following are not allowed as hidden side effects of local work:

```text
live wallet calls from local proof validation
live ledger mutation from proof validation
live RPC submission from tests unless explicitly requested
production minting
production burning
production settlement
production deployment
exchange-facing behavior
staking behavior
liquidity behavior
fake success output
fake finality output
client-side authority over state transitions
duplicate state machines in different crates
```

The following are allowed and expected:

```text
compile-tested Anchor program code
local validator tests
local proof validation
local CLI inspection
local coordinator models
local RPC evidence models
local relayer dry-run models
unit tests
integration tests
chaos tests
```

---

## 6. References

Current repo reference files:

```text
README.md
TODO.md
BUILD_PLAN.md
Cargo.toml
Anchor.toml
crates/rox-anchor-core
crates/rox-anchor-proof
crates/rox-anchor-cli
crates/rox-anchor-rpc-proof
crates/rox-anchor-coordinator
crates/rox-anchor-relayer
programs/rox-anchor
tests
```

This IDB should stay short. If it starts becoming a second build plan, move the work back into code and tests.
