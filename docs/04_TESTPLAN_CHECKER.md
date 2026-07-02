---

title: ROX Anchor Testplan and Docs-Only Checker
version: 0.3.0
status: draft
last-updated: 2026-07-01
audience: contributors, auditors, reviewers
scope: docs-only / threat-model / decision-gate
-----------------------------------------------

# ROX Anchor Testplan and Docs-Only Checker

> **North Star:** Internal ROC truth stays with `svc-wallet` + `ron-ledger`. ROX-anchor is planning, not runtime. Bridge remains docs / threat-model / decision-gate only.

RO:WHAT — Defines the Phase 0 docs-only acceptance checker and review testplan for the `rox-anchor` repository.
RO:WHY — Makes the no-runtime boundary executable and prevents hidden bridge/Solana/Anchor implementation drift.
RO:INTERACTS — All `docs/*.md` files, repository file layout, forbidden runtime-shaped files, hidden implementation markers, and future reviewer gates.
RO:INVARIANTS — Checker green means docs-only planning gate, not runtime authorization.
RO:SECURITY — Blocks forbidden directories, hidden implementation markers, missing scope/meaning declarations, product-language drift, and implementation-shaped files before authorization.
RO:TEST — `bash scripts/check-rox-anchor-docs-only.sh`.

Anchor meaning used in this document: ROX-anchor planning repo.

ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT

---

## 0. Scope and Non-Authorization Notice

This document defines a static docs-only checker.

The checker does **not** authorize:

```text id="tv445g"
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

The checker is a planning gate.

The checker is not an audit.

The checker is not a security proof.

The checker is not runtime validation.

The checker is not bridge validation.

The checker is not Solana validation.

A green checker means only:

```text id="uimz1s"
required docs exist
required doctrine markers exist
forbidden runtime-shaped files/directories were not found
hidden Solana/Anchor implementation markers were not found outside allowed docs-only context
the repo remains docs / threat-model / decision-gate only
```

A green checker does **not** mean:

```text id="cqbgsl"
ROX is live
bridge is live
Solana code exists
bridge behavior is authorized
mint/burn behavior is authorized
external settlement is active
CrabLink bridge UI is authorized
staking or liquidity is authorized
```

---

## 1. Invariants (MUST)

* [I-1] The checker must run from the `rox-anchor` repo root.
* [I-2] The checker must verify the exact Phase 0 core file set exists.
* [I-3] The checker must verify the North Star appears in every major doc.
* [I-4] The checker must verify every major doc declares anchor meaning.
* [I-5] The checker must verify every major doc contains required RO headers.
* [I-6] The checker must fail if forbidden runtime directories exist.
* [I-7] The checker must fail if hidden implementation markers exist in implementation-shaped files.
* [I-8] The checker must fail if executable Solana/Anchor/coordinator/relayer files appear before authorization.
* [I-9] The checker must fail if implementation-shaped files appear outside allowed docs/checker scope.
* [I-10] The checker must fail if forbidden product/runtime language appears outside explicit allowed context.
* [I-11] The checker must not run builds.
* [I-12] The checker must not call RPC endpoints.
* [I-13] The checker must not call wallet commands.
* [I-14] The checker must not run simulations.
* [I-15] The checker must not create runtime artifacts.
* [I-16] The checker must be static-only.
* [I-17] A passing checker does not authorize runtime.
* [I-18] A passing checker does not authorize later gates.
* [I-19] Any checker failure blocks the Phase 0 docs-only safe label.
* [I-20] Any future expansion of the checker must preserve the docs-only / threat-model / decision-gate boundary.
* [I-21] Phase 0 remains limited to the five required docs plus the checker unless a later docs-expansion decision gate authorizes more.
* [I-22] Harmless root metadata may exist only if it does not introduce build, runtime, dependency, deployment, token, wallet, or bridge behavior.

---

## 2. Design Principles (SHOULD)

* [P-1] Prefer simple shell checks first.
* [P-2] Make failure messages explicit and actionable.
* [P-3] Avoid requiring external dependencies.
* [P-4] Scan for hidden implementation by content, not just directory names.
* [P-5] Treat docs-only green as a planning milestone only.
* [P-6] Fail closed when ambiguous files appear.
* [P-7] Prefer false positives over silent runtime drift.
* [P-8] Keep the checker readable enough for auditors.
* [P-9] Keep the checker deterministic and local-only.
* [P-10] Make forbidden paths and markers visible in this document before encoding them in the script.
* [P-11] Keep Phase 0 small: five docs plus one checker.
* [P-12] Treat any runtime-shaped addition as unsafe until a later decision gate explicitly authorizes it.
* [P-13] Allow harmless repository metadata only when it is non-executable and non-runtime.

---

## 3. Phase 0 Core File Set (HOW)

The authoritative Phase 0 docs-only bundle is:

```text id="27nz0b"
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
docs/04_TESTPLAN_CHECKER.md
scripts/check-rox-anchor-docs-only.sh
```

Optional root files such as `README.md` may summarize this repository, but the five docs above plus the checker script are the required Phase 0 proof surface.

Phase 0 is intentionally limited to these five documents plus the checker. Additional planning documents require a later docs-expansion decision gate.

Do not split Phase 0 into additional charter, UI, scanner, reviewer-packet, or implementation documents unless a later docs-expansion gate authorizes it.

---

## 4. Harmless Metadata Allowlist (HOW)

The checker may allow common root-level repository metadata if it is non-runtime, non-build, and non-deployment.

Allowed root metadata examples:

```text id="8xg0a6"
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

These files must not contain:

```text id="81d0qn"
build commands
deployment commands
Solana commands
Anchor commands
wallet commands
token mint commands
bridge runtime instructions
coordinator runtime instructions
relayer runtime instructions
RPC service instructions
staking instructions
liquidity instructions
exchange-facing instructions
```

Allowlisted metadata does not weaken the five-doc Phase 0 requirement.

Allowlisted metadata does not authorize extra planning documents.

Allowlisted metadata does not authorize runtime-shaped files.

---

## 5. Required Document Markers (HOW)

Every required doc must include:

```text id="5w594n"
North Star:
RO:WHAT
RO:WHY
RO:INVARIANTS
RO:SECURITY
RO:TEST
Anchor meaning used in this document:
docs / threat-model / decision-gate
```

Every required doc should include at least one explicit context marker where high-risk terms are discussed:

```text id="ks3ddm"
ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT
```

The checker should treat missing required markers as a failure.

---

## 6. Safe Label Checks (HOW)

The checker must verify that the repo contains the current safe doctrine:

```text id="ghup3u"
Internal ROC Beta Phase 6: COMPLETE / GREEN / PARKED.
Internal ROC value-loop proof: COMPLETE / GREEN / PARKED.
QuickChain boundary/preflight through Phase 5: COMPLETE / GREEN / PARKED, not public chain/runtime completion.
Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED.
Bridge / ROX / Solana / staking / liquidity / external settlement: docs / threat-model / decision-gate only.
```

The checker must verify the Phase 0 safe label appears in the proper docs/checker output:

```text id="fvs13b"
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This label means only that the docs-only planning gate passed.

It does not authorize runtime.

---

## 7. Forbidden Runtime Directories (HOW)

The checker must fail if any of these directories exist before a later explicit decision gate authorizes implementation-shaped work:

```text id="g0nyqm"
programs
anchor
migrations
app
src
relayer
coordinator
crablink-bridge-ui
solana-program
token-mint
mint
burn
stake
staking
liquidity
dex
cex
mainnet
devnet
devnet-deploy
deployment
deploy
rpc-proof
proof-service
bridge
bridge-ui
wallet
token
```

Reason:

```text id="w3i2g2"
These names imply runtime, deployment, bridge, token, wallet, coordinator, relayer, staking, liquidity, or external settlement behavior.
```

At Phase 0, such directories are forbidden even if empty.

---

## 8. Forbidden Runtime-Shaped Files (HOW)

At Phase 0, implementation-shaped files outside allowed docs/checker scope are forbidden.

Implementation-shaped extensions include:

```text id="phu8oq"
.rs
.ts
.tsx
.js
.jsx
.mjs
.cjs
.toml
.json
.yaml
.yml
.lock
.so
.dylib
.a
.o
.wasm
```

The checker should fail if such files appear outside allowed docs/checker locations, except for explicitly harmless repository metadata if allowlisted.

Examples of forbidden files:

```text id="lvpl6r"
Anchor.toml
Cargo.toml
package.json
tsconfig.json
src/lib.rs
src/main.rs
programs/*/src/lib.rs
migrations/*.ts
deploy*.sh
solana*.sh
anchor*.sh
mint*.sh
burn*.sh
stake*.sh
liquidity*.sh
bridge*.sh
```

Phase 0 does not need build files.

Phase 0 does not need dependencies.

Phase 0 does not need package managers.

Phase 0 does not need runtime entrypoints.

---

## 9. Hidden Implementation Marker Scanner (HOW)

The checker must scan implementation-shaped files for Solana/Anchor/coordinator/relayer markers.

Forbidden hidden markers include:

```text id="dwitsk"
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
pub mod instructions
pub mod accounts
pub mod state
solana_program
solana_sdk
RpcClient
send_transaction
get_transaction
get_account
get_signature_status
```

These markers are allowed only in docs/checker files where they are explicitly framed as forbidden, threat-modeled, or future-gated.

Forbidden marker rule:

```text id="ckwxk3"
A hidden implementation marker in executable or implementation-shaped files is a blocker finding.
```

Allowed context rule:

```text id="lt2m6x"
The same marker may appear in docs only when clearly documented as forbidden, threat-modeled, or future-gated.
```

---

## 10. Forbidden Product / Runtime Language Scanner (HOW)

The checker should scan required docs for high-risk product/runtime language.

High-risk terms include:

```text id="kpktaq"
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
live bridge
cash-out flow
```

These terms may appear only when explicitly framed as:

```text id="5opv27"
forbidden
anti-scope
threat-modeled
future-gated
not authorized
not runtime
```

Allowed context markers:

```text id="wmut1o"
ROX-ANCHOR:ANTI-SCOPE-CONTEXT
ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT
ROX-ANCHOR:THREAT-MODEL-CONTEXT
ROX-ANCHOR:FUTURE-GATED-CONTEXT
```

The checker may be conservative.

False positives are acceptable.

Silent product-language drift is not acceptable.

---

## 11. Static-Only Execution Contract (HOW)

The checker may run:

```text id="ukpslq"
bash -n scripts/check-rox-anchor-docs-only.sh
grep
find
test
printf
wc
sed
awk
```

The checker must not run:

```text id="tfegot"
cargo build
cargo test
cargo clippy
npm install
npm build
npm test
anchor build
anchor test
anchor deploy
solana-test-validator
solana program deploy
spl-token
RPC calls
wallet commands
token mint commands
bridge simulation
coordinator service
relayer service
```

The checker must not require:

```text id="cu0g2m"
Rust toolchain
Node toolchain
Solana CLI
Anchor CLI
network access
wallet access
private keys
environment secrets
```

The checker must be local, deterministic, and static.

---

## 12. Checker Output Contract (HOW)

On success, the checker must print:

```text id="rvbzxd"
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

It must also print a non-authorization reminder:

```text id="zkb9gb"
this does not authorize ROX runtime, Solana runtime, bridge runtime, staking, liquidity, external settlement, or user-facing bridge behavior
```

On failure, the checker must:

```text id="ns3hjz"
print FAIL lines
name the failing file or directory
name the missing marker or forbidden marker
exit non-zero
not modify repo contents
not attempt repair
```

The checker should aggregate failures when possible so reviewers can fix multiple issues in one pass.

---

## 13. Manual Reviewer Testplan (HOW)

After the checker passes, reviewers should manually confirm:

```text id="yh5jpd"
[ ] The five-doc set exists.
[ ] No extra planning docs are required for Phase 0.
[ ] The North Star appears in every required doc.
[ ] Every required doc declares anchor meaning.
[ ] docs/00_IDB_ROX_ANCHOR.md defines the constitution, invariants, anti-scope, and IDB method.
[ ] docs/01_SCOPE_DECISION_GATE.md defines the five-doc bundle, terminology lock, and decision gate ladder.
[ ] docs/02_THREAT_MODEL.md covers off-chain coordination, RPC, replay, internal boundary, external lifecycle, upgrade, recovery, UI, product-language, and hidden implementation risks.
[ ] docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md remains sketch-only and defines conceptual paths, states, transitions, proof fields, validation posture, halt/recovery, and CrabLink display-only rules.
[ ] docs/04_TESTPLAN_CHECKER.md defines static-only proof, checker behavior, forbidden directories, hidden markers, product-language checks, and manual reviewer duties.
[ ] No runtime-shaped directories exist.
[ ] No implementation-shaped files exist outside allowed scope.
[ ] Harmless root metadata, if present, contains no build/runtime/deployment/bridge/token/wallet behavior.
[ ] No Solana/Anchor markers exist outside docs/checker context.
[ ] No product language implies live bridge behavior.
[ ] No doc claims runtime authorization.
[ ] No doc claims ROX launch.
[ ] No doc claims Solana deployment.
[ ] No doc claims external settlement.
```

---

## 14. Phase Gate Interpretation

### 14.1 Phase 0 — Docs-Only Planning Gate

Checker green may support this label:

```text id="s9fbs9"
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This means:

```text id="0yx2d8"
the repo has the required docs
the repo has the checker
the checker passed
the docs-only boundary is preserved
```

This does **not** mean:

```text id="1uczgq"
threat model is complete
state/proof design is final
runtime is authorized
skeleton code is authorized
Solana code is authorized
bridge behavior is authorized
```

### 14.2 Later gates

Later labels require separate review.

Examples:

```text id="9rxc0y"
ROX Anchor Phase 1 — Threat Model Review Gate:
COMPLETE / GREEN / PARKED.

ROX Anchor Phase 2 — State / Proof Design Gate:
COMPLETE / GREEN / PARKED.
```

These labels also do **not** authorize runtime.

Any skeleton/runtime/devnet/deployment/user-facing gate requires a later explicit decision.

---

## 15. Acceptance Gates (PROOF)

Phase 0 checker passes only when:

```text id="cjj1oo"
[G-1] exact Phase 0 core file set exists
[G-2] required script exists
[G-3] shell syntax is valid
[G-4] every required doc has RO:WHAT
[G-5] every required doc has RO:WHY
[G-6] every required doc has RO:INVARIANTS
[G-7] every required doc has RO:SECURITY
[G-8] every required doc has RO:TEST
[G-9] every required doc has North Star
[G-10] every required doc has anchor meaning declaration
[G-11] every required doc has docs / threat-model / decision-gate scope language
[G-12] current safe labels exist
[G-13] Phase 0 safe label exists
[G-14] forbidden directories are absent
[G-15] forbidden runtime-shaped files are absent outside allowed scope, except harmless allowlisted metadata
[G-16] allowlisted metadata contains no build/runtime/deployment/bridge/token/wallet behavior
[G-17] hidden implementation markers are absent from implementation-shaped files
[G-18] high-risk product/runtime language appears only in allowed context
[G-19] checker is static-only
[G-20] checker does not run build, deploy, RPC, wallet, mint, simulation, coordinator, or relayer commands
[G-21] checker prints clear green label and non-authorization reminder on success
[G-22] Phase 0 remains limited to five docs plus checker unless a later docs-expansion gate authorizes more
```

Final pass label:

```text id="lh03pv"
ROX Anchor Phase 0 — Docs-Only Planning Gate:
COMPLETE / GREEN / PARKED.
```

This label does **not** authorize runtime.

---

## 16. Anti-Scope (Forbidden)

ROX-ANCHOR:FORBIDDEN-SCOPE-CONTEXT

The checker must not create or run:

```text id="fd6qev"
Solana build
Anchor build
cargo build
cargo test
cargo clippy
npm install
npm build
npm test
deployment commands
RPC calls
wallet commands
token mint commands
bridge simulation
coordinator service
relayer service
```

The checker must not require:

```text id="sazr5g"
secrets
private keys
wallet files
network access
Solana CLI
Anchor CLI
Rust build
Node install
```

The checker must not claim:

```text id="5mkv3c"
risk is solved
bridge is live
ROX is live
runtime is authorized
settlement is active
Solana integration is active
users can convert
users can redeem
users can cash out
users can stake
users can earn yield
```

The checker is static-only.

---

## 17. Reviewer Checklist

Before this testplan/checker doc can be considered reviewed, confirm:

```text id="yujcl3"
[ ] It states that checker green is not runtime authorization.
[ ] It defines the exact Phase 0 core file set.
[ ] It reinforces that Phase 0 is limited to five docs plus checker.
[ ] It defines harmless root metadata rules.
[ ] It defines required document markers.
[ ] It defines safe label checks.
[ ] It defines forbidden runtime directories.
[ ] It defines forbidden runtime-shaped files.
[ ] It defines hidden implementation markers.
[ ] It defines product/runtime language checks.
[ ] It defines static-only execution rules.
[ ] It forbids builds, deploys, RPC calls, wallet commands, mint commands, simulations, coordinator services, and relayer services.
[ ] It defines checker success output.
[ ] It defines checker failure behavior.
[ ] It explicitly names the role of each of the other four core docs.
[ ] It defines manual reviewer checks.
[ ] It defines Phase 0 gate interpretation.
[ ] It references the docs-only checker script.
```

---

## 18. References

```text id="y1g05l"
docs/00_IDB_ROX_ANCHOR.md
docs/01_SCOPE_DECISION_GATE.md
docs/02_THREAT_MODEL.md
docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md
scripts/check-rox-anchor-docs-only.sh
```
