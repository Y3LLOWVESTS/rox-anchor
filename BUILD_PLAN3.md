# ROX Anchor Build Plan 3

## Private Testnet Pilot / Controlled Live-Testnet Operations

Status:

```text
draft / ready / not started
```

Preconditions:

```text
BUILD_PLAN.md is complete / green / parked.
BUILD_PLAN2.md is complete / green / parked.
ROX Anchor local implementation is compile-green and test-green.
ROX Anchor testnet-only hardening is complete.
Private testnet pilot readiness has been proven by tests and readiness gates.
```

This is the third ROX Anchor implementation build plan.

The purpose is to move from **testnet readiness** to a **controlled private testnet pilot**.

This plan may introduce carefully gated, explicit, capped, private testnet operations.

This plan does **not** authorize public launch, mainnet deployment, public ROX mint/burn, production bridge settlement, production ROC release, exchange-facing behavior, staking, liquidity, or any real user value movement.

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

Allowed in this plan:

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

If a future phase needs public users, real ROC movement, production settlement, mainnet, public mint/burn, exchange/liquidity/staking behavior, or public bridge UI, that must become a separate future build plan.

---

## 1. Current Starting Point

The completed first build plan produced:

```text
rox-anchor-core shared type foundation
rox-anchor-proof deterministic proof validation
rox-anchor-cli local inspection
rox-anchor-rpc-proof local evidence/quorum model
rox-anchor-coordinator local decision model
rox-anchor-relayer local dry-run model
compile-tested Anchor program
Anchor state rules
local ROC ↔ ROX mint/burn semantics
integration tests
chaos tests
final local green run
```

The completed second build plan produced:

```text
testnet scope locks
testnet configuration model
operator authority/key model
read-only RPC adapter shape
transaction simulation model
test-only mint/token harness
testnet deployment drill checker
capped testnet submission authorization model
end-to-end testnet shadow flow
expanded adversarial tests
observability and audit records
halt/recovery/kill-switch drills
testnet chaos drills
audit preparation docs
testnet readiness gate
```

Current repo posture:

```text
implementation exists
local tests pass
Anchor build/test pass
testnet readiness gate passes
secret failsafe bundle generation passes
private testnet pilot is the next safe workstream
```

---

## 2. Build Rules For This Plan

Every phase must follow the QuickChain-style discipline:

```text
small behavior patch
focused test
cargo fmt
focused cargo test/check
fix first failure
then expand
```

Additional rules for this plan:

```text
Every live testnet action must be explicit.
Every live testnet action must be capped.
Every live testnet action must have a receipt.
Every live testnet action must be distinguishable from simulation.
Every live testnet action must be impossible from default config.
Every live testnet action must reject mainnet-beta.
Every live testnet action must reject production/public mint labels.
Every live testnet action must keep secrets external and redacted.
```

Do not let CLI, coordinator, relayer, RPC proof, or CrabLink create competing rules.

`rox-anchor-core` and `rox-anchor-proof` remain the shared source of truth.

The Anchor program remains the owner of on-chain state transitions.

RustyOnions `svc-wallet -> ron-ledger` remains the only future path for real internal ROC mutation. ROX Anchor must not directly issue or release real ROC.

---

## 3. Clippy Checkpoint Policy

Run focused Clippy at natural quarter checkpoints:

```text
after Phase 4
after Phase 8
after Phase 12
after Phase 16
```

Use focused Clippy first:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Only run workspace Clippy when the focused surfaces are green.

---

## Phase 0 — Freeze BUILD_PLAN2 Green Baseline

Purpose:

Create a clean starting point for the private testnet pilot.

Work:

```text
1. Confirm BUILD_PLAN.md remains parked.
2. Confirm BUILD_PLAN2.md remains parked.
3. Confirm BUILD_PLAN3.md is the active plan.
4. Confirm cargo tests still pass.
5. Confirm Anchor build/test still pass.
6. Confirm testnet readiness gate still passes.
7. Confirm no tracked testnet key material exists.
8. Confirm generated codebundle remains secret-safe.
```

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_testnet_readiness_gate.sh .
bash scripts/make_codebundle.sh
```

Exit condition:

```text
The repo starts BUILD_PLAN3 from a known green and secret-safe state.
```

---

## Phase 1 — Private Pilot Operator Workspace Hygiene

Purpose:

Prepare the operator-side environment without committing secrets.

Work:

```text
1. Define the expected private pilot local directory layout.
2. Keep all keys, RPC URLs, deployment outputs, and pilot receipts outside tracked source.
3. Extend ignore rules only if new local artifact names are needed.
4. Add a checker that rejects tracked pilot key material and raw RPC/provider tokens.
5. Add tests for the checker.
6. Document the local-only artifact layout in docs/audit or docs/pilot.
7. Do not generate or load real keypairs in tests.
```

Likely files:

```text
.gitignore
scripts/check_private_pilot_hygiene.sh
crates/rox-anchor-cli/tests/private_pilot_hygiene.rs
docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
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
The private pilot has a safe local workspace model and tracked files cannot include common key/RPC secret material.
```

---

## Phase 2 — External Testnet Config Loader

Purpose:

Move from in-code config models to an explicit external pilot config format without storing secrets.

Work:

```text
1. Define a private pilot config shape.
2. Keep RPC URL and keypair paths externally supplied.
3. Reject missing mode.
4. Reject mainnet-beta.
5. Reject public/production labels.
6. Redact RPC URLs, tokenized paths, signatures, and key paths in all reports.
7. Add fixture configs that contain no secrets.
8. Add tests for valid testnet config, missing fields, mainnet rejection, and redaction.
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

## Phase 3 — Testnet Program Artifact Manifest

Purpose:

Track deployment artifacts safely without treating them as production truth.

Work:

```text
1. Define a non-secret deployment manifest.
2. Record program ID, cluster, build hash, IDL hash, deploy slot if supplied, and operator-visible labels.
3. Reject mainnet-beta cluster.
4. Reject empty program ID.
5. Reject mismatched program ID versus Anchor.toml expected testnet/devnet binding.
6. Redact local paths.
7. Add tests for manifest validation and redacted display.
8. Do not treat manifest presence as proof of successful deployment.
```

Likely files:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/tests/testnet_program_manifest.rs
docs/pilot/TESTNET_PROGRAM_MANIFEST.md
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
The private pilot can describe a deployed testnet program artifact without storing secrets or claiming production finality.
```

---

## Phase 4 — Private Testnet Deployment Drill / Optional Deployment

Purpose:

Perform or prepare the first real private testnet deployment using external keys, without treating the drill as production launch.

Work:

```text
1. Build the Anchor program.
2. Confirm IDL generation.
3. Confirm external deploy keypair path.
4. Confirm external payer path.
5. Confirm external upgrade authority policy.
6. Confirm no deploy output is committed.
7. Produce a redacted deployment drill report.
8. If an actual deploy is run, capture only non-secret metadata in a local ignored artifact.
9. Add tests that the drill report never claims production launch or public token availability.
```

Likely files:

```text
scripts/check_private_testnet_deploy.sh
docs/pilot/PRIVATE_TESTNET_DEPLOYMENT_RUNBOOK.md
crates/rox-anchor-cli/tests/private_testnet_deploy_drill.rs
```

Commands:

```bash
anchor build
bash scripts/check_private_testnet_deploy.sh .
cargo test -p rox-anchor-cli --test private_testnet_deploy_drill
cargo test --workspace
```

Optional manual command, only after the checker is green and external keys are ready:

```bash
# anchor deploy --provider.cluster testnet --provider.wallet /external/path/to/testnet-payer.json
```

Exit condition:

```text
A private testnet deployment drill is reproducible, external-key-only, redacted, and non-production.
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

---

## Phase 5 — Test-Only Mint Initialization Runbook

Purpose:

Initialize private testnet state with test-only assets.

Work:

```text
1. Define the test-only ROX mint initialization checklist.
2. Require explicit testnet mode.
3. Require explicit test-only mint label.
4. Require tiny supply/cap limits.
5. Require mint authority separation.
6. Require halt/recovery authorities.
7. Reject public/production labels.
8. Produce redacted initialization intent reports.
9. Add tests for initialization intent validation.
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

## Phase 6 — Live Read-Only RPC Verification Against Testnet

Purpose:

Verify deployed private testnet state through live read-only RPC.

Work:

```text
1. Add or harden the live read-only RPC command path.
2. Fetch current slot.
3. Fetch deployed program account status.
4. Fetch config account status if supplied.
5. Fetch mint account status if supplied.
6. Fetch token account status if supplied.
7. Convert live reads into existing RPC proof observations.
8. Reject stale, missing, disputed, or mismatched observations.
9. Keep all submission disabled.
10. Add fake-adapter tests for every failure case.
```

Likely files:

```text
crates/rox-anchor-rpc-proof/src/rpc.rs
crates/rox-anchor-rpc-proof/src/quorum.rs
crates/rox-anchor-rpc-proof/src/readiness.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/tests/private_testnet_read_only_rpc.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-cli --test private_testnet_read_only_rpc
cargo test --workspace
```

Optional manual command, only after tests pass and external config is provided:

```bash
# cargo run -p rox-anchor-cli -- proof read-only --config /external/path/to/private-testnet.toml
```

Exit condition:

```text
The private pilot can verify testnet state using read-only RPC without any transaction submission.
```

---

## Phase 7 — Simulation-Only Pilot Transaction Plans

Purpose:

Create actual pilot transaction plans but do not send them.

Work:

```text
1. Build transaction plan models for initialize, observe, challenge, resolve, halt, recover, and finalize.
2. Require accepted proof before simulation.
3. Require accepted coordinator decision before simulation.
4. Require relayer dry-run acceptance before simulation.
5. Require read-only RPC verification before simulation.
6. Reject halted, challenged, recovery-required, stale, disputed, or mismatched inputs.
7. Produce deterministic simulation reports.
8. Add tests proving simulation cannot bypass proof/coordinator/relayer/read-only RPC gates.
```

Likely files:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-coordinator/src/decision.rs
crates/rox-anchor-cli/src/commands/submit.rs
crates/rox-anchor-relayer/tests/private_pilot_simulation.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test private_pilot_simulation
cargo test -p rox-anchor-coordinator
cargo test -p rox-anchor-cli
cargo test --workspace
```

Exit condition:

```text
Private pilot transaction plans can be simulated only after all prior proof and read-only gates pass.
```

---

## Phase 8 — Explicit Capped Private Testnet Sender

Purpose:

Introduce the first real send-capable path, still disabled by default and restricted to private testnet.

Work:

```text
1. Add explicit send-capable adapter behind TestnetSubmitCapped.
2. Require explicit operator approval.
3. Require external config.
4. Require testnet/devnet cluster.
5. Reject mainnet-beta.
6. Require accepted proof review.
7. Require accepted coordinator decision.
8. Require relayer dry-run acceptance.
9. Require successful simulation.
10. Require retry cap.
11. Require amount cap.
12. Require operation cap.
13. Require receipt output path.
14. Require no pending halt/challenge/recovery blocker.
15. Add tests proving every missing guard blocks send authorization.
16. Do not make send the default behavior of any command.
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

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test private_testnet_sender
cargo test -p rox-anchor-cli --test private_testnet_submit_command
cargo test --workspace
```

Optional manual command, only after tests pass and external config is provided:

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

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

---

## Phase 9 — Pilot Receipt Ledger and Audit Trail

Purpose:

Make every private pilot action inspectable and replay-resistant before full pilot flows are attempted.

Work:

```text
1. Define pilot receipt IDs.
2. Define receipt chain/hash linkage if needed.
3. Persist proof review, RPC quorum, coordinator decision, simulation result, send authorization, transaction signature when present, and readback verification.
4. Redact secrets and sensitive paths.
5. Reject duplicate receipt IDs.
6. Reject mismatched operation IDs.
7. Reject receipts claiming live submission when no send occurred.
8. Reject receipts claiming production settlement.
9. Add deterministic display tests.
```

Likely files:

```text
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/audit.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-relayer/tests/private_pilot_receipts.rs
crates/rox-anchor-cli/tests/private_pilot_receipt_display.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test private_pilot_receipts
cargo test -p rox-anchor-cli --test private_pilot_receipt_display
cargo test --workspace
```

Exit condition:

```text
Every private pilot action has a redacted, deterministic, replay-resistant receipt trail.
```

---

## Phase 10 — Pilot CLI Command Surface

Purpose:

Make private pilot operations usable without unsafe defaults before running full ROC↔ROX pilot flows.

Work:

```text
1. Add or harden pilot status commands.
2. Add read-only proof command.
3. Add simulation-only command.
4. Add capped submit command.
5. Add receipt inspect command.
6. Add halt/recovery drill command support for pilot mode.
7. Require explicit flags for every non-read-only path.
8. Reject unknown or ambiguous flags.
9. Add help output tests proving production/mainnet/public launch modes do not exist.
```

Likely files:

```text
crates/rox-anchor-cli/src/commands/mod.rs
crates/rox-anchor-cli/src/commands/proof.rs
crates/rox-anchor-cli/src/commands/submit.rs
crates/rox-anchor-cli/src/commands/status.rs
crates/rox-anchor-cli/tests/private_pilot_cli.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --test private_pilot_cli
cargo test -p rox-anchor-cli
cargo test --workspace
```

Exit condition:

```text
The CLI can run private pilot read-only, simulation, capped submit, status, and receipt inspection paths without unsafe defaults.
```

---

## Phase 11 — CrabLink / Internal ROC Dry-Run Adapter

Purpose:

Prepare the future CrabLink/RustyOnions handoff before running ROC↔ROX pilot flows, without real ROC mutation.

Work:

```text
1. Define a dry-run internal ROC burn-intent input shape.
2. Define a dry-run internal ROC release-intent output shape.
3. Keep all values test-only.
4. Require explicit non-production mode.
5. Do not call svc-wallet.
6. Do not mutate ron-ledger.
7. Do not unlock paid content.
8. Do not let CrabLink claim final settlement.
9. Add tests for display-safe statuses.
10. Add tests proving no wallet/ledger mutation call path exists.
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

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-core
cargo test -p rox-anchor-coordinator --test internal_roc_dry_run_adapter
cargo test -p rox-anchor-cli --test crablink_private_pilot_status
cargo test --workspace
```

Exit condition:

```text
ROX Anchor can exchange dry-run intent/status shapes with future CrabLink/RustyOnions integration without mutating real ROC or claiming final settlement.
```

---

## Phase 12 — Private ROC-to-ROX Testnet Pilot Flow

Purpose:

Run the forward bridge shape privately with test-only inputs, after receipt, CLI, and dry-run adapter surfaces exist.

Work:

```text
1. Model a CrabLink/internal ROC burn-intent input.
2. Do not burn real ROC.
3. Require explicit test operation ID.
4. Require explicit idempotency key.
5. Require nonce.
6. Review proof package.
7. Run RPC quorum review.
8. Run coordinator decision.
9. Run relayer dry-run.
10. Run simulation.
11. If explicitly approved, send a capped private testnet transaction that mints only test ROX.
12. Persist receipt.
13. Verify receipt by read-only RPC.
14. Add replay, mismatch, wrong mint, wrong token account, stale RPC, and halted-path tests.
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/private_roc_to_rox_pilot.rs
crates/rox-anchor-relayer/tests/private_roc_to_rox_pilot.rs
crates/rox-anchor-rpc-proof/tests/private_roc_to_rox_readback.rs
crates/rox-anchor-cli/tests/private_roc_to_rox_command.rs
docs/pilot/ROC_TO_ROX_PRIVATE_PILOT.md
```

Commands:

```bash
cargo fmt -p rox-anchor-coordinator -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-coordinator --test private_roc_to_rox_pilot
cargo test -p rox-anchor-relayer --test private_roc_to_rox_pilot
cargo test -p rox-anchor-rpc-proof --test private_roc_to_rox_readback
cargo test -p rox-anchor-cli --test private_roc_to_rox_command
cargo test --workspace
```

Exit condition:

```text
A private ROC-to-ROX pilot flow can mint only test ROX after every proof, simulation, cap, and receipt gate passes.
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
```

---

## Phase 13 — Private ROX-to-ROC Testnet Pilot Flow

Purpose:

Run the reverse bridge shape privately without real internal ROC release.

Work:

```text
1. Observe test ROX burn evidence.
2. Do not release real ROC.
3. Produce internal ROC release-intent only.
4. Require explicit test operation ID.
5. Require explicit idempotency key.
6. Require nonce.
7. Review proof package.
8. Run RPC quorum review.
9. Run coordinator decision.
10. Run relayer dry-run.
11. Run simulation where applicable.
12. Persist receipt.
13. Verify burn/readback by read-only RPC.
14. Add tests proving no direct internal ROC mutation can occur.
15. Add tests proving any future real ROC path must be delegated to svc-wallet/ron-ledger.
```

Likely files:

```text
crates/rox-anchor-coordinator/tests/private_rox_to_roc_pilot.rs
crates/rox-anchor-relayer/tests/private_rox_to_roc_pilot.rs
crates/rox-anchor-rpc-proof/tests/private_rox_to_roc_readback.rs
crates/rox-anchor-cli/tests/private_rox_to_roc_command.rs
docs/pilot/ROX_TO_ROC_PRIVATE_PILOT.md
```

Commands:

```bash
cargo fmt -p rox-anchor-coordinator -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-coordinator --test private_rox_to_roc_pilot
cargo test -p rox-anchor-relayer --test private_rox_to_roc_pilot
cargo test -p rox-anchor-rpc-proof --test private_rox_to_roc_readback
cargo test -p rox-anchor-cli --test private_rox_to_roc_command
cargo test --workspace
```

Exit condition:

```text
A private ROX-to-ROC pilot flow can prove test ROX burn evidence and produce only a dry-run internal ROC release-intent, with no real ROC mutation.
```

---

## Phase 14 — Live Testnet Chaos and Incident Drills

Purpose:

Prove private testnet operations fail safely under realistic conditions.

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

Commands:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Optional manual private testnet drills after local tests pass:

```bash
# read-only RPC outage drill
# simulation-only failure drill
# capped-submit cap-exceeded drill
# halt-during-pending-operation drill
# readback-missing-after-send drill
```

Exit condition:

```text
Private testnet chaos drills fail safely, deterministically, and with inspectable receipts.
```

---

## Phase 15 — Authority, Upgrade, Halt, and Recovery Operational Drills

Purpose:

Practice the human/operator safety procedures before any broader pilot.

Work:

```text
1. Run upgrade authority checklist.
2. Run mint authority checklist.
3. Run halt authority checklist.
4. Run recovery authority checklist.
5. Run wrong-authority rejection drill.
6. Run key rotation intent drill.
7. Run halted-system read-only status drill.
8. Run recovery-from-halt drill.
9. Produce redacted operator reports.
10. Add tests proving authority drills never expose private keys or claim production safety.
```

Likely files:

```text
docs/pilot/PRIVATE_PILOT_AUTHORITY_DRILLS.md
docs/pilot/PRIVATE_PILOT_HALT_RECOVERY_DRILLS.md
crates/rox-anchor-core/tests/private_pilot_authority_drills.rs
crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
cargo test -p rox-anchor-core --test private_pilot_authority_drills
cargo test -p rox-anchor-cli --test private_pilot_drill_reports
cargo test --workspace
anchor build
anchor test
```

Exit condition:

```text
Private pilot authority, upgrade, halt, and recovery procedures are tested, redacted, and operator-readable.
```

---

## Phase 16 — Private Testnet Pilot Closeout Gate

Purpose:

Decide whether ROX Anchor has completed the private testnet pilot and is ready for a separate future production-readiness plan.

Work:

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

Likely files:

```text
docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md
scripts/check_private_testnet_pilot_closeout.sh
crates/rox-anchor-cli/tests/private_testnet_pilot_closeout.rs
```

Commands:

```bash
cargo fmt --all
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test -p rox-anchor-cli --test private_testnet_pilot_closeout
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Exit condition:

```text
ROX Anchor private testnet pilot is complete / green / parked.
A future production-readiness plan may be drafted, but no public/mainnet/production behavior is authorized by this plan.
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

## Final Status For This Plan

Successful completion means:

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

Successful completion does **not** mean:

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

## Build Order Summary

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

## First Command For The Next Session

Start with:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_testnet_readiness_gate.sh .
bash scripts/make_codebundle.sh
```

Then begin Phase 0 by confirming the green baseline and making `BUILD_PLAN3.md` the active next-phase plan.
