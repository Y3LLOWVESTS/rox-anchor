# ROX Anchor

**Status:** In build process / scaffolded / phase-gated.

ROX Anchor is the reserved planning and future implementation repository for the RustyOnions / CrabLink ROC ↔ ROX anchor path.

This project is currently being built out as a structured, phase-gated scaffold. The repository may contain empty placeholder files and directories for future docs, specs, schemas, crates, tests, operations runbooks, audit records, CrabLink display surfaces, and Solana/Anchor program surfaces. Those placeholders are navigation and planning surfaces only.

They do not mean the bridge is live.

They do not mean ROX is live.

They do not mean Solana runtime is active.

They do not mean staking, liquidity, external settlement, exchange-facing behavior, or value-bearing bridge behavior is authorized.

---

## Current Safe Status

```text
ROX Anchor: IN BUILD PROCESS.
Full inert scaffold: allowed as empty placeholders.
Current authorized work: docs, threat model, decision gates, static checkers, and inert scaffold.
Bridge / ROX / Solana / staking / liquidity / external settlement: docs / threat-model / decision-gate only.
No runtime is authorized by this repository state.
```

RustyOnions / CrabLink internal status carried into this repo:

```text
Internal ROC Beta Phase 6: COMPLETE / GREEN / PARKED.
Internal ROC value-loop proof: COMPLETE / GREEN / PARKED.
Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED.
QuickChain boundary/preflight through Phase 5: COMPLETE / GREEN / PARKED, not public chain/runtime completion.
```

---

## North Star

Internal ROC truth stays with:

```text
svc-wallet + ron-ledger
```

ROX Anchor is planning and future gated infrastructure.

Bridge work remains:

```text
docs / threat-model / decision-gate only
```

until a later explicit runtime authorization gate says otherwise.

---

## What This Repository Is

ROX Anchor is a phase-gated build repository for:

```text
ROC ↔ ROX anchor planning
future proof package design
future challenge/finality design
future recovery and halt planning
future non-value local validation
future coordination boundary work
future CrabLink display-only status surfaces
future audit and recovery drill records
future runtime decision documentation
```

At the current stage, the repository is primarily:

```text
a scaffold
a roadmap
a safety boundary
a documentation surface
a checker surface
a future implementation map
```

---

## What This Repository Is Not

This repository is not currently:

```text
a live bridge
a ROX launch
a Solana launch
a staking system
a liquidity system
an exchange-facing integration
a public settlement system
a public validator economy
a cash-out path
a redemption path
a swap path
a deployed Anchor program
```

The presence of future-looking folders such as `crates/`, `programs/`, `schemas/`, `tests/`, `ops/`, `audits/`, or `crablink-bridge-ui/` does not authorize runtime behavior.

Those files are placeholders unless and until a later phase explicitly authorizes content.

---

## Scaffold Policy

The full scaffold may contain empty placeholder files.

Placeholder files must remain:

```text
empty
inert
non-executable
dependency-free
runtime-free
deployment-free
wallet-free
RPC-free
mint/burn-free
staking-free
liquidity-free
external-settlement-free
```

A placeholder file is not permission to implement the feature.

A placeholder manifest is not permission to add dependencies.

A placeholder script is not permission to run commands.

A placeholder Solana/Anchor file is not permission to deploy anything.

---

## Current Build Process

The intended build path is:

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
→ CrabLink display-only status
→ pre-audit hardening
→ audit/recovery drills
→ runtime decision
```

The intended build path is not:

```text
scaffold
→ runtime
→ public bridge
```

Each phase must have its own checker, review, and closeout before the next phase gains authority.

---

## Phase Map

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

Passing one phase does not authorize the next.

A green docs gate does not authorize runtime.

A green scaffold gate does not authorize runtime.

A green audit gate does not authorize runtime by itself.

Only an explicit later runtime decision gate can authorize runtime behavior.

---

## Authority Boundaries

Internal ROC authority remains:

```text
user intent
→ quote / prepare
→ explicit confirmation
→ svc-wallet
→ ron-ledger
→ backend receipt / access
→ paid render
→ display-only cache
→ backend balance refresh
```

Forbidden authority paths include:

```text
direct ledger mutation outside svc-wallet
fake balances
fake receipts
fake finality
silent spend
cache-only paid unlock
client-side settlement
coordinator unilateral finality
relayer unilateral finality
single-RPC settlement truth
manual hidden mint
manual hidden issue
```

---

## CrabLink Boundary

CrabLink may eventually display backend-derived ROX Anchor status if a later phase authorizes it.

CrabLink must remain:

```text
display-only
stale-aware
backend-derived
user-intent only
explicit-confirmation only
```

CrabLink must not:

```text
construct bridge proofs
claim finality from cache
claim offline completion
call direct wallet mutation
call direct ledger mutation
call direct mint/burn instructions
show cash-out or redemption claims unless explicitly authorized later
```

---

## Development Note

This project is in the build process.

Some files may intentionally be empty because they are placeholders for future phases.

Do not treat empty files as broken.

Do not fill future files early.

Do not add dependencies, runtime logic, deployment commands, or user-facing behavior unless the relevant phase gate explicitly authorizes it.

---

## Safety Language

Use safe wording:

```text
planning
scaffold
future-gated
non-value
display-only
backend-derived
evidence
challenge window
decision gate
not runtime authorized
```

Avoid unsafe wording unless framed as forbidden or threat-modeled:

```text
live bridge
cash out
redeem
converted
swap complete
mainnet ready
staking active
liquidity active
settlement complete
guaranteed finality
```

---

## License

This repository should follow the RustyOnions / CrabLink licensing posture unless a later legal review chooses a different license for ROX Anchor.

---

## Final Reminder

ROX Anchor is currently in the build process.

The scaffold gives the project shape.

The gates give the project authority.

Until a later explicit decision says otherwise:

```text
ROX Anchor is planning, not runtime.
Internal ROC truth stays with svc-wallet + ron-ledger.
Bridge remains docs / threat-model / decision-gate only.
```
