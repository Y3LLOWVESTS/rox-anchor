
---

title: ROX Anchor Full-Phase Buildplan
version: 0.1.0
status: draft
last-updated: 2026-07-01
audience: contributors, auditors, reviewers, future bridge planners
scope: full scaffold roadmap / phase-gated / inert-until-authorized
-------------------------------------------------------------------

# ROX Anchor Full-Phase Buildplan

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only unless a later explicit runtime gate authorizes otherwise.

RO:WHAT — Defines the complete phase-by-phase buildplan for the `rox-anchor` repository, from docs-only planning through inert scaffold, threat review, proof design, disabled skeleton, non-value local proof engine, coordination boundaries, private non-value dry run, CrabLink display-only status, pre-audit hardening, audit/recovery drills, and later runtime decision.

RO:WHY — Allows the full ROX Anchor project to be built methodically without confusing empty placeholders, docs, skeletons, tests, dry runs, audits, or future runtime decisions with active bridge behavior.

RO:INTERACTS — ROX Anchor scaffold, docs, specs, schemas, scripts, crates, Solana/Anchor program surfaces, CrabLink display-only UI, test fixtures, ops runbooks, audit records, RustyOnions Internal ROC, `svc-wallet`, `ron-ledger`, QuickChain future proof infrastructure, future ROX/Solana/Anchor planning.

RO:INVARIANTS — Internal ROC truth remains with `svc-wallet` + `ron-ledger`; ROX Anchor does not become economic truth; proof packages are evidence, not finality; CrabLink remains display/user intent only; each phase must be explicitly parked before the next phase gains authority.

RO:SECURITY — No fake finality, no client-side settlement, no direct ROC mutation, no hidden mint/issue path, no single-RPC settlement truth, no coordinator/relayer unilateral finality, no public bridge path, no value-bearing devnet, no staking, no liquidity, no exchange-facing behavior unless later separately authorized.

RO:TEST — Every phase has a local checker, review checklist, acceptance gates, closeout document, and non-authorization reminder. Runtime-shaped checks must fail closed unless a phase explicitly authorizes the relevant surface.

Anchor meaning used in this document: ROX-anchor planning repo.

ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

---

## 0. Current Safe Status

The project begins from this safe status:

```text
Internal ROC Beta Phase 6: COMPLETE / GREEN / PARKED.
Internal ROC value-loop proof: COMPLETE / GREEN / PARKED.
Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED.
QuickChain boundary/preflight through Phase 5: COMPLETE / GREEN / PARKED, not public chain/runtime completion.
Bridge / ROX / Solana / staking / liquidity / external settlement: docs / threat-model / decision-gate only.
```

This buildplan does not change that status by itself.

This buildplan does not authorize runtime.

This buildplan does not authorize bridge behavior.

This buildplan does not authorize ROX launch.

This buildplan does not authorize Solana deployment.

This buildplan does not authorize staking, liquidity, exchange-facing behavior, or external settlement.

This buildplan exists to make future authorization explicit, staged, testable, reviewable, and reversible.

---

## 1. Core Build Model

### 1.1 Phase

A **phase** is a coherent authorization boundary.

Each phase must define:

```text
purpose
allowed file surfaces
forbidden file surfaces
implementation authority, if any
checker / test gate
review checklist
closeout label
non-authorization reminder
```

Passing one phase does not authorize the next.

### 1.2 Round

A **round** is a focused pass inside a phase.

A round may touch:

```text
docs
checker scripts
schemas
fixtures
one crate family
one service family
one UI surface
one ops/audit surface
```

Rounds should stay small enough to review safely.

### 1.3 Patch

A **patch** is one pasteable, testable unit of work.

Preferred patch shape:

```text
1. state purpose
2. list files touched
3. provide full paste-ready file contents or script
4. run focused checker/test
5. record result
6. update phase closeout only after evidence is green
```

### 1.4 Closeout

A **closeout** is the only document that may mark a phase complete.

Every closeout must say:

```text
what passed
what did not pass
what remains forbidden
what remains future-gated
what label is now safe to use
what label must not be inferred
```

---

## 2. Repository Scaffold Policy

The full scaffold may exist as empty placeholder files.

Empty placeholders are allowed only under this interpretation:

```text
placeholder exists
placeholder is empty
placeholder has no executable content
placeholder has no dependencies
placeholder has no build/run/deploy behavior
placeholder has no bridge behavior
placeholder has no token behavior
placeholder has no staking/liquidity behavior
placeholder has no external settlement behavior
placeholder has no user-facing claim
```

The existence of a placeholder file does not authorize the file’s future content.

The existence of `Cargo.toml` does not authorize Rust build behavior.

The existence of `package.json` does not authorize JS/TS build behavior.

The existence of `Anchor.toml` does not authorize Solana/Anchor behavior.

The existence of `programs/` does not authorize Solana program code.

The existence of `crablink-bridge-ui/` does not authorize a user-facing bridge UI.

The existence of `ops/deployment/` does not authorize deployment.

The existence of `tests/` does not authorize simulations that imply value movement.

The correct wording is:

```text
full inert scaffold exists
future implementation surfaces are reserved
future content requires explicit phase gate
```

Avoid this wording:

```text
runtime exists
bridge is started
ROX is active
Solana integration is active
devnet is authorized
UI is authorized
```

---

## 3. Global Invariants

### 3.1 Internal ROC truth

* [I-1] `svc-wallet` remains the only internal ROC economic mutation front-door.
* [I-2] `ron-ledger` remains durable internal ROC receipt, replay, conservation, and balance truth.
* [I-3] ROX Anchor never mutates internal ROC directly.
* [I-4] ROX Anchor never becomes accounting, policy, rewarder, gateway, storage, index, omnigate, or CrabLink authority.
* [I-5] Any future ROX → ROC issue path must route through `svc-wallet`.
* [I-6] Any future recovery issue path must route through `svc-wallet` and settle into `ron-ledger` truth.

### 3.2 Future bridge/finality posture

* [I-7] No proof package is finality by itself.
* [I-8] No single RPC response may become settlement truth.
* [I-9] No single observer may become settlement truth.
* [I-10] No coordinator may unilaterally finalize value movement.
* [I-11] No relayer may unilaterally finalize value movement.
* [I-12] No challenge window may be skipped for convenience.
* [I-13] No finality may be claimed before proof/challenge/finality lifecycle completes.
* [I-14] `FinalizedByDecisionGate` remains planning terminology until later runtime authorization.

### 3.3 CrabLink posture

* [I-15] CrabLink remains display, routing, user intent, and explicit confirmation only.
* [I-16] CrabLink must not construct bridge proofs.
* [I-17] CrabLink must not claim finality from cache.
* [I-18] CrabLink must not treat offline status as settlement truth.
* [I-19] CrabLink must not call direct mint/burn/settlement APIs.
* [I-20] CrabLink may display future bridge status only if backend-derived, stale-aware, and explicitly authorized.

### 3.4 Product-language posture

* [I-21] No doc or UI may imply live conversion, redemption, cash-out, swap, yield, staking, exchange access, public bridge, or guaranteed settlement.
* [I-22] High-risk words may appear only in forbidden, threat-modeled, anti-scope, or future-gated context.
* [I-23] Every phase must preserve the distinction between planning, skeleton, dry run, runtime, and public readiness.

### 3.5 Empty scaffold posture

* [I-24] Empty future files may exist as inert navigation surfaces.
* [I-25] Empty future files must not contain imports, dependencies, command bodies, code stubs, build metadata, scripts, or runtime instructions until authorized.
* [I-26] Placeholder scripts should remain non-executable unless the phase explicitly authorizes a checker.
* [I-27] Placeholder manifests should remain empty until the phase explicitly authorizes dependency or workspace metadata.
* [I-28] A scaffold checker must distinguish empty placeholder existence from executable content.

---

## 4. Status Labels

Use these labels only when the required gate passes.

### Draft

```text
DRAFT
```

Meaning:

```text
work is being written
not reviewed
not green
not parked
not authoritative
```

### Reviewed

```text
REVIEWED
```

Meaning:

```text
human/AI review completed
issues may remain
not necessarily parked
```

### Complete / Green / Parked

```text
COMPLETE / GREEN / PARKED
```

Meaning:

```text
phase checker passed
manual review passed
closeout exists
known blockers are recorded
next phase may be considered
```

### Forbidden label misuse

Do not use parked labels to imply:

```text
runtime authorized
bridge live
ROX live
Solana deployed
mainnet ready
users can convert
users can redeem
users can cash out
staking active
liquidity active
external settlement active
```

---

## 5. Phase Overview

```text
Phase 0   — Core Docs-Only Planning Gate
Phase 0A  — Full Inert Scaffold Gate
Phase 0B  — Scaffold-Aware Checker Gate
Phase 1   — Threat Model Review Gate
Phase 2   — State / Proof Design Gate
Phase 3   — Disabled Skeleton Decision Gate
Phase 4   — Local Non-Value Proof Engine Gate
Phase 5   — Coordination Layer Boundary Gate
Phase 6   — Private Non-Value Dry-Run Gate
Phase 7   — CrabLink Display-Only Status Gate
Phase 8   — Pre-Audit Hardening Gate
Phase 9   — Audit / Recovery Drill Gate
Phase 10  — Runtime Decision Gate
```

---

# Phase 0 — Core Docs-Only Planning Gate

## 0.1 Purpose

Create and review the five core docs and one static docs-only checker.

Phase 0 proves:

```text
scope is defined
North Star is present
anchor meaning is declared
safe labels are present
high-risk language is framed
runtime is not authorized
checker exists
checker is static-only
```

## 0.2 Authorized files

```text
README.md
LICENSE
.gitignore
.gitattributes
NOTICE
SECURITY.md
CONTRIBUTING.md
CODE_OF_CONDUCT.md
CHANGELOG.md

docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md

scripts/check-rox-anchor-docs-only.sh
```

## 0.3 Required work

1. Finalize `docs/00_IDB_ROX_ANCHOR.md`.
2. Finalize `docs/01_SCOPE_DECISION_GATE.md`.
3. Finalize `docs/02_THREAT_MODEL.md`.
4. Finalize `docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md`.
5. Finalize `docs/04_TESTPLAN_CHECKER.md`.
6. Create the Phase 0 checker.
7. Ensure checker is static-only.
8. Ensure root metadata does not contain build/runtime/deploy instructions.
9. Run checker.
10. Record Phase 0 closeout status.

## 0.4 Checker requirements

The checker must verify:

```text
five docs exist
checker exists
RO headers exist
North Star exists
anchor meaning exists
safe labels exist
context markers exist
forbidden positive claims absent
runtime-shaped content absent unless allowed as docs context
checker does not run build/deploy/RPC/wallet/mint/simulation commands
green label is printed only on pass
```

## 0.5 Forbidden during Phase 0

```text
runtime code
non-empty Cargo manifests
non-empty package manifests
Anchor program code
Solana account structs
coordinator runtime
relayer runtime
RPC proof service
CrabLink bridge UI
devnet scripts
mainnet scripts
mint/burn harnesses
staking/liquidity logic
exchange-facing logic
```

## 0.6 Exit label

```text
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

---

# Phase 0A — Full Inert Scaffold Gate

## 0A.1 Purpose

Create the full planned repository tree as inert placeholders.

Phase 0A proves:

```text
future project navigation is visible
all future surfaces are named
all future surfaces are empty
nothing executable was created
nothing operational was enabled
```

## 0A.2 Authorized file surfaces

All files in the full scaffold may exist as empty placeholders, including:

```text
Cargo.toml
package.json
Anchor.toml
docs/phase*/
specs/
schemas/
scripts/
crates/
programs/
crablink-bridge-ui/
tests/
ops/
audits/
```

## 0A.3 Strict empty-placeholder rules

Every future implementation-shaped file must be:

```text
zero-byte or comment-only if specifically allowed
non-executable
dependency-free
command-free
runtime-free
deployment-free
RPC-free
wallet-free
mint/burn-free
bridge-free
staking-free
liquidity-free
external-settlement-free
```

For maximum safety, prefer zero-byte placeholders.

## 0A.4 Required work

1. Create the complete tree.
2. Preserve existing five docs.
3. Preserve existing checker.
4. Do not overwrite non-empty files.
5. Do not chmod placeholder scripts.
6. Do not insert Rust code.
7. Do not insert TypeScript code.
8. Do not insert JSON schemas.
9. Do not insert Cargo dependencies.
10. Do not insert npm dependencies.
11. Record scaffold creation in `CHANGELOG.md` only if root metadata is authorized.
12. Run a scaffold inventory check.

## 0A.5 Acceptance gates

```text
[G-0A-1] full file tree exists
[G-0A-2] five docs are preserved
[G-0A-3] checker is preserved
[G-0A-4] future placeholder files are empty
[G-0A-5] placeholder scripts are not executable unless explicitly authorized
[G-0A-6] no dependencies exist in placeholder manifests
[G-0A-7] no build behavior exists
[G-0A-8] no deploy behavior exists
[G-0A-9] no wallet/RPC/mint/burn/bridge behavior exists
[G-0A-10] no user-facing bridge UI behavior exists
```

## 0A.6 Exit label

```text
ROX Anchor Phase 0A — Full Inert Scaffold Gate:
COMPLETE / GREEN / PARKED.
```

This label means only that the scaffold exists inertly.

It does not authorize implementation.

---

# Phase 0B — Scaffold-Aware Checker Gate

## 0B.1 Purpose

Update the checker model so it understands the difference between:

```text
forbidden executable implementation
```

and:

```text
empty inert placeholder
```

Phase 0B is necessary because the full scaffold intentionally contains future-shaped paths.

## 0B.2 Required checker changes

The checker must classify files as:

```text
required core docs
required checker
allowed root metadata
allowed empty placeholder
forbidden non-empty future implementation file
forbidden executable placeholder script
forbidden dependency manifest
forbidden secret/key/config
forbidden build/deploy/runtime instruction
```

## 0B.3 Placeholder checks

The checker must verify that future placeholders remain empty.

High-risk placeholder classes:

```text
Cargo.toml
package.json
Anchor.toml
crates/**/*.rs
programs/**/*.rs
crablink-bridge-ui/**/*.ts
crablink-bridge-ui/**/*.tsx
crablink-bridge-ui/**/*.json
schemas/**/*.json
tests/**/*.rs
scripts/*.sh
ops/deployment/*.md
```

Allowed exceptions:

```text
five core docs may be non-empty
Phase 0 checker may be non-empty
root metadata may be non-empty if non-runtime
future docs may become non-empty only after their phase gate
```

## 0B.4 Forbidden content scanner

The checker must fail on non-empty placeholders containing:

```text
use anchor_lang
#[program]
declare_id!
#[derive(Accounts)]
solana_program
RpcClient
send_transaction
mint_to
burn
transfer_checked
cargo build
cargo test
anchor build
anchor deploy
npm install
npm run
solana program deploy
spl-token
private key
seed phrase
wallet file
mainnet ready
bridge live
cash out
redeem
staking active
liquidity active
```

## 0B.5 Acceptance gates

```text
[G-0B-1] checker passes with full inert scaffold
[G-0B-2] checker fails if placeholder Rust file receives code
[G-0B-3] checker fails if placeholder manifest receives dependencies
[G-0B-4] checker fails if placeholder script becomes executable without authorization
[G-0B-5] checker fails if bridge/live/cash-out language appears outside context
[G-0B-6] checker prints scaffold-specific non-authorization reminder
```

## 0B.6 Exit label

```text
ROX Anchor Phase 0B — Scaffold-Aware Checker Gate:
COMPLETE / GREEN / PARKED.
```

This label authorizes the scaffold-aware checker only.

---

# Phase 1 — Threat Model Review Gate

## 1.1 Purpose

Expand and review the threat model before any design or skeleton work proceeds.

Phase 1 proves:

```text
attacker classes are complete enough
risk register exists
threat categories are ranked
mitigation requirements are written
runtime blockers are visible
```

## 1.2 Authorized files

```text
docs/phase1-threat-review/00_PHASE1_THREAT_MODEL_REVIEW.md
docs/phase1-threat-review/01_RISK_REGISTER.md
docs/phase1-threat-review/02_ATTACKER_MODEL_EXPANSION.md
docs/phase1-threat-review/03_MITIGATION_REQUIREMENTS.md
docs/phase1-threat-review/04_PHASE1_CLOSEOUT.md
scripts/check-phase1-threat-review.sh
audits/phase1-threat-review.md
```

## 1.3 Required work

1. Expand attacker classes.
2. Build a risk register.
3. Assign severity levels.
4. Define required mitigations.
5. Map mitigations to later phases.
6. Identify runtime blockers.
7. Create Phase 1 checker.
8. Create manual reviewer checklist.
9. Record audit/review notes.
10. Produce closeout.

## 1.4 Required threat categories

Phase 1 must cover:

```text
coordinator compromise
relayer compromise
single-RPC failure
stale/forked RPC
observer compromise
proof package replay
cross-domain replay
cross-direction replay
devnet/mainnet confusion
program-id spoofing
mint spoofing
nonce replay
operation-id replay
idempotency misuse
challenge griefing
halt abuse
recovery abuse
upgrade authority compromise
key custody failure
CrabLink stale display
product-language creep
hidden implementation drift
verifiable-build failure
internal ROC boundary bypass
external mint/burn lifecycle abuse
```

## 1.5 Forbidden during Phase 1

```text
code
schemas used as runtime
fixtures used as tests
coordinator implementation
relayer implementation
RPC client implementation
Solana program code
CrabLink bridge UI
devnet dry run
runtime simulation
```

## 1.6 Acceptance gates

```text
[G-1-1] risk register exists
[G-1-2] attacker model expansion exists
[G-1-3] mitigation requirements exist
[G-1-4] every Critical risk has blocker or mitigation path
[G-1-5] no Critical risk is silently accepted
[G-1-6] product-language risks are explicitly listed
[G-1-7] hidden implementation drift is listed
[G-1-8] Phase 1 checker passes
[G-1-9] Phase 1 closeout exists
```

## 1.7 Exit label

```text
ROX Anchor Phase 1 — Threat Model Review Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

---

# Phase 2 — State / Proof Design Gate

## 2.1 Purpose

Define the conceptual state machine, proof package, challenge window, validation posture, halt/recovery model, and CrabLink display status before skeleton work begins.

Phase 2 proves:

```text
state machine is reviewable
proof package fields are complete enough
challenge/finality lifecycle is explicit
recovery paths are bounded
CrabLink labels are conservative
schemas/specs are design artifacts only
```

## 2.2 Authorized files

```text
docs/phase2-state-proof-design/*
specs/bridge-operation-identity.md
specs/proof-package.md
specs/state-machine.md
specs/challenge-window.md
specs/rpc-quorum.md
specs/recovery-cases.md
specs/crablink-status-labels.md
schemas/*.schema.json
scripts/check-phase2-state-proof-design.sh
audits/phase2-state-proof-review.md
```

## 2.3 Required work

1. Define bridge operation identity.
2. Define idempotency key semantics.
3. Define nonce semantics.
4. Define direction binding.
5. Define source/target domain binding.
6. Define state machine.
7. Define forbidden states.
8. Define transition rules.
9. Define proof package fields.
10. Define challenge-window semantics.
11. Define proof validation output states.
12. Define RPC quorum requirements.
13. Define recovery cases.
14. Define CrabLink display labels.
15. Draft JSON schemas as non-runtime validation artifacts.
16. Create Phase 2 checker.
17. Produce Phase 2 closeout.

## 2.4 Required proof dimensions

Every proof package design must bind:

```text
schema_version
source_domain
target_domain
direction
operation_id
idempotency_key
nonce
source_account
target_account
cluster
program_id
mint
token_account
transaction_signature
slot
block_time
commitment_level
rpc_quorum_observations
observer_set
observer_attestations
challenge_window_open_time
challenge_window_close_time
challenge_status
finality_decision_reference
halt_status
recovery_status
created_at
expires_at
```

## 2.5 Required state labels

Allowed conceptual labels:

```text
Draft
Requested
Observed
ProofPackaged
EvidenceInsufficient
QuorumDisputed
ChallengeOpen
Challenged
ChallengeRejected
ChallengeAccepted
Expired
FinalityEligible
FinalizedByDecisionGate
Failed
RecoveryQueued
Recovered
HaltRequested
Halted
ResumeEligible
Abandoned
```

Forbidden optimistic labels:

```text
InstantComplete
Guaranteed
Redeemed
Swapped
CashedOut
LiveBridge
MainnetReady
Converted
Withdrawable
Deposited
SettledByClient
FinalFromCache
RpcFinal
CoordinatorFinal
RelayerFinal
```

## 2.6 Acceptance gates

```text
[G-2-1] state-machine spec exists
[G-2-2] proof-package spec exists
[G-2-3] challenge-window spec exists
[G-2-4] RPC quorum spec exists
[G-2-5] recovery-cases spec exists
[G-2-6] CrabLink status-label spec exists
[G-2-7] schemas exist as non-runtime artifacts
[G-2-8] schemas contain no runtime claim
[G-2-9] no state implies instant or guaranteed completion
[G-2-10] no proof package implies settlement by itself
[G-2-11] Phase 2 checker passes
[G-2-12] Phase 2 closeout exists
```

## 2.7 Exit label

```text
ROX Anchor Phase 2 — State / Proof Design Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

---

# Phase 3 — Disabled Skeleton Decision Gate

## 3.1 Purpose

Decide whether to allow non-value-bearing disabled skeleton files to become non-empty.

Phase 3 is the first phase where implementation-shaped files may receive content.

Phase 3 still does not authorize runtime.

## 3.2 Authorized file surfaces

Potentially authorized after the Phase 3 decision:

```text
Cargo.toml
package.json
Anchor.toml

crates/rox-anchor-core/**
crates/rox-anchor-proof/**
crates/rox-anchor-cli/**

programs/rox-anchor/**
crablink-bridge-ui/**
```

The exact authorized subset must be listed in:

```text
docs/phase3-disabled-skeleton/01_DISABLED_SKELETON_SCOPE.md
```

## 3.3 Skeleton rules

Skeleton code must be:

```text
disabled-by-default
non-value-bearing
local-only
non-deployed
non-RPC
non-wallet
non-minting
non-burning
non-settling
non-user-facing
feature-gated
kill-switchable
headered with RO comments
```

## 3.4 Manifest rules

Manifests may only contain:

```text
workspace metadata
package metadata
no network runtime dependencies unless justified
no Anchor deploy config
no default features enabling runtime
no scripts that deploy or run services
```

## 3.5 First skeleton order

Use this order:

```text
1. rox-anchor-core
2. rox-anchor-proof
3. rox-anchor-cli
4. tests/vectors
5. scripts/check-phase3-disabled-skeleton.sh
6. only then consider programs/ or UI skeletons
```

## 3.6 Forbidden during Phase 3

```text
real RPC calls
real wallet calls
mint/burn instructions with behavior
deployed program IDs
devnet deployment
mainnet deployment
user-facing BridgeIntentPage behavior
coordinator service loop
relayer service loop
proof service listener
```

## 3.7 Acceptance gates

```text
[G-3-1] disabled skeleton decision doc exists
[G-3-2] exact allowed file list exists
[G-3-3] no file outside list is populated
[G-3-4] every non-empty code file has RO header
[G-3-5] every code path is disabled by default
[G-3-6] no manifest enables runtime behavior
[G-3-7] no script deploys, runs RPC, or mutates wallets
[G-3-8] no Solana program deploy path exists
[G-3-9] checker verifies non-value-bearing status
[G-3-10] closeout exists
```

## 3.8 Exit label

```text
ROX Anchor Phase 3 — Disabled Skeleton Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize devnet, runtime, or user-facing behavior.

---

# Phase 4 — Local Non-Value Proof Engine Gate

## 4.1 Purpose

Build a local proof package validator that works only on fixtures and local data.

Phase 4 proves:

```text
proof package validation can reject unsafe evidence
state transitions can be tested locally
replay/domain binding can be tested locally
RPC quorum logic can be modeled without calling RPC
challenge windows can be modeled without value movement
```

## 4.2 Authorized file surfaces

```text
crates/rox-anchor-core/**
crates/rox-anchor-proof/**
crates/rox-anchor-cli/**
tests/vectors/**
tests/unit/**
scripts/check-phase4-local-proof-engine.sh
docs/phase4-local-proof-engine/**
```

## 4.3 Implementation order

```text
Round 4.1 — core type definitions
Round 4.2 — operation identity and nonce validation
Round 4.3 — state machine transition validation
Round 4.4 — proof package parser / validator
Round 4.5 — replay rejection
Round 4.6 — quorum evaluator using fixtures only
Round 4.7 — challenge-window evaluator
Round 4.8 — recovery-case classifier
Round 4.9 — CLI read-only inspection commands
Round 4.10 — unit tests and vector tests
Round 4.11 — closeout
```

## 4.4 Required rejections

The local proof engine must reject:

```text
missing schema version
unsupported schema version
missing source/target domain
invalid direction
missing operation_id
reused nonce
idempotency key treated as authority
cluster mismatch
program mismatch
mint mismatch
token account mismatch
single-RPC finality
RPC disagreement
commitment downgrade
challenge window skipped
halted state finalization
recovery bypass
proof package claiming settlement
```

## 4.5 CLI limits

The CLI may support:

```text
check
proof inspect
proof validate-local
status explain
recover classify
halt explain
```

The CLI must not support:

```text
deploy
send transaction
mint
burn
settle
bridge
cash out
stake
provide liquidity
connect wallet for authority
```

## 4.6 Acceptance gates

```text
[G-4-1] local proof engine compiles if code phase authorized
[G-4-2] no network access required
[G-4-3] no wallet access required
[G-4-4] no Solana CLI required
[G-4-5] valid fixture validates as evidence only
[G-4-6] invalid fixtures fail closed
[G-4-7] replay fixtures rejected
[G-4-8] quorum disagreement rejected
[G-4-9] challenge-window violations rejected
[G-4-10] no output says settlement complete
[G-4-11] Phase 4 checker passes
[G-4-12] closeout exists
```

## 4.7 Exit label

```text
ROX Anchor Phase 4 — Local Non-Value Proof Engine Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

---

# Phase 5 — Coordination Layer Boundary Gate

## 5.1 Purpose

Define and possibly skeletonize coordinator, relayer, observer, and RPC proof-service boundaries without authorizing runtime or finality authority.

Phase 5 proves:

```text
coordinator cannot finalize
relayer cannot finalize
RPC proof service cannot become settlement truth
observer evidence is bounded
readiness does not imply bridge readiness
redaction exists
queues are bounded
```

## 5.2 Authorized file surfaces

Potentially authorized by Phase 5 decision:

```text
crates/rox-anchor-coordinator/**
crates/rox-anchor-relayer/**
crates/rox-anchor-rpc-proof/**
docs/phase5-coordination-layer/**
scripts/check-phase5-coordination-layer.sh
tests/integration/coordinator_relayer_boundary.rs
tests/chaos/coordinator_stale_evidence.rs
tests/chaos/relayer_retry_storm.rs
```

## 5.3 Implementation order

```text
Round 5.1 — coordinator boundary doc
Round 5.2 — relayer boundary doc
Round 5.3 — RPC quorum boundary doc
Round 5.4 — observer set boundary doc
Round 5.5 — config structures disabled by default
Round 5.6 — readiness structures that do not imply bridge readiness
Round 5.7 — redaction helpers
Round 5.8 — bounded queue model
Round 5.9 — evidence assembly skeleton
Round 5.10 — boundary tests
Round 5.11 — closeout
```

## 5.4 Hard boundaries

Coordinator may:

```text
collect evidence
normalize evidence
queue evidence
classify evidence status
suggest candidate next state
emit audit-friendly records
```

Coordinator must not:

```text
mint
burn
issue ROC
settle
finalize unilaterally
bypass challenge window
override halt
call wallet mutation
claim public readiness
```

Relayer may:

```text
submit authorized messages in future phases
retry idempotently
record submission evidence
report failure
```

Relayer must not:

```text
create finality
invent receipts
mint/burn without authorization
retry into duplicate operation
submit to mainnet without gate
```

RPC proof service may:

```text
collect observations
compare RPC responses
report quorum status
report disagreement
```

RPC proof service must not:

```text
treat single RPC as truth
treat commitment as finality by itself
call settlement complete
become mint/issue authority
```

## 5.5 Acceptance gates

```text
[G-5-1] coordinator boundary doc exists
[G-5-2] relayer boundary doc exists
[G-5-3] RPC quorum boundary doc exists
[G-5-4] observer boundary doc exists
[G-5-5] all configs disabled by default
[G-5-6] no network loop runs by default
[G-5-7] no service claims bridge readiness
[G-5-8] boundary tests prove no unilateral finality
[G-5-9] redaction helpers exist
[G-5-10] Phase 5 checker passes
[G-5-11] closeout exists
```

## 5.6 Exit label

```text
ROX Anchor Phase 5 — Coordination Layer Boundary Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize live coordinator/relayer/runtime behavior.

---

# Phase 6 — Private Non-Value Dry-Run Gate

## 6.1 Purpose

Allow private, non-value, non-public dry-run behavior only if prior gates are complete.

Phase 6 proves:

```text
dry-run mechanics can be exercised without value
cluster/program/mint binding is enforced
deployment dry-run rules are clear
devnet/mainnet confusion is blocked
public readiness is not implied
```

## 6.2 Authorized file surfaces

```text
docs/phase6-private-nonvalue-devnet/**
ops/deployment/devnet-dry-run.md
tests/integration/local_nonvalue_roc_to_rox.rs
tests/integration/local_nonvalue_rox_to_roc.rs
scripts/check-phase6-private-nonvalue-devnet.sh
```

Potential code surfaces require explicit Phase 6 decision:

```text
crates/rox-anchor-rpc-proof/**
crates/rox-anchor-coordinator/**
crates/rox-anchor-relayer/**
programs/rox-anchor/**
```

## 6.3 Non-value rules

Dry run must be:

```text
private
non-value-bearing
non-public
non-exchange-facing
non-user-facing
not mainnet
not public beta
not real bridge
not real ROX
not real cash-out
not staking
not liquidity
```

## 6.4 Required binding checks

Every dry run must bind:

```text
cluster
program_id
mint
direction
operation_id
nonce
commitment policy
challenge window
halt status
source domain
target domain
```

## 6.5 Forbidden during Phase 6

```text
mainnet deployment
public devnet announcement
toy-value public demo
real user funds
external settlement
exchange listing
cash-out language
staking
liquidity
public bridge UI
```

## 6.6 Acceptance gates

```text
[G-6-1] private non-value decision doc exists
[G-6-2] devnet non-value rules exist
[G-6-3] cluster/program/mint binding plan exists
[G-6-4] deployment dry-run rules exist
[G-6-5] all configs prove non-value status
[G-6-6] dry run cannot touch real wallet or ledger value
[G-6-7] public-language scanner passes
[G-6-8] Phase 6 checker passes
[G-6-9] closeout exists
```

## 6.7 Exit label

```text
ROX Anchor Phase 6 — Private Non-Value Dry-Run Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize public bridge behavior.

---

# Phase 7 — CrabLink Display-Only Status Gate

## 7.1 Purpose

Define and possibly build backend-derived, stale-aware, display-only CrabLink bridge-status surfaces.

Phase 7 proves:

```text
CrabLink can show future status safely
status is backend-derived
cache is not finality
offline is uncertainty
labels are conservative
user intent does not mutate value
```

## 7.2 Authorized file surfaces

```text
docs/phase7-crablink-display/**
specs/crablink-status-labels.md
schemas/status-response.schema.json
crablink-bridge-ui/**
tests/integration/crablink_status_display.rs
scripts/check-phase7-crablink-display.sh
```

## 7.3 UI implementation order

```text
Round 7.1 — status label spec
Round 7.2 — backend-derived status API spec
Round 7.3 — stale/offline label spec
Round 7.4 — TypeScript types
Round 7.5 — status label helper
Round 7.6 — stale status helper
Round 7.7 — BridgeStatusPanel display-only component
Round 7.8 — warning panel
Round 7.9 — intent page shell, no mutation
Round 7.10 — tests
Round 7.11 — closeout
```

## 7.4 Allowed labels

```text
Not available
Planning only
Pending observation
Evidence incomplete
Challenge open
Challenged
Expired
Failed
Halted
Recovery review required
Stale status
Offline — status unknown
```

## 7.5 Forbidden labels

```text
Converted
Redeemed
Cashed out
Swap complete
Bridge complete
Settlement complete
Final from cache
Mainnet ready
Live bridge
```

## 7.6 Forbidden UI behavior

```text
client-side proof construction
client-side finality claim
cache-only bridge completion
offline finality display
direct Solana RPC authority
direct wallet mutation
direct ledger mutation
mint/burn command
settlement command
```

## 7.7 Acceptance gates

```text
[G-7-1] backend-derived status API spec exists
[G-7-2] stale/failure labels doc exists
[G-7-3] no-client-finality UX doc exists
[G-7-4] UI package does not call wallet mutation
[G-7-5] UI package does not call Solana RPC as authority
[G-7-6] forbidden labels test fails on unsafe language
[G-7-7] stale status test passes
[G-7-8] Phase 7 checker passes
[G-7-9] closeout exists
```

## 7.8 Exit label

```text
ROX Anchor Phase 7 — CrabLink Display-Only Status Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize user-facing bridge execution.

---

# Phase 8 — Pre-Audit Hardening Gate

## 8.1 Purpose

Prepare the repo for formal audit/recovery drills.

Phase 8 proves:

```text
builds can be reproduced
source revisions bind to artifacts
dependency locks are recorded
keys and authorities are documented
chaos/failure drills are planned
open risks are visible
```

## 8.2 Authorized file surfaces

```text
docs/phase8-preaudit-hardening/**
ops/release/**
ops/runbooks/**
audits/phase8-preaudit-review.md
audits/findings/**
scripts/check-phase8-preaudit-hardening.sh
scripts/check-reproducible-build-evidence.sh
scripts/check-no-value-bearing-config.sh
```

## 8.3 Required work

1. Reproducible build plan.
2. Artifact hash plan.
3. Source revision binding.
4. Dependency lock evidence.
5. Auditor reproduction path.
6. Key custody plan.
7. Key rotation plan.
8. Emergency halt runbook.
9. Recovery runbook.
10. Upgrade runbook.
11. Incident response runbook.
12. Chaos/failure drill plan.
13. Pre-audit review record.
14. Open/resolved/accepted risk registers.

## 8.4 Required drills planned

```text
RPC equivocation
coordinator compromise
relayer compromise
observer disagreement
challenge griefing
halt requested during pending finality
stuck challenge
failed recovery
key compromise
lost key
upgrade mismatch
reproducible build mismatch
CrabLink stale display incident
```

## 8.5 Acceptance gates

```text
[G-8-1] reproducible build plan exists
[G-8-2] key custody plan exists
[G-8-3] chaos/failure drill plan exists
[G-8-4] halt runbook exists
[G-8-5] recovery runbook exists
[G-8-6] upgrade runbook exists
[G-8-7] incident response runbook exists
[G-8-8] findings registers exist
[G-8-9] no value-bearing config is enabled
[G-8-10] Phase 8 checker passes
[G-8-11] closeout exists
```

## 8.6 Exit label

```text
ROX Anchor Phase 8 — Pre-Audit Hardening Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize deployment.

---

# Phase 9 — Audit / Recovery Drill Gate

## 9.1 Purpose

Complete formal audit/recovery drills before any runtime decision.

Phase 9 proves:

```text
audit scope was reviewed
critical findings are resolved or explicitly blocked
recovery drills were performed
halt drills were performed
upgrade drills were performed
reproducible build evidence was checked
runtime blockers are known
```

## 9.2 Authorized file surfaces

```text
docs/phase9-audit-recovery-drills/**
audits/phase9-audit-report.md
audits/findings/**
ops/release/**
ops/runbooks/**
scripts/check-phase9-audit-recovery-drills.sh
tests/chaos/**
```

## 9.3 Required audit scope

Audit must cover:

```text
state machine
proof package validation
replay protection
RPC quorum
challenge window
halt behavior
recovery behavior
coordinator boundary
relayer boundary
RPC proof-service boundary
CrabLink display boundary
program/account model if present
schemas
configs
build/release process
key custody
upgrade authority
forbidden language
hidden implementation drift
```

## 9.4 Required drill records

```text
recovery drill record
halt drill record
upgrade drill record
key rotation drill record
RPC equivocation drill record
challenge griefing drill record
stale UI incident drill record
```

## 9.5 Acceptance gates

```text
[G-9-1] audit scope exists
[G-9-2] audit report exists
[G-9-3] open findings register exists
[G-9-4] resolved findings register exists
[G-9-5] accepted risk register exists
[G-9-6] no Critical finding is unresolved without blocker status
[G-9-7] recovery drill record exists
[G-9-8] halt drill record exists
[G-9-9] upgrade drill record exists
[G-9-10] reproducible build evidence exists
[G-9-11] Phase 9 checker passes
[G-9-12] closeout exists
```

## 9.6 Exit label

```text
ROX Anchor Phase 9 — Audit / Recovery Drill Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime by itself.

It only permits a later runtime decision gate to be considered.

---

# Phase 10 — Runtime Decision Gate

## 10.1 Purpose

Decide whether any runtime or value-bearing path may begin.

This is not an automatic implementation phase.

This is a decision gate.

## 10.2 Authorized files

```text
docs/phase10-runtime-decision/00_PHASE10_RUNTIME_DECISION_GATE.md
docs/phase10-runtime-decision/01_RUNTIME_AUTHORIZATION_SCOPE.md
docs/phase10-runtime-decision/02_VALUE_BEARING_LIMITS.md
docs/phase10-runtime-decision/03_PUBLIC_READINESS_BOUNDARY.md
docs/phase10-runtime-decision/04_PHASE10_CLOSEOUT.md
scripts/check-phase10-runtime-decision.sh
```

## 10.3 Required decision questions

Phase 10 must answer:

```text
Is any runtime authorized?
Which runtime surfaces are authorized?
Which surfaces remain forbidden?
Is value-bearing behavior authorized?
What caps apply?
What accounts are allowed?
What cluster is allowed?
What program ID is allowed?
What mint is allowed?
What challenge window applies?
What halt switch applies?
What recovery path applies?
What public language is allowed?
What user-facing behavior is allowed?
What audit findings remain?
What rollback path exists?
```

## 10.4 Runtime authorization scope

If runtime is authorized, the scope must state:

```text
authorized components
authorized commands
authorized environments
authorized accounts
authorized limits
authorized operators
authorized test windows
authorized user visibility
authorized rollback
authorized halt
```

## 10.5 Value-bearing limits

If value-bearing behavior is authorized, the limits must state:

```text
maximum value
maximum accounts
maximum operation size
maximum daily volume
maximum supply delta
challenge window duration
manual review thresholds
halt thresholds
recovery thresholds
audit monitoring requirements
public communication limits
```

## 10.6 Public readiness boundary

The public readiness boundary must state whether these are allowed:

```text
public beta wording
bridge wording
conversion wording
redeem wording
cash-out wording
staking wording
liquidity wording
exchange wording
mainnet wording
ROX live wording
```

Default answer is no unless explicitly authorized.

## 10.7 Acceptance gates

```text
[G-10-1] runtime decision doc exists
[G-10-2] authorization scope exists
[G-10-3] value-bearing limits exist, even if answer is "not authorized"
[G-10-4] public-readiness boundary exists
[G-10-5] Phase 9 closeout is referenced
[G-10-6] unresolved findings are listed
[G-10-7] kill switch exists if runtime authorized
[G-10-8] rollback exists if runtime authorized
[G-10-9] public language is reviewed
[G-10-10] Phase 10 checker passes
[G-10-11] closeout exists
```

## 10.8 Possible outcomes

```text
Outcome A — Runtime rejected.
Outcome B — Runtime deferred pending fixes.
Outcome C — Non-value runtime authorized only.
Outcome D — Private limited value-bearing pilot authorized.
Outcome E — Public value-bearing runtime authorized with caps.
```

Outcome E requires the highest level of evidence and must not be inferred from any previous phase.

## 10.9 Exit labels

If rejected:

```text
ROX Anchor Phase 10 — Runtime Decision Gate:
COMPLETE / GREEN / PARKED.
Runtime authorization: REJECTED.
```

If deferred:

```text
ROX Anchor Phase 10 — Runtime Decision Gate:
COMPLETE / GREEN / PARKED.
Runtime authorization: DEFERRED.
```

If limited non-value runtime is authorized:

```text
ROX Anchor Phase 10 — Runtime Decision Gate:
COMPLETE / GREEN / PARKED.
Runtime authorization: NON-VALUE LIMITED ONLY.
```

If private limited value pilot is authorized:

```text
ROX Anchor Phase 10 — Runtime Decision Gate:
COMPLETE / GREEN / PARKED.
Runtime authorization: PRIVATE LIMITED VALUE PILOT ONLY.
```

If public runtime is authorized:

```text
ROX Anchor Phase 10 — Runtime Decision Gate:
COMPLETE / GREEN / PARKED.
Runtime authorization: PUBLIC VALUE-BEARING RUNTIME AUTHORIZED WITH CAPS.
```

The exact authorization scope must be attached.

---

## 6. Cross-Phase Build Order

Use this order for major implementation work after docs gates:

```text
1. docs and decision gate
2. checker for that phase
3. specs
4. schemas
5. fixtures
6. core types
7. proof validation
8. local CLI diagnostics
9. service boundary skeletons
10. Solana/Anchor program skeleton, if authorized
11. CrabLink display-only UI, if authorized
12. integration tests
13. chaos tests
14. ops runbooks
15. audit records
16. closeout
```

Do not reverse this order.

Especially do not start with:

```text
programs/
coordinator runtime
relayer runtime
CrabLink UI
deployment scripts
mainnet readiness
```

---

## 7. Per-Artifact Build Ownership

### Docs

Docs define authority and boundaries.

Docs must include:

```text
North Star
RO headers
anchor meaning
invariants
anti-scope
acceptance gates
review checklist
non-authorization reminder
```

### Specs

Specs define conceptual behavior.

Specs must not imply active runtime.

### Schemas

Schemas define validation shapes.

Schemas must not imply settlement authority.

### Scripts

Scripts must be static/local unless a later phase authorizes more.

Phase checker scripts must not run deployment, RPC, wallet, mint, burn, settlement, coordinator, or relayer commands unless the phase explicitly authorizes a non-value local check.

### Crates

Rust crates must start as disabled skeletons.

They must use safe Rust.

They must include RO headers.

They must keep authority boundaries explicit.

### Programs

Solana/Anchor program surfaces are highest risk.

They require explicit authorization after threat model, proof design, disabled skeleton gate, audit preparation, and later runtime decision.

### CrabLink UI

CrabLink UI must remain display-only unless later authorization says otherwise.

Even then, user intent collection must not become authority.

### Tests

Tests must distinguish:

```text
fixture validation
local non-value validation
integration dry-run
chaos drill
value-bearing runtime
```

### Ops

Ops runbooks must not create deployment authority by themselves.

### Audits

Audit records must preserve findings and blockers.

Accepted risk must be explicit.

---

## 8. Standard Round Template

Use this for each round:

```text
Round <phase>.<round> — <Name>

Purpose:
  <one paragraph>

Files:
  <exact files touched>

Allowed:
  <allowed actions>

Forbidden:
  <forbidden actions>

Patch:
  <what will be changed>

Test:
  <focused checker/test>

Exit:
  <what must be true before moving on>
```

---

## 9. Standard Closeout Template

Every closeout should use:

```markdown
# ROX Anchor Phase <N> Closeout

RO:WHAT — Records the completed Phase <N> gate.
RO:WHY — Prevents future sessions from confusing this gate with runtime authorization.
RO:INTERACTS — <files/surfaces>
RO:INVARIANTS — <boundaries>
RO:SECURITY — <forbidden scope>
RO:TEST — <checker/test evidence>

## 0. Safe label

<phase label>

## 1. What this phase proved

<proof list>

## 2. What this phase did not prove

<non-authorization list>

## 3. Files reviewed

<file list>

## 4. Tests/checkers run

<commands/results>

## 5. Known open issues

<issues>

## 6. Next allowed phase

<next phase>

## 7. Still forbidden

<forbidden list>
```

---

## 10. Global Forbidden Scope Until Explicit Runtime Gate

Unless a later gate explicitly authorizes otherwise, the following remain forbidden:

```text
ROX live claim
Solana live claim
bridge live claim
mainnet ready claim
public bridge beta
user-facing bridge execution
external settlement
exchange-facing behavior
cash out
redeem
swap
staking
yield
liquidity
client-side settlement
client-side proof construction
cache finality
single-RPC settlement truth
single-observer settlement truth
coordinator unilateral finality
relayer unilateral finality
direct ledger mutation
internal issue outside svc-wallet
manual balance mutation
hidden mint path
hidden issue path
```

---

## 11. Recommended Immediate Next Steps

Current safest next sequence:

```text
1. Confirm full inert scaffold exists.
2. Confirm future placeholders are empty.
3. Update checker into scaffold-aware Phase 0B checker.
4. Run scaffold-aware checker.
5. Mark Phase 0A / 0B only if green.
6. Begin Phase 1 threat review docs.
```

Do not populate code yet unless moving into an explicitly authorized skeleton phase.

Do not populate manifests with dependencies yet.

Do not chmod future placeholder scripts yet.

Do not write Solana/Anchor code yet.

Do not write CrabLink bridge UI logic yet.

---

## 12. Final Buildplan Thesis

The build path is:

```text
docs
→ inert scaffold
→ scaffold-aware checker
→ threat review
→ state/proof design
→ disabled skeleton
→ local non-value proof engine
→ coordination boundaries
→ private non-value dry run
→ display-only CrabLink status
→ pre-audit hardening
→ audit/recovery drills
→ runtime decision
```

The build path is not:

```text
scaffold
→ runtime
→ bridge
→ public beta
```

The scaffold gives the project shape.

The gates give the project authority.

Until a later explicit decision says otherwise:

```text
ROX Anchor is planning, not runtime.
Internal ROC truth stays with svc-wallet + ron-ledger.
Bridge remains docs / threat-model / decision-gate only.
```

