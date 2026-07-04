# ROX Anchor Build Plan 2

## Testnet-Only Production Readiness / Security Hardening

Status:

```text
draft
```

Precondition:

```text
BUILD_PLAN.md is complete / green / parked.
ROX Anchor local implementation is compile-green and test-green.
This plan starts after the final local green run.
```

This is the second ROX Anchor implementation build plan.

The purpose is to move from a local compile-tested implementation to a controlled, testnet-only hardening surface.

This is **not** a public launch plan.

This is **not** a mainnet plan.

This is **not** an exchange/liquidity/staking plan.

This is **not** authorization to make public ROX mint/burn available.

---

## 0. Non-Negotiable Scope Boundary

The following remain forbidden in this build plan:

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

Allowed in this plan:

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

If a future phase needs anything beyond this boundary, it must become a separate build plan.

---

## 1. Current Starting Point

The first build plan produced:

```text
rox-anchor-core shared type foundation
rox-anchor-proof deterministic proof review
rox-anchor-cli local inspection
rox-anchor-rpc-proof local evidence model
rox-anchor-coordinator local decision model
rox-anchor-relayer local dry-run model
compile-tested Anchor program
Anchor state rules
local ROC ↔ ROX mint/burn semantics
integration tests
chaos tests
final local green run
```

Final local green means:

```text
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

all pass locally.

This plan starts from that green state and hardens toward controlled testnet readiness.

---

## 2. Build Rules For This Plan

Every phase must follow the same QuickChain-style discipline:

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

Do not let CLI, coordinator, relayer, or testnet adapters invent competing state rules.

`rox-anchor-core` and `rox-anchor-proof` remain the source of shared truth.

The Anchor program remains the owner of on-chain state transitions.

---

## Phase 0 — Freeze Local Green Baseline

Purpose:

Lock the completed local implementation baseline before adding testnet-only surfaces.

Work:

```text
1. Record current green commands.
2. Confirm BUILD_PLAN.md remains unchanged.
3. Confirm BUILD_PLAN2.md is the active next-phase plan.
4. Confirm all new work is testnet/localnet only.
5. Confirm no mainnet/public token behavior exists.
```

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
The repo is still green before testnet hardening begins.
```

---

## Phase 1 — Testnet Scope Locks and Safety Flags

Purpose:

Add explicit runtime/config boundaries so testnet code cannot accidentally become public launch code.

Work:

```text
1. Add explicit environment mode enum:
   - LocalOnly
   - TestnetOnly
   - ProductionDisabled
2. Add explicit cluster allowlist:
   - localnet
   - devnet/testnet, if configured
   - no mainnet-beta
3. Add a submission mode enum:
   - DryRunOnly
   - SimulateOnly
   - TestnetSubmitCapped
4. Add compile/runtime checks that reject mainnet-beta.
5. Add tests proving mainnet-beta is rejected.
6. Add tests proving public launch flags do not exist.
7. Add tests proving default mode is non-submitting.
```

Files likely touched:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-core/src/state.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-rpc-proof/src/config.rs
crates/rox-anchor-coordinator/src/config.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-coordinator
cargo test -p rox-anchor-core
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-coordinator
cargo test --workspace
```

Exit condition:

```text
The code has explicit safety boundaries preventing accidental production/mainnet behavior.
```

---

## Phase 2 — Testnet Configuration Model

Purpose:

Create a testnet configuration surface without secrets or committed key material.

Work:

```text
1. Define non-secret testnet config structs.
2. Separate config from secrets.
3. Require RPC URL to be externally supplied.
4. Require payer/keypair path to be externally supplied.
5. Reject missing explicit mode.
6. Reject mainnet-beta endpoints by cluster label.
7. Redact sensitive paths and URLs in reports.
8. Add config parsing tests.
```

Files likely touched:

```text
crates/rox-anchor-rpc-proof/src/config.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-coordinator/src/config.rs
crates/rox-anchor-cli/src/commands/check.rs
crates/rox-anchor-cli/src/commands/status.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-cli
```

Exit condition:

```text
Testnet configuration can be represented, validated, and redacted without storing secrets.
```

---

## Phase 3 — Operator Key and Authority Safety Model

Purpose:

Model safe testnet authority handling before any live testnet submission.

Work:

```text
1. Define operator role types:
   - observer
   - coordinator
   - relayer
   - upgrade authority
   - mint authority
   - halt authority
   - recovery authority
2. Define authority separation checks.
3. Reject one key owning every critical authority unless explicitly test-only.
4. Add key redaction helpers.
5. Add tests for role separation.
6. Add tests for wrong authority rejection.
7. Add tests for authority rotation intent.
8. Do not load real keypairs yet.
```

Files likely touched:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-core/src/state.rs
programs/rox-anchor/src/state.rs
crates/rox-anchor-cli/src/commands/status.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor -p rox-anchor-cli
cargo test -p rox-anchor-core
cargo test -p rox-anchor
cargo test -p rox-anchor-cli
```

Exit condition:

```text
The system has a tested authority model before any testnet key use.
```

---

## Phase 4 — Read-Only Live RPC Adapter

Purpose:

Allow testnet/devnet RPC reading without transaction submission.

Work:

```text
1. Add read-only RPC adapter shape.
2. Fetch current slot.
3. Fetch account existence/status.
4. Fetch transaction/signature status if supplied.
5. Convert live read observations into existing rpc-proof observation structs.
6. Keep all submission disabled.
7. Add adapter trait so tests can use fake RPC.
8. Add tests for stale, missing, mismatched, and disputed live-read-shaped evidence.
```

Files likely touched:

```text
crates/rox-anchor-rpc-proof/src/rpc.rs
crates/rox-anchor-rpc-proof/src/quorum.rs
crates/rox-anchor-rpc-proof/src/readiness.rs
crates/rox-anchor-cli/src/commands/proof.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-cli
cargo test --workspace
```

Exit condition:

```text
Read-only testnet RPC observations can feed the existing proof path without live submission.
```

---

## Phase 5 — Transaction Simulation Model

Purpose:

Add simulation-first transaction planning before any real testnet send.

Work:

```text
1. Define transaction plan structs.
2. Define simulated transaction result structs.
3. Require proof acceptance before simulation.
4. Require coordinator acceptance before simulation.
5. Require relayer dry-run acceptance before simulation.
6. Reject blocked/challenged/halted/recovery-required proof reviews.
7. Add tests proving simulation cannot bypass dry-run state.
8. Do not send transactions.
```

Files likely touched:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/readiness.rs
crates/rox-anchor-coordinator/src/decision.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-coordinator
cargo test --workspace
```

Exit condition:

```text
A transaction can be planned and simulated conceptually only after the local proof/coordinator/relayer path accepts it.
```

---

## Phase 6 — Test-Only Mint and Token Account Harness

Purpose:

Create a controlled testnet/localnet token harness without public mint/burn.

Work:

```text
1. Define test-only mint fixture metadata.
2. Define test-only token account fixture metadata.
3. Keep real public mint identifiers out of defaults.
4. Require explicit testnet mode.
5. Require explicit test mint label.
6. Enforce tiny caps for test amounts.
7. Add tests for cap enforcement.
8. Add tests rejecting production/public mint labels.
9. Add tests for token account binding mismatch.
```

Files likely touched:

```text
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/initialize.rs
programs/rox-anchor/src/instructions/finalize.rs
crates/rox-anchor-core/src/types.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor
cargo test -p rox-anchor-core
cargo test -p rox-anchor
anchor build
anchor test
```

Exit condition:

```text
The program and local crates can reason about test-only mint/token-account fixtures while rejecting public/production semantics.
```

---

## Phase 7 — Testnet Deployment Drill

Purpose:

Practice testnet deployment procedure without treating it as production.

Work:

```text
1. Add documented local command checklist for testnet deploy drill.
2. Confirm program ID handling.
3. Confirm IDL generation.
4. Confirm upgrade authority location is external and non-committed.
5. Confirm deploy keypair path is ignored by git.
6. Confirm no local secrets are committed.
7. Add script checks for forbidden key files.
8. Add script checks for mainnet-beta rejection.
```

Files likely touched:

```text
scripts/
Anchor.toml
.gitignore
```

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Optional manual testnet-only command, only when ready:

```bash
anchor build
# anchor deploy --provider.cluster testnet
```

Exit condition:

```text
A testnet deployment drill can be performed with external keys and no committed secrets.
```

---

## Phase 8 — Capped Testnet Submission Path

Purpose:

Allow a strictly capped testnet-only transaction send path after read-only RPC and simulation are green.

Work:

```text
1. Add TestnetSubmitCapped mode.
2. Require explicit CLI flag or config value.
3. Require accepted proof review.
4. Require accepted coordinator decision.
5. Require relayer dry-run acceptance.
6. Require successful simulation.
7. Require retry cap.
8. Require amount cap.
9. Require operation cap per run.
10. Require receipt persistence.
11. Reject live submission in LocalOnly and SimulateOnly modes.
12. Add tests proving every guard blocks submission when missing.
```

Files likely touched:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/retry.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/src/commands/check.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-cli
cargo test --workspace
```

Exit condition:

```text
The only live submission path is testnet-only, capped, explicit, receipt-backed, and impossible to trigger from default mode.
```

---

## Phase 9 — End-to-End Testnet Shadow Flow

Purpose:

Run the ROC ↔ ROX shape against testnet-only fixtures without real ROC release or public ROX.

Work:

```text
1. Model ROC-to-ROX testnet flow with fake/internal test input.
2. Model ROX-to-ROC testnet flow with fake/internal test output.
3. Do not release real ROC.
4. Do not expose public mint/burn.
5. Use explicit test operation IDs.
6. Use explicit test idempotency keys.
7. Persist receipts.
8. Prove replay is rejected.
9. Prove mismatches are rejected.
10. Prove halt/challenge/recovery blockers work in testnet mode.
```

Files likely touched:

```text
crates/rox-anchor-coordinator/tests/
crates/rox-anchor-relayer/tests/
crates/rox-anchor-rpc-proof/tests/
programs/rox-anchor/src/state.rs
```

Commands:

```bash
cargo fmt --all
cargo test --workspace
anchor build
anchor test
```

Exit condition:

```text
The testnet shadow flow proves the full shape without production value movement.
```

---

## Phase 10 — Security Regression Expansion

Purpose:

Expand adversarial tests beyond the first local chaos suite.

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

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
Known adversarial cases are covered by deterministic tests before any broader testnet use.
```

---

## Phase 11 — Observability, Receipts, and Audit Trail

Purpose:

Make every testnet action inspectable and explainable.

Work:

```text
1. Add structured testnet receipt shape.
2. Redact secrets and signatures as needed.
3. Include proof decision.
4. Include coordinator decision.
5. Include RPC quorum result.
6. Include relayer retry result.
7. Include simulation result.
8. Include submission result only in capped testnet mode.
9. Include halt/challenge/recovery posture.
10. Add CLI display tests.
11. Add deterministic report tests.
```

Files likely touched:

```text
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/redaction.rs
crates/rox-anchor-coordinator/src/redaction.rs
crates/rox-anchor-rpc-proof/src/redaction.rs
crates/rox-anchor-cli/src/commands/status.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-relayer
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-cli
```

Exit condition:

```text
Every testnet-relevant action has a safe, deterministic, redacted receipt/report.
```

---

## Phase 12 — Halt, Recovery, and Kill-Switch Drills

Purpose:

Prove we can stop unsafe behavior before any public exposure.

Work:

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

Commands:

```bash
cargo fmt --all
cargo test --workspace
anchor build
anchor test
```

Exit condition:

```text
Halt and recovery behavior is proven across local, simulated, and capped testnet modes.
```

---

## Phase 13 — Testnet Chaos Drills

Purpose:

Run realistic failure drills before any broader testnet rollout.

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

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Optional testnet-only manual drills after local tests pass:

```bash
# read-only RPC drill
# simulation-only drill
# capped testnet submission drill
```

Exit condition:

```text
Testnet chaos drills fail safely, deterministically, and with inspectable receipts.
```

---

## Phase 14 — Audit Preparation

Purpose:

Prepare the repo for an external or self-directed security review.

Work:

```text
1. Create invariant-to-test map.
2. Create authority map.
3. Create state transition map.
4. Create RPC trust boundary map.
5. Create relayer submission boundary map.
6. Create mint/burn boundary map.
7. Create halt/recovery runbook.
8. Create key rotation runbook.
9. Create deployment rollback runbook.
10. Create known non-goals list.
11. Confirm no public launch behavior exists.
```

Files likely added:

```text
docs/audit/INVARIANT_TEST_MAP.md
docs/audit/AUTHORITY_MODEL.md
docs/audit/STATE_TRANSITIONS.md
docs/audit/RPC_BOUNDARY.md
docs/audit/RELAYER_BOUNDARY.md
docs/audit/HALT_RECOVERY_RUNBOOK.md
docs/audit/TESTNET_DEPLOYMENT_RUNBOOK.md
```

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
The implementation has audit-ready maps and runbooks tied to real tests.
```

---

## Phase 15 — Testnet Readiness Gate

Purpose:

Decide whether ROX Anchor is ready for a private testnet pilot.

This is not a public launch gate.

Work:

```text
1. Confirm all local tests pass.
2. Confirm all Anchor tests pass.
3. Confirm all testnet-only guards pass.
4. Confirm mainnet-beta is rejected.
5. Confirm no public mint/burn path exists.
6. Confirm no public bridge UI exists.
7. Confirm no exchange/staking/liquidity behavior exists.
8. Confirm operator runbooks exist.
9. Confirm receipt/reporting works.
10. Confirm halt/recovery drills work.
11. Confirm testnet capped submission works only when explicitly enabled.
12. Confirm all known adversarial cases are tested or documented.
```

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Optional manual testnet-only proof:

```bash
# read-only testnet RPC proof
# simulation-only testnet proof
# capped private testnet submit proof
```

Exit condition:

```text
ROX Anchor is ready for a private testnet-only pilot.
No public token launch is authorized.
No public bridge is authorized.
No production settlement is authorized.
```

---

## Final Status For This Plan

Successful completion means:

```text
ROX Anchor testnet-only hardening is complete / green / parked.
Private testnet pilot readiness is proven.
Public launch remains unauthorized.
Production bridge remains unauthorized.
Mainnet remains unauthorized.
Public mint/burn remains unauthorized.
Exchange/liquidity/staking remains unauthorized.
```

---

## Build Order Summary

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

---

## First Command For The Next Session

Start with:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Then begin Phase 0 by confirming the green baseline and checking that `BUILD_PLAN.md` remains parked while `BUILD_PLAN2.md` is the active testnet-only hardening plan.
