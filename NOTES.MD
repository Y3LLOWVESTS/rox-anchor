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


### BEGIN NOTE - JULY 3 2026 - 17:25 CST

Below is a paste-ready carry-over note pack for the next ROX Anchor session. I based this on the newest `BUILD_PLAN.md`, the newest `CODEBUNDLE.md`, the latest green terminal output, and the current dirty diff state. The build plan confirms this is implementation-first code/test/check work, with Phases 0–13 ordered from workspace recovery through final local green run.  The newest codebundle shows the active repo has 94 files, Git commit `0427e11 (dirty)`, all six Rust crates, the Anchor program, scripts, and future unit/integration/chaos test surfaces.  The latest terminal output confirms `cargo test -p rox-anchor` is **30/30**, `cargo check -p rox-anchor` is green, `cargo check --workspace` is green, `anchor build` is green, and `anchor test` runs the full workspace suite green. 

# ROX Anchor — Comprehensive Carry-Over Notes for Next Session

## 0. Session opener

We are continuing active implementation work on **RustyOnions / CrabLink / ROX Anchor**.

Current project identity:

```text
RustyOnions = backend/substrate/value-plane.
CrabLink = Tauri-first product/client for crab:// links, b3 assets, RON Passports, internal ROC paid access, creator pages, receipts, and future UX surfaces.
ROX Anchor = Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.
```

Current posture:

```text
Implementation-first.
QuickChain-style build rhythm.
Real Rust code + focused tests + compile checks + fixes before expansion.
No docs-only gate work.
No scaffold theater.
No live wallet calls.
No live RPC submission.
No production deployment.
No production minting/burning.
No bridge settlement.
No staking/liquidity/exchange-facing behavior.
No fake finality or fake success output.
```

The current completion estimate is approximately:

```text
ROX Anchor local implementation completion: ~76%

Compile/test foundation: ~95%
Core/proof/CLI/local models: ~90–95%
Anchor state rules: ~85–90%
Phase 10 mint/burn state machine: ~50–60%
Integration + chaos + final hardening: ~20–30%
Production/live readiness: intentionally 0% / not authorized
```

The project is now well past the initial compile-foundation stage. Phases 0–9 are effectively green. Phase 10 is active and meaningfully advanced with local mint/burn binding semantics, mint authority config, PDA replay-binding, settlement-action classification, authority-gated finalize, and expanded event observability.

---

## 1. Current source/reference attachments for next session

Use these as the active references:

```text
BUILD_PLAN.md
CODEBUNDLE.md
latest terminal output
```

The newest `BUILD_PLAN.md` is the phase authority. It confirms the implementation rules:

```text
Write Rust code.
Run focused compile/tests.
Fix failures before moving forward.
Do not add broad planning docs or decision gates.
Keep comments useful: what code does, validates, rejects, and returns.
Do not add live deployment, live wallet calls, live RPC submission, or real value movement until local program/tests are compile-green.
```

The newest `CODEBUNDLE.md` is the source snapshot for review/sharing. It says the source of truth remains the repo and that the bundle excludes secrets, ignored local metadata, build outputs, dependency folders, local Solana keys, and generated codebundles. It records:

```text
Root: /Users/mymac/Desktop/rox-anchor
Git: 0427e11 (dirty)
Files: 94
```

The repo file tree includes:

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

scripts/
  make_codebundle.sh
```

---

## 2. Current local green status

Latest verified terminal state:

```text
cargo test -p rox-anchor
  30 passed
  0 failed

cargo check -p rox-anchor
  green

cargo check --workspace
  green

anchor build
  green

anchor test
  green
  runs cargo test --workspace through Anchor.toml
```

Full workspace test suite currently includes:

```text
rox-anchor
  30 tests passing

rox-anchor-cli
  4 unit tests passing
  3 check_command integration tests passing

rox-anchor-core
  5 tests passing

rox-anchor-proof
  5 tests passing

rox-anchor-rpc-proof
  7 tests passing

rox-anchor-coordinator
  6 tests passing

rox-anchor-relayer
  6 tests passing

All doc-tests
  0 tests, all green
```

The current dirty files are:

```text
M programs/rox-anchor/src/events.rs
M programs/rox-anchor/src/instructions/finalize.rs
M programs/rox-anchor/src/state.rs
M scripts/make_codebundle.sh
```

Do not assume these have been committed. User explicitly does not want GitHub pushes for a while unless specifically requested.

---

## 3. Rust line count as of current repo

The user ran `cloc` locally.

Total repo Rust:

```text
65 Rust files
4,835 Rust code lines
828 blank lines
414 comment lines
```

Cleaner active `crates/ + programs/` count:

```text
59 Rust files
3,846 Rust code lines
681 blank lines
357 comment lines
```

Largest Rust source file:

```text
programs/rox-anchor/src/state.rs
970 code lines
```

This is expected because the Anchor state machine is carrying most of the Phase 8–10 behavior.

Interpretation:

```text
ROX Anchor is still lean.
The current ~4.8k Rust LOC already covers core types, proof review, CLI, RPC proof model, coordinator, relayer dry-run, Anchor state rules, PDA binding, finalize authority gate, event semantics, and full workspace/Anchor tests.
```

---

## 4. Development workflow and patch style to preserve

Use this flow every session:

```text
1. Run/inspect focused compile or test output.
2. Fix the first compiler/test failure.
3. Add the smallest useful behavior.
4. Add focused tests beside it.
5. Run cargo fmt.
6. Run focused cargo test/check.
7. Run cargo check --workspace when the focused surface is green.
8. Run anchor build / anchor test when Anchor program changes.
```

User patch preferences:

```text
Use paste-ready terminal patches.
Wrap patches in ~~~~bash fences.
Prefer complete file writes or clear scripted edits.
No downloads unless explicitly requested.
No git add/commit/push unless explicitly requested.
After each patch, include exact focused commands to run next.
Avoid Python helpers when simple Bash/Node is enough, but prior patches used Node/Python successfully when needed.
Use GIT_PAGER=cat and PAGER=cat for long patch runs to avoid terminal pager lockups.
```

Useful terminal environment setup:

```bash
export GIT_PAGER=cat
export PAGER=cat
```

Terminal note:

```text
The user reported needing to hold Enter to make long terminal output continue. This is likely because git diff or another pager enters an interactive pager. Use GIT_PAGER=cat and PAGER=cat in future scripts.
```

---

## 5. Build plan phase status

## Phase 0 — Workspace Recovery and Compile Spine

Status:

```text
COMPLETE / GREEN
```

What was accomplished:

```text
- Root Cargo workspace is active.
- Workspace includes all expected crates and the Anchor program:
  - crates/rox-anchor-core
  - crates/rox-anchor-proof
  - crates/rox-anchor-cli
  - crates/rox-anchor-rpc-proof
  - crates/rox-anchor-coordinator
  - crates/rox-anchor-relayer
  - programs/rox-anchor
- Cargo.lock exists and includes Anchor/Solana dependency stack.
- cargo check --workspace is green.
```

Root `Cargo.toml` currently uses:

```toml
[workspace]
resolver = "2"
members = [
  "crates/rox-anchor-core",
  "crates/rox-anchor-proof",
  "crates/rox-anchor-cli",
  "crates/rox-anchor-coordinator",
  "crates/rox-anchor-relayer",
  "crates/rox-anchor-rpc-proof",
  "programs/rox-anchor",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
publish = false
license = "MIT OR Apache-2.0"

[profile.release]
overflow-checks = true
```

The `overflow-checks = true` release profile setting was important for Anchor build safety.

---

## Phase 1 — `rox-anchor-core`

Status:

```text
COMPLETE / GREEN
```

Crate:

```text
crates/rox-anchor-core/
```

Files:

```text
src/errors.rs
src/ids.rs
src/labels.rs
src/lib.rs
src/state.rs
src/types.rs
```

Responsibilities now owned by `rox-anchor-core`:

```text
shared IDs
domains
operation IDs
idempotency keys
nonces
clusters
program IDs
mints
token accounts
directions
lifecycle states
challenge posture
halt posture
recovery posture
safe labels
shared reusable errors
```

Important invariant:

```text
Other crates must not invent competing state machines, labels, direction enums, or acceptance rules.
```

Current tests prove:

```text
identifier validation
rejection of empty/whitespace/control-byte identifiers
expected anchor binding shapes accepted
lifecycle states classify blockers
challenge/halt/recovery posture helpers block unsafe acceptance
safe labels are deterministic
```

Known green count:

```text
cargo test -p rox-anchor-core
5 passed
```

---

## Phase 2 — `rox-anchor-proof`

Status:

```text
COMPLETE / GREEN
```

Crate:

```text
crates/rox-anchor-proof/
```

Files:

```text
src/package.rs
src/validate.rs
src/replay.rs
src/quorum.rs
src/challenge.rs
src/recovery.rs
src/fixtures.rs
src/lib.rs
```

Current proof engine behavior:

```text
valid package accepted
missing/disputed evidence blocked
source domain mismatch rejected
target domain mismatch rejected
direction mismatch rejected
cluster mismatch rejected
program ID mismatch rejected
mint mismatch rejected
token account mismatch rejected
operation ID mismatch rejected
idempotency key mismatch rejected
nonce mismatch rejected
replayed operation ID rejected
replayed idempotency key rejected
replayed nonce rejected
challenge open blocks acceptance
challenge accepted blocks acceptance
halt posture blocks acceptance
recovery posture blocks acceptance
deterministic findings/decision order
```

Important terms:

```text
ReviewDecision::Accepted
ReviewDecision::Blocked
ReviewDecision::Rejected
```

Important finding categories include:

```text
PackageAccepted
ReplayOperationId
ReplayIdempotencyKey
ReplayNonce
EvidenceMissing
QuorumDisputed
ChallengeOpen
Halted
RecoveryRequired
Domain/mint/token/account/program binding mismatches
```

Known green count:

```text
cargo test -p rox-anchor-proof
5 passed
```

---

## Phase 3 — Focused Unit Test Lift

Status:

```text
COMPLETE ENOUGH / ACTIVE THROUGH CRATE-LOCAL TESTS
```

What exists:

```text
Core tests in rox-anchor-core.
Proof tests in rox-anchor-proof.
CLI tests in rox-anchor-cli.
RPC proof tests in rox-anchor-rpc-proof.
Coordinator tests in rox-anchor-coordinator.
Relayer tests in rox-anchor-relayer.
Anchor program tests in programs/rox-anchor/src/state.rs.
```

Important caveat:

```text
Root-level tests/ exist under tests/unit, tests/integration, and tests/chaos, but the root workspace is currently virtual.
Do not assume root tests are active as ordinary Cargo integration tests until a proper harness is added.
```

Future Phase 11/12 should intentionally activate root tests by either:

```text
- creating a real test harness crate, or
- moving integration/chaos tests into crate-local tests folders, or
- adding a dedicated workspace package for integration tests.
```

---

## Phase 4 — `rox-anchor-cli`

Status:

```text
COMPLETE / GREEN
```

Crate:

```text
crates/rox-anchor-cli/
```

Files:

```text
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

```bash
cargo run -p rox-anchor-cli -- check
```

The CLI prints a real proof review from `rox-anchor-proof`, not a fake success string.

Supported fixture modes include:

```text
valid
mismatch
replay
missing-evidence
disputed
challenge
halt
recovery
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

Known tests:

```text
unknown_command_is_error
check_challenge_fixture_reports_blocked_status
default_command_runs_check_against_valid_fixture
check_replay_fixture_uses_real_replay_rejection

check_default_fixture_prints_valid_review
check_halt_fixture_prints_halt_blocked_review
check_mismatch_fixture_prints_rejection_findings
```

Known green count:

```text
rox-anchor-cli unit tests: 4 passed
rox-anchor-cli check_command tests: 3 passed
```

Future CLI improvement:

```text
Add JSON proof package input support later, after local integration/chaos surfaces are stable.
```

Do not add live RPC, wallet calls, live submission, or fake finality to CLI.

---

## Phase 5 — `rox-anchor-rpc-proof`

Status:

```text
COMPLETE / GREEN
```

Crate:

```text
crates/rox-anchor-rpc-proof/
```

Files:

```text
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
insufficient matching sources classified
missing evidence classification
conflicting/disputed signatures classified
stale observation rejection
binding mismatch rejection
redaction of signatures/report shape
readiness review
mapping RPC evidence into proof EvidenceBundle
```

Tests currently cover:

```text
readiness rejects zero thresholds
redaction shortens signatures without hiding shape
binding mismatch is rejected
stale observation is rejected
too few matching sources is missing evidence
conflicting signatures are disputed evidence
matching RPC observations agree and map to proof evidence
```

Known green count:

```text
cargo test -p rox-anchor-rpc-proof
7 passed
```

Important invariant:

```text
RPC proof remains local evidence modeling only.
It does not make live network calls.
```

---

## Phase 6 — `rox-anchor-coordinator`

Status:

```text
COMPLETE / GREEN
```

Crate:

```text
crates/rox-anchor-coordinator/
```

Files:

```text
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
CoordinatorQueue rejects duplicate operation IDs.
CoordinatorQueue enforces queue capacity.
CoordinatorDecision runs RPC proof review.
CoordinatorDecision maps RPC evidence into rox-anchor-proof EvidenceBundle.
CoordinatorReadiness rejects unusable queue/RPC configs.
Redacted coordinator reports preserve decision shape without exposing raw internals.
```

Tests currently cover:

```text
readiness requires queue capacity and RPC config
queue capacity is enforced
stale RPC evidence prevents acceptance
redacted report keeps decision shape
valid observations hand off to proof and accept
duplicate operation is rejected by queue
```

Known green count:

```text
cargo test -p rox-anchor-coordinator
6 passed
```

Important invariant:

```text
Coordinator is orchestration only.
It must not invent separate proof acceptance rules.
It must keep calling rox-anchor-proof.
```

---

## Phase 7 — `rox-anchor-relayer`

Status:

```text
COMPLETE / GREEN
```

Crate:

```text
crates/rox-anchor-relayer/
```

Files:

```text
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
Blocked/rejected proof reviews do not submit and produce local receipts with 0 attempts.
Duplicate idempotency keys produce DuplicateRequest.
Receipt capacity is enforced.
Receipts are local dry-run receipts and have live_submission=false.
Readiness rejects zero retry/receipt limits.
Redacted reports keep status, operation, target, proof decision, attempts, and live_submission posture.
```

Tests currently cover:

```text
relayer readiness rejects zero limits
retry policy is bounded
blocked proof review does not submit
accepted proof review produces dry-run receipt
redacted receipt report keeps local shape
idempotency replay is rejected
```

Known green count:

```text
cargo test -p rox-anchor-relayer
6 passed
```

Important invariant:

```text
Relayer remains dry-run only.
No live RPC submission exists.
No transaction submission exists.
No settlement exists.
```

---

## Phase 8 — Anchor Program Compile Foundation

Status:

```text
COMPLETE / GREEN
```

Program:

```text
programs/rox-anchor/
```

Files:

```text
Cargo.toml
src/lib.rs
src/errors.rs
src/events.rs
src/state.rs
src/instructions/mod.rs
src/instructions/initialize.rs
src/instructions/observe_burn.rs
src/instructions/open_challenge.rs
src/instructions/resolve_challenge.rs
src/instructions/halt.rs
src/instructions/recover.rs
src/instructions/finalize.rs
```

Anchor setup:

```text
anchor-lang = 0.31.1
program crate compiles
Anchor.toml points localnet program to:
  U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR
provider wallet:
  .solana/rox-anchor-localnet-payer.json
Anchor test script:
  cargo test --workspace
```

Important local-only key handling:

```text
.solana/rox-anchor-localnet-payer.json is local-only and ignored.
Do not share or commit local payer keypairs.
The current codebundle generator is hardened to avoid including local Solana keypair files or 64-byte keypair-shaped JSON.
```

Anchor CLI status:

```text
anchor build: green
anchor test: green
```

The local payer was rotated after an older CODEBUNDLE accidentally included key material. Git protections worked, but the codebundle generator had to be hardened.

---

## Phase 9 — Anchor Program State Rules

Status:

```text
COMPLETE / GREEN
```

Core Anchor account/state surfaces:

```text
RoxAnchorConfig
RoxAnchorOperation
InitializeConfigArgs
OperationBindingArgs
OperationStateCode
AnchorTransferDirection
AnchorSettlementAction
```

Current config behavior includes:

```text
authority binding
rox_mint binding
mint_authority binding
mint_authority_bump storage
halt flag
recovery_required flag
authority checks
mint checks
mint authority checks
observation-open checks
halt/recover methods
```

Current operation behavior includes:

```text
operation_id_hash binding
operation_bump storage
mint binding
token_account binding
direction storage
amount_atoms storage
burn_evidence_hash storage
state storage
challenge_open flag
recovery_required flag
PDA seed prefix
PDA derivation helper
PDA binding check
direction helpers
settlement action helper
state transition helpers
finalize eligibility checks
```

Current instruction/state transitions:

```text
initialize
observe_burn
open_challenge
resolve_challenge
halt
recover
finalize
```

State blockers enforced:

```text
halted config blocks observation/finalization
config recovery_required blocks observation/finalization
challenge_open blocks finalization
accepted challenge blocks finalization
operation recovery_required blocks finalization
invalid/corrupt direction blocks finalization
zero amount blocks finalization
zero burn evidence hash blocks finalization
wrong authority cannot halt/recover/finalize
wrong mint blocks finalization
wrong mint authority blocks configuration behavior
wrong token account blocks binding
finalized operation cannot be reopened
finalized operation cannot be challenged again
finalized operation cannot be recovered again
```

Known Anchor tests include:

```text
accepted_challenge_blocks_finalization
binding_mismatch_is_rejected
challenge_resolution_controls_finalization
config_initialize_records_mint_authority_binding
config_initialize_rejects_empty_mint_bindings
config_mint_authority_helper_rejects_wrong_authority
config_rox_mint_helper_rejects_wrong_mint
config_rox_mint_mismatch_blocks_finalize
corrupt_mint_burn_binding_blocks_finalize
direction_code_round_trips
direction_specific_helpers_reject_corrupt_direction
finalize_authority_gate_rejects_wrong_signer_before_transition
finalized_operations_cannot_be_reopened
halt_and_recovery_are_explicit_blockers
initialize_records_local_mint_burn_binding
initialize_rejects_empty_binding_parts
initialize_with_bump_records_operation_pda_bump
invalid_config_binding_blocks_finalize
operation_can_finalize_only_when_clear
operation_pda_derivation_changes_with_config_or_operation_hash
operation_pda_derivation_is_stable_and_binding_checked
operation_pda_derivation_rejects_wrong_account_or_bump
operation_seed_prefix_is_stable
operation_state_code_round_trips
roc_to_rox_direction_helper_accepts_expected_direction
rox_to_roc_direction_helper_accepts_expected_direction
settlement_action_maps_direction_to_local_finalize_intent
wrong_authority_cannot_halt_or_recover_config
wrong_direction_is_rejected_by_specific_helpers
test_id
```

Known green count:

```text
cargo test -p rox-anchor
30 passed
```

---

## Phase 10 — ROX / ROC Mint-Burn Program Logic

Status:

```text
ACTIVE / PARTIALLY COMPLETE / LOCAL-ONLY
```

Phase 10 purpose from the build plan:

```text
Implement actual mint/burn semantics once the state machine compiles.
```

Build plan Phase 10 tasks:

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

What has already landed in Phase 10:

```text
OperationBindingArgs includes:
  operation_id_hash: [u8; 32]
  direction: AnchorTransferDirection
  mint: Pubkey
  token_account: Pubkey
  amount_atoms: u64
  burn_evidence_hash: [u8; 32]

AnchorTransferDirection:
  RocToRox
  RoxToRoc

RoxAnchorOperation stores:
  operation_id_hash
  operation_bump
  mint
  token_account
  direction
  amount_atoms
  burn_evidence_hash
  state
  challenge_open
  recovery_required

RoxAnchorConfig stores:
  authority
  rox_mint
  mint_authority
  mint_authority_bump
  halted
  recovery_required
```

Binding/validation now rejects:

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
config rox_mint mismatch
config mint_authority mismatch
wrong authority
wrong PDA account/bump
```

Direction helpers now exist:

```text
direction_code()
is_roc_to_rox()
is_rox_to_roc()
require_direction(expected)
require_roc_to_rox()
require_rox_to_roc()
requires_internal_roc_burn_evidence()
requires_external_rox_burn_evidence()
```

Settlement action semantics now exist:

```text
AnchorSettlementAction::MintRoxForRocBurn
AnchorSettlementAction::ReleaseRocForRoxBurn
```

Mapping:

```text
RocToRox -> MintRoxForRocBurn -> u8 value 1
RoxToRoc -> ReleaseRocForRoxBurn -> u8 value 2
```

PDA/replay-binding work now exists:

```text
RoxAnchorOperation::SEED_PREFIX = b"rox-anchor-operation"
RoxAnchorOperation::derive_address(program_id, config_key, operation_id_hash)
RoxAnchorOperation::require_derived_address(program_id, config_key, operation_account)
operation account contexts use seeds:
  RoxAnchorOperation::SEED_PREFIX
  config.key().as_ref()
  operation.operation_id_hash.as_ref()
operation stores operation_bump
transition contexts use bump = operation.operation_bump
```

Finalize has been hardened:

```text
Finalize now requires configured authority signer.
Finalize calls config.require_authority(ctx.accounts.authority.key()).
Finalize computes settlement_action from operation direction.
Finalize transitions operation state through operation.finalize(config).
Finalize emits detailed RoxAnchorFinalized event.
```

`RoxAnchorFinalized` now emits:

```text
authority
operation_id_hash
settlement_action
direction
mint
token_account
amount_atoms
burn_evidence_hash
```

Important limitation:

```text
This is still local Anchor state-machine semantics only.
There is no SPL token CPI.
There is no live mint.
There is no live burn.
There is no production settlement.
There is no live RPC submission.
There is no live wallet call.
There is no live ledger mutation from this repo.
```

Phase 10 completion estimate:

```text
Phase 10 is roughly 50–60% complete.
The local state/binding/finalize/event backbone is strong.
Remaining Phase 10 work is token account/CPI preparation and local-validator proof.
```

---

## 6. Current dirty working tree details

Dirty files:

```text
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/state.rs
scripts/make_codebundle.sh
```

### `events.rs` dirty changes

`RoxAnchorFinalized` was expanded from only:

```rust
pub operation_id_hash: [u8; 32],
```

to:

```rust
pub authority: Pubkey,
pub operation_id_hash: [u8; 32],
pub settlement_action: u8,
pub direction: u8,
pub mint: Pubkey,
pub token_account: Pubkey,
pub amount_atoms: u64,
pub burn_evidence_hash: [u8; 32],
```

Purpose:

```text
Make finalization observable with the signer and full local settlement binding.
```

### `finalize.rs` dirty changes

Finalize context now includes:

```rust
pub authority: Signer<'info>,
```

Handler now requires the configured authority:

```rust
config.require_authority(ctx.accounts.authority.key())?;
```

Handler now computes settlement action before state transition:

```rust
let settlement_action = operation.settlement_action()?.as_u8();
```

Handler emits:

```rust
RoxAnchorFinalized {
    authority: ctx.accounts.authority.key(),
    operation_id_hash: operation.operation_id_hash,
    settlement_action,
    direction: operation.direction,
    mint: operation.mint,
    token_account: operation.token_account,
    amount_atoms: operation.amount_atoms,
    burn_evidence_hash: operation.burn_evidence_hash,
}
```

Purpose:

```text
Finalize is no longer permissionless.
Finalize event now carries terminal transition evidence needed for later local-validator/integration tests.
```

### `state.rs` dirty changes

Added:

```rust
AnchorSettlementAction
```

with:

```rust
MintRoxForRocBurn -> 1
ReleaseRocForRoxBurn -> 2
```

Added operation helper:

```rust
settlement_action(&self) -> Result<AnchorSettlementAction>
```

Added/updated tests:

```text
settlement_action_maps_direction_to_local_finalize_intent
finalize_authority_gate_rejects_wrong_signer_before_transition
corrupt direction rejects settlement_action
```

Purpose:

```text
Make ROC→ROX and ROX→ROC finalization intent explicit and deterministic before adding any SPL token CPI.
```

### `scripts/make_codebundle.sh` dirty changes

The codebundle generator was hardened after an earlier generated CODEBUNDLE accidentally included local `.solana` keypair contents.

Current behavior:

```text
uses git ls-files -co --exclude-standard when inside Git repo
respects gitignore
skips secret/key-shaped paths
skips local Solana keypair naming patterns
detects Solana 64-byte keypair JSON arrays
skips target/build/dependency/generated outputs
allows harmless .gitignore rule `.solana/`
allows harmless Anchor.toml provider wallet path string
rejects actual .solana file sections/TOC entries
```

Verified:

```text
CODEBUNDLE generation passes secret failsafe.
Local payer remains ignored/untracked.
No Solana 64-byte keypair-shaped JSON is included.
Anchor.toml may still show wallet path:
  wallet = ".solana/rox-anchor-localnet-payer.json"
```

Important:

```text
Do not weaken this generator.
Do not share any local keypair file.
Do not commit `.solana/rox-anchor-localnet-payer.json`.
```

---

## 7. Anchor.toml and local program details

Current Anchor config:

```toml
[features]
seeds = false
skip-lint = false

[programs.localnet]
rox_anchor = "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR"

[provider]
cluster = "Localnet"
wallet = ".solana/rox-anchor-localnet-payer.json"

[scripts]
test = "cargo test --workspace"
```

Program ID:

```text
U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR
```

Provider wallet path:

```text
.solana/rox-anchor-localnet-payer.json
```

Important:

```text
The wallet path string in Anchor.toml is harmless.
The actual keypair file is sensitive and must remain ignored/untracked.
```

---

## 8. Current build/tool dependency posture

Current Anchor/Solana dependency stack:

```text
anchor-lang = 0.31.1
solana-program = 2.3.0 via anchor-lang
```

Important dependency pins/compatibility decisions already solved:

```text
proc-macro-crate pinned/downgraded to compatible 3.3.0 path
toml_datetime compatible 0.6.11
toml_edit compatible 0.22.27
blake3 1.5.5
indexmap 2.7.1
hashbrown 0.15.5
zeroize 1.8.1
unicode-segmentation 1.12.0
```

Anchor program features include:

```toml
[features]
default = []
no-entrypoint = []
no-idl = []
no-log-ix-name = []
cpi = ["no-entrypoint"]
anchor-program = []
idl-build = ["anchor-lang/idl-build"]
```

Do not churn dependency versions unless a compiler error requires it.

---

## 9. Security posture and non-authorization boundaries

Current allowed work:

```text
compile-tested Rust crates
shared core types
deterministic local proof validation
replay rejection
binding mismatch rejection
challenge/halt/recovery review
focused unit tests
local CLI inspection
local RPC evidence models
local coordinator model
local relayer dry-run model
compile-tested Anchor program code
Anchor state rules
local validator tests when ready
integration tests
chaos tests
```

Current forbidden work unless explicitly requested:

```text
live wallet calls
live ledger mutation outside intended Anchor/local test context
live RPC submission
production deployment
production minting
production burning
bridge settlement
staking
liquidity
exchange-facing behavior
fake finality
fake success output
client-side authority over state transitions
```

Important design boundary:

```text
ROX Anchor must not bypass RustyOnions internal ROC authority.
```

Internal ROC authority remains:

```text
svc-wallet = mutation front-door
ron-ledger = durable receipt/balance truth
ron-accounting = snapshots/reports only
svc-rewarder = capped planning only
ron-policy = declarative gates only
CrabLink = display/user intent only
```

Future ROX→ROC internal ROC release must eventually route through:

```text
svc-wallet -> ron-ledger
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

Future ROC→ROX must eventually require:

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

---

## 10. Known risks already covered or partially covered

Covered / partially covered locally:

```text
replay at proof layer
replay/idempotency at relayer dry-run layer
operation PDA replay-binding at Anchor state layer
domain mismatch rejection
direction mismatch rejection
cluster mismatch rejection
program ID mismatch rejection
mint mismatch rejection
mint authority mismatch rejection
token account mismatch rejection
operation ID mismatch rejection
idempotency key mismatch rejection
nonce mismatch rejection
missing evidence classification
quorum dispute classification
RPC stale evidence rejection
RPC conflicting signature dispute
challenge-open blocking
accepted challenge blocking
halt blocking
recovery-required blocking
relayer retry storm bounded locally
duplicate relayer idempotency rejected
wrong authority rejected for halt/recover/finalize
finalized operation cannot reopen
finalized operation cannot be challenged/recovered again
unsafe finalization blocked by state/config/binding mismatches
```

Remaining risk areas needing deeper work:

```text
SPL token CPI preparation
actual local mint account constraints
actual token account ownership/mint validation
mint authority PDA signer model in CPI context
separate finalize paths or account constraints for ROC→ROX vs ROX→ROC
on-chain duplicate operation behavior under real account init/local validator tests
local-validator instruction tests beyond cargo unit tests
cross-crate integration tests
chaos tests
upgrade authority policy
key rotation / authority recovery
stuck challenge recovery flows
CrabLink stale/offline status language
JSON input support for CLI/proof packages
real status display adapters later
```

---

## 11. Exact next-session first commands

Start the next session with:

```bash
export GIT_PAGER=cat
export PAGER=cat

cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

Expected result:

```text
cargo test -p rox-anchor: 30/30
cargo check -p rox-anchor: green
cargo check --workspace: green
anchor build: green
anchor test: green
```

Then optionally smoke the local non-Anchor surfaces:

```bash
cargo test -p rox-anchor-core
cargo test -p rox-anchor-proof
cargo test -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-relayer

cargo run -p rox-anchor-cli -- check
cargo run -p rox-anchor-rpc-proof
cargo run -p rox-anchor-coordinator
cargo run -p rox-anchor-relayer
```

Then continue Phase 10.

---

## 12. Recommended next implementation step

Recommended next step:

```text
Phase 10 Round 2E / Token CPI preparation boundary
```

But proceed carefully.

Two good options exist.

### Option A — Safer next patch: finalize-path classification / account-plan helpers only

Add more local helper semantics before introducing `anchor-spl`.

Possible helpers:

```rust
impl RoxAnchorOperation {
    pub fn requires_rox_mint_output(&self) -> Result<bool>
    pub fn requires_internal_roc_release(&self) -> Result<bool>
    pub fn require_finalizable_roc_to_rox(&self, config: &RoxAnchorConfig) -> Result<()>
    pub fn require_finalizable_rox_to_roc(&self, config: &RoxAnchorConfig) -> Result<()>
}
```

Tests:

```text
roc_to_rox_finalization_classifies_rox_mint_output
rox_to_roc_finalization_classifies_internal_roc_release
corrupt_direction_rejects_finalization_classification
halted_config_rejects_direction_specific_finalization
wrong_mint_rejects_roc_to_rox_finalize_path
```

Why Option A is safer:

```text
No dependency churn.
No SPL CPI yet.
No account constraint complexity yet.
Keeps Phase 10 moving with real tests.
```

### Option B — Token CPI preparation only, no live CPI

Add `anchor-spl = "0.31.1"` and compile-test account constraints only.

Potential account additions need care:

```text
ROX mint account constrained to config.rox_mint
destination token account constrained to mint
token_program account
maybe mint authority PDA account
```

Potential challenge:

```text
A single finalize instruction may not have the same account requirements for ROC→ROX and ROX→ROC.
RocToRox wants ROX mint destination account.
RoxToRoc probably wants proof of external ROX burn and later internal ROC release path, not ROX minting.
```

A safer design may split later:

```text
finalize_roc_to_rox
finalize_rox_to_roc
```

or keep one finalize for state transition and add later settlement-specific instructions.

Recommended short-term choice:

```text
Do Option A first.
Do not add anchor-spl until we decide whether finalize remains unified or splits into direction-specific finalization instructions.
```

---

## 13. What not to do next

Do not immediately add:

```text
live SPL mint CPI
live burn CPI
live token transfer
live RPC client
live wallet signing
mainnet/devnet deployment
production mint authority behavior
bridge settlement claims
CrabLink exchange/swap/cashout language
staking
liquidity
exchange support
```

Do not accidentally turn local dry-run receipts into real transaction receipts.

Do not treat event emission as settlement by itself.

Do not treat CLI output as finality.

Do not commit or push unless explicitly requested.

---

## 14. Potential next patch name

Use a clear patch label:

```text
ROX Anchor Phase 10 Round 2E — Direction-specific finalize readiness helpers
```

Possible patch goals:

```text
- Add local-only helper methods that classify finalization path.
- Keep settlement_action as the canonical event code.
- Ensure corrupt directions cannot classify.
- Ensure halt/recovery/challenge blockers still apply before either path is considered ready.
- Add focused tests.
- Run cargo fmt -p rox-anchor.
- Run cargo test -p rox-anchor.
- Run cargo check -p rox-anchor.
- Run cargo check --workspace.
- Run anchor build.
- Run anchor test.
```

Expected result after next patch:

```text
cargo test -p rox-anchor: likely 32–34 tests passing
workspace green
anchor build/test green
still no live value movement
```

---

## 15. Current session recap / what was accomplished since last carry-over notes

Major accomplishments since earlier Phase 10 notes:

```text
1. Anchor CLI/provider wallet problem solved.
2. Local payer key rotated after codebundle leakage.
3. .solana local payer is ignored/untracked.
4. Codebundle generator hardened:
   - respects gitignore
   - skips secret/key-shaped paths
   - skips local Solana keypairs
   - detects 64-byte Solana keypair JSON arrays
   - avoids false positives from harmless .gitignore and Anchor.toml path strings
5. anchor build green.
6. anchor test green.
7. cargo test --workspace green through Anchor.toml script.
8. Operation PDA seed/bump replay-binding landed.
9. PDA-constrained transition contexts landed.
10. Settlement action helper landed.
11. Finalized event expanded to include settlement semantics.
12. Finalize authority gate landed.
13. Finalized event now includes authority signer.
14. rox-anchor tests rose to 30 passing.
15. Full workspace/Anchor suite green.
```

This is now a strong local implementation base.

---

## 16. Current codebundle status

The newest codebundle was generated by:

```text
scripts/make_codebundle.sh
```

Generation timestamp:

```text
2026-07-03T22:15:51Z
```

It reports:

```text
Git: 0427e11 (dirty)
Files: 94
Skipped Files: None
```

Interpretation:

```text
“Skipped Files: None” means none of the tracked/unignored text files selected by the generator were skipped.
It does not mean ignored local files such as .solana keypairs were included.
The generator says it excludes secrets, ignored local metadata, build outputs, dependency folders, local Solana keys, and generated codebundles.
```

Important caveat:

```text
The CODEBUNDLE is a review artifact, not runtime authorization.
The local repo is the source of truth.
```

---

## 17. Suggested next-session prompt

Paste this into the next session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
ROX Anchor is active implementation work. Use QuickChain-style build rhythm: real Rust code, focused tests, compile checks, and fix first failure before expanding. Do not use Grok. Do not add docs-only gates or scaffold theater.

Current local state:
Phases 0–9 are effectively complete/green. Phase 10 is active. Latest verified run:
cargo test -p rox-anchor: 30/30
cargo check -p rox-anchor: green
cargo check --workspace: green
anchor build: green
anchor test: green

Current dirty files:
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/state.rs
scripts/make_codebundle.sh

Important current Phase 10 behavior:
- OperationBindingArgs includes operation_id_hash, direction, mint, token_account, amount_atoms, burn_evidence_hash.
- RoxAnchorConfig stores rox_mint, mint_authority, mint_authority_bump.
- RoxAnchorOperation stores operation_bump and has PDA seed/bump binding.
- Direction helpers and settlement_action exist.
- Finalize requires the configured authority signer.
- RoxAnchorFinalized emits authority, operation_id_hash, settlement_action, direction, mint, token_account, amount_atoms, and burn_evidence_hash.
- Codebundle generator is hardened against local Solana keypair leakage.

Please review the attached CODEBUNDLE.md and BUILD_PLAN.md. Start with:
export GIT_PAGER=cat
export PAGER=cat
cd /Users/mymac/Desktop/rox-anchor
cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test

If green, continue Phase 10 with a small local-only patch. Recommended next patch: direction-specific finalize readiness helpers/classification before adding anchor-spl or SPL token CPI. Do not add live RPC, wallet calls, production deployment, production mint/burn, bridge settlement, staking, liquidity, exchange behavior, or fake finality.
```

---

## 18. Short status summary for fast recall

```text
ROX Anchor completion: ~76%

Green:
- workspace compile
- core shared types
- deterministic proof review
- CLI check
- RPC proof local evidence
- coordinator local model
- relayer dry-run model
- Anchor compile foundation
- Anchor state rules
- Phase 10 local binding/finalize/event semantics so far
- anchor build
- anchor test

Current tests:
- rox-anchor: 30
- cli unit: 4
- cli integration: 3
- coordinator: 6
- core: 5
- proof: 5
- relayer: 6
- rpc-proof: 7
- doc-tests: 0 each / green

Current dirty files:
- events.rs
- finalize.rs
- state.rs
- scripts/make_codebundle.sh

Current next work:
- continue Phase 10
- prefer direction-specific finalize readiness helpers before SPL CPI
- then consider anchor-spl account-constraint preparation
- then local validator tests
- then Phase 11 integration tests
- then Phase 12 chaos tests
- then Phase 13 final local green run

Do not push commits unless explicitly requested.
```

---

## 19. Exact next command block

```bash
export GIT_PAGER=cat
export PAGER=cat

cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

If that is green, proceed to:

```text
Phase 10 Round 2E — Direction-specific finalize readiness helpers.
```

These notes are ready to paste into the next session.


### END NOTE - JULY 3 2026 - 17:25 CST


### BEGIN NOTE - JULY 3 2026 - 19:47 CST

Confirmed: latest ROX Anchor output is fully green at **54/54 `rox-anchor` tests**, with `cargo check -p rox-anchor`, `cargo check --workspace`, `anchor build`, and full `anchor test` workspace suite green. The generated codebundle was refreshed afterward with the secret failsafe scan and records the repo as `/Users/mymac/Desktop/rox-anchor`, Git `0427e11 (dirty)`, 94 files.  

Here are the comprehensive carry-over notes for the next session.

# ROX Anchor — Comprehensive Carry-Over Notes for Next Session

## 0. Session opener

We are continuing active implementation work on **RustyOnions / CrabLink / ROX Anchor**.

Project identity:

```text
RustyOnions = backend/substrate/value-plane.
CrabLink = Tauri-first product/client for crab:// links, b3 assets, RON Passports, internal ROC paid access, creator pages, receipts, and future UX surfaces.
ROX Anchor = Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.
```

Current posture:

```text
Implementation-first.
QuickChain-style code/test/check/fix loop.
Real Rust code + focused tests + compile checks + fixes before expansion.
No docs-only gate work.
No placeholder scaffold theater.
No live bridge runtime.
No production mint/burn.
No live wallet calls.
No live RPC submission.
No production deployment.
No staking/liquidity/exchange-facing behavior.
No fake finality or fake success output.
```

The project is now deep into **Phase 10 — ROX / ROC mint-burn program logic**, but still strictly local and compile-tested. The local Anchor state-machine backbone is strong. There is still no SPL CPI, no live token minting/burning, no production settlement, and no live RPC submission.

---

## 1. Current attached references

Use these as the active references for the next session:

```text
Pasted text.txt      latest terminal output
CODEBUNDLE.md        latest regenerated source snapshot
BUILD_PLAN.md        active phase/build authority
```

The latest codebundle was generated by:

```bash
bash scripts/make_codebundle.sh
```

The script completed the secret failsafe scan and wrote a safe `CODEBUNDLE.md`.

Codebundle metadata:

```text
Generated: 2026-07-04T00:30:20Z
Root: /Users/mymac/Desktop/rox-anchor
Git: 0427e11 (dirty)
Files: 94
Skipped files: None
```

The codebundle is a review artifact only. Source of truth remains the local repo.

The build plan remains the phase authority. It says this is an implementation build plan and the rules for every phase are:

```text
Write Rust code.
Run focused compile/tests.
Fix failures before moving forward.
Do not add broad planning docs or decision gates.
Keep comments about what code does, validates, rejects, and returns.
Do not add live deployment, live wallet calls, live RPC submission, or real value movement until local program/tests are compile-green.
```

---

## 2. Latest verified green status

Latest terminal output confirms:

```text
cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

All completed green.

Current `rox-anchor` test status:

```text
running 54 tests
54 passed
0 failed
0 ignored
```

Full `anchor test` workspace suite is also green:

```text
rox-anchor                 54 tests passing
rox-anchor-cli unit         4 tests passing
rox-anchor-cli integration  3 tests passing
rox-anchor-coordinator      6 tests passing
rox-anchor-core             5 tests passing
rox-anchor-proof            5 tests passing
rox-anchor-relayer          6 tests passing
rox-anchor-rpc-proof        7 tests passing
doc-tests                   0 tests each / green
```

The latest patch, **Phase 10 Round 2P**, landed successfully. It added the missing ROX→ROC direction-specific finalize coverage:

```text
finalize_for_direction_accepts_expected_rox_to_roc_path
```

That moved `cargo test -p rox-anchor` from 53 to 54 tests.

---

## 3. Current repo layout

The regenerated codebundle shows the active repo still has 94 files and this core layout:

```text
.
  .gitattributes
  .gitignore
  Anchor.toml
  BUILD_PLAN.md
  Cargo.lock
  Cargo.toml
  IDB.md
  LICENSE
  NOTES.MD
  NOTICE
  README.md
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

  scripts/
    make_codebundle.sh

  tests/
    chaos/
    integration/
    unit/
```

Workspace members in root `Cargo.toml`:

```text
crates/rox-anchor-core
crates/rox-anchor-proof
crates/rox-anchor-cli
crates/rox-anchor-coordinator
crates/rox-anchor-relayer
crates/rox-anchor-rpc-proof
programs/rox-anchor
```

Important caution:

```text
The root tests/ tree exists, but the workspace root is virtual.
Root-level tests/unit, tests/integration, and tests/chaos are future surfaces.
They are not normal active Cargo integration tests unless we intentionally add a harness package or move tests into crate-local tests.
Current active tests are crate-local.
```

---

## 4. Current phase status

## Phase 0 — Workspace recovery and compile spine

Status:

```text
COMPLETE / GREEN
```

Proof:

```text
cargo check --workspace is green.
Root workspace includes all six crates plus programs/rox-anchor.
Cargo.lock exists and includes Anchor/Solana dependency graph.
```

---

## Phase 1 — `rox-anchor-core`

Status:

```text
COMPLETE / GREEN
```

Current role:

```text
Shared source of truth for IDs, domains, directions, lifecycle states, challenge posture, halt posture, recovery posture, status labels, and reusable errors.
```

Current active files:

```text
crates/rox-anchor-core/src/errors.rs
crates/rox-anchor-core/src/ids.rs
crates/rox-anchor-core/src/labels.rs
crates/rox-anchor-core/src/lib.rs
crates/rox-anchor-core/src/state.rs
crates/rox-anchor-core/src/types.rs
```

Current tests:

```text
5 passing
```

Core invariant:

```text
Other crates must not invent duplicate state machines, duplicate labels, duplicate direction concepts, or duplicate acceptance rules.
```

---

## Phase 2 — `rox-anchor-proof`

Status:

```text
COMPLETE / GREEN
```

Current role:

```text
Deterministic local proof review engine.
```

Current active files:

```text
crates/rox-anchor-proof/src/challenge.rs
crates/rox-anchor-proof/src/fixtures.rs
crates/rox-anchor-proof/src/lib.rs
crates/rox-anchor-proof/src/package.rs
crates/rox-anchor-proof/src/quorum.rs
crates/rox-anchor-proof/src/recovery.rs
crates/rox-anchor-proof/src/replay.rs
crates/rox-anchor-proof/src/validate.rs
```

Current tests:

```text
5 passing
```

Current behavior:

```text
valid package acceptance
binding mismatch rejection
replay rejection
missing/disputed evidence blocking
challenge/halt/recovery blocking
deterministic finding order
```

Important invariant:

```text
Coordinator, RPC proof, CLI, and relayer must keep using rox-anchor-proof rather than inventing their own acceptance rules.
```

---

## Phase 3 — Unit test lift

Status:

```text
COMPLETE ENOUGH FOR CURRENT ACTIVE WORK
```

Current test model:

```text
core/proof/CLI/RPC/coordinator/relayer tests are crate-local.
Anchor program tests live inside programs/rox-anchor.
Root tests/ are future integration/chaos surfaces.
```

Do not assume the root tests are active until a real test harness is added.

---

## Phase 4 — `rox-anchor-cli`

Status:

```text
COMPLETE / GREEN
```

Current tests:

```text
4 unit tests passing
3 integration tests passing
```

Current behavior:

```bash
cargo run -p rox-anchor-cli -- check
```

The command uses real `rox-anchor-proof` behavior and does not print fake success.

Current fixtures:

```text
valid
mismatch
replay
missing-evidence
disputed
challenge
halt
recovery
```

Future CLI improvement:

```text
JSON proof package input support.
```

Do not add live RPC or wallet behavior to CLI.

---

## Phase 5 — `rox-anchor-rpc-proof`

Status:

```text
COMPLETE / GREEN
```

Current tests:

```text
7 passing
```

Current behavior:

```text
local RPC observation model
commitment classification
quorum agreement/disagreement
missing evidence
stale observation rejection
binding mismatch rejection
signature redaction
mapping accepted local RPC evidence into proof evidence
```

Still local only:

```text
No live RPC calls.
No network submission.
No external side effects.
```

---

## Phase 6 — `rox-anchor-coordinator`

Status:

```text
COMPLETE / GREEN
```

Current tests:

```text
6 passing
```

Current behavior:

```text
local coordinator config
observation intake
queue capacity
duplicate operation rejection
stale RPC evidence blocking
handoff to rox-anchor-proof
redacted report shape
readiness checks
```

Important invariant:

```text
Coordinator orchestrates only.
It must not create separate acceptance rules.
```

---

## Phase 7 — `rox-anchor-relayer`

Status:

```text
COMPLETE / GREEN
```

Current tests:

```text
6 passing
```

Current behavior:

```text
local dry-run submission model
bounded retry policy
idempotency replay rejection
receipt generation
blocked proof review does not submit
accepted proof review produces dry-run receipt
redacted receipt report
readiness checks
```

Important invariant:

```text
Relayer remains dry-run only.
No live RPC submission.
No transaction submission.
No production settlement.
```

---

## Phase 8 — Anchor program compile foundation

Status:

```text
COMPLETE / GREEN
```

Current active Anchor files:

```text
programs/rox-anchor/Cargo.toml
programs/rox-anchor/src/errors.rs
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/instructions/halt.rs
programs/rox-anchor/src/instructions/initialize.rs
programs/rox-anchor/src/instructions/mod.rs
programs/rox-anchor/src/instructions/observe_burn.rs
programs/rox-anchor/src/instructions/open_challenge.rs
programs/rox-anchor/src/instructions/recover.rs
programs/rox-anchor/src/instructions/resolve_challenge.rs
programs/rox-anchor/src/lib.rs
programs/rox-anchor/src/state.rs
```

Current proof:

```text
cargo check -p rox-anchor is green.
anchor build is green.
```

`programs/rox-anchor/src/lib.rs` includes:

```rust
#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![forbid(unsafe_code)]
```

This suppresses Anchor/Solana macro cfg/deprecated noise while keeping unsafe code forbidden.

---

## Phase 9 — Anchor state rules

Status:

```text
COMPLETE / GREEN
```

Implemented instruction/state surfaces:

```text
initialize
observe_burn
open_challenge
resolve_challenge
halt
recover
finalize
```

State blockers implemented:

```text
halted config blocks unsafe observation/finalization
config recovery_required blocks unsafe observation/finalization
operation challenge_open blocks finalization
accepted challenge blocks finalization
operation recovery_required blocks finalization
finalized operation cannot be reopened
wrong authority cannot halt/recover/finalize
invalid/corrupt direction blocks finalization
zero amount blocks finalization
zero burn evidence hash blocks finalization
wrong mint blocks finalization
wrong token account binding is rejected
wrong PDA account/bump is rejected
```

---

## Phase 10 — ROX / ROC mint-burn program logic

Status:

```text
ACTIVE / LOCAL STATE-MACHINE SEMANTICS ADVANCED / NOT COMPLETE
```

Phase 10 build-plan target:

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

Current Phase 10 local implementation is strong for:

```text
direction binding
mint binding
token account binding
amount binding
burn evidence binding
config rox_mint binding
mint authority config storage
operation PDA replay-binding
authority-gated finalize
settlement action classification
finalize plan snapshots
finalized event integrity
direction-specific finalize routes
```

Still not implemented:

```text
anchor-spl dependency
SPL Token account constraints
SPL mint account constraints
token account owner validation
token account mint validation
mint authority PDA signer seeds for CPI
actual mint_to CPI
actual burn or transfer evidence CPI
local validator transaction tests
production deployment
live mint/burn/settlement
```

---

## 5. Current Anchor program state details

## `RoxAnchorConfig`

Current fields:

```rust
pub struct RoxAnchorConfig {
    pub authority: Pubkey,
    pub rox_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub halted: bool,
    pub recovery_required: bool,
}
```

Current helpers:

```text
initialize(authority, InitializeConfigArgs)
require_authority(authority)
require_mint_authority(mint_authority)
require_rox_mint(mint)
require_configured_for_operation(operation)
require_observation_open()
halt(authority)
recover(authority)
```

Current config validations:

```text
InitializeConfigArgs rejects default rox_mint.
InitializeConfigArgs rejects default mint_authority.
Config stores mint_authority_bump.
Config rejects wrong authority.
Config rejects wrong mint authority.
Config rejects wrong rox_mint.
Config recovery/halt states block unsafe behavior.
```

## `RoxAnchorOperation`

Current fields:

```rust
pub struct RoxAnchorOperation {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub direction: u8,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
    pub operation_bump: u8,
    pub state: u8,
    pub challenge_open: bool,
    pub recovery_required: bool,
}
```

Current PDA/replay binding:

```rust
pub const SEED_PREFIX: &'static [u8] = b"rox-anchor-operation";
```

PDA derivation uses:

```text
RoxAnchorOperation::SEED_PREFIX
config key
operation_id_hash
program id
```

Transition contexts use:

```text
seeds = [
  RoxAnchorOperation::SEED_PREFIX,
  config.key().as_ref(),
  operation.operation_id_hash.as_ref(),
]
bump = operation.operation_bump
```

Current PDA helpers:

```text
derive_address(program_id, config_key, operation_id_hash)
require_derived_address(program_id, config_key, operation_account)
```

Current operation helpers:

```text
initialize(authority, args)
initialize_with_bump(authority, args, operation_bump)
state_code()
direction_code()
is_roc_to_rox()
is_rox_to_roc()
require_direction(expected)
require_roc_to_rox()
require_rox_to_roc()
requires_internal_roc_burn_evidence()
requires_external_rox_burn_evidence()
settlement_action()
requires_rox_mint_output()
requires_internal_roc_release()
require_finalizable(config)
finalize_plan(config)
finalize_for_direction(config, expected)
require_finalizable_roc_to_rox(config)
require_finalizable_rox_to_roc(config)
require_not_finalized()
require_binding(args)
open_challenge()
resolve_challenge(accepted)
mark_recovery_required()
recover()
can_finalize(config)
finalize(config)
```

## `OperationBindingArgs`

Current fields:

```rust
pub struct OperationBindingArgs {
    pub operation_id_hash: [u8; 32],
    pub direction: AnchorTransferDirection,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
}
```

Validation rejects:

```text
operation_id_hash == [0; 32]
mint == Pubkey::default()
token_account == Pubkey::default()
amount_atoms == 0
burn_evidence_hash == [0; 32]
```

It also includes direction helpers:

```text
is_roc_to_rox()
is_rox_to_roc()
requires_internal_roc_burn_evidence()
requires_external_rox_burn_evidence()
```

## `AnchorTransferDirection`

Current variants:

```rust
pub enum AnchorTransferDirection {
    RocToRox,
    RoxToRoc,
}
```

Current mapping:

```text
RocToRox -> 1
RoxToRoc -> 2
```

## `OperationStateCode`

Current variants:

```rust
Observed
ChallengeOpen
ChallengeAccepted
ChallengeRejected
RecoveryRequired
RecoveryResolved
Finalized
```

Current mapping:

```text
Observed           -> 1
ChallengeOpen     -> 2
ChallengeAccepted -> 3
ChallengeRejected -> 4
RecoveryRequired  -> 5
RecoveryResolved  -> 6
Finalized         -> 7
```

## `AnchorSettlementAction`

Current variants:

```rust
MintRoxForRocBurn
ReleaseRocForRoxBurn
```

Current mapping:

```text
MintRoxForRocBurn    -> 1
ReleaseRocForRoxBurn -> 2
```

Direction-to-settlement mapping:

```text
RocToRox -> MintRoxForRocBurn
RoxToRoc -> ReleaseRocForRoxBurn
```

---

## 6. Current finalize plan and event integrity model

## `AnchorFinalizePlan`

Current fields:

```rust
pub struct AnchorFinalizePlan {
    pub operation_id_hash: [u8; 32],
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
    pub settlement_action: AnchorSettlementAction,
    pub requires_rox_mint_output: bool,
    pub requires_internal_roc_release: bool,
}
```

Current validation helpers:

```text
require_consistent()
require_matches_operation(operation)
```

`require_consistent()` enforces:

```text
MintRoxForRocBurn:
  requires_rox_mint_output == true
  requires_internal_roc_release == false

ReleaseRocForRoxBurn:
  requires_rox_mint_output == false
  requires_internal_roc_release == true
```

`require_matches_operation()` enforces snapshot matching:

```text
operation_id_hash
direction
mint
token_account
amount_atoms
burn_evidence_hash
settlement_action
requires_rox_mint_output
requires_internal_roc_release
```

`finalize_plan(config)` now constructs the full terminal snapshot and immediately validates it with:

```rust
plan.require_matches_operation(self)?;
```

## `RoxAnchorFinalized`

Current fields:

```rust
pub struct RoxAnchorFinalized {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub settlement_action: u8,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
    pub requires_rox_mint_output: bool,
    pub requires_internal_roc_release: bool,
}
```

Current event builder:

```rust
RoxAnchorFinalized::from_operation_plan(authority, operation, plan)
```

It enforces:

```text
operation must already be Finalized
event authority must match operation.authority
plan must match operation snapshot through AnchorFinalizePlan::require_matches_operation()
event emits from plan snapshot, not mutable operation fields
```

This protects against:

```text
pre-finalized event construction
wrong authority event construction
operation_id tamper after plan
direction tamper after plan
mint tamper after plan
token account tamper after plan
amount tamper after plan
burn evidence tamper after plan
stale/wrong direction plan
inconsistent plan flags
```

---

## 7. Current finalize instruction routes

`programs/rox-anchor/src/instructions/finalize.rs` now has:

```text
handler(ctx)
handler_roc_to_rox(ctx)
handler_rox_to_roc(ctx)
finalize_with_direction(ctx, expected_direction)
```

The generic route remains:

```rust
pub fn handler(ctx: Context<Finalize>) -> Result<()> {
    finalize_with_direction(ctx, None)
}
```

Direction-specific routes:

```rust
pub fn handler_roc_to_rox(ctx: Context<Finalize>) -> Result<()> {
    finalize_with_direction(ctx, Some(AnchorTransferDirection::RocToRox))
}

pub fn handler_rox_to_roc(ctx: Context<Finalize>) -> Result<()> {
    finalize_with_direction(ctx, Some(AnchorTransferDirection::RoxToRoc))
}
```

`finalize_with_direction` enforces:

```text
configured authority signer
PDA-bound operation account
optional expected direction
state transition via operation.finalize_for_direction(...) or operation.finalize(...)
finalized event built through RoxAnchorFinalized::from_operation_plan(...)
```

`programs/rox-anchor/src/lib.rs` exposes:

```rust
pub fn finalize(ctx: Context<Finalize>) -> Result<()>
pub fn finalize_roc_to_rox(ctx: Context<Finalize>) -> Result<()>
pub fn finalize_rox_to_roc(ctx: Context<Finalize>) -> Result<()>
```

This gives explicit Anchor instruction surfaces for each direction without adding token CPI yet.

---

## 8. Current `rox-anchor` test coverage

Current `rox-anchor` tests: **54 passing**.

Important groups covered:

### Config/mint authority

```text
config_initialize_records_mint_authority_binding
config_initialize_rejects_empty_mint_bindings
config_mint_authority_helper_rejects_wrong_authority
config_rox_mint_helper_rejects_wrong_mint
config_rox_mint_mismatch_blocks_finalize
```

### Basic operation binding/state

```text
initialize_records_local_mint_burn_binding
initialize_rejects_empty_binding_parts
initialize_with_bump_records_operation_pda_bump
binding_mismatch_is_rejected
operation_state_code_round_trips
direction_code_round_trips
operation_seed_prefix_is_stable
```

### PDA/replay binding

```text
operation_pda_derivation_is_stable_and_binding_checked
operation_pda_derivation_changes_with_config_or_operation_hash
operation_pda_derivation_rejects_wrong_account_or_bump
```

### Challenge/halt/recovery/finality blockers

```text
challenge_resolution_controls_finalization
accepted_challenge_blocks_finalization
halt_and_recovery_are_explicit_blockers
halted_config_rejects_direction_specific_finalization
operation_can_finalize_only_when_clear
finalized_operations_cannot_be_reopened
wrong_authority_cannot_halt_or_recover_config
invalid_config_binding_blocks_finalize
```

### Direction-specific helpers

```text
roc_to_rox_direction_helper_accepts_expected_direction
rox_to_roc_direction_helper_accepts_expected_direction
wrong_direction_is_rejected_by_specific_helpers
direction_specific_helpers_reject_corrupt_direction
```

### Settlement/finalization classification

```text
settlement_action_maps_direction_to_local_finalize_intent
roc_to_rox_finalization_classifies_rox_mint_output
rox_to_roc_finalization_classifies_internal_roc_release
wrong_mint_rejects_direction_specific_finalize_path
corrupt_mint_burn_binding_blocks_finalize
corrupt_direction_rejects_finalize_plan_classification
```

### Direction-specific finalize routes

```text
finalize_for_direction_accepts_expected_roc_to_rox_path
finalize_for_direction_accepts_expected_rox_to_roc_path
finalize_for_direction_rejects_wrong_direction_without_transition
finalize_authority_gate_rejects_wrong_signer_before_transition
```

### Finalize plan snapshot integrity

```text
inconsistent_finalize_plan_flags_are_rejected
finalize_plan_matches_finalized_operation_snapshot
finalize_plan_rejects_operation_snapshot_mismatch
finalize_plan_rejects_action_flag_mismatch_against_operation
```

### Finalized event integrity

```text
finalized_event_shape_carries_finalize_plan_flags
rox_to_roc_finalized_event_shape_carries_finalize_plan_flags
finalized_event_builder_rejects_pre_finalized_operation
finalized_event_builder_rejects_wrong_authority
finalized_event_builder_rejects_stale_or_wrong_direction_plan
finalized_event_builder_rejects_inconsistent_plan_flags
finalized_event_builder_rejects_operation_id_tamper_after_plan
finalized_event_builder_rejects_direction_tamper_after_plan
finalized_event_builder_rejects_mint_tamper_after_plan
finalized_event_builder_rejects_token_account_tamper_after_plan
finalized_event_builder_rejects_amount_tamper_after_plan
finalized_event_builder_rejects_burn_evidence_tamper_after_plan
```

---

## 9. Recent Phase 10 round history

Useful recent round history:

```text
Round 2H:
  Finalized event expanded to include settlement plan flags.

Round 2I:
  Finalized event builder requires operation state == Finalized.

Round 2J:
  Finalized event builder rejects wrong authority.

Round 2K:
  AnchorFinalizePlan captures full terminal snapshot:
    operation_id_hash
    direction
    mint
    token_account
    amount_atoms
    burn_evidence_hash
    settlement_action
    output flags

Round 2L:
  Completed snapshot tamper coverage for operation ID, direction, and mint.

Round 2M:
  Centralized snapshot validation in AnchorFinalizePlan::require_matches_operation(...).

Round 2N:
  finalize_plan(...) now uses the centralized snapshot validator.

Round 2O:
  Added direction-specific finalize routes:
    finalize_roc_to_rox
    finalize_rox_to_roc

Round 2P:
  Added ROX→ROC success coverage for finalize_for_direction.
```

Important debugging history:

```text
Round 2M initially had one test-data bug:
  operation_id_hash was changed to [7;32], but the fixture already used [7;32].
Fix:
  mutate the first byte relative to plan.operation_id_hash.
Result:
  50/50 green.

Round 2O first attempt did not apply because an exact state.rs marker did not match.
The command block kept running because it did not begin with set -e.
Fix:
  use set -e and brace-aware insertion.
Result:
  53/53 green.

Round 2P landed cleanly.
Result:
  54/54 green.
```

Important patch style lesson:

```text
Use `set -e` at the top of future bash patch blocks.
Prefer brace-aware Python edits over fragile exact multi-line markers for state.rs.
```

---

## 10. Current generated bundle / script status

`scripts/make_codebundle.sh` exists and was run after the latest green test suite.

Current behavior:

```text
generates CODEBUNDLE.md
runs secret failsafe scan
excludes ignored build outputs and dependency folders
excludes local Solana keys
excludes generated codebundles
prints "Wrote safe codebundle: CODEBUNDLE.md"
```

Current terminal proof:

```text
== Secret failsafe scan on generated bundle ==
Wrote safe codebundle: CODEBUNDLE.md
```

Do not weaken this script. It is valuable because the repo now includes Anchor/Solana-related paths and should never leak keypair material.

---

## 11. Current limitations and anti-scope

Still not implemented:

```text
anchor-spl dependency
SPL Token account constraints
SPL Mint account constraints
actual token mint CPI
actual token burn CPI
token account owner validation
token account mint validation
mint authority PDA signer seeds for token CPI
local validator transaction tests for token logic
cross-crate integration harness activation
chaos test activation
production deployment
live wallet calls
live RPC submission
production minting/burning
bridge settlement
staking/liquidity/exchange-facing behavior
```

Do not add these as hidden side effects:

```text
live wallet calls
live ledger mutation
live RPC submission
production deployment
production minting
production burning
production settlement
exchange-facing behavior
staking behavior
liquidity behavior
fake success output
fake finality output
client-side authority over state transitions
```

Allowed now:

```text
compile-tested Rust
focused unit tests
local deterministic proof validation
CLI inspection
local RPC evidence models
local coordinator model
local relayer dry-run model
compile-tested Anchor program code
Anchor state rules
local validator tests when ready
integration and chaos tests once harness is real
```

---

## 12. Internal ROC authority rules to preserve

ROX Anchor must not bypass the RustyOnions internal ROC authority model.

Internal ROC authority remains:

```text
svc-wallet     = internal ROC mutation front-door
ron-ledger     = durable economic truth
ron-accounting = snapshots/reports only
svc-rewarder   = capped planning only
ron-policy     = declarative gates only
CrabLink       = display/user intent only
```

Future ROX→ROC must eventually route internal ROC issuance/release through:

```text
svc-wallet -> ron-ledger
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

Future ROC→ROX must eventually require:

```text
explicit user intent
internal ROC burn proof
deterministic proof package
quorum/RPC evidence when applicable
challenge window
halt/recovery checks
Anchor state eligibility
only then mint-side behavior
```

Current work remains local compile/test only.

---

## 13. CrabLink future integration rules

CrabLink is not active in this immediate ROX Anchor coding session, but future integration must follow:

```text
CrabLink displays backend-derived proof/bridge status.
CrabLink does not decide bridge finality.
CrabLink does not create balances.
CrabLink does not create receipts.
CrabLink does not unlock paid content from anchor evidence.
CrabLink cache/localStorage/sessionStorage/IndexedDB are never paid access truth.
CrabLink must show pending/challenge/not-final status honestly.
```

Safe future UI language:

```text
pending verification
pending challenge window
not final
local proof accepted
local dry-run receipt
manual review may be required
experimental anchor path
```

Forbidden future UI language until actually authorized/proven:

```text
swap
exchange
cash out
guaranteed
instant
risk-free
redeem guaranteed value
final before proof/challenge close
```

---

## 14. Recommended next-session first command block

Start next session by verifying the new green baseline:

```bash
set -e
export GIT_PAGER=cat
export PAGER=cat

cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

Expected result:

```text
rox-anchor: 54/54 passing
workspace tests green
anchor build green
anchor test green
```

If anything fails:

```text
Fix the first failure only.
Do not expand scope.
Do not add anchor-spl until the existing local state-machine remains green.
```

---

## 15. Recommended next work

Recommended next patch:

```text
Phase 10 Round 2Q — local token-account / mint-account preparation without SPL CPI.
```

Purpose:

```text
Prepare the Anchor state machine for token-account validation before adding anchor-spl.
Keep it local and compile-tested.
Do not introduce actual token CPI yet.
```

Safe options for Round 2Q:

```text
Option A:
  Add local helper methods that classify which token-side account checks will be needed for each direction.

Option B:
  Add explicit "settlement output requirements" helper methods on AnchorFinalizePlan:
    requires_rox_mint_output()
    requires_internal_roc_release()
    requires_token_cpi()
    requires_internal_ledger_release()

Option C:
  Add tests proving finalize_for_direction does not mutate state on mismatch, halt, recovery, challenge, or config-mint mismatch for both directions.

Option D:
  Add a small local-only token binding intent struct:
    AnchorTokenSettlementBinding
  with fields:
    mint
    token_account
    mint_authority
    direction
    settlement_action
  and tests that it derives from AnchorFinalizePlan + RoxAnchorConfig.
```

Best next patch recommendation:

```text
Phase 10 Round 2Q:
  Add AnchorTokenSettlementBinding as a local-only intent model.
  Derive it from RoxAnchorConfig + AnchorFinalizePlan.
  Validate:
    plan.mint == config.rox_mint
    mint_authority == config.mint_authority
    RocToRox requires ROX mint output
    RoxToRoc requires internal ROC release, not ROX mint output
  Add focused tests.
```

Why this is the best next step:

```text
It advances Phase 10 toward token-account/CPI preparation.
It does not add anchor-spl yet.
It does not perform value movement.
It keeps compile/test green.
It creates a clean seam before SPL account constraints.
```

Expected test growth if implemented:

```text
54 -> 56 or 57 rox-anchor tests
```

Do not jump straight to:

```text
anchor-spl
mint_to CPI
burn CPI
local validator token tests
devnet
production deployment
```

Those come later.

---

## 16. Later Phase 10 roadmap

After Round 2Q, likely sequence:

### Phase 10 Round 2R — Token account constraint preparation

Possible work:

```text
Add anchor-spl dependency only if needed.
Add TokenAccount/Mint account context fields behind compile-tested instruction contexts.
Do not call mint_to/burn yet.
Add account constraint compile tests where possible.
```

Risk:

```text
Anchor macro/account constraints may produce compile failures.
Keep patch small.
```

### Phase 10 Round 2S — Mint authority PDA seed model

Possible work:

```text
Define mint authority PDA seed prefix.
Derive expected mint authority PDA.
Require config.mint_authority matches derived PDA.
Test stable derivation.
Test config mismatch rejection.
```

This may be better before anchor-spl CPI.

### Phase 10 Round 2T — Local SPL CPI compile-only skeleton

Possible work:

```text
Add account contexts for minting path.
No live production behavior.
Compile only.
```

### Phase 10 Round 3 — Local validator test path

Possible work:

```text
anchor test with local validator
initialize config
observe burn
open/resolve challenge path
finalize direction-specific path
event assertion
```

Still non-production.

### Phase 11 — Integration tests

Activate or move tests for:

```text
local_nonvalue_roc_to_rox
local_nonvalue_rox_to_roc
coordinator_relayer_boundary
crablink_status_display
```

Need real harness because root tests are currently inert under virtual workspace.

### Phase 12 — Chaos tests

Activate or move tests for:

```text
challenge_griefing
coordinator_stale_evidence
halt_resume
relayer_retry_storm
rpc_equivocation
```

### Phase 13 — Final local green run

Run:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

---

## 17. Exact next-session prompt

Use this prompt at the start of the next session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
ROX Anchor is active implementation work. Phases 0–9 are green. Phase 10 is active and advanced through Round 2P. Latest green run:
- cargo test -p rox-anchor: 54/54 passed
- cargo check -p rox-anchor: green
- cargo check --workspace: green
- anchor build: green
- anchor test: green across workspace

The latest codebundle was regenerated after the green run with scripts/make_codebundle.sh and the secret failsafe scan passed.

Please review the attached CODEBUNDLE.md, BUILD_PLAN.md, and latest terminal output. Start by verifying:
set -e
cd /Users/mymac/Desktop/rox-anchor
cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test

If green, continue Phase 10 with a small local-only patch. Recommended next patch: Round 2Q, local token settlement binding intent / token-account preparation without anchor-spl CPI yet. Do not add live RPC, wallet calls, production deployment, production mint/burn, bridge settlement, staking, liquidity, exchange behavior, or fake finality.
```

---

## 18. Short status summary for fast recall

```text
ROX Anchor completion estimate:
  local compile/test implementation: ~80%
  production/live readiness: intentionally 0%

Green:
  Phase 0 workspace compile spine
  Phase 1 core shared types
  Phase 2 proof engine
  Phase 3 crate-local test lift
  Phase 4 CLI check
  Phase 5 RPC proof local model
  Phase 6 coordinator local model
  Phase 7 relayer dry-run model
  Phase 8 Anchor compile foundation
  Phase 9 Anchor state rules
  Phase 10 local binding/finalize/event semantics through Round 2P

Latest tests:
  rox-anchor: 54
  cli unit: 4
  cli integration: 3
  coordinator: 6
  core: 5
  proof: 5
  relayer: 6
  rpc-proof: 7
  doc-tests: 0 / green

Latest generated bundle:
  CODEBUNDLE.md
  94 files
  Git: 0427e11 (dirty)
  secret failsafe scan passed

Current next work:
  Phase 10 Round 2Q
  local-only token settlement binding / token-account preparation
  no SPL CPI yet unless explicitly chosen after a green baseline

Do not:
  git add/commit/push unless explicitly requested
  add live RPC
  add wallet calls
  deploy
  mint/burn real value
  claim bridge finality
```

---

## 19. Exact command block to run first next time

```bash
set -e
export GIT_PAGER=cat
export PAGER=cat

cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

If green, proceed to:

```text
Phase 10 Round 2Q — local token settlement binding intent / token-account preparation, no CPI.
```

These notes are ready to paste into the next session.


### END NOTE - JULY 3 2026 - 19:47 CST


### BEGIN NOTE - JULY 3 2026 - 20:35 CST

Below is the comprehensive next-session handoff. This is written so you can paste it directly into the next ROX Anchor session and continue without re-explaining the project.

# ROX Anchor — Comprehensive Carry-Over Notes for Next Session

## 0. Session opener

We are continuing active implementation work on **RustyOnions / CrabLink / ROX Anchor**.

```text
RustyOnions = backend/substrate/value-plane.
CrabLink = Tauri-first product/client for crab:// links, b3 assets, RON Passports, internal ROC paid access, creator pages, receipts, and future UX surfaces.
ROX Anchor = Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.
```

Current posture:

```text
Implementation-first.
QuickChain-style code/test/check/fix loop.
Real Rust code + focused tests + compile checks + fixes before expansion.
No docs-only gate work.
No placeholder scaffold theater.
No live bridge runtime.
No production mint/burn.
No live wallet calls.
No live RPC submission.
No production deployment.
No staking/liquidity/exchange-facing behavior.
No fake finality or fake success output.
```

The latest attached codebundle was generated from `/Users/mymac/Desktop/rox-anchor`, records Git `88c1bc5`, includes `94` files, and explicitly says the codebundle is not runtime authorization and does not authorize ROX runtime, Solana runtime, bridge runtime, staking, liquidity, exchange-facing logic, or external settlement. 

The build plan remains the authority for phase order. It says every phase should write Rust code, run focused compile/tests, fix failures before moving forward, avoid broad planning docs/decision gates, and avoid live deployment, live wallet calls, live RPC submission, or real value movement until local program/tests are compile-green. 

---

## 1. Current phase and completion estimate

Current phase:

```text
Phase 10 — ROX / ROC Mint-Burn Program Logic
Current round complete: Phase 10 Round 2X
Status: COMPLETE / GREEN through local CPI readiness gate
```

Estimated completion after this session:

```text
Entire ROX Anchor local implementation: ~77–78%
Best single-number estimate: 78%
```

Interpretation:

```text
We are deep into Phase 10, but not done with Phase 10.
Phases 0–9 are effectively green.
Phase 10 is active and significantly advanced.
Phase 11 integration tests, Phase 12 chaos tests, and Phase 13 final local green run remain.
Production/live readiness remains intentionally not authorized.
```

Why it is not higher yet:

```text
Still missing:
- anchor-spl dependency and typed SPL token account/mint contexts
- actual local SPL token CPI path
- mint_to CPI for ROC→ROX
- burn/verified burn mechanics for ROX→ROC
- local validator transaction tests proving token behavior
- cross-crate integration tests
- chaos tests
- final local green run
- production/deployment policy, intentionally deferred
```

---

## 2. Latest verified terminal state

Latest terminal output confirmed:

```text
Phase 10 Round 2X patch status: 0
cargo fmt -p rox-anchor: status 0
cargo test -p rox-anchor: 88 passed
cargo check -p rox-anchor: status 0
cargo check --workspace: status 0
anchor build: status 0
anchor test: status 0
```

Full `anchor test` also confirmed the rest of the workspace suite stayed green:

```text
rox-anchor: 88 tests passed
rox-anchor-cli unit: 4 tests passed
rox-anchor-cli integration check_command: 3 tests passed
rox-anchor-coordinator: 6 tests passed
rox-anchor-core: 5 tests passed
rox-anchor-proof: 5 tests passed
rox-anchor-relayer: 6 tests passed
rox-anchor-rpc-proof: 7 tests passed
doc-tests: all green / zero tests
```

Current exact green summary:

```text
cargo test -p rox-anchor: 88 passed
cargo check -p rox-anchor: green
cargo check --workspace: green
anchor build: green
anchor test: green full suite
```

---

## 3. Phase status summary

### Phase 0 — Workspace Recovery and Compile Spine

```text
Status: COMPLETE / GREEN
```

Accomplished:

```text
- Root workspace compiles.
- Workspace includes all six Rust crates and the Anchor program.
- cargo check --workspace is green.
- Anchor.toml is present and anchor test delegates to cargo test --workspace.
```

Current workspace members:

```text
crates/rox-anchor-core
crates/rox-anchor-proof
crates/rox-anchor-cli
crates/rox-anchor-coordinator
crates/rox-anchor-relayer
crates/rox-anchor-rpc-proof
programs/rox-anchor
```

The codebundle confirms all six crates, the Anchor program, scripts, unit test placeholders, integration test placeholders, and chaos test placeholders are present. 

---

### Phase 1 — Core Type Foundation

```text
Status: COMPLETE / GREEN
```

`rox-anchor-core` owns shared language:

```text
domains
operation IDs
idempotency keys
nonces
cluster IDs
program IDs
mint IDs
token account IDs
directions
lifecycle states
challenge posture
halt posture
recovery posture
safe labels
shared errors
```

Important invariant:

```text
Other crates must not invent duplicate state machines, labels, directions, or acceptance rules.
```

Tests green:

```text
rox-anchor-core: 5 passed
```

---

### Phase 2 — Local Proof Engine

```text
Status: COMPLETE / GREEN
```

`rox-anchor-proof` owns deterministic local proof review.

Behavior covered:

```text
valid package accepted
missing evidence blocks acceptance
binding mismatches rejected
replay operation ID rejected
replay idempotency key rejected
replay nonce rejected
challenge posture blocks acceptance
halt posture blocks acceptance
recovery posture blocks acceptance
quorum disagreement / disputed evidence classified
deterministic finding order
```

Tests green:

```text
rox-anchor-proof: 5 passed
```

---

### Phase 3 — Unit Test Lift

```text
Status: COMPLETE ENOUGH / GREEN FOR CURRENT SURFACE
```

Current reality:

```text
- Active tests are crate-local.
- Root tests/unit, tests/integration, and tests/chaos exist as future surfaces.
- Because the workspace root is virtual, root-level test files should not be assumed active until intentionally wired through a harness package or moved into crate-level tests.
```

The build plan still lists Phase 11 and Phase 12 root integration/chaos files as intended future work. 

---

### Phase 4 — CLI Inspection Tool

```text
Status: COMPLETE / GREEN
```

Current behavior:

```text
cargo run -p rox-anchor-cli -- check
```

This uses real proof/core behavior, not fake success strings.

CLI tests green:

```text
rox-anchor-cli unit: 4 passed
rox-anchor-cli integration check_command: 3 passed
```

Current CLI fixtures already prove:

```text
valid fixture prints accepted review
mismatch fixture prints rejection findings
halt fixture prints halt-blocked review
challenge fixture reports blocked status
replay fixture uses real replay rejection
unknown command is error
default command runs check
```

Future CLI work, later:

```text
Add JSON input support once integration/chaos surfaces are stable.
Do not add live wallet/RPC behavior to CLI.
```

---

### Phase 5 — RPC Proof Local Evidence Model

```text
Status: COMPLETE / GREEN
```

Current behavior:

```text
local RPC observation model
readiness validation
commitment classification
matching observation agreement
binding mismatch rejection
stale observation rejection
too-few matching sources rejection
conflicting signature / equivocation detection
redacted report shape
```

Tests green:

```text
rox-anchor-rpc-proof: 7 passed
```

Still local only:

```text
No live RPC calls.
No live Solana submission.
No production evidence fetch.
```

---

### Phase 6 — Coordinator Local Model

```text
Status: COMPLETE / GREEN
```

Current behavior:

```text
local observer input model
queue capacity enforcement
duplicate operation rejection
stale RPC evidence rejection
handoff to rpc-proof and proof engine
accepted observations produce proof review
redacted report preserves decision shape
```

Tests green:

```text
rox-anchor-coordinator: 6 passed
```

Important invariant:

```text
Coordinator must keep calling core/proof/rpc-proof behavior.
It must not invent separate bridge acceptance rules.
```

---

### Phase 7 — Relayer Local Dry-Run Model

```text
Status: COMPLETE / GREEN
```

Current behavior:

```text
bounded retry policy
readiness rejects zero limits
blocked proof review does not submit
accepted proof review produces dry-run receipt
redacted receipt report keeps local shape
idempotency replay rejected
```

Tests green:

```text
rox-anchor-relayer: 6 passed
```

Still dry-run only:

```text
No live transaction submission.
No live Solana RPC.
No production relay.
No fake live success.
```

---

### Phase 8 — Anchor Program Compile Foundation

```text
Status: COMPLETE / GREEN
```

Current Anchor program files:

```text
programs/rox-anchor/src/lib.rs
programs/rox-anchor/src/errors.rs
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/initialize.rs
programs/rox-anchor/src/instructions/observe_burn.rs
programs/rox-anchor/src/instructions/open_challenge.rs
programs/rox-anchor/src/instructions/resolve_challenge.rs
programs/rox-anchor/src/instructions/halt.rs
programs/rox-anchor/src/instructions/recover.rs
programs/rox-anchor/src/instructions/finalize.rs
```

Current proof:

```text
cargo check -p rox-anchor: green
anchor build: green
anchor test: green
```

---

### Phase 9 — Anchor Program State Rules

```text
Status: COMPLETE / GREEN
```

State rules implemented and tested:

```text
initialize config
observe burn / operation binding
open challenge
resolve challenge
halt
recover
finalize only when eligible
reject challenge-open finalization
reject accepted-challenge finalization
reject halted finalization
reject recovery-required finalization
reject wrong authority
reject wrong mint/config binding
reject corrupt direction
reject zero amount
reject zero burn evidence hash
reject finalized operation reopening
```

Important current state types:

```text
RoxAnchorConfig
RoxAnchorOperation
OperationBindingArgs
OperationStateCode
AnchorTransferDirection
AnchorSettlementAction
AnchorFinalizePlan
AnchorTokenSettlementBinding
AnchorTokenAccountConstraintSnapshot
AnchorTokenSettlementExecutionPlan
AnchorTokenSettlementExecutionReceipt
AnchorTokenCpiReadiness
```

---

## 4. What we did in this session

This session advanced Phase 10 from local binding/finalization semantics into a much more complete local settlement-readiness chain.

Session started with earlier state around Phase 10 Round 2T and continued through:

```text
Round 2U — token-settlement execution plan
Round 2V — execution receipt
Round 2W — planned settlement event snapshot
Round 2X — token CPI readiness gate
```

Everything remained local, deterministic, compile-tested, and non-value-moving.

---

## 5. Round-by-round details from this session

### Phase 10 Round 2U — Token-settlement execution plan

```text
Status: COMPLETE / GREEN
```

Added:

```text
AnchorTokenSettlementExecutionKind
AnchorTokenSettlementExecutionPlan
```

Execution kinds:

```text
MintRoxToTokenAccount
VerifyRoxBurnForInternalRocRelease
```

Purpose:

```text
Create a deterministic local execution-plan layer from:
- derived config
- program ID
- config key
- token settlement binding
- finalize plan
- token account constraint snapshot
```

Behavior:

```text
ROC→ROX maps to MintRoxToTokenAccount.
ROX→ROC maps to VerifyRoxBurnForInternalRocRelease.
PDA-derived mint authority must be valid.
Settlement must match config.
Token account snapshot must match settlement and finalize plan.
ROX→ROC requires token account amount >= plan amount.
```

Rejected:

```text
wrong program ID
wrong config key
manual/wrong mint authority
wrong snapshot mint
empty token account owner
short ROX balance
tampered plan flags
```

---

### Phase 10 Round 2V — Execution receipt

```text
Status: COMPLETE / GREEN
```

Added:

```text
AnchorTokenSettlementExecutionReceipt
```

Purpose:

```text
Create a deterministic local receipt-like snapshot from:
- finalized operation
- finalize plan
- execution plan
```

Important fields:

```text
operation_id_hash
execution_kind
direction
mint
token_account
token_account_owner
amount_atoms
mint_authority
mint_authority_bump
used_mint_authority_pda
token_mint_cpi_planned
internal_roc_release_planned
live_value_moved
```

Important invariant:

```text
live_value_moved = false
```

Behavior:

```text
ROC→ROX receipt records local planned mint path.
ROX→ROC receipt records local internal ROC release-review path.
Receipt validates operation/plan/execution consistency.
Receipt rejects fake or inconsistent direction flags.
Receipt rejects execution plan tamper.
Receipt rejects operation/finalize plan mismatch.
```

---

### Phase 10 Round 2W — Planned settlement event snapshot

```text
Status: COMPLETE / GREEN
```

Added:

```text
RoxAnchorTokenSettlementPlanned
```

Purpose:

```text
Create an Anchor event-safe planned-settlement snapshot from:
- finalized operation
- execution receipt
```

Behavior:

```text
Event records ROC→ROX planned local mint receipt.
Event records ROX→ROC planned local release receipt.
Event requires operation already finalized.
Event requires authority match.
Event requires operation ID, direction, mint, token account, and amount to match.
Event requires PDA mint authority posture.
Event requires non-empty token account owner.
Event rejects fake live-value movement.
```

Rejected:

```text
pre-finalized operation
receipt token account tamper
receipt amount tamper
fake live_value_moved = true
wrong authority
missing PDA authority flag
invalid mint/release flags
invalid execution kind
```

Current event posture:

```text
This is event-safe local planning only.
It does not emit actual SPL mint/burn CPI yet.
```

---

### Phase 10 Round 2X — Token CPI readiness gate

```text
Status: COMPLETE / GREEN
```

Added:

```text
AnchorTokenCpiReadiness
```

Purpose:

```text
Create a deterministic local readiness gate for a future SPL CPI path.
```

This checks consistency across:

```text
RoxAnchorConfig
program_id
config_key
AnchorTokenSettlementExecutionReceipt
RoxAnchorTokenSettlementPlanned event
```

Behavior:

```text
ROC→ROX readiness accepts planned local token mint CPI posture.
ROX→ROC readiness accepts internal ROC release review posture.
Readiness requires derived mint authority PDA.
Readiness requires receipt/event operation binding match.
Readiness requires receipt/event direction match.
Readiness requires receipt/event mint match and config.rox_mint match.
Readiness requires token account match.
Readiness requires non-default token account owner.
Readiness requires amount > 0.
Readiness requires mint authority and bump match config.
Readiness requires PDA flag.
Readiness rejects live value moved.
```

Helpers added:

```text
mint_authority_bump_bytes()
is_ready_for_local_token_mint_cpi()
is_ready_for_internal_roc_release_review()
```

Rejected:

```text
wrong program ID
wrong config key
receipt/event token account mismatch
receipt/event amount mismatch
invalid execution kind
mint authority tamper
fake live_value_moved = true
missing PDA flag
```

Current critical invariant:

```text
AnchorTokenCpiReadiness has requires_anchor_spl = true,
but the program still does not depend on anchor-spl and does not call mint_to or burn.
It is a readiness model, not execution.
```

The current codebundle shows `AnchorTokenCpiReadiness` carrying execution kind, direction, mint, token account, token account owner, amount, mint authority, bump, PDA flag, future anchor-spl requirement, CPI/release flags, and live-value movement posture. 

---

## 6. Current `rox-anchor` test surface after this session

Current Anchor program tests:

```text
88 total rox-anchor tests
88 passed
0 failed
```

Major test categories now covered:

```text
authority helpers
config mint authority binding
config ROX mint binding
config initialization rejection for empty mint bindings
config mismatch blocks finalize
challenge blocks finalization
halt and recovery blockers
operation state round trips
direction code round trips
direction-specific helper acceptance
corrupt direction rejection
operation PDA derivation
operation PDA rejection for wrong account/bump
mint authority PDA derivation
mint authority PDA signer seeds
mint authority PDA rejection for wrong bump/empty binding
local mint-burn binding initialization
operation finalize eligibility
finalized operation cannot reopen
finalize plan matching
finalize plan tamper rejection
finalized event shape
finalized event builder tamper rejection
token settlement binding derivation
derived settlement binding PDA authority
derived binding rejection for manual authority/wrong program/config
token account constraint snapshot acceptance/rejection
execution plan ROC→ROX and ROX→ROC derivation
execution plan tamper rejection
execution receipt ROC→ROX and ROX→ROC recording
execution receipt tamper rejection
planned settlement event recording
planned settlement event tamper rejection
CPI readiness accepting ROC→ROX planned event
CPI readiness accepting ROX→ROC planned event
CPI readiness rejecting program/config mismatch
CPI readiness rejecting receipt/event mismatch
CPI readiness rejecting mint authority/live value/PDA tamper
```

---

## 7. Current build plan position

The build plan defines Phase 10 as implementing ROX/ROC mint-burn semantics after the state machine compiles, including ROC burn observation fields, ROX mint authority model, reverse ROX burn fields, ROC release/recovery model, mint binding, token account binding, replay protection, challenge window, halt/recovery controls, and events. 

We have implemented a large amount of Phase 10 local semantics:

```text
Done inside Phase 10:
- ROC burn / reverse burn evidence binding shape
- direction-specific ROC→ROX / ROX→ROC classification
- ROX mint authority PDA model
- config ROX mint binding
- settlement action mapping
- token account binding
- token account constraint snapshot
- finalize plan
- execution plan
- execution receipt
- planned settlement event
- CPI readiness gate
- challenge/halt/recovery controls remain enforced
- replay protection at proof/local relayer and operation-PDA model level
```

Still not done inside Phase 10:

```text
- anchor-spl dependency
- typed Anchor SPL Mint / TokenAccount account contexts
- token_program account constraint
- mint account authority constraint against PDA
- token account mint constraint against rox_mint
- token account owner constraint
- actual local mint_to CPI for ROC→ROX
- actual local burn or burn verification path for ROX→ROC
- local validator transaction tests that create mint/accounts and prove behavior
```

The build plan says Phase 10 exits only when local validator tests prove the mint/burn state machine works without touching production.  We are not at that exit condition yet.

---

## 8. Current safety boundary

Absolutely still true:

```text
No live wallet calls.
No live ledger mutation.
No live RPC submission.
No production deployment.
No production minting.
No production burning.
No bridge settlement.
No staking.
No liquidity.
No exchange-facing behavior.
No fake finality.
No fake success output.
```

Current code is allowed because it is:

```text
local
compile-tested
deterministic
non-value-moving
not production
not live RPC
not live wallet
not live settlement
```

The codebundle itself preserves this boundary by stating it is not runtime authorization and does not authorize ROX/Solana/bridge runtime or external settlement. 

---

## 9. What the completed ROX Anchor path is intended to support

Final intended capability after all later authorized/local-tested work:

```text
ROC → ROX:
CrabLink user explicitly requests/authorizes anchor action
→ internal ROC is burned/locked through RustyOnions wallet/ledger authority
→ proof package is reviewed
→ challenge/halt/recovery windows clear
→ relayer submits eligible Anchor instruction
→ ROX is minted on Solana to the bound token account
```

Reverse:

```text
ROX → ROC:
User burns ROX on Solana
→ Solana burn evidence is observed
→ RPC/quorum/proof review validates it
→ challenge/halt/recovery windows clear
→ RustyOnions internal ROC is released/issued through svc-wallet/ron-ledger
→ CrabLink displays backend-derived receipt/status
```

Current capability:

```text
Not live.
Not value moving.
Not yet SPL CPI.
Current surface is local proof/state/planning/readiness only.
```

---

## 10. What should be done next

Next recommended patch:

```text
Phase 10 Round 2Y — anchor-spl dependency and typed token account/mint context preparation
```

Goal:

```text
Add the first compile-tested SPL-token surface without calling mint_to or burn yet.
```

Recommended scope:

```text
1. Add anchor-spl dependency to programs/rox-anchor/Cargo.toml, matching Anchor 0.31.1.
2. Import typed token account/mint types.
3. Add compile-tested account context shape or helper struct for future token CPI.
4. Ensure config rox_mint, mint_authority PDA, token account, token account owner, amount, and readiness are still required.
5. Add tests that prove the typed-readiness layer remains non-value-moving.
6. Keep live_value_moved = false.
7. Do not call mint_to.
8. Do not call burn.
```

Important caution:

```text
The next patch should prepare account constraints/types, not perform CPI yet.
```

Suggested exact name:

```text
Phase 10 Round 2Y — Anchor SPL typed account readiness
```

Alternative if wanting one more purely local patch before dependency:

```text
Phase 10 Round 2Y — Token CPI instruction intent enum
```

But the stronger next step is likely `anchor-spl` compile preparation, since Round 2X already introduced `requires_anchor_spl = true`.

---

## 11. Suggested next-session first command block

Start the next session with:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

Expected result:

```text
rox-anchor: 88 passed
workspace check: green
anchor build: green
anchor test: green
```

Then proceed to the next Phase 10 patch only if this is green.

---

## 12. Suggested next patch sequence after Round 2X

### Round 2Y — Anchor SPL typed readiness

```text
Add anchor-spl dependency and typed account/context prep.
No CPI calls yet.
No mint_to.
No burn.
No value movement.
```

Expected tests:

```text
typed SPL readiness accepts ROC→ROX readiness
typed SPL readiness accepts ROX→ROC readiness/review path
rejects missing PDA posture
rejects wrong mint
rejects wrong token account
rejects wrong owner
rejects fake live value moved
```

---

### Round 2Z — CPI context shape for ROC→ROX mint

```text
Define the account context needed for future mint_to:
- config
- operation
- authority
- rox_mint
- recipient token account
- mint authority PDA
- token program
```

Still no CPI call unless explicitly scoped.

Expected tests:

```text
context/readiness accepts exact mint/token/PDA binding
wrong mint rejected
wrong recipient token account rejected
wrong mint authority rejected
operation not finalized rejected
challenge/halt/recovery blockers still reject
```

---

### Round 3A — Local mint_to CPI behind test-only/local path

```text
Add actual mint_to only after typed contexts compile cleanly.
Use local validator tests only.
No production deployment.
No live external bridge.
```

Expected tests:

```text
local validator creates mint/account
ROC→ROX finalized eligible operation mints exact amount
wrong authority cannot mint
wrong token account cannot mint
halted config cannot mint
challenge-open operation cannot mint
replay/finalized duplicate cannot mint twice
```

---

### Round 3B — ROX→ROC burn verification / burn path

Depending on desired architecture:

```text
Option A:
Anchor program verifies observed burn evidence and finalizes local release event only.

Option B:
Anchor program includes a burn CPI when user submits token account authority locally.

Either way:
Internal ROC release must NOT be done by Anchor directly.
Future internal ROC release routes through RustyOnions svc-wallet → ron-ledger.
```

Expected tests:

```text
burn evidence matches operation
wrong mint rejected
wrong token account rejected
short balance rejected
wrong direction rejected
internal release remains planned/off-chain, not Anchor-mutated
```

---

### Phase 11 — Integration tests

The build plan says Phase 11 should wire proof engine to coordinator, coordinator to relayer dry-run, RPC proof evidence into proof review, and test local ROC→ROX / ROX→ROC paths plus status labels. 

Recommended Phase 11 work:

```text
- Decide root test harness strategy.
- Either create a dedicated workspace test crate or move integration tests into crate-level tests.
- Wire proof → rpc-proof → coordinator → relayer dry-run → Anchor local state helpers.
- Keep all integration tests non-value-moving unless local validator CPI tests are explicitly in scope.
```

Existing integration placeholder files:

```text
tests/integration/local_nonvalue_roc_to_rox.rs
tests/integration/local_nonvalue_rox_to_roc.rs
tests/integration/coordinator_relayer_boundary.rs
tests/integration/crablink_status_display.rs
```

---

### Phase 12 — Chaos tests

The build plan says Phase 12 should prove bad conditions fail cleanly, including challenge griefing, stale coordinator evidence, halt/resume, relayer retry storms, and RPC equivocation. 

Existing chaos placeholder files:

```text
tests/chaos/challenge_griefing.rs
tests/chaos/coordinator_stale_evidence.rs
tests/chaos/halt_resume.rs
tests/chaos/relayer_retry_storm.rs
tests/chaos/rpc_equivocation.rs
```

Recommended chaos test behavior:

```text
challenge griefing remains bounded
stale coordinator evidence is rejected
halt/resume remains deterministic
relayer retry storms stay bounded
RPC equivocation is detected
wrong cluster/program/mint/token account cannot pass
fake finality cannot pass
```

---

### Phase 13 — Final local green run

The build plan’s final phase requires:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
repo compile-green
repo test-green
proof engine works
CLI works
local service models work
Anchor program compiles
mint/burn state machine is locally tested
```

---

## 13. Files most likely touched next

Likely next patch files:

```text
programs/rox-anchor/Cargo.toml
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/instructions/mod.rs
programs/rox-anchor/src/lib.rs
programs/rox-anchor/src/events.rs
```

Possible later files:

```text
programs/rox-anchor/src/instructions/mint_rox.rs
programs/rox-anchor/src/instructions/verify_rox_burn.rs
programs/rox-anchor/src/instructions/settle.rs
tests/integration/local_nonvalue_roc_to_rox.rs
tests/integration/local_nonvalue_rox_to_roc.rs
tests/chaos/*
```

Do not create broad new docs unless explicitly requested.

---

## 14. Current implementation chain to preserve

The current local Phase 10 chain is:

```text
RoxAnchorOperation
→ AnchorFinalizePlan
→ AnchorTokenSettlementBinding
→ AnchorTokenAccountConstraintSnapshot
→ AnchorTokenSettlementExecutionPlan
→ AnchorTokenSettlementExecutionReceipt
→ RoxAnchorTokenSettlementPlanned
→ AnchorTokenCpiReadiness
```

This chain is good and should not be bypassed.

Future CPI should use this chain rather than recomputing loose strings or trusting raw account values without prior binding.

---

## 15. Critical invariants to preserve in every future patch

```text
Replay must be rejected.
Domain mismatch must be rejected.
Direction mismatch must be rejected.
Cluster mismatch must be rejected.
Program ID mismatch must be rejected.
Mint mismatch must be rejected.
Token account mismatch must be rejected.
Operation ID mismatch must be rejected.
Idempotency key mismatch must be rejected.
Nonce mismatch must be rejected.
Challenge state must block unsafe acceptance/finalization.
Halt state must block unsafe acceptance/finalization.
Recovery state must block unsafe acceptance/finalization.
CLI output must come from proof/core behavior.
Coordinator/RPC proof/relayer must call core/proof.
Anchor program owns on-chain state transitions.
No fake finality.
No fake live value moved.
```

Specific Phase 10 invariants now added:

```text
config.rox_mint must match settlement mint.
config.mint_authority must match derived PDA.
mint_authority_bump must match derived PDA bump.
token_account must match operation/finalize plan.
token_account_mint must match ROX mint.
token_account_owner cannot be default.
amount_atoms must be > 0.
burn_evidence_hash cannot be zero.
ROC→ROX must require token mint CPI planning and not internal ROC release.
ROX→ROC must require internal ROC release planning and not token mint CPI.
live_value_moved must remain false until actual local CPI phase.
```

---

## 16. Important anti-scope reminders for the next assistant

Do not do these unless explicitly requested:

```text
git add
git commit
git push
downloads
live wallet calls
live ledger mutation
live RPC submission
production deployment
production minting
production burning
public bridge settlement
staking
liquidity
exchange-facing behavior
fake success output
fake finality output
```

Patch style required:

```text
Use paste-ready terminal patches.
Wrap terminal patches in ~~~~bash fences.
Prefer clear scripted edits or complete file writes.
No Python helpers for repo patches.
No downloads.
After every patch, include exact focused commands to run.
```

---

## 17. Good next-session prompt

Use this to start the next session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
ROX Anchor is active implementation work. Phases 0–9 are effectively green. Phase 10 is active. The latest verified state is Phase 10 Round 2X COMPLETE / GREEN: cargo test -p rox-anchor passed 88/88, cargo check -p rox-anchor green, cargo check --workspace green, anchor build green, and anchor test green.

This session added:
- Round 2U token-settlement execution plan
- Round 2V execution receipt
- Round 2W planned settlement event snapshot
- Round 2X token CPI readiness gate

The current chain is:
RoxAnchorOperation → AnchorFinalizePlan → AnchorTokenSettlementBinding → AnchorTokenAccountConstraintSnapshot → AnchorTokenSettlementExecutionPlan → AnchorTokenSettlementExecutionReceipt → RoxAnchorTokenSettlementPlanned → AnchorTokenCpiReadiness.

Please review CODEBUNDLE.md and BUILD_PLAN.md. Start with:
cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test

If green, continue Phase 10 Round 2Y with anchor-spl typed account readiness preparation. Do not call mint_to or burn yet. Do not add live RPC, live wallet calls, production deployment, production mint/burn, settlement, staking, liquidity, exchange behavior, or fake finality.
```

---

## 18. Exact command block for next session

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
cargo check --workspace
anchor build
anchor test
```

If green, proceed to:

```text
Phase 10 Round 2Y — Anchor SPL typed account readiness preparation
```

---

## 19. Short final summary

```text
Current project phase:
Phase 10 — ROX / ROC Mint-Burn Program Logic

Current completed round:
Phase 10 Round 2X — token CPI readiness gate

Current green proof:
88/88 rox-anchor tests, workspace check green, anchor build green, anchor test green

Current completion estimate:
~77–78%

Next work:
Add anchor-spl typed account/mint readiness surface without mint_to/burn yet, then local CPI context, then local validator token tests, then Phase 11 integration, Phase 12 chaos, Phase 13 final local green run.
```


### END NOTE - JULY 3 2026 - 20:35 CST


### BEGIN NOTE - JULY 3 2026 - 23:15 CST

Use these as the next-session carryover notes.

# ROX Anchor Carryover Notes — Local Plan Complete / BUILD_PLAN2 Ready

## Session Status

We made major progress in this session. The original ROX Anchor local implementation build plan is now complete, green, and ready to park.

Current status:

```text
ROX Anchor local implementation build plan: COMPLETE / GREEN / PARKED
BUILD_PLAN.md: preserve as completed reference
BUILD_PLAN2.md: written and ready as the next active plan
Next workstream: testnet-only production-readiness / security hardening
Public launch: not authorized
Mainnet: not authorized
Public mint/burn: not authorized
Production bridge settlement: not authorized
Exchange/liquidity/staking: not authorized
```

The final local closeout sequence passed:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

The terminal output showed the full workspace suite passing, the Anchor program tests passing, `cargo check --workspace` passing, `anchor build` passing, and `anchor test` passing. The Anchor program suite showed 91 tests passing, with no failures. 

This means the original local implementation plan can be treated as finished and parked. The next session should not restart planning for the local implementation. It should begin from `BUILD_PLAN2.md`.

---

## What We Accomplished In This Session

### 1. Finished Phase 12 — Chaos Tests

We completed the last explicit Phase 12 chaos item: RPC equivocation detection.

Added:

```text
crates/rox-anchor-rpc-proof/tests/rpc_equivocation_chaos.rs
```

The new chaos test target covered:

```text
conflicting RPC signatures are disputed, not accepted as agreement
same-source equivocation is flagged and disputed
repeated equivocation reviews remain deterministic
clean RPC agreement after an equivocation case can still accept
```

The focused test passed:

```text
rpc_equivocation_chaos: 4 passed / 0 failed
```

The broader RPC proof crate also passed:

```text
rox-anchor-rpc-proof unit tests: 7 passed / 0 failed
rpc_to_proof_boundary: 3 passed / 0 failed
rpc_equivocation_chaos: 4 passed / 0 failed
```

This completed the final Phase 12 item:

```text
RPC equivocation is detected
```

Phase 12 status:

```text
Phase 12 — Chaos Tests: COMPLETE / GREEN
```

---

### 2. Ran Phase 13 — Final Local Green Run

After Phase 12 completed, we ran the final closeout sequence:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Everything passed.

The final output confirmed:

```text
rox-anchor program tests: 91 passed
rox-anchor-cli tests: passed
rox-anchor-coordinator tests: passed
rox-anchor-core tests: passed
rox-anchor-proof tests: passed
rox-anchor-relayer tests: passed
rox-anchor-rpc-proof tests: passed
doc-tests: passed
cargo check --workspace: passed
anchor build: passed
anchor test: passed
```

Phase 13 status:

```text
Phase 13 — Final Local Green Run: COMPLETE / GREEN
```

Overall original plan status:

```text
BUILD_PLAN.md: COMPLETE / GREEN / PARKED
```

---

### 3. Created BUILD_PLAN2.md

We created the next build plan:

```text
BUILD_PLAN2.md
```

Title:

```text
ROX Anchor Build Plan 2
Testnet-Only Production Readiness / Security Hardening
```

Purpose:

```text
Move from a local compile-tested implementation to a controlled testnet-only hardening surface.
```

The new plan is explicitly not:

```text
a public launch plan
a mainnet plan
an exchange/liquidity/staking plan
authorization to make public ROX mint/burn available
authorization for production bridge settlement
```

The plan correctly keeps the original completed `BUILD_PLAN.md` around for reference and makes `BUILD_PLAN2.md` the next active workstream. It includes phases from baseline freeze through private testnet readiness gate. 

---

### 4. Verified BUILD_PLAN2.md Completeness

We reviewed `BUILD_PLAN2.md` and confirmed it is complete enough to use as the next-session build plan.

It includes:

```text
precondition that BUILD_PLAN.md is complete / green / parked
clear testnet-only scope
hard forbidden list
allowed testnet/localnet scope
QuickChain-style build rules
local green baseline freeze
testnet safety flags
testnet configuration model
operator key and authority model
read-only live RPC adapter
transaction simulation model
test-only mint/token harness
testnet deployment drill
capped testnet submission path
end-to-end testnet shadow flow
security regression expansion
observability / receipts / audit trail
halt / recovery / kill-switch drills
testnet chaos drills
audit preparation
private testnet readiness gate
final non-authorization for public launch/mainnet/production bridge
```

Only optional future edits were identified:

```text
change Status from draft to active when Phase 0 begins
make Phase 8 wording extra explicit: testnet transaction submission with test-only assets only
make Phase 15 title extra explicit: private testnet pilot only
```

These are optional wording improvements, not missing structural phases.

---

### 5. Counted Current Production Rust LOC

We ran `cloc` on production Rust, excluding `target`, `.git`, `node_modules`, `tests`, and lock files.

Current production Rust count:

```text
Rust files: 58
Blank lines: 1,092
Comment lines: 354
Code lines: 7,328
```

Breakdown:

```text
programs/rox-anchor:        4,814 Rust code lines
crates/rox-anchor-cli:        433 Rust code lines
crates/rox-anchor-coordinator: 768 Rust code lines
crates/rox-anchor-core:       405 Rust code lines
crates/rox-anchor-proof:      823 Rust code lines
crates/rox-anchor-relayer:    460 Rust code lines
crates/rox-anchor-rpc-proof:  669 Rust code lines
```

Interpretation:

```text
ROX Anchor is much leaner than expected.
The majority of code is concentrated in the Anchor program, where state-transition rules belong.
The supporting crates are intentionally small and focused.
The project is serious but not bloated.
```

A reasonable expected size for this scope could have been 12k–20k Rust LOC. The current 7,328 production Rust LOC means the implementation stayed compact while still covering the local proof/coordinator/relayer/Anchor state surfaces.

---

## Current Completion Estimate

### Original local implementation plan

```text
BUILD_PLAN.md completion: 100%
Status: COMPLETE / GREEN / PARKED
```

### BUILD_PLAN2 implementation

```text
BUILD_PLAN2 implementation completion: 0%
Status: written / ready / not started
```

### Overall ROX Anchor path including private testnet readiness

```text
Approximate overall ROX Anchor completion toward private testnet readiness: ~60%
```

Reasoning:

```text
The local implementation foundation is complete.
The Anchor program compiles and tests green.
The proof engine, RPC proof model, coordinator, relayer, and CLI are green.
Integration and chaos coverage are green.
The next major work is testnet-only hardening, not rebuilding the local foundation.
```

What remains is not “make the local thing work.” It is:

```text
testnet safety gates
testnet config model
operator authority/key model
read-only RPC adapter
simulation-before-submit path
test-only mint/token harness
capped testnet submission
private testnet shadow flow
expanded adversarial tests
audit maps
halt/recovery drills
private testnet readiness gate
```

---

## Active Project Boundary Going Forward

This is extremely important for the next session.

We are not launching a public Solana token.

We are not enabling public mint/burn.

We are not enabling production bridge settlement.

We are not making exchange-facing, liquidity, staking, or public bridge behavior.

The next plan is strictly:

```text
localnet + devnet/testnet only
test-only assets only
controlled private testnet pilot readiness only
read-only RPC before simulation
simulation before testnet submission
testnet submission capped, explicit, and gated
no default live submission
no production settlement
no real user value movement
```

Any mainnet/public launch behavior must be a separate future plan after testnet hardening, audit preparation, private testnet pilot proof, and explicit authorization.

---

## Final State Of Original BUILD_PLAN.md

The original build plan produced and proved:

```text
rox-anchor-core shared type foundation
rox-anchor-proof deterministic proof validation
rox-anchor-cli check/status behavior
rox-anchor-rpc-proof local RPC evidence/quorum model
rox-anchor-coordinator local decision model
rox-anchor-relayer dry-run model
compile-tested Anchor program
Anchor state rules
local ROC ↔ ROX mint/burn semantics
local nonvalue bidirectional flows
integration tests
chaos tests
final local green run
```

Critical invariants now covered locally:

```text
replay rejection
idempotency rejection
nonce reuse rejection
domain mismatch rejection
direction mismatch rejection
cluster mismatch rejection
program ID mismatch rejection
mint mismatch rejection
token account mismatch rejection
operation ID mismatch rejection
halt blocks unsafe acceptance/finalization
challenge states block unsafe acceptance/finalization
recovery-required state blocks unsafe acceptance/finalization
stale RPC evidence rejection
missing RPC evidence rejection
disputed RPC evidence rejection
RPC equivocation detection
coordinator stale evidence rejection
relayer retry storm bounding
relayer idempotency replay rejection
receipt capacity bounding
status output avoids fake runtime/settlement claims
Anchor state transition tests pass
```

Important distinction:

```text
The local mint/burn semantics are tested locally.
This is not public mint/burn.
This is not production bridge settlement.
This is not public ROX runtime launch.
```

The ROX→ROC local path must continue to avoid real internal ROC release. Future real ROC release, if ever built, belongs to RustyOnions `svc-wallet -> ron-ledger`, not directly to Anchor or the relayer.

---

## New Active File: BUILD_PLAN2.md

`BUILD_PLAN2.md` should now be the phase/round reference for future ROX Anchor work.

Its build order:

```text
Phase 0  — Freeze local green baseline
Phase 1  — Testnet scope locks and safety flags
Phase 2  — Testnet configuration model
Phase 3  — Operator key and authority safety model
Phase 4  — Read-only live RPC adapter
Phase 5  — Transaction simulation model
Phase 6  — Test-only mint and token account harness
Phase 7  — Testnet deployment drill
Phase 8  — Capped testnet submission path
Phase 9  — End-to-end testnet shadow flow
Phase 10 — Security regression expansion
Phase 11 — Observability, receipts, and audit trail
Phase 12 — Halt, recovery, and kill-switch drills
Phase 13 — Testnet chaos drills
Phase 14 — Audit preparation
Phase 15 — Testnet readiness gate
```

The plan ends at:

```text
Private testnet-only pilot readiness
```

It does not end at:

```text
public token launch
mainnet launch
public bridge
production bridge settlement
exchange/liquidity/staking readiness
```

---

## Recommended Next-Session Opening Prompt

Use something like this in the next session:

```text
We are continuing ROX Anchor.

Current status:
BUILD_PLAN.md is COMPLETE / GREEN / PARKED.
The local ROX Anchor implementation is complete and passed the final local green run:
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test

Current active plan:
BUILD_PLAN2.md — Testnet-Only Production Readiness / Security Hardening.

Strict boundary:
No public Solana token launch.
No mainnet.
No public ROX mint/burn.
No production bridge settlement.
No production ROC release.
No exchange, liquidity, staking, or public bridge UI.
Testnet/localnet only.
Read-only RPC before simulation.
Simulation before capped testnet submission.
Capped testnet submission only after explicit gates and only with test-only assets.

Please start with BUILD_PLAN2 Phase 0:
freeze the local green baseline, confirm BUILD_PLAN.md remains parked, confirm BUILD_PLAN2.md is active, and then move into Phase 1 only after the repo is still green.
Use QuickChain-style implementation: small Rust patch, focused tests, compile/check, fix first failure before expanding.
```

---

## Files To Attach In The Next Session

Attach these first:

```text
BUILD_PLAN2.md
BUILD_PLAN.md
Cargo.toml
Anchor.toml
README.md
TODO.md
IDB.md
latest CODEBUNDLE.md
latest terminal output showing final green run
```

Optional but useful:

```text
COMBINED_IDB.MD
CODEBUNDLE_RS.md if the wider RustyOnions context matters
SESSION_PACK.md if continuing broader RustyOnions/CrabLink linkage
```

For immediate Phase 0 and Phase 1 work, the most important files are:

```text
BUILD_PLAN2.md
BUILD_PLAN.md
Cargo.toml
Anchor.toml
CODEBUNDLE.md
latest terminal output
```

---

## Next Session First Commands

Start from repo root:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

If green, Phase 0 is effectively confirmed.

Then begin Phase 1.

---

## Next Implementation Target: BUILD_PLAN2 Phase 0

Phase 0 is a baseline freeze. It likely does not need much new code unless we decide to add a tiny status note or test to prove the active plan boundary.

Phase 0 work:

```text
1. Confirm current repo green.
2. Confirm BUILD_PLAN.md remains unchanged.
3. Confirm BUILD_PLAN2.md is active.
4. Confirm all new work is testnet/localnet only.
5. Confirm no mainnet/public token behavior exists.
```

Recommended Phase 0 outcome:

```text
Phase 0 — COMPLETE / GREEN
```

Do not overbuild Phase 0.

A small verification script may be useful, but avoid turning it into docs theater.

---

## Next Real Implementation Target: BUILD_PLAN2 Phase 1

Phase 1 is where the real next code begins.

Goal:

```text
Add explicit runtime/config boundaries so testnet code cannot accidentally become public launch code.
```

Likely new shared core types:

```text
AnchorRuntimeMode:
- LocalOnly
- TestnetOnly
- ProductionDisabled

SubmissionMode:
- DryRunOnly
- SimulateOnly
- TestnetSubmitCapped

AllowedCluster:
- Localnet
- Devnet
- Testnet
```

Mainnet-beta must be rejected.

Default mode must be non-submitting.

There must be no public launch flag.

Potential files:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-core/src/state.rs
crates/rox-anchor-core/src/errors.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-rpc-proof/src/config.rs
crates/rox-anchor-coordinator/src/config.rs
```

Possible tests:

```text
runtime_mode_defaults_to_non_submitting
mainnet_beta_cluster_is_rejected
testnet_submit_requires_explicit_mode
production_disabled_rejects_live_submission
public_launch_mode_does_not_exist
submission_modes_classify_live_behavior
```

Suggested commands:

```bash
cargo fmt -p rox-anchor-core
cargo test -p rox-anchor-core

cargo fmt -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-coordinator
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-coordinator

cargo test --workspace
cargo check --workspace
```

Do not add live RPC submission in Phase 1.

Do not add Solana transaction sending in Phase 1.

Do not add key loading in Phase 1.

---

## Recommended Phase 1 Patch Shape

Best first patch in the next session:

```text
Add shared safety mode types to rox-anchor-core.
Add tests proving:
- mainnet-beta is rejected
- default mode is dry-run/non-submitting
- testnet submit mode is explicit
- production remains disabled
- no public launch variant exists
```

Then wire those types into relayer/rpc-proof/coordinator configs in a later patch.

Reasoning:

```text
rox-anchor-core should own shared safety vocabulary.
Other crates should not invent their own competing safety flags.
```

This follows the core invariant from the original plan: core owns shared types and reusable acceptance boundaries.

---

## Recommended BUILD_PLAN2 Phase 1 Round Order

Use small rounds:

```text
Phase 1 Round 1A:
Add core runtime/submission/cluster safety types + tests.

Phase 1 Round 1B:
Wire relayer config to shared safety types; prove default dry-run and mainnet rejection.

Phase 1 Round 1C:
Wire rpc-proof config to shared cluster safety types; prove read-only/testnet-only behavior.

Phase 1 Round 1D:
Wire coordinator config to shared safety types; prove accepted proof cannot become live submission by default.

Phase 1 Round 1E:
Workspace green + Anchor green checkpoint.
```

This keeps the patch sequence clean.

---

## Security Notes For The Next Session

Keep repeating these as implementation boundaries:

```text
No mainnet-beta.
No public token launch.
No public mint/burn.
No production settlement.
No production ROC release.
No exchange-facing behavior.
No staking.
No liquidity.
No public bridge UI.
No live user funds.
No silent live RPC submission.
No silent wallet/key usage.
No operator key material committed to repo.
No fake finality.
No fake success output.
```

Allowed:

```text
localnet
devnet/testnet
test-only mint
test-only accounts
read-only RPC
simulation
explicit capped testnet submission after gates
operator safety models
key rotation drills
halt/recovery drills
monitoring prototypes
audit prep
chaos/security tests
```

---

## Important Design Principle Going Forward

The next workstream must not let “testnet readiness” become “production bridge.”

The correct order is:

```text
local green foundation
→ safety flags
→ testnet config
→ authority model
→ read-only RPC
→ simulation
→ test-only mint harness
→ deployment drill
→ capped testnet submission
→ private shadow flow
→ security regression expansion
→ observability
→ halt/recovery drills
→ chaos drills
→ audit prep
→ private testnet readiness gate
```

Do not skip from read-only RPC directly to submission.

Do not skip from simulation to public mint/burn.

Do not allow CLI success strings to claim settlement.

Do not allow coordinator/relayer to create their own rules instead of using core/proof.

---

## What Not To Do Next

Do not work on:

```text
mainnet deployment
public ROX token launch
public bridge interface
production minting
production burning
production ROC release
staking
liquidity pools
exchange listings
market-making
public faucet
claim page
public bridge UI
```

Do not add:

```text
real keypair loading
unbounded relayer retries
default live transaction submission
mainnet RPC URLs
hard-coded secrets
fake success messages
placeholder-only crates
new broad planning docs instead of code
```

---

## Practical Next-Session Summary

The next session should start like this:

```text
1. Confirm repo green.
2. Mark BUILD_PLAN2 as active mentally, not necessarily by changing status yet.
3. Do Phase 0 quickly.
4. Begin Phase 1 with shared core safety types.
5. Add focused tests.
6. Run focused checks.
7. Fix first failure.
8. Only then wire relayer/rpc-proof/coordinator.
```

Most likely first patch:

```text
rox-anchor-core:
- add runtime/testnet safety mode types
- add cluster allowlist helpers
- add submission mode helpers
- add tests for mainnet rejection and non-submitting defaults
```

Expected first commands after patch:

```bash
cargo fmt -p rox-anchor-core
cargo test -p rox-anchor-core
cargo check --workspace
```

Then continue with relayer/rpc-proof/coordinator config integration.

---

## Final Carryover Status

```text
ROX Anchor local implementation:
COMPLETE / GREEN / PARKED

BUILD_PLAN.md:
completed reference plan; keep it around

BUILD_PLAN2.md:
ready as active next build plan

Next session:
start BUILD_PLAN2 Phase 0, then Phase 1

Current production Rust LOC:
7,328

Current posture:
lean, compact, well-contained, test-green

Main safety boundary:
testnet-only hardening next; no public launch or real value movement
```

These notes are ready to carry into the next session.


### END NOTE - JULY 3 2026 - 23:15 CST



### BEGIN NOTE - JULY 4 2026 - 13:50 CST

Here is the full carry-over note pack for the next ROX Anchor session.

# ROX Anchor — Carry-Over Notes for Next Session

## 0. Session opener

We are continuing work on **RustyOnions / CrabLink / ROX Anchor**.

Current active workstream:

```text
ROX Anchor BUILD_PLAN2:
Testnet-Only Production Readiness / Security Hardening
```

The original `BUILD_PLAN.md` local implementation is complete / green / parked. The active plan is now `BUILD_PLAN2.md`, which moves the already-green local ROX Anchor implementation toward a controlled, private, testnet-only hardening surface. The current codebundle says the repo is ROX Anchor, generated from `/Users/mymac/Desktop/rox-anchor`, git state `d68e51b (dirty)`, and included 121 tracked/unignored files before the Phase 6 patch. 

This is still **not** a public launch plan, not a mainnet plan, not a public bridge plan, not a staking/liquidity/exchange plan, and not production settlement authorization. `BUILD_PLAN2.md` explicitly forbids mainnet-beta deployment, public ROX minting/burning, production bridge settlement, production ROC release, exchange-facing behavior, staking, liquidity pools, public bridge UI, live value movement with real user funds, silent live RPC submission, silent wallet/key use, unbounded relayer retries, committed operator keys, fake finality, and fake success output. 

Allowed in this plan:

```text
localnet tests
devnet/testnet experiments
test-only ROX mint metadata
test-only token-account metadata
test-only Anchor deployment drills
read-only live RPC checks
transaction simulation
strictly capped testnet submission only after explicit gates
operator safety drills
key rotation drills
upgrade authority drills
halt/recovery drills
monitoring / alerting prototypes
security tests
chaos tests
audit prep
private testnet-readiness proof
```

## 1. Current status summary

Current state:

```text
Original BUILD_PLAN.md:
  COMPLETE / GREEN / PARKED

BUILD_PLAN2.md:
  Phase 0 — green
  Phase 1 — green
  Phase 2 — green
  Phase 3 — green
  Phase 4 — green
  Phase 5 — green
  Phase 6 — green

Next phase:
  Phase 7 — Testnet Deployment Drill
```

Latest terminal output confirms the Phase 6 patch passed. The new test-only asset harness tests passed in `rox-anchor-core`, the Anchor program tests increased to **96 passed / 0 failed**, the full workspace tests passed, focused checks passed, `anchor build` passed, and `anchor test` passed with the full Anchor test suite showing **96 passed / 0 failed**. 

Current rough completion estimate:

```text
Original local ROX Anchor implementation:
  100% complete / green / parked

BUILD_PLAN2 testnet-only hardening:
  roughly 40–45% complete by phase count
  roughly 35–40% complete by implementation weight

Entire ROX Anchor project, including original local build + BUILD_PLAN2:
  roughly 80–82% complete
```

Reason for the weighted estimate: Phases 0–6 are green, but the remaining phases include deployment drill safety, capped testnet submission, end-to-end testnet shadow flow, expanded security regressions, observability/audit trail, halt/recovery drills, testnet chaos drills, audit prep, and final readiness gate. Those are heavier and security-sensitive even though the foundation is now very strong.

## 2. What we accomplished in this session block

### 2.1 Phase 4 — Read-only RPC adapter was completed earlier

Phase 4 added a safe, read-only RPC adapter shape in `rox-anchor-rpc-proof`.

Accomplished:

```text
Added read-only RPC adapter trait.
Added current slot read model.
Added account status read model.
Added signature status read model.
Converted read-only RPC-shaped observations into existing RPC proof observations.
Kept all submission disabled.
Added fake adapter tests for stale, missing, mismatched, and disputed evidence.
Updated CLI proof command to report read-only RPC adapter shape.
Added CLI test proving proof command does not claim live submission, mint, burn, settlement, or key loading.
```

Important boundary:

```text
No real network client was added.
No live RPC submission exists.
No wallet/key loading exists.
No mint/burn execution exists.
The adapter is proof-shaped and testable, but still safe.
```

Phase 4 result:

```text
green
```

### 2.2 Phase 5 — Transaction simulation model was completed

Phase 5 added a simulation-only transaction planning model.

Main files touched:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/tests/transaction_simulation.rs
crates/rox-anchor-coordinator/src/decision.rs
crates/rox-anchor-coordinator/tests/transaction_simulation_gate.rs
```

Accomplished:

```text
Added TransactionSimulationPlan.
Added TransactionSimulationResult.
Added TransactionSimulationStatus.
Added simulation-only gate in relayer.
Added coordinator helper: permits_transaction_simulation().
Required accepted proof review before simulation.
Required accepted coordinator decision before simulation.
Required relayer dry-run acceptance before simulation.
Required safe non-submitting scope.
Required non-empty instruction plan.
Blocked duplicate idempotency dry-run receipts from simulating.
Blocked challenged/blocked/rejected proof reviews from simulating.
Guaranteed live_submission=false in simulation result.
```

Tests added:

```text
accepted_proof_coordinator_and_relayer_path_can_simulate_without_live_submission
simulation_rejects_missing_coordinator_acceptance_even_after_relayer_dry_run
simulation_rejects_blocked_proof_reviews
simulation_rejects_duplicate_relayer_dry_run_receipts
simulation_rejects_empty_instruction_plan
simulation_rejects_unsafe_or_submitting_scope
accepted_coordinator_decision_permits_simulation_gate
rejected_evidence_coordinator_decision_blocks_simulation_gate
```

Phase 5 result:

```text
green
```

Key invariant proven:

```text
A transaction can only become simulation-eligible after:
  proof accepted
  coordinator accepted
  relayer dry-run accepted
  safe non-submitting scope
  non-empty instruction plan

Even then:
  live_submission=false
```

### 2.3 Phase 6 — Test-only mint and token-account harness was completed

Phase 6 added deterministic test-only asset fixture validation.

Main files touched:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-core/tests/test_only_asset_harness.rs
programs/rox-anchor/src/errors.rs
programs/rox-anchor/src/state.rs
```

Accomplished in `rox-anchor-core`:

```text
Added TEST_ONLY_MAX_AMOUNT_UNITS = 10_000.
Added TestOnlyAssetHarnessFinding.
Added TestOnlyMintFixture.
Added TestOnlyTokenAccountFixture.
Added TestOnlyAssetHarness.
Added TestOnlyAssetHarnessReview.
Added label review helpers.
Added public/production/live/mainnet label rejection.
Added explicit testnet-only mode requirement.
Added zero amount rejection.
Added over-cap amount rejection.
Added token-account mint mismatch rejection.
```

Core tests added:

```text
test_only_asset_harness_accepts_explicit_testnet_simulation_fixture
test_only_asset_harness_requires_explicit_testnet_mode
test_only_asset_harness_rejects_public_or_production_mint_labels
test_only_asset_harness_rejects_token_account_mint_mismatch
test_only_asset_harness_rejects_zero_and_over_cap_amounts
```

Accomplished in Anchor program state:

```text
Added PROGRAM_TEST_ONLY_MAX_AMOUNT_UNITS = 10_000.
Added TestOnlyMintHarnessSnapshot.
Added validation for explicit testnet label.
Added validation for test-only mint label.
Added validation for test-only token-account label.
Added public/production/live/mainnet label rejection.
Added zero / over-cap amount rejection.
Added mint/token-account mint binding validation.
Added program-side unit tests.
```

Anchor errors added:

```text
TestOnlyModeRequired
TestOnlyLabelRequired
PublicMintLabelRejected
PublicTokenAccountLabelRejected
TestAmountCapExceeded
TestTokenAccountMintMismatch
```

Program tests added:

```text
program_test_only_harness_accepts_explicit_testnet_fixture
program_test_only_harness_rejects_local_or_missing_testnet_mode
program_test_only_harness_rejects_public_or_production_labels
program_test_only_harness_rejects_zero_and_over_cap_amounts
program_test_only_harness_rejects_token_account_mint_mismatch
```

Phase 6 result:

```text
green
```

Latest terminal proof:

```text
rox-anchor-core test-only asset harness:
  5 passed / 0 failed

rox-anchor program:
  96 passed / 0 failed

workspace tests:
  green

cargo check / focused clippy:
  green

anchor build:
  green

anchor test:
  green
```

The terminal output shows the new `test_only_mint_harness_tests` running inside the Anchor program and passing, and the final Anchor test suite also passing with 96 tests. 

## 3. Current architecture state

### 3.1 `rox-anchor-core`

Current role:

```text
Shared source of truth for IDs, domains, directions, states, labels, challenge posture, halt posture, recovery posture, safety flags, testnet config, authority model, and test-only asset harness.
```

Important completed surfaces:

```text
typed IDs
status labels
lifecycle classification
challenge/halt/recovery posture helpers
testnet scope locks
testnet config model
authority roles and authority separation
authority rotation intent
test-only mint/token-account harness
```

Important tests currently proven:

```text
ID validation
safe labels
state/posture helpers
testnet config validation
mainnet-beta rejection
public-launch flag absence
authority role stability
critical authority separation
wrong authority redaction
rotation intent validation
test-only asset harness acceptance/rejection
```

### 3.2 `rox-anchor-proof`

Current role:

```text
Deterministic local proof review.
```

Important completed behavior:

```text
valid package acceptance
binding mismatch rejection
replay rejection
missing/disputed evidence blocking
challenge/halt/recovery blocking
deterministic findings
```

Important tests currently proven:

```text
valid_fixture_is_accepted_and_finality_eligible
binding_mismatches_are_rejected_in_deterministic_order
missing_or_disputed_evidence_blocks_acceptance
challenge_halt_and_recovery_postures_block_acceptance
replayed_operation_id_idempotency_key_or_nonce_is_rejected
challenge griefing tests
halt/resume tests
```

### 3.3 `rox-anchor-rpc-proof`

Current role:

```text
Local RPC evidence model plus read-only adapter shape.
```

Important completed behavior:

```text
RPC observation model
expected RPC binding
commitment classification
quorum agreement/disagreement
stale evidence rejection
missing evidence classification
same-source equivocation detection
read-only adapter trait
fake adapter tests
conversion from read-only signature status to proof observation
```

Important boundary:

```text
Read-only only.
No submission.
No wallet/keypair.
No real network client yet.
```

### 3.4 `rox-anchor-coordinator`

Current role:

```text
Local coordinator model that assembles observations and produces proof-backed decisions.
```

Important completed behavior:

```text
queue capacity enforcement
duplicate operation rejection
stale RPC evidence rejection
handoff to rpc-proof
handoff to rox-anchor-proof
redacted decision shape
authority-aware readiness
simulation gate
```

Important invariant:

```text
Coordinator does not invent acceptance rules.
It routes through rpc-proof and proof review.
```

### 3.5 `rox-anchor-relayer`

Current role:

```text
Local relayer dry-run, retry, receipt, and simulation-only model.
```

Important completed behavior:

```text
bounded retry policy
dry-run receipt generation
idempotency replay rejection
blocked/rejected proof uses zero attempts
receipt capacity limits
authority-aware readiness
simulation-only transaction planning
simulation result with live_submission=false
```

Important invariant:

```text
Relayer still cannot submit live transactions.
```

### 3.6 `rox-anchor-cli`

Current role:

```text
Terminal inspection surface.
```

Important completed behavior:

```text
check command uses real proof fixtures
status command includes safe labels
status command includes testnet config shape
status command includes redacted authority model shape
proof command reports read-only RPC adapter shape
tests reject fake runtime/settlement/submission claims
```

Important invariant:

```text
CLI output must not fake success, finality, settlement, mint, burn, or runtime authority.
```

### 3.7 `programs/rox-anchor`

Current role:

```text
Anchor program state machine and on-chain transition surface.
```

Important completed behavior:

```text
config initialization shape
operation PDA helpers
mint authority PDA helpers
direction helpers
challenge/finalization controls
halt/recovery controls
mint/burn binding helpers
token account constraint snapshots
token settlement planned events
token settlement execution plans
token CPI readiness checks
token CPI execution receipt checks
test-only mint harness validation
```

Current Anchor test count:

```text
96 passed / 0 failed
```

## 4. Safety boundaries that must remain locked

Do not add any of the following unless explicitly authorized by a later build plan:

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
production ROC release
exchange-facing behavior
staking
liquidity pools
public faucet
public claim page
public bridge UI
live value movement with real user funds
silent live RPC submission
silent wallet/key usage
unbounded relayer retries
operator key material committed to repo
fake finality
fake success output
```

Current work may continue to add:

```text
testnet-only config validation
read-only RPC proof
simulation-only transaction planning
test-only mint/account metadata
secret/key-file scans
deployment drill scripts
capped submit models with all gates
shadow-flow tests
security regressions
observability/reporting
halt/recovery drills
testnet chaos tests
audit maps
```

But live submission remains disallowed until the plan reaches Phase 8 and every preceding gate is green.

## 5. Immediate next phase: Phase 7 — Testnet Deployment Drill

Next session should begin with **BUILD_PLAN2 Phase 7**.

Purpose:

```text
Practice a testnet deployment procedure without treating it as production.
```

This should be a safety/script/check phase, not a live deployment.

Phase 7 should not run `anchor deploy`.

Phase 7 should add a local deploy-drill readiness script that checks:

```text
Anchor.toml exists.
Anchor.toml provider cluster is not mainnet-beta.
Anchor.toml wallet path is not committed key material.
Program ID handling is visible.
Deployment key files are ignored by git patterns.
Forbidden key file names are absent from the repo.
Common Solana/Anchor keypair files are absent from tracked surfaces.
Mainnet-beta is rejected in config/script checks.
Target deploy keypairs are ignored.
No public launch/public bridge/mainnet language is introduced.
No production mint/burn behavior is introduced.
```

Likely files to touch:

```text
scripts/dev-testnet-deploy-drill-check.sh
.gitignore maybe, only if a missing forbidden pattern is found
Anchor.toml maybe, only if provider sanity needs adjustment
```

Recommended tests/checks:

```text
bash scripts/dev-testnet-deploy-drill-check.sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
anchor build
anchor test
```

Suggested Phase 7 patch shape:

```text
1. Add scripts/dev-testnet-deploy-drill-check.sh.
2. Script should be read-only and fail closed.
3. Script should scan for forbidden key material file names.
4. Script should scan Anchor.toml for mainnet-beta.
5. Script should confirm expected ignored key paths/patterns exist in .gitignore.
6. Script should print "testnet_deploy_drill_ready: true" only after all checks pass.
7. Add no deployment command execution.
8. Add no key loading.
9. Add no RPC call.
10. Add no live submission.
```

Potential script checks:

```text
forbid files:
  id.json
  keypair.json
  wallet.json
  payer.json
  authority.json
  mint-authority.json
  upgrade-authority.json
  program-authority.json
  deploy-authority.json
  admin.json
  owner.json
  validator-keypair.json
  faucet-keypair.json
  target/deploy/*.json
  programs/**/target/deploy/*.json

require .gitignore patterns:
  .env
  secrets/
  private/
  keypairs/
  wallets/
  mnemonics/
  target/deploy/*.json
  target/deploy/*-keypair.json
  *-keypair.json
  *.wallet.json
  *.authority.json
  .solana/
  CODEBUNDLE*.md

forbid text in Anchor.toml / scripts:
  mainnet-beta
  mainnet
  public launch
  production bridge
  exchange
  liquidity
  staking
```

Important: `.gitignore` already has a strong secret/key material section in the codebundle, including many Solana/Anchor keypair patterns and generated deploy keypair exclusions. Use that before adding duplicate patterns.

Phase 7 exit condition:

```text
A testnet deployment drill can be performed with external keys and no committed secrets.
No live deploy is performed by the script.
No live RPC call is performed by the script.
No wallet/keypair is loaded by the script.
No mainnet-beta surface is allowed.
```

## 6. Thereafter: BUILD_PLAN2 remaining phases

### Phase 8 — Capped Testnet Submission Path

Purpose:

```text
Add a strictly capped testnet-only transaction send path after read-only RPC and simulation are green.
```

Do not start Phase 8 until Phase 7 deploy-drill checks are green.

Expected work:

```text
Add TestnetSubmitCapped path in relayer.
Require explicit config or CLI flag.
Require accepted proof review.
Require accepted coordinator decision.
Require relayer dry-run acceptance.
Require successful simulation.
Require retry cap.
Require amount cap.
Require operation cap per run.
Require receipt persistence.
Reject live submission in LocalOnly and SimulateOnly modes.
Add tests proving each missing guard blocks submission.
```

Likely files:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/retry.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/src/commands/check.rs
```

Important approach:

```text
First add the model and tests.
Do not add a real Solana transaction sender immediately.
If a submit function is added, it should require an injected trait/fake sender in tests.
Default path must remain non-submitting.
```

Phase 8 exit:

```text
The only live-submission-shaped path is testnet-only, capped, explicit, receipt-backed, and impossible to trigger from default mode.
```

### Phase 9 — End-to-End Testnet Shadow Flow

Purpose:

```text
Prove the ROC ↔ ROX shape against testnet-only fixtures without real ROC release or public ROX.
```

Expected work:

```text
Model ROC-to-ROX testnet shadow flow.
Model ROX-to-ROC testnet shadow flow.
Use fake/internal test input.
Do not release real ROC.
Do not expose public mint/burn.
Use explicit test operation IDs.
Use explicit test idempotency keys.
Persist receipts.
Prove replay rejection.
Prove mismatches are rejected.
Prove halt/challenge/recovery blockers work in testnet mode.
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/
crates/rox-anchor-relayer/tests/
crates/rox-anchor-rpc-proof/tests/
programs/rox-anchor/src/state.rs
```

Phase 9 exit:

```text
The testnet shadow flow proves the full shape without production value movement.
```

### Phase 10 — Security Regression Expansion

Purpose:

```text
Expand adversarial tests beyond the first local chaos suite.
```

Required test groups:

```text
replay storms
idempotency collisions
nonce reuse
operation ID reuse
RPC equivocation
RPC stale evidence
RPC missing evidence
RPC provider disagreement
cluster mismatch
program ID mismatch
mint mismatch
token account mismatch
wrong direction
wrong authority
halt bypass attempt
recovery bypass attempt
challenge bypass attempt
finalize-before-eligible attempt
duplicate finalization attempt
receipt tampering
simulation result tampering
testnet submit mode bypass attempt
mainnet-beta rejection
secret/key path leak check
unbounded retry rejection
rate-limit enforcement
```

Commands:

```text
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Phase 10 exit:

```text
Known adversarial cases are covered by deterministic tests before broader testnet use.
```

### Phase 11 — Observability, Receipts, and Audit Trail

Purpose:

```text
Make every testnet action inspectable and explainable.
```

Expected work:

```text
Add structured testnet receipt shape.
Redact secrets and signatures as needed.
Include proof decision.
Include coordinator decision.
Include RPC quorum result.
Include relayer retry result.
Include simulation result.
Include submission result only in capped testnet mode.
Include halt/challenge/recovery posture.
Add CLI display tests.
Add deterministic report tests.
```

Likely files:

```text
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/redaction.rs
crates/rox-anchor-coordinator/src/redaction.rs
crates/rox-anchor-rpc-proof/src/redaction.rs
crates/rox-anchor-cli/src/commands/status.rs
```

Phase 11 exit:

```text
Every testnet-relevant action has a safe, deterministic, redacted receipt/report.
```

### Phase 12 — Halt, Recovery, and Kill-Switch Drills

Purpose:

```text
Prove unsafe behavior can be stopped before any public exposure.
```

Expected work:

```text
Test halt before proof acceptance.
Test halt after proof acceptance but before simulation.
Test halt after simulation but before submission.
Test halt after capped testnet submission.
Test recovery path from halted state.
Test wrong authority cannot halt/recover.
Test status output reflects halt/recovery.
Test relayer refuses submission while halted.
Test coordinator refuses finalization while halted.
```

Phase 12 exit:

```text
Halt and recovery behavior is proven across local, simulated, and capped testnet modes.
```

### Phase 13 — Testnet Chaos Drills

Purpose:

```text
Run realistic failure drills before broader testnet rollout.
```

Drills:

```text
RPC outage
RPC disagreement
RPC stale slots
RPC equivocation
program account missing
wrong program ID
wrong mint
wrong token account
reorg-like stale evidence
duplicate relayer request
retry storm
receipt capacity pressure
operator key unavailable
wrong authority key
halt during pending operation
recovery during pending operation
simulation passes but send disabled
send enabled but cap exceeded
```

Phase 13 exit:

```text
Testnet chaos drills fail safely, deterministically, and with inspectable receipts.
```

### Phase 14 — Audit Preparation

Purpose:

```text
Prepare the repo for external or self-directed security review.
```

Expected docs may be added here, but only after the real implementation/tests exist:

```text
docs/audit/INVARIANT_TEST_MAP.md
docs/audit/AUTHORITY_MODEL.md
docs/audit/STATE_TRANSITIONS.md
docs/audit/RPC_BOUNDARY.md
docs/audit/RELAYER_BOUNDARY.md
docs/audit/HALT_RECOVERY_RUNBOOK.md
docs/audit/TESTNET_DEPLOYMENT_RUNBOOK.md
```

Important: This is the proper time for audit docs, not earlier. Keep them tied to real tests.

Phase 14 exit:

```text
Implementation has audit-ready maps and runbooks tied to real tests.
```

### Phase 15 — Testnet Readiness Gate

Purpose:

```text
Decide whether ROX Anchor is ready for a private testnet pilot.
```

This is not a public launch gate.

Readiness checklist:

```text
all local tests pass
all Anchor tests pass
all testnet-only guards pass
mainnet-beta is rejected
no public mint/burn path exists
no public bridge UI exists
no exchange/staking/liquidity behavior exists
operator runbooks exist
receipt/reporting works
halt/recovery drills work
testnet capped submission works only when explicitly enabled
known adversarial cases are tested or documented
```

Final successful status for BUILD_PLAN2:

```text
ROX Anchor testnet-only hardening complete / green / parked.
Private testnet pilot readiness proven.
Public launch remains unauthorized.
Production bridge remains unauthorized.
Mainnet remains unauthorized.
Public mint/burn remains unauthorized.
Exchange/liquidity/staking remains unauthorized.
```

## 7. Recommended next-session opening prompt

Use this at the start of the next session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
Original BUILD_PLAN.md is COMPLETE / GREEN / PARKED.
Active workstream is BUILD_PLAN2 — testnet-only production readiness / security hardening.
BUILD_PLAN2 Phases 0–6 are green. Latest completed phase is Phase 6: test-only mint and token-account harness. The latest terminal output showed rox-anchor-core test-only asset harness tests passed, rox-anchor program tests increased to 96 passed, full workspace tests passed, workspace Clippy passed, anchor build passed, and anchor test passed.

Next phase:
BUILD_PLAN2 Phase 7 — Testnet Deployment Drill.

Important boundaries:
No mainnet-beta deployment, public ROX mint/burn, production bridge settlement, production ROC release, exchange-facing behavior, staking, liquidity, public bridge UI, live value movement, silent RPC submission, silent wallet/key use, committed operator keys, fake finality, or fake success output.

Please begin with a small real patch for Phase 7: a local read-only deployment-drill readiness script/check surface that verifies key material is not committed, mainnet-beta is rejected, Anchor.toml/provider assumptions are safe, and the drill does not deploy, submit, load keys, or call live RPC. Use paste-ready terminal patches in ~~~~bash fences and include exact commands to run next.
```

## 8. Exact commands to run at the beginning of next session

Before Phase 7 patch, run or ask for current output from:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
cargo test --workspace
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
anchor build
anchor test
```

If the user wants to avoid a full duplicate run because the last run was just green, start Phase 7 with focused commands after patch:

```bash
bash scripts/dev-testnet-deploy-drill-check.sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
anchor build
anchor test
```

## 9. Patch discipline for the next session

Use the same rules that got us green:

```text
Start with the first focused behavior.
Patch exact files.
Run focused test/check.
Fix first failure before expanding.
Use Clippy after each major quarter / natural hardening boundary.
Do not add broad docs unless the phase asks for audit/runbook docs later.
Do not add live deployment/submission behavior.
Do not include git add/commit/push unless explicitly requested.
Use ~~~~bash fences for paste-ready patches.
```

For Phase 7 specifically:

```text
Prefer one script + focused command first.
Do not alter core Rust unless needed.
Do not add deployment commands that execute.
Do not add external dependencies.
Do not download anything.
Do not require live RPC.
Do not require real key files.
Do not load keypairs.
Do not write secrets.
```

## 10. Current milestone statement

ROX Anchor is now in a strong position.

The project has moved from:

```text
local compile-tested implementation
```

to:

```text
testnet-hardening with:
  safety flags
  config redaction
  authority separation
  read-only RPC adapter shape
  transaction simulation gate
  test-only mint/token-account harness
```

The next major transition is from code-level testnet hardening into operational readiness checks:

```text
Phase 7:
  deployment drill safety

Phase 8:
  capped testnet submission model

Phase 9:
  end-to-end private testnet shadow flow

Phase 10–13:
  security regressions, observability, halt/recovery, chaos

Phase 14–15:
  audit prep and readiness gate
```

The most important principle going forward:

```text
We are allowed to get closer to Solana/devnet reality,
but only one safety layer at a time.
```

These notes are ready to paste into the next session.


### END NOTE - JULY 4 2026 - 13:50 CST


### BEGIN NOTE - JULY 4 2026 - 18:25 CST

Absolutely — this was a very strong session. Phase 11 is now **complete / green / parked**, and the repo is in a clean position to open the next session directly on **BUILD_PLAN2 Phase 12 — Halt, Recovery, and Kill-Switch Drills**. The latest codebundle was generated after the final posture-audit work and shows the new audit files/tests are now part of the repo surface.  BUILD_PLAN2 confirms Phase 12 is the next phase and defines the halt/recovery drill requirements. 

# ROX Anchor — Carry-Over Notes for Next Session

## 0. Session status

We are continuing work on **RustyOnions / CrabLink / ROX Anchor**.

Current active workstream:

```text
ROX Anchor
BUILD_PLAN2 — Testnet-Only Production Readiness / Security Hardening
```

Current status:

```text
BUILD_PLAN.md: complete / green / parked
BUILD_PLAN2 Phase 0–11: complete / green / parked
Current next phase: BUILD_PLAN2 Phase 12 — Halt, Recovery, and Kill-Switch Drills
```

The latest local terminal output proves the final Phase 11 work is green:

```text
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --test posture_audit_report
cargo test -p rox-anchor-cli --test relayer_audit_report
cargo test -p rox-anchor-cli --test audit_report
cargo test -p rox-anchor-cli
cargo run -p rox-anchor-cli -- --help
cargo run -p rox-anchor-cli -- audit-posture
cargo check --workspace
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
anchor build
anchor test
```

All of those passed in the most recent output. The final `audit-posture` command rendered the expected posture audit report with one accepted clear case and five blocked cases: challenge open, challenge accepted, halted, recovery required, and recovery in review. The full workspace test suite, focused/workspace Clippy, Anchor build, and Anchor test all passed. 

The latest codebundle was generated at:

```text
2026-07-04T23:12:36Z
Root: /Users/mymac/Desktop/rox-anchor
Git: d68e51b (dirty)
Files: 139
```

The codebundle includes the Phase 11 audit files and tests:

```text
crates/rox-anchor-cli/src/commands/audit.rs
crates/rox-anchor-cli/src/commands/posture_audit.rs
crates/rox-anchor-cli/tests/audit_report.rs
crates/rox-anchor-cli/tests/relayer_audit_report.rs
crates/rox-anchor-cli/tests/posture_audit_report.rs
crates/rox-anchor-rpc-proof/src/audit.rs
crates/rox-anchor-rpc-proof/tests/rpc_audit_record.rs
crates/rox-anchor-coordinator/src/audit.rs
crates/rox-anchor-coordinator/tests/coordinator_audit_record.rs
crates/rox-anchor-relayer/src/audit.rs
crates/rox-anchor-relayer/tests/testnet_audit_record.rs
```

The codebundle also preserves the full current project surface: core, proof, CLI, coordinator, relayer, RPC proof, Anchor program, integration tests, chaos tests, and testnet-hardening tests. 

---

## 1. Active project boundaries

ROX Anchor is the Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.

This is active implementation work, not docs-only planning.

The current plan is **testnet-only hardening**, not public launch. BUILD_PLAN2 explicitly says this is not a public launch plan, not a mainnet plan, not an exchange/liquidity/staking plan, and not authorization for public ROX mint/burn. 

Forbidden unless explicitly moved into a future authorized plan:

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
production ROC release
exchange-facing behavior
staking
liquidity pools
public faucet
public claim page
public bridge UI
live value movement with real user funds
silent live RPC submission
silent wallet/key usage
unbounded relayer retries
operator key material committed to repo
fake finality
fake success output
```

Allowed in BUILD_PLAN2:

```text
localnet tests
Solana devnet/testnet experiments
test-only ROX mint
test-only token accounts
test-only Anchor deployment drills
read-only live RPC checks
transaction simulation
strictly capped testnet transaction submission after explicit test gates
operator safety drills
key rotation drills
upgrade authority drills
halt/recovery drills
monitoring and alerting prototypes
security tests
chaos tests
audit prep
documentation of invariants proven by tests
```

BUILD_PLAN2 also states that if future work needs anything beyond that boundary, it must become a separate build plan. 

---

## 2. Build discipline to preserve

Continue the same QuickChain-style workflow:

```text
small behavior patch
focused test
cargo fmt
focused cargo test/check
fix first failure
then expand
```

Do not add broad planning docs instead of code.

Do not add placeholder-only scaffolding.

Do not add live submission until the earlier test/simulation phases are green.

Do not let CLI, coordinator, relayer, RPC proof, or testnet adapters invent competing state rules.

`rox-anchor-core` and `rox-anchor-proof` remain the shared source of truth.

The Anchor program remains the owner of on-chain state transitions. 

Patch style for the next session:

```text
- provide paste-ready terminal patches
- wrap terminal patches in ~~~~bash fences
- prefer complete file writes or narrow scripted edits
- no git add/commit/push unless explicitly requested
- no downloads unless explicitly requested
- run focused checks first
- run workspace checks after focused green
- run focused Clippy before workspace Clippy at major hardening boundaries
```

---

## 3. What we accomplished in this session

### 3.1 Completed BUILD_PLAN2 Phase 11

Phase 11 purpose:

```text
Observability, Receipts, and Audit Trail
```

BUILD_PLAN2 required Phase 11 to make every testnet-relevant action inspectable and explainable, including structured receipt/report shape, redaction, proof decision, coordinator decision, RPC quorum result, relayer retry result, simulation result, capped submission result, halt/challenge/recovery posture, CLI display tests, and deterministic report tests. 

That phase is now complete.

Final status:

```text
BUILD_PLAN2 Phase 11: complete / green / parked
```

---

## 4. Phase 11 implementation details

### 4.1 RPC proof audit

We added and/or stabilized the RPC proof audit surface.

Relevant files:

```text
crates/rox-anchor-rpc-proof/src/audit.rs
crates/rox-anchor-rpc-proof/tests/rpc_audit_record.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/tests/proof_read_only_rpc.rs
```

Current CLI command:

```text
rox-anchor proof
```

Purpose:

```text
Show deterministic read-only RPC proof audit behavior from real local RPC-proof review shape.
```

It reports:

```text
status: read_only_rpc_adapter_shape
submission: disabled
wallet_key_loading: disabled
network_client: not_enabled
current_slot
observations_checked
quorum_decision
audit_record=rpc-proof-audit-v1
expected cluster/program/mint/token account/operation ID
minimum commitment
observation count
accepted observations
required observations
decision
findings
redacted observations
evidence_consistent
display_safe
```

Security posture:

```text
no live RPC submission
no wallet/key loading
no mint/burn
no ROC release
no settlement
```

What this proves:

```text
- RPC audit output is deterministic.
- RPC observations can be rendered safely.
- Signatures are redacted.
- Agreement/disagreement/mismatch behavior is visible.
- CLI output comes from real RPC proof behavior, not fake success strings.
```

---

### 4.2 Coordinator audit

We added and stabilized the coordinator audit surface.

Relevant files:

```text
crates/rox-anchor-coordinator/src/audit.rs
crates/rox-anchor-coordinator/tests/coordinator_audit_record.rs
crates/rox-anchor-cli/src/commands/audit.rs
crates/rox-anchor-cli/tests/audit_report.rs
```

Current CLI command:

```text
rox-anchor audit
```

Purpose:

```text
Show deterministic coordinator audit report for accepted local proof/RPC/coordinator path.
```

It reports:

```text
rox-anchor audit
status: coordinator_audit_report
submission: disabled
wallet_key_loading: disabled
network_client: not_enabled
runtime_authority: not_enabled
coordinator_status
rpc_decision
proof_decision
permits_simulation
audit_record=coordinator-testnet-audit-v1
operation ID
idempotency key
nonce
direction
source/target domain
cluster
program ID
mint
token account
current slot
observation count
RPC decision/findings
proof decision/findings
coordinator status
permits_simulation
status_consistent
display_safe
```

Security posture:

```text
report-only
no RPC submission
no wallet/key loading
no mint/burn
no ROC release
no settlement
```

Important bug fixed during this phase:

```text
The CLI dispatch already supported the audit command, but --help initially did not list it.
We patched help_text() so audit appears in the command list.
```

Result:

```text
cargo test -p rox-anchor-cli --test audit_report: 3/3 passed
```

---

### 4.3 Relayer / simulation / capped submission audit

We added and stabilized the relayer audit CLI surface.

Relevant files:

```text
crates/rox-anchor-relayer/src/audit.rs
crates/rox-anchor-relayer/tests/testnet_audit_record.rs
crates/rox-anchor-cli/src/commands/audit.rs
crates/rox-anchor-cli/tests/relayer_audit_report.rs
```

Current CLI command:

```text
rox-anchor audit-relayer
```

Alias:

```text
rox-anchor relayer-audit
```

Purpose:

```text
Show deterministic relayer + simulation + capped testnet submission audit output without sending anything.
```

It reports:

```text
rox-anchor audit-relayer
status: relayer_simulation_capped_audit_report
submission: capped_testnet_report_only
wallet_key_loading: disabled
network_client: not_enabled
runtime_authority: not_enabled
relayer_status
proof_decision
simulation_status
capped_submission_status
receipt_persisted
authorized
live_submission_attempted
network_submitted
audit_record=relayer-testnet-audit-v1
target
attempts_used
instruction_count
requested_attempts
requested_operations
amount_units
live_submission_permitted
pipeline_consistent
display_safe
```

Security posture:

```text
report-only
no RPC submission
no wallet/key loading
no mint/burn execution
no ROC release
no settlement
network_submitted=false
live_submission_attempted=false
```

What this proves:

```text
- Relayer audit output is deterministic.
- Simulation status is visible.
- Capped submission authorization is visible.
- Receipt persistence is visible.
- Network submission remains false.
- The command cannot be confused with a real submit path.
```

Result:

```text
cargo test -p rox-anchor-cli --test relayer_audit_report: 4/4 passed
```

---

### 4.4 Halt / challenge / recovery posture audit

We added and stabilized the proof posture audit CLI surface.

Relevant files:

```text
crates/rox-anchor-cli/src/commands/posture_audit.rs
crates/rox-anchor-cli/tests/posture_audit_report.rs
crates/rox-anchor-proof/src/challenge.rs
crates/rox-anchor-proof/src/recovery.rs
crates/rox-anchor-proof/src/validate.rs
crates/rox-anchor-proof/src/fixtures.rs
crates/rox-anchor-core/src/state.rs
```

Current CLI command:

```text
rox-anchor audit-posture
```

Alias:

```text
rox-anchor posture-audit
```

Purpose:

```text
Show deterministic halt/challenge/recovery posture audit output from real proof review behavior.
```

It reports:

```text
rox-anchor audit-posture
status: proof_posture_audit_report
submission: disabled
wallet_key_loading: disabled
network_client: not_enabled
runtime_authority: not_enabled
audit_record=proof-posture-audit-v1
case_count=6
accepted_cases=1
blocked_cases=5
display_safe=true
```

Current cases:

```text
case.0 clear
  challenge_posture=Clear
  halt_posture=Active
  recovery_posture=NotRequired
  decision=Accepted
  lifecycle_state=FinalityEligible
  findings=PackageAccepted
  permits_acceptance=true

case.1 challenge_open
  challenge_posture=Open
  decision=Blocked
  lifecycle_state=ChallengeOpen
  findings=ChallengeOpen
  permits_acceptance=false

case.2 challenge_accepted
  challenge_posture=Accepted
  decision=Blocked
  lifecycle_state=ChallengeAccepted
  findings=ChallengeAccepted
  permits_acceptance=false

case.3 halted
  halt_posture=Halted
  decision=Blocked
  lifecycle_state=Halted
  findings=Halted
  permits_acceptance=false

case.4 recovery_required
  recovery_posture=Required
  decision=Blocked
  lifecycle_state=RecoveryRequired
  findings=RecoveryRequired
  permits_acceptance=false

case.5 recovery_in_review
  recovery_posture=InReview
  decision=Blocked
  lifecycle_state=RecoveryInReview
  findings=RecoveryInReview
  permits_acceptance=false
```

Important fix in Round 7:

```text
The initial posture audit code called ReviewDecision::is_accepted(), but ReviewDecision did not expose that helper.
We patched posture_audit.rs with a local helper:

fn is_accepted_decision(decision: &ReviewDecision) -> bool {
    matches!(decision, ReviewDecision::Accepted)
}

This fixed the compile failure cleanly without changing proof-core semantics.
```

Result:

```text
cargo test -p rox-anchor-cli --test posture_audit_report: 4/4 passed
```

The final terminal output confirmed the CLI command rendered correctly and all workspace/Anchor checks passed. 

---

## 5. Final Phase 11 CLI surface

At the end of this session, `rox-anchor --help` lists:

```text
check [--fixture <name>]   review a deterministic local proof fixture
proof                      show proof-review command notes
audit                      show coordinator audit report
audit-relayer              show relayer simulation and capped submission audit report
audit-posture              show halt/challenge/recovery posture audit report
status                     show display-safe status labels
submit-capped              report capped testnet submit authorization
halt                       show halt posture notes
recover                    show recovery posture notes
```

This gives us the full Phase 11 audit chain:

```text
rox-anchor proof          -> RPC proof audit
rox-anchor audit          -> coordinator audit
rox-anchor audit-relayer  -> relayer/simulation/capped-submit audit
rox-anchor audit-posture  -> halt/challenge/recovery posture audit
```

That is exactly what Phase 11 needed: every testnet-relevant action now has a deterministic, display-safe, redacted, inspectable report.

---

## 6. Final Phase 11 test results

Latest confirmed green results:

```text
cargo test -p rox-anchor-cli --test posture_audit_report
  4 passed

cargo test -p rox-anchor-cli --test relayer_audit_report
  4 passed

cargo test -p rox-anchor-cli --test audit_report
  3 passed

cargo test -p rox-anchor-cli
  all CLI unit/integration tests passed

cargo run -p rox-anchor-cli -- --help
  command list includes audit, audit-relayer, audit-posture

cargo run -p rox-anchor-cli -- audit-posture
  rendered proof-posture-audit-v1 safely

cargo check --workspace
  passed

cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
  passed

cargo clippy --workspace --all-targets -- -D warnings
  passed

cargo test --workspace
  passed

anchor build
  passed

anchor test
  passed
```

The terminal output also confirms the Anchor program’s large unit suite passed, including 96 program tests covering config initialization, challenge resolution, binding mismatch, finalize plans, mint authority PDA derivation, token settlement binding, token CPI readiness, execution receipts, wrong authority, direction helpers, and test-only mint harness behavior. 

---

## 7. Current codebase shape after this session

The latest codebundle includes these workspace members:

```text
crates/rox-anchor-core
crates/rox-anchor-proof
crates/rox-anchor-cli
crates/rox-anchor-coordinator
crates/rox-anchor-relayer
crates/rox-anchor-rpc-proof
programs/rox-anchor
```

Root files include:

```text
Anchor.toml
BUILD_PLAN.md
BUILD_PLAN2.md
Cargo.lock
Cargo.toml
IDB.md
README.md
TODO.md
NOTES.MD
scripts/check_testnet_deploy_drill.sh
scripts/make_codebundle.sh
```

The root `Cargo.toml` workspace includes all active crates and the Anchor program. The codebundle also preserves the current no-secret posture and notes that the artifact itself is not runtime authorization. 

The codebase has mature surfaces for:

```text
core shared types
proof review
CLI inspection
read-only RPC proof model
coordinator local decision model
relayer dry-run / simulation / capped submit model
testnet config model
operator authority model
test-only asset harness
testnet deployment drill checks
testnet shadow flow
security regression tests
observability and audit reports
Anchor program state and token settlement model
```

---

## 8. Current BUILD_PLAN2 progress

BUILD_PLAN2 phases:

```text
Phase 0  — Freeze Local Green Baseline
Phase 1  — Testnet Scope Locks and Safety Flags
Phase 2  — Testnet Configuration Model
Phase 3  — Operator Key and Authority Safety Model
Phase 4  — Read-Only Live RPC Adapter
Phase 5  — Transaction Simulation Model
Phase 6  — Test-Only Mint and Token Account Harness
Phase 7  — Testnet Deployment Drill
Phase 8  — Capped Testnet Submission Path
Phase 9  — End-to-End Testnet Shadow Flow
Phase 10 — Security Regression Expansion
Phase 11 — Observability, Receipts, and Audit Trail
Phase 12 — Halt, Recovery, and Kill-Switch Drills
Phase 13 — Testnet Chaos Drills
Phase 14 — Audit Preparation
Phase 15 — Testnet Readiness Gate
```

Current status:

```text
Phase 0: complete / green / parked
Phase 1: complete / green / parked
Phase 2: complete / green / parked
Phase 3: complete / green / parked
Phase 4: complete / green / parked
Phase 5: complete / green / parked
Phase 6: complete / green / parked
Phase 7: complete / green / parked
Phase 8: complete / green / parked
Phase 9: complete / green / parked
Phase 10: complete / green / parked
Phase 11: complete / green / parked
Phase 12: next
Phase 13: thereafter
Phase 14: thereafter
Phase 15: final readiness gate
```

BUILD_PLAN2 defines Phase 12 as the next phase and says its purpose is to prove halt/recovery behavior across local, simulated, and capped testnet modes. 

---

## 9. What to do first in the next session

Start the next session with this status paragraph:

```text
We are continuing ROX Anchor. BUILD_PLAN.md is complete / green / parked. BUILD_PLAN2 Phase 11 — Observability, Receipts, and Audit Trail — is complete / green / parked. The latest green run passed cargo fmt/check/test, focused and workspace Clippy, cargo test --workspace, anchor build, and anchor test. Start BUILD_PLAN2 Phase 12 — Halt, Recovery, and Kill-Switch Drills. Keep all work testnet/local only: no live wallet calls, no silent live RPC submission, no production mint/burn, no production bridge settlement, no public launch behavior.
```

Then run the baseline:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo check --workspace
```

If green, run the fuller baseline only if needed:

```bash
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
anchor build
anchor test
```

Because the last session already ended green, the next patch should not waste time rebuilding everything before writing code unless the repo changed. A good flow is:

```text
1. cargo check --workspace
2. inspect first failure if any
3. if green, begin Phase 12 Round 1
4. write one focused halt/recovery drill test
5. patch the exact crate surface needed
6. run focused test
7. then workspace check/test/clippy at boundary
```

---

## 10. BUILD_PLAN2 Phase 12 scope

Phase 12 title:

```text
Halt, Recovery, and Kill-Switch Drills
```

Purpose:

```text
Prove we can stop unsafe behavior before any public exposure.
```

Phase 12 required work:

```text
1. Test halt before proof acceptance.
2. Test halt after proof acceptance but before simulation.
3. Test halt after simulation but before submission.
4. Test halt after capped testnet submission.
5. Test recovery path from halted state.
6. Test wrong authority cannot halt/recover.
7. Test status output reflects halt/recovery.
8. Test relayer refuses submission while halted.
9. Test coordinator refuses finalization while halted.
```

Exit condition:

```text
Halt and recovery behavior is proven across local, simulated, and capped testnet modes.
```

Commands from BUILD_PLAN2:

```bash
cargo fmt --all
cargo test --workspace
anchor build
anchor test
```

Source: BUILD_PLAN2 Phase 12. 

---

## 11. Recommended next-session implementation order

### Phase 12 Round 1 — Proof-level halt drill expansion

Goal:

```text
Prove halt blocks proof acceptance before downstream coordinator/relayer/simulation can do anything.
```

Likely files:

```text
crates/rox-anchor-proof/tests/halt_resume.rs
crates/rox-anchor-proof/src/fixtures.rs
crates/rox-anchor-proof/src/validate.rs
```

Likely tests:

```text
halt_before_proof_acceptance_blocks_review
recovery_required_before_proof_acceptance_blocks_review
recovery_in_review_before_proof_acceptance_blocks_review
clear_posture_restores_acceptance
```

Expected behavior:

```text
- Halted proof packages return Blocked.
- Lifecycle state is Halted.
- Finding includes Halted.
- RecoveryRequired returns Blocked.
- RecoveryInReview returns Blocked.
- Clear/Active/NotRequired returns Accepted for a valid package.
```

Focused commands:

```bash
cargo fmt -p rox-anchor-proof
cargo test -p rox-anchor-proof --test halt_resume
cargo test -p rox-anchor-proof
```

Why first:

```text
Phase 12 should begin at the source of truth: proof acceptance must block before coordinator, relayer, simulation, or CLI reports can accidentally imply progress.
```

---

### Phase 12 Round 2 — Coordinator refusal while halted

Goal:

```text
Prove coordinator refuses finalization/simulation progression when the proof package is halted or recovery-blocked.
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/
crates/rox-anchor-coordinator/src/decision.rs
crates/rox-anchor-coordinator/src/audit.rs
```

Likely tests:

```text
coordinator_rejects_halted_package_before_simulation
coordinator_rejects_recovery_required_package_before_simulation
coordinator_audit_marks_halted_review_not_permitting_simulation
```

Expected behavior:

```text
- Coordinator decision is not Accepted.
- permits_simulation=false.
- audit status is display-safe but blocked.
- relayer handoff does not happen.
```

Focused commands:

```bash
cargo fmt -p rox-anchor-coordinator
cargo test -p rox-anchor-coordinator
```

---

### Phase 12 Round 3 — Relayer refuses submission while halted

Goal:

```text
Prove relayer dry-run, simulation, and capped authorization cannot be rescued by operator approval when the proof/coordinator state is halted.
```

Likely files:

```text
crates/rox-anchor-relayer/tests/
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/audit.rs
```

Likely tests:

```text
relayer_refuses_halted_proof_review
simulation_rejects_halted_review
capped_submit_rejects_halted_pipeline_even_with_operator_approval
```

Expected behavior:

```text
- relayer status is blocked/rejected
- attempts_used=0 where appropriate
- simulation_status is not Simulated
- capped_submission_status is not Authorized
- network_submitted=false
- live_submission_attempted=false
```

Focused commands:

```bash
cargo fmt -p rox-anchor-relayer
cargo test -p rox-anchor-relayer
```

---

### Phase 12 Round 4 — Halt after proof acceptance but before simulation

Goal:

```text
Model an accepted proof that becomes unsafe before simulation and prove simulation is blocked.
```

Likely design:

```text
Use an accepted proof/coordinator path first.
Then introduce a halted posture or halted pipeline state before simulation.
Ensure simulation cannot proceed on stale accepted proof.
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
crates/rox-anchor-relayer/tests/transaction_simulation.rs
crates/rox-anchor-relayer/src/submit.rs
```

Expected behavior:

```text
- previously accepted proof is not enough if current posture is halted.
- simulation must see current safety posture.
- stale accepted decisions cannot bypass halt.
```

This is important because a kill switch only matters if it can stop operations after a prior acceptance.

---

### Phase 12 Round 5 — Halt after simulation but before capped submission

Goal:

```text
Prove a simulation result cannot authorize capped submission after halt is triggered.
```

Likely files:

```text
crates/rox-anchor-relayer/tests/capped_testnet_submission.rs
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
```

Likely tests:

```text
halt_after_simulation_blocks_capped_authorization
operator_approval_cannot_override_halt_after_simulation
persisted_receipt_cannot_override_halt_after_simulation
```

Expected behavior:

```text
- simulation may have passed earlier
- current halted posture blocks capped authorization
- explicit operator approval does not override halt
- receipt_persisted=true does not override halt
- network_submitted=false
```

---

### Phase 12 Round 6 — Halt after capped testnet submission

Goal:

```text
Model post-submission halt bookkeeping and prove later duplicate/retry/finalization behavior is stopped.
```

This one may need careful local modeling because we do not perform real network submission. The point should be:

```text
- if a capped submission authorization occurred in report-only/testnet model,
  a subsequent halt blocks further attempts, retries, duplicate submits, or finalization claims.
```

Likely files:

```text
crates/rox-anchor-relayer/tests/capped_testnet_submission.rs
crates/rox-anchor-relayer/tests/relayer_retry_storm.rs
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
```

Expected behavior:

```text
- second attempt after halt is rejected
- retry count stays bounded or zero
- audit records halt as blocker
- no fake finality
- no network_submitted=true
```

---

### Phase 12 Round 7 — Recovery restoration path

Goal:

```text
Prove recovery can restore safe operation only through explicit recovery posture/state, never by bypass.
```

Likely files:

```text
crates/rox-anchor-proof/tests/halt_resume.rs
crates/rox-anchor-coordinator/tests/
programs/rox-anchor/src/state.rs
```

Likely tests:

```text
recovery_required_blocks_after_halt
recovery_in_review_blocks_after_halt
recovery_resolved_or_not_required_restores_clean_acceptance
wrong_recovery_path_does_not_restore_acceptance
```

Expected behavior:

```text
- halted blocks
- recovery required blocks
- recovery in review blocks
- active + no recovery + clear challenge allows acceptance only if binding/evidence remains valid
```

---

### Phase 12 Round 8 — Wrong authority cannot halt/recover

Goal:

```text
Expand authority-specific halt/recovery tests across core/program/coordinator/relayer where appropriate.
```

Existing coverage already includes wrong-authority behavior in places, including program state tests and authority model tests. The next session should inspect before adding duplicate tests.

Likely files:

```text
crates/rox-anchor-core/tests/operator_authority_model.rs
programs/rox-anchor/src/state.rs
crates/rox-anchor-coordinator/tests/authority_readiness.rs
crates/rox-anchor-relayer/tests/authority_readiness.rs
```

Likely tests:

```text
wrong_halt_authority_cannot_trigger_halt_drill
wrong_recovery_authority_cannot_clear_recovery_drill
shared_unsafe_authority_cannot_override_kill_switch
```

Expected behavior:

```text
- wrong authority is rejected
- redacted reports do not leak key material
- test-only authority sharing remains explicitly test-only
```

---

### Phase 12 Round 9 — CLI status reflects halt/recovery

Goal:

```text
Make CLI display the halt/recovery drill state safely and deterministically.
```

Likely files:

```text
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/src/commands/halt.rs
crates/rox-anchor-cli/src/commands/recover.rs
crates/rox-anchor-cli/src/commands/posture_audit.rs
crates/rox-anchor-cli/tests/status_display_boundary.rs
crates/rox-anchor-cli/tests/posture_audit_report.rs
```

Potential CLI behavior:

```text
rox-anchor halt
rox-anchor recover
rox-anchor audit-posture
rox-anchor status
```

Expected output principles:

```text
- status labels come from core labels/postures
- no fake “resumed” or “safe” success if recovery is still required
- no wallet/key loading
- no live RPC submission
- no settlement claims
- display_safe=true only when report has no sensitive material
```

Focused commands:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli
cargo run -p rox-anchor-cli -- halt
cargo run -p rox-anchor-cli -- recover
cargo run -p rox-anchor-cli -- audit-posture
```

---

## 12. Likely best first patch for next session

The best first Phase 12 patch should probably be:

```text
BUILD_PLAN2 Phase 12 Round 1 — Coordinator/Relayer Halt Drill Skeleton
```

But do not create empty scaffold. It should add real behavior tests.

A very strong first patch would add a new focused test file:

```text
crates/rox-anchor-coordinator/tests/halt_recovery_kill_switch.rs
```

Test goals:

```text
1. halted package is blocked before coordinator permits simulation
2. recovery-required package is blocked before coordinator permits simulation
3. blocked coordinator review cannot produce relayer dry-run receipt
4. audit remains display-safe and reports permits_simulation=false
```

Why this may be better than adding more proof tests:

```text
The proof crate already has halt/challenge/recovery posture tests.
Phase 12 is about drills across the pipeline.
Coordinator is the next logical boundary after proof acceptance.
```

Alternative first patch:

```text
crates/rox-anchor-relayer/tests/halt_recovery_kill_switch.rs
```

Test goals:

```text
1. halted proof review does not submit dry-run
2. halted proof review cannot simulate
3. halted simulation cannot authorize capped submit
4. operator approval cannot override halted state
```

Both are valid. My recommended order:

```text
Round 1: coordinator halt/recovery kill-switch tests
Round 2: relayer simulation/capped-submit halt tests
Round 3: post-simulation halt blocks capped authorization
Round 4: recovery restoration path
Round 5: CLI status/report drill
Round 6: full workspace/Anchor green confirmation
```

---

## 13. Current estimated completion

Original BUILD_PLAN.md:

```text
complete / green / parked
```

BUILD_PLAN2:

```text
Phase 0–11 complete / green / parked
Phase 12 next
Phase 13 after that
Phase 14 after that
Phase 15 final readiness gate
```

Estimated BUILD_PLAN2 completion:

```text
about 75% complete by phase count
about 80–85% complete by implementation weight
```

Why not higher:

```text
Phase 12 halt/recovery drills are important and security-heavy.
Phase 13 testnet chaos drills may add multiple realistic failure simulations.
Phase 14 audit prep will map invariants, authority, RPC, relayer, mint/burn, halt/recovery, deployment rollback, and non-goals.
Phase 15 is the private testnet readiness gate.
```

Estimated entire ROX Anchor project status including original BUILD_PLAN and BUILD_PLAN2:

```text
about 85–90% complete toward private testnet pilot readiness
```

Caution:

```text
This does not mean public launch readiness.
It means we are far along on local/testnet-only hardening.
Public launch, mainnet, production settlement, public mint/burn, exchange-facing behavior, staking, liquidity, and public bridge UI remain unauthorized.
```

---

## 14. What remains after Phase 12

### Phase 13 — Testnet Chaos Drills

Purpose:

```text
Run realistic failure drills before broader testnet rollout.
```

BUILD_PLAN2 lists drills such as:

```text
RPC outage
RPC disagreement
RPC stale slots
RPC equivocation
program account missing
wrong program ID
wrong mint
wrong token account
reorg-like stale evidence
duplicate relayer request
retry storm
receipt capacity pressure
operator key unavailable
wrong authority key
halt during pending operation
recovery during pending operation
simulation passes but send disabled
send enabled but cap exceeded
```

Exit condition:

```text
Testnet chaos drills fail safely, deterministically, and with inspectable receipts.
```

This phase will likely extend existing chaos tests rather than invent entirely new systems, because the repo already has:

```text
tests/chaos/challenge_griefing.rs
tests/chaos/coordinator_stale_evidence.rs
tests/chaos/halt_resume.rs
tests/chaos/relayer_retry_storm.rs
tests/chaos/rpc_equivocation.rs
```

and crate-local chaos/security tests.

---

### Phase 14 — Audit Preparation

Purpose:

```text
Prepare the repo for external or self-directed security review.
```

BUILD_PLAN2 likely expects docs/audit files such as:

```text
docs/audit/INVARIANT_TEST_MAP.md
docs/audit/AUTHORITY_MODEL.md
docs/audit/STATE_TRANSITIONS.md
docs/audit/RPC_BOUNDARY.md
docs/audit/RELAYER_BOUNDARY.md
docs/audit/HALT_RECOVERY_RUNBOOK.md
docs/audit/TESTNET_DEPLOYMENT_RUNBOOK.md
```

Important: when Phase 14 arrives, docs become appropriate because the code/tests will already exist. Avoid doing Phase 14-style docs during Phase 12 unless a narrow runbook is needed to support a test.

Phase 14 should map:

```text
invariant -> code path -> test
authority -> role -> rejection test
state transition -> valid/invalid transitions
RPC trust boundary -> what is trusted vs observed
relayer boundary -> dry-run/simulate/capped-submit restrictions
mint/burn boundary -> public-vs-test-only guardrails
halt/recovery -> drills and restoration path
deployment rollback -> external key/runbook model
known non-goals -> public launch/mainnet/staking/liquidity/etc.
```

---

### Phase 15 — Testnet Readiness Gate

Purpose:

```text
Decide whether ROX Anchor is ready for a private testnet-only pilot.
```

This is not a public launch gate.

It should confirm:

```text
all local tests pass
all Anchor tests pass
all testnet-only guards pass
mainnet-beta is rejected
no public mint/burn path exists
no public bridge UI exists
no exchange/staking/liquidity behavior exists
operator runbooks exist
receipt/reporting works
halt/recovery drills work
testnet capped submission works only when explicitly enabled
known adversarial cases are tested or documented
```

Exit condition:

```text
ROX Anchor is ready for private testnet-only pilot.
No public token launch is authorized.
No public bridge is authorized.
No production settlement is authorized.
```

---

## 15. Important existing tests to know about

The codebase already has strong coverage. Avoid duplicating tests blindly; extend the exact boundary that Phase 12 needs.

Key existing tests include:

```text
crates/rox-anchor-proof/tests/halt_resume.rs
crates/rox-anchor-proof/tests/challenge_griefing.rs
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
crates/rox-anchor-coordinator/tests/coordinator_stale_evidence_chaos.rs
crates/rox-anchor-coordinator/tests/transaction_simulation_gate.rs
crates/rox-anchor-relayer/tests/capped_testnet_submission.rs
crates/rox-anchor-relayer/tests/transaction_simulation.rs
crates/rox-anchor-relayer/tests/relayer_retry_storm.rs
crates/rox-anchor-rpc-proof/tests/rpc_equivocation_chaos.rs
crates/rox-anchor-cli/tests/posture_audit_report.rs
programs/rox-anchor/src/state.rs tests
```

Use these as examples for new Phase 12 tests.

---

## 16. Suggested next-session command ladder

Start:

```bash
cd /Users/mymac/Desktop/rox-anchor
cargo check --workspace
```

If green, inspect the relevant files before patching:

```bash
sed -n '1,240p' crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
sed -n '1,220p' crates/rox-anchor-relayer/tests/capped_testnet_submission.rs
sed -n '1,220p' crates/rox-anchor-relayer/tests/transaction_simulation.rs
sed -n '1,220p' crates/rox-anchor-proof/tests/halt_resume.rs
```

After first Phase 12 patch:

```bash
cargo fmt -p rox-anchor-coordinator -p rox-anchor-relayer -p rox-anchor-proof
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-proof
```

At a natural boundary:

```bash
cargo check --workspace
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
anchor build
anchor test
```

---

## 17. Next-session opener prompt

Use this in the next session:

```text
We are continuing ROX Anchor.

Current safe status:
BUILD_PLAN.md is complete / green / parked.
BUILD_PLAN2 Phase 0–11 is complete / green / parked.
Most recent completed phase: BUILD_PLAN2 Phase 11 — Observability, Receipts, and Audit Trail.
Latest green run passed focused CLI tests, cargo test -p rox-anchor-cli, cargo check --workspace, focused/workspace Clippy -D warnings, cargo test --workspace, anchor build, and anchor test.

Current workstream:
BUILD_PLAN2 Phase 12 — Halt, Recovery, and Kill-Switch Drills.

Primary references:
BUILD_PLAN2.md
CODEBUNDLE.md
latest terminal output from Phase 11 completion

Start with cargo check --workspace. If green, begin Phase 12 with focused Rust tests proving halt/recovery behavior across proof, coordinator, relayer, simulation, capped-submit, CLI status/reporting, and Anchor state surfaces. Use small behavior patches with focused tests. Fix the first compiler/test failure before expanding.

Scope remains testnet/local only:
no live wallet calls
no silent live RPC submission
no production mint/burn
no production ROC release
no production bridge settlement
no public launch
no mainnet
no exchange/staking/liquidity behavior
```

---

## 18. One-sentence GitHub push summary

```text
Complete Phase 11 audit observability chain.
```

Seven-word version:

```text
Completed deterministic ROX Anchor audit observability.
```

More technical version:

```text
Add CLI audit reports for coordinator, relayer, and posture.
```

---

## 19. Final carry-over conclusion

We ended this session in an excellent state.

The system now has:

```text
safe deterministic RPC proof audit
safe deterministic coordinator audit
safe deterministic relayer/simulation/capped-submit audit
safe deterministic halt/challenge/recovery posture audit
CLI display tests
deterministic output tests
redaction/display-safety checks
full workspace green
Anchor build/test green
```

Phase 11 is complete and should be parked.

The next session should not reopen Phase 11 unless a regression appears. Begin Phase 12 immediately.

The strategic focus now shifts from “can we observe and explain every testnet action?” to:

```text
Can we stop every unsafe path at every stage?
Can halt override prior acceptance?
Can recovery restore operation only through explicit safe paths?
Can wrong authorities fail cleanly?
Can coordinator/relayer/simulation/capped-submit refuse halted states?
Can CLI and audit output show the truth without fake success?
```

That is exactly what BUILD_PLAN2 Phase 12 is meant to prove.


### END NOTE - JULY 4 2026 - 18:25 CST


### BEGIN NOTE - JULY 4 2026 - 20:35 CST

Here are the carry-over notes for the next session. The latest terminal output confirms the corrected `BUILD_PLAN3.md` was written, then `cargo fmt --all`, `cargo test --workspace`, `cargo check --workspace`, `anchor build`, `anchor test`, `scripts/check_testnet_readiness_gate.sh`, and `scripts/make_codebundle.sh` all completed green, ending with the readiness gate passing and `Wrote safe codebundle: CODEBUNDLE.md`. 

# ROX Anchor — Comprehensive Carry-Over Notes for Next Session

## 0. Session Status Summary

We are continuing work on **RustyOnions / CrabLink / ROX Anchor**.

Current active workstream:

```text
ROX Anchor — BUILD_PLAN3 private testnet pilot preparation
```

Current status:

```text
BUILD_PLAN.md: complete / green / parked
BUILD_PLAN2.md: complete / green / parked
BUILD_PLAN3.md: created / corrected / ready to begin
ROX Anchor local implementation: complete / green / parked
ROX Anchor testnet-only hardening: complete / green / parked
Private testnet pilot readiness: proven for review
Private testnet pilot execution: next
Public launch/mainnet/production bridge: not authorized
```

The latest terminal output confirms the repo is green after the `BUILD_PLAN3.md` correction:

```text
cargo fmt --all: green
cargo test --workspace: green
cargo check --workspace: green
anchor build: green
anchor test: green
scripts/check_testnet_readiness_gate.sh: green
scripts/make_codebundle.sh: green
safe CODEBUNDLE.md regenerated
```

Important boundary:

```text
We are not launching publicly.
We are not deploying to mainnet.
We are not authorizing production bridge settlement.
We are not authorizing public ROX mint/burn.
We are not releasing real internal ROC.
We are not adding staking, liquidity, exchange-facing behavior, or public claim/bridge UI.
```

The next session should begin from a clean, green baseline and start **BUILD_PLAN3 Phase 0 / Phase 1**.

---

## 1. What We Accomplished in This Session

### 1.1 Confirmed BUILD_PLAN2 Is Fully Complete

We confirmed that **every phase in BUILD_PLAN2 is complete / green / parked**.

BUILD_PLAN2 covered the testnet-only hardening layer:

```text
Phase 0  — Freeze local green baseline
Phase 1  — Testnet scope locks and safety flags
Phase 2  — Testnet configuration model
Phase 3  — Operator key and authority safety model
Phase 4  — Read-only live RPC adapter
Phase 5  — Transaction simulation model
Phase 6  — Test-only mint and token account harness
Phase 7  — Testnet deployment drill
Phase 8  — Capped testnet submission path
Phase 9  — End-to-end testnet shadow flow
Phase 10 — Security regression expansion
Phase 11 — Observability, receipts, and audit trail
Phase 12 — Halt, recovery, and kill-switch drills
Phase 13 — Testnet chaos drills
Phase 14 — Audit preparation
Phase 15 — Testnet readiness gate
```

The final BUILD_PLAN2 status is:

```text
complete / green / parked
```

The final readiness posture is:

```text
Private testnet pilot readiness is proven.
Public launch remains unauthorized.
Production bridge remains unauthorized.
Mainnet remains unauthorized.
Public mint/burn remains unauthorized.
Exchange/liquidity/staking remains unauthorized.
```

---

### 1.2 Confirmed Final Anchor Tooling Stamp

Earlier in the session, we had a Cargo/readiness-side green status. Then we ran the optional final Anchor tooling commands:

```bash
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Those passed.

This gave the full final green stamp:

```text
anchor build: green
anchor test: green
workspace tests: green
doc-tests: green
secret failsafe scan: green
safe CODEBUNDLE.md regenerated
```

This means the current implementation is not merely Cargo-green; it is also Anchor-tooling-green.

---

### 1.3 Clarified What ROX Anchor Can and Cannot Do Right Now

We clarified the current practical capability:

What we can do now:

```text
compile and test the Anchor program
prove the ROC ↔ ROX state machine locally
model ROC-to-ROX burn-observation -> ROX mint-intent
model ROX-to-ROC burn-observation -> internal ROC release-intent
reject replay
reject mismatches
reject wrong authority
reject wrong mint/account
reject challenge/halt/recovery bypasses
run testnet readiness, deployment-drill, shadow-flow, chaos, audit, and capped-submit checks
```

What we cannot claim yet:

```text
real production ROC burn -> real ROX mint
real ROX burn -> real internal ROC release
public bridge settlement
mainnet launch
public ROX token launch
public bridge UI
exchange/staking/liquidity readiness
```

Correct interpretation:

```text
Local/testnet proof shape: yes
Private capped testnet pilot readiness: yes
Production bridge settlement: no
Public mainnet ROX mint/burn: no
```

---

### 1.4 Estimated Overall ROX Anchor Completion

We estimated the full ROX Anchor project at roughly:

```text
70% complete overall
```

Breakdown:

```text
BUILD_PLAN.md implementation layer: 100%
BUILD_PLAN2 testnet hardening/readiness layer: 100%
Private testnet pilot execution: 0–10%
Production/mainnet bridge readiness: future work
```

The next major chunk is not more local scaffolding. It is the controlled private testnet pilot layer.

---

### 1.5 Created BUILD_PLAN3.md

We created a new plan:

```text
BUILD_PLAN3.md
```

Title:

```text
ROX Anchor Build Plan 3
Private Testnet Pilot / Controlled Live-Testnet Operations
```

Purpose:

```text
Move from testnet readiness to a controlled private testnet pilot.
```

Core meaning:

```text
BUILD_PLAN3 may introduce carefully gated, explicit, capped, private testnet operations.
```

Critical non-authorization boundary:

```text
BUILD_PLAN3 does not authorize public launch.
BUILD_PLAN3 does not authorize mainnet deployment.
BUILD_PLAN3 does not authorize public ROX mint/burn.
BUILD_PLAN3 does not authorize production bridge settlement.
BUILD_PLAN3 does not authorize production ROC release.
BUILD_PLAN3 does not authorize exchange-facing behavior.
BUILD_PLAN3 does not authorize staking.
BUILD_PLAN3 does not authorize liquidity.
BUILD_PLAN3 does not authorize real user value movement.
```

Allowed BUILD_PLAN3 work:

```text
private devnet/testnet deployment
external non-committed testnet keypairs
test-only ROX mint
test-only token accounts
test-only amounts
read-only live RPC verification
transaction simulation
strictly capped private testnet submission
explicit operator approval before every live send path
receipt-backed pilot runs
operator safety drills
halt/recovery drills
upgrade/key-rotation drills
multi-RPC observation checks
private pilot runbooks
CrabLink/internal ROC dry-run adapter work
shadow ROC burn-intent modeling
shadow ROC release-intent modeling
no real internal ROC mutation
```

---

### 1.6 Verified BUILD_PLAN3.md for Correctness

We reviewed the first version of BUILD_PLAN3 and confirmed it was fundamentally correct:

```text
correct scope
correct safety boundary
correct private testnet pilot direction
correct no-production/no-mainnet/no-public-launch language
correct continuation after BUILD_PLAN2
```

We identified one sequencing improvement:

Original order:

```text
Phase 8  — Explicit capped private testnet sender
Phase 9  — Private ROC-to-ROX testnet pilot flow
Phase 10 — Private ROX-to-ROC testnet pilot flow
Phase 11 — Pilot receipt ledger and audit trail
Phase 12 — Pilot CLI command surface
Phase 13 — CrabLink / internal ROC dry-run adapter
```

Improved order:

```text
Phase 8  — Explicit capped private testnet sender
Phase 9  — Pilot receipt ledger and audit trail
Phase 10 — Pilot CLI command surface
Phase 11 — CrabLink / internal ROC dry-run adapter
Phase 12 — Private ROC-to-ROX testnet pilot flow
Phase 13 — Private ROX-to-ROC testnet pilot flow
```

Reason:

```text
The forward/reverse pilot flows depend on receipts, CLI operator commands, and dry-run ROC intent/release shapes. Those should exist before the full pilot flows.
```

We also improved Phase 4 wording.

Original Phase 4 title:

```text
Real Private Testnet Deployment Drill
```

Corrected Phase 4 title:

```text
Private Testnet Deployment Drill / Optional Deployment
```

Reason:

```text
The phase should support a checked deployment drill and optional actual private testnet deploy, without making actual deployment sound automatic or required.
```

---

### 1.7 Patched BUILD_PLAN3.md

We wrote a complete file replacement patch for `BUILD_PLAN3.md`.

The patch:

```text
rewrote BUILD_PLAN3.md
kept the non-negotiable scope boundary
kept the private testnet pilot purpose
renamed Phase 4 safely
moved receipt ledger before pilot flows
moved CLI command surface before pilot flows
moved CrabLink/internal ROC dry-run adapter before pilot flows
renumbered Phases 9–13 correctly
updated the final status wording
updated the build order summary
preserved the first-command section
```

Corrected Phase order is now:

```text
Phase 0  — Freeze BUILD_PLAN2 green baseline
Phase 1  — Private pilot operator workspace hygiene
Phase 2  — External testnet config loader
Phase 3  — Testnet program artifact manifest
Phase 4  — Private testnet deployment drill / optional deployment
Phase 5  — Test-only mint initialization runbook
Phase 6  — Live read-only RPC verification against testnet
Phase 7  — Simulation-only pilot transaction plans
Phase 8  — Explicit capped private testnet sender
Phase 9  — Pilot receipt ledger and audit trail
Phase 10 — Pilot CLI command surface
Phase 11 — CrabLink / internal ROC dry-run adapter
Phase 12 — Private ROC-to-ROX testnet pilot flow
Phase 13 — Private ROX-to-ROC testnet pilot flow
Phase 14 — Live testnet chaos and incident drills
Phase 15 — Authority, upgrade, halt, and recovery operational drills
Phase 16 — Private testnet pilot closeout gate
```

---

### 1.8 Ran Final Validation Commands After BUILD_PLAN3 Patch

After patching `BUILD_PLAN3.md`, the following command batch was run:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_testnet_readiness_gate.sh .
bash scripts/make_codebundle.sh
```

Result:

```text
all green
```

Key confirmations from latest terminal output:

```text
cargo fmt --all completed
cargo test --workspace passed
cargo check --workspace passed
anchor build passed
anchor test passed
testnet readiness gate passed
secret failsafe scan passed
safe CODEBUNDLE.md regenerated
```

Program tests:

```text
programs/rox-anchor: 96 tests passed
```

The test suite covered, among many others:

```text
test-only mint harness rejection/acceptance
challenge blocking
halt/recovery blocking
config mint authority binding
wrong authority rejection
wrong mint rejection
direction helper rejection
finalize plan validation
finalized event tamper rejection
PDA derivation checks
token account constraint snapshots
token CPI readiness
token CPI execution receipts
ROC-to-ROX finalization classification
ROX-to-ROC finalization classification
settlement binding derivation
planned event receipt checks
wrong direction rejection
```

CLI tests remained green, including:

```text
check command
audit command
posture audit
proof read-only RPC command
relayer audit
status display boundary
testnet config status
testnet deployment drill checker
testnet readiness gate
kill-switch drill command
capped submit report
```

Coordinator tests remained green, including:

```text
readiness
queue capacity
duplicate operation rejection
stale RPC evidence rejection
valid observation handoff to proof
authority readiness
audit records
coordinator-relayer boundary
stale evidence chaos
halt/recovery finalization gate
local nonvalue bidirectional flow
testnet shadow flow
transaction simulation gate
```

Core tests remained green, including:

```text
identifier validation
lifecycle state classification
safe deterministic labels
posture helpers
kill-switch drills
operator authority model
test-only asset harness
testnet config model
testnet scope locks
```

Proof tests remained green, including:

```text
valid fixture accepted
binding mismatches rejected in deterministic order
missing/disputed evidence blocks acceptance
challenge/halt/recovery postures block acceptance
replay of operation ID/idempotency key/nonce rejected
challenge griefing
halt/resume behavior
```

Relayer tests remained green, including:

```text
bounded retry
readiness rejection of zero limits
blocked proof does not submit
dry-run receipt generation
idempotency replay rejection
authority readiness
capped testnet submission
halt/recovery submit gate
retry storm containment
testnet audit record
testnet chaos drills
transaction simulation
testnet scope locks
```

RPC proof tests remained green, including:

```text
readiness threshold rejection
binding mismatch rejection
matching RPC observations
conflicting signatures
redaction
stale observation rejection
too few sources
read-only RPC adapter
RPC audit records
RPC equivocation chaos
RPC-to-proof boundary
testnet chaos drills
testnet config model
testnet scope locks
```

The readiness gate confirmed:

```text
BUILD_PLAN2.md found
Anchor.toml found
audit docs found
testnet readiness doc found
audit prep checker found
deployment drill checker found
testnet readiness checker found
testnet readiness gate tests found
build plan phase 15 recognized
phase 15 non-launch boundary recognized
private testnet pilot exit condition recognized
workspace test/check command references present
readiness audit/deploy/gate references present
private pilot boundary present
mainnet-beta rejection test present
no public launch flags test present
authority rotation test present
wrong-authority halt/recovery test present
ROC-to-ROX shadow flow test present
ROX-to-ROC shadow flow test present
capped submit all-gates test present
receipt persistence test present
kill-switch halt drill test present
Anchor.toml has no mainnet program section
Anchor.toml provider cluster is not mainnet
tracked files do not include common Solana key material names
Phase 14 audit prep checker remains green
testnet deployment drill checker remains green
Phase 15 testnet readiness gate checks passed
private testnet-only pilot review surface is present
public launch/mainnet/production settlement/public mint-burn/staking/liquidity/exchange behavior remain unauthorized
script did not deploy, submit, mint, burn, settle, call RPC, or load a wallet
```

Final bundle result:

```text
== Secret failsafe scan on generated bundle ==
Wrote safe codebundle: CODEBUNDLE.md
```

---

## 2. Current Final Status at End of Session

Final status:

```text
BUILD_PLAN.md: COMPLETE / GREEN / PARKED
BUILD_PLAN2.md: COMPLETE / GREEN / PARKED
BUILD_PLAN3.md: CREATED / CORRECTED / READY TO BEGIN
ROX Anchor implementation: GREEN
ROX Anchor testnet hardening: GREEN
ROX Anchor private testnet pilot readiness: GREEN
ROX Anchor private testnet pilot execution: NEXT
```

Current codebase health:

```text
cargo fmt --all: green
cargo test --workspace: green
cargo check --workspace: green
anchor build: green
anchor test: green
testnet readiness gate: green
safe codebundle generation: green
```

Current build phase:

```text
Ready to begin BUILD_PLAN3 Phase 0.
```

Important nuance:

```text
BUILD_PLAN3 has been created and validated, but its implementation phases are not yet worked through.
```

We have a strong starting baseline for Phase 0 because the final validation commands are already green, but in the next session we should still formally begin with Phase 0 and preserve the green baseline before modifying code.

---

## 3. What BUILD_PLAN3 Means

BUILD_PLAN3 is not another local-only proof plan.

It is the next safe layer:

```text
private testnet pilot
controlled live-testnet operations
external non-committed keys
test-only assets
read-only RPC
simulation
explicit capped submit
receipts
operator drills
CrabLink/internal ROC dry-run adapter
```

BUILD_PLAN3 is still not:

```text
mainnet
public launch
production bridge
public token launch
real ROC release
real user funds
public bridge UI
staking
liquidity
exchange-facing behavior
```

The mental model:

```text
BUILD_PLAN.md  = local implementation
BUILD_PLAN2.md = testnet-only hardening/readiness
BUILD_PLAN3.md = private testnet pilot
future plan     = production-readiness/mainnet/public decision, if ever authorized
```

---

## 4. BUILD_PLAN3 Phase-by-Phase Carry-Over

### Phase 0 — Freeze BUILD_PLAN2 Green Baseline

Purpose:

```text
Create a clean starting point for private testnet pilot work.
```

Current status:

```text
Ready to begin.
```

Evidence already available:

```text
The final command batch is green.
```

Recommended next-session first action:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_testnet_readiness_gate.sh .
bash scripts/make_codebundle.sh
```

Then mark Phase 0 baseline as confirmed.

Expected Phase 0 exit:

```text
The repo starts BUILD_PLAN3 from a known green and secret-safe state.
```

No code change may be needed for Phase 0 unless we decide to add a `BUILD_PLAN3` baseline checker.

Possible optional Phase 0 patch:

```text
Add scripts/check_build_plan3_baseline.sh
Add crates/rox-anchor-cli/tests/build_plan3_baseline.rs
Check BUILD_PLAN3.md exists
Check BUILD_PLAN.md and BUILD_PLAN2.md exist
Check readiness gate remains green
Check no common key material tracked
Check Anchor.toml has no mainnet section
Check BUILD_PLAN3 non-authorization boundary exists
```

But avoid unnecessary checker theater. If we add a checker, it must be useful and should not replace code work.

---

### Phase 1 — Private Pilot Operator Workspace Hygiene

Purpose:

```text
Prepare the operator-side environment without committing secrets.
```

Likely first real BUILD_PLAN3 implementation phase.

Work items:

```text
define expected local-only pilot directory layout
ensure keys/RPC URLs/deployment outputs/pilot receipts stay outside tracked source
extend .gitignore only if necessary
add checker rejecting tracked key/RPC/provider secret material
add tests for checker
document local-only artifact layout
do not generate or load real keypairs in tests
```

Likely files:

```text
.gitignore
scripts/check_private_pilot_hygiene.sh
crates/rox-anchor-cli/tests/private_pilot_hygiene.rs
docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
```

Recommended implementation shape:

```text
1. Create docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md.
2. Add local ignored directories if missing:
   - .rox-anchor-private-pilot/
   - private-pilot/
   - pilot-receipts/
   - pilot-artifacts/
   - pilot-config/
3. Add a script that checks tracked files for common forbidden key names and provider/RPC token files.
4. Add a CLI integration test that runs the checker.
5. Run focused tests.
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
bash scripts/check_private_pilot_hygiene.sh .
cargo test -p rox-anchor-cli --test private_pilot_hygiene
cargo test -p rox-anchor-cli
```

Exit condition:

```text
Private pilot workspace model exists and tracked files cannot include common key/RPC secret material.
```

---

### Phase 2 — External Testnet Config Loader

Purpose:

```text
Move from in-code config models to an explicit external pilot config format without storing secrets.
```

Important rule:

```text
The config may reference external paths and redacted URLs, but it must not commit secrets.
```

Work items:

```text
private pilot config shape
external RPC URL
external keypair path
explicit mode
mainnet-beta rejection
public/production label rejection
redacted reports
fixture configs with no secrets
tests for valid config, missing fields, mainnet rejection, redaction
```

Likely files:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-rpc-proof/src/config.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-coordinator/src/config.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/tests/private_pilot_config.rs
```

Potential approach:

```text
Reuse existing testnet config types where possible.
Do not duplicate safety profile/state enums.
Add private pilot config wrappers only if needed.
Keep all validation deterministic.
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-rpc-proof -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-core
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-cli --test private_pilot_config
```

Exit condition:

```text
Private pilot config can be loaded, validated, and redacted without secrets or production/mainnet ambiguity.
```

---

### Phase 3 — Testnet Program Artifact Manifest

Purpose:

```text
Track deployment artifacts safely without treating them as production truth.
```

Work items:

```text
non-secret deployment manifest
program ID
cluster
build hash
IDL hash
deploy slot if supplied
operator-visible labels
mainnet-beta rejection
empty program ID rejection
Anchor.toml binding check
path redaction
manifest validation tests
no deployment success claims from manifest presence alone
```

Likely files:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/tests/testnet_program_manifest.rs
docs/pilot/TESTNET_PROGRAM_MANIFEST.md
```

Important distinction:

```text
A manifest can describe an artifact.
A manifest does not prove deployment success.
A manifest does not authorize production runtime.
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
cargo test -p rox-anchor-core
cargo test -p rox-anchor-cli --test testnet_program_manifest
cargo test -p rox-anchor-cli
anchor build
```

Exit condition:

```text
Private pilot can describe deployed testnet program artifact without storing secrets or claiming production finality.
```

---

### Phase 4 — Private Testnet Deployment Drill / Optional Deployment

Purpose:

```text
Prepare or perform the first real private testnet deployment using external keys, without treating the drill as production launch.
```

Important:

```text
Actual deploy is optional.
External keys required.
No committed secrets.
No production claim.
```

Likely files:

```text
scripts/check_private_testnet_deploy.sh
docs/pilot/PRIVATE_TESTNET_DEPLOYMENT_RUNBOOK.md
crates/rox-anchor-cli/tests/private_testnet_deploy_drill.rs
```

Manual command remains commented/optional:

```bash
# anchor deploy --provider.cluster testnet --provider.wallet /external/path/to/testnet-payer.json
```

Exit condition:

```text
Deployment drill is reproducible, external-key-only, redacted, and non-production.
```

Clippy checkpoint after Phase 4:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

---

### Phase 5 — Test-Only Mint Initialization Runbook

Purpose:

```text
Initialize private testnet state with test-only assets.
```

Work items:

```text
test-only ROX mint initialization checklist
explicit testnet mode
explicit test-only mint label
tiny supply/cap limits
mint authority separation
halt/recovery authorities
public/production label rejection
redacted initialization intent reports
initialization intent validation tests
```

Likely files:

```text
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/initialize.rs
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-cli/tests/test_only_mint_initialization.rs
docs/pilot/TEST_ONLY_MINT_INITIALIZATION.md
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor -p rox-anchor-cli
cargo test -p rox-anchor-core
cargo test -p rox-anchor
cargo test -p rox-anchor-cli --test test_only_mint_initialization
anchor build
anchor test
```

Exit condition:

```text
Private testnet initialization can only target explicit test-only assets with separated authority and tiny caps.
```

---

### Phase 6 — Live Read-Only RPC Verification Against Testnet

Purpose:

```text
Verify deployed private testnet state through live read-only RPC.
```

Work items:

```text
read-only RPC command path
current slot fetch
program account status
config account status if supplied
mint account status if supplied
token account status if supplied
convert live reads into RPC proof observations
reject stale/missing/disputed/mismatched observations
keep all submission disabled
fake-adapter tests for failures
```

Likely files:

```text
crates/rox-anchor-rpc-proof/src/rpc.rs
crates/rox-anchor-rpc-proof/src/quorum.rs
crates/rox-anchor-rpc-proof/src/readiness.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/tests/private_testnet_read_only_rpc.rs
```

Optional manual command:

```bash
# cargo run -p rox-anchor-cli -- proof read-only --config /external/path/to/private-testnet.toml
```

Exit condition:

```text
Private pilot can verify testnet state using read-only RPC without transaction submission.
```

---

### Phase 7 — Simulation-Only Pilot Transaction Plans

Purpose:

```text
Create actual pilot transaction plans but do not send them.
```

Work items:

```text
transaction plan models for initialize/observe/challenge/resolve/halt/recover/finalize
require accepted proof before simulation
require accepted coordinator decision before simulation
require relayer dry-run before simulation
require read-only RPC verification before simulation
reject halted/challenged/recovery-required/stale/disputed/mismatched inputs
deterministic simulation reports
tests proving no bypass
```

Likely files:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-coordinator/src/decision.rs
crates/rox-anchor-cli/src/commands/submit.rs
crates/rox-anchor-relayer/tests/private_pilot_simulation.rs
```

Exit condition:

```text
Private pilot transaction plans can be simulated only after all prior proof and read-only gates pass.
```

---

### Phase 8 — Explicit Capped Private Testnet Sender

Purpose:

```text
Introduce first real send-capable path, still disabled by default and restricted to private testnet.
```

Work items:

```text
explicit send-capable adapter behind TestnetSubmitCapped
operator approval required
external config required
testnet/devnet cluster required
mainnet-beta rejected
accepted proof required
accepted coordinator decision required
relayer dry-run required
successful simulation required
retry cap required
amount cap required
operation cap required
receipt output path required
no halt/challenge/recovery blocker
tests for every missing guard
send never default
```

Likely files:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-cli/src/commands/submit.rs
crates/rox-anchor-relayer/tests/private_testnet_sender.rs
crates/rox-anchor-cli/tests/private_testnet_submit_command.rs
```

Optional manual command:

```bash
# cargo run -p rox-anchor-cli -- submit capped-testnet \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT"
```

Exit condition:

```text
The only live send path is private-testnet-only, capped, explicit, simulation-backed, receipt-backed, and impossible from default mode.
```

Clippy checkpoint after Phase 8:

```bash
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

---

### Phase 9 — Pilot Receipt Ledger and Audit Trail

Purpose:

```text
Make every private pilot action inspectable and replay-resistant before full pilot flows.
```

Work items:

```text
pilot receipt IDs
receipt chain/hash linkage if needed
persist proof review
persist RPC quorum
persist coordinator decision
persist simulation result
persist send authorization
persist transaction signature when present
persist readback verification
redact secrets and sensitive paths
reject duplicate receipt IDs
reject mismatched operation IDs
reject receipts claiming live submit when no send occurred
reject receipts claiming production settlement
deterministic display tests
```

Likely files:

```text
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/audit.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-relayer/tests/private_pilot_receipts.rs
crates/rox-anchor-cli/tests/private_pilot_receipt_display.rs
```

Exit condition:

```text
Every private pilot action has a redacted, deterministic, replay-resistant receipt trail.
```

---

### Phase 10 — Pilot CLI Command Surface

Purpose:

```text
Make private pilot operations usable without unsafe defaults before full ROC↔ROX pilot flows.
```

Work items:

```text
pilot status commands
read-only proof command
simulation-only command
capped submit command
receipt inspect command
halt/recovery drill command support for pilot mode
explicit flags for every non-read-only path
unknown/ambiguous flag rejection
help output tests proving production/mainnet/public launch modes do not exist
```

Likely files:

```text
crates/rox-anchor-cli/src/commands/mod.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/src/commands/submit.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/tests/private_pilot_cli.rs
```

Exit condition:

```text
CLI can run private pilot read-only, simulation, capped submit, status, and receipt inspection paths without unsafe defaults.
```

---

### Phase 11 — CrabLink / Internal ROC Dry-Run Adapter

Purpose:

```text
Prepare future CrabLink/RustyOnions handoff before running ROC↔ROX pilot flows, without real ROC mutation.
```

Work items:

```text
dry-run internal ROC burn-intent input shape
dry-run internal ROC release-intent output shape
test-only values
explicit non-production mode
do not call svc-wallet
do not mutate ron-ledger
do not unlock paid content
do not let CrabLink claim final settlement
display-safe status tests
tests proving no wallet/ledger mutation path exists
```

Likely files:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-coordinator/src/observer.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-coordinator/tests/internal_roc_dry_run_adapter.rs
crates/rox-anchor-cli/tests/crablink_private_pilot_status.rs
docs/pilot/CRABLINK_INTERNAL_ROC_DRY_RUN_ADAPTER.md
```

Exit condition:

```text
ROX Anchor can exchange dry-run intent/status shapes with future CrabLink/RustyOnions integration without mutating real ROC or claiming final settlement.
```

---

### Phase 12 — Private ROC-to-ROX Testnet Pilot Flow

Purpose:

```text
Run forward bridge shape privately with test-only inputs after receipt, CLI, and dry-run adapter surfaces exist.
```

Work items:

```text
model CrabLink/internal ROC burn-intent input
do not burn real ROC
explicit test operation ID
explicit idempotency key
nonce
proof review
RPC quorum review
coordinator decision
relayer dry-run
simulation
optional explicitly approved capped private testnet transaction
mint only test ROX
persist receipt
verify receipt by read-only RPC
replay/mismatch/wrong mint/wrong token/stale RPC/halted path tests
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/private_roc_to_rox_pilot.rs
crates/rox-anchor-relayer/tests/private_roc_to_rox_pilot.rs
crates/rox-anchor-rpc-proof/tests/private_roc_to_rox_readback.rs
crates/rox-anchor-cli/tests/private_roc_to_rox_command.rs
docs/pilot/ROC_TO_ROX_PRIVATE_PILOT.md
```

Exit condition:

```text
Private ROC-to-ROX pilot flow can mint only test ROX after every proof, simulation, cap, and receipt gate passes.
```

Clippy checkpoint after Phase 12:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
```

---

### Phase 13 — Private ROX-to-ROC Testnet Pilot Flow

Purpose:

```text
Run reverse bridge shape privately without real internal ROC release.
```

Work items:

```text
observe test ROX burn evidence
do not release real ROC
produce internal ROC release-intent only
explicit test operation ID
explicit idempotency key
nonce
proof review
RPC quorum review
coordinator decision
relayer dry-run
simulation where applicable
persist receipt
verify burn/readback by read-only RPC
prove no direct internal ROC mutation
prove future real ROC path must delegate to svc-wallet/ron-ledger
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/private_rox_to_roc_pilot.rs
crates/rox-anchor-relayer/tests/private_rox_to_roc_pilot.rs
crates/rox-anchor-rpc-proof/tests/private_rox_to_roc_readback.rs
crates/rox-anchor-cli/tests/private_rox_to_roc_command.rs
docs/pilot/ROX_TO_ROC_PRIVATE_PILOT.md
```

Exit condition:

```text
Private ROX-to-ROC pilot flow can prove test ROX burn evidence and produce only a dry-run internal ROC release-intent, with no real ROC mutation.
```

---

### Phase 14 — Live Testnet Chaos and Incident Drills

Purpose:

```text
Prove private testnet operations fail safely under realistic conditions.
```

Required drills:

```text
RPC outage during read-only proof
RPC disagreement during readback
stale slot/readback evidence
wrong program ID
wrong mint
wrong token account
missing program account
missing config account
missing receipt file
receipt tamper
duplicate receipt
duplicate operation ID
duplicate idempotency key
nonce reuse
operator approval omitted
wrong authority attempted
halt before simulation
halt after simulation before submit
halt after capped testnet submit
recovery during pending operation
simulation passes but send disabled
send enabled but cap exceeded
readback missing after send
```

Exit condition:

```text
Private testnet chaos drills fail safely, deterministically, and with inspectable receipts.
```

---

### Phase 15 — Authority, Upgrade, Halt, and Recovery Operational Drills

Purpose:

```text
Practice human/operator safety procedures before any broader pilot.
```

Work items:

```text
upgrade authority checklist
mint authority checklist
halt authority checklist
recovery authority checklist
wrong-authority rejection drill
key rotation intent drill
halted-system read-only status drill
recovery-from-halt drill
redacted operator reports
tests proving no private key exposure or production-safety claims
```

Likely files:

```text
docs/pilot/PRIVATE_PILOT_AUTHORITY_DRILLS.md
docs/pilot/PRIVATE_PILOT_HALT_RECOVERY_DRILLS.md
crates/rox-anchor-core/tests/private_pilot_authority_drills.rs
crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs
```

Exit condition:

```text
Private pilot authority, upgrade, halt, and recovery procedures are tested, redacted, and operator-readable.
```

---

### Phase 16 — Private Testnet Pilot Closeout Gate

Purpose:

```text
Decide whether ROX Anchor has completed the private testnet pilot and is ready for a separate future production-readiness plan.
```

Work items:

```text
confirm all local tests pass
confirm all Anchor tests pass
confirm all private pilot checks pass
confirm private pilot receipts exist if manual runs were performed
confirm no key material is tracked
confirm no public launch behavior
confirm no mainnet behavior
confirm no production settlement behavior
confirm no real ROC release behavior
confirm no public bridge UI
confirm no exchange/staking/liquidity behavior
confirm halt/recovery drills
confirm known pilot failures documented
confirm any next plan is separate and explicitly scoped
```

Likely files:

```text
docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md
scripts/check_private_testnet_pilot_closeout.sh
crates/rox-anchor-cli/tests/private_testnet_pilot_closeout.rs
```

Exit condition:

```text
ROX Anchor private testnet pilot is complete / green / parked.
Future production-readiness may be drafted separately.
No public/mainnet/production behavior is authorized by BUILD_PLAN3.
```

Final Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

---

## 5. Key Invariants to Preserve in Next Session

### 5.1 Core Ownership

```text
rox-anchor-core owns:
- shared IDs
- domains
- directions
- states
- labels
- challenge posture
- halt posture
- recovery posture
- testnet scope types
- authority role concepts where shared
- reusable errors
```

Do not create competing enums in CLI/coordinator/relayer/RPC proof.

---

### 5.2 Proof Ownership

```text
rox-anchor-proof owns deterministic proof review.
```

It must remain the source for:

```text
valid acceptance
missing evidence
disputed evidence
replay rejection
binding mismatch rejection
challenge blocking
halt blocking
recovery blocking
deterministic findings
```

---

### 5.3 Replay Rejection

Always reject reuse of:

```text
operation ID
idempotency key
nonce
```

Any BUILD_PLAN3 private pilot flow must retain this.

---

### 5.4 Binding Rejection

Always reject mismatches in:

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
authority
receipt ID
simulation result
readback result
```

---

### 5.5 No Hidden Live Behavior

Live behavior must be:

```text
explicit
capped
testnet/devnet only
operator-approved
receipt-backed
not default
impossible from local-only/simulate-only mode
blocked by halt/challenge/recovery
blocked by missing receipt
blocked by failed simulation
blocked by missing proof/coordinator/relayer acceptance
```

No command should silently:

```text
load a wallet
call live RPC submission
deploy
mint
burn
settle
release ROC
claim finality
```

---

### 5.6 CrabLink Boundary

CrabLink is future display/user-intent surface only.

CrabLink must not:

```text
create bridge receipts
create balances
claim bridge finality
release paid content based on pilot state
mutate internal ROC
bypass svc-wallet/ron-ledger
display production settlement
```

Allowed future CrabLink statuses:

```text
private pilot
test-only
dry-run
pending proof
pending readback
simulation only
capped testnet submit
not production
not final
manual review required
halted
recovery required
```

---

### 5.7 Internal ROC Boundary

For real future ROC mutation, only this path is acceptable:

```text
svc-wallet -> ron-ledger
```

ROX Anchor must not directly mutate:

```text
ron-ledger
svc-wallet balance state
internal ROC balances
paid access truth
CrabLink display balances
```

BUILD_PLAN3 Phase 11 should explicitly prove that the adapter is dry-run only.

---

## 6. Recommended Next-Session Opening Prompt

Use this prompt in the next session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
BUILD_PLAN.md is complete / green / parked.
BUILD_PLAN2.md is complete / green / parked.
BUILD_PLAN3.md has been created, corrected, and is ready to begin.

The latest terminal output after patching BUILD_PLAN3 shows:
cargo fmt --all: green
cargo test --workspace: green
cargo check --workspace: green
anchor build: green
anchor test: green
scripts/check_testnet_readiness_gate.sh: green
scripts/make_codebundle.sh: green

Current workstream:
BUILD_PLAN3 — Private Testnet Pilot / Controlled Live-Testnet Operations.

Important boundary:
No mainnet, no public launch, no production bridge settlement, no public ROX mint/burn, no real internal ROC release, no staking, no liquidity, no exchange-facing behavior, no real user funds.

Please start with BUILD_PLAN3 Phase 0, confirm the green baseline, then proceed to Phase 1: Private Pilot Operator Workspace Hygiene. Use QuickChain-style patches: real code/checkers/tests, focused commands, fix first failure before expanding.
```

---

## 7. Files to Attach in Next Session

Minimum:

```text
BUILD_PLAN3.md
latest CODEBUNDLE.md
latest terminal output
```

Recommended:

```text
BUILD_PLAN.md
BUILD_PLAN2.md
IDB.md
TODO.md
NOTES.MD
```

Only attach the large historical RustyOnions/CrabLink docs if needed. For the next coding session, the most useful files are:

```text
BUILD_PLAN3.md
CODEBUNDLE.md
Pasted terminal output
```

---

## 8. Immediate Next Commands for Next Session

Start with:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_testnet_readiness_gate.sh .
bash scripts/make_codebundle.sh
```

Then begin BUILD_PLAN3 Phase 0.

If Phase 0 is treated as baseline-only, then the first real patch should be Phase 1:

```text
Private Pilot Operator Workspace Hygiene
```

Likely first Phase 1 patch:

```text
docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
scripts/check_private_pilot_hygiene.sh
crates/rox-anchor-cli/tests/private_pilot_hygiene.rs
.gitignore additions if needed
```

Focused commands after Phase 1 patch:

```bash
bash scripts/check_private_pilot_hygiene.sh .
cargo test -p rox-anchor-cli --test private_pilot_hygiene
cargo test -p rox-anchor-cli
cargo test --workspace
```

---

## 9. Completion Estimate After This Session

Updated estimate:

```text
ROX Anchor local implementation: 100%
ROX Anchor testnet hardening/readiness: 100%
BUILD_PLAN3 plan creation: 100%
BUILD_PLAN3 implementation: 0%
Entire ROX Anchor project: ~70%
```

After BUILD_PLAN3 completes, the project would likely be around:

```text
85–90% complete toward private/testnet operational confidence
```

Remaining after BUILD_PLAN3 would be a separate future plan for:

```text
external audit
production authority/multisig
mainnet decision gate
public bridge UI gating
real RustyOnions svc-wallet/ron-ledger adapter
production incident response
economic limits
public launch controls
legal/compliance review
mainnet deployment
production settlement
```

Do not start that future production plan until BUILD_PLAN3 is green/parked and explicitly requested.

---

## 10. Recommended First Patch in Next Session

The best first implementation patch is:

```text
BUILD_PLAN3 Phase 1 — Private Pilot Operator Workspace Hygiene
```

Why Phase 1 first:

```text
Before any private testnet config, manifest, deploy drill, read-only RPC, simulation, or capped send path, we need the operator workspace and secret hygiene boundary locked.
```

Expected patch contents:

```text
1. Create docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md.
2. Create scripts/check_private_pilot_hygiene.sh.
3. Add crates/rox-anchor-cli/tests/private_pilot_hygiene.rs.
4. Possibly extend .gitignore for:
   - .rox-anchor-private-pilot/
   - private-pilot/
   - pilot-receipts/
   - pilot-artifacts/
   - pilot-config/
   - *.pilot-receipt.json
   - *.pilot-manifest.json
   - *.pilot-config.toml
5. Run focused tests.
```

Script should check for:

```text
common Solana keypair names
testnet keypair artifacts
private pilot key material
provider token files
RPC URL local files
raw secret-like filenames
committed pilot receipts containing obvious secret names
```

The script should not:

```text
load wallets
call RPC
deploy
submit
mint
burn
settle
read real external secret files
```

Test should verify:

```text
checker exists
checker passes current repo
checker references BUILD_PLAN3/private pilot boundary
checker rejects key material patterns
checker does not claim deployment/submission/success
```

---

## 11. Git Commit Message Options

If committing this session’s work:

Longer:

```text
Add BUILD_PLAN3 private testnet pilot plan
```

Ten-word option:

```text
Add corrected BUILD_PLAN3 for private testnet pilot operations
```

Seven-word option:

```text
Add private testnet pilot build plan
```

Very short:

```text
Add BUILD_PLAN3 private pilot plan
```

---

## 12. Final Carry-Over Bottom Line

This session completed the transition from BUILD_PLAN2 into BUILD_PLAN3.

We:

```text
confirmed BUILD_PLAN2 is complete / green / parked
confirmed the final Anchor tooling stamp is green
clarified current ROC↔ROX capability
estimated overall ROX Anchor completion around 70%
created BUILD_PLAN3.md
verified BUILD_PLAN3 for correctness
fixed BUILD_PLAN3 phase ordering and Phase 4 wording
ran the full green command suite after the patch
regenerated a safe CODEBUNDLE.md
```

The next session should not redesign the plan. It should begin implementing it.

Next active target:

```text
BUILD_PLAN3 Phase 0 -> Phase 1
```

First meaningful implementation target:

```text
Private Pilot Operator Workspace Hygiene
```

End-state target for BUILD_PLAN3:

```text
ROX Anchor private testnet pilot complete / green / parked,
with no public/mainnet/production bridge behavior authorized.
```

This should be enough to restart cleanly in the next session with `BUILD_PLAN3.md`, the new codebundle, and the terminal output.


### END NOTE - JULY 4 2026 - 20:35 CST