---

title: ROX Anchor Scope and Decision Gate
version: 0.3.0
status: draft
last-updated: 2026-07-01
audience: contributors, auditors, reviewers
scope: docs-only / threat-model / decision-gate
-----------------------------------------------

# ROX Anchor Scope and Decision Gate

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.

RO:WHAT — Defines the scope, terminology, current authorization boundary, and decision gates for the `rox-anchor` planning repo.
RO:WHY — Prevents bridge/runtime/scope creep before any future ROX/Solana/Anchor work is separately authorized.
RO:INTERACTS — Internal ROC, CrabLink, QuickChain future proof work, future Solana Anchor planning docs.
RO:INVARIANTS — No runtime; no token launch; no Solana deployment; no mint/burn behavior; no external settlement; no user-facing bridge path.
RO:SECURITY — Every future value-adjacent path remains blocked until explicit decision gates authorize a later phase.
RO:TEST — `bash scripts/check-rox-anchor-docs-only.sh`.

Anchor meaning used in this document: ROX-anchor planning repo.

ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

---

## 0. Current Safe Status

The `rox-anchor` repository starts from this project status:

```text
Internal ROC Beta Phase 6: COMPLETE / GREEN / PARKED.
Internal ROC value-loop proof: COMPLETE / GREEN / PARKED.
QuickChain boundary/preflight through Phase 5: COMPLETE / GREEN / PARKED, not public chain/runtime completion.
Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED.
Bridge / ROX / Solana / staking / liquidity / external settlement: docs / threat-model / decision-gate only.
```

This repository does **not** change that status.

This repository does **not** authorize runtime.

This repository is a planning repository.

---

## 1. Invariants (MUST)

* [I-1] `rox-anchor` is a separate planning repository.
* [I-2] `rox-anchor` does not mutate ROC.
* [I-3] `rox-anchor` does not mint ROX.
* [I-4] `rox-anchor` does not deploy Solana programs.
* [I-5] `rox-anchor` does not create bridge runtime.
* [I-6] `rox-anchor` does not create user-facing bridge behavior.
* [I-7] Internal ROC truth remains with `svc-wallet` + `ron-ledger`.
* [I-8] Any future internal ROC issue path must route through `svc-wallet`.
* [I-9] No finality may be claimed before proof/challenge close.
* [I-10] A docs-only green gate is not runtime authorization.
* [I-11] A threat-model green gate is not runtime authorization.
* [I-12] A blueprint green gate is not runtime authorization.
* [I-13] A checker green gate is not runtime authorization.
* [I-14] No future implementation-shaped work may begin without a later explicit decision gate.
* [I-15] No future skeleton code may be treated as harmless by default; implementation-shaped files require authorization.
* [I-16] No future bridge status may be displayed in CrabLink as truth from local/cache state.
* [I-17] No future external proof may create internal ROC without `svc-wallet`.
* [I-18] No future internal burn evidence may create external ROX without a separately authorized proof/challenge/finality lifecycle.
* [I-19] No single RPC response or single observer may become settlement truth.
* [I-20] No off-chain coordinator may unilaterally finalize value movement.

---

## 2. Design Principles (SHOULD)

* [P-1] Use explicit safe labels in every phase.
* [P-2] Prefer fewer stronger docs over many fragmented docs.
* [P-3] Keep scope, terminology, and decision gates together.
* [P-4] Treat bridge terms as hazardous unless explicitly framed as forbidden, threat-modeled, or future-gated.
* [P-5] Separate QuickChain anchor language from Solana Anchor language.
* [P-6] Treat “planning” as non-operational by default.
* [P-7] Treat “proof sketch” as documentation, not settlement.
* [P-8] Treat “state-machine sketch” as documentation, not runtime logic.
* [P-9] Treat “future Solana Anchor model” as conceptual, not deployed code.
* [P-10] Prefer delayed, challengeable, auditable future models over fast or optimistic language.
* [P-11] Prefer burn/proof/challenge/finality/mint concepts over custody, swap, liquidity, or exchange concepts.
* [P-12] Prefer failure-closed status labels over optimistic completion labels.

---

## 3. Implementation Patterns (HOW)

### 3.1 Current safe label

```text
Internal ROC Beta Phase 6: COMPLETE / GREEN / PARKED.
Internal ROC value-loop proof: COMPLETE / GREEN / PARKED.
QuickChain boundary/preflight through Phase 5: COMPLETE / GREEN / PARKED, not public chain/runtime completion.
Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED.
Bridge / ROX / Solana / staking / liquidity / external settlement: docs / threat-model / decision-gate only.
```

### 3.2 Allowed repository contents in Phase 0

This repository may contain:

```text
docs
blueprints
threat models
state-machine sketches
DTO/proof sketches
checker scripts
review packets
decision gates
```

### 3.3 Forbidden repository contents in Phase 0

This repository may not contain:

```text
Solana program code
Anchor instruction code
Anchor account structs
mint/burn runtime
relayer runtime
coordinator runtime
RPC proof service runtime
CrabLink bridge UI
devnet deployment scripts
mainnet deployment scripts
token mint scripts
staking scripts
liquidity scripts
exchange-facing scripts
value-bearing anything
```

### 3.4 Terminology lock

```text
QuickChain anchor:
  internal evidence/checkpoint/posture artifact
  not balance mutation
  not ROX minting
  not external settlement
  not public chain/runtime completion

Solana Anchor:
  possible future smart contract framework
  not active now
  not deployed now
  not authorized now
  not implied by this repo name

ROX-anchor:
  planning repository
  docs / threat-model / decision-gate only
  not runtime
  not a token launch
  not a deployment
  not a public bridge
```

### 3.5 Required wording distinction

Allowed wording:

```text
future-gated
conceptual
docs-only
threat-modeled
not authorized
decision-gate only
planning repository
non-runtime
```

Forbidden positive wording:

```text
live
launched
active bridge
mainnet ready
users can convert
users can redeem
users can cash out
users can swap
users can stake
users can earn yield
```

### 3.6 Future-safe conceptual path wording

Allowed conceptual wording:

```text
Future ROC → ROX concept:
internal ROC burn request through svc-wallet
→ ron-ledger burn evidence
→ proof package sketch
→ challenge window sketch
→ finality decision gate
→ future external mint request, if separately authorized
```

Allowed conceptual wording:

```text
Future ROX → ROC concept:
external burn evidence
→ proof package sketch
→ challenge window sketch
→ finality decision gate
→ internal ROC issue request through svc-wallet, if separately authorized
```

Forbidden shortcut wording:

```text
ROC converts to ROX
ROX redeems for ROC
instant swap
bridge live
cash out
guaranteed redemption
single-RPC settlement
coordinator-finalized settlement
cache-finalized bridge status
```

### 3.7 Phase 0 core file set

The authoritative Phase 0 docs-only bundle is:

```text
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```

Optional root files such as `README.md` may summarize this repo, but the five files above are the required Phase 0 documentation set.

Do not split the Phase 0 scope into additional charter, UI, scanner, or review-packet documents unless a later docs-expansion gate authorizes it.

The purpose of the five-doc structure is to keep `rox-anchor` small, reviewable, and hard to drift.

---

## 4. Decision Gate Ladder

This repository uses explicit gates. Passing one gate does not authorize the next gate.

### Gate 0 — Docs-Only Planning Gate

Allowed:

```text
IDB
scope doc
threat model
system/state/proof blueprint
testplan/checker doc
docs-only checker script
```

Forbidden:

```text
runtime
skeleton code
Solana program files
Anchor account structs
coordinator code
relayer code
deployment scripts
CrabLink UI
value-bearing anything
```

Safe label after proof:

```text
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This Gate 0 label does **not** authorize:

```text
ROX runtime
Solana runtime
bridge runtime
mint/burn runtime
coordinator runtime
relayer runtime
CrabLink bridge UI
devnet deployment
mainnet deployment
staking
liquidity
external settlement
exchange-facing behavior
user-facing bridge behavior
```

This label means only:

```text
docs-only planning gate is parked
scope is documented
terminology is locked
forbidden runtime is blocked
checker passed
```

### Gate 1 — Threat Model Review Gate

Allowed only after Gate 0 is green:

```text
expanded adversarial review
Grok / external reviewer packets
threat category expansion
risk ranking
mitigation sketches
recovery drill requirements
```

Still forbidden:

```text
runtime
skeleton code
Solana deployment
mint/burn behavior
coordinator service
relayer service
CrabLink bridge UI
```

Safe label after proof:

```text
ROX Anchor Phase 1 — Threat Model Review Gate:
COMPLETE / GREEN / PARKED.
```

This label does **not** authorize runtime.

### Gate 2 — State / Proof Design Gate

Allowed only after Gate 1 is green:

```text
state-machine sketches
proof package sketches
nonce/replay/finality sketches
pause/halt/recovery sketches
upgrade authority sketches
verifiable-build requirements
```

Still forbidden:

```text
Anchor code
Solana account structs
instruction handlers
mint/burn test harnesses
coordinator implementation
relayer implementation
deployment scripts
```

Safe label after proof:

```text
ROX Anchor Phase 2 — State / Proof Design Gate:
COMPLETE / GREEN / PARKED.
```

This label does **not** authorize runtime.

### Gate 3 — Disabled Skeleton Decision Gate

This gate is **not authorized by default**.

It may only be considered after Gates 0–2 are complete and a separate explicit decision authorizes skeleton planning.

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

Still forbidden unless separately authorized later:

```text
value-bearing devnet
public demo
mint/burn execution
external settlement
staking
liquidity
exchange-facing behavior
user-facing bridge path
```

### Gate 9 — Audit / Recovery Drill Gate

No value-bearing devnet, toy-value public bridge demo, external mint/burn, or user-facing ROX bridge path may begin before this class of gate is complete and a later runtime decision gate authorizes the next step.

Required future proof would include:

```text
recovery drills
halt drills
upgrade drills
key rotation drills
quorum failure drills
RPC equivocation drills
challenge griefing drills
verifiable build reproduction
auditor review
```

This gate is referenced for long-term discipline only. It is not active now.

---

## 5. Acceptance Gates (PROOF)

Phase 0 passes only when:

```text
[G-1] the exact Phase 0 core file set exists: 00_IDB_ROX_ANCHOR.md, 01_SCOPE_DECISION_GATE.md, 02_THREAT_MODEL.md, 03_SYSTEM_STATE_PROOF_BLUEPRINT.md, 04_TESTPLAN_CHECKER.md, and scripts/check-rox-anchor-docs-only.sh
[G-2] checker exists
[G-3] checker passes
[G-4] no forbidden runtime directory exists
[G-5] no hidden Solana/Anchor implementation marker exists outside docs-only context
[G-6] every major document includes the North Star
[G-7] every major document declares anchor meaning
[G-8] this scope document includes the current safe label
[G-9] this scope document includes the decision gate ladder
[G-10] this scope document clearly states that docs-only green is not runtime authorization
[G-11] this scope document includes the authoritative five-doc Phase 0 bundle
```

Safe Phase 0 label after proof:

```text
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This means:

```text
docs-only planning gate is parked
scope is documented
terminology is locked
forbidden runtime is blocked
checker passed
```

This does **not** mean:

```text
ROX launched
Solana deployed
bridge live
mint/burn active
external settlement active
CrabLink bridge UI authorized
staking authorized
liquidity authorized
exchange-facing behavior authorized
```

---

## 6. Anti-Scope (Forbidden)

ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT

### 6.1 Forbidden current-scope behavior

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

### 6.2 Forbidden positive claims

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

### 6.3 Forbidden authority paths

```text
rox-anchor → internal ROC mutation
rox-anchor → external ROX mint
CrabLink → bridge finality
CrabLink cache → bridge completion
gateway → bridge settlement
omnigate → bridge settlement
storage/index pointer → paid unlock
accounting snapshot → balance truth
rewarder plan → payout execution
policy allow → receipt truth
external proof → internal issue without svc-wallet
internal burn evidence → external mint without finality gate
off-chain coordinator → finality without challenge close
single RPC → settlement truth
single observer → settlement truth
```

### 6.4 Forbidden hidden implementation drift

The following are forbidden before a later explicit decision gate:

```text
programs/
anchor/
migrations/
src/
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
devnet-deploy/
```

Forbidden hidden markers in implementation-shaped files:

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
TransferChecked
set_authority
spl_token
spl_associated_token_account
invoke_signed
pub mod instructions
pub mod accounts
pub mod state
```

---

## 7. Reviewer Signoff Checklist

Before this document can be considered reviewed, confirm:

```text
[ ] It preserves Internal ROC truth with svc-wallet + ron-ledger.
[ ] It states that rox-anchor is planning, not runtime.
[ ] It blocks ROX runtime.
[ ] It blocks Solana runtime.
[ ] It blocks bridge runtime.
[ ] It blocks mint/burn runtime.
[ ] It blocks staking, liquidity, yield, and exchange-facing behavior.
[ ] It blocks user-facing bridge behavior.
[ ] It distinguishes QuickChain anchor from Solana Anchor.
[ ] It defines a decision gate ladder.
[ ] It names the exact five-doc Phase 0 core file set.
[ ] It says docs-only green is not runtime authorization.
[ ] It says threat-model green is not runtime authorization.
[ ] It says blueprint green is not runtime authorization.
[ ] It keeps future internal issue routed through svc-wallet.
[ ] It blocks single-RPC or single-observer settlement truth.
[ ] It blocks client/cache finality.
[ ] It blocks hidden implementation-shaped files.
[ ] It references the docs-only checker.
```

---

## 8. References

```text
docs/00_IDB_ROX_ANCHOR.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```
