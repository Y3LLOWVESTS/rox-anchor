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