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


### BEGIN NOTE - JULY 5 2026 - 12:35 CST

Absolutely. Here are the carry-over notes for the next ROX Anchor / BUILD_PLAN3 session.

# ROX Anchor — BUILD_PLAN3 Carry-Over Notes For Next Session

## 0. Current Project Posture

We are working on **ROX Anchor**, the Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path.

Current active plan:

```text
BUILD_PLAN3 — Private Testnet Pilot / Controlled Live-Testnet Operations
```

Current status:

```text
BUILD_PLAN.md  — complete / green / parked
BUILD_PLAN2.md — complete / green / parked
BUILD_PLAN3.md — active
```

Current active phase for next session:

```text
BUILD_PLAN3 Phase 9 — Pilot Receipt Ledger and Audit Trail
```

Current completion rate:

```text
BUILD_PLAN3: Phase 0 through Phase 8 complete.
9 of 17 phases complete.
Approximate BUILD_PLAN3 completion: 52.9%, rounded to 53%.
```

The broader ROX Anchor multi-plan arc is roughly:

```text
BUILD_PLAN.md complete
BUILD_PLAN2.md complete
BUILD_PLAN3 now about 53% complete
Overall ROX Anchor arc roughly 83% complete
```

This completion estimate is phase-count based, not a guarantee of production readiness. The current work remains private testnet / controlled pilot only. It does **not** authorize mainnet, public launch, public ROX mint/burn, production bridge settlement, production ROC release, staking, liquidity, exchange behavior, custody, or real user value movement.

The latest terminal output shows the stale CLI test was corrected, the focused CLI reruns passed, the full workspace test run proceeded green through the shown suites, and the Phase 8-specific tests are now clean.  The latest codebundle was regenerated after this work, contains 191 tracked/unignored text files, excludes secrets/local keys/generated bundles, and explicitly states that the bundle is not runtime authorization. 

---

## 1. Session Summary

This session advanced BUILD_PLAN3 from the post-Phase-6 state through **Phase 7** and **Phase 8**, then parked Phase 8 green.

The work accomplished:

```text
Phase 6 was confirmed green / parked.
Phase 7 was implemented and closed green.
Phase 8 was implemented, fixed, tested, clippy-checked, and closed green.
The active next phase is Phase 9.
```

The most important strategic milestone from this session:

```text
ROX Anchor now has a private-pilot send-capable authorization model, but it is still gated, capped, explicit, external-config-backed, simulation-backed, receipt-path-backed, and non-executing in tests/CLI reports.
```

This means we moved past pure read-only and simulation-only surfaces into the first carefully controlled “send-capable shape,” while still preserving the core safety rule:

```text
No default send path.
No silent live RPC submission.
No wallet/key loading in tests.
No signing in tests.
No automatic network submission.
No real ROC mutation.
No production settlement claim.
No public mint/burn path.
```

---

## 2. What Was Completed Before This Session

The following BUILD_PLAN3 phases were already complete / green / parked before the latest Phase 7 and Phase 8 work:

### Phase 0 — Freeze BUILD_PLAN2 Green Baseline

Purpose:

```text
Start BUILD_PLAN3 from a known green, secret-safe state.
```

Completed posture:

```text
BUILD_PLAN.md parked.
BUILD_PLAN2.md parked.
BUILD_PLAN3 active.
Workspace test/check/build surfaces green.
Testnet readiness gate green.
Codebundle generation secret-safe.
```

### Phase 1 — Private Pilot Operator Workspace Hygiene

Purpose:

```text
Keep private pilot operator artifacts, keys, RPC URLs, receipts, and deployment outputs out of tracked source.
```

Completed posture:

```text
Private pilot workspace hygiene checker exists.
.gitignore covers pilot artifacts.
CLI hygiene tests exist.
Docs describe the private operator workspace.
Tracked files reject obvious key/RPC secret material.
```

### Phase 2 — External Testnet Config Loader

Purpose:

```text
Load private pilot/testnet config from external shapes without storing secrets.
```

Completed posture:

```text
External pilot config model exists.
Mainnet-beta rejected.
Production/public labels rejected.
RPC URLs and key paths redacted.
Core/RPC-proof/relayer/coordinator/CLI tests cover config handling.
```

### Phase 3 — Testnet Program Artifact Manifest

Purpose:

```text
Represent testnet/devnet program artifact metadata without treating manifest presence as proof of deployment or production finality.
```

Completed posture:

```text
Manifest tracks non-secret program artifact shape.
Anchor.toml devnet/testnet program ID binding is checked.
Mainnet-beta/localnet/private/public label issues are rejected.
Display remains redacted and non-finality-claiming.
```

### Phase 4 — Private Testnet Deployment Drill / Optional Deployment

Purpose:

```text
Prepare or perform a private testnet deployment drill using external keys without treating it as production.
```

Completed posture:

```text
Deployment drill script exists.
Runbook exists.
CLI test checks deployment drill docs/script boundary.
External deploy keypair, payer, and authority paths are expected.
No committed deploy keys or raw deploy outputs are allowed.
```

### Phase 5 — Test-Only Mint Initialization Runbook

Purpose:

```text
Validate private testnet/test-only mint initialization intent without live mint initialization.
```

Completed posture:

```text
Test-only mint initialization intent/review/finding model exists.
Test-only labels required.
Tiny caps enforced.
Authority separation required.
Halt/recovery authorities required.
Public/production labels rejected.
Tests cover zero/over-cap supply, bad labels, missing authority, unsafe shared authority, and non-testnet harnesses.
```

### Phase 6 — Live Read-Only RPC Verification Against Testnet

Purpose:

```text
Verify deployed private testnet state using read-only RPC-shaped adapters without transaction submission.
```

Completed posture:

```text
Private testnet read-only RPC verification model exists.
Fake adapters test all failure cases.
Program/config/mint/token account readback shape exists.
Missing, stale, mismatched, disputed, and owner-mismatched observations fail closed.
All submission remains disabled.
CLI proof/read-only boundary remains safe.
```

The corrected BUILD_PLAN3 order places Phase 9 receipt ledger before the full ROC↔ROX pilot flows, because the pilot flows depend on receipts, CLI operator commands, and dry-run ROC intent/release shapes. 

---

## 3. What Was Accomplished In This Session

## Phase 7 — Simulation-Only Pilot Transaction Plans

Phase 7 purpose:

```text
Create actual pilot transaction plans but do not send them.
```

### 3.1 Phase 7 Core Work Added

This session added a private pilot simulation wrapper around the existing transaction simulation model.

Added conceptual model:

```text
PrivatePilotTransactionKind
PrivatePilotTransactionStep
PrivatePilotSimulationPlan
PrivatePilotSimulationStatus
PrivatePilotSimulationResult
simulate_private_pilot_transaction_plan(...)
```

The new simulation model supports transaction-shaped pilot steps such as:

```text
initialize
observe
open_challenge
resolve_challenge
halt
recover
finalize
```

### 3.2 Phase 7 Safety Properties

Phase 7 requires:

```text
read-only RPC verification before simulation
non-empty transaction step list
declared instruction count must match summed step instruction count
accepted proof review
accepted coordinator decision
accepted relayer dry-run receipt
non-submitting safety scope
```

Phase 7 rejects:

```text
missing read-only RPC verification
missing transaction steps
instruction-count mismatch
unsafe/submitting scope
missing coordinator acceptance
blocked proof review
relayer dry-run rejection
```

Phase 7 report output remains safe:

```text
private_pilot_simulation: local_only
live_submission: false
wallet_key_loading: false
internal_roc_mutation: false
base_live_submission: false
```

### 3.3 Phase 7 Tests Added

Added relayer tests:

```text
crates/rox-anchor-relayer/tests/private_pilot_simulation.rs
```

Test coverage:

```text
private_pilot_simulation_accepts_only_after_read_only_and_dry_run_gates
private_pilot_simulation_requires_read_only_rpc_verification
private_pilot_simulation_rejects_missing_steps_and_instruction_mismatch
private_pilot_simulation_rejects_missing_coordinator_acceptance
private_pilot_simulation_rejects_blocked_proof_and_relayer_dry_run
private_pilot_simulation_rejects_unsafe_submitting_scope
```

Added CLI doc/runbook boundary test:

```text
crates/rox-anchor-cli/tests/private_pilot_simulation.rs
```

Added runbook:

```text
docs/pilot/SIMULATION_ONLY_PILOT_TRANSACTION_PLANS.md
```

### 3.4 Phase 7 Validation

Focused Phase 7 checks passed:

```text
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test private_pilot_simulation
cargo test -p rox-anchor-relayer --test transaction_simulation
cargo test -p rox-anchor-cli --test private_pilot_simulation
```

Closure checks passed:

```text
cargo test --workspace
anchor build
bash scripts/make_codebundle.sh
```

Phase 7 status:

```text
complete / green / parked
```

---

## Phase 8 — Explicit Capped Private Testnet Sender

Phase 8 purpose:

```text
Introduce the first real send-capable path, still disabled by default and restricted to private testnet.
```

This was the biggest step of the session.

### 3.5 Phase 8 Core Work Added

Added the explicit private testnet capped sender authorization model.

New concept:

```text
PRIVATE_TESTNET_CAPPED_SEND_APPROVAL
```

Exact approval phrase:

```text
I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT
```

New sender status model:

```text
PrivateTestnetSenderStatus
```

Important statuses:

```text
Authorized
MissingExternalConfig
UnsafeExternalConfig
MissingReceiptOutputPath
PendingOperationalBlocker
MissingOperatorApproval
SimulationNotAccepted
CappedAuthorizationRejected
```

New request model:

```text
PrivateTestnetSenderRequest
```

It carries:

```text
external private pilot config
private pilot simulation result
capped submission limits
requested attempts
requested operation count
amount units
operator approval phrase
receipt output path declaration
operational posture
```

New authorization model:

```text
PrivateTestnetSenderAuthorization
```

It reports:

```text
status
capped_result
authorized
live_submission_permitted
live_submission_attempted
network_submitted
wallet_key_loading
signing
external_config_validated
receipt_output_path_redacted
```

New function:

```text
authorize_private_testnet_sender(...)
```

### 3.6 Phase 8 Required Gates

The private sender authorization requires all of the following:

```text
external config exists
external config validates
relayer safety mode is TestnetSubmitCapped
pilot testnet submission mode is TestnetSubmitCapped
receipt output path is declared
operational posture does not block submission
operator approval phrase exactly matches PRIVATE_TESTNET_CAPPED_SEND_APPROVAL
private pilot simulation result is simulated
private pilot simulation did not set live_submission
read-only RPC verification happened before simulation
base transaction simulation exists
base transaction simulation is simulated
base transaction simulation did not set live_submission
capped testnet submission authorization passes
retry cap passes
operation cap passes
amount cap passes
receipt persistence requirement passes
```

### 3.7 Phase 8 Explicit Non-Execution Boundary

Even when authorization succeeds, the model still reports:

```text
live_submission_attempted: false
network_submitted: false
wallet_key_loading: false
signing: false
```

The redacted report also keeps these safety lines:

```text
rpc_submission: disabled_in_local_authorization_model
mint_burn_execution: disabled_in_local_authorization_model
internal_roc_mutation: disabled
settlement_claim: none
```

This distinction is important:

```text
Phase 8 can authorize a capped private testnet send path in model/report form.
Phase 8 does not perform a real send in tests.
Phase 8 does not load wallets or sign.
Phase 8 does not submit to RPC.
Phase 8 does not mint or burn real value.
Phase 8 does not release ROC.
```

### 3.8 Phase 8 CLI Command Surface Added

The CLI now accepts the private-pilot command group:

```text
rox-anchor submit
rox-anchor submit capped-testnet
```

Important CLI design:

```text
submit is now a valid command group
capped-testnet is explicit
unknown submit subcommands fail closed
there is no default send path
```

The nested submit command maps to the already-safe capped submit report path:

```text
cargo run -p rox-anchor-cli -- submit capped-testnet --authorize-testnet-submit-capped --receipt-persisted
```

This still produces a report/authorization shape, not an actual network submit.

### 3.9 Phase 8 Runbook Added

Added:

```text
docs/pilot/EXPLICIT_CAPPED_PRIVATE_TESTNET_SENDER.md
```

The runbook states the phase requires:

```text
external config
operator approval
receipt output path
successful simulation
read-only RPC verification
no default send path
no wallet loading
no signing
no internal ROC mutation
```

### 3.10 Phase 8 Tests Added

Added relayer test file:

```text
crates/rox-anchor-relayer/tests/private_testnet_sender.rs
```

Test coverage:

```text
private_testnet_sender_authorizes_only_after_every_phase8_gate
private_testnet_sender_rejects_missing_external_config
private_testnet_sender_rejects_unsafe_external_config_mode
private_testnet_sender_rejects_missing_operator_approval_and_receipt_output_path
private_testnet_sender_rejects_pending_operational_blockers
private_testnet_sender_rejects_unverified_or_blocked_simulation
private_testnet_sender_surfaces_capped_authorization_rejections
```

Added CLI test file:

```text
crates/rox-anchor-cli/tests/private_testnet_submit_command.rs
```

Test coverage:

```text
submit_group_help_has_no_default_send_path
nested_capped_testnet_submit_command_uses_existing_safe_report
unknown_submit_subcommand_fails_closed
private_testnet_sender_runbook_is_explicit_capped_and_non_executing
```

Existing test retained and passed:

```text
crates/rox-anchor-cli/tests/capped_submit_report.rs
```

### 3.11 Phase 8 Initial Workspace Failure

The initial Phase 8 closure run found one stale test:

```text
rox-anchor-cli src/lib.rs test: unknown_command_is_error
```

Cause:

```text
The old test expected `rox-anchor submit` to be an unknown command.
Phase 8 intentionally made `submit` a valid command group.
```

The failure was correct and expected after the feature change. The stale test was updated to use:

```text
rox-anchor definitely-unknown
```

instead of:

```text
rox-anchor submit
```

### 3.12 Phase 8 Fix

The stale CLI unit test was patched so it still verifies unknown top-level commands fail closed, without treating the new valid `submit` group as invalid.

Focused reruns passed:

```text
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --lib
cargo test -p rox-anchor-cli --test private_testnet_submit_command
```

The latest terminal output shows:

```text
rox-anchor-cli lib tests: 6 passed
private_testnet_submit_command tests: 4 passed
```



### 3.13 Phase 8 Closure

Phase 8 closure commands were run:

```text
cargo test --workspace
anchor build
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
bash scripts/make_codebundle.sh
```

The current codebundle was regenerated after this phase and lists the Phase 8 files, including:

```text
crates/rox-anchor-cli/tests/private_testnet_submit_command.rs
crates/rox-anchor-relayer/tests/private_testnet_sender.rs
docs/pilot/EXPLICIT_CAPPED_PRIVATE_TESTNET_SENDER.md
```

It also shows the repo now contains 191 tracked/unignored text files in the safe bundle. 

Phase 8 status:

```text
complete / green / parked
```

---

## 4. Current Phase Map

The corrected BUILD_PLAN3 order is:

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

The codebundle confirms this corrected order and the key reason for it: the pilot flows depend on receipts, CLI operator commands, and dry-run ROC intent/release shapes, so those surfaces must exist before full ROC-to-ROX / ROX-to-ROC pilot flows. 

Current active point:

```text
Phase 0 through Phase 8 are complete.
Phase 9 is next.
```

---

## 5. Current Safety Boundary To Preserve

Do not regress any of these rules:

```text
No mainnet-beta deployment.
No public Solana token launch.
No public ROX minting.
No public ROX burning.
No production bridge settlement.
No production ROC release.
No real internal ROC release from ROX burn evidence.
No real user funds.
No public bridge UI.
No public faucet.
No public claim page.
No exchange-facing behavior.
No staking.
No liquidity pools.
No market making.
No custody services.
No unbounded relayer submission.
No silent wallet/key usage.
No silent live RPC submission.
No committed operator key material.
No raw private keys in logs.
No fake finality.
No fake success output.
No CrabLink display claiming final settlement before backend proof.
```

Still allowed in BUILD_PLAN3:

```text
private devnet/testnet deployment or deployment drill
external non-committed testnet keypairs
test-only ROX mint
test-only token accounts
test-only amounts
read-only live RPC verification
transaction simulation
strictly capped private testnet submission authorization
explicit operator approval before every live send path
receipt-backed pilot run modeling
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

## 6. What To Do Next Session

Next session should begin with **BUILD_PLAN3 Phase 9 — Pilot Receipt Ledger and Audit Trail**.

Phase 9 purpose:

```text
Make every private pilot action inspectable and replay-resistant before full pilot flows are attempted.
```

This phase is important because Phase 8 introduced a send-capable authorization model. Before any forward/reverse pilot flows are built, the project needs a durable local receipt model that can record the proof/coordinator/RPC/simulation/send-authorization story in one deterministic, redacted, replay-resistant trail.

Phase 9 should not create a broad planning doc only. It should add real Rust behavior and tests.

Recommended next-session starting posture:

```text
Do not patch broad future pilot flows yet.
Do not start Phase 10 CLI expansion yet.
Do not start CrabLink/internal ROC adapter yet.
Start by building the private pilot receipt ledger model in rox-anchor-relayer.
Add focused tests beside it.
Then add CLI display tests only after the core receipt behavior is real.
```

---

## 7. Phase 9 Design Intent

The receipt ledger should answer these questions for any private pilot action:

```text
What operation was this for?
Which proof review was used?
Which RPC quorum/read-only readback was used?
Which coordinator decision was used?
Which relayer dry-run receipt was used?
Which simulation result was used?
Which capped sender authorization was used?
Was a transaction signature present?
Was there readback verification after a send?
Was the receipt local-only or send-authorized?
Was any live submission actually attempted?
Does the receipt claim anything it is not allowed to claim?
Is the receipt duplicate?
Does the receipt match the operation ID?
Does it expose secrets or sensitive paths?
```

The model should make it impossible for a receipt to safely claim:

```text
production settlement
public launch
mainnet finality
real ROC release
access granted
mint complete
burn complete
bridge complete
wallet/key loaded
RPC submitted
```

unless future plans explicitly authorize those states, which BUILD_PLAN3 does not.

---

## 8. Suggested Phase 9 Implementation Shape

Best first target:

```text
crates/rox-anchor-relayer/src/receipts.rs
```

Likely new types:

```text
PrivatePilotReceiptId
PrivatePilotReceiptStage
PrivatePilotReceiptKind
PrivatePilotReceiptRecord
PrivatePilotReceiptLedger
PrivatePilotReceiptFinding
PrivatePilotReceiptReview
PrivatePilotReceiptDisplay
```

Possible stage enum:

```text
ProofReviewed
RpcQuorumReviewed
CoordinatorAccepted
RelayerDryRunAccepted
SimulationAccepted
SenderAuthorized
TransactionObserved
ReadbackVerified
Blocked
Rejected
```

Possible receipt kind enum:

```text
ReadOnlyRpc
SimulationOnly
CappedSenderAuthorization
PostSendReadback
HaltDrill
RecoveryDrill
PilotFlowDryRun
```

Possible critical fields:

```text
receipt_id
operation_id
idempotency_key
nonce
cluster
program_id
mint
token_account
direction
proof_decision
rpc_quorum_decision
coordinator_status
relayer_status
simulation_status
sender_status
signature_redacted
readback_status
created_slot_or_observed_slot
previous_receipt_id
receipt_hash_or_link_hash
live_submission_attempted
network_submitted
wallet_key_loading
signing
internal_roc_mutation
settlement_claim
```

The receipt can remain local/deterministic for now. It does not need crypto-grade hashing yet unless the existing code already has a hash utility. If adding a chain/hash linkage, keep it deterministic and testable without new dependencies unless needed.

Recommended conservative approach:

```text
Use existing ID wrappers where possible.
Use string IDs only through core validated identifier wrappers.
Keep hash/linkage simple and deterministic.
Avoid adding external crypto dependencies unless the workspace already has a suitable local hash path.
Do not invent new state machines that overlap with core/proof decisions.
Use existing proof/coordinator/relayer statuses rather than duplicating semantics.
```

---

## 9. Phase 9 Required Rejections

Phase 9 tests should prove the receipt ledger rejects:

```text
duplicate receipt IDs
duplicate receipt IDs with different contents
mismatched operation IDs across linked records
mismatched idempotency keys across linked records
receipt claiming live submission when sender authorization says no send occurred
receipt claiming network_submitted when network_submitted is false upstream
receipt claiming wallet_key_loading/signing when local report says disabled
receipt claiming production settlement
receipt claiming real ROC release
receipt claiming public launch
receipt claiming mainnet-beta
receipt with unredacted key path
receipt with raw RPC token URL
receipt with raw signature where redaction is required
receipt with missing proof/coordinator/simulation evidence when kind requires it
receipt with send authorization but missing receipt output path
receipt with sender authorization but failed simulation
receipt with sender authorization but missing read-only RPC verification
```

---

## 10. Phase 9 Tests To Add

Likely relayer test:

```text
crates/rox-anchor-relayer/tests/private_pilot_receipts.rs
```

Test names to consider:

```text
private_pilot_receipt_records_full_authorized_non_executing_chain
private_pilot_receipt_rejects_duplicate_receipt_id
private_pilot_receipt_rejects_operation_mismatch
private_pilot_receipt_rejects_live_submission_claim_without_send_attempt
private_pilot_receipt_rejects_production_settlement_claims
private_pilot_receipt_redacts_sensitive_paths_and_signatures
private_pilot_receipt_links_previous_record_deterministically
private_pilot_receipt_rejects_sender_authorization_without_simulation
private_pilot_receipt_rejects_readback_mismatch
```

Likely CLI display test:

```text
crates/rox-anchor-cli/tests/private_pilot_receipt_display.rs
```

Test names to consider:

```text
private_pilot_receipt_display_is_deterministic_and_redacted
private_pilot_receipt_display_does_not_claim_settlement_or_access
private_pilot_receipt_display_rejects_unknown_or_unsafe_claims
```

Possible docs/runbook file:

```text
docs/pilot/PILOT_RECEIPT_LEDGER.md
```

The doc should be tied to real tests and should not become broad docs theater. Keep it short, operational, and test-referenced.

---

## 11. Phase 9 Commands For Next Session

Start focused. Do not jump straight to broad expansions.

Recommended first commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test private_pilot_receipts
cargo test -p rox-anchor-cli --test private_pilot_receipt_display
```

Then focused crate checks:

```bash
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-cli
```

Then closure:

```bash
cargo test --workspace
anchor build
bash scripts/make_codebundle.sh
```

No Clippy checkpoint is required at Phase 9 by policy. The next scheduled Clippy checkpoint is after Phase 12. However, a focused Clippy run can still be used if the new receipt code is complex or warnings are suspected.

---

## 12. Implementation Boundaries For Phase 9

Do not do these in Phase 9:

```text
Do not implement full ROC-to-ROX pilot flow yet.
Do not implement full ROX-to-ROC pilot flow yet.
Do not call live RPC.
Do not load keypairs.
Do not sign transactions.
Do not submit transactions.
Do not mutate internal ROC.
Do not unlock CrabLink paid access.
Do not create public bridge UI.
Do not add production/mainnet labels.
Do not add fake finality or fake success strings.
```

Phase 9 should only build the receipt/audit substrate needed before pilot flows.

Phase 9 may reference transaction signatures as optional redacted fields, but should not actually create or submit any transaction.

---

## 13. Important Existing Surfaces To Reuse

Phase 9 should call into or record from existing real surfaces, not invent parallel rules.

Existing surfaces to reuse:

```text
rox-anchor-core:
  shared IDs
  domains
  clusters
  directions
  operation IDs
  idempotency keys
  nonces
  posture helpers
  safety profiles
  labels
  authority model
  private pilot config shapes

rox-anchor-proof:
  ProofReview
  ReviewDecision
  findings
  expected binding
  replay rejection posture
  challenge/halt/recovery blockers

rox-anchor-rpc-proof:
  RPC quorum review
  read-only RPC verification
  current slot/account/signature readback model
  agreement/missing/disputed/stale/mismatched evidence

rox-anchor-coordinator:
  coordinator decision records
  queue/review gates
  stale evidence rejection
  local nonvalue bidirectional flow shapes
  testnet shadow flow behavior

rox-anchor-relayer:
  RelayerReceipt
  RelayerDryRun
  TransactionSimulationPlan
  PrivatePilotSimulationResult
  CappedTestnetSubmissionResult
  PrivateTestnetSenderAuthorization
  retry/operation/amount caps
  redacted reports

rox-anchor-cli:
  submit command group
  capped-testnet safe report
  status/proof/audit displays
  display boundary tests
```

The key invariant:

```text
Phase 9 receipts should describe the decisions from existing components; they should not become a second proof engine, second coordinator, second relayer, or second settlement state machine.
```

---

## 14. Important Phase 8 Details To Preserve In Phase 9

When building receipts, preserve these Phase 8 semantics:

```text
Authorized sender result can mean live_submission_permitted: true.
It does not mean live_submission_attempted: true.
It does not mean network_submitted: true.
It does not mean wallet_key_loading: true.
It does not mean signing: true.
It does not mean mint/burn executed.
It does not mean settlement happened.
It does not mean ROC was released.
```

The receipt ledger must preserve the distinction between:

```text
authorized to send under strict private testnet caps
actually attempted a send
network accepted/submitted transaction
readback verified on testnet
settlement/finality claim
```

At the current state:

```text
Only the first concept exists in local authorization form.
The latter concepts must remain false/none unless explicitly modeled later in safe phases.
```

---

## 15. Known Stale-Test Lesson From Phase 8

Phase 8 changed the CLI command map:

```text
rox-anchor submit
```

is now valid and returns safe help.

Therefore, tests must not expect `submit` to be unknown.

The corrected unknown-command test now uses:

```text
rox-anchor definitely-unknown
```

This should remain as-is.

Future CLI tests should distinguish:

```text
unknown top-level command
unknown submit subcommand
known submit group help
known capped-testnet report command
```

Current CLI posture:

```text
submit group exists
capped-testnet subcommand exists
unknown submit subcommands fail closed
no default send path exists
```

---

## 16. Next Phases After Phase 9

## Phase 10 — Pilot CLI Command Surface

Purpose:

```text
Make private pilot operations usable without unsafe defaults before running full ROC↔ROX pilot flows.
```

This should come after Phase 9 because the CLI should inspect real receipt records, not fake receipt strings.

Expected work:

```text
harden pilot status commands
add receipt inspect command
add read-only proof command shape if not already enough
add simulation-only command shape
add capped submit command shape
add halt/recovery drill command support for pilot mode
require explicit flags for every non-read-only path
reject unknown or ambiguous flags
help output proves no production/mainnet/public launch modes exist
```

Expected tests:

```text
crates/rox-anchor-cli/tests/private_pilot_cli.rs
```

Important:

```text
Do not use CLI success strings as fake finality.
Every CLI output should come from real core/proof/rpc/coordinator/relayer/receipt behavior.
```

## Phase 11 — CrabLink / Internal ROC Dry-Run Adapter

Purpose:

```text
Prepare future CrabLink/RustyOnions handoff without real ROC mutation.
```

Expected work:

```text
dry-run internal ROC burn-intent input shape
dry-run internal ROC release-intent output shape
test-only values
explicit non-production mode
no svc-wallet calls
no ron-ledger mutation
no paid-content unlock
no final settlement display
display-safe statuses
tests proving no wallet/ledger mutation path exists
```

Important invariant:

```text
RustyOnions svc-wallet -> ron-ledger remains the only future real internal ROC mutation path.
ROX Anchor must not directly issue/release real ROC.
```

## Phase 12 — Private ROC-to-ROX Testnet Pilot Flow

Purpose:

```text
Run the forward bridge shape privately with test-only inputs.
```

Expected work:

```text
model CrabLink/internal ROC burn-intent input
do not burn real ROC
require explicit test operation ID
require idempotency key
require nonce
review proof package
run RPC quorum review
run coordinator decision
run relayer dry-run
run simulation
if explicitly approved, send capped private testnet transaction that mints only test ROX
persist receipt
verify receipt by read-only RPC
add replay/mismatch/wrong mint/wrong token/stale RPC/halted path tests
```

This is the next Clippy checkpoint boundary after Phase 12:

```text
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
```

## Phase 13 — Private ROX-to-ROC Testnet Pilot Flow

Purpose:

```text
Run the reverse bridge shape privately without real internal ROC release.
```

Expected work:

```text
observe test ROX burn evidence
do not release real ROC
produce internal ROC release-intent only
require explicit test operation ID
require idempotency key
require nonce
review proof package
run RPC quorum review
run coordinator decision
run relayer dry-run
run simulation if applicable
persist receipt
verify burn/readback by read-only RPC
prove no direct internal ROC mutation can occur
prove any future real ROC path must delegate to svc-wallet/ron-ledger
```

## Phase 14 — Live Testnet Chaos and Incident Drills

Purpose:

```text
Prove private testnet operations fail safely under realistic conditions.
```

Expected drills:

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

## Phase 15 — Authority, Upgrade, Halt, and Recovery Operational Drills

Purpose:

```text
Practice human/operator safety procedures before broader pilot.
```

Expected work:

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
tests proving authority drills never expose private keys or claim production safety
```

## Phase 16 — Private Testnet Pilot Closeout Gate

Purpose:

```text
Decide whether ROX Anchor has completed the private testnet pilot and is ready for a separate future production-readiness plan.
```

Expected work:

```text
confirm all local tests pass
confirm Anchor tests pass
confirm private pilot checks pass
confirm pilot receipts exist for manual runs if any were performed
confirm no key material tracked
confirm no public launch behavior exists
confirm no mainnet behavior exists
confirm no production settlement behavior exists
confirm no real ROC release behavior exists
confirm no public bridge UI exists
confirm no exchange/staking/liquidity behavior exists
confirm halt/recovery drills performed or simulated
confirm known pilot failures documented
confirm any future plan is separate and explicitly scoped
```

Final Clippy checkpoint occurs after Phase 16:

```text
core
proof
cli
rpc-proof
coordinator
relayer
anchor program
```

---

## 17. Recommended First Message For Next Session

Use something like:

```text
We are continuing ROX Anchor BUILD_PLAN3.
BUILD_PLAN.md and BUILD_PLAN2.md are complete / green / parked.
BUILD_PLAN3 Phase 8 — Explicit Capped Private Testnet Sender is complete / green / parked.
Current active phase: BUILD_PLAN3 Phase 9 — Pilot Receipt Ledger and Audit Trail.
Start by running the focused baseline if needed, then implement a real receipt ledger/audit trail in rox-anchor-relayer with focused tests. Do not add live RPC, wallet loading, signing, transaction submission, real ROC mutation, public mint/burn, or production settlement behavior.
Use QuickChain style: small Rust behavior patch, focused tests, fix first failure, then closure.
```

---

## 18. Current Confidence / Risk Assessment

Current confidence is high.

Reasons:

```text
BUILD_PLAN3 Phase 0–8 are green.
Phase 8 focused tests passed.
The stale CLI unit test was correctly fixed.
Workspace testing passed after the fix.
Phase 8 Clippy checkpoint was run for relayer and CLI.
The codebundle regenerated safely.
The active next phase is well-sequenced before full pilot flows.
```

Main risk for next session:

```text
Receipt ledger design could become too broad or duplicate existing state machines.
```

Mitigation:

```text
Keep Phase 9 focused on recording and validating existing decisions.
Do not create new acceptance/finality rules.
Use existing proof/coordinator/rpc/relayer/simulation/sender statuses.
Keep receipt claims conservative.
Reject unsafe claims aggressively.
Add tests beside each real behavior.
```

Second risk:

```text
Confusing authorization with execution.
```

Mitigation:

```text
Keep these separate in receipt records:
authorized
live_submission_permitted
live_submission_attempted
network_submitted
readback_verified
settlement_claim
internal_roc_mutation
```

Third risk:

```text
Display wording accidentally implies finality, settlement, access, mint/burn completion, or production readiness.
```

Mitigation:

```text
Add display boundary tests early.
Reuse existing unsafe phrase patterns.
Prefer explicit disabled/none wording.
```

---

## 19. Final Session State

Final state at handoff:

```text
ROX Anchor BUILD_PLAN3 Phase 8 is complete / green / parked.
Next phase is Phase 9 — Pilot Receipt Ledger and Audit Trail.
BUILD_PLAN3 completion is about 53%.
Overall ROX Anchor multi-plan arc is roughly 83%.
No production, mainnet, public launch, real ROC, public bridge, staking, liquidity, exchange, or custody behavior is authorized.
```

The next session should begin by building the Phase 9 receipt ledger and audit trail in small compile-tested Rust patches, with focused tests first and workspace closure only after the focused pass is green.

That should be enough to paste directly into the next session as the working handoff.


### END NOTE - JULY 5 2026 - 12:35 CST


### BEGIN NOTE - JULY 5 2026 - 17:45 CST

Below are the carry-over notes for the next ROX Anchor session.

# ROX Anchor / BUILD_PLAN3 Carry-Over Notes

## Current Session Status

We ended this session at a clean stopping point:

```text
BUILD_PLAN3 Phase 13 — Private ROX-to-ROC Testnet Pilot Flow
Status: COMPLETE / GREEN / PARKED
```

The next session should begin at:

```text
BUILD_PLAN3 Phase 14 — Live Testnet Chaos and Incident Drills
Status: NOT STARTED
```

Current BUILD_PLAN3 progress:

```text
Phases complete: 0 through 13
Count complete: 14 of 17 phases
BUILD_PLAN3 completion estimate: ~82%
```

This is **private testnet pilot progress only**. It does **not** mean production bridge readiness, public ROX launch, mainnet readiness, public mint/burn, exchange readiness, staking readiness, liquidity readiness, or real ROC release readiness.

BUILD_PLAN3 itself explicitly keeps the boundary narrow: it allows private devnet/testnet deployment, external non-committed keypairs, test-only ROX mint/token accounts/amounts, read-only RPC verification, transaction simulation, capped private testnet submission, explicit operator approval, receipt-backed pilot runs, and dry-run ROC intent modeling, while continuing to forbid public launch, mainnet, production bridge settlement, production ROC release, real user value movement, public bridge UI, staking, liquidity, exchange-facing behavior, silent submission, committed operator keys, fake finality, and fake success output. 

---

# 1. What We Accomplished In This Session

## 1.1 We finished the reverse private pilot flow

The major session goal was BUILD_PLAN3 Phase 13:

```text
Private ROX-to-ROC Testnet Pilot Flow
```

Purpose:

```text
Run the reverse bridge shape privately without real internal ROC release.
```

The Phase 13 work required the system to observe **test ROX burn evidence**, review it through the same proof/RPC/coordinator/relayer path, and produce only a **dry-run internal ROC release-intent**. The build plan required explicit operation ID, idempotency key, nonce, proof review, RPC quorum, coordinator decision, relayer dry-run, simulation where applicable, receipt/readback verification, tests proving no direct internal ROC mutation, and tests proving any future real ROC release path must go through `svc-wallet -> ron-ledger`. 

We got that surface green.

The final focused test run proved:

```text
rox-anchor-relayer private_rox_to_roc_pilot: 3/3 passed
rox-anchor-relayer private_pilot_simulation: 6/6 passed
rox-anchor-cli private_rox_to_roc_command: 4/4 passed
rox-anchor-coordinator private_rox_to_roc_pilot: 4/4 passed
rox-anchor-rpc-proof private_rox_to_roc_readback: 3/3 passed
cargo test --workspace: passed
secret failsafe codebundle generation: passed
```

The last terminal output shows the relayer reverse pilot test passed all 3 tests, including replay rejection, missing read-only RPC gate rejection, and the dry-run release-intent path. It also shows the private pilot simulation tests passed all 6 tests, the CLI reverse command tests passed all 4 tests, the coordinator reverse pilot tests passed all 4 tests, and the RPC readback tests passed all 3 tests. 

## 1.2 We fixed the final Phase 13 report mismatch

At the end of Phase 13, behavior was already mostly correct, but the relayer test exposed a reporting mismatch. The test expected explicit wording around no network submission, but the report only implied it through existing fields.

The final patch updated the private pilot simulation report to include explicit safe lines:

```text
network_submission: disabled
settlement_claim: none
```

This was important because for ROX-to-ROC we do **not** want ambiguous reporting. A reverse pilot report must not merely imply safety; it should directly say:

```text
no network submission
no wallet key loading
no internal ROC mutation
no settlement claim
```

That made the report stronger and brought the focused relayer Phase 13 test green.

## 1.3 We preserved the ROX-to-ROC safety boundary

Phase 13’s most important invariant is:

```text
ROX Anchor may prove test ROX burn evidence,
but it must not directly release real ROC.
```

The Phase 13 pilot document records the correct reverse boundary:

```text
The reverse pilot accepts only test ROX burn evidence.
It produces only a dry-run internal ROC release intent.
It does not release real ROC.
It does not call svc-wallet.
It does not mutate ron-ledger.
It does not unlock paid content.
It does not claim final settlement.
```

It also records the future real ROC release boundary:

```text
svc-wallet -> ron-ledger
```

ROX Anchor remains evidence, coordination, simulation, and on-chain Anchor state machinery. It must not become a direct internal ROC issuer. 

## 1.4 We confirmed the current codebundle is secret-safe and reviewable

The generated codebundle reports that it was created by `scripts/make_codebundle.sh`, includes tracked/unignored text files, excludes secrets, ignored local metadata, build outputs, dependency folders, local Solana keys, and generated codebundles, and explicitly says the artifact is not runtime authorization. 

The codebundle file list confirms all relevant Phase 13 and future Phase 14 surfaces are present, including:

```text
crates/rox-anchor-cli/tests/private_rox_to_roc_command.rs
crates/rox-anchor-coordinator/tests/private_rox_to_roc_pilot.rs
crates/rox-anchor-relayer/tests/private_rox_to_roc_pilot.rs
crates/rox-anchor-rpc-proof/tests/private_rox_to_roc_readback.rs
docs/pilot/ROX_TO_ROC_PRIVATE_PILOT.md
crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs
crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs
```

That means the next session has a good base for Phase 14, with existing chaos-test surfaces already present and ready to expand instead of starting from scratch. 

---

# 2. Current Architecture Posture

## 2.1 The project is no longer in scaffold mode

ROX Anchor is now a real compile-tested, test-backed private testnet pilot implementation surface.

The completed layers now include:

```text
shared core types
proof package validation
replay rejection
challenge/halt/recovery blockers
read-only RPC evidence modeling
coordinator decision path
relayer dry-run path
simulation gates
explicit capped private testnet sender authorization
pilot receipt ledger/audit trail
pilot CLI command surface
CrabLink/internal ROC dry-run adapter
private ROC-to-ROX flow
private ROX-to-ROC flow
testnet shadow flow
chaos and safety regression surfaces
Anchor program state/mint-burn logic tests
```

The current posture is **not** “fake bridge success.” It is controlled private-testnet/test-only bridge-shape validation with strong non-production boundaries.

## 2.2 Core division of responsibility remains correct

Carry this forward strongly:

```text
rox-anchor-core:
  shared IDs, domains, directions, states, safety labels,
  challenge/halt/recovery posture, internal ROC dry-run shapes,
  testnet scope locks, authority model, reusable errors

rox-anchor-proof:
  deterministic local proof review,
  replay rejection,
  mismatch rejection,
  challenge/halt/recovery blockers

rox-anchor-rpc-proof:
  read-only RPC evidence and quorum classification,
  stale/missing/disputed/wrong-binding readback rejection

rox-anchor-coordinator:
  observation intake,
  queue/replay discipline,
  proof handoff,
  accepted/rejected coordinator decisions,
  no direct settlement mutation

rox-anchor-relayer:
  dry-run receipt generation,
  transaction simulation gates,
  capped testnet send authorization model,
  receipt ledger/audit trail,
  no default live submission

rox-anchor-cli:
  operator-visible check/status/proof/pilot/submit/receipt/drill surfaces,
  explicit flags only,
  safe deterministic reporting

programs/rox-anchor:
  Anchor-owned on-chain state transitions,
  mint/burn binding rules,
  halt/recovery/challenge/finalization state rules
```

No crate should create its own competing state machine or safety interpretation. The build plan explicitly says `rox-anchor-core` and `rox-anchor-proof` remain the shared source of truth, the Anchor program owns on-chain state transitions, and real internal ROC mutation must remain behind RustyOnions `svc-wallet -> ron-ledger`. 

## 2.3 Current non-authorization boundary

Do not lose this boundary in the next session.

Still forbidden:

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
production ROC release
real internal ROC release from ROX burn evidence
real user funds
public bridge UI
public faucet
public claim page
exchange-facing behavior
staking
liquidity pools
market making
custody services
unbounded relayer submission
silent wallet/key usage
silent live RPC submission
operator key material committed to repo
raw private keys in logs
fake finality
fake success output
CrabLink display claiming final settlement before backend proof
```

Allowed only within BUILD_PLAN3’s private pilot boundary:

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

BUILD_PLAN3 says any future work needing public users, real ROC movement, production settlement, mainnet, public mint/burn, exchange/liquidity/staking behavior, or public bridge UI must become a separate future build plan. 

---

# 3. Phase Completion Ledger

## BUILD_PLAN3 completed phases

```text
Phase 0  — Freeze BUILD_PLAN2 green baseline: COMPLETE / GREEN / PARKED
Phase 1  — Private pilot operator workspace hygiene: COMPLETE / GREEN / PARKED
Phase 2  — External testnet config loader: COMPLETE / GREEN / PARKED
Phase 3  — Testnet program artifact manifest: COMPLETE / GREEN / PARKED
Phase 4  — Private testnet deployment drill / optional deployment: COMPLETE / GREEN / PARKED
Phase 5  — Test-only mint initialization runbook: COMPLETE / GREEN / PARKED
Phase 6  — Live read-only RPC verification against testnet: COMPLETE / GREEN / PARKED
Phase 7  — Simulation-only pilot transaction plans: COMPLETE / GREEN / PARKED
Phase 8  — Explicit capped private testnet sender: COMPLETE / GREEN / PARKED
Phase 9  — Pilot receipt ledger and audit trail: COMPLETE / GREEN / PARKED
Phase 10 — Pilot CLI command surface: COMPLETE / GREEN / PARKED
Phase 11 — CrabLink / internal ROC dry-run adapter: COMPLETE / GREEN / PARKED
Phase 12 — Private ROC-to-ROX Testnet Pilot Flow: COMPLETE / GREEN / PARKED
Phase 13 — Private ROX-to-ROC Testnet Pilot Flow: COMPLETE / GREEN / PARKED
```

## Remaining BUILD_PLAN3 phases

```text
Phase 14 — Live Testnet Chaos and Incident Drills: NEXT
Phase 15 — Authority, Upgrade, Halt, and Recovery Operational Drills: AFTER PHASE 14
Phase 16 — Private Testnet Pilot Closeout Gate: FINAL PHASE
```

---

# 4. What Phase 13 Now Proves

Phase 13 now proves the reverse pilot shape:

```text
test ROX burn evidence can be observed
wrong mint/token-account evidence is rejected
stale or missing readback is rejected
replay is rejected before release-intent use
halted posture blocks unsafe release simulation
coordinator can accept a dry-run release intent
relayer can produce only a dry-run release-intent simulation
CLI can report the reverse flow without unsafe effects
read-only RPC can verify burn/readback shape
reports explicitly avoid network submission and settlement claims
```

Phase 13 does **not** prove or authorize:

```text
real internal ROC release
direct ron-ledger mutation
direct svc-wallet mutation
paid content unlock
public bridge UI
mainnet deployment
production settlement
exchange-facing behavior
staking
liquidity
```

The reverse pilot document makes this exact boundary explicit and says future real ROC release must be delegated to `svc-wallet -> ron-ledger`. 

---

# 5. Important Final Session Fixes To Remember

## 5.1 Initial failing shape

The earlier failing relayer test expected:

```text
network_submitted: false
```

But `RelayerReceipt` did not have `network_submitted` or `internal_roc_mutation` fields. Its available fields included:

```text
operation_id
idempotency_key
target
status
proof_decision
attempts_used
live_submission
```

So the test had drifted away from the real relayer receipt shape.

## 5.2 First correction

We removed invalid field assumptions from the test and moved toward checking report output rather than nonexistent struct fields.

## 5.3 Second failing shape

The relayer report test then expected:

```text
network_submission: disabled
```

But the actual `PrivatePilotSimulationResult::redacted_report_lines()` output did not yet include that explicit line.

## 5.4 Final correction

We patched `crates/rox-anchor-relayer/src/submit.rs` so the simulation report includes:

```text
network_submission: disabled
settlement_claim: none
```

This is better than simply weakening the test. It strengthened the actual relayer report surface and makes the operator-visible safety report more explicit.

## 5.5 Why this matters

For private pilot work, “safe by implication” is not enough. Reports should say the safety state clearly. Phase 13 is stronger now because the report explicitly says there is no network submission and no settlement claim.

---

# 6. Next Session: Start With Phase 14

## 6.1 Phase 14 purpose

BUILD_PLAN3 Phase 14 is:

```text
Live Testnet Chaos and Incident Drills
```

Purpose:

```text
Prove private testnet operations fail safely under realistic conditions.
```

The build plan’s Phase 14 required drills include RPC outage, RPC disagreement, stale slot/readback evidence, wrong program ID, wrong mint, wrong token account, missing program/config accounts, missing/tampered/duplicate receipts, duplicate operation ID, duplicate idempotency key, nonce reuse, omitted operator approval, wrong authority, halt timing cases, recovery during pending operation, send-disabled cases, cap-exceeded cases, and readback-missing-after-send. 

The exit condition is:

```text
Private testnet chaos drills fail safely, deterministically, and with inspectable receipts.
```

## 6.2 First commands for next session

Start by confirming the Phase 13 green baseline remains intact:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
cargo test --workspace
cargo check --workspace
bash scripts/make_codebundle.sh
```

If Anchor CLI is available and the environment is ready:

```bash
anchor build
anchor test
```

Then start Phase 14 with focused tests rather than broad new docs.

## 6.3 Recommended first Phase 14 patch direction

Do **not** start Phase 14 by making a broad new runbook only. Start by adding or expanding real chaos tests beside existing behavior.

There are already relevant existing surfaces:

```text
crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs
crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs
crates/rox-anchor-coordinator/tests/coordinator_stale_evidence_chaos.rs
crates/rox-anchor-proof/tests/challenge_griefing.rs
crates/rox-anchor-proof/tests/halt_resume.rs
tests/chaos/coordinator_stale_evidence.rs
tests/chaos/rpc_equivocation.rs
tests/chaos/relayer_retry_storm.rs
```

The codebundle confirms these files exist in the current repo. 

Recommended Phase 14 patch shape:

```text
1. Expand RPC-proof chaos coverage.
2. Expand relayer chaos coverage.
3. Add coordinator/relayer incident boundary tests if needed.
4. Add CLI incident/drill report tests only if the report surface needs operator visibility.
5. Add a focused docs/pilot incident note only after tests exist, not before.
```

## 6.4 Best first concrete Phase 14 target

Start with the areas most directly named by Phase 14 and closest to existing crates:

```text
crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs
crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs
```

These already exist, so the first patch can expand them instead of adding new test scaffolding.

Recommended first batch of Phase 14 tests:

```text
RPC outage during read-only proof fails without fabricating agreement.
RPC disagreement during readback remains disputed.
Stale slot/readback evidence remains rejected.
Wrong program ID / wrong mint / wrong token account are binding tamper.
Missing program account does not become accepted evidence.
Simulation passes but send-disabled scope refuses capped submit.
Send enabled but cap exceeded is rejected.
Halt during pending operation blocks simulation and capped submit.
Recovery during pending operation does not bypass halt/recovery rules.
```

Some of these already appear covered; the next patch should inspect existing tests first and fill gaps rather than duplicate.

## 6.5 Phase 14 should not introduce real live RPC submission

Despite the title “Live Testnet Chaos,” Phase 14 can and should begin with deterministic local/fake-adapter tests. Optional manual private testnet drills are allowed only after local tests are green and external config/keys are supplied. The build plan lists manual drills as optional, not the first required action. 

The safe implementation order is:

```text
local fake-adapter incident tests first
focused cargo test
workspace cargo test
then optional manual private testnet drills only when explicitly ready
```

---

# 7. Phase 14 Detailed Work Plan

## 7.1 RPC-proof incident drills

Target:

```text
crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs
crates/rox-anchor-rpc-proof/tests/private_testnet_read_only_rpc.rs
crates/rox-anchor-rpc-proof/src/rpc.rs
crates/rox-anchor-rpc-proof/src/quorum.rs
```

Drills to prove:

```text
RPC outage returns missing evidence, not accepted quorum.
RPC provider disagreement returns disputed evidence, not accepted quorum.
Stale signature/readback evidence is rejected.
Wrong program ID is rejected.
Wrong mint is rejected.
Wrong token account is rejected.
Missing program account is visible and non-accepting.
Missing config account is visible and non-accepting.
Readback missing after send does not claim success.
Reorg-like stale evidence remains rejected.
```

Expected focused command:

```bash
cargo fmt -p rox-anchor-rpc-proof
cargo test -p rox-anchor-rpc-proof --test testnet_chaos_drills
cargo test -p rox-anchor-rpc-proof --test private_testnet_read_only_rpc
```

## 7.2 Relayer incident drills

Target:

```text
crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs
crates/rox-anchor-relayer/tests/capped_testnet_submission.rs
crates/rox-anchor-relayer/tests/private_testnet_sender.rs
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
```

Drills to prove:

```text
simulation passes but send-disabled scope refuses capped submit
send enabled but operation cap exceeded is rejected
send enabled but amount cap exceeded is rejected
operator approval omitted blocks send
missing receipt output path blocks send
duplicate receipt blocks authorization
receipt tamper blocks authorization
halt before simulation blocks simulation
halt after simulation before submit blocks capped submit
halt after capped testnet submit is represented in receipt/posture, not ignored
recovery during pending operation does not bypass blocker rules
retry storms remain bounded
```

Expected focused command:

```bash
cargo fmt -p rox-anchor-relayer
cargo test -p rox-anchor-relayer --test testnet_chaos_drills
cargo test -p rox-anchor-relayer --test capped_testnet_submission
cargo test -p rox-anchor-relayer --test private_testnet_sender
```

## 7.3 Coordinator incident drills

Target:

```text
crates/rox-anchor-coordinator/tests/coordinator_stale_evidence_chaos.rs
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
crates/rox-anchor-coordinator/tests/halt_recovery_finalization_gate.rs
```

Drills to prove:

```text
duplicate operation ID rejected before relayer work
duplicate idempotency key rejected before relayer work
nonce reuse rejected before relayer work
stale evidence rejected before relayer attempts
halted posture blocks coordinator finalization
recovery-required posture blocks coordinator finalization
fresh evidence after stale failure can still accept
replay storm does not poison clean review
```

Expected focused command:

```bash
cargo fmt -p rox-anchor-coordinator
cargo test -p rox-anchor-coordinator --test coordinator_stale_evidence_chaos
cargo test -p rox-anchor-coordinator --test testnet_shadow_flow
cargo test -p rox-anchor-coordinator --test halt_recovery_finalization_gate
```

## 7.4 CLI incident/drill reporting

Only add CLI work if an operator-visible command/report needs to show Phase 14 outcomes.

Target candidates:

```text
crates/rox-anchor-cli/src/commands/drill.rs
crates/rox-anchor-cli/src/commands/pilot.rs
crates/rox-anchor-cli/tests/kill_switch_drill_command.rs
crates/rox-anchor-cli/tests/private_pilot_cli.rs
```

Possible CLI requirements:

```text
incident reports must not claim finality
incident reports must not claim production safety
incident reports must redact secret-looking paths/tokens
unknown or ambiguous incident flags must fail closed
operator approval omitted must be visible
send-disabled and cap-exceeded cases must be visible
halt/recovery state must be visible
```

Expected focused command:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --test kill_switch_drill_command
cargo test -p rox-anchor-cli --test private_pilot_cli
cargo test -p rox-anchor-cli
```

## 7.5 Documentation for Phase 14

Only after the tests exist, add a concise runbook such as:

```text
docs/pilot/PRIVATE_TESTNET_INCIDENT_DRILLS.md
```

It should summarize tested incident cases and manual optional private-testnet drills, but it should not become a decision gate or broad planning document.

It should clearly say:

```text
local deterministic tests are the source of proof
manual private testnet drills are optional
no production/mainnet/public bridge behavior is authorized
no real ROC release is authorized
no success/finality is claimed unless backed by receipts/readback
```

---

# 8. Phase 15 Preview

After Phase 14 is green, move to:

```text
Phase 15 — Authority, Upgrade, Halt, and Recovery Operational Drills
```

Purpose:

```text
Practice the human/operator safety procedures before any broader pilot.
```

Expected work:

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
tests proving no private key exposure
tests proving no production-safety claims
```

Likely files from BUILD_PLAN3:

```text
docs/pilot/PRIVATE_PILOT_AUTHORITY_DRILLS.md
docs/pilot/PRIVATE_PILOT_HALT_RECOVERY_DRILLS.md
crates/rox-anchor-core/tests/private_pilot_authority_drills.rs
crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs
```

Phase 15 should be treated as an operator-safety phase, not a bridge expansion phase.

Do not add new settlement powers here.

Do not add real wallet/key loading.

Do not add production mint/release logic.

## Phase 15 likely tests

```text
upgrade authority is separate and redacted
mint authority is separate and redacted
halt authority can halt only when authorized
recovery authority can recover only from valid halted/recovery state
wrong authority is rejected before any runtime claim
key rotation intent is redacted and non-mutating unless explicitly authorized
halted-system status remains read-only and safe
recovery-from-halt drill returns to safe dry-run readiness only
```

---

# 9. Phase 16 Preview

After Phase 15 is green, move to:

```text
Phase 16 — Private Testnet Pilot Closeout Gate
```

Purpose:

```text
Decide whether ROX Anchor has completed the private testnet pilot and is ready for a separate future production-readiness plan.
```

Expected work:

```text
confirm all local tests pass
confirm all Anchor tests pass
confirm all private pilot checks pass
confirm private pilot receipts exist if manual runs were performed
confirm no key material is tracked
confirm no public launch behavior exists
confirm no mainnet behavior exists
confirm no production settlement behavior exists
confirm no real ROC release behavior exists
confirm no public bridge UI exists
confirm no exchange/staking/liquidity behavior exists
confirm halt/recovery drills were performed or simulated
confirm known pilot failures are documented
confirm any next plan is separate and explicitly scoped
```

Likely files from BUILD_PLAN3:

```text
docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md
scripts/check_private_testnet_pilot_closeout.sh
crates/rox-anchor-cli/tests/private_testnet_pilot_closeout.rs
```

Phase 16 should end with:

```text
ROX Anchor private testnet pilot is complete / green / parked.
A future production-readiness plan may be drafted separately.
No public/mainnet/production behavior is authorized by BUILD_PLAN3.
```

---

# 10. Clippy Status And Next Clippy Point

BUILD_PLAN3’s Clippy checkpoint policy is:

```text
after Phase 4
after Phase 8
after Phase 12
after Phase 16
```

It says to run focused Clippy first and only run workspace Clippy after focused surfaces are green. 

We already crossed Phase 12, and earlier sessions handled that checkpoint. The next required Clippy checkpoint is after Phase 16.

However, if Phase 14 or 15 touches a crate heavily, it is still reasonable to run focused Clippy opportunistically on the touched crates:

```bash
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

Do not run a massive Clippy sweep after every small patch unless the user requests it or the phase boundary calls for it.

---

# 11. Exact Recommended Start Prompt For Next Session

Use this at the start of the next session:

```text
We are continuing ROX Anchor / BUILD_PLAN3.

Current safe status:
BUILD_PLAN.md: complete / green / parked.
BUILD_PLAN2.md: complete / green / parked.
BUILD_PLAN3 phases 0–13: complete / green / parked.
Current phase: BUILD_PLAN3 Phase 14 — Live Testnet Chaos and Incident Drills.

Last green evidence:
cargo test --workspace passed.
Focused Phase 13 tests passed:
- rox-anchor-relayer --test private_rox_to_roc_pilot
- rox-anchor-relayer --test private_pilot_simulation
- rox-anchor-cli --test private_rox_to_roc_command
- rox-anchor-coordinator --test private_rox_to_roc_pilot
- rox-anchor-rpc-proof --test private_rox_to_roc_readback
scripts/make_codebundle.sh passed the secret failsafe scan.

Important boundary:
No public launch, no mainnet, no production bridge settlement, no real ROC release, no public ROX mint/burn, no exchange/staking/liquidity, no silent live RPC submission, no committed keys. BUILD_PLAN3 remains private testnet/test-only, explicit, capped, receipt-backed, and operator-approved only.

Start by confirming the baseline with focused commands, then patch Phase 14 incident/chaos tests. Prefer expanding existing test files:
- crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs
- crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs
- crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
- crates/rox-anchor-cli tests only if operator-visible incident reporting is needed.

Use paste-ready bash patches only.
```

---

# 12. Exact Commands To Run First Next Session

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
cargo test --workspace
cargo check --workspace
bash scripts/make_codebundle.sh
```

If Anchor CLI is available and local environment is ready:

```bash
anchor build
anchor test
```

Then begin Phase 14 focused work:

```bash
cargo test -p rox-anchor-rpc-proof --test testnet_chaos_drills
cargo test -p rox-anchor-relayer --test testnet_chaos_drills
cargo test -p rox-anchor-coordinator --test testnet_shadow_flow
```

---

# 13. High-Value Phase 14 Acceptance Criteria

Phase 14 should not be considered complete until the following are true:

```text
RPC outage cases fail closed.
RPC disagreement cases fail as disputed, not accepted.
Stale readback evidence is rejected.
Wrong program ID is rejected.
Wrong mint is rejected.
Wrong token account is rejected.
Missing program/config accounts do not fabricate evidence.
Missing receipt file is handled safely.
Receipt tamper is rejected.
Duplicate receipt is rejected.
Duplicate operation ID is rejected.
Duplicate idempotency key is rejected.
Nonce reuse is rejected.
Operator approval omitted blocks send.
Wrong authority attempted is rejected.
Halt before simulation blocks simulation.
Halt after simulation before submit blocks submit.
Halt after capped testnet submit is inspectable and does not hide risk.
Recovery during pending operation does not bypass blockers.
Simulation passes but send disabled remains non-submitting.
Send enabled but cap exceeded is rejected.
Readback missing after send does not claim finality.
All reports remain deterministic, redacted, and inspectable.
cargo test --workspace passes.
cargo check --workspace passes.
codebundle generation passes secret failsafe scan.
```

---

# 14. Strategic Interpretation

ROX Anchor is now past the “does the bridge shape exist?” stage for local/private-testnet modeling.

The forward and reverse pilot shapes both exist:

```text
ROC-to-ROX:
  dry-run internal ROC burn intent -> proof/RPC/coordinator/relayer/simulation -> test ROX mint shape only

ROX-to-ROC:
  test ROX burn evidence -> proof/RPC/coordinator/relayer/simulation -> dry-run internal ROC release intent only
```

The next question is not whether the happy path exists. It does.

The next question is whether the private testnet system fails safely under realistic operational incidents. That is exactly Phase 14.

In other words:

```text
Phase 12 and 13 proved controlled pilot flow shape.
Phase 14 must prove incident containment.
Phase 15 must prove operator authority/halt/recovery discipline.
Phase 16 must prove private pilot closeout readiness.
```

This is the right order. We should not jump to production-readiness language, public UI, real ROC release, mainnet, or broad bridge settlement. The remaining work is safety closure, not expansion of authority.

---

# 15. Final Carry-Over Summary

The next session should begin from this statement:

```text
ROX Anchor BUILD_PLAN3 Phase 13 is complete / green / parked.
The repo is workspace-test green.
The reverse ROX-to-ROC private pilot can prove test ROX burn evidence and produce only a dry-run internal ROC release intent.
No real ROC release exists or is authorized.
No public/mainnet/production bridge behavior exists or is authorized.
The next phase is BUILD_PLAN3 Phase 14: Live Testnet Chaos and Incident Drills.
```

Proceed with Phase 14 by expanding deterministic incident tests first, using existing RPC-proof, relayer, coordinator, and CLI surfaces. Keep every patch small, run focused tests, fix the first failure, and only then move to workspace tests.

This is a strong stopping point: Phase 14 is now the next real hardening layer, not another planning loop.


### END NOTE - JULY 5 2026 - 17:45 CST


### BEGIN NOTE - JULY 5 2026 - 19:20 CST

I reviewed the new codebundle, BUILD_PLAN3, and IDB. The codebundle was generated cleanly with **211 files**, includes the Phase 15 test file, and repeats the non-authorization boundary for runtime, Solana runtime, bridge runtime, staking, liquidity, exchange-facing logic, and external settlement.  BUILD_PLAN3 confirms we are at the end of the private testnet pilot plan, with Phase 15 followed only by Phase 16 closeout.  The IDB still anchors the core rules: real Rust behavior, focused tests, compile checks, core/proof as shared truth, no duplicate state machines, and no hidden live behavior. 

# ROX Anchor Carry-Over Notes — BUILD_PLAN3 Final Stretch

## Current Session Status

We are continuing work on **RustyOnions / CrabLink / ROX Anchor**, specifically **BUILD_PLAN3 — Private Testnet Pilot / Controlled Live-Testnet Operations**.

The project is now in the final stretch of BUILD_PLAN3.

BUILD_PLAN3 has **17 total phases**, counted from Phase 0 through Phase 16.

Current phase:

```text
BUILD_PLAN3 Phase 15 — Authority, Upgrade, Halt, and Recovery Operational Drills
```

Current Phase 15 status:

```text
Focused CLI implementation: GREEN
Focused CLI test: GREEN
Focused CLI compile check: GREEN
```

The latest terminal output confirms:

```text
cargo test -p rox-anchor-cli --test private_pilot_drill_reports
Result: 3 passed / 0 failed

cargo run -p rox-anchor-cli -- drill phase15
Result: rendered local-only authority drill report

cargo check -p rox-anchor-cli
Result: green
```

The latest codebundle was generated successfully after the Phase 15 fix:

```text
Generated by scripts/make_codebundle.sh on 2026-07-06T00:07:51Z
Root: /Users/mymac/Desktop/rox-anchor
Git: 8a631f3 (dirty)
Files: 211
Skipped files: none
Secret failsafe bundle generation: passed
```

The codebundle’s non-authorization notice remains intact:

```text
This codebundle does not authorize ROX runtime, Solana runtime, bridge runtime,
staking, liquidity, exchange-facing logic, or external settlement.
```

That boundary remains critical.

---

## What We Accomplished In This Session

### 1. Advanced through late BUILD_PLAN3 Phase 14 into Phase 15

The prior state had Phase 14 nearly complete with local incident drills. This session pushed the project beyond the Phase 14 chaos/incident surface and began Phase 15.

Phase 14 had already covered the live-testnet chaos / incident drill surface, including:

```text
RPC disagreement during readback
stale readback
missing readback after send
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

The previous Phase 14 CLI report was expanded to a broader local incident drill surface. That kept the project inside local/report-only behavior and did not create runtime settlement claims.

### 2. Added the Phase 15 CLI drill command

We added a new CLI drill surface:

```text
rox-anchor drill phase15
```

Purpose:

```text
Authority, upgrade, halt, recovery, wrong-authority, key-rotation,
halted-status, and recovery-from-halt operational drills.
```

This command produces a local-only operator-readable report. It is intentionally not a live execution command.

It reports:

```text
rox-anchor phase15 authority drill
command: drill phase15
scope: authority_upgrade_halt_recovery_operational_drills
source: rox-anchor-core authority map review
source: rox-anchor-core kill-switch review
mode: local_report_only
```

It also explicitly reports that unsafe runtime behavior is disabled:

```text
runtime: disabled
wallet_key_loading: disabled
private_key_material: not_loaded
rpc_submission: disabled
signing: disabled
mint_burn_execution: disabled
internal_roc_mutation: disabled
production_safety_claim: none
settlement_claim: none
public_bridge_authorization: none
```

This is exactly the right posture for Phase 15.

### 3. Added Phase 15 operator authority drill output

The Phase 15 report now includes:

```text
drill_count: 8
strict_authority_separation_valid: true
explicit_test_only_shared_authority_valid: true
```

It checks and displays authority/operator role posture:

```text
operator_role: observer critical=false
operator_role: coordinator critical=false
operator_role: relayer critical=false
operator_role: upgrade_authority critical=true
operator_role: mint_authority critical=true
operator_role: halt_authority critical=true
operator_role: recovery_authority critical=true
```

The report includes the Phase 15 checklists:

```text
upgrade_authority_checklist: external_key_required
mint_authority_checklist: separated_or_explicit_test_only
halt_authority_checklist: can_block_acceptance_simulation_submission_finalization
recovery_authority_checklist: can_only_recover_from_halted_recovery_required_state
```

This makes the report useful for an operator without creating any real runtime power.

### 4. Added wrong-authority rejection drill

The report now includes:

```text
phase15_wrong_authority_rejection_drill: local_only
status: WrongAuthority
accepted: false
action_permitted: false
blocks_submission: false
private_key_material: not_loaded
transaction_submission: not_performed
settlement_claim: none
```

This proves that the wrong authority cannot operate the kill-switch path.

Important: this is local review only. It does not load a keypair or sign anything.

### 5. Added halted-system read-only status drill

The report now includes:

```text
phase15_halted_system_read_only_status_drill: local_only
status: HaltAccepted
accepted: true
action_permitted: true
blocks_acceptance: true
blocks_simulation: true
blocks_submission: true
blocks_finalization: true
read_only_status: available
rpc_submission: disabled
settlement_claim: none
```

This is important because it proves the desired operational posture:

```text
A halt should block unsafe progress, but operators should still be able to inspect read-only status.
```

This aligns with the fortress model.

### 6. Added recovery-from-halt drill

The report now includes:

```text
phase15_recovery_from_halt_drill: local_only
status: RecoveryAccepted
accepted: true
action_permitted: true
blocks_acceptance: false
blocks_simulation: false
blocks_submission: false
blocks_finalization: false
post_recovery_runtime_claim: none
transaction_submission: not_performed
settlement_claim: none
```

This proves that recovery can be reviewed from a halted/recovery-required posture without claiming that real runtime activity resumed.

Important distinction:

```text
The local review says recovery is accepted.
It does not claim runtime recovery was executed.
It does not submit RPC.
It does not settle.
```

### 7. Added key-rotation intent drill

The report now includes:

```text
phase15_key_rotation_intent_drill: local_only
rotation_scope: intent_only
old_upgrade_authority: redacted_authority_key
new_upgrade_authority: redacted_authority_key
old_mint_authority: redacted_authority_key
new_mint_authority: redacted_authority_key
old_halt_authority: redacted_authority_key
new_halt_authority: redacted_authority_key
old_recovery_authority: redacted_authority_key
new_recovery_authority: redacted_authority_key
private_key_material: not_loaded
keypair_path: redacted
signing: disabled
upgrade_execution: not_performed
production_safety_claim: none
```

This is the right Phase 15 shape because it models operator key-rotation intent without performing key rotation, loading keypairs, signing, or claiming an upgrade occurred.

### 8. Added Phase 15 CLI tests

A new test file exists:

```text
crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs
```

It includes three focused tests:

```text
phase15_authority_drill_report_is_operator_readable_and_local_only
phase15_authority_drill_report_does_not_leak_or_claim_runtime_power
drill_help_mentions_phase15_authority_drills
```

These tests passed:

```text
running 3 tests
test drill_help_mentions_phase15_authority_drills ... ok
test phase15_authority_drill_report_is_operator_readable_and_local_only ... ok
test phase15_authority_drill_report_does_not_leak_or_claim_runtime_power ... ok

test result: ok. 3 passed; 0 failed
```

The tests prove that Phase 15 output includes the expected local-only operator details and does not leak or claim unsafe runtime behavior.

### 9. Fixed the first compiler failure immediately

The first Phase 15 compiler failure was:

```text
error[E0631]: type mismatch in function arguments
.map(str::to_string)
```

Cause:

```text
The vector already contained owned String values.
The code then attempted to map str::to_string over String items.
```

Fix applied:

```rust
]
```

instead of:

```rust
]
.into_iter()
.map(str::to_string)
.collect()
```

After the fix:

```text
cargo test -p rox-anchor-cli --test private_pilot_drill_reports
cargo run -p rox-anchor-cli -- drill phase15
cargo check -p rox-anchor-cli
```

all passed.

This followed the correct QuickChain rule:

```text
fix the first compiler failure before expanding scope.
```

---

## Current Project Position

We are now here:

```text
BUILD_PLAN3 Phase 15 — focused CLI green
```

By phase count:

```text
Phase 15 is the 16th of 17 phases.
Only Phase 16 remains after Phase 15 is fully closed.
```

Estimated completion:

```text
BUILD_PLAN3 completion: approximately 94–96%
Overall ROX Anchor private-testnet implementation track: approximately 97–98%
```

This estimate assumes:

```text
No hidden workspace-wide failures
No Anchor build/test surprise
No final Clippy surprise
No secret-scan issue in final closeout
```

Phase 15 is not fully parked until the next session confirms the remaining focused Phase 15 surfaces and/or decides whether a core-local authority drill test is needed before Phase 16.

---

## Important Invariants To Preserve

The IDB remains the source of truth for how we should finish.

The most important invariants are:

```text
rox-anchor-core owns shared domain truth
rox-anchor-proof owns deterministic local review
replay must be rejected
binding mismatches must be rejected
challenge/halt/recovery must block unsafe transitions
CLI must expose real local behavior, not fake success
coordinator/rpc-proof/relayer must use core/proof rules
Anchor program owns on-chain transitions
mint/burn behavior must be state-machine protected
tests must grow alongside code
no hidden live behavior
```

Do not violate these.

The remaining work must not introduce:

```text
live wallet calls
silent RPC submission
silent key usage
production minting
production burning
production settlement
mainnet behavior
public launch behavior
exchange-facing behavior
staking
liquidity
fake finality
fake success output
duplicate state machines
client-side authority over transitions
```

---

## Current BUILD_PLAN3 Boundary

BUILD_PLAN3 explicitly does **not** authorize:

```text
public launch
mainnet deployment
public ROX mint/burn
production bridge settlement
production ROC release
exchange-facing behavior
staking
liquidity
real user value movement
```

BUILD_PLAN3 allows only controlled private testnet work:

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

The future real internal ROC path remains:

```text
svc-wallet -> ron-ledger
```

ROX Anchor must not directly issue or release real ROC.

---

## What To Do First In The Next Session

Start the next session by pasting the latest codebundle, BUILD_PLAN3, IDB, and these notes.

The first task should be to confirm the Phase 15 patch state and run the focused Phase 15 commands again if needed:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --test private_pilot_drill_reports
cargo run -p rox-anchor-cli -- drill phase15
cargo check -p rox-anchor-cli
```

Expected result:

```text
private_pilot_drill_reports: 3 passed / 0 failed
drill phase15 renders local-only authority drill report
cargo check -p rox-anchor-cli: green
```

Then decide whether to finish Phase 15 with a small additional core-side test and/or docs, because BUILD_PLAN3 Phase 15 lists likely files:

```text
docs/pilot/PRIVATE_PILOT_AUTHORITY_DRILLS.md
docs/pilot/PRIVATE_PILOT_HALT_RECOVERY_DRILLS.md
crates/rox-anchor-core/tests/private_pilot_authority_drills.rs
crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs
```

We already have:

```text
crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs
```

We do not yet appear to have:

```text
crates/rox-anchor-core/tests/private_pilot_authority_drills.rs
docs/pilot/PRIVATE_PILOT_AUTHORITY_DRILLS.md
docs/pilot/PRIVATE_PILOT_HALT_RECOVERY_DRILLS.md
```

Recommended next patch:

```text
Finish Phase 15 with a focused rox-anchor-core authority drill test.
```

That test should verify the core behavior directly, not just CLI wording.

Suggested test coverage:

```text
strict authority separation validates
explicit test-only shared authority validates
wrong halt authority is rejected
halt authority can halt and block unsafe stages
recovery authority can recover from halted/recovery-required posture
recovery is rejected or blocked when posture is not recoverable
critical roles are classified correctly
no private key material is represented in core authority IDs
```

Suggested command after that patch:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
cargo test -p rox-anchor-core --test private_pilot_authority_drills
cargo test -p rox-anchor-cli --test private_pilot_drill_reports
cargo check -p rox-anchor-core
cargo check -p rox-anchor-cli
```

If that is green, Phase 15 can be treated as functionally complete.

Optional but likely useful before Phase 16:

```text
Add minimal docs/pilot/PRIVATE_PILOT_AUTHORITY_DRILLS.md
Add minimal docs/pilot/PRIVATE_PILOT_HALT_RECOVERY_DRILLS.md
```

Only do this if they are useful and tied to real command/test behavior. Do not create broad planning docs. These should be operator runbooks backed by the actual `rox-anchor drill phase15` output.

---

## Recommended Phase 15 Exit Checklist

Before marking Phase 15 complete / green / parked, confirm:

```text
cargo test -p rox-anchor-cli --test private_pilot_drill_reports passes
cargo run -p rox-anchor-cli -- drill phase15 renders expected report
cargo check -p rox-anchor-cli passes
cargo test -p rox-anchor-core --test private_pilot_authority_drills passes, if added
cargo check -p rox-anchor-core passes
Phase 15 output is local-only
Phase 15 output says private_key_material: not_loaded
Phase 15 output says wallet_key_loading: disabled
Phase 15 output says rpc_submission: disabled
Phase 15 output says signing: disabled
Phase 15 output says mint_burn_execution: disabled
Phase 15 output says internal_roc_mutation: disabled
Phase 15 output says production_safety_claim: none
Phase 15 output says settlement_claim: none
Phase 15 output says public_bridge_authorization: none
No code path loads keys
No code path signs
No code path submits RPC
No code path mutates ROC
No code path claims production safety
```

Then move to Phase 16.

---

## Phase 16 — What Remains

Phase 16 is:

```text
Private Testnet Pilot Closeout Gate
```

Purpose:

```text
Decide whether ROX Anchor has completed the private testnet pilot and is ready
for a separate future production-readiness plan.
```

This is the final phase of BUILD_PLAN3.

Phase 16 work items:

```text
1. Confirm all local tests pass.
2. Confirm all Anchor tests pass.
3. Confirm all private pilot checks pass.
4. Confirm private pilot receipts exist for manual runs, if manual runs were performed.
5. Confirm no key material is tracked.
6. Confirm no public launch behavior exists.
7. Confirm no mainnet behavior exists.
8. Confirm no production settlement behavior exists.
9. Confirm no real ROC release behavior exists.
10. Confirm no public bridge UI exists.
11. Confirm no exchange/staking/liquidity behavior exists.
12. Confirm halt/recovery drills were performed or simulated.
13. Confirm known pilot failures are documented.
14. Confirm the next plan, if any, is separate and explicitly scoped.
```

Likely Phase 16 files:

```text
docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md
scripts/check_private_testnet_pilot_closeout.sh
crates/rox-anchor-cli/tests/private_testnet_pilot_closeout.rs
```

Recommended Phase 16 implementation style:

```text
Small real checker script
Focused CLI/script test
No broad closeout theater
No fake production readiness claim
No public-launch wording
No mainnet permission
No settlement claim
```

The closeout checker should be concrete and behavior-based.

Suggested Phase 16 checker responsibilities:

```text
confirm required scripts exist
confirm required pilot docs exist
confirm required audit docs exist
confirm required tests exist
confirm no tracked private pilot key patterns appear
confirm BUILD_PLAN3 boundaries remain present
confirm codebundle non-authorization notice remains present after generation
confirm public/mainnet/production/staking/liquidity/exchange paths are not authorized
confirm Phase 14 and Phase 15 drill command tests exist
confirm Phase 16 closeout test itself exists
```

Suggested Phase 16 CLI test should prove:

```text
closeout checker rejects missing required files
closeout checker rejects unsafe production/mainnet/public-launch phrases if they appear in active source/docs
closeout checker accepts the current repo layout when all expected artifacts exist
closeout output remains non-authorizing
```

Keep the closeout checker deterministic.

Avoid any network calls.

Avoid any wallet calls.

Avoid any RPC submission.

Avoid any secret reading.

---

## Recommended Phase 16 Command Sequence

After the Phase 16 patch, run:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test -p rox-anchor-cli --test private_testnet_pilot_closeout
cargo test -p rox-anchor-cli
cargo check -p rox-anchor-cli
```

Then run a broader final sweep:

```bash
cargo test --workspace
cargo check --workspace
```

If Anchor CLI is installed and previous Anchor commands are expected to pass:

```bash
anchor build
anchor test
```

Then run the final codebundle:

```bash
bash scripts/make_codebundle.sh
```

Then run the final Clippy checkpoint required by BUILD_PLAN3 after Phase 16:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Only run workspace Clippy if the focused Clippy passes are green.

---

## Expected End State After Phase 16

BUILD_PLAN3 complete / green / parked means:

```text
ROX Anchor private testnet pilot is complete / green / parked.
Private testnet deployment and/or deployment drill is proven.
Read-only RPC verification is proven.
Simulation-only flow is proven.
Strictly capped private testnet submission is proven if manually executed.
Pilot receipts and audit trail are inspectable before and during pilot flows.
Pilot CLI command surface is safe and explicit.
CrabLink/internal ROC dry-run adapter shape is ready for future integration.
ROC-to-ROX private pilot test flow is proven with test-only assets.
ROX-to-ROC private pilot test flow is proven without real ROC release.
Halt/recovery/authority drills are proven.
```

BUILD_PLAN3 complete does **not** mean:

```text
public launch
mainnet launch
production bridge settlement
public ROX mint/burn
real internal ROC release
public bridge UI
exchange readiness
staking readiness
liquidity readiness
```

Those require a separate future plan.

---

## Suggested Opening Prompt For The Next Session

Use this to start the new session:

```text
We are continuing RustyOnions / CrabLink / ROX Anchor.

Current status:
BUILD_PLAN3 is almost complete. We are at Phase 15 of 16 by numbering, meaning the 16th of 17 total phases.

Latest green terminal output:
cargo test -p rox-anchor-cli --test private_pilot_drill_reports passed 3/3.
cargo run -p rox-anchor-cli -- drill phase15 rendered the local-only authority drill report.
cargo check -p rox-anchor-cli passed.

Latest codebundle:
Generated 2026-07-06T00:07:51Z.
211 files.
Secret-safe codebundle generation passed.
Non-authorization notice remains intact.

Current task:
Finish Phase 15 if needed with a focused rox-anchor-core private pilot authority drill test and any minimal operator docs tied to real behavior. Then move into Phase 16 — Private Testnet Pilot Closeout Gate.

Rules:
Use QuickChain-style patches: real Rust/code/script behavior, focused tests, cargo fmt, focused cargo test/check, fix first failure before expanding.
Use paste-ready terminal patches in bash fences.
Do not add broad planning docs or placeholder-only scaffolding.
Do not include git add/commit/push.
No live wallet calls.
No live ledger mutation.
No live RPC submission unless explicitly requested.
No production deployment.
No production minting/burning.
No bridge settlement.
No staking/liquidity/exchange-facing behavior.
No fake finality or fake success output.

Start by inspecting the latest codebundle and BUILD_PLAN3, then propose the next patch.
```

---

## Most Important Reminder

We are close enough that the biggest risk is accidentally overbuilding or crossing scope.

The next session should not start a new subsystem.

It should only:

```text
1. Finish/park Phase 15 cleanly.
2. Implement Phase 16 closeout gate.
3. Run focused tests.
4. Run final workspace/check/Anchor/Clippy/codebundle sweeps as appropriate.
5. Mark BUILD_PLAN3 complete / green / parked.
```

The final posture must remain:

```text
private testnet pilot complete
public launch unauthorized
mainnet unauthorized
production bridge unauthorized
real ROC release unauthorized
staking/liquidity/exchange unauthorized
```

That should give the next session enough context to finish BUILD_PLAN3 without wandering back into earlier phases.


### END NOTE - JULY 5 2026 - 19:20 CST


### BEGIN NOTE - JULY 5 2026 - 19:55 CST

Here is the full handoff set. I’ve written it so it can be pasted into a new session as the authoritative carry-over note.

# ROX Anchor Carry-Over Notes — BUILD_PLAN3 Complete / Green / Parked

## 1. Current Project Status

We are working on **RustyOnions / CrabLink / ROX Anchor**.

The active implementation surface is:

```text
ROX Anchor
Rust + Solana/Anchor implementation surface for the ROC ↔ ROX anchor path
```

The current active build plan was:

```text
BUILD_PLAN3.md
Private Testnet Pilot / Controlled Live-Testnet Operations
```

As of the latest terminal output and regenerated codebundle, **BUILD_PLAN3 is complete / green / parked**.

Final status:

```text
BUILD_PLAN.md: complete / green / parked
BUILD_PLAN2.md: complete / green / parked
BUILD_PLAN3.md: complete / green / parked

ROX Anchor private testnet pilot: complete / green / parked
```

The latest generated codebundle reports:

```text
Generated by scripts/make_codebundle.sh on 2026-07-06T00:41:09Z
Root: /Users/mymac/Desktop/rox-anchor
Git: 8a631f3 (dirty)
Files: 214
Skipped files: none
```

The codebundle also repeats the critical non-authorization boundary: it does not authorize ROX runtime, Solana runtime, bridge runtime, staking, liquidity, exchange-facing logic, or external settlement. 

## 2. Final Green Evidence

The final command batch for BUILD_PLAN3 included:

```bash
cargo fmt --all
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

The Phase 16 closeout checker passed and confirmed:

```text
BUILD_PLAN3 Phase 16 private testnet pilot closeout checks passed
closeout doc is present
closeout completion remains conditional on local green commands
required pilot docs and scripts are present
tracked-file scan found no key-shaped files
no public/mainnet/production/ROC-release/staking/liquidity/exchange authorization is present
the script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, sign, or load a wallet
```

That evidence appears directly in the latest terminal output. 

The final focused Clippy checkpoint also passed for all required packages:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

The terminal tail shows each focused Clippy command finishing green, including the Anchor program crate. 

## 3. Required Qualifier

BUILD_PLAN3 completion does **not** authorize production or public behavior.

Required qualifier:

```text
No public launch, mainnet launch, production bridge settlement, public ROX mint/burn, real internal ROC release, public bridge UI, exchange readiness, staking readiness, or liquidity readiness is authorized.
```

BUILD_PLAN3 itself says successful completion means the private testnet pilot is complete / green / parked, but explicitly does **not** mean public launch, mainnet launch, production bridge settlement, public ROX mint/burn, real internal ROC release, public bridge UI, exchange readiness, staking readiness, or liquidity readiness. 

This qualifier must carry into every future session.

## 4. What Was Completed in BUILD_PLAN3

BUILD_PLAN3 moved ROX Anchor from testnet-readiness into a controlled private testnet pilot posture.

It completed the following phase ladder:

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

The latest codebundle includes the Phase 16 files:

```text
docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md
scripts/check_private_testnet_pilot_closeout.sh
crates/rox-anchor-cli/tests/private_testnet_pilot_closeout.rs
```

The codebundle file list confirms those files are now present in the tracked/unignored source snapshot. 

## 5. Phase 16 Closeout Work Completed

The final Phase 16 patch added a concrete closeout gate rather than a fake “done” string.

New file:

```text
docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md
```

Purpose:

```text
Records the BUILD_PLAN3 Phase 16 private testnet pilot closeout gate.
Keeps completion conditional on local green commands.
States no public/mainnet/production behavior is authorized.
```

New file:

```text
scripts/check_private_testnet_pilot_closeout.sh
```

Purpose:

```text
Checks BUILD_PLAN3 Phase 16 markers.
Checks required pilot docs and scripts.
Checks required prior pilot artifacts.
Checks closeout doc non-authorization language.
Rejects unsafe authorization phrases.
Scans git-tracked files for key-shaped/private authority-shaped files.
Does not deploy, submit, mint, burn, settle, call RPC, mutate ROC, sign, or load a wallet.
```

The codebundle includes the checker script and its checklist, including the local gate and final Clippy checkpoint commands. 

New file:

```text
crates/rox-anchor-cli/tests/private_testnet_pilot_closeout.rs
```

Purpose:

```text
Tests that the closeout checker accepts the current repo safety shape.
Tests that the checklist is non-launching and operator safe.
Tests that the closeout doc is conditional and non-authorizing.
```

The test file is present in the latest codebundle. 

Focused Phase 16 result:

```text
cargo test -p rox-anchor-cli --test private_testnet_pilot_closeout
3 passed / 0 failed
```

## 6. What the Current Codebase Proves

The current implementation proves the following at a local/test/private-pilot level.

### Core shared type and invariant foundation

`rox-anchor-core` owns the shared type/state foundation. It contains the common domain/state/binding vocabulary that other crates rely on.

It has tests proving:

```text
identifier validation
safe label determinism
lifecycle blocker classification
challenge/halt/recovery posture helpers
testnet scope locks
private pilot config validation
test-only asset harness behavior
test-only mint initialization
operator authority model
kill-switch drills
internal ROC dry-run shapes
testnet program manifest validation
```

Important invariant:

```text
Other crates must not invent duplicate state machines, duplicate labels, or duplicate acceptance rules.
```

### Deterministic proof engine

`rox-anchor-proof` owns deterministic local proof review.

It proves:

```text
valid packages are accepted
binding mismatches are rejected in deterministic order
missing/disputed evidence blocks acceptance
replayed operation ID, idempotency key, or nonce is rejected
challenge, halt, and recovery postures block acceptance
halt/resume behavior remains deterministic
challenge griefing cases are bounded and deterministic
```

Important invariant:

```text
Replay must be rejected.
Domain, direction, cluster, program ID, mint, token account, operation ID, idempotency key, and nonce mismatches must be rejected.
Challenge, halt, and recovery states must block unsafe acceptance/finalization.
```

### CLI safety and inspection surface

`rox-anchor-cli` is now a real local/operator inspection surface.

It supports safe/reporting-oriented command surfaces including:

```text
check
proof
audit
audit-relayer
audit-posture
status
pilot
receipts
submit
submit-capped
drill
halt
recover
```

It also has private pilot surfaces for:

```text
pilot status
pilot read-only proof
pilot simulate
pilot roc-to-rox
pilot rox-to-roc
pilot submit-capped
pilot receipts
pilot drill
pilot halt
pilot recover
```

The CLI tests prove:

```text
unknown commands fail closed
unknown/ambiguous pilot flags fail closed
help output does not expose launch/mainnet/public modes
capped submit requires explicit approval
safe reports do not claim runtime authority or settlement
status uses core safe labels
receipt display is deterministic and non-authorizing
Phase 14/15 drill reports are local-only and non-authorizing
Phase 16 closeout is conditional and non-authorizing
```

### RPC proof model

`rox-anchor-rpc-proof` models read-only RPC evidence without live submission by default.

It proves:

```text
matching RPC observations can agree and map into proof evidence
stale observations are rejected
too few matching sources produce missing evidence
conflicting signatures produce disputed evidence
binding mismatches are rejected
sensitive signatures/paths are redacted
private pilot config validates and redacts
testnet scope locks reject unsafe clusters/modes
testnet chaos drills fail safely
read-only RPC adapter remains read-only
```

Important invariant:

```text
Read-only RPC verification is evidence only. It must not become submission, finality, or settlement.
```

### Coordinator model

`rox-anchor-coordinator` assembles local observations and gates decisions through core/proof behavior.

It proves:

```text
valid observations hand off to proof and accept
stale RPC evidence prevents acceptance
duplicate operation queue entries are rejected
queue capacity is enforced
redacted reports preserve decision shape
authority-aware readiness validates separated roles
coordinator rejects unsafe shared critical authority
coordinator refuses finalization while halted/recovery-required
local nonvalue ROC-to-ROX and ROX-to-ROC flows reach dry-run receipts
private pilot forward/reverse flow reviews are accepted only through proper gates
testnet shadow flow rejects replay, mismatches, stale evidence, under-quorum evidence, and blockers
incident drill reports do not promote rejected decisions
```

Important invariant:

```text
Coordinator calls into core/proof. It must not invent separate acceptance rules.
```

### Relayer model

`rox-anchor-relayer` models dry-run, retry, capped authorization, receipts, and submit-readiness without unsafe live defaults.

It proves:

```text
accepted proof review produces dry-run receipt
blocked proof review does not submit
idempotency replay is rejected
retry policy is bounded
receipt reports are redacted
capped submit requires every gate
operator approval is required
receipt persistence is required where configured
retry, operation, and amount caps are enforced
simulation cannot bypass proof/coordinator/relayer/read-only gates
private testnet sender rejects missing external config
private testnet sender rejects unsafe modes
private ROC-to-ROX pilot authorizes only test ROX capped path without sending
private ROX-to-ROC pilot produces only dry-run release intent
receipt ledger rejects duplicate IDs, mismatched operation IDs, live submission claims without send, and production settlement claims
```

Important invariant:

```text
The relayer may model or authorize a private testnet capped path only after explicit gates. It must not silently submit, settle, or claim finality.
```

### Anchor program surface

The Anchor program crate is compile/test/Clippy green.

The program surface includes:

```text
initialize
observe_burn
finalize
open_challenge
resolve_challenge
halt
recover
state
events
errors
```

Program tests prove:

```text
state transition rules
challenge blocking
halt/recovery blocking
wrong authority rejection
mint authority binding
ROX mint binding
direction-specific helpers
operation PDA derivation
mint authority PDA derivation
token account constraints
token CPI readiness/execution receipt shapes
finalize plan/event tamper rejection
test-only mint harness validation
test-only asset caps and public/production label rejection
```

Important invariant:

```text
The Anchor program owns on-chain state transitions. Other crates model, inspect, prove, coordinate, or dry-run; they do not replace on-chain state rules.
```

## 7. Manual / Live Behavior Status

The current completion is private-pilot complete/green/parked in the repo/test sense.

Allowed/proven:

```text
private testnet deployment drill
testnet/devnet/localnet scoped config and manifests
external key expectations
read-only RPC verification model
simulation-only pilot transaction plans
strictly capped private testnet send authorization model
receipt/audit trail model
CrabLink/internal ROC dry-run adapter shape
private ROC-to-ROX test flow with test-only assets
private ROX-to-ROC test flow without real ROC release
halt/recovery/authority drills
closeout gate
```

Not authorized:

```text
mainnet-beta deployment
public token launch
public ROX minting
public ROX burning
production bridge settlement
real internal ROC release
real user funds
public bridge UI
public faucet
public claim page
exchange-facing behavior
staking
liquidity pools
market making
custody services
silent wallet/key usage
silent live RPC submission
operator key material committed to repo
raw private keys in logs
fake finality
fake success output
CrabLink display claiming final settlement before backend proof
```

The codebundle non-authorization notice must remain intact in future bundles. 

## 8. Important Design Boundaries to Preserve

### Core/proof authority boundary

Preserve this rule:

```text
rox-anchor-core owns shared types, labels, states, direction/domain vocabulary, and reusable invariant helpers.
rox-anchor-proof owns deterministic local proof review.
```

Do not let these crates be bypassed.

### No duplicate acceptance rules

Coordinator, RPC proof, relayer, CLI, and future CrabLink integration must call into core/proof behavior. They must not create alternate “accepted”, “finalized”, “safe”, or “settled” state machines.

### Replay rejection is mandatory

Replay rejection must remain central:

```text
operation ID replay rejected
idempotency key replay rejected
nonce replay rejected
receipt replay rejected
duplicate operation/retry storms bounded
```

### Binding mismatch rejection is mandatory

Reject mismatches in:

```text
domain
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
receipt operation binding
simulation result binding
```

### Operational blockers are mandatory

Unsafe acceptance/finalization/submission must be blocked by:

```text
open challenge
accepted challenge
halt
recovery-required posture
wrong authority
stale evidence
missing evidence
RPC disagreement
receipt tamper
operator approval missing
cap exceeded
unsafe config mode
mainnet-beta cluster
public/production labels
```

### Real ROC boundary

Real internal ROC must never be directly mutated by ROX Anchor.

The future real ROC path, if authorized later, must remain:

```text
svc-wallet -> ron-ledger
```

ROX Anchor may produce evidence, status, receipts, and dry-run/release-intent shapes, but not direct internal ROC issuance/release.

### Live network boundary

Default behavior must remain non-submitting.

Any future live path must be:

```text
explicit
private/testnet/devnet scoped
capped
receipt-backed
operator-approved
simulation-backed
read-only proof-backed
impossible from default config
mainnet-beta rejecting
secret-redacted
```

## 9. Known Good Final Commands

The final successful gate was effectively:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Then:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Then:

```bash
bash scripts/make_codebundle.sh
```

Result:

```text
safe codebundle written
final focused Clippy checkpoint green
BUILD_PLAN3 complete / green / parked
```

## 10. Git / Repo Status

Latest codebundle reports:

```text
Git: 8a631f3 (dirty)
Files: 214
```

That means the repo has uncommitted changes relative to commit `8a631f3`.

Recommended next action before starting a new major plan:

```bash
git status --short
```

Then commit if desired.

Suggested commit message:

```text
Complete ROX Anchor private testnet pilot gate
```

Very short 7-word version:

```text
Complete ROX Anchor private pilot closeout gate
```

Do not add push commands unless explicitly requested.

## 11. Recommended Next Step

The next step should **not** be more BUILD_PLAN3 patching.

BUILD_PLAN3 is done.

Recommended next move:

```text
Start a separate future plan.
```

Possible name:

```text
BUILD_PLAN4.md
Production-Readiness / External Review / Controlled Release Preparation
```

But it must remain explicitly scoped and must not silently authorize public launch or mainnet behavior.

The next plan should be about readiness for a future decision, not production activation.

Recommended posture:

```text
BUILD_PLAN4 should be an audit/security/release-readiness plan first.
It should not jump straight to mainnet.
It should not add public bridge UI.
It should not add real ROC release.
It should not add public ROX mint/burn.
It should not add exchange, staking, or liquidity.
```

## 12. Recommended BUILD_PLAN4 Scope

A safe next plan could be:

```text
ROX Anchor Build Plan 4
External Review / Production-Readiness Hardening / Release Decision Prep
```

Allowed focus areas:

```text
formal audit prep
threat model expansion
invariant map finalization
state transition review
security review checklist
fuzz/property tests
program account sizing review
PDA/seed review
authority separation review
upgrade authority review
key rotation review
incident response playbooks
deployment rollback drills
observability/reporting hardening
CrabLink UI status wording hardening
RustyOnions wallet/ledger handoff design review
manual private testnet receipt review
operator procedure review
configuration hardening
secret scanning hardening
CI hardening
release artifact verification
```

Still forbidden unless explicitly authorized in a separate later decision:

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
real internal ROC release
public bridge UI
public faucet
public claim page
exchange-facing behavior
staking
liquidity pools
custody services
market making
real user funds
```

## 13. Suggested BUILD_PLAN4 Phase Outline

A reasonable next plan could be:

```text
Phase 0  — Freeze BUILD_PLAN3 final green baseline
Phase 1  — Formal invariant inventory and test traceability
Phase 2  — External audit package cleanup
Phase 3  — Anchor account sizing and PDA/seed review
Phase 4  — Authority, upgrade, halt, and recovery review
Phase 5  — Fuzz/property/adversarial regression expansion
Phase 6  — Receipt/audit trail tamper-hardening review
Phase 7  — Config/secret/CI hardening
Phase 8  — CrabLink display/status wording boundary
Phase 9  — RustyOnions wallet/ledger handoff doctrine
Phase 10 — Manual private testnet evidence review package
Phase 11 — Production-readiness decision gate
Phase 12 — Final green run / park
```

Exit condition should be:

```text
ROX Anchor is ready for external review and a future release decision.
No public/mainnet/production behavior is authorized by this plan.
```

## 14. First Command for the Next Session

Start the next session with a baseline confirmation, not a patch.

Recommended command:

```bash
cd /Users/mymac/Desktop/rox-anchor

git status --short

cargo fmt --all
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Then, if still green, start drafting the next explicitly scoped plan.

If we begin with a new plan file, keep it tight and implementation-oriented. Do not turn the project into scaffold theater.

## 15. What Not To Do Next

Do not keep adding Phase 16 “closeout” embellishments.

Do not add fake success strings.

Do not add production language.

Do not add public launch language.

Do not add mainnet config.

Do not add hidden live RPC calls.

Do not add wallet/key loading by default.

Do not add direct `svc-wallet` or `ron-ledger` mutation from ROX Anchor.

Do not add public bridge UI.

Do not add exchange/staking/liquidity behavior.

Do not create broad placeholder-only docs with no tests or checks.

Do not bypass the core/proof invariants.

## 16. High-Level Completion Assessment

Current completion posture:

```text
Local implementation: complete / green / parked
Testnet hardening: complete / green / parked
Private testnet pilot: complete / green / parked
Production/mainnet/public launch: not authorized / not started
```

Estimated ROX Anchor project completion by scope:

```text
ROX Anchor local + private pilot implementation scope: essentially 100%
ROX Anchor production/mainnet release scope: not started as an authorized plan
Full public bridge ecosystem: future separate workstream
```

The important distinction:

```text
The implementation has reached a very strong private-pilot completion milestone.
It has not crossed into public launch or production value movement.
```

## 17. Final Handoff Summary

ROX Anchor has completed the full BUILD_PLAN3 private testnet pilot path.

The repo is green across:

```text
fmt
workspace tests
workspace check
Anchor build
Anchor test
focused Clippy with -D warnings
secret-safe codebundle generation
Phase 16 closeout checker
```

The system now has:

```text
shared core state/type foundation
deterministic proof validation
safe CLI inspection/reporting
read-only RPC evidence modeling
coordinator decision modeling
relayer dry-run/capped authorization modeling
Anchor state transition program
test-only mint/burn semantics
integration and chaos tests
testnet hardening gates
private pilot config and workspace hygiene
deployment drill safety
read-only RPC verification
simulation-only transaction planning
explicit capped testnet sender model
pilot receipt/audit trail
CrabLink/internal ROC dry-run adapter shape
private ROC-to-ROX and ROX-to-ROC pilot flows
incident drills
authority/halt/recovery drills
private pilot closeout gate
```

The next session should begin from this completed/parked state and either:

```text
1. Prepare a concise commit/push summary, or
2. Start BUILD_PLAN4 as a separate, explicitly scoped production-readiness / audit / external review plan.
```

Required final qualifier for all future work:

```text
No public launch, mainnet launch, production bridge settlement, public ROX mint/burn, real internal ROC release, public bridge UI, exchange readiness, staking readiness, or liquidity readiness is authorized by BUILD_PLAN3 completion.
```

That should be enough to open a clean new session and continue without re-attaching every prior artifact.


### END NOTE - JULY 5 2026 - 19:55 CST


### BEGIN NOTE - JULY 5 2026 - 20:35 CST

Below is the carry-over note set for the next session. I framed it so we can paste it at the top of a new chat and immediately begin **BUILD_PLAN4** without re-explaining the project. Key boundaries are preserved from the current ROX Anchor codebundle, which explicitly says the artifact is not runtime authorization and does not authorize ROX runtime, Solana runtime, bridge runtime, staking, liquidity, exchange-facing logic, or external settlement.  The RustyOnions boundary remains that `svc-wallet` is the ROC mutation front-door and `ron-ledger` is durable economic truth.  CrabLink remains app/client display and user-intent infrastructure, with Tauri command bridges and boundary scanners already present in its source tree. 

# ROX Anchor Carry-Over Notes — Begin BUILD_PLAN4

## 0. Session Purpose

The next session should begin active work on:

```text
BUILD_PLAN4.md
Actual Private Testnet Execution / Test-Only Bridge Evidence
```

The goal of BUILD_PLAN4 is to move ROX Anchor from:

```text
private testnet pilot software readiness
```

to:

```text
actual private Solana devnet/testnet execution with test-only assets,
real deployed testnet program accounts,
real read-only RPC evidence,
real simulation against deployed addresses,
real capped private testnet transaction receipts,
real readback verification,
and no real internal ROC mutation.
```

The expected target after BUILD_PLAN4:

```text
Private testnet / test-only bridge goal: complete / green / parked
```

BUILD_PLAN4 is **not** the production/mainnet/public bridge plan.

BUILD_PLAN5 is the later final plan for:

```text
production-readiness
controlled real ROC ↔ ROX bridge release
mainnet decision gates
internal real-value canaries
CrabLink bridge UX
public-readiness gate
```

---

## 1. Current Status Going Into BUILD_PLAN4

Current completed plan status:

```text
BUILD_PLAN.md: complete / green / parked
BUILD_PLAN2.md: complete / green / parked
BUILD_PLAN3.md: complete / green / parked
BUILD_PLAN4.md: drafted in chat / not yet created in repo / next active plan
BUILD_PLAN5.md: drafted in chat / future / do not begin until BUILD_PLAN4 is complete
```

Current ROX Anchor implementation status:

```text
rox-anchor-core: green
rox-anchor-proof: green
rox-anchor-cli: green
rox-anchor-rpc-proof: green
rox-anchor-coordinator: green
rox-anchor-relayer: green
programs/rox-anchor: green
workspace tests: green
workspace check: green
anchor build: green
anchor test: green
focused Clippy with -D warnings: green
secret-safe codebundle generation: green
```

Current capability:

```text
compile and test the Anchor program
prove the ROC ↔ ROX state machine locally
model ROC-to-ROX burn-observation -> ROX mint intent
model ROX-to-ROC burn-observation -> internal ROC release intent
reject replay
reject mismatches
reject wrong authority
reject wrong mint/account
reject challenge/halt/recovery bypasses
run testnet readiness checks
run deployment-drill checks
run read-only RPC model checks
run simulation-only pilot plans
run explicit capped submit gate checks
run pilot receipt/audit checks
run private ROC-to-ROX and ROX-to-ROC pilot models
run chaos and authority drills
run private testnet pilot closeout checks
```

What is not done yet:

```text
actual private devnet/testnet deployment with external keys
actual deployed program account readback
actual test-only ROX mint initialization on devnet/testnet
actual live read-only RPC verification against deployed accounts
actual transaction simulation against deployed addresses
actual capped private testnet transaction send
actual receipt capture from real testnet transaction signatures
actual readback verification after live testnet sends
actual negative drills against deployed state
actual external evidence package from manual pilot run
```

Estimated completion at start of BUILD_PLAN4:

```text
Local implementation: 100%
Testnet software readiness: 100%
Private testnet/test-only bridge goal: ~85–90%
Production/mainnet real bridge goal: ~65–70%
```

Expected completion after BUILD_PLAN4:

```text
Private testnet/test-only bridge goal: 100%
Production/mainnet real bridge goal: still future BUILD_PLAN5 work
```

---

## 2. Critical Boundary To Preserve

BUILD_PLAN4 must not accidentally become BUILD_PLAN5.

BUILD_PLAN4 allows:

```text
private devnet/testnet deployment
external non-committed testnet keypairs
test-only ROX mint
test-only token accounts
test-only tiny amounts
live read-only RPC verification
transaction simulation against deployed testnet accounts
strictly capped private testnet submission
explicit operator approval before every live send path
receipt-backed pilot runs
operator safety drills
halt/recovery drills
authority/key-rotation drills
multi-RPC observation checks
private pilot runbooks
shadow ROC burn-intent modeling
shadow ROC release-intent modeling
CrabLink/internal ROC dry-run status display
no real internal ROC mutation
```

BUILD_PLAN4 does **not** authorize:

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
production ROC release
real internal ROC release from ROX burn evidence
real user funds
public bridge UI
public faucet
public claim page
exchange-facing behavior
staking
liquidity pools
market making
custody services
unbounded relayer submission
silent wallet/key usage
silent live RPC submission
operator key material committed to repo
raw private keys in logs
fake finality
fake success output
CrabLink display claiming final settlement before backend proof
```

The next session must keep this exact distinction:

```text
BUILD_PLAN4 = actual private testnet / test-only ROX evidence
BUILD_PLAN5 = real production/mainnet ROC ↔ ROX bridge
```

---

## 3. Authority Model To Preserve

### 3.1 ROX Anchor authority model

```text
rox-anchor-core:
  shared IDs, domains, directions, states, labels, challenge posture,
  halt posture, recovery posture, reusable errors

rox-anchor-proof:
  deterministic local proof review,
  replay rejection,
  binding mismatch rejection,
  challenge/halt/recovery blocker review

rox-anchor-rpc-proof:
  read-only RPC observation/evidence modeling

rox-anchor-coordinator:
  evidence assembly, local decision modeling, proof/core handoff

rox-anchor-relayer:
  dry-run, simulation, capped submit gating, receipt/audit modeling

rox-anchor-cli:
  operator inspection, check/status/proof/pilot/receipt/drill surfaces

programs/rox-anchor:
  Anchor-owned on-chain Solana state transitions,
  mint/burn binding rules,
  challenge/halt/recovery/finalization rules
```

No crate should create a competing state machine.

No crate should invent a separate acceptance rule.

No crate should call something “final” unless the relevant proof/readback/reconciliation gate has passed.

### 3.2 RustyOnions authority model

```text
svc-wallet:
  internal ROC mutation front-door

ron-ledger:
  durable ROC economic truth and replay

ron-accounting:
  snapshots/reports only, not balance truth

svc-rewarder:
  payout planning only, not balance mutation

ron-policy:
  policy/economics gates only, not balance mutation

svc-gateway:
  public/backend boundary, not ledger truth

omnigate:
  hydration/access composition, not ledger truth

svc-index:
  lookup/pointer service, not payment truth

svc-storage:
  b3/artifact storage, not payment truth
```

Hard rule:

```text
ROX Anchor must not directly issue, release, or mutate real internal ROC.
```

The future real ROC path remains:

```text
svc-wallet -> ron-ledger
```

### 3.3 CrabLink authority model

```text
CrabLink:
  display and user intent only

React:
  display, route rendering, confirmation UX

TypeScript adapters:
  typed request/status adapters, redaction, gateway-first calls

Tauri Rust:
  native privilege boundary only, bounded/allowlisted commands

RustyOnions backend:
  receipt truth, paid-access truth, wallet/ledger truth

ROX Anchor:
  proof/evidence/Solana program/relayer/readback/reconciliation path
```

CrabLink must not become:

```text
wallet truth
ledger truth
receipt truth
bridge truth
Solana runtime authority
ROX mint authority
ROX burn authority
paid unlock authority from cache
finality authority
settlement authority
```

---

## 4. What BUILD_PLAN4 Must Accomplish

BUILD_PLAN4 should deliver actual private testnet evidence.

The core evidence chain:

```text
external operator workspace
→ actual Anchor build artifact capture
→ actual private devnet/testnet deploy or safe failed deploy receipt
→ test-only ROX mint/config initialization
→ live read-only RPC verification
→ simulation against actual deployed account addresses
→ capped private testnet ROC-to-ROX test flow
→ capped private testnet ROX-to-ROC test flow
→ receipt ledger reconciliation
→ negative drills
→ halt/recovery/authority drills
→ RustyOnions dry-run handoff
→ optional CrabLink display-only private testnet status
→ evidence package
→ closeout gate
```

Successful BUILD_PLAN4 completion should produce:

```text
actual private testnet deployment evidence
actual test-only mint/config evidence
actual live read-only RPC evidence
actual simulation receipts
actual capped send receipts, if sends are performed
actual readback receipts for every sent transaction
actual failure receipts for negative drills
actual authority/halt/recovery drill reports
actual dry-run RustyOnions handoff report
actual display-only CrabLink status boundary, if touched
actual final private testnet evidence package
```

---

## 5. BUILD_PLAN4 Phase Ledger

The drafted BUILD_PLAN4 has this phase order:

```text
Phase 0  — Freeze BUILD_PLAN3 green baseline
Phase 1  — External operator workspace actualization
Phase 2  — Actual Anchor build artifact capture
Phase 3  — Actual private devnet/testnet deployment
Phase 4  — Test-only ROX mint and program config initialization
Phase 5  — Live read-only RPC evidence against deployed accounts
Phase 6  — Simulation against actual deployed testnet addresses
Phase 7  — Actual capped testnet ROC-to-ROX flow
Phase 8  — Actual capped testnet ROX-to-ROC flow
Phase 9  — Receipt ledger reconciliation for actual runs
Phase 10 — Actual negative drills against deployed testnet state
Phase 11 — Actual halt, recovery, and authority drills
Phase 12 — RustyOnions dry-run handoff evidence
Phase 13 — CrabLink display-only private testnet status
Phase 14 — Actual private testnet evidence package
Phase 15 — BUILD_PLAN4 closeout gate
```

Do not jump ahead.

Do not start with Phase 3.

Start by adding the actual `BUILD_PLAN4.md` file to the repo, then run Phase 0.

---

## 6. First Concrete Task In Next Session

The first concrete task is:

```text
Create BUILD_PLAN4.md in the rox-anchor repo from the drafted plan.
```

The plan was drafted in the previous session, but it is likely not yet written into the repository.

Expected file:

```text
/Users/mymac/Desktop/rox-anchor/BUILD_PLAN4.md
```

After creating it, run baseline checks.

First patch should be a file write only:

```bash
cd /Users/mymac/Desktop/rox-anchor

cat > BUILD_PLAN4.md <<'EOF'
# ROX Anchor Build Plan 4

## Actual Private Testnet Execution / Test-Only Bridge Evidence

...
EOF

cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Do not include git add/commit/push unless explicitly requested.

---

## 7. Phase 0 Notes — Freeze BUILD_PLAN3 Green Baseline

Purpose:

```text
Prove the repo is still green before any actual private testnet work begins.
```

Required checks:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Focused Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Exit condition:

```text
BUILD_PLAN4.md exists.
BUILD_PLAN3 closeout checker still passes.
Workspace tests/checks still pass.
Anchor build/test still passes.
Focused Clippy still passes.
Safe codebundle regenerates.
No tracked secrets appear.
```

What to do if Phase 0 fails:

```text
Fix the first compiler/test/checker failure only.
Do not begin private testnet workspace/deploy work until Phase 0 is green.
```

---

## 8. Phase 1 Notes — External Operator Workspace Actualization

Purpose:

```text
Create a safe external private pilot workspace model without committing secrets.
```

This phase should not deploy.

This phase should not submit.

This phase should not load real keys inside a checked-in script.

Likely files to add:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
scripts/check_actual_private_testnet_workspace.sh
crates/rox-anchor-cli/tests/actual_private_testnet_workspace.rs
```

Need to prove:

```text
operator workspace is external
keypair paths are external
RPC URL/provider is external or redacted
receipt output path is external/ignored
local artifact paths are ignored
docs only show examples, not secrets
checker rejects key-shaped tracked files
checker rejects raw RPC provider tokens
checker rejects raw mnemonic/secret/key content
CLI/test display redacts paths and provider details
```

Allowed external local paths:

```text
.rox-anchor-pilot/
.rox-anchor-private-pilot/
private-pilot/
pilot-artifacts/
pilot-rpc/
pilot-keys/
pilot-keypairs/
pilot-wallets/
pilot-secrets/
pilot-receipts/
pilot-audit/
pilot-deploy/
pilot-ledger/
pilot-tmp/
```

Suggested test command:

```bash
cargo fmt -p rox-anchor-cli
bash scripts/check_actual_private_testnet_workspace.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_workspace
cargo test -p rox-anchor-cli
```

Exit condition:

```text
External private pilot workspace shape is documented, redacted, ignored, and checker-covered.
```

---

## 9. Phase 2 Notes — Actual Anchor Build Artifact Capture

Purpose:

```text
Capture real build metadata before deploying.
```

This is still non-deploying.

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_BUILD_ARTIFACTS.md
crates/rox-anchor-core/tests/actual_testnet_artifact_manifest.rs
crates/rox-anchor-cli/tests/actual_testnet_artifact_manifest_status.rs
```

Need to capture non-secret metadata:

```text
program ID
IDL hash
program binary hash
Anchor version
Solana CLI version
Rust version
build timestamp if deterministic enough for report
artifact path redacted
```

Need to avoid:

```text
claiming deployment
claiming finality
claiming testnet run success
committing build output
committing key paths
committing private artifacts
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
anchor build
cargo test -p rox-anchor-core --test actual_testnet_artifact_manifest
cargo test -p rox-anchor-cli --test actual_testnet_artifact_manifest_status
cargo check -p rox-anchor
```

Exit condition:

```text
Build artifacts are captured and validated as build evidence only, not deployment evidence.
```

---

## 10. Phase 3 Notes — Actual Private Devnet/Testnet Deployment

Purpose:

```text
Perform or safely attempt actual private devnet/testnet deployment using external keys.
```

This is the first phase where manual live devnet/testnet deployment may happen.

Before manual deployment:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_deploy.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
anchor build
cargo test --workspace
cargo check --workspace
```

Manual deployment command shape, only after checks:

```bash
# anchor deploy \
#   --provider.cluster testnet \
#   --provider.wallet /external/path/to/testnet-payer.json
```

Must reject:

```text
mainnet-beta
production labels
public labels
missing external config
missing external payer
tracked key paths
unredacted provider token
```

Must capture:

```text
program ID
deploy signature
deploy slot
cluster
IDL hash
program binary hash
upgrade authority policy
payer redacted
authority redacted
failure reason if deployment failed
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_DEPLOYMENT.md
scripts/check_actual_private_testnet_deploy_receipt.sh
crates/rox-anchor-cli/tests/actual_private_testnet_deploy_receipt.rs
```

Post-manual gate:

```bash
bash scripts/check_actual_private_testnet_deploy_receipt.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_deploy_receipt
bash scripts/check_private_pilot_hygiene.sh .
```

Exit condition:

```text
Actual private testnet deployment or safe failed deployment attempt is evidenced by redacted receipt.
```

---

## 11. Phase 4 Notes — Test-Only ROX Mint and Program Config Initialization

Purpose:

```text
Initialize deployed testnet program with test-only ROX assets and separated authorities.
```

This phase may create or bind test-only mint/config state.

It must not create public/mainnet/production ROX.

Need to prove:

```text
mint is test-only
token account is test-only
amount caps are tiny
supply caps are tiny
mint authority is separated
halt authority is separated
recovery authority is separated
upgrade authority policy is documented
config account is initialized
read-only RPC confirms state
```

Likely files:

```text
docs/pilot/ACTUAL_TEST_ONLY_MINT_INITIALIZATION.md
scripts/check_actual_test_only_mint_initialization.sh
crates/rox-anchor-cli/tests/actual_test_only_mint_initialization.rs
crates/rox-anchor-rpc-proof/tests/actual_test_only_mint_readback.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot initialize-test-only-mint \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/init-mint.pilot-receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT"
```

Commands:

```bash
cargo fmt -p rox-anchor-cli -p rox-anchor-rpc-proof -p rox-anchor
bash scripts/check_actual_test_only_mint_initialization.sh .
cargo test -p rox-anchor-cli --test actual_test_only_mint_initialization
cargo test -p rox-anchor-rpc-proof --test actual_test_only_mint_readback
anchor test
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Exit condition:

```text
Test-only mint/config initialization is verified by read-only RPC and cannot be confused with public ROX launch.
```

---

## 12. Phase 5 Notes — Live Read-Only RPC Evidence Against Deployed Accounts

Purpose:

```text
Verify deployed program/config/mint/token state through live read-only RPC.
```

This phase does not submit.

Required observations:

```text
current slot
program account
program owner/executable state
program config account
test-only ROX mint
test token accounts
recent deploy/init signatures
upgrade authority status where available
commitment/finality label
RPC source label
timestamp
staleness window
```

Reject:

```text
stale readback
wrong program ID
wrong mint
wrong token account
wrong cluster
under-quorum RPC evidence
RPC disagreement
mainnet-beta if not explicitly allowed
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_READ_ONLY_EVIDENCE.md
crates/rox-anchor-rpc-proof/tests/actual_private_testnet_read_only_rpc.rs
crates/rox-anchor-cli/tests/actual_private_testnet_read_only_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot proof read-only \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/read-only-evidence.pilot-receipt.json
```

Commands:

```bash
cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof --test actual_private_testnet_read_only_rpc
cargo test -p rox-anchor-cli --test actual_private_testnet_read_only_command
cargo test --workspace
```

Exit condition:

```text
Live read-only RPC evidence confirms actual deployed testnet accounts without submission.
```

---

## 13. Phase 6 Notes — Simulation Against Actual Deployed Testnet Addresses

Purpose:

```text
Simulate transactions against real deployed testnet accounts before any live send.
```

Must require:

```text
deployed program ID
config account
test-only mint
test token accounts
read-only RPC evidence
accepted proof review
accepted coordinator decision
relayer dry-run acceptance
test-only labels
tiny amount caps
correct operation ID
correct idempotency key
correct nonce
```

Must reject:

```text
wrong program
wrong mint
wrong token account
wrong cluster
stale evidence
missing read-only evidence
missing proof acceptance
missing coordinator decision
missing dry-run
production labels
mainnet labels
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_SIMULATION.md
crates/rox-anchor-relayer/tests/actual_private_testnet_simulation.rs
crates/rox-anchor-coordinator/tests/actual_private_testnet_simulation_gate.rs
crates/rox-anchor-cli/tests/actual_private_testnet_simulation_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot simulate \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/simulation.pilot-receipt.json \
#   --simulate-only
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test actual_private_testnet_simulation
cargo test -p rox-anchor-coordinator --test actual_private_testnet_simulation_gate
cargo test -p rox-anchor-cli --test actual_private_testnet_simulation_command
cargo test --workspace
```

Exit condition:

```text
Simulation succeeds or fails safely against actual deployed private testnet addresses without sending.
```

---

## 14. Phase 7 Notes — Actual Capped Testnet ROC-to-ROX Flow

Purpose:

Run test-only forward bridge shape:

```text
shadow internal ROC burn intent
→ proof review
→ RPC evidence
→ coordinator decision
→ relayer dry-run
→ simulation
→ explicit capped private testnet send
→ test-only ROX mint/finalize action
→ readback
→ receipt
```

Must not burn real ROC.

Must not mutate real RustyOnions ledger.

Must use shadow/dry-run internal ROC intent only.

Required gates before live send:

```text
explicit test operation ID
explicit idempotency key
nonce
accepted proof review
live read-only RPC evidence
accepted coordinator decision
relayer dry-run acceptance
successful simulation
operator approval phrase
tiny amount cap
retry cap
operation cap
test-only labels
not halted
not challenge-blocked
not recovery-required
```

Likely files:

```text
docs/pilot/ACTUAL_ROC_TO_ROX_PRIVATE_TESTNET_RUN.md
crates/rox-anchor-relayer/tests/actual_roc_to_rox_capped_send.rs
crates/rox-anchor-rpc-proof/tests/actual_roc_to_rox_readback.rs
crates/rox-anchor-coordinator/tests/actual_roc_to_rox_decision.rs
crates/rox-anchor-cli/tests/actual_roc_to_rox_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot roc-to-rox \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/roc-to-rox-send.pilot-receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND" \
#   --max-operations 1 \
#   --max-amount-minor 1
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test actual_roc_to_rox_capped_send
cargo test -p rox-anchor-rpc-proof --test actual_roc_to_rox_readback
cargo test -p rox-anchor-coordinator --test actual_roc_to_rox_decision
cargo test -p rox-anchor-cli --test actual_roc_to_rox_command
cargo test --workspace
```

Exit condition:

```text
Capped private testnet ROC-to-ROX run produces test-only ROX evidence and readback-verified receipt without burning real ROC.
```

---

## 15. Phase 8 Notes — Actual Capped Testnet ROX-to-ROC Flow

Purpose:

Run test-only reverse bridge shape:

```text
test-only ROX burn/finalize evidence
→ read-only RPC verification
→ proof review
→ coordinator decision
→ relayer dry-run
→ simulation or capped send where applicable
→ dry-run internal ROC release intent only
→ receipt
```

Must not release real ROC.

Must not mutate real RustyOnions ledger.

Must produce internal ROC release-intent only.

Required gates:

```text
explicit test operation ID
explicit idempotency key
nonce
read-only RPC evidence
accepted proof review
accepted coordinator decision
relayer dry-run acceptance
successful simulation where applicable
operator approval for any live send
test-only labels
tiny amount cap
not halted
not challenge-blocked
not recovery-required
```

Likely files:

```text
docs/pilot/ACTUAL_ROX_TO_ROC_PRIVATE_TESTNET_RUN.md
crates/rox-anchor-relayer/tests/actual_rox_to_roc_capped_send.rs
crates/rox-anchor-rpc-proof/tests/actual_rox_to_roc_readback.rs
crates/rox-anchor-coordinator/tests/actual_rox_to_roc_decision.rs
crates/rox-anchor-cli/tests/actual_rox_to_roc_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot rox-to-roc \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/rox-to-roc.pilot-receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND" \
#   --max-operations 1 \
#   --max-amount-minor 1
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test actual_rox_to_roc_capped_send
cargo test -p rox-anchor-rpc-proof --test actual_rox_to_roc_readback
cargo test -p rox-anchor-coordinator --test actual_rox_to_roc_decision
cargo test -p rox-anchor-cli --test actual_rox_to_roc_command
cargo test --workspace
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
```

Exit condition:

```text
Capped private testnet ROX-to-ROC run proves test-only ROX burn/readback and produces only dry-run internal ROC release intent.
```

---

## 16. Phase 9 Notes — Receipt Ledger Reconciliation For Actual Runs

Purpose:

```text
Make every actual private testnet run inspectable, linked, redacted, and replay-resistant.
```

Receipt types to collect:

```text
build artifact manifest
deploy receipt
initialization receipt
read-only RPC evidence receipt
simulation receipt
ROC-to-ROX send receipt
ROC-to-ROX readback receipt
ROX-to-ROC burn/observe receipt
ROX-to-ROC readback receipt
negative drill failure receipts
halt/recovery drill receipts
authority drill receipts
dry-run RustyOnions handoff receipt
```

Need to reject:

```text
duplicate receipt IDs
mismatched operation IDs
mismatched idempotency keys
mismatched signatures
mismatched mint/account/program ID
live submission claim without transaction signature
readback success without readback evidence
production settlement claim
real ROC release claim
unredacted RPC/provider/key path
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_RECEIPT_LEDGER.md
scripts/check_actual_private_testnet_receipts.sh
crates/rox-anchor-relayer/tests/actual_private_testnet_receipt_ledger.rs
crates/rox-anchor-cli/tests/actual_private_testnet_receipt_display.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
bash scripts/check_actual_private_testnet_receipts.sh .
cargo test -p rox-anchor-relayer --test actual_private_testnet_receipt_ledger
cargo test -p rox-anchor-cli --test actual_private_testnet_receipt_display
cargo test --workspace
```

Exit condition:

```text
Actual private testnet receipts form deterministic, redacted, replay-resistant evidence ledger.
```

---

## 17. Phase 10 Notes — Actual Negative Drills Against Deployed Testnet State

Purpose:

```text
Prove deployed private testnet operations fail safely.
```

Required drills:

```text
wrong program ID
wrong mint
wrong token account
wrong authority
missing config account
missing mint account
stale readback
under-quorum RPC evidence
RPC provider disagreement
duplicate operation ID
duplicate idempotency key
nonce reuse
receipt tamper
missing receipt
operator approval omitted
send disabled
cap exceeded
halt before simulation
halt after simulation before send
halt after send before readback
recovery during pending operation
readback missing after send
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md
crates/rox-anchor-coordinator/tests/actual_testnet_negative_drills.rs
crates/rox-anchor-relayer/tests/actual_testnet_negative_drills.rs
crates/rox-anchor-rpc-proof/tests/actual_testnet_negative_drills.rs
crates/rox-anchor-cli/tests/actual_testnet_negative_drill_reports.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-coordinator -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-coordinator --test actual_testnet_negative_drills
cargo test -p rox-anchor-relayer --test actual_testnet_negative_drills
cargo test -p rox-anchor-rpc-proof --test actual_testnet_negative_drills
cargo test -p rox-anchor-cli --test actual_testnet_negative_drill_reports
cargo test --workspace
```

Exit condition:

```text
Actual private testnet negative drills fail safely and produce inspectable failure receipts.
```

---

## 18. Phase 11 Notes — Actual Halt, Recovery, and Authority Drills

Purpose:

```text
Practice operator safety procedures against deployed private testnet state.
```

Drills:

```text
halt before simulation
halt after simulation before send
halt after capped send before readback
recovery after halt
wrong-authority halt attempt
wrong-authority recovery attempt
key rotation intent drill
upgrade authority checklist
clean operation resumes only after valid recovery
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md
docs/pilot/ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md
crates/rox-anchor-core/tests/actual_private_testnet_authority_drills.rs
crates/rox-anchor-cli/tests/actual_private_testnet_drill_reports.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
cargo test -p rox-anchor-core --test actual_private_testnet_authority_drills
cargo test -p rox-anchor-cli --test actual_private_testnet_drill_reports
cargo test --workspace
anchor build
anchor test
```

Exit condition:

```text
Authority, halt, and recovery procedures work against deployed private testnet state and remain redacted/operator-readable.
```

---

## 19. Phase 12 Notes — RustyOnions Dry-Run Handoff Evidence

Purpose:

```text
Connect actual private testnet receipts to RustyOnions dry-run intent/status shapes without mutating real ROC.
```

Must prove:

```text
shadow ROC burn-intent maps to RustyOnions-shaped dry-run input
ROX burn/readback evidence maps to RustyOnions-shaped dry-run release-intent output
svc-wallet is not called for real mutation
ron-ledger is not mutated
paid content is not unlocked
real balances are not updated
operation ID is preserved
idempotency key is preserved
receipt linkage is preserved
Solana signatures/paths are redacted where needed
future real ROC path must go through svc-wallet/ron-ledger
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md
crates/rox-anchor-core/tests/actual_rustyonions_dry_run_handoff.rs
crates/rox-anchor-coordinator/tests/actual_rustyonions_dry_run_handoff.rs
crates/rox-anchor-cli/tests/actual_rustyonions_dry_run_status.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-core --test actual_rustyonions_dry_run_handoff
cargo test -p rox-anchor-coordinator --test actual_rustyonions_dry_run_handoff
cargo test -p rox-anchor-cli --test actual_rustyonions_dry_run_status
cargo test --workspace
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

Exit condition:

```text
Actual private testnet evidence can be represented as RustyOnions dry-run handoff status without real ROC mutation.
```

---

## 20. Phase 13 Notes — CrabLink Display-Only Private Testnet Status

Purpose:

```text
Prepare CrabLink-facing private testnet status display without giving CrabLink authority.
```

This phase may stay in ROX Anchor docs/tests only unless we explicitly touch CrabLink.

CrabLink may display:

```text
private testnet status
test-only ROX status
read-only RPC status
proof status
receipt status
halt/recovery status
dry-run internal ROC status
backend-derived labels
explicit warnings that no real ROC moved
```

CrabLink must not:

```text
submit Solana transactions
mint ROX
burn ROX
release ROC
mutate wallet
mutate ledger
unlock paid content from private testnet evidence
claim final settlement
store private keys
hold Solana authority
treat local cache as bridge truth
```

Likely ROX Anchor files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md
crates/rox-anchor-cli/tests/actual_crablink_private_testnet_status.rs
```

Possible future CrabLink files if touched:

```text
docs/tauri/ROX_ANCHOR_PRIVATE_TESTNET_STATUS_BOUNDARY.md
scripts/check-rox-anchor-private-testnet-status-boundary.mjs
apps/crablink-tauri/src/pages/quickchain/ or future status page
```

ROX Anchor commands:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --test actual_crablink_private_testnet_status
cargo test --workspace
```

CrabLink commands only if CrabLink is touched:

```bash
cd /Users/mymac/Desktop/crablink

npm run check:tauri
npm run build
```

Exit condition:

```text
CrabLink can display private testnet status as backend-derived, display-only evidence without wallet/ledger/bridge authority.
```

---

## 21. Phase 14 Notes — Actual Private Testnet Evidence Package

Purpose:

```text
Create audit-ready evidence package for the actual private testnet/test-only run.
```

Evidence package should include:

```text
build artifact manifest
deploy receipt
test-only mint/config initialization receipt
read-only RPC evidence receipt
simulation receipts
ROC-to-ROX receipts
ROX-to-ROC receipts
negative drill receipts
halt/recovery drill reports
authority drill reports
RustyOnions dry-run handoff reports
CrabLink display-only status report if produced
known failure notes
operator environment notes
redaction report
receipt index
operation ID index
idempotency key index
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md
scripts/check_actual_private_testnet_evidence_package.sh
crates/rox-anchor-cli/tests/actual_private_testnet_evidence_package.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
bash scripts/check_actual_private_testnet_evidence_package.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_evidence_package
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
Actual private testnet/test-only bridge run has audit-ready, redacted evidence package.
```

---

## 22. Phase 15 Notes — BUILD_PLAN4 Closeout Gate

Purpose:

```text
Declare whether actual private testnet/test-only bridge goal is complete.
```

Closeout must verify:

```text
all local Rust tests pass
all Anchor tests pass
all actual private testnet checks pass
actual deploy receipt exists if deployment was performed
actual test-only mint initialization receipt exists if initialization was performed
live read-only RPC evidence exists
simulation receipts exist
capped send receipts exist if capped sends were performed
readback receipts exist for every capped send
failure receipts exist for negative drills
halt/recovery drills were performed or simulated
authority drills were performed or simulated
RustyOnions handoff remains dry-run only
CrabLink status remains display-only
no key material is tracked
no mainnet behavior exists
no public launch behavior exists
no production settlement behavior exists
no real internal ROC mutation exists
no exchange/staking/liquidity behavior exists
known pilot failures are documented
BUILD_PLAN5 remains separate and future
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md
scripts/check_actual_private_testnet_closeout.sh
crates/rox-anchor-cli/tests/actual_private_testnet_closeout.rs
```

Commands:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
bash scripts/check_actual_private_testnet_closeout.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_closeout
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
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
cargo clippy --workspace --all-targets -- -D warnings
```

Exit condition:

```text
BUILD_PLAN4 is complete / green / parked.
Actual private testnet/test-only bridge evidence is complete.
No production/mainnet/public/real-ROC behavior is authorized by BUILD_PLAN4.
BUILD_PLAN5 may begin later.
```

---

## 23. First Patch Strategy For Next Session

The next assistant should not start by editing random crates.

Start with the plan file.

Recommended first patch:

```bash
cd /Users/mymac/Desktop/rox-anchor

cat > BUILD_PLAN4.md <<'EOF'
# ROX Anchor Build Plan 4

## Actual Private Testnet Execution / Test-Only Bridge Evidence

[Use the full drafted BUILD_PLAN4 text from the prior session.]
EOF

cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

If that passes, next patch begins Phase 1.

If it fails, fix the first failure only.

---

## 24. Expected First Real Implementation Patch After BUILD_PLAN4.md Exists

Phase 1 likely adds:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
scripts/check_actual_private_testnet_workspace.sh
crates/rox-anchor-cli/tests/actual_private_testnet_workspace.rs
```

The checker should inspect tracked files and confirm:

```text
no private key JSON files
no seed phrases
no Solana keypair arrays
no raw RPC token URLs
no unredacted external key paths
no production/mainnet labels
no public launch labels
actual local artifact examples are ignored/redacted
operator config examples are safe placeholders
```

Suggested test assertions:

```text
workspace doc says external only
workspace doc says no committed keys
workspace doc says no real ROC
workspace doc says test-only ROX
workspace doc says no public/mainnet/production
checker passes clean repo
checker rejects artificial key-shaped fixture
checker rejects artificial mainnet/public claim
CLI status redacts external paths
```

---

## 25. Commands To Keep Handy

### ROX Anchor full baseline

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

### Focused ROX Anchor Clippy

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

### Workspace Clippy only after focused Clippy is green

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### RustyOnions baseline if/when RustyOnions dry-run handoff is touched

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo test -p ron-proto
cargo test -p ron-ledger
cargo test -p svc-wallet
cargo test -p ron-accounting
cargo test -p ron-policy
```

### CrabLink baseline if/when CrabLink status display is touched

```bash
cd /Users/mymac/Desktop/crablink

npm run check:tauri
npm run build
```

---

## 26. Common Failure Modes To Watch

### 26.1 BUILD_PLAN4 turning into production launch

Reject any language that implies:

```text
public launch
mainnet launch
production bridge settlement
public ROX mint/burn
real ROC release
real user funds
exchange readiness
staking readiness
liquidity readiness
```

BUILD_PLAN4 must stay private testnet/test-only.

### 26.2 Committing secrets

Reject any tracked files that look like:

```text
Solana keypair JSON array
mnemonic
seed phrase
private key
payer key
authority key
raw RPC URL with token
provider auth token
wallet secret
.env with sensitive content
```

### 26.3 Fake finality

Reject wording like:

```text
settled
finalized
production complete
real bridge complete
public ready
mainnet ready
funds released
ROC released
ROX minted
```

unless it is explicitly scoped and receipt/readback/reconciliation-supported.

For BUILD_PLAN4, prefer labels:

```text
test-only
private testnet
read-only verified
simulation accepted
capped send attempted
testnet readback verified
dry-run ROC intent
dry-run ROC release intent
not production
not mainnet
not public
```

### 26.4 Direct ROC mutation

Reject any path where ROX Anchor directly calls real RustyOnions wallet/ledger mutation in BUILD_PLAN4.

Allowed:

```text
dry-run intent
dry-run release intent
status shape
receipt shape
handoff model
```

Not allowed:

```text
svc-wallet real burn
svc-wallet real release
ron-ledger mutation
paid unlock
balance update
```

### 26.5 CrabLink authority creep

If CrabLink is touched, reject:

```text
raw invoke("solana_*")
raw invoke("rox_*")
raw invoke("bridge_*")
client-side token mint/burn
wallet/ledger mutation from client
cache-only paid unlock
local receipt treated as proof
Solana finality claimed in React
```

CrabLink can display backend-derived status only.

---

## 27. How To Answer “Are We Ready For Solana?”

Correct answer at start of BUILD_PLAN4:

```text
We are ready to begin actual private devnet/testnet execution with test-only assets.
We are not ready for mainnet/public/production real-value bridge yet.
```

After BUILD_PLAN4 completion:

```text
We should have actual private testnet/test-only evidence and be ready to start BUILD_PLAN5.
```

After BUILD_PLAN5 completion, assuming all gates pass:

```text
We should be able to burn/lock real ROC through svc-wallet -> ron-ledger and mint actual ROX on Solana,
and burn ROX on Solana to release ROC through svc-wallet -> ron-ledger.
```

But never claim BUILD_PLAN4 alone enables real ROC ↔ ROX production value movement.

---

## 28. Completion Rate Language For Next Session

At start of BUILD_PLAN4:

```text
BUILD_PLAN4: 0% started in repo unless BUILD_PLAN4.md has been written
Private testnet/test-only bridge goal: ~85–90%
Production/mainnet bridge goal: ~65–70%
Overall real ROC ↔ ROX goal: ~65–70%
```

After Phase 0:

```text
BUILD_PLAN4 baseline frozen
Private testnet/test-only bridge goal still ~85–90%
```

After Phase 3:

```text
Actual deployment evidence exists or safe failed deployment receipt exists
Private testnet/test-only bridge goal moves materially closer
```

After Phase 5:

```text
Actual deployed/read-only RPC verification exists
Private testnet/test-only bridge goal should be ~90–95%
```

After Phase 8:

```text
Both private testnet directions have actual or safely failed evidence
Private testnet/test-only bridge goal should be ~95–98%
```

After Phase 15:

```text
BUILD_PLAN4 complete / green / parked
Private testnet/test-only bridge goal: 100%
Production/mainnet bridge goal remains future BUILD_PLAN5
```

---

## 29. What BUILD_PLAN5 Will Do Later

Do not start BUILD_PLAN5 yet.

BUILD_PLAN5 is for:

```text
production bridge DTOs in ron-proto
ron-ledger bridge truth model
svc-wallet bridge front-door
policy/economics bridge gates
ROX Anchor production config
Anchor production account/PDA review
production relayer/submit path
bidirectional reconciliation
readback/finality labels
emergency halt/recovery/rollback
external audit package
mainnet deployment decision gate
mainnet deployment/read-only verification
internal-owned real-value canaries
backend API/gateway enforcement
CrabLink controlled bridge UX
allowlisted beta gate
public-readiness gate
final project closeout
```

BUILD_PLAN5 is the plan that, if completed and explicitly approved, should allow:

```text
real ROC burn/lock through svc-wallet -> ron-ledger
real ROX mint/finalize on Solana
real ROX SPL tokens stored in Solana wallets
real ROX burn/finalize evidence from Solana
real ROC release through svc-wallet -> ron-ledger
```

But BUILD_PLAN5 is future.

BUILD_PLAN4 must first prove the actual private testnet/test-only run.

---

## 30. Final Starting Instruction For New Session

Start the new session with this working assumption:

```text
We are beginning BUILD_PLAN4.
The first task is to create BUILD_PLAN4.md in the rox-anchor repo from the drafted plan.
Then run Phase 0 baseline checks.
Then begin Phase 1 only after the baseline is green.
```

Do not ask for clarification if the user provides latest terminal output and asks for the next patch.

Default next action after receiving fresh terminal output:

```text
1. Inspect the first failing command, if any.
2. Patch the exact file(s).
3. Run the focused command again.
4. Do not expand scope until green.
```

If the baseline is green, move to Phase 1:

```text
External operator workspace actualization
```

First likely Phase 1 patch:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
scripts/check_actual_private_testnet_workspace.sh
crates/rox-anchor-cli/tests/actual_private_testnet_workspace.rs
```

---

## 31. Final One-Screen Summary

```text
Current state:
  BUILD_PLAN3 complete / green / parked.
  ROX Anchor implementation is local/testnet-readiness green.
  BUILD_PLAN4 and BUILD_PLAN5 were drafted in chat.
  BUILD_PLAN4 needs to be created in repo and started.

BUILD_PLAN4 objective:
  Actual private devnet/testnet execution with test-only assets.
  Real deployed testnet program accounts.
  Real read-only RPC evidence.
  Real simulation against deployed addresses.
  Real capped private testnet receipts.
  Real readback verification.
  No real ROC mutation.

Next session first step:
  Create BUILD_PLAN4.md.
  Run Phase 0 baseline.
  Fix first failure if any.
  Begin Phase 1 only when green.

Still forbidden:
  mainnet
  public launch
  public ROX mint/burn
  production bridge settlement
  real internal ROC release
  real user funds
  public bridge UI
  exchange/staking/liquidity
  fake finality
  committed key material

Future:
  BUILD_PLAN5 is the final production/mainnet/real ROC ↔ ROX bridge plan.
```

That is the carry-over package I would use to open the next session cleanly and immediately start BUILD_PLAN4.


### END NOTE - JULY 5 2026 - 20:35 CST