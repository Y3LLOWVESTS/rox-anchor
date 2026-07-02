---

title: ROX Anchor Threat Model
version: 0.3.0
status: draft
last-updated: 2026-07-01
audience: contributors, auditors, reviewers
scope: docs-only / threat-model / decision-gate
-----------------------------------------------

# ROX Anchor Threat Model

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.

RO:WHAT — Defines the Phase 0/1 threat model for future ROX-anchor planning.
RO:WHY — Identifies high-risk bridge/anchor failure modes before any implementation-shaped work is authorized.
RO:INTERACTS — Internal ROC, svc-wallet, ron-ledger, CrabLink, QuickChain future proof work, future proof packages, future Solana RPC observations, future UI language.
RO:INVARIANTS — No single observer, RPC, coordinator, relayer, cache, UI layer, proof sketch, or planning document may become settlement/finality truth.
RO:SECURITY — Assumes off-chain coordination compromise, stale RPC, forked RPC, replay, key compromise, challenge griefing, stale UI, product-language creep, hidden implementation drift, and recovery-path abuse.
RO:TEST — `bash scripts/check-rox-anchor-docs-only.sh`.

Anchor meaning used in this document: ROX-anchor planning repo.

ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

---

## 0. Scope and Non-Authorization Notice

This document is a threat model.

It does **not** authorize:

```text
ROX runtime
Solana runtime
bridge runtime
mint/burn runtime
coordinator runtime
relayer runtime
RPC proof service runtime
CrabLink bridge UI
devnet deployment
mainnet deployment
staking
liquidity
external settlement
exchange-facing behavior
user-facing bridge behavior
```

A threat described here is not an implemented mitigation.

A mitigation sketched here is not runtime authorization.

A green threat-model gate is not runtime authorization.

This threat model is scoped to the current docs-only / planning phase. It must be re-reviewed and expanded before any later disabled skeleton, implementation-shaped, devnet, runtime, deployment, or user-facing decision gate is considered.

---

## 1. Invariants (MUST)

* [I-1] Threat modeling does not authorize runtime.
* [I-2] No threat model scenario may be treated as implemented mitigation.
* [I-3] Internal ROC truth remains with `svc-wallet` + `ron-ledger`.
* [I-4] No future bridge proof package may rely on one RPC response.
* [I-5] No future bridge proof package may rely on one observer.
* [I-6] No off-chain coordinator, relayer, observer, or proof assembler may unilaterally finalize value movement.
* [I-7] No cache may become finality truth.
* [I-8] No UI may claim finality from stale/offline data.
* [I-9] Any future recovery path must not become hidden mint/issue authority.
* [I-10] Any future upgrade path must be pausable, reviewable, delayed, and recoverable.
* [I-11] Any future internal ROC issue path must route through `svc-wallet`.
* [I-12] Any future internal ROC balance mutation must be recorded by `ron-ledger`.
* [I-13] Any future external ROX mint path must require a separately authorized burn/proof/challenge/finality lifecycle.
* [I-14] Any future ROX → ROC path must require external burn evidence and internal issue through `svc-wallet`.
* [I-15] No proof package may be treated as finality by itself.
* [I-16] No challenge window may be skipped for convenience.
* [I-17] No devnet, toy-value demo, or local skeleton may imply public readiness.
* [I-18] No product copy may imply live conversion, cash-out, redemption, swap, yield, staking, exchange access, or guaranteed settlement.
* [I-19] Hidden implementation-shaped files are threat events, not harmless scaffolding.
* [I-20] Every future deployment-shaped artifact must require reproducible build evidence before release consideration.
* [I-21] CrabLink must remain strictly display-only for any future bridge status.
* [I-22] CrabLink must never construct proofs, claim finality, or treat cached/offline state as settlement truth.

---

## 2. Protected Assets

The threat model protects these assets:

```text
internal ROC ledger truth
internal ROC receipt truth
internal ROC replay/conservation evidence
svc-wallet mutation boundary
ron-ledger durable truth boundary
operation_id identity
idempotency_key retry semantics
future proof package integrity
future nonce uniqueness
future challenge-window integrity
future finality-decision integrity
future pause/halt authority
future recovery queue integrity
future upgrade authority integrity
future key custody and rotation process
CrabLink display-only status boundary
product language safety
docs-only repository scope
```

The threat model explicitly does **not** protect a live bridge, live ROX token, live Solana deployment, or live external settlement path, because none is authorized.

---

## 3. Trust Boundaries

### 3.1 Trusted internal truth boundary

Trusted for internal ROC truth:

```text
svc-wallet
ron-ledger
```

Only `svc-wallet` may act as the internal economic mutation front-door.

Only `ron-ledger` may be durable internal economic truth.

### 3.2 Non-authority internal services

These must not become economic truth:

```text
ron-accounting
svc-rewarder
ron-policy
svc-storage
svc-index
svc-gateway
omnigate
CrabLink
rox-anchor
```

### 3.3 Future external observation boundary

Future external observations are untrusted until proven otherwise.

Untrusted by default:

```text
Solana RPC responses
single observer reports
coordinator output
relayer output
indexer output
cached UI state
off-chain proof package assembly
user-provided transaction signatures
third-party explorer data
```

### 3.4 Documentation boundary

This repository is itself a boundary.

Allowed:

```text
docs
threat models
blueprints
state-machine sketches
DTO/proof sketches
checker scripts
decision gates
review packets
```

Forbidden without a later explicit decision gate:

```text
runtime code
Solana program code
Anchor account structs
coordinator code
relayer code
deployment scripts
CrabLink bridge UI
mint/burn harnesses
```

---

## 4. Attacker Classes

The model assumes the following attacker classes:

```text
malicious user seeking double-mint/double-issue
malicious coordinator
malicious relayer
compromised RPC provider
stale or forked RPC provider
compromised maintainer key
compromised upgrade authority
compromised multisig participant
malicious documentation contributor
scope-creep contributor
product-language contributor who overpromises runtime
CrabLink cache/stale-state confusion
bot attempting challenge-window griefing
attacker replaying proof across cluster/mint/direction/operation
attacker exploiting recovery path as hidden mint/issue
attacker hiding implementation files in unexpected directories
attacker attempting devnet-to-mainnet replay
attacker relying on user confusion around “anchor”
```

---

## 5. Design Principles (SHOULD)

* [P-1] Assume every off-chain actor can be compromised.
* [P-2] Assume every RPC can be stale, forked, malicious, censored, or inconsistent.
* [P-3] Assume every product phrase can create legal/security confusion.
* [P-4] Assume every recovery path can become an exploit path.
* [P-5] Prefer delayed, challengeable, observable finality over fast finality.
* [P-6] Prefer failure-closed states over optimistic settlement states.
* [P-7] Prefer multi-source evidence over single-source evidence.
* [P-8] Prefer explicit halt states over ambiguous pending states.
* [P-9] Prefer boring key ceremony and reproducible build evidence over speed.
* [P-10] Prefer refusing implementation-shaped work over accepting “harmless” scaffolding.
* [P-11] Prefer conservative UI language over confidence-building language.
* [P-12] Prefer domain-bound identifiers over reusable identifiers.

---

## 6. Threat Categories (HOW)

### 6.1 Off-chain observation / coordination layer compromise

Threat:

```text
A coordinator, relayer, observer, indexer, or proof assembler submits false, stale, incomplete, reordered, censored, or replayed observations.
```

Impact:

```text
false finality
double-mint attempt
double-issue attempt
censorship of legitimate finalization
challenge-window manipulation
incorrect recovery routing
proof package poisoning
```

Required future mitigations:

```text
multi-party observation
operation_id binding
nonce binding
direction binding
challenge window
pause/halt controls
no unilateral coordinator finality
no unilateral relayer finality
coordinator output treated as evidence, not truth
relayer output treated as evidence, not truth
independent verifier path
audit log of coordinator / relayer assertions
```

Gate requirement:

```text
No coordinator or relayer implementation may be created until a later explicit decision gate authorizes it.
```

---

### 6.2 Malicious or stale Solana RPC

Threat:

```text
A future RPC source reports forked, stale, filtered, inconsistent, censored, or false transaction/account state.
```

Impact:

```text
false external burn evidence
false mint evidence
missed challenge evidence
incorrect commitment assumption
fork-based replay
devnet/mainnet confusion
```

Required future mitigations:

```text
multi-RPC quorum
minimum commitment policy
cluster binding
slot and block-time recording
program-id binding
mint binding
transaction signature binding
disagreement handling
RPC diversity
quorum failure state
commitment downgrade rejection
```

Gate requirement:

```text
No single RPC response or single observer may become settlement truth.
```

---

### 6.3 Replay and domain confusion

Threat:

```text
A proof package from one direction, cluster, mint, operation, or environment is replayed in another context.
```

Impact:

```text
cross-direction replay
cross-cluster replay
devnet-to-mainnet replay
mint spoofing
program spoofing
duplicate finality claim
double issue
double mint
```

Required future mitigations:

```text
source domain binding
target domain binding
direction binding
cluster binding
program binding
mint binding
operation_id binding
nonce uniqueness
idempotency_key separation
devnet-to-mainnet replay rejection
proof package version binding
chain/environment binding
```

Gate requirement:

```text
No proof sketch is acceptable unless it binds source domain, target domain, direction, cluster, program, mint, operation_id, and nonce.
```

---

### 6.4 Internal ROC boundary bypass

Threat:

```text
A future ROX-anchor path attempts to create, restore, issue, unlock, or mutate internal ROC without svc-wallet and ron-ledger.
```

Impact:

```text
unauthorized ROC creation
ledger conservation break
receipt forgery
accounting mismatch
reward abuse
policy bypass
```

Required future mitigations:

```text
svc-wallet-only internal issue path
ron-ledger receipt requirement
no direct ledger mutation
no accounting-derived balance truth
no policy-derived receipt truth
no rewarder direct payout execution
recovery path routes through svc-wallet
```

Gate requirement:

```text
Every future internal issue or recovery path must route through svc-wallet and settle into ron-ledger truth.
```

---

### 6.5 External mint/burn lifecycle abuse

Threat:

```text
A future external mint or burn path is triggered without the full burn/proof/challenge/finality lifecycle.
```

Impact:

```text
unbacked ROX mint
unbacked internal ROC issue
double-spend across domains
false circulating supply
broken conservation model
```

Required future mitigations:

```text
burn evidence before mint
external burn evidence before internal issue
challenge window
finality decision reference
operation_id uniqueness
nonce uniqueness
pause/halt on mismatch
supply reconciliation report
```

Gate requirement:

```text
No external mint or internal issue may occur from proof evidence alone.
```

---

### 6.6 Upgrade authority compromise

Threat:

```text
A future program, coordinator, proof service, or configuration upgrade path is used to introduce malicious logic.
```

Impact:

```text
bypassed proof verification
malicious mint authority
disabled challenge window
stolen recovery authority
silent config activation
deployment mismatch
```

Required future mitigations:

```text
multi-party upgrade control
delayed upgrades
public upgrade notice
pause before upgrade
reproducible build evidence
artifact hash record
source revision binding
dependency lock evidence
auditor reproduction path
rollback / halt plan
upgrade authority review
```

Gate requirement:

```text
No deployment-shaped artifact may be release-ready without reproducible build evidence and upgrade authority review.
```

---

### 6.7 Key custody and rotation failure

Threat:

```text
A future key, multisig, authority, signer, or recovery credential is lost, leaked, compromised, or rotated incorrectly.
```

Impact:

```text
unauthorized upgrade
unauthorized pause/halt
inability to halt
inability to recover
malicious finality decision
coordinator impersonation
relayer impersonation
```

Required future mitigations:

```text
key inventory
role separation
multi-party custody
rotation ceremony
revocation ceremony
emergency halt drill
lost-key drill
compromised-key drill
auditor-observed recovery drill
```

Gate requirement:

```text
No future runtime authority may exist without documented key custody and rotation drills.
```

---

### 6.8 Challenge-window griefing

Threat:

```text
Attackers spam, manipulate, censor, or exploit challenges to block legitimate finalizations or force operator intervention.
```

Impact:

```text
stuck finalizations
operator overload
denial of service
grief-cost imbalance
forced halt
recovery queue congestion
```

Required future mitigations:

```text
challenge cost model
rate limits
challenge validity rules
bounded challenge windows
recovery queue
griefing metrics
halt escalation path
spam rejection rules
operator review thresholds
```

Gate requirement:

```text
No finality model may be accepted without challenge-window abuse analysis.
```

---

### 6.9 Pause, halt, and recovery abuse

Threat:

```text
Pause, halt, or recovery controls become hidden authority paths or are abused to censor, mint, issue, or finalize incorrectly.
```

Impact:

```text
hidden mint path
hidden issue path
censorship
stuck funds
manual balance mutation
operator discretion risk
```

Required future mitigations:

```text
safe default halted state
bounded recovery cases
recovery classification
recovery issue path through svc-wallet only
manual action audit log
halt reason codes
resume checklist
no manual balance mutation
```

Gate requirement:

```text
Recovery must never become a hidden mint, issue, or settlement path.
```

---

### 6.10 Stale CrabLink finality display

Threat:

```text
CrabLink displays a cached, offline, stale, optimistic, or locally inferred state as complete/final.
```

Impact:

```text
user confusion
false settlement belief
support burden
legal/product risk
phishing amplification
trust loss
```

Required future mitigations:

```text
display-only bridge status
backend-derived status only
stale labels
offline uncertainty labels
no local finality truth
no cache-only bridge completion
no client-side proof construction
no client-side finality claim
```

Gate requirement:

```text
CrabLink must remain strictly display-only for any future bridge status.
Even if separately authorized later, CrabLink must never construct proofs, claim finality, or treat cached/offline state as settlement truth.
CrabLink may display future bridge status only as backend-derived, stale-aware, display-only information if separately authorized.
```

---

### 6.11 Product language creep

Threat:

```text
Docs or UI imply instant conversion, redemption, cash-out, exchange, staking, yield, bridge live status, public beta status, or guaranteed settlement.
```

Impact:

```text
user misunderstanding
regulatory confusion
security assumptions
social engineering
exchange-facing drift
runtime pressure
```

Required future mitigations:

```text
forbidden-language scanner
review checklist
explicit future-gated language
no public bridge beta language
no exchange-facing language
no instant wording
no guaranteed wording
no cash-out wording
no yield/staking wording
```

Gate requirement:

```text
High-risk product language may appear only in forbidden-scope, threat-model, anti-scope, or future-gated context.
```

---

### 6.12 Hidden implementation drift

Threat:

```text
Implementation-shaped files appear inside the repo before authorization, including Solana/Anchor markers hidden outside normal directories.
```

Impact:

```text
runtime creep
review bypass
accidental deployment path
false readiness
scope confusion
unsafe scaffolding
```

Required future mitigations:

```text
docs-only checker
forbidden directory scanner
hidden marker scanner
no programs directory
no src directory
no Anchor markers
no coordinator files
no relayer files
no deployment files
```

Gate requirement:

```text
Hidden implementation-shaped files are blocker findings during Phase 0.
```

---

### 6.13 Verifiable-build failure

Threat:

```text
A future deployment-shaped artifact cannot be reproduced from reviewed source, dependencies, and build environment.
```

Impact:

```text
supply-chain attack
auditor blind spot
malicious binary/program deployment
source/deployment mismatch
upgrade compromise
```

Required future mitigations:

```text
reproducible build instructions
build environment lock
artifact hash record
source revision binding
dependency lock evidence
auditor reproduction path
release checklist
deployment hash comparison
```

Gate requirement:

```text
No future deployment-shaped artifact may pass release/audit review without reproducible build evidence.
```

---

### 6.14 Documentation ambiguity around “anchor”

Threat:

```text
Readers confuse QuickChain anchors, Solana Anchor, and the rox-anchor planning repository.
```

Impact:

```text
scope confusion
runtime assumption
bridge authorization confusion
public-chain confusion
Solana deployment assumption
```

Required future mitigations:

```text
anchor meaning declaration
terminology lock
North Star block
reviewer checklist
forbidden positive claims
```

Gate requirement:

```text
Every major document that uses anchor language must declare the intended meaning.
```

---

## 7. Abuse-Case Matrix

| Abuse case                          | Impact                   | Required future defense                         | Current status      |
| ----------------------------------- | ------------------------ | ----------------------------------------------- | ------------------- |
| Coordinator submits false evidence  | False finality           | Challenge window, multi-observer evidence, halt | Threat-modeled only |
| Relayer submits stale evidence      | False finality           | Multi-observer evidence, audit log, halt        | Threat-modeled only |
| RPC returns forked state            | False proof              | Multi-RPC quorum, commitment policy             | Threat-modeled only |
| Devnet proof replayed on mainnet    | Cross-environment replay | Cluster binding, environment binding            | Threat-modeled only |
| Proof reused in opposite direction  | Cross-direction replay   | Direction binding                               | Threat-modeled only |
| External proof creates ROC directly | Internal truth bypass    | svc-wallet-only issue path                      | Forbidden           |
| Internal burn mints ROX immediately | Fake finality            | Challenge/finality lifecycle                    | Forbidden           |
| CrabLink cache shows final          | False UI confidence      | backend-derived stale-aware display             | Forbidden           |
| Recovery path issues manually       | Hidden mint/issue        | recovery through svc-wallet only                | Forbidden           |
| Upgrade swaps malicious logic       | Authority compromise     | delayed upgrade, reproducible build, audit      | Threat-modeled only |
| Docs say users can cash out         | Product/legal drift      | forbidden-language scanner                      | Forbidden           |
| Anchor code appears in repo         | Runtime creep            | docs-only checker                               | Forbidden           |

---

## 8. Risk Severity Guide

Severity levels:

```text
Critical:
  can create unbacked value, bypass svc-wallet, bypass ron-ledger, fake finality, or enable runtime bridge behavior

High:
  can mislead users, break challenge/finality assumptions, compromise keys, or corrupt proof evidence

Medium:
  can cause denial of service, stuck states, ambiguity, or operational recovery burden

Low:
  can cause documentation confusion without direct value or authority impact
```

Default severity assignments:

```text
internal ROC boundary bypass: Critical
external mint/burn lifecycle abuse: Critical
single-RPC settlement truth: Critical
coordinator unilateral finality: Critical
relayer unilateral finality: Critical
upgrade authority compromise: Critical
key compromise: Critical
stale CrabLink finality display: High
product language creep: High
challenge-window griefing: High
hidden implementation drift: High
documentation anchor ambiguity: Medium
```

Any Critical threat blocks later runtime gates until mitigated, tested, reviewed, and parked.

---

## 9. Acceptance Gates (PROOF)

Threat model v1 is acceptable only when it covers:

```text
[G-1] off-chain observation / coordination layer compromise
[G-2] coordinator compromise
[G-3] relayer compromise
[G-4] malicious Solana RPC
[G-5] stale/forked RPC evidence
[G-6] cross-cluster replay
[G-7] devnet-to-mainnet replay
[G-8] program-id spoofing
[G-9] mint spoofing
[G-10] nonce replay
[G-11] direction replay
[G-12] operation-id replay
[G-13] challenge-window spam
[G-14] challenge griefing
[G-15] upgrade authority compromise
[G-16] multisig compromise
[G-17] key rotation failure
[G-18] pause/halt abuse
[G-19] recovery path abuse
[G-20] stale CrabLink finality display
[G-21] client-side proof/finality claims
[G-22] product language creep
[G-23] single-observer proof failure
[G-24] multi-RPC quorum failure
[G-25] minimum commitment downgrade
[G-26] verifiable-build failure
[G-27] hidden implementation drift
[G-28] internal ROC boundary bypass
[G-29] external mint/burn lifecycle abuse
[G-30] documentation ambiguity around anchor
[G-31] explicit statement that threat model green is not runtime authorization
[G-32] explicit statement that threat model must be re-reviewed before any skeleton/runtime gate
```

Safe label after this document is reviewed and checker-passing:

```text
ROX Anchor Phase 1 — Threat Model Review Gate:
COMPLETE / GREEN / PARKED.
```

This label does **not** authorize runtime.

---

## 10. Anti-Scope (Forbidden)

ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT

Threat modeling must not create:

```text
runtime code
coordinator service
relayer service
RPC proof service
Solana program code
Anchor account structs
CrabLink bridge UI
deployment scripts
mint/burn harnesses
staking or liquidity logic
exchange-facing logic
```

Threat modeling must not claim:

```text
risk is solved
bridge is live
ROX is live
settlement is active
future finality is guaranteed
single-RPC proof is acceptable
client-side settlement is acceptable
users can convert ROC to ROX
users can redeem ROX
users can cash out
users can stake
users can earn yield
```

Threat modeling must not weaken:

```text
svc-wallet-only internal issue path
ron-ledger internal truth
docs-only current scope
challenge-window requirement
multi-RPC / multi-observer future requirement
pause/halt/recovery review requirement
verifiable-build requirement
CrabLink display-only boundary
```

---

## 11. Reviewer Checklist

Before this threat model can be considered reviewed, confirm:

```text
[ ] It states that threat modeling does not authorize runtime.
[ ] It states that the threat model must be re-reviewed before any skeleton/runtime gate.
[ ] It preserves Internal ROC truth with svc-wallet + ron-ledger.
[ ] It treats all off-chain actors as compromisable.
[ ] It treats all RPC observations as untrusted until quorum/finality rules are satisfied.
[ ] It blocks single-RPC settlement truth.
[ ] It blocks single-observer settlement truth.
[ ] It blocks coordinator unilateral finality.
[ ] It blocks relayer unilateral finality.
[ ] It blocks client/cache finality.
[ ] It keeps CrabLink strictly display-only for future bridge status.
[ ] It routes any future internal issue through svc-wallet.
[ ] It treats recovery paths as possible exploit paths.
[ ] It covers key custody and rotation failure.
[ ] It covers upgrade authority compromise.
[ ] It covers challenge-window griefing.
[ ] It covers hidden implementation drift.
[ ] It covers product language creep.
[ ] It covers ambiguity around anchor terminology.
[ ] It defines acceptance gates.
[ ] It keeps all mitigations as future-gated unless separately implemented later.
[ ] It references the docs-only checker.
```

---

## 12. References

```text
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```
