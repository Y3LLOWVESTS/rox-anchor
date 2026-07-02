Use this as the fixed version of:

```text
docs/00_IDB_ROX_ANCHOR.md
```

I corrected the stale scaffold and first-patch sections from the uploaded IDB, which still listed the older many-document structure instead of the final five-doc Phase 0 bundle. 

---

title: ROX Anchor Invariant-Driven Blueprint
version: 0.3.0
status: draft
last-updated: 2026-07-01
audience: contributors, auditors, reviewers, future bridge planners
scope: docs-only / threat-model / decision-gate
-----------------------------------------------

# 🪓 ROX Anchor Invariant-Driven Blueprint

*A docs-first fortress blueprint for future ROC ↔ ROX anchoring work*

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.

RO:WHAT — Defines the invariant-driven blueprinting method and Phase 0 doctrine for the separate `rox-anchor` planning repository.
RO:WHY — Allows serious future ROX-anchor planning without authorizing bridge runtime, Solana runtime, external settlement, staking, liquidity, or user-facing bridge behavior.
RO:INTERACTS — RustyOnions Internal ROC, CrabLink, svc-wallet, ron-ledger, QuickChain future proof infrastructure, future ROX/Solana/Anchor documents.
RO:INVARIANTS — Internal ROC truth remains with svc-wallet + ron-ledger; ROX-anchor is planning, not runtime; bridge scope remains docs / threat-model / decision-gate only.
RO:SECURITY — No fake finality, no client-side settlement, no direct ROC mutation, no public bridge path, no value-bearing devnet, no staking, no liquidity, no external settlement.
RO:TEST — `bash scripts/check-rox-anchor-docs-only.sh`.

Anchor meaning used in this document: ROX-anchor planning repo.

ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

---

## 0. Scope Clarification

This document governs **ROX Anchor Phase 0**.

Phase 0 is intentionally limited to:

```text
five core planning docs
one static docs-only checker
harmless root metadata
```

Phase 0 does **not** authorize runtime, skeleton code, deployment, mint/burn behavior, bridge behavior, staking, liquidity, external settlement, or user-facing bridge behavior.

Important clarification:

```text
Outside Phase 0 does not mean forbidden forever.
```

Implementation-shaped bridge/Solana/ROX files and directories are outside the current Phase 0 scaffold because Phase 0 is a planning and checker gate. A later explicit decision gate may authorize disabled, non-value-bearing skeleton work or other implementation-shaped surfaces.

Use this distinction everywhere:

```text
Correct:
  outside Phase 0
  not authorized in current phase
  requires later explicit decision gate

Avoid:
  forbidden forever
  impossible
  never allowed in any future phase
```

---

## 1. Definition

**Invariant-Driven Blueprinting (IDB)** is the required documentation style for `rox-anchor`.

For this repository, IDB means every major document must clearly separate:

1. **Invariants (MUSTs)** — Non-negotiable laws that must never be violated.
2. **Design Principles (SHOULDs)** — Guiding heuristics and rationale.
3. **Implementation Patterns (HOW)** — Planning mechanics, document structure, future-safe sketches, and checker patterns.
4. **Acceptance Gates (PROOF)** — Tests, scanners, review gates, and checklist evidence required before status can advance.
5. **Anti-Scope (Forbidden / Outside Current Phase)** — What is not allowed in the current phase, to prevent bridge/runtime/scope drift.

In `rox-anchor`, **Implementation Patterns** do not mean active runtime implementation unless a later explicit decision gate authorizes it.

At Phase 0, all implementation patterns are limited to:

```text
docs
blueprints
threat models
state-machine sketches
DTO/proof sketches
checker scripts
decision gates
review notes inside the five-doc set
```

They do **not** include Phase 0 runtime artifacts such as:

```text
Solana program code
Anchor instruction code
mint/burn runtime
relayer runtime
coordinator runtime
RPC proof service runtime
CrabLink bridge UI
devnet deployment scripts
mainnet deployment scripts
value-bearing anything
```

---

## 2. Current Project Status Lock

The `rox-anchor` repository starts from this safe doctrine:

```text
Internal ROC Beta Phase 6: COMPLETE / GREEN / PARKED.
Internal ROC value-loop proof: COMPLETE / GREEN / PARKED.
QuickChain boundary/preflight through Phase 5: COMPLETE / GREEN / PARKED, not public chain/runtime completion.
Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED.
Bridge / ROX / Solana / staking / liquidity / external settlement: docs / threat-model / decision-gate only.
```

This repository does **not** change that status.

This repository does **not** authorize:

```text
ROX token launch
Solana deployment
bridge runtime
mint/burn runtime
staking runtime
liquidity runtime
external settlement runtime
exchange-facing logic
public validator economy
user-facing bridge path
```

---

## 3. Origins

The RustyOnions IDB method borrows from but goes beyond:

* **RFCs** — strong structure and rationale, but often weak on enforcement gates.
* **ADRs** — useful decision history, but too thin for economic/security boundaries.
* **Formal methods** — strong invariants, but often low developer ergonomics.
* **Definition of Done** — strong delivery framing, but weak architectural grounding.
* **Safety-critical systems** — strong proof discipline, but often too heavy for daily development.

For `rox-anchor`, IDB acts as a **constitution**:

```text
No invariant without a gate.
No design principle without anti-scope.
No future mechanism without a threat model.
No bridge-shaped work without a decision gate.
No runtime before authorization.
No future implementation surface without explicit phase authorization.
```

---

## 4. Required North Star Block

Every major `rox-anchor` document must include this near the top:

```text
North Star: Internal ROC truth stays with svc-wallet + ron-ledger. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.
```

Purpose:

```text
prevent runtime drift
prevent bridge language creep
preserve the RustyOnions truth boundary
remind reviewers that planning artifacts are not authorization
```

A document may expand this block, but must not weaken it.

---

## 5. Required Anchor Meaning Declaration

Every major `rox-anchor` document that uses the word **anchor** must declare which meaning it uses.

Allowed meanings:

```text
QuickChain anchor:
  evidence/checkpoint/posture artifact
  does not mutate balances
  does not mint ROX
  does not settle value
  does not authorize bridge runtime

Solana Anchor:
  possible future Solana smart contract framework
  not active now
  not deployed now
  not authorized now

ROX-anchor:
  separate planning repository
  docs / threat-model / decision-gate only during Phase 0
  not a runtime bridge
  not a token launch
  not a deployment
```

Forbidden ambiguity:

```text
anchor means live settlement
anchor means bridge is active
anchor means Solana program exists
anchor means ROX is launched
anchor means external mint/burn is authorized
```

Every future blueprint must include one of the following:

```text
Anchor meaning used in this document: QuickChain anchor
```

```text
Anchor meaning used in this document: Solana Anchor
```

```text
Anchor meaning used in this document: ROX-anchor planning repo
```

If a document uses more than one meaning, it must define each one separately.

---

## 6. The ROX Anchor IDB Template

Every major `rox-anchor` document should follow this structure:

```markdown
---
title: <Blueprint Name>
version: <semver>
status: draft|reviewed|final|parked
last-updated: YYYY-MM-DD
audience: contributors, auditors, reviewers
scope: docs-only|threat-model|decision-gate|future-disabled-skeleton
---

# <Blueprint Name>

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.

RO:WHAT — What this document defines.
RO:WHY — Why this document exists.
RO:INTERACTS — What systems or future systems it touches.
RO:INVARIANTS — The hard boundaries this doc must preserve.
RO:SECURITY — Security posture and forbidden authority.
RO:TEST — Checker, test, review, or proof gate.

Anchor meaning used in this document: <QuickChain anchor | Solana Anchor | ROX-anchor planning repo>

ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

## 1. Invariants (MUST)
- [I-1] Non-negotiable law.
- [I-2] Non-negotiable law.

## 2. Design Principles (SHOULD)
- [P-1] Guideline or heuristic.
- [P-2] Design rationale.

## 3. Implementation Patterns (HOW)
- [C-1] Docs-only pattern, sketch, checker rule, or future-safe structure.
- [C-2] Engineering pattern, if authorized by the current phase.

## 4. Acceptance Gates (PROOF)
- [G-1] Static checker requirement.
- [G-2] Threat-model review requirement.
- [G-3] Decision-gate signoff requirement.

## 5. Anti-Scope (Forbidden / Outside Current Phase)
- What is not allowed in the current phase.
- What terms are forbidden outside explicit forbidden-scope context.
- What future work requires a later decision gate.

## 6. References
- Linked docs, specs, blueprints, threat models, and review packets.
```

---

## 7. Core Invariants (MUST)

These are the first laws of `rox-anchor`.

### 7.1 Internal ROC truth

* [I-1] Internal ROC truth remains with `svc-wallet` + `ron-ledger`.
* [I-2] `svc-wallet` remains the only internal economic mutation front-door.
* [I-3] `ron-ledger` remains the durable source of internal receipt, replay, conservation, and balance truth.
* [I-4] `rox-anchor` must never become internal ROC truth.
* [I-5] `rox-anchor` must never bypass `svc-wallet` for any future internal ROC issue path.
* [I-6] Any future recovery-account issue path must still route through `svc-wallet` and `ron-ledger`.

### 7.2 ROX-anchor Phase 0 scope

* [I-7] `rox-anchor` is docs / threat-model / decision-gate only during Phase 0.
* [I-8] No active ROX token launch is authorized in Phase 0.
* [I-9] No Solana deployment is authorized in Phase 0.
* [I-10] No bridge runtime is authorized in Phase 0.
* [I-11] No mint/burn runtime is authorized in Phase 0.
* [I-12] No coordinator runtime is authorized in Phase 0.
* [I-13] No relayer runtime is authorized in Phase 0.
* [I-14] No value-bearing devnet is authorized in Phase 0.
* [I-15] No toy-value public bridge demo is authorized in Phase 0.
* [I-16] No public or user-facing bridge path is authorized in Phase 0.
* [I-17] Later implementation-shaped work requires a later explicit decision gate; it is not forbidden forever.

### 7.3 Authority boundaries

* [I-18] `rox-anchor` must not become gateway authority.
* [I-19] `rox-anchor` must not become omnigate authority.
* [I-20] `rox-anchor` must not become accounting authority.
* [I-21] `rox-anchor` must not become rewarder authority.
* [I-22] `rox-anchor` must not become policy authority.
* [I-23] `rox-anchor` must not become storage authority.
* [I-24] `rox-anchor` must not become index authority.
* [I-25] `rox-anchor` must not become CrabLink authority.
* [I-26] CrabLink must remain display, routing, user intent, and explicit confirmation only.
* [I-27] CrabLink must never construct bridge proofs or claim bridge finality from local/cache state.

### 7.4 Future bridge model

* [I-28] Any future ROC → ROX path must be burn/proof/challenge/finality/mint, not custody/swap/liquidity.
* [I-29] Any future ROX → ROC path must be external burn evidence/proof/challenge/finality/internal issue through `svc-wallet`.
* [I-30] No future external mint may occur without separately authorized burn/proof/finality lifecycle.
* [I-31] No future internal issue may occur without separately authorized external burn/proof/finality lifecycle.
* [I-32] No fake finality is allowed.
* [I-33] No finality may be claimed before proof/challenge close.
* [I-34] No off-chain coordinator may unilaterally finalize value movement.
* [I-35] No single RPC response or single-observer proof may be treated as sufficient settlement truth.
* [I-36] Any future proof package must require explicit multi-RPC, commitment, cluster, program, mint, direction, nonce, and operation binding.
* [I-37] Cross-domain replay must be impossible by design.

### 7.5 Solana / Anchor future constraints

* [I-38] Solana Anchor means the future Solana smart contract framework, not QuickChain anchoring.
* [I-39] QuickChain anchors are evidence/checkpoint/posture artifacts; they do not mutate balances or mint ROX.
* [I-40] Any future Solana program account model must bind cluster, program ID, mint, direction, operation ID, and nonce.
* [I-41] Any future Solana program must be designed so proof verification and mint authority cannot be bypassed by off-chain coordinator compromise.
* [I-42] Any future deployment artifact must require verifiable/reproducible build evidence before release consideration, audit signoff, or deployment authorization.
* [I-43] Any future upgrade authority must be explicitly threat-modeled, multi-party controlled, recoverable, and pausable.
* [I-44] Any future Solana/Anchor code must be hidden-implementation-scanned for unauthorized program patterns, even if placed outside normal directories.
* [I-45] During Phase 0, Solana/Anchor account-model content must remain conceptual and non-executable.

### 7.6 Config and feature flags

* [I-46] All future bridge/ROX/Solana/staking/liquidity placeholders must be inert unless a later gate authorizes otherwise.
* [I-47] All placeholders must be disabled-by-default.
* [I-48] All placeholders must be non-operational in Phase 0.
* [I-49] All placeholders must be non-yield-bearing.
* [I-50] All placeholders must be non-user-facing in Phase 0.
* [I-51] No config field may silently enable runtime bridge behavior.
* [I-52] Any future enablement requires a separate decision gate, caps, pause defaults, challenge windows, user limits, cluster binding, mint binding, and audit/recovery proof.

---

## 8. Design Principles (SHOULD)

### 8.1 Determinism before distribution

* [P-1] Define deterministic DTOs before roots.
* [P-2] Define roots before validators.
* [P-3] Define proofs before pruning.
* [P-4] Define internal ROC truth before external anchoring.
* [P-5] Define threat models before implementation.

### 8.2 Burn/mint over custody/liquidity

* [P-6] Prefer burn/mint delayed-proof models over custody.
* [P-7] Avoid liquidity pool framing entirely during Phase 0.
* [P-8] Avoid swap/exchange/redemption language except in forbidden, threat-modeled, or future-gated contexts.
* [P-9] Treat “bridge” as a high-risk term that must remain scoped and gated.
* [P-10] Treat every conversion-shaped phrase as a product/legal/security hazard until proven safe.

### 8.3 Challenge windows and finality humility

* [P-11] Future proof packages should assume RPC equivocation, fork risk, stale reads, and coordinator compromise.
* [P-12] Future finality should be delayed, challengeable, and explicitly labeled.
* [P-13] UI should prefer “pending,” “observed,” “challenged,” “expired,” or “failed” over “complete” until finality rules are satisfied.
* [P-14] Cached UI state must never display final settlement as local truth.
* [P-15] Offline UI must degrade toward uncertainty, not confidence.

### 8.4 Keys, upgrades, and recovery

* [P-16] Future key management must be boring, documented, multi-party, and drill-tested.
* [P-17] Future upgrade authority must be minimized, delayed, observable, and recoverable.
* [P-18] Future pause/halt controls must default safe.
* [P-19] Recovery paths must not become hidden mint paths.
* [P-20] Emergency controls must never bypass `svc-wallet` + `ron-ledger` for internal ROC.

### 8.5 Product-language restraint

* [P-21] Product language should never imply guaranteed redemption.
* [P-22] Product language should never imply instant liquidity.
* [P-23] Product language should never imply ROX is live before a decision gate.
* [P-24] Product language should never imply external settlement is active.
* [P-25] Product language must distinguish planning from runtime.

### 8.6 Checker-first planning

* [P-26] Every new document category should have a checker rule before it becomes a planning dependency.
* [P-27] Forbidden-scope terms should be blocked by default and allowed only in explicit anti-scope/threat-model contexts.
* [P-28] A future implementation-shaped file should be treated as unsafe during Phase 0 until a later decision gate explicitly permits it.
* [P-29] Hidden implementation patterns matter more than directory names.
* [P-30] A green docs-only gate is not runtime authorization.
* [P-31] Phase 0 should stay small and reviewable.
* [P-32] Future implementation surfaces require explicit gates, but are not forbidden forever.

---

## 9. Implementation Patterns (HOW)

At current scope, “implementation” means docs and checkers only.

### 9.1 Authoritative Phase 0 repo structure

The Phase 0 scaffold is intentionally small:

```text
rox-anchor/
  README.md
  LICENSE
  .gitignore

  docs/
    00_IDB_ROX_ANCHOR.md
    01_SCOPE_DECISION_GATE.md
    02_THREAT_MODEL.md
    03_SYSTEM_STATE_PROOF_BLUEPRINT.md
    04_TESTPLAN_CHECKER.md

  scripts/
    check-rox-anchor-docs-only.sh
```

Required Phase 0 proof surface:

```text
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```

Optional harmless root metadata may exist:

```text
README.md
LICENSE
LICENSE-MIT
LICENSE-APACHE
NOTICE
CONTRIBUTING.md
CODE_OF_CONDUCT.md
SECURITY.md
.gitignore
.gitattributes
```

Root metadata must not introduce:

```text
build behavior
runtime behavior
deployment behavior
bridge behavior
token behavior
wallet behavior
staking behavior
liquidity behavior
exchange-facing behavior
```

Do not add additional planning documents during Phase 0 unless a later docs-expansion decision gate authorizes them.

### 9.2 Outside Phase 0 implementation-surface names

These names are outside the current Phase 0 scaffold:

```text
Cargo.toml
package.json
Anchor.toml
programs/
src/
anchor/
migrations/
app/
relayer/
coordinator/
crablink-bridge-ui/
solana-program/
token-mint/
mint/
burn/
stake/
staking/
liquidity/
dex/
cex/
mainnet/
devnet/
deployment/
deploy/
bridge/
bridge-ui/
wallet/
token/
rpc-proof/
proof-service/
```

These are **not forbidden forever**.

They are future implementation-surface names that require a later explicit decision gate before they are added.

### 9.3 Docs-only checker pattern

The Phase 0 checker should verify:

```text
required five docs exist
required checker exists
required RO headers exist
required North Star blocks exist
required anchor meaning declarations exist
required safe labels exist
outside-Phase-0 runtime-shaped directories do not exist
outside-Phase-0 runtime-shaped files do not exist
high-risk product/runtime language is controlled
hidden Solana/Anchor implementation patterns do not exist outside docs/checker context
no Solana program code exists
no Anchor program code exists
no coordinator runtime exists
no relayer runtime exists
no mint/burn runtime exists
no staking/liquidity/exchange-facing runtime exists
```

### 9.4 Runtime-shaped file and directory scanner pattern

At Phase 0, the checker should fail on runtime-shaped files/directories such as:

```text
programs/
anchor/
migrations/
app/
src/
src/lib.rs
src/main.rs
src/instructions/
src/accounts/
src/state.rs
src/processor.rs
relayer/
coordinator/
crablink-bridge-ui/
solana-program/
token-mint/
mint/
burn/
stake/
staking/
liquidity/
dex/
cex/
mainnet/
devnet/
devnet-deploy/
deployment/
```

Exceptions may only be explicit docs/checker references, not implementation files.

### 9.5 Hidden implementation scanner pattern

The checker must scan implementation-shaped files for Solana/Anchor markers.

Forbidden hidden implementation markers include:

```text
#[program]
declare_id!
#[derive(Accounts)]
anchor_lang
anchor_spl
Context<
Program<
Account<
AccountInfo<
Signer<
UncheckedAccount
InterfaceAccount
MintTo
Burn
TransferChecked
set_authority
spl_token
spl_associated_token_account
invoke_signed
system_program
instruction module
pub mod instructions
pub mod accounts
pub mod state
```

Allowed only inside explicit docs-only discussion context:

```text
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT
```

Rule:

```text
A hidden implementation marker must not appear in executable or implementation-shaped files during Phase 0.
A hidden implementation marker may appear only inside docs/checker content that explicitly frames it as forbidden, threat-modeled, or future-gated.
```

### 9.6 Forbidden language scanner pattern

The checker should scan docs and comments for high-risk product/runtime language.

High-risk terms include:

```text
instant
guaranteed
convert
conversion
swap
trade
cash out
cash-out
redeem
redemption
exchange
liquidity
yield
stake
staking
DEX
CEX
bridge live
mainnet ready
public beta
user-facing bridge
client-side settlement
settles
settlement
finalizes
unlocks
withdraw
deposit
```

However, these terms may appear in explicit forbidden-scope, anti-scope, threat-model, or future-gated sections.

Allowed context markers:

```text
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT
```

Rule:

```text
High-risk terms may appear only when explicitly framed as forbidden, threat-modeled, anti-scope, or future-gated.
```

### 9.7 Future DTO sketch pattern

DTO sketches may exist only as documentation in Phase 0, for example:

```text
BridgeOperationId
BridgeDirection
InternalBurnEvidence
ExternalBurnEvidence
ProofPackage
ChallengeWindow
FinalityDecision
RecoveryCase
ClusterBinding
ProgramBinding
MintBinding
CommitmentEvidence
RpcQuorumObservation
```

During Phase 0:

```text
No generated code.
No Rust implementation.
No TypeScript implementation.
No schema that claims runtime readiness.
```

A later explicit decision gate may authorize non-value-bearing DTO skeletons, but that is outside Phase 0.

### 9.8 Future state-machine sketch pattern

State-machine sketches may use labels like:

```text
Draft
Requested
Observed
ProofPackaged
EvidenceInsufficient
QuorumDisputed
ChallengeOpen
Challenged
Expired
FinalityEligible
FinalizedByDecisionGate
Failed
Recovered
Halted
Abandoned
```

Forbidden state names in current product/runtime claims:

```text
InstantComplete
Guaranteed
Redeemed
Swapped
CashedOut
LiveBridge
MainnetReady
```

### 9.9 Future proof-package planning pattern

Future proof-package sketches must include these dimensions before they are considered complete:

```text
source domain
target domain
direction
operation_id
nonce
idempotency_key
cluster
program_id
mint
token account
transaction signature
slot
block time
commitment level
RPC quorum observations
challenge window open time
challenge window close time
finality decision reference
halt status
recovery status
```

No proof package may imply settlement by itself.

---

## 10. Acceptance Gates (PROOF)

### 10.1 Phase 0 docs-only gate

Phase 0 is complete only when:

```text
[G-1] docs/00_IDB_ROX_ANCHOR.md exists.
[G-2] docs/01_SCOPE_DECISION_GATE.md exists.
[G-3] docs/02_THREAT_MODEL.md exists.
[G-4] docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md exists.
[G-5] docs/04_TESTPLAN_CHECKER.md exists.
[G-6] scripts/check-rox-anchor-docs-only.sh exists.
[G-7] Checker passes.
[G-8] No outside-Phase-0 runtime-shaped files/directories exist.
[G-9] No forbidden runtime language appears outside allowed context.
[G-10] No hidden Solana/Anchor implementation markers appear outside allowed docs/checker context.
[G-11] Every required doc includes the North Star block.
[G-12] Every required doc declares anchor meaning.
[G-13] Every required doc includes required RO headers.
[G-14] Current safe labels are present.
[G-15] Phase 0 green label is printed by the checker.
```

### 10.2 Threat model gate

Threat model v1 must cover:

```text
off-chain observation / coordination layer compromise
coordinator compromise
relayer compromise
malicious Solana RPC
stale/forked RPC evidence
cross-cluster replay
devnet-to-mainnet replay
program-id spoofing
mint spoofing
nonce replay
direction replay
operation-id replay
challenge-window spam
challenge griefing
upgrade authority compromise
multisig compromise
key rotation failure
pause/halt abuse
recovery path abuse
stale CrabLink finality display
client-side proof/finality claims
product language creep
single-observer proof failure
multi-RPC quorum failure
minimum commitment downgrade
verifiable-build failure
hidden implementation drift
internal ROC boundary bypass
external mint/burn lifecycle abuse
```

Threat Model Review Gate label, if separately reviewed and checker-passing:

```text
ROX Anchor Phase 1 — Threat Model Review Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

### 10.3 State / Proof Design Gate

State / Proof Design Gate must cover:

```text
conceptual ROC → ROX path
conceptual ROX → ROC path
conceptual system boundaries
allowed states
forbidden states
allowed transitions
forbidden transitions
proof package field sketch
proof validation sketch
nonce/replay/finality requirements
pause/halt/recovery requirements
upgrade/verifiable-build requirements
CrabLink display-only status sketch
```

State / Proof Design Gate label, if separately reviewed and checker-passing:

```text
ROX Anchor Phase 2 — State / Proof Design Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

### 10.4 Future implementation gate

No implementation-shaped work may begin until a later decision gate explicitly authorizes it.

Implementation-shaped work includes:

```text
Anchor program code
Solana account structs
instruction handlers
coordinator services
relayer services
RPC proof services
CrabLink bridge UI
devnet deployment scripts
mint/burn test harnesses
token program integration
```

Possible future allowed work, if separately authorized:

```text
disabled-by-default skeleton
non-value-bearing local-only structure
no devnet
no mainnet
no token
no bridge runtime
no user-facing behavior
```

### 10.5 Verifiable build gate

Any future deployment-shaped work, if ever separately authorized, must include:

```text
[G-20] reproducible build instructions
[G-21] build environment lock
[G-22] artifact hash record
[G-23] source revision binding
[G-24] dependency lock evidence
[G-25] auditor reproduction path
[G-26] upgrade authority review
[G-27] deployment pause/halt plan
```

No deployment-shaped artifact may be treated as release-ready without these gates.

### 10.6 Final Phase 0 safe label

Phase 0 may use this label only after the docs-only checker passes:

```text
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This label does not authorize runtime.

---

## 11. Anti-Scope (Forbidden / Outside Current Phase)

ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT

The following are forbidden in current Phase 0 scope:

```text
ROX runtime
Solana runtime
bridge runtime
mint/burn runtime
staking runtime
liquidity runtime
yield runtime
external settlement runtime
exchange-facing logic
public validator economy
user-facing bridge path
devnet deployment
mainnet deployment
toy-value bridge demo
CrabLink bridge UI
client-side settlement
client-side proof construction
client-side finality claim
coordinator service
relayer service
RPC proof service
custodial bridge
instant swap
guaranteed redemption
cash-out flow
DEX integration
CEX integration
```

These are not forbidden forever. They are outside the current Phase 0 scope unless a later explicit decision gate authorizes the relevant future phase.

Forbidden positive claims in current docs/product language:

```text
ROX is live
bridge is live
users can convert ROC to ROX
users can cash out
users can redeem
users can swap
users can stake
users can earn yield
external settlement is active
Solana integration is active
mainnet is ready
public bridge beta is ready
```

Forbidden authority paths:

```text
CrabLink → ledger mutation
gateway → ledger mutation
omnigate → ledger mutation
rox-anchor → ledger mutation
storage/index pointer → paid unlock
accounting snapshot → balance truth
rewarder plan → payout execution
policy allow → receipt truth
external proof → internal issue without svc-wallet
off-chain coordinator → finality without challenge close
single RPC → settlement truth
single observer → settlement truth
cache → bridge finality
```

---

## 12. First Patch Scope

The first `rox-anchor` patch should create only:

```text
README.md
LICENSE
.gitignore
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```

Optional harmless metadata may also be added if needed:

```text
SECURITY.md
CONTRIBUTING.md
NOTICE
CODE_OF_CONDUCT.md
.gitattributes
```

Do not create during Phase 0:

```text
Cargo.toml
package.json
Anchor.toml
programs/
src/
anchor/
migrations/
relayer/
coordinator/
crablink-bridge-ui/
deployment scripts
```

These are future implementation-surface names that require a later explicit decision gate.

---

## 13. Reviewer Checklist

Before accepting any `rox-anchor` document, reviewers must verify:

```text
[ ] Does it preserve Internal ROC truth with svc-wallet + ron-ledger?
[ ] Does it include the North Star block?
[ ] Does it declare the meaning of "anchor" if the word is used?
[ ] Does it avoid authorizing runtime?
[ ] Does it avoid fake finality?
[ ] Does it avoid single-RPC or single-observer settlement truth?
[ ] Does it avoid client-side settlement claims?
[ ] Does it avoid swap/exchange/liquidity/staking language outside forbidden/threat/future-gated contexts?
[ ] Does it distinguish QuickChain anchor from Solana Anchor?
[ ] Does it require future decision gates?
[ ] Does it include anti-scope?
[ ] Does it define proof gates?
[ ] Does it keep CrabLink display-only?
[ ] Does it keep future recovery paths routed through svc-wallet?
[ ] Does it avoid adding implementation-shaped files during Phase 0?
[ ] Does it avoid hidden implementation markers outside docs/checker context?
[ ] Does it say future implementation surfaces are not forbidden forever?
[ ] Does the checker pass?
```

---

## 14. Why Adopt IDB for ROX Anchor

`rox-anchor` is a high-risk planning domain because words like “bridge,” “anchor,” “mint,” “burn,” “settlement,” and “token” can accidentally imply active value movement.

IDB prevents that by making every document answer:

```text
What must never break?
What is only a design preference?
What mechanics are allowed at this phase?
What proves the gate is green?
What is outside current scope?
```

For this repo, IDB is not paperwork.

It is the safety boundary between:

```text
serious future bridge planning
```

and

```text
unauthorized bridge runtime
```

The north star:

```text
Internal ROC truth stays with svc-wallet + ron-ledger.
ROX-anchor is planning, not runtime.
Bridge remains docs / threat-model / decision-gate only.
```

---

## 15. References

```text
SESSION_PACK.MD
POST_QUICKCHAIN_DECISION_GATE.md
INTERNAL_ROC_MODEL.md
INTERNAL_ROC_BETA_VALUE_LOOP_BLUEPRINT.md
INTERNAL_ROC_BETA_BUILDPLAN.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```
