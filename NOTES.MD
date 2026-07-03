### BEGIN NOTE - JULY 3 2026 - 12:50 CST

Below is the carry-over note pack I would paste into the next ROX Anchor session. It assumes the new ROX Anchor project instructions are active, and it preserves the RustyOnions / CrabLink / QuickChain context without dragging us back into old docs-only paralysis.

---

# ROX Anchor — First Carry-Over Notes After Repo Restart

## 0. Session opener / safe status

We are continuing work on **RustyOnions / CrabLink / ROX Anchor**.

Current active workstream:

```text
ROX Anchor implementation.
```

RustyOnions is the backend/substrate/value-plane. CrabLink is the Tauri-first product/client for `crab://` links, b3 assets, RON Passports, internal ROC paid access, creator pages, receipts, and future UX surfaces. ROX Anchor is the Rust + Solana/Anchor implementation surface for the future ROC ↔ ROX anchor path.

Current ROX Anchor posture:

```text
active implementation work
not planning-only work
not docs-only gate work
not fake scaffold work
```

The repo has been reduced to the active Rust/Anchor build surface. The README says the repo is now an active work-in-progress built in phases with a QuickChain-style balance of real Rust code, focused tests, compile checks, and fixes before moving forward. It also states that every meaningful change should make code compile, add behavior, add tests, or fix failures. 

Immediate priority:

```text
1. Make cargo check --workspace pass.
2. Build rox-anchor-core.
3. Build rox-anchor-proof.
4. Add focused tests beside behavior.
5. Bring up rox-anchor-cli check.
6. Bring up local RPC proof / coordinator / relayer models.
7. Bring programs/rox-anchor into compile-tested Anchor shape.
```

The README’s build order is Phase 0 workspace recovery, Phase 1 core shared types, Phase 2 proof validation, Phase 3 tests, Phase 4 CLI, Phase 5 RPC proof, Phase 6 coordinator, Phase 7 relayer, Phase 8 Anchor compile foundation, Phase 9 Anchor state rules, Phase 10 ROC ↔ ROX mint/burn logic, Phase 11 integration tests, Phase 12 chaos tests, and Phase 13 final local green run. 

---

## 1. Brief note on the earlier jam-up

We had an earlier jam-up where the first ROX Anchor repo posture was too docs-only / decision-gate-heavy and included repeated “not authorized runtime” language. That created friction because we were trying to build software but the repo looked like a planning artifact.

We corrected that by:

```text
- cleaning the repo down to Rust/Anchor implementation surfaces
- adding a short README
- adding TODO.md
- adding IDB.md
- adding BUILD_PLAN.md
- creating project instructions that preserve safety without blocking local implementation
```

Move forward from that. Do not keep relitigating it. The right path now is:

```text
cargo check
first compiler failure
small patch
focused test/check
repeat
```

---

## 2. Source material reviewed / carried forward

The next session should treat these as the active ROX Anchor local reference files:

```text
README.md
TODO.md
IDB.md
BUILD_PLAN.md
Cargo.toml
Anchor.toml
```

The TODO says the repo has been reduced to the active Rust/Anchor implementation surface, and its priority list is: make the workspace compile, build `rox-anchor-core`, build `rox-anchor-proof`, bring the CLI online, bring coordinator/RPC proof/relayer online as local models, bring the Anchor program online, and add tests as each surface becomes real. 

RustyOnions / CrabLink / QuickChain context to carry forward:

```text
SESSION_PACK.md
INTERNAL_ROC_MODEL.md
QUICKCHAIN_BUILDPLAN.MD
ALLNOTES.MD
COMBINED_IDB.MD
CODEBUNDLE_RS.md
CODEBUNDLE_TAURI_APP.md
```

The session pack records that Internal ROC Phase 6 and the internal ROC beta value-loop proof are complete/green/parked, and that bridge work had previously been docs/threat-model/decision-gate only with no ROX/Solana/bridge/staking/liquidity/external settlement runtime authorized. 

Important interpretation:

```text
The old bridge docs-only posture remains valuable as threat model and boundary history.
The new ROX Anchor posture is implementation-first for local compile-tested Rust/Anchor work.
This still does not authorize live deployment, production mint/burn, public bridge runtime, exchange-facing behavior, staking, liquidity, or external settlement.
```

---

## 3. Project identity

### RustyOnions

RustyOnions is the substrate/value-plane. It owns service boundaries, wallet/ledger truth, internal ROC accounting, policy, storage, gateway, omnigate, index, reward planning, and future proof/checkpoint infrastructure.

Current relevant authority model:

```text
svc-wallet      = internal ROC mutation front-door
ron-ledger      = durable economic truth
ron-accounting  = snapshots/reports only
svc-rewarder    = capped planning only
ron-policy      = declarative gate/policy only
svc-storage     = b3 bytes/artifacts only
svc-index       = lookup/pointer/search only
svc-gateway     = ingress/proxy/admission boundary
omnigate        = hydration/access composition
CrabLink        = display/user intent/explicit confirmation
```

### CrabLink

CrabLink is the Tauri-first client/product surface. It may display backend-derived receipts, balances, paid access status, proof status, and future bridge status, but it must not invent balances, receipts, finality, settlement, paid unlocks, or bridge completion.

CrabLink remains display/user-intent only. The Phase 6 closeout says the internal value loop proved CrabLink user intent → quote/prepare → explicit confirmation → svc-wallet → ron-ledger receipt/balance truth → backend paid enforcement → CrabLink display-only receipt/balance UX → replay/conservation proof → accounting/rewarder/policy paths without creating new runtime authority. 

### ROX Anchor

ROX Anchor is the new isolated implementation surface for the ROC ↔ ROX anchor path. It should build:

```text
shared core types
deterministic local proof validation
CLI inspection
RPC evidence/quorum model
coordinator local model
relayer dry-run model
Solana/Anchor program code
Anchor state rules
eventual local mint/burn state-machine tests
integration and chaos tests
```

---

## 4. Current ROX Anchor repo shape

The current TODO file records this repo shape:

```text
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
    rox-anchor-coordinator/
    rox-anchor-core/
    rox-anchor-proof/
    rox-anchor-relayer/
    rox-anchor-rpc-proof/

  programs/
    rox-anchor/

  tests/
    chaos/
    integration/
    unit/
```

The TODO also lists core file ownership: `rox-anchor-core` should own shared IDs, states, labels, errors, and domain concepts; `rox-anchor-proof` should validate proof packages, reject replay, detect domain/cluster/program/mint mismatches, classify challenge/recovery/halt posture, and return review decisions. 

Important current file roles:

```text
crates/rox-anchor-core
  shared source of truth for IDs, domains, nonces, directions, states, labels, errors

crates/rox-anchor-proof
  deterministic local review engine

crates/rox-anchor-cli
  terminal inspection tool, starting with check

crates/rox-anchor-rpc-proof
  local RPC observation/evidence/quorum model

crates/rox-anchor-coordinator
  local observation queue and review coordinator

crates/rox-anchor-relayer
  local dry-run submission/retry/receipt model

programs/rox-anchor
  Solana/Anchor program surface

tests/unit
  focused behavior tests

tests/integration
  cross-crate flow tests

tests/chaos
  adversarial/failure-mode tests
```

Important caution:

```text
If the root Cargo.toml is a virtual workspace only, root-level tests/ are not automatically active as normal Cargo integration tests.
Early tests should be crate-local:
  crates/rox-anchor-core/tests/
  crates/rox-anchor-proof/tests/
  crates/rox-anchor-cli/tests/
Then root integration/chaos tests can be activated later via a real test harness strategy.
```

---

## 5. Active build doctrine

Use the QuickChain rhythm that worked:

```text
small real behavior
focused test
cargo fmt
focused cargo test/check
fix first failure
only then expand
```

Do not write broad docs unless explicitly requested. We already have enough docs to begin. The README says to start with `cargo check --workspace`, then fix the first compiler failure, and after the workspace compiles the next target is `rox-anchor-core`, then `rox-anchor-proof`, with tests added alongside each real behavior. 

Default command sequence for the next session:

```bash
cd /Users/mymac/Desktop/rox-anchor
cargo check --workspace
```

Then paste the first compiler error. Fix one failure at a time.

If the workspace is too noisy:

```bash
cargo check -p rox-anchor-core
cargo test -p rox-anchor-core

cargo check -p rox-anchor-proof
cargo test -p rox-anchor-proof

cargo check -p rox-anchor-cli
cargo run -p rox-anchor-cli -- check

cargo check -p rox-anchor
```

If Anchor CLI is installed:

```bash
anchor build
anchor test
```

Do not start with `anchor build` until the ordinary Cargo workspace and program crate manifest are sane.

---

## 6. Phase-by-phase build notes

## Phase 0 — Workspace recovery and compile spine

Goal:

```text
cargo check --workspace passes
```

Likely first issues:

```text
root Cargo.toml missing workspace members
root Cargo.toml still excluding crates/program
crate manifests missing lib/bin targets
binary crates missing main.rs wiring
programs/rox-anchor manifest not compatible with current workspace
old docs-only comments or placeholders causing compile errors
root tests inert because workspace root has no package
```

Expected root workspace members:

```text
crates/rox-anchor-core
crates/rox-anchor-proof
crates/rox-anchor-cli
crates/rox-anchor-coordinator
crates/rox-anchor-rpc-proof
crates/rox-anchor-relayer
programs/rox-anchor
```

Do not add large dependency stacks yet. Phase 0 should only repair compile spine.

Phase 0 exit:

```text
cargo check --workspace is green
or at least reaches real source-code errors instead of manifest/workspace errors, then those source-code errors are fixed.
```

## Phase 1 — `rox-anchor-core`

Goal:

```text
core owns all shared language
```

Must define or centralize:

```text
AnchorDomain
OperationId
IdempotencyKey
Nonce
ClusterId
ProgramId
MintId
TokenAccountId
AccountId if needed
AnchorDirection
AnchorState
ChallengePosture
HaltPosture
RecoveryPosture
status labels
shared errors
```

Rules:

```text
No duplicate direction enum in proof/coordinator/relayer.
No duplicate challenge/halt/recovery state enums outside core unless they wrap core types.
Core should be dependency-light.
Core should have focused unit tests.
```

Core test ideas:

```text
empty ID rejected
whitespace ID rejected
valid IDs accepted
direction labels stable
state helper methods stable
challenge/halt/recovery posture methods stable
safe labels stable
```

Commands:

```bash
cargo fmt -p rox-anchor-core
cargo test -p rox-anchor-core
```

## Phase 2 — `rox-anchor-proof`

Goal:

```text
deterministic local proof review
```

Must validate:

```text
required fields
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
evidence posture
quorum posture
challenge posture
halt posture
recovery posture
replay posture
```

Decision set should be deterministic:

```text
ValidForLocalReview
EvidenceIncomplete
Rejected
ChallengeBlocked
HaltBlocked
RecoveryBlocked
```

Finding set should be explicit:

```text
MissingField
SourceDomainMismatch
TargetDomainMismatch
DirectionMismatch
ClusterMismatch
ProgramMismatch
MintMismatch
TokenAccountMismatch
OperationIdMismatch
IdempotencyKeyMismatch
NonceMismatch
ReplayDetected
EvidenceIncomplete
QuorumDisputed
ChallengeOpen
ChallengeAccepted
Halted
RecoveryRequired
```

Proof tests:

```text
valid package accepted
missing field incomplete/rejected
source/target domain mismatch rejected
direction mismatch rejected
cluster mismatch rejected
program ID mismatch rejected
mint mismatch rejected
token account mismatch rejected
operation mismatch rejected
idempotency mismatch rejected
nonce mismatch rejected
replay rejected
quorum disagreement blocked/rejected
challenge open blocks acceptance
halt blocks acceptance
recovery required blocks acceptance
same input produces same findings/order
```

Commands:

```bash
cargo fmt -p rox-anchor-proof
cargo test -p rox-anchor-proof
```

## Phase 3 — Test lift

Goal:

```text
move from placeholder root tests to active crate-local tests
```

Priority:

```text
crates/rox-anchor-core/tests/core_ids.rs
crates/rox-anchor-core/tests/state_labels.rs
crates/rox-anchor-proof/tests/proof_package_validation.rs
crates/rox-anchor-proof/tests/replay_rejection.rs
crates/rox-anchor-proof/tests/challenge_halt_recovery.rs
```

Do not overbuild fixture infrastructure. Since JSON vectors were removed, use small in-code deterministic fixtures in `rox-anchor-proof/src/fixtures.rs` first.

## Phase 4 — CLI

Goal:

```text
cargo run -p rox-anchor-cli -- check
```

Must call real proof code. It should not print fake success.

Minimum useful output:

```text
decision: ValidForLocalReview
status_label: ...
findings:
  - ...
```

Later add JSON input support. First version can use an in-code fixture from the proof crate.

Commands:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli
cargo run -p rox-anchor-cli -- check
```

## Phase 5 — RPC proof model

Goal:

```text
model RPC observations locally
```

No live RPC calls yet unless explicitly authorized later.

Must model:

```text
source name
cluster
slot/height if needed
commitment
signature
program id
mint
token account
observed event
observed amount
observed operation/nonce binding
```

Must classify:

```text
agreement
disagreement
missing evidence
stale evidence
equivocation
insufficient commitment
wrong cluster
wrong program
wrong mint
wrong token account
```

## Phase 6 — Coordinator model

Goal:

```text
local coordinator queues observations and calls rox-anchor-proof
```

Must not create new acceptance rules. Coordinator is orchestration, not truth.

Needs:

```text
config
observer records
queue item
decision wrapper around proof review
readiness
redaction
duplicate/stale evidence tests
```

## Phase 7 — Relayer dry-run model

Goal:

```text
dry-run submission/retry/receipt behavior without live submission
```

No live RPC submission yet.

Needs:

```text
submission request
retry policy
bounded retries
idempotency
receipt record
redacted report
retry storm tests
```

## Phase 8 — Anchor program compile foundation

Goal:

```text
programs/rox-anchor compiles as Anchor program
```

Before choosing Anchor dependency versions, capture local toolchain:

```bash
anchor --version || true
solana --version || true
rustc --version
cargo --version
```

Program files:

```text
programs/rox-anchor/src/lib.rs
programs/rox-anchor/src/errors.rs
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/initialize.rs
programs/rox-anchor/src/instructions/observe_burn.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/instructions/open_challenge.rs
programs/rox-anchor/src/instructions/resolve_challenge.rs
programs/rox-anchor/src/instructions/halt.rs
programs/rox-anchor/src/instructions/recover.rs
```

Keep first Anchor pass compile-focused:

```text
valid manifest
declare program id
account structs compile
error enum compiles
event structs compile
instruction contexts compile
handlers compile
no real value-bearing deployment
```

## Phase 9 — Anchor state rules

Implement state transitions:

```text
initialize
observe burn
open challenge
resolve challenge
halt
recover
finalize
```

Must reject:

```text
wrong domain
wrong mint
wrong program binding
wrong token account
replay
finalize while challenged
finalize while halted
finalize while recovery required
invalid transition
```

## Phase 10 — ROC ↔ ROX mint/burn logic

This is later, after state rules compile and tests exist.

Model:

```text
ROC → ROX:
  user explicit intent
  internal ROC burn proof
  proof package
  challenge window
  quorum/commitment review
  Anchor mint only after eligible state

ROX → ROC:
  external ROX burn observation
  proof package
  challenge window
  quorum/commitment review
  internal ROC issue path must route through svc-wallet/ron-ledger
```

Do not implement live production mint/burn or bridge settlement during early phases.

---

## 7. Core invariants to preserve

ROX Anchor invariants:

```text
rox-anchor-core owns shared IDs, domains, directions, states, labels, and postures.
rox-anchor-proof owns deterministic local proof review.
Replay must be rejected.
Mismatches must be rejected.
Challenge, halt, and recovery states block unsafe acceptance/finalization.
CLI output must come from real proof/core behavior.
Coordinator/RPC proof/relayer must call core/proof instead of inventing rules.
Anchor program owns on-chain state transitions.
Tests grow beside behavior.
No hidden live behavior.
```

Binding mismatch rejection must include:

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

Bridge/proof risks to carry from previous docs:

```text
double mint
double internal issue
replay
RPC equivocation
cluster confusion
program ID confusion
mint confusion
token account confusion
coordinator compromise
relayer retry storm
challenge griefing
halt/recovery stuck states
stale UI/status language
upgrade authority compromise
CPI/token account validation errors
```

The bridge decision material specifically calls out mandatory future bridge fixes such as cross-domain nonce binding, multi-RPC/quorum proof model, minimum Solana commitment policy, cluster/program/mint/direction/operation binding, transaction signature binding, challenge-window mitigation, halt semantics, coordinator compromise drills, key rotation, upgrade authority policy, stuck challenge recovery, Anchor account constraints, and stale/offline CrabLink language. 

---

## 8. RustyOnions / QuickChain lessons to reuse

QuickChain worked because it used:

```text
one crate pair / surface at a time
small focused patches
strict boundary tests
park labels when complete
real code plus tests
no fake receipts
no fake finality
no client authority
no broad reruns unless needed
```

QuickChain doctrine to carry forward:

```text
determinism before distribution
DTOs before roots
roots before validators
proofs before pruning
internal ROC before external anchors
```

QuickChain Phase 5 Round 1 was anchor-only dry-run. It tested compact external commitments without external settlement, and its exit gate was that an external anchor can prove timestamp/commitment only, cannot mutate ROC balances, and cannot replace wallet/ledger truth. 

For ROX Anchor, translate that into:

```text
local proof before relayer
relayer dry-run before live submission
state machine before mint/burn
challenge/halt/recovery before finalization
local validator before devnet
audit/recovery drills before any value-bearing run
```

---

## 9. Internal ROC doctrine to preserve

Internal ROC tokenomics doctrine:

```text
ROC remains internal until explicitly extended by reviewed later phases.
svc-wallet remains mutation front-door.
ron-ledger remains durable economic truth.
raw engagement does not directly mint or allocate protocol ROC.
all mutable tokenomics values live in configs/roc-economics.toml.
no fake receipts, fake balances, fake finality, or silent spend.
```

Internal ROC value loop:

```text
event classification
→ ron-accounting snapshot/report
→ svc-rewarder capped payout planning
→ ron-policy validation/gating
→ svc-wallet approved mutation
→ ron-ledger durable receipt
```

ROX Anchor must not bypass this.

For future ROX → ROC, any internal ROC issue path must eventually route through:

```text
svc-wallet
→ ron-ledger
```

not directly from:

```text
Anchor program
relayer
coordinator
RPC proof
CrabLink
gateway
omnigate
policy
accounting
rewarder
```

---

## 10. CrabLink future integration notes

CrabLink is not active in the first ROX Anchor coding phases, but future status display must follow these rules:

```text
CrabLink displays backend-derived bridge/proof status.
CrabLink does not decide bridge finality.
CrabLink does not unlock paid content from anchor evidence.
CrabLink does not create receipts.
CrabLink does not create balances.
CrabLink cache/localStorage/sessionStorage/IndexedDB are never paid access truth.
CrabLink must show pending/challenge/not-final status honestly.
```

Allowed future UI language:

```text
planned bridge
experimental bridge design
burn request
external mint request
pending verification
pending challenge window
not final
delayed
manual review may be required
```

Forbidden future UI language:

```text
convert
exchange
trade
swap
instant
guaranteed
cash out
redeem guaranteed value
risk-free
final before proof/challenge close
```

The session pack explicitly says bridge planning is allowed as docs, decision gates, threat models, DTO sketches, state-machine design, test plans, review prompts, and inert/off-by-default configuration placeholders, but not runtime activation, Solana deployment, ROX live token launch, mint/burn execution, external settlement, liquidity, staking, exchange support, or public validator economics. 

For the new ROX Anchor repo, this becomes:

```text
local compile-tested code is allowed now
local proof validation is allowed now
local service/dry-run models are allowed now
compile-tested Anchor program code is allowed now
live bridge runtime is still not allowed
production deployment is still not allowed
value-bearing devnet is still not allowed
```

---

## 11. Test strategy

Test pyramid for ROX Anchor:

```text
core unit tests
proof unit tests
CLI smoke tests
RPC proof unit tests
coordinator unit tests
relayer unit tests
Anchor program tests
integration tests
chaos tests
local validator tests
```

Start small:

```text
Phase 1:
  core ID/state/label tests

Phase 2:
  proof validation/replay/mismatch tests

Phase 3:
  active crate-local tests replacing inert root tests

Phase 4:
  CLI check smoke test

Phase 5–7:
  local model tests for RPC proof/coordinator/relayer

Phase 8–10:
  Anchor compile and state tests

Phase 11–12:
  integration + chaos tests
```

Important test principles:

```text
test decisions, not copy text
assert deterministic finding order where useful
reject unknown/wrong bindings
reject replay
never fake finality
never treat a stub as proof
```

---

## 12. Patch style for future sessions

User preference:

```text
paste-ready terminal patches
wrap all patches in ~~~~bash fences
prefer complete file writes or clear scripted edits
no downloads
no git add/commit/push unless explicitly requested
after each patch, include exact focused commands to run next
```

Do not use giant patches unless needed. The next session should usually do:

```text
1. Ask for / inspect cargo check output.
2. Patch the exact files needed.
3. Run focused check.
4. Move to next failure.
```

Avoid:

```text
broad architecture essays
new docs unless requested
decision-gate churn
placeholder-only crates
repeating safety disclaimers inside every source file
workspace clippy too early
Anchor dependency guessing without checking local Anchor tooling
```

---

## 13. Next session attachments

Minimum attachments for next session:

```text
README.md
TODO.md
IDB.md
BUILD_PLAN.md
latest ROX Anchor terminal output from cargo check --workspace
```

Recommended if code patching starts:

```text
latest ROX Anchor CODEBUNDLE.md
or focused pasted file contents for the failing crate
```

Optional context if the assistant seems to lose project history:

```text
SESSION_PACK.md
INTERNAL_ROC_MODEL.md
QUICKCHAIN_BUILDPLAN.MD
ALLNOTES.MD
COMBINED_IDB.MD
CODEBUNDLE_RS.md
CODEBUNDLE_TAURI_APP.md
```

But do not overload every coding session. Once we are actively fixing `rox-anchor-core`, a focused codebundle or pasted files are better than reattaching all of RustyOnions.

---

## 14. Exact next-session first move

Start with:

```bash
cd /Users/mymac/Desktop/rox-anchor
cargo check --workspace
```

Then paste:

```text
the first compiler error
the surrounding 20–40 lines if needed
the current root Cargo.toml if the error is workspace/member related
the failing crate Cargo.toml if the error is manifest related
```

Most efficient next patch likely falls into one of these buckets:

```text
root workspace members/excludes
crate manifest repair
missing lib/bin target
old placeholder compile failure
core/proof duplicate type repair
Anchor program dependency/manifest conflict
```

Do not jump ahead to Anchor mint/burn before the workspace and core/proof are green.

---

## 15. First coding milestone definition

The first real milestone is:

```text
ROX Anchor Phase 0 COMPLETE:
cargo check --workspace passes.
```

The second milestone is:

```text
ROX Anchor Phase 1 COMPLETE:
cargo test -p rox-anchor-core passes with real shared ID/state/label tests.
```

The third milestone is:

```text
ROX Anchor Phase 2 COMPLETE:
cargo test -p rox-anchor-proof passes with valid/replay/mismatch/challenge/halt/recovery tests.
```

The first meaningful demo milestone is:

```text
cargo run -p rox-anchor-cli -- check
```

That command should print a real proof review decision from `rox-anchor-proof`.

---

## 16. Working mental model

Build ROX Anchor like this:

```text
core vocabulary
→ deterministic proof review
→ tests
→ CLI
→ local evidence models
→ coordinator
→ relayer dry-run
→ Anchor program compile
→ Anchor state rules
→ mint/burn state machine
→ integration
→ chaos
→ local green run
```

Do not build it like this:

```text
docs
→ more docs
→ forbidden-scope scanners
→ fake skeletons
→ fake green labels
→ no code
```

The target is a small, real, testable bridge/anchor subsystem. The safety model stays, but the work is now code-first.


### END NOTE - JULY 3 2026 - 12:50 CST


### BEGIN NOTE - JULY 3 2026 - 15:00 CST

Below is the carry-over note pack for the next ROX Anchor session.

The attached codebundle shows the current repo at `/Users/mymac/Desktop/rox-anchor`, dirty Git state, 94 files, and the active workspace/file tree with all six Rust crates plus `programs/rox-anchor` present.  The attached build plan confirms the active order: Phase 0 workspace recovery through Phase 13 final green run, with Phase 10 next for ROX/ROC mint-burn logic after Anchor state rules. 

---

# ROX Anchor Carry-Over Notes — July 3, 2026

## 0. Next-session opener

We are continuing active implementation work on **RustyOnions / CrabLink / ROX Anchor**.

Current project:

```text
RustyOnions = backend/substrate/value-plane.
CrabLink = Tauri-first product/client for crab:// links, b3 assets, RON Passports, internal ROC paid access, creator pages, receipts, and future UX surfaces.
ROX Anchor = Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.
```

Current posture:

```text
Implementation-first.
QuickChain-style code/test/check/fix loop.
No docs-only gate work.
No placeholder-only scaffold theater.
No live bridge runtime.
No production mint/burn.
No live wallet calls.
No live RPC submission.
No production deployment.
```

The next assistant should start by checking the current terminal state and then continue from the first real compiler/test failure, not by re-planning the project.

---

## 1. Attached files for next session

Use these as the active references:

```text
CODEBUNDLE.md
BUILD_PLAN.md
latest terminal output
```

The codebundle is the current source snapshot. The build plan is the phase authority. The source of truth remains the local repo, not the generated bundle.

Important current repo shape from the codebundle:

```text
crates/
  rox-anchor-core/
  rox-anchor-proof/
  rox-anchor-cli/
  rox-anchor-rpc-proof/
  rox-anchor-coordinator/
  rox-anchor-relayer/

programs/
  rox-anchor/

tests/
  unit/
  integration/
  chaos/
```

The codebundle also records the root workspace members and the Anchor program file layout. 

---

## 2. Build doctrine to preserve

The build plan’s rules remain active:

```text
Write Rust code.
Run focused compile/tests.
Fix failures before moving forward.
Do not add broad planning docs or decision gates.
Keep comments about what code does, validates, rejects, and returns.
Do not add live deployment, live wallet calls, live RPC submission, or real value movement until local program/tests are compile-green.
```

This is an implementation repo. Every patch should do one or more of:

```text
compile a crate
add real behavior
add focused tests
fix a compiler/test failure
reduce warning noise without hiding real errors
```

Do not drift back into broad bridge essays, decision gates, or placeholder-only files.

---

## 3. Current milestone status

### Phase 0 — Workspace recovery and compile spine

Status:

```text
COMPLETE / GREEN
```

What was accomplished:

```text
- Root Cargo workspace is active.
- Workspace includes:
  - rox-anchor-core
  - rox-anchor-proof
  - rox-anchor-cli
  - rox-anchor-rpc-proof
  - rox-anchor-coordinator
  - rox-anchor-relayer
  - programs/rox-anchor
- Cargo.lock exists and includes Anchor/Solana dependencies after the Anchor program was brought online.
- cargo check --workspace reached green repeatedly during later phases.
```

The root workspace manifest in the codebundle includes all expected crates and the Anchor program member. 

---

### Phase 1 — `rox-anchor-core`

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
crates/rox-anchor-core
  src/errors.rs
  src/ids.rs
  src/labels.rs
  src/lib.rs
  src/state.rs
  src/types.rs
```

Responsibilities now owned by core:

```text
typed IDs
domains
operation IDs
idempotency keys
nonces
cluster IDs
program IDs
mint IDs
token-account IDs
directions
lifecycle states
challenge posture
halt posture
recovery posture
safe status labels
reusable core errors
```

Important invariant:

```text
Other crates must not invent duplicate state machines, duplicate direction enums, duplicate labels, or duplicate acceptance rules.
```

Current proof from codebundle:

```text
- `rox-anchor-core/src/lib.rs` re-exports errors, ids, labels, state, and types.
- Core tests cover identifier rejection/acceptance, lifecycle blockers, and label mapping.
- Labels include safe display strings such as Draft, Requested, Observed, Challenge Open, Halted, Recovery Required, Finality Eligible, Finalized, Failed, and Abandoned.
```

The codebundle shows `rox-anchor-core` owning shared labels and re-exporting the public API. 

---

### Phase 2 — `rox-anchor-proof`

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
crates/rox-anchor-proof
  src/package.rs
  src/validate.rs
  src/replay.rs
  src/quorum.rs
  src/challenge.rs
  src/recovery.rs
  src/fixtures.rs
  src/lib.rs
```

Proof engine behavior now exists:

```text
valid proof review
source domain mismatch rejection
target domain mismatch rejection
direction mismatch rejection
cluster mismatch rejection
program ID mismatch rejection
mint mismatch rejection
token account mismatch rejection
operation ID mismatch rejection
idempotency key mismatch rejection
nonce mismatch rejection
replay rejection
missing evidence classification
quorum dispute classification
challenge-open / challenge-accepted blocking
halt blocking
recovery-required / recovery-in-review / recovery-rejected blocking
deterministic finding/decision output
```

Important current terms:

```text
ReviewDecision::Accepted
ReviewDecision::Blocked
ReviewDecision::Rejected

ProofFindingCode::PackageAccepted
ProofFindingCode::ReplayOperationId
ProofFindingCode::ReplayIdempotencyKey
ProofFindingCode::ReplayNonce
ProofFindingCode::EvidenceMissing
ProofFindingCode::QuorumDisputed
ProofFindingCode::ChallengeOpen
ProofFindingCode::Halted
ProofFindingCode::RecoveryRequired
...
```

The proof crate is now the deterministic local review engine. Coordinator, CLI, RPC proof, and relayer surfaces should keep using it instead of inventing new rules.

---

### Phase 3 — Focused unit test lift

Status:

```text
COMPLETE ENOUGH FOR CURRENT PHASES
```

What changed:

```text
- Core/proof tests live beside implementation.
- CLI has its own command tests.
- RPC proof, coordinator, relayer, and Anchor program have crate-local tests.
- Root tests under tests/unit, tests/integration, and tests/chaos still exist as future surfaces, but the root workspace is virtual, so early active tests are crate-local.
```

Important caution:

```text
The root-level tests/ tree is present but should not be assumed to run as normal integration tests until we intentionally add a harness package or move tests into crate-level test dirs.
```

Future Phase 11/12 work should either activate those root tests through a real workspace test harness or move focused tests into crate-local `tests/` folders.

---

### Phase 4 — `rox-anchor-cli`

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
crates/rox-anchor-cli
  src/main.rs
  src/lib.rs
  src/commands/check.rs
  src/commands/proof.rs
  src/commands/status.rs
  src/commands/halt.rs
  src/commands/recover.rs
  tests/check_command.rs
```

Current behavior:

```text
cargo run -p rox-anchor-cli -- check
```

prints a real local proof review from `rox-anchor-proof`, not fake success.

CLI fixtures:

```text
--fixture valid
--fixture mismatch
--fixture replay
--fixture missing-evidence
--fixture disputed
--fixture challenge
--fixture halt
--fixture recovery
```

CLI output includes:

```text
command
fixture
direction
source_domain
target_domain
cluster
program_id
mint
token_account
operation_id
idempotency_key
nonce
decision
status_label
findings
```

Important prior fix:

```text
run_from_args(["rox-anchor"]) no longer panics. It defaults safely to check.
```

Current next CLI improvement, later:

```text
Add JSON proof package input support after local integration/chaos surfaces are stable.
```

Do not add live RPC or wallet behavior to the CLI.

---

### Phase 5 — `rox-anchor-rpc-proof`

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
crates/rox-anchor-rpc-proof
  src/commitment.rs
  src/config.rs
  src/rpc.rs
  src/quorum.rs
  src/readiness.rs
  src/redaction.rs
  src/main.rs
  src/lib.rs
```

Current behavior:

```text
local RPC observation model
commitment classification
quorum agreement
missing evidence
disputed/conflicting signatures
stale observation rejection
binding mismatch rejection
redacted signature/report shape
readiness review
conversion into proof evidence bundle
```

Known green run from this session:

```text
cargo test -p rox-anchor-rpc-proof
7 passed
cargo run -p rox-anchor-rpc-proof
rox-anchor-rpc-proof local evidence model: ready=true
cargo check --workspace
green
```

Important invariant:

```text
RPC proof remains local evidence modeling only. It does not make live network calls.
```

---

### Phase 6 — `rox-anchor-coordinator`

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
crates/rox-anchor-coordinator
  src/config.rs
  src/observer.rs
  src/decision.rs
  src/queue.rs
  src/readiness.rs
  src/redaction.rs
  src/main.rs
  src/lib.rs
```

Current behavior:

```text
CoordinatorConfig wraps RPC proof settings and queue capacity.
CoordinatorObservation wraps RPC observations.
CoordinatorQueue rejects duplicate operation IDs and queue capacity overflow.
CoordinatorDecision runs RPC proof review, maps RPC evidence into a proof EvidenceBundle, then calls rox-anchor-proof.
CoordinatorReadiness rejects unusable queue/RPC configs.
Redacted coordinator reports preserve decision shape without exposing raw internals.
```

Known green run from this session:

```text
cargo test -p rox-anchor-coordinator
6 passed
cargo run -p rox-anchor-coordinator
rox-anchor-coordinator local model: ready=true
cargo check --workspace
green
```

Important invariant:

```text
Coordinator is orchestration only. It does not invent acceptance rules and must keep calling rox-anchor-proof.
```

---

### Phase 7 — `rox-anchor-relayer`

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
crates/rox-anchor-relayer
  src/config.rs
  src/retry.rs
  src/receipts.rs
  src/submit.rs
  src/readiness.rs
  src/redaction.rs
  src/main.rs
  src/lib.rs
```

Current behavior:

```text
RelayerConfig controls max attempts and receipt capacity.
RetryPolicy caps requested attempts.
RelayerSubmissionRequest carries operation ID, idempotency key, target, and proof review.
RelayerDryRun accepts only ReviewDecision::Accepted.
Blocked/rejected proof reviews produce local receipts with 0 attempts.
Duplicate idempotency keys produce DuplicateRequest.
Receipt capacity is enforced.
Receipts are local dry-run receipts and explicitly have live_submission=false.
Readiness rejects zero retry/receipt limits.
Redacted reports keep status, operation, target, proof decision, attempts, and live_submission posture.
```

Known green run from this session:

```text
cargo test -p rox-anchor-relayer
6 passed
cargo run -p rox-anchor-relayer
rox-anchor-relayer dry-run model: ready=true
cargo check --workspace
green
```

Important invariant:

```text
Relayer is still dry-run only. No live RPC submission or transaction submission exists.
```

---

### Phase 8 — Anchor program compile foundation

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
programs/rox-anchor/Cargo.toml
programs/rox-anchor/src/lib.rs
programs/rox-anchor/src/errors.rs
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/
  initialize.rs
  observe_burn.rs
  open_challenge.rs
  resolve_challenge.rs
  halt.rs
  recover.rs
  finalize.rs
```

Key Anchor compile work completed:

```text
- Added Anchor dependency.
- Added program crate type.
- Added placeholder program ID.
- Added account state structs.
- Added program errors.
- Added events.
- Added instruction modules and handlers.
- Fixed Anchor macro account context issue by using root-level re-exports.
- Fixed generated `__client_accounts_*` visibility by making bridge exports pub(crate).
- Compiled with cargo check -p rox-anchor.
```

Important final warning cleanup patch provided at the end of the session:

```rust
#![allow(unexpected_cfgs)]
#![forbid(unsafe_code)]

#[allow(deprecated)]
#[program]
pub mod rox_anchor { ... }
```

Reason:

```text
Anchor/Solana macros generated `unexpected_cfgs` warnings and one macro-generated deprecated realloc warning. The code itself was green. The warning cleanup patch suppresses macro-stack noise without suppressing ordinary Rust warnings globally.
```

Next session should verify whether that warning-cleanup patch was actually run and whether `cargo check -p rox-anchor` is now clean.

---

### Phase 9 — Anchor program state rules

Status:

```text
COMPLETE / GREEN
```

What was built:

```text
RoxAnchorConfig
RoxAnchorOperation
OperationBindingArgs
OperationStateCode
AnchorTransferDirection
state-rule helper methods
handler calls into helpers instead of open-coding transitions
13 Anchor program tests passing before the warning cleanup patch
```

Known green terminal state before warning cleanup:

```text
cargo check -p rox-anchor: green with Anchor macro warnings
cargo test -p rox-anchor: 13 passed
cargo check --workspace: green with Anchor macro warnings
```

The terminal output showed 13 passing `rox-anchor` tests and a final successful workspace check, with only Anchor/Solana macro warnings remaining. 

State helper behavior now includes:

```text
config.initialize(authority)
config.require_authority(authority)
config.require_observation_open()
config.halt(authority)
config.recover(authority)

operation.initialize(authority, args)
operation.state_code()
operation.direction_code()
operation.require_not_finalized()
operation.require_binding(args)
operation.open_challenge()
operation.resolve_challenge(accepted)
operation.mark_recovery_required()
operation.recover()
operation.can_finalize(config)
operation.finalize(config)
```

State blockers implemented:

```text
halted config blocks observation/finalization
config recovery_required blocks observation/finalization
operation challenge_open blocks finalization
operation recovery_required blocks finalization
accepted challenge blocks finalization
finalized operation cannot be reopened/challenged/recovered
wrong authority cannot halt or recover config
invalid/corrupt direction blocks finalization
zero amount blocks finalization
zero burn evidence hash blocks finalization
```

The codebundle shows `RoxAnchorOperation` carrying authority, operation hash, mint, token account, direction, amount atoms, burn evidence hash, state, challenge flag, and recovery flag. 

---

### Phase 10 — ROX / ROC mint-burn program logic

Status:

```text
STARTED / ROUND 1 LOCAL BINDING SEMANTICS LANDED / STILL NOT COMPLETE
```

What was added in Phase 10 Round 1:

```text
OperationBindingArgs now includes:
  operation_id_hash: [u8; 32]
  direction: AnchorTransferDirection
  mint: Pubkey
  token_account: Pubkey
  amount_atoms: u64
  burn_evidence_hash: [u8; 32]

AnchorTransferDirection:
  RocToRox
  RoxToRoc

RoxAnchorOperation now stores:
  direction: u8
  amount_atoms: u64
  burn_evidence_hash: [u8; 32]
```

Validation now rejects:

```text
zero operation_id_hash
default Pubkey mint
default Pubkey token_account
zero amount_atoms
zero burn_evidence_hash
direction mismatch
mint mismatch
token account mismatch
amount mismatch
burn evidence mismatch
```

Finalization now also requires:

```text
known direction
amount_atoms > 0
burn_evidence_hash != [0; 32]
eligible state:
  Observed
  ChallengeRejected
  RecoveryResolved
```

The current `OperationBindingArgs` and its validation require nonzero operation hash, non-default mint/token account, positive amount, and nonzero burn evidence hash.  The current `can_finalize` helper requires clear halt/recovery/challenge blockers plus known direction, positive amount, nonzero burn evidence, and eligible operation state. 

Important limitation:

```text
This is still local state-machine binding semantics only.
There is no SPL token CPI yet.
There is no mint authority PDA model yet.
There is no token account ownership validation yet.
There is no local validator test yet.
There is no live value movement.
```

---

## 4. Most recent terminal state

Last terminal output before the requested carry-over notes showed:

```text
cargo check -p rox-anchor: finished
cargo test -p rox-anchor: 13 passed
cargo check --workspace: finished
warnings only
```

The warnings were:

```text
unexpected cfg condition value: custom-heap
unexpected cfg condition value: custom-panic
unexpected cfg condition value: solana
unexpected cfg condition value: anchor-debug
deprecated AccountInfo::realloc generated by Anchor macro
```

These warnings originate from Anchor/Solana macros around `#[program]` and `#[derive(Accounts)]`, not from our state logic. A cleanup patch was provided to add:

```rust
#![allow(unexpected_cfgs)]
#[allow(deprecated)]
```

to the program root. Next session should verify that patch was applied and that warnings are cleaned.

---

## 5. Exact next-session first move

Start here:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo check -p rox-anchor
cargo test -p rox-anchor
cargo check --workspace
```

Interpretation:

```text
If this is clean:
  Phase 9 is fully green and Phase 10 Round 1 is verified.

If this still shows only Anchor macro warnings:
  Apply or re-apply the warning cleanup patch.

If this fails:
  Fix the first compiler/test failure only.
```

Do not jump to live token behavior.

---

## 6. Pending verification from the last patch

The last patch suggested was warning cleanup:

```bash
perl -0pi -e 's/#!\[forbid\(unsafe_code\)\]/#![allow(unexpected_cfgs)]\n#![forbid(unsafe_code)]/' programs/rox-anchor/src/lib.rs

perl -0pi -e 's/#\[program\]\npub mod rox_anchor/#\[allow(deprecated)\]\n#[program]\npub mod rox_anchor/' programs/rox-anchor/src/lib.rs
```

Next session should confirm:

```text
programs/rox-anchor/src/lib.rs contains:
  #![allow(unexpected_cfgs)]
  #![forbid(unsafe_code)]

and:
  #[allow(deprecated)]
  #[program]
  pub mod rox_anchor
```

Then run:

```bash
cargo check -p rox-anchor
cargo test -p rox-anchor
cargo check --workspace
```

Expected result:

```text
No real compiler errors.
Ideally no Anchor macro warning noise.
13 rox-anchor tests still pass.
Workspace still green.
```

---

## 7. Current code details worth preserving

### Anchor account state

Current program state:

```rust
RoxAnchorConfig {
    authority: Pubkey,
    halted: bool,
    recovery_required: bool,
}

RoxAnchorOperation {
    authority: Pubkey,
    operation_id_hash: [u8; 32],
    mint: Pubkey,
    token_account: Pubkey,
    direction: u8,
    amount_atoms: u64,
    burn_evidence_hash: [u8; 32],
    state: u8,
    challenge_open: bool,
    recovery_required: bool,
}
```

### Anchor operation states

```rust
OperationStateCode::Observed
OperationStateCode::ChallengeOpen
OperationStateCode::ChallengeAccepted
OperationStateCode::ChallengeRejected
OperationStateCode::RecoveryRequired
OperationStateCode::RecoveryResolved
OperationStateCode::Finalized
```

### Anchor directions

```rust
AnchorTransferDirection::RocToRox
AnchorTransferDirection::RoxToRoc
```

### Current program events

```text
RoxAnchorInitialized
RoxAnchorBurnObserved
RoxAnchorChallengeOpened
RoxAnchorChallengeResolved
RoxAnchorHalted
RoxAnchorRecovered
RoxAnchorFinalized
```

`RoxAnchorBurnObserved` now includes direction, amount, and burn evidence hash in addition to authority, operation ID hash, mint, and token account.

### Current program errors

```text
ProgramHalted
ChallengeOpen
RecoveryRequired
InvalidStateTransition
InvalidBinding
AlreadyFinalized
OperationBindingMismatch
DirectionBindingMismatch
MintBindingMismatch
TokenAccountBindingMismatch
AmountBindingMismatch
BurnEvidenceBindingMismatch
AuthorityMismatch
```

---

## 8. What is next after warning cleanup

The next real implementation phase is:

```text
Phase 10 Round 2 — Split local ROC→ROX and ROX→ROC semantics more explicitly, still without live SPL token CPI.
```

The build plan says Phase 10 should implement actual mint/burn semantics after the state machine compiles, including ROC burn observation fields, ROX mint authority model, reverse ROX burn fields, ROC release/recovery model, mint binding, token account binding, replay protection, challenge window, halt/recovery controls, and events. 

Recommended Phase 10 Round 2 scope:

```text
1. Add explicit operation kind / settlement side helpers:
   - is_roc_to_rox()
   - is_rox_to_roc()
   - requires_internal_roc_burn_evidence()
   - requires_external_rox_burn_evidence()

2. Rename/generalize burn_evidence_hash if needed:
   - current field is `burn_evidence_hash`
   - could become `source_burn_evidence_hash`
   - avoid churn unless needed; tests are green now.

3. Add authority/mint model placeholders as state, not live CPI:
   - config.rox_mint
   - config.mint_authority
   - config.bump or pda bump later
   - authority checks for configure/finalize path

4. Add replay protection inside Anchor state model:
   - current replay protection is mostly off-chain proof / idempotency / local models.
   - on-chain operation account currently prevents duplicate account init only if account addressing/seeds are enforced later.
   - next step should add PDA seed strategy or operation-hash account binding comments/tests.
   - Do not use random account init as replay protection long-term.

5. Add tests:
   - RocToRox accepts ROC burn evidence shape.
   - RoxToRoc accepts ROX burn evidence shape.
   - wrong direction rejects expected path.
   - zero amount rejected.
   - wrong mint rejected.
   - wrong token account rejected.
   - finalized operation cannot be replayed.
```

Do not add live CPI yet unless explicitly requested. The next patch should stay compile-tested and local.

---

## 9. Suggested next patch shape

Recommended next patch after verifying green:

```text
Phase 10 Round 2A:
  - Add explicit binding helper methods in programs/rox-anchor/src/state.rs.
  - Add tests for direction-specific finalization posture.
  - Do not add SPL token dependencies yet.
```

Good helper names:

```rust
impl OperationBindingArgs {
    pub fn is_roc_to_rox(self) -> bool
    pub fn is_rox_to_roc(self) -> bool
}

impl RoxAnchorOperation {
    pub fn is_roc_to_rox(&self) -> bool
    pub fn is_rox_to_roc(&self) -> bool
    pub fn require_direction(&self, expected: AnchorTransferDirection) -> Result<()>
    pub fn require_roc_to_rox(&self) -> Result<()>
    pub fn require_rox_to_roc(&self) -> Result<()>
}
```

Suggested tests:

```rust
roc_to_rox_direction_helper_accepts_expected_direction
rox_to_roc_direction_helper_accepts_expected_direction
wrong_direction_is_rejected
direction_specific_helpers_reject_corrupt_direction
```

Why this is the right next step:

```text
It deepens Phase 10 mint/burn semantics without jumping into live SPL token CPI.
It keeps behavior small.
It adds real tests.
It avoids live submission/mutation.
It protects against direction confusion, which is one of the major bridge-risk classes.
```

---

## 10. Phase 10 later rounds

After Round 2A, likely sequence:

### Phase 10 Round 2B — Config mint authority model

Add to `RoxAnchorConfig`:

```rust
pub rox_mint: Pubkey,
pub mint_authority: Pubkey,
```

Potentially:

```rust
pub bump: u8
```

Add config initializer args:

```rust
InitializeConfigArgs {
    rox_mint: Pubkey,
    mint_authority: Pubkey,
}
```

Tests:

```text
initialize rejects default rox_mint
initialize rejects default mint_authority
config stores authority/mint authority
wrong authority cannot finalize/mint-side transition
```

### Phase 10 Round 2C — PDA/seed replay protection model

Add operation seed strategy:

```text
operation account should be derived from operation_id_hash
or operation_id_hash + direction + mint
```

Need care with Anchor macro constraints. Do not overcomplicate in one patch.

Tests:

```text
operation binding is stable
operation hash cannot be swapped after initialize
finalized operation cannot reopen/recover
duplicate operation should fail at account/PDA level when local validator tests exist
```

### Phase 10 Round 2D — Token CPI preparation only

Add dependencies only when ready:

```toml
anchor-spl = compatible version
```

Then add account contexts for mint/token accounts, but first compile only.

Do not do live production minting.

### Phase 10 Round 3 — Local validator tests if Anchor CLI/tooling ready

Run:

```bash
anchor build
anchor test
```

Only after plain cargo checks are clean.

---

## 11. Integration phase notes

Phase 11 should come after Phase 10 local state-machine logic is stable.

Build plan Phase 11 calls for:

```text
local_nonvalue_roc_to_rox.rs
local_nonvalue_rox_to_roc.rs
coordinator_relayer_boundary.rs
crablink_status_display.rs
```

Purpose:

```text
Wire proof engine to coordinator.
Wire coordinator output to relayer dry-run.
Wire RPC proof evidence into proof review.
Test ROC-to-ROX local path.
Test ROX-to-ROC local path.
Test status display labels.
```

Important implementation caveat:

```text
Root tests/ are not currently active as normal workspace integration tests unless there is a root package or test harness. Activate them intentionally.
```

Possible approach:

```text
Create a crate:
  crates/rox-anchor-integration-tests
or move tests into:
  crates/rox-anchor-proof/tests/
  crates/rox-anchor-coordinator/tests/
  crates/rox-anchor-relayer/tests/
```

The cleaner short-term route is probably crate-level integration tests, not a new docs/test harness crate, unless needed.

---

## 12. Chaos phase notes

Phase 12 should test adversarial/failure cases after integration is stable.

Build plan Phase 12 calls for:

```text
challenge_griefing
coordinator_stale_evidence
halt_resume
relayer_retry_storm
rpc_equivocation
```

Expected behavior:

```text
challenge griefing remains bounded
stale coordinator evidence is rejected
halt/resume stays deterministic
relayer retry storms are bounded
RPC equivocation is detected
```

These should remain local tests. No live RPC.

---

## 13. Safety/authority rules to keep repeating internally

ROX Anchor must not bypass RustyOnions internal ROC authority.

Internal ROC authority remains:

```text
svc-wallet = mutation front-door
ron-ledger = durable receipt/balance truth
ron-accounting = snapshots/reports only
svc-rewarder = capped planning only
ron-policy = declarative gates only
CrabLink = display/user intent only
```

Future ROX → ROC must eventually route any internal ROC issue through:

```text
svc-wallet → ron-ledger
```

Not through:

```text
Anchor program directly
relayer directly
coordinator directly
RPC proof directly
CrabLink directly
gateway/omnigate directly
accounting/rewarder/policy directly
```

Future ROC → ROX must eventually require:

```text
explicit user intent
internal ROC burn proof
deterministic proof package
RPC/quorum evidence where applicable
challenge window
halt/recovery checks
Anchor state eligibility
only then mint-side behavior
```

But current work remains local compile/test only.

---

## 14. Current “do not do yet” list

Do not add unless explicitly requested:

```text
live wallet calls
live ledger mutation from this repo
live RPC submission
production deployment
production minting
production burning
bridge settlement
staking behavior
liquidity behavior
exchange-facing behavior
fake finality
fake success output
client-side authority
```

Allowed now:

```text
compile-tested Rust
local deterministic proof validation
local CLI inspection
local RPC proof models
local coordinator model
local relayer dry-run model
compile-tested Anchor code
Anchor state rules
local validator tests when tooling is ready
integration and chaos tests
```

---

## 15. Known risk areas to keep in mind

Major risks already covered or partially covered:

```text
replay
domain mismatch
direction mismatch
cluster mismatch
program ID mismatch
mint mismatch
token account mismatch
operation ID mismatch
idempotency key mismatch
nonce mismatch
missing evidence
quorum dispute
RPC stale evidence
RPC disputed signatures
challenge-open blocking
halt blocking
recovery-required blocking
relayer retry storms bounded locally
duplicate relayer idempotency locally rejected
```

Major risks still needing deeper work:

```text
on-chain replay/account seed enforcement
PDA derivation and bump strategy
Anchor account constraints
SPL token mint authority model
token account owner/mint validation
real local validator tests
cross-crate integration tests
chaos tests
upgrade authority policy
key rotation / authority recovery
stuck challenge recovery
CrabLink stale/offline status language
```

---

## 16. Commands to run at next session start

Run this first:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo check -p rox-anchor
cargo test -p rox-anchor
cargo check --workspace
```

Then, if clean, run the focused non-Anchor services once:

```bash
cargo test -p rox-anchor-core
cargo test -p rox-anchor-proof
cargo test -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-relayer
```

Then smoke the binaries:

```bash
cargo run -p rox-anchor-cli -- check
cargo run -p rox-anchor-rpc-proof
cargo run -p rox-anchor-coordinator
cargo run -p rox-anchor-relayer
```

Only after those are green, continue Phase 10 Round 2.

---

## 17. Good next-session prompt

Use this in the next session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
ROX Anchor is active implementation work. Phases 0–9 are effectively green. Phase 10 Round 1 local mint/burn binding semantics have started: Anchor OperationBindingArgs now includes direction, mint, token_account, amount_atoms, and burn_evidence_hash; state tests reached 13 passing; workspace check was green with only Anchor macro warnings. A warning-cleanup patch was proposed at the end, so first verify whether it landed.

Please review the attached CODEBUNDLE.md and BUILD_PLAN.md. Start by running/inspecting:
cargo fmt -p rox-anchor
cargo check -p rox-anchor
cargo test -p rox-anchor
cargo check --workspace

Then fix the first failure if any. If clean, continue Phase 10 Round 2 with small compile-tested Anchor state helpers/tests for direction-specific ROC→ROX and ROX→ROC semantics. Do not add live RPC, wallet calls, production deployment, production mint/burn, settlement, staking, liquidity, exchange behavior, or fake finality.
```

---

## 18. Summary

Current state:

```text
Phase 0 workspace compile spine: green
Phase 1 core shared types: green
Phase 2 proof engine: green
Phase 3 focused crate-local tests: green enough/current
Phase 4 CLI check: green
Phase 5 RPC proof local model: green
Phase 6 coordinator local model: green
Phase 7 relayer dry-run model: green
Phase 8 Anchor compile foundation: green
Phase 9 Anchor state rules: green
Phase 10 mint/burn logic: started, local binding semantics only
```

Next action:

```text
Verify warning cleanup.
Re-run focused Anchor checks/tests.
Then continue Phase 10 Round 2:
  direction-specific ROC→ROX / ROX→ROC helpers and tests,
  no SPL CPI yet,
  no live behavior.
```

The build remains on the right track: real Rust code, focused tests, compile checks, and no fake runtime authority.


### END NOTE - JULY 3 2026 - 15:00 CST