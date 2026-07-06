# ROX Anchor Build Plan 4

## Actual Private Testnet Execution / Test-Only Bridge Evidence

Status:

```text
draft / ready / not started
```

Preconditions:

```text
BUILD_PLAN.md is complete / green / parked.
BUILD_PLAN2.md is complete / green / parked.
BUILD_PLAN3.md is complete / green / parked.
ROX Anchor local implementation is compile-green and test-green.
ROX Anchor private pilot software gate is complete.
Anchor build/test passes.
Focused Clippy passes with -D warnings.
Secret-safe codebundle generation passes.
```

This is the fourth ROX Anchor implementation build plan.

The purpose is to move from:

```text
private testnet pilot software readiness
```

to:

```text
actual private Solana devnet/testnet execution with test-only assets,
real deployed testnet program accounts,
real read-only RPC evidence,
real capped testnet transaction receipts,
and no real internal ROC mutation.
```

This plan covers the remaining work for the goal previously estimated as:

```text
Private testnet / test-only bridge goal: approximately 85–90% complete.
```

Successful completion of this plan should bring that private testnet / test-only bridge goal to:

```text
complete / green / parked
```

This plan does **not** authorize:

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
production ROC release
real internal ROC release
real user funds
public bridge UI
public faucet
public claim page
exchange-facing behavior
staking
liquidity pools
market making
custody services
public claim/airdrop pages
uncapped submission
silent wallet/key usage
silent live RPC submission
operator key material committed to repo
raw private keys in logs
fake finality
fake success output
CrabLink display claiming final settlement before backend proof
```

This plan may authorize, only within this plan and only after explicit gates:

```text
private devnet/testnet deployment
external non-committed testnet keypairs
test-only ROX mint
test-only token accounts
test-only amounts
live read-only RPC verification
transaction simulation against deployed testnet accounts
strictly capped private testnet transaction submission
explicit operator approval before every live send path
receipt-backed pilot runs
operator safety drills
halt/recovery drills
authority/key-rotation drills
multi-RPC observation checks
shadow ROC burn-intent modeling
shadow ROC release-intent modeling
CrabLink/internal ROC dry-run status display
no real internal ROC mutation
```

If a future phase needs real ROC movement, production bridge settlement, mainnet-beta, public ROX mint/burn, public bridge UI, public users, exchange-facing behavior, staking, or liquidity, that must remain deferred to BUILD_PLAN5 or a later explicitly authorized plan.

---

## 0. Current Starting Point

The completed prior build plans produced:

```text
rox-anchor-core shared type foundation
rox-anchor-proof deterministic proof validation
rox-anchor-cli local inspection and pilot command surface
rox-anchor-rpc-proof local and read-only RPC evidence model
rox-anchor-coordinator local/private pilot decision model
rox-anchor-relayer dry-run, simulation, capped authorization, and receipt model
compile-tested Anchor program
Anchor state rules
local ROC ↔ ROX mint/burn semantics
test-only mint/token-account harness
private pilot config model
private testnet program manifest model
deployment drill checker
read-only RPC verification model
simulation-only transaction plan model
explicit capped testnet sender model
pilot receipt ledger and audit trail
CrabLink/internal ROC dry-run adapter shape
private ROC-to-ROX test flow model
private ROX-to-ROC test flow model
chaos/incident drills
authority/halt/recovery drills
private testnet pilot closeout gate
```

Current repo posture:

```text
implementation exists
local tests pass
workspace check passes
Anchor build/test passes
focused Clippy passes
secret-safe codebundle generation passes
BUILD_PLAN3 is parked
```

What is missing for the private testnet/test-only goal:

```text
actual external operator workspace populated outside repo
actual private devnet/testnet deploy
actual program artifact manifest from real deploy metadata
actual test-only ROX mint/config initialization
actual live read-only RPC evidence against deployed accounts
actual simulation against deployed account addresses
actual capped testnet ROC-to-ROX test transaction
actual capped testnet ROX-to-ROC test transaction or reverse-flow proof step
actual readback receipts from live testnet signatures
actual pilot receipt ledger from manual runs
actual operational evidence package
```

---

## 1. Build Rules For This Plan

Every phase must follow the same QuickChain-style discipline:

```text
small behavior patch
focused test
cargo fmt
focused cargo test/check
fix first failure
then expand
```

Additional BUILD_PLAN4 rules:

```text
manual live steps must be explicit
manual live steps must be capped
manual live steps must use external non-committed keys
manual live steps must write only to ignored local artifact directories
manual live steps must produce receipts
manual live steps must have readback verification
manual live steps must never touch real ROC
manual live steps must never claim production finality
manual live steps must never become default CLI behavior
```

Any command that can submit a transaction must require:

```text
explicit testnet/devnet cluster
explicit external config
explicit operator approval phrase
explicit receipt output path
explicit operation cap
explicit amount cap
explicit retry cap
accepted proof review
accepted coordinator decision
relayer dry-run acceptance
successful simulation
read-only RPC verification
no halt/challenge/recovery blocker
no mainnet-beta
no production/public labels
```

Default CLI behavior must remain:

```text
read-only or simulation-only
```

---

## 2. Repository and Workspace Rules

All private pilot local artifacts must remain outside tracked source.

Allowed ignored local paths:

```text
.rox-anchor-pilot/
.rox-anchor-private-pilot/
private-pilot/
pilot-artifacts/
pilot-rpc/
pilot-keys/
pilot-keypairs/
pilot-wallets/
pilot-secrets/
pilot-receipts/
pilot-audit/
pilot-deploy/
pilot-ledger/
pilot-tmp/
```

Allowed ignored local file patterns:

```text
*.pilot-config.local.toml
*.pilot-config.local.json
*.pilot-rpc.txt
*.pilot-provider.txt
*.pilot-keypair.json
*.pilot-wallet.json
*.pilot-authority.json
*.pilot-payer.json
*.pilot-receipt.json
*.pilot-audit.json
*.pilot-deploy-output.json
*.pilot-ledger.json
```

Before and after every manual run:

```bash
git status --short
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
```

No private key, seed, mnemonic, payer keypair, authority keypair, provider token, raw RPC URL with token, or deployment output containing sensitive paths may be committed.

---

## 3. Clippy Checkpoint Policy

Run focused Clippy at natural checkpoints:

```text
after Phase 4
after Phase 8
after Phase 12
after Phase 15
```

Focused Clippy commands:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Only run workspace Clippy after focused Clippy is green:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Phase 0 — Freeze BUILD_PLAN3 Green Baseline

Purpose:

Confirm the repo is still at the exact safe baseline before actual private testnet execution.

Work:

```text
1. Confirm BUILD_PLAN.md remains parked.
2. Confirm BUILD_PLAN2.md remains parked.
3. Confirm BUILD_PLAN3.md remains parked.
4. Confirm BUILD_PLAN4.md is the active plan.
5. Confirm no tracked key material exists.
6. Confirm closeout checker still passes.
7. Confirm workspace tests/checks still pass.
8. Confirm Anchor build/test still pass.
9. Confirm focused Clippy still passes.
10. Regenerate a safe codebundle.
```

Commands:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Focused Clippy:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Exit condition:

```text
The BUILD_PLAN3 green baseline is preserved before live private testnet work begins.
```

---

## Phase 1 — External Operator Workspace Actualization

Purpose:

Create the actual external local workspace for private testnet operations without committing secrets.

Work:

```text
1. Create the external private pilot directory outside tracked source.
2. Create a redacted example config in docs only.
3. Create the real local config outside repo.
4. Point to external payer/keypair/authority paths.
5. Point to external RPC URL/provider path.
6. Confirm all local artifacts are ignored.
7. Confirm no external paths leak into committed docs/tests.
8. Add or harden checker coverage for actual artifact names used by the operator.
9. Add tests proving redaction of operator paths and RPC URLs.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
scripts/check_actual_private_testnet_workspace.sh
crates/rox-anchor-cli/tests/actual_private_testnet_workspace.rs
```

Manual local-only artifact examples:

```text
/private/path/rox-anchor-private-pilot/private-testnet.toml
/private/path/rox-anchor-private-pilot/testnet-payer.json
/private/path/rox-anchor-private-pilot/authority-notes.local.txt
/private/path/rox-anchor-private-pilot/receipts/
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
bash scripts/check_actual_private_testnet_workspace.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_workspace
cargo test -p rox-anchor-cli
```

Exit condition:

```text
The private testnet operator workspace exists as an external, redacted, non-committed artifact model.
```

---

## Phase 2 — Actual Anchor Build Artifact Capture

Purpose:

Build the Anchor program and capture non-secret build metadata for the private testnet deploy.

Work:

```text
1. Run anchor build.
2. Confirm IDL generation.
3. Confirm program ID matches Anchor.toml expected devnet/testnet binding.
4. Capture IDL hash.
5. Capture program binary hash.
6. Capture Anchor version, Solana CLI version, and Rust version.
7. Store full local deploy artifacts only in ignored local paths.
8. Store redacted non-secret manifest shape in repo docs/tests.
9. Add tests that manifest presence is not deployment proof by itself.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_BUILD_ARTIFACTS.md
crates/rox-anchor-core/tests/actual_testnet_artifact_manifest.rs
crates/rox-anchor-cli/tests/actual_testnet_artifact_manifest_status.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
anchor build
cargo test -p rox-anchor-core --test actual_testnet_artifact_manifest
cargo test -p rox-anchor-cli --test actual_testnet_artifact_manifest_status
cargo check -p rox-anchor
```

Exit condition:

```text
A real private testnet build artifact manifest can be captured, validated, and redacted without claiming deployment or finality.
```

---

## Phase 3 — Actual Private Devnet/Testnet Deployment

Purpose:

Deploy the Anchor program to private devnet/testnet using external keys, then capture only non-secret metadata.

Work:

```text
1. Confirm external payer path exists outside repo.
2. Confirm external deploy authority path exists outside repo.
3. Confirm cluster is devnet or testnet.
4. Reject mainnet-beta.
5. Run deployment manually only after checker is green.
6. Capture program ID.
7. Capture deploy signature.
8. Capture deploy slot.
9. Capture IDL hash.
10. Capture program binary hash.
11. Capture upgrade authority policy.
12. Write redacted deploy receipt to ignored local artifact path.
13. Update a non-secret testnet manifest if appropriate.
14. Add checker/test coverage for redacted deploy receipt shape.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_DEPLOYMENT.md
scripts/check_actual_private_testnet_deploy_receipt.sh
crates/rox-anchor-cli/tests/actual_private_testnet_deploy_receipt.rs
```

Manual command, only after local checks pass:

```bash
# anchor deploy \
#   --provider.cluster testnet \
#   --provider.wallet /external/path/to/testnet-payer.json
```

Required pre-manual gate:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_deploy.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
anchor build
cargo test --workspace
cargo check --workspace
```

Post-manual gate:

```bash
bash scripts/check_actual_private_testnet_deploy_receipt.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_deploy_receipt
bash scripts/check_private_pilot_hygiene.sh .
```

Exit condition:

```text
The Anchor program has either been actually deployed to private devnet/testnet or the deployment attempt failed safely with a redacted, non-secret receipt.
```

---

## Phase 4 — Test-Only ROX Mint and Program Config Initialization

Purpose:

Initialize the deployed private testnet program with test-only ROX assets and separated authorities.

Work:

```text
1. Create or identify test-only ROX mint on devnet/testnet.
2. Confirm mint label is test-only.
3. Confirm token account label is test-only.
4. Confirm amount/supply cap is tiny.
5. Confirm mint authority is separated.
6. Confirm halt authority is separated.
7. Confirm recovery authority is separated.
8. Confirm upgrade authority policy is external and documented.
9. Initialize program config account.
10. Persist redacted initialization receipt.
11. Verify initialization by read-only RPC.
12. Add tests that initialization receipts cannot imply public mint availability.
```

Likely files:

```text
docs/pilot/ACTUAL_TEST_ONLY_MINT_INITIALIZATION.md
scripts/check_actual_test_only_mint_initialization.sh
crates/rox-anchor-cli/tests/actual_test_only_mint_initialization.rs
crates/rox-anchor-rpc-proof/tests/actual_test_only_mint_readback.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot initialize-test-only-mint \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/init-mint.pilot-receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT"
```

Commands:

```bash
cargo fmt -p rox-anchor-cli -p rox-anchor-rpc-proof -p rox-anchor
bash scripts/check_actual_test_only_mint_initialization.sh .
cargo test -p rox-anchor-cli --test actual_test_only_mint_initialization
cargo test -p rox-anchor-rpc-proof --test actual_test_only_mint_readback
anchor test
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

Exit condition:

```text
The private testnet program is initialized with explicit test-only assets and verified by read-only RPC.
```

---

## Phase 5 — Live Read-Only RPC Evidence Against Deployed Accounts

Purpose:

Prove deployed program/config/mint/token-account status through live read-only RPC before any send.

Work:

```text
1. Query current slot.
2. Query deployed program account.
3. Query program config account.
4. Query test-only ROX mint.
5. Query test token accounts.
6. Query upgrade authority metadata where available.
7. Query recent deploy/init signatures.
8. Convert observations into existing rpc-proof evidence model.
9. Reject stale readbacks.
10. Reject wrong program ID.
11. Reject wrong mint.
12. Reject wrong token account.
13. Reject under-quorum RPC evidence.
14. Persist redacted read-only evidence receipts.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_READ_ONLY_EVIDENCE.md
crates/rox-anchor-rpc-proof/tests/actual_private_testnet_read_only_rpc.rs
crates/rox-anchor-cli/tests/actual_private_testnet_read_only_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot proof read-only \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/read-only-evidence.pilot-receipt.json
```

Commands:

```bash
cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof --test actual_private_testnet_read_only_rpc
cargo test -p rox-anchor-cli --test actual_private_testnet_read_only_command
cargo test --workspace
```

Exit condition:

```text
The deployed private testnet accounts are verified by live read-only RPC evidence without submission.
```

---

## Phase 6 — Simulation Against Actual Deployed Testnet Addresses

Purpose:

Run transaction simulation against real deployed account addresses without sending.

Work:

```text
1. Build ROC-to-ROX transaction plan using deployed program/config/mint/token accounts.
2. Build ROX-to-ROC transaction plan using deployed program/config/mint/token accounts.
3. Require live read-only RPC evidence.
4. Require accepted local proof review.
5. Require accepted coordinator decision.
6. Require relayer dry-run acceptance.
7. Require test-only labels.
8. Require tiny amount caps.
9. Reject wrong account/mint/program bindings.
10. Persist simulation receipts.
11. Add tests that simulation receipts cannot be promoted into send receipts.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_SIMULATION.md
crates/rox-anchor-relayer/tests/actual_private_testnet_simulation.rs
crates/rox-anchor-coordinator/tests/actual_private_testnet_simulation_gate.rs
crates/rox-anchor-cli/tests/actual_private_testnet_simulation_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot simulate \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/simulation.pilot-receipt.json \
#   --simulate-only
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test actual_private_testnet_simulation
cargo test -p rox-anchor-coordinator --test actual_private_testnet_simulation_gate
cargo test -p rox-anchor-cli --test actual_private_testnet_simulation_command
cargo test --workspace
```

Exit condition:

```text
Simulation succeeds or fails safely against actual private testnet addresses without any live send.
```

---

## Phase 7 — Actual Capped Testnet ROC-to-ROX Flow

Purpose:

Execute the forward test-only bridge shape:

```text
shadow internal ROC burn intent
→ proof review
→ RPC evidence
→ coordinator decision
→ relayer dry-run
→ simulation
→ explicit capped private testnet send
→ test-only ROX mint or finalize action
→ readback
→ receipt
```

Work:

```text
1. Create shadow ROC burn-intent only.
2. Do not burn real ROC.
3. Require explicit test operation ID.
4. Require explicit idempotency key.
5. Require nonce.
6. Require accepted proof review.
7. Require live read-only RPC evidence.
8. Require accepted coordinator decision.
9. Require relayer dry-run acceptance.
10. Require successful simulation.
11. Require operator approval.
12. Require tiny amount cap.
13. Require retry cap.
14. Require operation cap.
15. Send exactly one capped private testnet transaction.
16. Persist send receipt.
17. Verify mint/finalize by read-only RPC.
18. Persist readback receipt.
19. Reject receipt if readback does not match expected delta.
20. Add tests for every missing guard.
```

Likely files:

```text
docs/pilot/ACTUAL_ROC_TO_ROX_PRIVATE_TESTNET_RUN.md
crates/rox-anchor-relayer/tests/actual_roc_to_rox_capped_send.rs
crates/rox-anchor-rpc-proof/tests/actual_roc_to_rox_readback.rs
crates/rox-anchor-coordinator/tests/actual_roc_to_rox_decision.rs
crates/rox-anchor-cli/tests/actual_roc_to_rox_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot roc-to-rox \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/roc-to-rox-send.pilot-receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND" \
#   --max-operations 1 \
#   --max-amount-minor 1
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test actual_roc_to_rox_capped_send
cargo test -p rox-anchor-rpc-proof --test actual_roc_to_rox_readback
cargo test -p rox-anchor-coordinator --test actual_roc_to_rox_decision
cargo test -p rox-anchor-cli --test actual_roc_to_rox_command
cargo test --workspace
```

Exit condition:

```text
A capped private testnet ROC-to-ROX run produces test-only ROX evidence and a readback-verified receipt without burning real ROC.
```

---

## Phase 8 — Actual Capped Testnet ROX-to-ROC Flow

Purpose:

Execute the reverse test-only bridge shape:

```text
test-only ROX burn/finalize evidence
→ read-only RPC verification
→ proof review
→ coordinator decision
→ relayer dry-run
→ simulation or capped send where applicable
→ dry-run internal ROC release intent only
→ receipt
```

Work:

```text
1. Burn or observe test-only ROX evidence on private testnet.
2. Do not release real ROC.
3. Produce internal ROC release-intent only.
4. Require explicit test operation ID.
5. Require explicit idempotency key.
6. Require nonce.
7. Require read-only RPC evidence.
8. Require accepted proof review.
9. Require accepted coordinator decision.
10. Require relayer dry-run acceptance.
11. Require successful simulation where applicable.
12. Require operator approval for any live testnet send.
13. Persist burn/observe receipt.
14. Persist dry-run release-intent receipt.
15. Verify readback by RPC.
16. Add tests proving no real ROC mutation path exists.
17. Add tests proving future real ROC release must be delegated to svc-wallet/ron-ledger.
```

Likely files:

```text
docs/pilot/ACTUAL_ROX_TO_ROC_PRIVATE_TESTNET_RUN.md
crates/rox-anchor-relayer/tests/actual_rox_to_roc_capped_send.rs
crates/rox-anchor-rpc-proof/tests/actual_rox_to_roc_readback.rs
crates/rox-anchor-coordinator/tests/actual_rox_to_roc_decision.rs
crates/rox-anchor-cli/tests/actual_rox_to_roc_command.rs
```

Manual command shape:

```bash
# cargo run -p rox-anchor-cli -- pilot rox-to-roc \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipts/rox-to-roc.pilot-receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND" \
#   --max-operations 1 \
#   --max-amount-minor 1
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test actual_rox_to_roc_capped_send
cargo test -p rox-anchor-rpc-proof --test actual_rox_to_roc_readback
cargo test -p rox-anchor-coordinator --test actual_rox_to_roc_decision
cargo test -p rox-anchor-cli --test actual_rox_to_roc_command
cargo test --workspace
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
```

Exit condition:

```text
A capped private testnet ROX-to-ROC run proves test-only ROX burn/readback and produces only a dry-run internal ROC release intent.
```

---

## Phase 9 — Receipt Ledger Reconciliation for Actual Runs

Purpose:

Make every actual private testnet run inspectable, linked, redacted, and replay-resistant.

Work:

```text
1. Collect deploy receipt.
2. Collect initialization receipt.
3. Collect read-only RPC evidence receipt.
4. Collect simulation receipt.
5. Collect ROC-to-ROX send receipt if performed.
6. Collect ROC-to-ROX readback receipt.
7. Collect ROX-to-ROC burn/observe receipt if performed.
8. Collect ROX-to-ROC readback receipt.
9. Link receipts by operation ID.
10. Link receipts by idempotency key.
11. Link receipts by prior receipt hash/link where available.
12. Reject duplicate receipt IDs.
13. Reject mismatched operation IDs.
14. Reject live submission claims without transaction signatures.
15. Reject production settlement claims.
16. Reject real ROC release claims.
17. Redact RPC URLs, key paths, signatures if necessary.
18. Produce operator-readable receipt ledger report.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_RECEIPT_LEDGER.md
scripts/check_actual_private_testnet_receipts.sh
crates/rox-anchor-relayer/tests/actual_private_testnet_receipt_ledger.rs
crates/rox-anchor-cli/tests/actual_private_testnet_receipt_display.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
bash scripts/check_actual_private_testnet_receipts.sh .
cargo test -p rox-anchor-relayer --test actual_private_testnet_receipt_ledger
cargo test -p rox-anchor-cli --test actual_private_testnet_receipt_display
cargo test --workspace
```

Exit condition:

```text
Actual private testnet receipts form a deterministic, redacted, replay-resistant evidence ledger.
```

---

## Phase 10 — Actual Negative Drills Against Deployed Testnet State

Purpose:

Prove live/deployed private testnet operations fail safely under real account bindings.

Required drills:

```text
wrong program ID
wrong mint
wrong token account
wrong authority
missing config account
missing mint account
stale readback
under-quorum RPC evidence
RPC provider disagreement
duplicate operation ID
duplicate idempotency key
nonce reuse
receipt tamper
missing receipt
operator approval omitted
send disabled
cap exceeded
halt before simulation
halt after simulation before send
halt after send before readback
recovery during pending operation
readback missing after send
```

Work:

```text
1. Use fake/adapted tests for all cases first.
2. Run manual negative drills only where safe and tiny.
3. Persist failure receipts.
4. Confirm failure receipts do not claim success/finality/settlement.
5. Confirm system returns to safe state after each drill.
6. Confirm clean operation still works after negative drill matrix.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md
crates/rox-anchor-coordinator/tests/actual_testnet_negative_drills.rs
crates/rox-anchor-relayer/tests/actual_testnet_negative_drills.rs
crates/rox-anchor-rpc-proof/tests/actual_testnet_negative_drills.rs
crates/rox-anchor-cli/tests/actual_testnet_negative_drill_reports.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-coordinator -p rox-anchor-relayer -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-coordinator --test actual_testnet_negative_drills
cargo test -p rox-anchor-relayer --test actual_testnet_negative_drills
cargo test -p rox-anchor-rpc-proof --test actual_testnet_negative_drills
cargo test -p rox-anchor-cli --test actual_testnet_negative_drill_reports
cargo test --workspace
```

Exit condition:

```text
Actual private testnet negative drills fail safely and produce inspectable failure receipts.
```

---

## Phase 11 — Actual Halt, Recovery, and Authority Drills

Purpose:

Practice operator safety procedures against deployed private testnet state.

Work:

```text
1. Run halt before simulation.
2. Run halt after simulation before send.
3. Run halt after capped send before readback.
4. Run recovery after halt.
5. Run wrong-authority halt attempt.
6. Run wrong-authority recovery attempt.
7. Run key rotation intent drill.
8. Run upgrade authority checklist.
9. Produce redacted operator reports.
10. Confirm no private keys or raw authority paths leak.
11. Confirm status output reflects halt/recovery state.
12. Confirm clean flow resumes only after valid recovery.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md
docs/pilot/ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md
crates/rox-anchor-core/tests/actual_private_testnet_authority_drills.rs
crates/rox-anchor-cli/tests/actual_private_testnet_drill_reports.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
cargo test -p rox-anchor-core --test actual_private_testnet_authority_drills
cargo test -p rox-anchor-cli --test actual_private_testnet_drill_reports
cargo test --workspace
anchor build
anchor test
```

Exit condition:

```text
Authority, halt, and recovery procedures work against deployed private testnet state and remain redacted/operator-readable.
```

---

## Phase 12 — RustyOnions Dry-Run Handoff Evidence

Purpose:

Connect actual private testnet receipts to RustyOnions dry-run intent/status shapes without mutating real ROC.

Work:

```text
1. Map shadow ROC burn-intent into RustyOnions-shaped dry-run input.
2. Map ROX burn/readback evidence into RustyOnions-shaped dry-run release-intent output.
3. Do not call svc-wallet.
4. Do not mutate ron-ledger.
5. Do not unlock paid content.
6. Do not update real balances.
7. Preserve operation ID and idempotency key.
8. Preserve receipt linkage.
9. Redact Solana signatures/paths where needed.
10. Add tests proving no direct wallet/ledger call path exists in ROX Anchor.
11. Add tests proving any future real ROC path must go through svc-wallet/ron-ledger.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md
crates/rox-anchor-core/tests/actual_rustyonions_dry_run_handoff.rs
crates/rox-anchor-coordinator/tests/actual_rustyonions_dry_run_handoff.rs
crates/rox-anchor-cli/tests/actual_rustyonions_dry_run_status.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-core --test actual_rustyonions_dry_run_handoff
cargo test -p rox-anchor-coordinator --test actual_rustyonions_dry_run_handoff
cargo test -p rox-anchor-cli --test actual_rustyonions_dry_run_status
cargo test --workspace
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

Exit condition:

```text
Actual private testnet evidence can be represented as RustyOnions dry-run handoff status without real ROC mutation.
```

---

## Phase 13 — CrabLink Display-Only Private Testnet Status

Purpose:

Prepare CrabLink-facing display/status shapes without giving the client authority.

Work:

```text
1. Define display-only status payload for private testnet pilot evidence.
2. Include proof status.
3. Include read-only RPC status.
4. Include receipt status.
5. Include halt/recovery status.
6. Include dry-run internal ROC status.
7. Clearly label test-only assets.
8. Clearly label no real ROC mutation.
9. Clearly label no final settlement.
10. Do not add Solana submit commands to CrabLink.
11. Do not add ROX mint/burn authority to CrabLink.
12. Do not unlock paid content from private testnet status.
13. Add scanner rules if needed.
```

Likely files in ROX Anchor:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md
crates/rox-anchor-cli/tests/actual_crablink_private_testnet_status.rs
```

Likely future CrabLink files, if this phase touches CrabLink repo:

```text
docs/tauri/ROX_ANCHOR_PRIVATE_TESTNET_STATUS_BOUNDARY.md
scripts/check-rox-anchor-private-testnet-status-boundary.mjs
apps/crablink-tauri/src/pages/quickchain/ or future rox-anchor status page
```

ROX Anchor commands:

```bash
cargo fmt -p rox-anchor-cli
cargo test -p rox-anchor-cli --test actual_crablink_private_testnet_status
cargo test --workspace
```

CrabLink commands, only if touched:

```bash
npm run check:tauri
npm run build
```

Exit condition:

```text
CrabLink can display private testnet status as backend-derived, display-only evidence without wallet/ledger/bridge authority.
```

---

## Phase 14 — Actual Private Testnet Evidence Package

Purpose:

Create an audit-ready evidence package for the real private testnet/test-only run.

Work:

```text
1. Collect final build artifact manifest.
2. Collect deploy receipt.
3. Collect initialization receipt.
4. Collect read-only evidence receipt.
5. Collect simulation receipts.
6. Collect ROC-to-ROX receipts.
7. Collect ROX-to-ROC receipts.
8. Collect negative drill receipts.
9. Collect halt/recovery drill reports.
10. Collect authority drill reports.
11. Collect dry-run RustyOnions handoff reports.
12. Collect CrabLink display-only status report if produced.
13. Redact all secrets.
14. Validate all links.
15. Validate all receipt IDs.
16. Validate operation IDs and idempotency keys.
17. Validate no production/mainnet/public claims exist.
18. Validate no real ROC mutation is claimed.
19. Produce final evidence index.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md
scripts/check_actual_private_testnet_evidence_package.sh
crates/rox-anchor-cli/tests/actual_private_testnet_evidence_package.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
bash scripts/check_actual_private_testnet_evidence_package.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_evidence_package
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
The actual private testnet/test-only bridge run has an audit-ready, redacted evidence package.
```

---

## Phase 15 — BUILD_PLAN4 Closeout Gate

Purpose:

Decide whether the private testnet / test-only bridge goal is complete.

Work:

```text
1. Confirm all local Rust tests pass.
2. Confirm all Anchor tests pass.
3. Confirm all actual private testnet checks pass.
4. Confirm actual deploy receipt exists if deployment was performed.
5. Confirm actual test-only mint initialization receipt exists if initialization was performed.
6. Confirm live read-only RPC evidence exists.
7. Confirm simulation receipts exist.
8. Confirm capped send receipts exist if capped sends were performed.
9. Confirm readback receipts exist for each capped send.
10. Confirm failure receipts exist for negative drills.
11. Confirm halt/recovery drills were performed or simulated.
12. Confirm authority drills were performed or simulated.
13. Confirm RustyOnions handoff remains dry-run only.
14. Confirm CrabLink status remains display-only.
15. Confirm no key material is tracked.
16. Confirm no mainnet behavior exists.
17. Confirm no public launch behavior exists.
18. Confirm no production settlement behavior exists.
19. Confirm no real internal ROC mutation exists.
20. Confirm no exchange/staking/liquidity behavior exists.
21. Confirm known pilot failures are documented.
22. Confirm BUILD_PLAN5 is separate and explicitly scoped.
```

Likely files:

```text
docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md
scripts/check_actual_private_testnet_closeout.sh
crates/rox-anchor-cli/tests/actual_private_testnet_closeout.rs
```

Commands:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
bash scripts/check_actual_private_testnet_closeout.sh .
cargo test -p rox-anchor-cli --test actual_private_testnet_closeout
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
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
cargo clippy --workspace --all-targets -- -D warnings
```

Exit condition:

```text
ROX Anchor actual private testnet / test-only bridge evidence is complete / green / parked.
A future production bridge plan may begin, but BUILD_PLAN4 does not authorize public/mainnet/production/real ROC behavior.
```

---

## Final Status For This Plan

Successful completion means:

```text
ROX Anchor actual private testnet/test-only bridge goal is complete / green / parked.
Actual private devnet/testnet deployment or deployment attempt is evidenced.
Actual test-only mint/config initialization is evidenced.
Actual live read-only RPC verification is evidenced.
Actual simulation against deployed addresses is evidenced.
Actual capped private testnet transaction receipt exists if sends were performed.
Actual readback verification exists for every sent transaction.
Actual negative drills fail safely.
Actual halt/recovery/authority drills are proven.
Actual receipt ledger is redacted, linked, and replay-resistant.
Actual RustyOnions handoff remains dry-run only.
Actual CrabLink status remains display-only.
```

Successful completion does **not** mean:

```text
mainnet launch
public launch
production bridge settlement
public ROX mint/burn
real internal ROC release
public bridge UI
exchange readiness
staking readiness
liquidity readiness
real user funds
```

Those require BUILD_PLAN5.

---

## Build Order Summary

```text
Phase 0  — Freeze BUILD_PLAN3 green baseline
Phase 1  — External operator workspace actualization
Phase 2  — Actual Anchor build artifact capture
Phase 3  — Actual private devnet/testnet deployment
Phase 4  — Test-only ROX mint and program config initialization
Phase 5  — Live read-only RPC evidence against deployed accounts
Phase 6  — Simulation against actual deployed testnet addresses
Phase 7  — Actual capped testnet ROC-to-ROX flow
Phase 8  — Actual capped testnet ROX-to-ROC flow
Phase 9  — Receipt ledger reconciliation for actual runs
Phase 10 — Actual negative drills against deployed testnet state
Phase 11 — Actual halt, recovery, and authority drills
Phase 12 — RustyOnions dry-run handoff evidence
Phase 13 — CrabLink display-only private testnet status
Phase 14 — Actual private testnet evidence package
Phase 15 — BUILD_PLAN4 closeout gate
```

---

## First Command For The Next Session

Start with:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_pilot_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Then begin Phase 0 by confirming BUILD_PLAN3 is parked and BUILD_PLAN4 is the active private testnet execution plan.
