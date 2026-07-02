---

title: ROX Anchor System, State, and Proof Blueprint
version: 0.3.0
status: draft
last-updated: 2026-07-01
audience: contributors, auditors, reviewers
scope: docs-only / threat-model / decision-gate
-----------------------------------------------

# ROX Anchor System, State, and Proof Blueprint

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.

RO:WHAT — Sketches the future conceptual system, state machine, proof package, finality model, and pause/recovery model without creating runtime code.
RO:WHY — Gives reviewers a shared model for future ROC ↔ ROX planning while keeping all value movement unauthorized.
RO:INTERACTS — Internal ROC burn/issue concepts, svc-wallet, ron-ledger, future proof packages, future Solana observations, challenge/finality planning, future display-only CrabLink status.
RO:INVARIANTS — Sketches are not runtime; proof packages are not settlement; future internal issue must route through svc-wallet; future finality must be delayed, challengeable, and separately authorized.
RO:SECURITY — No single RPC, cache, coordinator, relayer, observer, proof package, or UI may claim finality or settlement truth.
RO:TEST — `bash scripts/check-rox-anchor-docs-only.sh`.

Anchor meaning used in this document: ROX-anchor planning repo.

ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

---

## 0. Scope and Non-Authorization Notice

This document is a conceptual blueprint.

It does **not** authorize:

```text id="ymghsb"
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

A conceptual path is not an implemented path.

A state-machine sketch is not runtime logic.

A proof-package sketch is not settlement.

A finality-decision sketch is not finality.

A green blueprint gate is not runtime authorization.

This blueprint must be re-reviewed and expanded before any later disabled skeleton, implementation-shaped, devnet, runtime, deployment, or user-facing decision gate is considered.

---

## 1. Invariants (MUST)

* [I-1] This blueprint is sketch-only.
* [I-2] This blueprint does not define executable code.
* [I-3] This blueprint does not authorize Solana program code.
* [I-4] This blueprint does not authorize mint/burn runtime.
* [I-5] This blueprint does not authorize coordinator or relayer runtime.
* [I-6] This blueprint does not authorize RPC proof service runtime.
* [I-7] This blueprint does not authorize CrabLink bridge UI.
* [I-8] Any future ROC → ROX path must begin with internal ROC truth and must not bypass `svc-wallet`.
* [I-9] Any future ROX → ROC path must end with internal issue through `svc-wallet`, never direct ledger mutation.
* [I-10] `ron-ledger` remains durable internal ROC receipt, replay, conservation, and balance truth.
* [I-11] No proof package is finality by itself.
* [I-12] No state may imply instant completion.
* [I-13] No state may imply guaranteed redemption, swap, exchange, liquidity, yield, staking, cash-out, or public bridge readiness.
* [I-14] No single RPC response may become settlement truth.
* [I-15] No single observer may become settlement truth.
* [I-16] No coordinator or relayer may unilaterally finalize value movement.
* [I-17] No cache may become bridge status truth.
* [I-18] CrabLink must remain strictly display-only for any future bridge status.
* [I-19] CrabLink must never construct proofs, claim finality, or treat cached/offline state as settlement truth.
* [I-20] Recovery must never become hidden mint, hidden issue, or manual balance mutation.
* [I-21] Any future proof package must bind source domain, target domain, direction, operation, nonce, cluster, program, mint, and commitment evidence.
* [I-22] Any future deployment-shaped artifact must require verifiable/reproducible build evidence before release consideration.
* [I-23] `FinalizedByDecisionGate` is planning terminology only until a later explicit runtime decision gate authorizes any real finality mechanism.
* [I-24] Future Solana / Anchor account model notes must not contain pseudo-code, Rust-like syntax, or Anchor-style account definitions during this docs-only phase.

---

## 2. Design Principles (SHOULD)

* [P-1] Model state transitions before modeling code.
* [P-2] Model failure states as first-class states.
* [P-3] Prefer delayed and challengeable finality.
* [P-4] Keep operation identity separate from retry identity.
* [P-5] Bind every future proof to source domain, target domain, direction, cluster, program, mint, operation, and nonce.
* [P-6] Keep UI labels conservative and stale-aware.
* [P-7] Prefer failure-closed status over optimistic completion.
* [P-8] Prefer explicit halt states over ambiguous pending states.
* [P-9] Prefer multi-RPC / multi-observer evidence over single-source evidence.
* [P-10] Prefer state names that describe evidence posture, not user-facing value promises.
* [P-11] Treat every recovery path as a possible exploit path.
* [P-12] Treat every future proof package as evidence, not authority.
* [P-13] Treat every future Solana / Anchor account concept as threat-model material until a later decision gate authorizes disabled skeleton work.

---

## 3. Conceptual System Boundary (HOW)

The future conceptual system has three separated domains.

### 3.1 Internal ROC truth domain

Authoritative internal ROC components:

```text id="1bje4a"
svc-wallet
ron-ledger
```

Rules:

```text id="5vk0t2"
svc-wallet is the internal economic mutation front-door
ron-ledger is durable internal economic truth
internal issue must go through svc-wallet
internal burn must go through svc-wallet
internal receipts must come from ron-ledger
internal balances must come from backend / ledger-derived truth
```

Non-authority components:

```text id="rmn9jy"
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

### 3.2 Future external observation domain

Future external observation sources are untrusted until future-gated proof rules are satisfied.

Untrusted by default:

```text id="jid8h9"
Solana RPC responses
single observer reports
coordinator output
relayer output
indexer output
third-party explorer output
user-provided transaction signatures
off-chain proof package assembly
```

Required future posture:

```text id="xdm82g"
multi-RPC evidence
multi-observer evidence
minimum commitment policy
cluster binding
program binding
mint binding
direction binding
operation binding
nonce binding
challenge window
halt path
```

### 3.3 Display domain

CrabLink may only ever be a display/user-intent surface.

Future display-only status, if separately authorized, must be:

```text id="clod28"
backend-derived
stale-aware
failure-labeled
uncertainty-preserving
non-authoritative
non-finality-claiming
```

Forbidden display behavior:

```text id="bnrutj"
client-side proof construction
client-side finality claim
cache-only bridge completion
offline finality display
local settlement truth
instant conversion language
cash-out language
swap/redeem language
```

---

## 4. Conceptual Future Paths (HOW)

These are future-gated conceptual paths only.

They are not active.

They are not runtime.

They are not implementation plans.

### 4.1 ROC → ROX conceptual path

```text id="0xyetn"
internal user intent
→ future quote / warning / explicit confirmation
→ internal ROC burn request through svc-wallet
→ ron-ledger burn receipt / burn evidence
→ proof package draft
→ multi-observer / multi-RPC evidence collection
→ proof package review
→ challenge window open
→ challenged / expired / halted / finality-eligible branch
→ challenge window close
→ future finality decision gate
→ future external ROX mint request, if separately authorized
```

Required boundary notes:

```text id="q0zavi"
internal burn evidence is not external mint authority
proof package draft is not finality
challenge window close is not automatic mint authority
future external mint request requires separate runtime authorization
```

### 4.2 ROX → ROC conceptual path

```text id="dkrqsa"
external burn evidence observed
→ proof package draft
→ multi-RPC quorum / commitment checks
→ proof package review
→ challenge window open
→ challenged / expired / halted / finality-eligible branch
→ challenge window close
→ future finality decision gate
→ internal ROC issue request through svc-wallet, if separately authorized
→ ron-ledger receipt truth
```

Required boundary notes:

```text id="jqnuzn"
external burn evidence is not internal issue authority
proof package draft is not internal ROC truth
future internal issue must route through svc-wallet
ron-ledger remains durable internal receipt truth
```

### 4.3 Forbidden conceptual shortcuts

```text id="2d0e7x"
instant conversion
client-side settlement
single-RPC settlement
single-observer settlement
coordinator-only finality
relayer-only finality
cache-only completion
direct ledger mutation
direct external mint without internal burn/finality lifecycle
direct internal issue without external burn/finality lifecycle
proof package treated as finality
challenge window skipped for convenience
```

---

## 5. State Machine Sketch (HOW)

### 5.1 Allowed future state labels

```text id="7ghxu6"
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

### 5.2 Forbidden state labels

```text id="d1xpie"
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

### 5.3 State rule

```text id="qqmnf6"
Only FinalizedByDecisionGate may represent a future finality decision, and even that state requires a separately authorized runtime decision gate before it can exist outside documentation.
```

Even the `FinalizedByDecisionGate` state is still a planning concept only. It does not represent active finality and requires a separate runtime decision gate before it can exist outside documentation.

### 5.4 State posture table

| State                   | Meaning                                    | User-facing? |   Authority? | Does not imply                                                              |
| ----------------------- | ------------------------------------------ | -----------: | -----------: | --------------------------------------------------------------------------- |
| Draft                   | Local conceptual planning artifact         |           No |           No | request, proof, finality, bridge activity                                   |
| Requested               | Future request observed by backend         | Future-gated |           No | proof, burn, mint, issue, finality                                          |
| Observed                | Evidence observed but not proven           | Future-gated |           No | correctness, quorum, finality                                               |
| ProofPackaged           | Evidence assembled into proof package      | Future-gated |           No | settlement, mint authority, issue authority                                 |
| EvidenceInsufficient    | Evidence does not meet requirements        | Future-gated |           No | progress toward finality                                                    |
| QuorumDisputed          | Observers/RPCs disagree                    | Future-gated |           No | finality eligibility                                                        |
| ChallengeOpen           | Challenge window is open                   | Future-gated |           No | successful completion                                                       |
| Challenged              | Challenge exists                           | Future-gated |           No | failure or finality by itself                                               |
| ChallengeRejected       | Challenge rejected by rules                | Future-gated |           No | automatic finality                                                          |
| ChallengeAccepted       | Challenge accepted by rules                | Future-gated |           No | automatic recovery or user value movement                                   |
| Expired                 | Request expired                            | Future-gated |           No | refund, recovery, mint, issue                                               |
| FinalityEligible        | Candidate for finality decision            | Future-gated |           No | finality, settlement, mint, issue                                           |
| FinalizedByDecisionGate | Future finality decision exists            | Future-gated | Future-gated | active bridge, live ROX, cash-out, client settlement, runtime authorization |
| Failed                  | Flow failed closed                         | Future-gated |           No | recovery, refund, mint, issue                                               |
| RecoveryQueued          | Recovery review required                   | Future-gated |           No | recovery success, manual issue, manual mint                                 |
| Recovered               | Recovery completed through authorized path | Future-gated | Future-gated | hidden mint, hidden issue, manual balance mutation                          |
| HaltRequested           | Halt request exists                        | Future-gated |           No | halt completed, finality, settlement                                        |
| Halted                  | Flow is halted                             | Future-gated |           No | failure, recovery, finality                                                 |
| ResumeEligible          | Conditions for future resume are met       | Future-gated |           No | resumed state, finality                                                     |
| Abandoned               | Flow ended without finality                | Future-gated |           No | recovery, refund, mint, issue                                               |

No state may imply active bridge behavior while the project remains docs / threat-model / decision-gate only.

No state may imply runtime authorization.

No state may imply user-facing value movement.

---

## 6. Transition Rules (HOW)

### 6.1 Allowed conceptual transitions

```text id="2ogy2t"
Draft → Requested
Requested → Observed
Observed → EvidenceInsufficient
Observed → ProofPackaged
ProofPackaged → QuorumDisputed
ProofPackaged → ChallengeOpen
ChallengeOpen → Challenged
ChallengeOpen → Expired
ChallengeOpen → FinalityEligible
Challenged → ChallengeRejected
Challenged → ChallengeAccepted
ChallengeAccepted → Failed
ChallengeRejected → ChallengeOpen
FinalityEligible → FinalizedByDecisionGate
Failed → RecoveryQueued
RecoveryQueued → Recovered
Any non-final state → HaltRequested
HaltRequested → Halted
Halted → ResumeEligible
ResumeEligible → ChallengeOpen
Any non-final state → Abandoned
```

### 6.2 Forbidden conceptual transitions

```text id="de5cpd"
Draft → FinalizedByDecisionGate
Requested → FinalizedByDecisionGate
Observed → FinalizedByDecisionGate
ProofPackaged → FinalizedByDecisionGate without ChallengeOpen
ChallengeOpen → FinalizedByDecisionGate before challenge close
QuorumDisputed → FinalizedByDecisionGate
EvidenceInsufficient → FinalizedByDecisionGate
Halted → FinalizedByDecisionGate
Failed → FinalizedByDecisionGate
Recovered → mint/issue without authorized path
Any state → external mint without future runtime decision gate
Any state → internal issue without svc-wallet
Any state → client-side finality
Any state → cache finality
```

### 6.3 Failure-closed rule

When evidence is incomplete, disputed, stale, ambiguous, replayed, or outside binding requirements, the conceptual flow must move toward:

```text id="eajr4a"
EvidenceInsufficient
QuorumDisputed
Challenged
Failed
RecoveryQueued
Halted
Abandoned
```

It must not move toward:

```text id="2ns0c0"
FinalityEligible
FinalizedByDecisionGate
external mint
internal issue
```

---

## 7. Proof Package Sketch (HOW)

Future proof package sketches must include:

```text id="5u87td"
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

Optional future fields, if separately authorized:

```text id="9n1x12"
evidence_hash
receipt_hash
burn_receipt_reference
ledger_receipt_reference
supply_reconciliation_reference
upgrade_epoch
policy_epoch
proof_package_hash
```

Rules:

```text id="qqap5s"
operation_id is durable operation identity
idempotency_key is retry identity only
nonce prevents replay
source_domain and target_domain prevent domain confusion
cluster prevents devnet/mainnet confusion
program_id prevents program spoofing
mint prevents asset spoofing
direction prevents cross-direction replay
transaction_signature is evidence, not finality
slot and block_time are evidence, not finality
RPC quorum prevents single-observer truth
challenge windows prevent instant finality
finality_decision_reference must point to a separately authorized decision path
halt_status can block finality
recovery_status must not bypass svc-wallet
```

---

## 8. Proof Validation Sketch (HOW)

Future proof validation, if separately authorized, must conceptually check:

```text id="7iaxzj"
schema_version is supported
operation_id is unique
nonce is unique within domain/direction
idempotency_key is not treated as authority
source_domain is expected
target_domain is expected
direction is expected
cluster is expected
program_id is expected
mint is expected
token_account is expected
transaction_signature is bound
slot is within acceptable range
commitment_level meets minimum policy
rpc_quorum_observations agree
observer_set meets policy
challenge_window_open_time is valid
challenge_window_close_time has not been skipped
challenge_status permits next state
halt_status does not block transition
recovery_status does not bypass normal authority
```

Validation output may only be:

```text id="x8mv27"
EvidenceInsufficient
QuorumDisputed
ChallengeOpen
Challenged
Expired
FinalityEligible
Failed
RecoveryQueued
Halted
```

Validation output must not directly be:

```text id="r18rsh"
external mint
internal issue
settlement
cash-out
conversion
client finality
cache finality
```

---

## 9. Pause, Halt, Recovery, and Upgrade Sketch (HOW)

Future pause/halt planning must include:

```text id="qq7z9y"
safe default halted state
manual halt procedure
automatic halt triggers
pending finalization behavior
stuck challenge behavior
recovery case classification
recovery issue path through svc-wallet only
upgrade delay
upgrade authority review
verifiable/reproducible build evidence
artifact hash record
source revision binding
dependency lock evidence
auditor reproduction path
```

Halt triggers should include:

```text id="899ubs"
RPC quorum disagreement
replay suspicion
nonce collision
cluster mismatch
program mismatch
mint mismatch
challenge spam above threshold
coordinator compromise suspicion
relayer compromise suspicion
upgrade authority compromise suspicion
reproducible build mismatch
CrabLink stale/finality display incident
```

Recovery must never become:

```text id="yy12d8"
hidden mint path
hidden issue path
coordinator override path
manual balance mutation path
operator discretion settlement path
```

---

## 10. Future Solana / Anchor Account Model Notes

This section is conceptual only.

It does not define Rust structs.

It does not define Anchor accounts.

It does not define instructions.

Even conceptual notes in this section must not contain pseudo-code, Rust-like syntax, or Anchor-style account definitions. All such content remains strictly forbidden until a later explicit decision gate authorizes disabled skeleton work.

Any future Solana/Anchor account model, if separately authorized, must bind:

```text id="h1iup6"
cluster
program_id
mint
direction
operation_id
nonce
challenge window
halt status
upgrade epoch
authority set
```

Forbidden before later authorization:

```text id="69j7b6"
#[program]
declare_id!
#[derive(Accounts)]
anchor_lang
anchor_spl
Context<
Program<
Account<
Signer<
MintTo
TransferChecked
invoke_signed
pub mod instructions
pub mod accounts
pub mod state
```

Any future account model sketch must remain documentation-only until an explicit later gate authorizes disabled skeleton work.

---

## 11. CrabLink Display Status Sketch

CrabLink must remain display-only.

If future bridge status display is separately authorized, acceptable labels should prefer:

```text id="6t0nnx"
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

Forbidden labels:

```text id="eclxxv"
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

CrabLink must never:

```text id="2rsv4g"
construct bridge proofs
store finality truth
claim finality from cache
unlock value from bridge status
display offline state as complete
call direct mint/burn functions
call Solana RPC as authority
```

---

## 12. Acceptance Gates (PROOF)

This blueprint is acceptable only when it includes:

```text id="3bmwt5"
[G-1] conceptual ROC → ROX path
[G-2] conceptual ROX → ROC path
[G-3] conceptual system boundaries
[G-4] allowed states
[G-5] forbidden states
[G-6] state posture table with "does not imply" column
[G-7] transition rules
[G-8] forbidden transitions
[G-9] proof package field sketch
[G-10] proof validation sketch
[G-11] nonce/replay/finality requirements
[G-12] pause/halt/recovery requirements
[G-13] upgrade/verifiable-build requirements
[G-14] future Solana/Anchor account model notes without pseudo-code, Rust-like syntax, or Anchor-style account definitions
[G-15] CrabLink display-only status sketch
[G-16] explicit no-runtime language
[G-17] explicit statement that blueprint green is not runtime authorization
[G-18] explicit re-review requirement before any skeleton/runtime gate
[G-19] explicit clarification that FinalizedByDecisionGate remains planning-only until a later runtime gate
```

Safe label after this document is reviewed and checker-passing:

```text id="mqky9o"
ROX Anchor Phase 2 — State / Proof Design Gate:
COMPLETE / GREEN / PARKED.
```

This label does **not** authorize runtime.

---

## 13. Anti-Scope (Forbidden)

ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT

This document must not create:

```text id="686rpq"
Solana program code
Anchor account structs
instruction handlers
coordinator service
relayer service
RPC proof service
CrabLink bridge UI
devnet deployment scripts
mainnet deployment scripts
mint/burn test harnesses
token program integration
staking logic
liquidity logic
exchange-facing logic
```

This document must not claim:

```text id="hgdwzo"
future path is active
ROX is live
bridge is live
settlement is live
users can convert
users can redeem
users can cash out
users can swap
users can stake
users can earn yield
mainnet is ready
public bridge beta is ready
```

This document must not weaken:

```text id="9sd1v9"
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

## 14. Reviewer Checklist

Before this blueprint can be considered reviewed, confirm:

```text id="sj2mca"
[ ] It states that the blueprint is sketch-only.
[ ] It states that blueprint green is not runtime authorization.
[ ] It preserves Internal ROC truth with svc-wallet + ron-ledger.
[ ] It requires any future internal issue to route through svc-wallet.
[ ] It treats proof packages as evidence, not finality.
[ ] It blocks single-RPC settlement truth.
[ ] It blocks single-observer settlement truth.
[ ] It blocks coordinator/relayer unilateral finality.
[ ] It defines conceptual ROC → ROX path.
[ ] It defines conceptual ROX → ROC path.
[ ] It defines allowed states.
[ ] It defines forbidden states.
[ ] It clarifies what states do not imply.
[ ] It clarifies that FinalizedByDecisionGate is planning-only until a later runtime gate.
[ ] It defines allowed transitions.
[ ] It defines forbidden transitions.
[ ] It defines proof package fields.
[ ] It defines proof validation posture.
[ ] It includes halt/recovery/upgrade requirements.
[ ] It keeps future Solana/Anchor notes free of pseudo-code, Rust-like syntax, and Anchor-style account definitions.
[ ] It keeps CrabLink display-only.
[ ] It avoids code, structs, instructions, deployment scripts, and runtime files.
[ ] It references the docs-only checker.
```

---

## 15. References

```text id="ufg9d4"
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```
