# ROX Anchor Build Plan 5

## Final Production-Readiness / Controlled ROC ↔ ROX Release Plan

Status:

```text
draft / future / not started
```

Preconditions:

```text
BUILD_PLAN.md is complete / green / parked.
BUILD_PLAN2.md is complete / green / parked.
BUILD_PLAN3.md is complete / green / parked.
BUILD_PLAN4.md is complete / green / parked.
Actual private testnet/test-only bridge evidence is complete.
Actual deploy/init/readback/simulation/capped-send receipts exist where applicable.
External private testnet evidence package is redacted and reviewable.
Internal ROC value loop remains complete / green / parked.
RustyOnions wallet/ledger boundaries are intact.
CrabLink remains display/user intent only.
```

This is the fifth and final planned ROX Anchor build plan.

The purpose is to move from:

```text
actual private testnet / test-only bridge evidence
```

to:

```text
production-readiness and controlled real ROC ↔ ROX bridge release
```

This plan is the first plan that may explicitly prepare real ROC ↔ ROX value movement, but only after all earlier phases in this plan pass.

Until the relevant phase gates pass, the following remain forbidden:

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

This plan does **not** include:

```text
staking
liquidity pools
market making
exchange integrations
custody service business logic
public validator economy
yield products
airdrop farming
public faucet
unbounded public bridge
```

This plan may eventually authorize, after explicit gates:

```text
production bridge DTOs
production bridge threat model
production wallet/ledger ROC burn/lock/release path
production ROX mint/burn account binding
production relayer submit path
mainnet-beta deployment decision
strictly capped real-value canary
allowlisted beta
public-readiness gate
controlled launch
emergency halt/recovery procedures
external audit remediation
```

The final release target is:

```text
ROC burn or lock through svc-wallet/ron-ledger
→ deterministic ROX Anchor proof/coordinator/relayer pipeline
→ Solana ROX mint or finalize action
→ readback verification
→ receipt reconciliation

and

Solana ROX burn/finalize evidence
→ read-only RPC verification
→ deterministic ROX Anchor proof/coordinator/relayer pipeline
→ svc-wallet/ron-ledger ROC release
→ receipt reconciliation
```

---

## 0. Non-Negotiable Authority Model

The authority split is mandatory.

```text
ron-proto:
  DTOs, wire shapes, canonical bridge payload shapes

svc-wallet:
  only allowed RustyOnions economic mutation front-door

ron-ledger:
  durable ROC economic truth, replay, conservation, receipts

ron-accounting:
  snapshots/reports only, not balance truth

svc-rewarder:
  capped payout planning only, not bridge mutation

ron-policy:
  declarative gates only, not mutation truth

svc-storage:
  b3/artifact storage only, not payment truth

svc-index:
  pointers/lookups only, not payment truth

svc-gateway:
  public/backend boundary and enforcement proxy, not ledger mutation

omnigate:
  hydration/access composition, not ledger mutation

CrabLink:
  display, routing, user intent, explicit confirmation UX only

ROX Anchor core/proof:
  shared types, deterministic local proof review, replay/mismatch/blocker decisions

ROX Anchor coordinator/rpc-proof/relayer:
  evidence, decisions, readback, submit gating, receipts

Anchor program:
  on-chain Solana state transitions, ROX mint/burn/finalize rules
```

Hard rule:

```text
ROX Anchor must not directly issue or release real internal ROC.
```

The real internal ROC path must be:

```text
svc-wallet -> ron-ledger
```

CrabLink must never become:

```text
wallet truth
ledger truth
receipt truth
bridge truth
ROX mint authority
Solana submit authority
paid unlock authority from local cache
external finality authority
```

---

## 1. Build Rules For This Plan

Every phase must use focused implementation with tests:

```text
small behavior patch
focused test
compile check
fix first failure
then expand
```

No fake success strings.

No fake finality.

No placeholder-only scaffolding.

No broad docs without checks.

No launch language before launch gates.

No bridge activation by config default.

No real-value path may be default enabled.

Every real-value operation must be:

```text
explicit
operator/user confirmed
capped
idempotent
receipt-backed
readback-verified
replay-protected
halt-aware
challenge-aware
recovery-aware
policy-gated
auditable
```

---

## 2. Cross-Repo Work Order

Use this order unless an actual compile/test failure forces a narrower fix:

```text
1. ron-proto + ron-ledger
2. svc-wallet + ron-accounting
3. ron-policy + svc-rewarder
4. svc-gateway + omnigate
5. svc-index + svc-storage
6. rox-anchor
7. CrabLink Tauri + client adapters
```

Reason:

```text
DTOs and ledger truth first.
Wallet mutation gate second.
Policy/accounting/reward boundaries third.
Backend enforcement fourth.
Storage/index evidence fifth.
Anchor/bridge execution sixth.
Client display last.
```

Do not start final bridge work in CrabLink.

CrabLink is last because it is display/user intent only.

---

## 3. Production Scope Locks

Production mode must be impossible to enter accidentally.

Required production mode controls:

```text
default mode = ProductionDisabled
explicit production config file required
explicit production compile/runtime label required
explicit cluster required
explicit program ID required
explicit ROX mint required
explicit wallet/ledger endpoint required
explicit policy config required
explicit operator approval required for canary phases
explicit rate/amount/operation caps required
mainnet-beta rejected until mainnet phase gate
public UI rejected until public UI phase gate
```

Required halt controls:

```text
global bridge halt
ROC-to-ROX halt
ROX-to-ROC halt
relayer halt
wallet handoff halt
readback halt
challenge halt
emergency config freeze
```

Required recovery controls:

```text
resume after halt requires authorized recovery
pending operation recovery requires explicit operator review
stuck mint requires reconciliation path
stuck burn requires reconciliation path
duplicate receipt requires quarantine
readback mismatch requires fail-safe status
ledger mismatch requires fail-safe status
```

---

## 4. Clippy and Gate Policy

Run focused Clippy at natural checkpoints:

```text
after Phase 4
after Phase 8
after Phase 12
after Phase 16
after Phase 20
```

ROX Anchor focused Clippy:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
```

RustyOnions focused gates will depend on touched crates, but the expected minimum is:

```bash
cargo fmt -p ron-proto -p ron-ledger -p svc-wallet -p ron-accounting -p ron-policy
cargo test -p ron-proto
cargo test -p ron-ledger
cargo test -p svc-wallet
cargo test -p ron-accounting
cargo test -p ron-policy
```

CrabLink gates, only after client work begins:

```bash
npm run check:tauri
npm run build
```

---

## Phase 0 — Freeze BUILD_PLAN4 Evidence Baseline

Purpose:

Confirm private testnet/test-only bridge evidence is complete before any production-readiness work begins.

Work:

```text
1. Confirm BUILD_PLAN4 is complete / green / parked.
2. Confirm actual private testnet evidence package exists.
3. Confirm deploy/init/readback/simulation/send receipts exist where applicable.
4. Confirm no key material is tracked.
5. Confirm no real ROC mutation occurred in BUILD_PLAN4.
6. Confirm RustyOnions dry-run handoff remained dry-run.
7. Confirm CrabLink private testnet status remained display-only.
8. Confirm all ROX Anchor tests/checks still pass.
9. Confirm Anchor build/test still passes.
10. Regenerate safe codebundle.
```

Commands:

```bash
cargo fmt --all
bash scripts/check_actual_private_testnet_closeout.sh .
bash scripts/check_private_pilot_hygiene.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Exit condition:

```text
The private testnet/test-only evidence baseline is frozen and safe before production-readiness begins.
```

---

## Phase 1 — Final Bridge Threat Model and Scope Lock

Purpose:

Create the formal final bridge threat model and prohibit every accidental production shortcut.

Work:

```text
1. Define real ROC-to-ROX threats.
2. Define real ROX-to-ROC threats.
3. Define replay threats across both ledgers.
4. Define idempotency failure modes.
5. Define stuck operation states.
6. Define readback mismatch states.
7. Define relayer equivocation threats.
8. Define RPC censorship/outage threats.
9. Define wallet/ledger mismatch threats.
10. Define Solana reorg/finality-label risks.
11. Define upgrade authority risks.
12. Define mint authority risks.
13. Define halt/recovery risks.
14. Define CrabLink display/finality wording risks.
15. Define public UI risks.
16. Define abuse/spam/rate-limit risks.
17. Define operator/key compromise risks.
18. Define emergency rollback posture.
19. Add checker that rejects launch/production wording outside allowed docs.
20. Add tests that the threat model is referenced by final release gate.
```

Likely files:

```text
docs/final/ROX_ANCHOR_FINAL_BRIDGE_THREAT_MODEL.md
docs/final/ROX_ANCHOR_PRODUCTION_SCOPE_LOCK.md
scripts/check_final_bridge_scope_lock.sh
crates/rox-anchor-cli/tests/final_bridge_scope_lock.rs
```

Commands:

```bash
cargo fmt -p rox-anchor-cli
bash scripts/check_final_bridge_scope_lock.sh .
cargo test -p rox-anchor-cli --test final_bridge_scope_lock
cargo test --workspace
```

Exit condition:

```text
The final bridge threat model exists, is checkable, and prevents accidental production/mainnet/public wording drift.
```

---

## Phase 2 — Production Bridge DTOs in ron-proto

Purpose:

Define strict RustyOnions-side bridge DTOs before ledger or wallet mutation exists.

Work:

```text
1. Define ROC-to-ROX bridge quote DTO.
2. Define ROC-to-ROX burn/lock intent DTO.
3. Define ROC-to-ROX accepted wallet receipt DTO.
4. Define ROX mint evidence DTO.
5. Define ROX-to-ROC burn evidence DTO.
6. Define ROX-to-ROC release quote DTO.
7. Define ROX-to-ROC release intent DTO.
8. Define bridge receipt reconciliation DTO.
9. Define bridge status DTO.
10. Define bridge error taxonomy.
11. Require integer minor-unit strings.
12. Require operation ID.
13. Require idempotency key.
14. Require nonce.
15. Require direction.
16. Require cluster/program/mint/token-account bindings.
17. Reject unknown fields.
18. Reject floats.
19. Add canonical byte tests for critical DTOs.
20. Add replay/mismatch DTO tests.
```

Likely RustyOnions files:

```text
crates/ron-proto/src/quickchain/anchor.rs
crates/ron-proto/src/quickchain/operation.rs
crates/ron-proto/src/quickchain/receipt.rs
crates/ron-proto/src/quickchain/mod.rs
crates/ron-proto/tests/rox_anchor_bridge_dto.rs
crates/ron-proto/tests/rox_anchor_bridge_locked_bytes.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p ron-proto
cargo test -p ron-proto --test rox_anchor_bridge_dto
cargo test -p ron-proto --test rox_anchor_bridge_locked_bytes
cargo test -p ron-proto
cargo clippy -p ron-proto --all-targets -- -D warnings
```

Exit condition:

```text
Bridge DTOs are strict, canonical, replay-aware, and inert.
They do not mutate balances.
```

---

## Phase 3 — ron-ledger Bridge Truth Model

Purpose:

Add durable ROC ledger semantics for bridge burn/lock/release without involving Solana execution yet.

Work:

```text
1. Add bridge burn/lock operation class if needed.
2. Add bridge release operation class if needed.
3. Preserve existing issue/transfer/burn/hold semantics.
4. Enforce balance conservation.
5. Enforce idempotency.
6. Enforce operation ID uniqueness.
7. Enforce replay stability.
8. Enforce no double release.
9. Enforce no double burn/lock.
10. Add bridge pending state if needed.
11. Add bridge failed/quarantined state if needed.
12. Add stuck operation recovery shape.
13. Add receipt references for ROX Anchor operation ID.
14. Add tests for duplicate replay.
15. Add tests for failed readback reconciliation.
16. Add tests for ledger replay equality.
17. Add tests for conservation.
```

Likely RustyOnions files:

```text
crates/ron-ledger/src/quickchain/anchor_dry_run.rs
crates/ron-ledger/src/quickchain/transition.rs
crates/ron-ledger/src/quickchain/execution_state.rs
crates/ron-ledger/src/quickchain/types.rs
crates/ron-ledger/tests/rox_anchor_bridge_ledger_truth.rs
crates/ron-ledger/tests/rox_anchor_bridge_replay.rs
crates/ron-ledger/tests/rox_anchor_bridge_conservation.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p ron-ledger
cargo test -p ron-ledger --test rox_anchor_bridge_ledger_truth
cargo test -p ron-ledger --test rox_anchor_bridge_replay
cargo test -p ron-ledger --test rox_anchor_bridge_conservation
cargo test -p ron-ledger
cargo clippy -p ron-ledger --all-targets -- -D warnings
```

Exit condition:

```text
ron-ledger can represent and replay bridge-related ROC truth without Solana submission or ROX mint/burn authority.
```

---

## Phase 4 — svc-wallet Bridge Front-Door

Purpose:

Make svc-wallet the only RustyOnions service that can request real ROC bridge mutations.

Work:

```text
1. Add bridge quote endpoint/model.
2. Add explicit confirmation requirement.
3. Add ROC-to-ROX burn/lock request path.
4. Add ROX-to-ROC release request path.
5. Require policy gate.
6. Require operation ID.
7. Require idempotency key.
8. Require nonce.
9. Require amount cap.
10. Require per-account/day cap.
11. Require global halt check.
12. Require direction-specific halt check.
13. Return durable ledger receipt.
14. Reject duplicate changed idempotency payload.
15. Reject direct release without ROX burn evidence status.
16. Add tests proving wallet is the only mutation front-door.
17. Add tests proving gateway/omnigate/client cannot mutate ledger directly.
```

Likely RustyOnions files:

```text
crates/svc-wallet/src/
crates/svc-wallet/tests/rox_anchor_bridge_wallet_frontdoor.rs
crates/svc-wallet/tests/rox_anchor_bridge_idempotency.rs
crates/svc-wallet/tests/rox_anchor_bridge_halt_policy.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p svc-wallet -p ron-ledger -p ron-proto
cargo test -p svc-wallet --test rox_anchor_bridge_wallet_frontdoor
cargo test -p svc-wallet --test rox_anchor_bridge_idempotency
cargo test -p svc-wallet --test rox_anchor_bridge_halt_policy
cargo test -p svc-wallet
cargo test -p ron-ledger
cargo clippy -p svc-wallet --all-targets -- -D warnings
```

Clippy checkpoint:

```bash
cargo clippy -p ron-proto --all-targets -- -D warnings
cargo clippy -p ron-ledger --all-targets -- -D warnings
cargo clippy -p svc-wallet --all-targets -- -D warnings
```

Exit condition:

```text
svc-wallet is the only approved ROC bridge mutation front-door and produces durable ron-ledger truth.
```

---

## Phase 5 — Policy and Economics Gates

Purpose:

Define bridge limits, fees, challenge windows, and halt controls without hard-coded economics.

Work:

```text
1. Add bridge economics section to configs/roc-economics.toml.
2. Keep bridge disabled by default until production gate.
3. Add per-direction fees if any.
4. Add per-user limits.
5. Add per-run limits.
6. Add daily global limits.
7. Add minimum/maximum bridge amount.
8. Add challenge window settings.
9. Add emergency halt settings.
10. Add remainder sink rules.
11. Add policy validation.
12. Add no-float tests.
13. Add basis-point tests.
14. Add disabled-by-default tests.
15. Add config drift tests.
16. Add tests proving config cannot directly create ledger receipts.
```

Likely RustyOnions files:

```text
configs/roc-economics.toml
crates/ron-policy/src/economics/internal_roc.rs
crates/ron-policy/tests/rox_anchor_bridge_policy.rs
crates/ron-policy/tests/rox_anchor_bridge_economics_config.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p ron-policy
cargo test -p ron-policy --test rox_anchor_bridge_policy
cargo test -p ron-policy --test rox_anchor_bridge_economics_config
cargo test -p ron-policy
cargo clippy -p ron-policy --all-targets -- -D warnings
```

Exit condition:

```text
Bridge economics and policy gates are config-driven, disabled by default, integer-safe, and non-authoritative until wallet/ledger execution.
```

---

## Phase 6 — ROX Anchor Production Config and Authority Model

Purpose:

Harden ROX Anchor for production configuration without deploying mainnet yet.

Work:

```text
1. Add production config shape.
2. Require explicit production mode.
3. Require selected cluster.
4. Reject mainnet-beta until mainnet phase gate.
5. Require program ID.
6. Require ROX mint.
7. Require token program.
8. Require mint authority model.
9. Require upgrade authority policy.
10. Require halt authority.
11. Require recovery authority.
12. Require operator/key separation.
13. Require KMS/hardware/external signer abstraction where possible.
14. Redact all key paths.
15. Reject shared critical authority unless explicit internal canary mode.
16. Add tests for production config validation.
17. Add tests for redaction.
18. Add tests for unsafe authority sharing.
```

Likely ROX Anchor files:

```text
crates/rox-anchor-core/src/types.rs
crates/rox-anchor-core/tests/production_config.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-relayer/tests/production_config.rs
crates/rox-anchor-cli/tests/production_status.rs
docs/final/PRODUCTION_CONFIG_MODEL.md
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-core -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-core --test production_config
cargo test -p rox-anchor-relayer --test production_config
cargo test -p rox-anchor-cli --test production_status
cargo test --workspace
```

Exit condition:

```text
ROX Anchor production config is explicit, redacted, authority-separated, and impossible to trigger by default.
```

---

## Phase 7 — Anchor Program Production Account Review

Purpose:

Review on-chain account sizing, PDA seeds, state transitions, and upgrade posture before any mainnet decision.

Work:

```text
1. Review all account sizes.
2. Review rent/space requirements.
3. Review PDA seed stability.
4. Review bump handling.
5. Review operation account lifecycle.
6. Review challenge account lifecycle.
7. Review config account lifecycle.
8. Review mint authority PDA.
9. Review token account constraints.
10. Review finalize logic.
11. Review halt/recovery logic.
12. Review duplicate finalization rejection.
13. Review event shape.
14. Add account sizing tests.
15. Add PDA regression tests.
16. Add instruction constraint tests.
17. Add external audit notes.
```

Likely ROX Anchor files:

```text
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/*.rs
programs/rox-anchor/src/events.rs
programs/rox-anchor/src/errors.rs
programs/rox-anchor/tests or crate tests
docs/final/ANCHOR_ACCOUNT_AND_PDA_REVIEW.md
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor
cargo test -p rox-anchor
cargo check -p rox-anchor
anchor build
anchor test
```

Exit condition:

```text
Anchor account/PDA/state rules are reviewed, tested, and audit-ready before mainnet deployment is considered.
```

---

## Phase 8 — Production Relayer and Submit Path

Purpose:

Create the production-capable relayer path, still disabled by default and not mainnet-enabled until later gates.

Work:

```text
1. Separate testnet capped submit from production submit.
2. Keep production submit disabled by default.
3. Require explicit production mode.
4. Require signer abstraction.
5. Require policy approval.
6. Require wallet/ledger receipt for ROC-to-ROX.
7. Require Solana burn/readback proof for ROX-to-ROC.
8. Require accepted proof review.
9. Require accepted coordinator decision.
10. Require successful simulation.
11. Require read-only RPC preflight.
12. Require transaction send cap.
13. Require retry cap.
14. Require amount cap.
15. Require receipt persistence.
16. Require readback verification.
17. Require reconciliation before success.
18. Add tests proving missing gates block production submit.
19. Add tests proving default mode cannot submit.
20. Add tests proving mainnet remains rejected until phase gate.
```

Likely ROX Anchor files:

```text
crates/rox-anchor-relayer/src/submit.rs
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-relayer/src/config.rs
crates/rox-anchor-relayer/tests/production_submit_gate.rs
crates/rox-anchor-cli/tests/production_submit_command.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test production_submit_gate
cargo test -p rox-anchor-cli --test production_submit_command
cargo test --workspace
```

Clippy checkpoint:

```bash
cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
```

Exit condition:

```text
Production submit path exists as a gated model but remains disabled until mainnet/canary phases explicitly authorize it.
```

---

## Phase 9 — Bidirectional Reconciliation Engine

Purpose:

Ensure every real bridge movement reconciles across RustyOnions and Solana evidence.

Work:

```text
1. Define ROC-to-ROX reconciliation record.
2. Link svc-wallet receipt.
3. Link ron-ledger receipt.
4. Link ROX Anchor operation ID.
5. Link Solana transaction signature.
6. Link readback evidence.
7. Link final Anchor event.
8. Define ROX-to-ROC reconciliation record.
9. Link Solana burn evidence.
10. Link readback proof.
11. Link svc-wallet release receipt.
12. Link ron-ledger receipt.
13. Reject partial success as final success.
14. Quarantine mismatched receipts.
15. Quarantine stuck operations.
16. Add deterministic report tests.
17. Add replay tests.
18. Add duplicate reconciliation tests.
```

Likely files:

```text
crates/rox-anchor-relayer/src/receipts.rs
crates/rox-anchor-coordinator/src/decision.rs
crates/rox-anchor-cli/src/commands/receipts.rs
crates/rox-anchor-relayer/tests/production_reconciliation.rs
crates/rox-anchor-coordinator/tests/production_reconciliation.rs
docs/final/PRODUCTION_RECONCILIATION.md
```

RustyOnions companion files:

```text
crates/ron-ledger/tests/rox_anchor_bridge_reconciliation.rs
crates/svc-wallet/tests/rox_anchor_bridge_reconciliation.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test production_reconciliation
cargo test -p rox-anchor-coordinator --test production_reconciliation
cargo test --workspace
```

RustyOnions commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p ron-ledger -p svc-wallet
cargo test -p ron-ledger --test rox_anchor_bridge_reconciliation
cargo test -p svc-wallet --test rox_anchor_bridge_reconciliation
```

Exit condition:

```text
No bridge operation can be called successful until RustyOnions receipt truth and Solana readback evidence reconcile.
```

---

## Phase 10 — Readback Finality Labels and Confirmation Policy

Purpose:

Avoid fake finality while defining exactly what confirmations are enough for each release stage.

Work:

```text
1. Define Solana commitment labels used by ROX Anchor.
2. Define minimum confirmation/readback policy for canary.
3. Define minimum confirmation/readback policy for allowlisted beta.
4. Define minimum confirmation/readback policy for public release if authorized.
5. Define stale readback rejection.
6. Define conflicting readback rejection.
7. Define provider disagreement rejection.
8. Define reorg-like incident handling.
9. Define “accepted but not final” display label.
10. Define “readback verified” label.
11. Define “reconciled” label.
12. Define “quarantined” label.
13. Add tests that labels cannot overclaim finality.
```

Likely files:

```text
crates/rox-anchor-rpc-proof/src/commitment.rs
crates/rox-anchor-core/src/labels.rs
crates/rox-anchor-rpc-proof/tests/production_finality_labels.rs
crates/rox-anchor-cli/tests/production_status_labels.rs
docs/final/FINALITY_AND_READBACK_POLICY.md
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-core -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof --test production_finality_labels
cargo test -p rox-anchor-cli --test production_status_labels
cargo test -p rox-anchor-core
cargo test --workspace
```

Exit condition:

```text
ROX Anchor uses explicit readback/finality labels and cannot claim final settlement before reconciliation.
```

---

## Phase 11 — Emergency Halt, Recovery, and Rollback Runbooks

Purpose:

Prepare for real-value safety before any canary.

Work:

```text
1. Define global halt process.
2. Define direction-specific halt process.
3. Define relayer halt process.
4. Define wallet handoff halt process.
5. Define readback mismatch halt process.
6. Define stuck ROC-to-ROX recovery.
7. Define stuck ROX-to-ROC recovery.
8. Define duplicate operation quarantine.
9. Define malicious or mistaken operator recovery.
10. Define upgrade rollback if possible.
11. Define emergency communication labels.
12. Define operator checklist.
13. Add CLI drill reports.
14. Add tests proving halt blocks every stage.
15. Add tests proving recovery requires explicit authority.
```

Likely files:

```text
docs/final/EMERGENCY_HALT_RECOVERY_AND_ROLLBACK.md
scripts/check_final_halt_recovery_runbook.sh
crates/rox-anchor-cli/tests/final_halt_recovery_runbook.rs
crates/rox-anchor-coordinator/tests/final_halt_recovery_gate.rs
crates/rox-anchor-relayer/tests/final_halt_recovery_submit_gate.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli -p rox-anchor-coordinator -p rox-anchor-relayer
bash scripts/check_final_halt_recovery_runbook.sh .
cargo test -p rox-anchor-cli --test final_halt_recovery_runbook
cargo test -p rox-anchor-coordinator --test final_halt_recovery_gate
cargo test -p rox-anchor-relayer --test final_halt_recovery_submit_gate
cargo test --workspace
```

Exit condition:

```text
Emergency halt/recovery/rollback procedures exist and are enforced by tests before real-value canary.
```

---

## Phase 12 — External Audit Package and Remediation Loop

Purpose:

Prepare and freeze an audit package before real-value canary.

Work:

```text
1. Build invariant-to-test map.
2. Build authority map.
3. Build wallet/ledger handoff map.
4. Build Anchor account/PDA map.
5. Build relayer submit boundary map.
6. Build RPC/readback boundary map.
7. Build receipt/reconciliation map.
8. Build halt/recovery map.
9. Build known non-goals list.
10. Build threat model index.
11. Build manual testnet evidence index.
12. Record all known findings.
13. Record mitigations.
14. Fix critical/high issues before proceeding.
15. Add regression tests for every fixed issue.
16. Require explicit audit signoff or self-review closeout.
```

Likely files:

```text
docs/final/AUDIT_PACKAGE_INDEX.md
docs/final/FINAL_INVARIANT_TEST_MAP.md
docs/final/FINAL_AUTHORITY_MAP.md
docs/final/FINAL_KNOWN_NON_GOALS.md
docs/final/AUDIT_FINDINGS_AND_REMEDIATION.md
scripts/check_final_audit_package.sh
crates/rox-anchor-cli/tests/final_audit_package.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli
bash scripts/check_final_audit_package.sh .
cargo test -p rox-anchor-cli --test final_audit_package
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Clippy checkpoint:

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
The final bridge audit package is complete, findings are remediated or explicitly blocked, and critical/high issues are closed before canary.
```

---

## Phase 13 — Mainnet Deployment Decision Gate

Purpose:

Decide whether mainnet-beta deployment may be performed.

Work:

```text
1. Confirm external audit package is complete.
2. Confirm private testnet evidence package is complete.
3. Confirm production config is explicit.
4. Confirm mainnet-beta is still rejected by default.
5. Confirm mainnet deployment requires explicit one-time gate.
6. Confirm program ID policy.
7. Confirm upgrade authority policy.
8. Confirm mint authority policy.
9. Confirm halt/recovery authorities.
10. Confirm signer/key policy.
11. Confirm no secrets are tracked.
12. Confirm no public UI exists yet unless explicitly approved.
13. Confirm no real ROC handoff enabled yet.
14. Confirm canary caps are tiny.
15. Confirm rollback/halt procedures are rehearsed.
16. Add checker for deployment decision.
```

Likely files:

```text
docs/final/MAINNET_DEPLOYMENT_DECISION_GATE.md
scripts/check_mainnet_deployment_decision_gate.sh
crates/rox-anchor-cli/tests/mainnet_deployment_decision_gate.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli
bash scripts/check_mainnet_deployment_decision_gate.sh .
cargo test -p rox-anchor-cli --test mainnet_deployment_decision_gate
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Manual command, only after gate passes and operator explicitly approves:

```bash
# anchor deploy \
#   --provider.cluster mainnet-beta \
#   --provider.wallet /external/mainnet/path/to/payer.json
```

Exit condition:

```text
Mainnet deployment is either explicitly approved and evidenced, or blocked with documented reasons.
No production bridge traffic is authorized by deployment alone.
```

---

## Phase 14 — Mainnet Program Deployment and Read-Only Verification

Purpose:

Deploy and verify the production program on mainnet, without enabling real bridge traffic yet.

Work:

```text
1. Deploy program if Phase 13 approved.
2. Capture deploy signature.
3. Capture deploy slot.
4. Capture program ID.
5. Capture IDL hash.
6. Capture program binary hash.
7. Capture upgrade authority policy.
8. Verify deployed program by read-only RPC.
9. Verify config account if initialized.
10. Verify mint authority setup if initialized.
11. Persist redacted deployment receipt.
12. Confirm no bridge traffic is enabled.
13. Confirm no public UI is enabled.
14. Confirm no real ROC handoff is enabled.
```

Likely files:

```text
docs/final/MAINNET_DEPLOYMENT_RECEIPT.md
scripts/check_mainnet_deployment_receipt.sh
crates/rox-anchor-cli/tests/mainnet_deployment_receipt.rs
crates/rox-anchor-rpc-proof/tests/mainnet_read_only_verification.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli -p rox-anchor-rpc-proof
bash scripts/check_mainnet_deployment_receipt.sh .
cargo test -p rox-anchor-cli --test mainnet_deployment_receipt
cargo test -p rox-anchor-rpc-proof --test mainnet_read_only_verification
cargo test --workspace
```

Exit condition:

```text
Mainnet program deployment, if performed, is read-only verified and does not yet authorize real bridge traffic.
```

---

## Phase 15 — Internal-Owned Real-Value Canary Gate

Purpose:

Decide whether a tiny, internally owned, real-value canary may run.

Work:

```text
1. Confirm mainnet deployment/read-only verification.
2. Confirm wallet/ledger bridge path is green.
3. Confirm svc-wallet bridge front-door is green.
4. Confirm ron-ledger bridge replay/conservation is green.
5. Confirm production relayer gate is green.
6. Confirm reconciliation is green.
7. Confirm halt/recovery is green.
8. Confirm policy limits are green.
9. Confirm canary amount is tiny.
10. Confirm canary accounts are internal/operator-owned.
11. Confirm no public users are involved.
12. Confirm no public UI is involved.
13. Confirm no exchange/staking/liquidity behavior exists.
14. Require explicit operator approval phrase.
15. Add canary checker.
```

Likely files:

```text
docs/final/INTERNAL_REAL_VALUE_CANARY_GATE.md
scripts/check_internal_real_value_canary_gate.sh
crates/rox-anchor-cli/tests/internal_real_value_canary_gate.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli
bash scripts/check_internal_real_value_canary_gate.sh .
cargo test -p rox-anchor-cli --test internal_real_value_canary_gate
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Exit condition:

```text
A tiny internal-owned real-value canary is either explicitly approved or blocked.
Public users remain unauthorized.
```

---

## Phase 16 — Internal ROC-to-ROX Real-Value Canary

Purpose:

Run the smallest possible internal-owned ROC-to-ROX real-value canary.

Work:

```text
1. Prepare ROC-to-ROX quote through svc-wallet.
2. Require explicit operator confirmation.
3. Burn/lock tiny ROC amount through svc-wallet.
4. Record ron-ledger receipt.
5. Feed receipt evidence into ROX Anchor proof path.
6. Run read-only RPC preflight.
7. Run coordinator decision.
8. Run relayer dry-run.
9. Run simulation.
10. Submit capped mainnet transaction if all gates pass.
11. Verify readback by RPC.
12. Reconcile ron-ledger receipt with Solana transaction.
13. Persist final canary receipt.
14. Quarantine if readback mismatch.
15. Halt if reconciliation fails.
16. Add regression tests for every incident.
```

Likely files:

```text
docs/final/INTERNAL_ROC_TO_ROX_CANARY_RUN.md
crates/rox-anchor-relayer/tests/internal_roc_to_rox_canary.rs
crates/rox-anchor-coordinator/tests/internal_roc_to_rox_canary.rs
crates/rox-anchor-cli/tests/internal_roc_to_rox_canary_command.rs
```

RustyOnions companion tests:

```text
crates/svc-wallet/tests/internal_roc_to_rox_canary.rs
crates/ron-ledger/tests/internal_roc_to_rox_canary.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-relayer -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test internal_roc_to_rox_canary
cargo test -p rox-anchor-coordinator --test internal_roc_to_rox_canary
cargo test -p rox-anchor-cli --test internal_roc_to_rox_canary_command
cargo test --workspace
```

RustyOnions commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p svc-wallet -p ron-ledger
cargo test -p svc-wallet --test internal_roc_to_rox_canary
cargo test -p ron-ledger --test internal_roc_to_rox_canary
```

Exit condition:

```text
A tiny internal-owned ROC-to-ROX real-value canary either succeeds with reconciled receipts or fails safely with halt/quarantine.
```

---

## Phase 17 — Internal ROX-to-ROC Real-Value Canary

Purpose:

Run the smallest possible internal-owned ROX-to-ROC real-value canary.

Work:

```text
1. Burn tiny ROX amount on Solana through controlled canary path.
2. Verify burn/readback by RPC.
3. Feed burn evidence into proof path.
4. Run coordinator decision.
5. Run relayer dry-run.
6. Prepare ROC release through svc-wallet.
7. Require policy gate.
8. Require explicit operator confirmation.
9. Release tiny ROC amount through svc-wallet.
10. Record ron-ledger receipt.
11. Reconcile Solana burn with ron-ledger release.
12. Persist final canary receipt.
13. Quarantine if mismatch.
14. Halt if reconciliation fails.
15. Add regression tests for every incident.
```

Likely files:

```text
docs/final/INTERNAL_ROX_TO_ROC_CANARY_RUN.md
crates/rox-anchor-rpc-proof/tests/internal_rox_to_roc_canary_readback.rs
crates/rox-anchor-coordinator/tests/internal_rox_to_roc_canary.rs
crates/rox-anchor-cli/tests/internal_rox_to_roc_canary_command.rs
```

RustyOnions companion tests:

```text
crates/svc-wallet/tests/internal_rox_to_roc_canary.rs
crates/ron-ledger/tests/internal_rox_to_roc_canary.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-coordinator -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof --test internal_rox_to_roc_canary_readback
cargo test -p rox-anchor-coordinator --test internal_rox_to_roc_canary
cargo test -p rox-anchor-cli --test internal_rox_to_roc_canary_command
cargo test --workspace
```

RustyOnions commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p svc-wallet -p ron-ledger
cargo test -p svc-wallet --test internal_rox_to_roc_canary
cargo test -p ron-ledger --test internal_rox_to_roc_canary
```

Clippy checkpoint:

```bash
cd /Users/mymac/Desktop/rox-anchor

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
A tiny internal-owned ROX-to-ROC real-value canary either succeeds with reconciled receipts or fails safely with halt/quarantine.
```

---

## Phase 18 — Backend API and Gateway Enforcement

Purpose:

Expose production bridge intent/status through backend services without letting gateway/omnigate mutate ledger truth.

Work:

```text
1. Add bridge quote route through svc-gateway/omnigate if needed.
2. Add bridge status route.
3. Add bridge receipt lookup route.
4. Add bridge operation history route.
5. Route mutation requests only to svc-wallet.
6. Route receipt truth only from wallet/ledger-derived backend data.
7. Add access controls.
8. Add rate limits.
9. Add abuse limits.
10. Add replay-safe idempotency.
11. Add tests proving gateway/omnigate do not mutate ledger directly.
12. Add tests proving backend refuses cache-only or index-only bridge proof.
```

RustyOnions likely files:

```text
crates/svc-gateway/tests/rox_anchor_bridge_routes.rs
crates/omnigate/tests/rox_anchor_bridge_hydration_boundary.rs
crates/svc-index/tests/rox_anchor_bridge_pointer_non_authority.rs
crates/svc-storage/tests/rox_anchor_bridge_artifact_non_authority.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p svc-gateway -p omnigate -p svc-index -p svc-storage
cargo test -p svc-gateway --test rox_anchor_bridge_routes
cargo test -p omnigate --test rox_anchor_bridge_hydration_boundary
cargo test -p svc-index --test rox_anchor_bridge_pointer_non_authority
cargo test -p svc-storage --test rox_anchor_bridge_artifact_non_authority
```

Exit condition:

```text
Backend bridge APIs expose quote/status/receipt surfaces while preserving svc-wallet/ron-ledger as the only economic mutation path.
```

---

## Phase 19 — CrabLink Controlled Bridge UX

Purpose:

Add CrabLink UX only after backend and canary gates prove real receipt truth.

Work:

```text
1. Add read-only bridge status page.
2. Add bridge quote display.
3. Add explicit confirmation UX.
4. Add pending operation display.
5. Add receipt display.
6. Add reconciled status display.
7. Add failed/quarantined status display.
8. Add halt display.
9. Add recovery-required display.
10. Add backend-derived balance refresh.
11. Prevent local cache from unlocking bridge status.
12. Prevent local cache from fabricating receipts.
13. Prevent client-side Solana submit.
14. Prevent client-side ROX mint/burn authority.
15. Prevent client-side ROC release.
16. Add boundary scanner.
17. Add UI tests.
```

CrabLink likely files:

```text
docs/tauri/ROX_ANCHOR_BRIDGE_CLIENT_BOUNDARY.md
scripts/check-rox-anchor-bridge-client-boundary.mjs
apps/crablink-tauri/src/pages/bridge/
apps/crablink-tauri/src/shared/api/bridgeClient.js
apps/crablink-tauri/src/shared/receipts/
```

Commands:

```bash
cd /Users/mymac/Desktop/crablink

npm run check:tauri
node scripts/check-rox-anchor-bridge-client-boundary.mjs
npm run build
```

Exit condition:

```text
CrabLink can display and confirm bridge operations using backend truth only, without becoming bridge, wallet, ledger, Solana, ROX, or paid-unlock authority.
```

---

## Phase 20 — Allowlisted Beta Gate

Purpose:

Decide whether a small allowlisted beta may begin after internal-owned canaries and CrabLink/backend UX pass.

Work:

```text
1. Confirm both internal-owned canary directions reconciled.
2. Confirm no unresolved critical/high audit findings.
3. Confirm emergency halt works.
4. Confirm stuck-operation recovery works.
5. Confirm monitoring exists.
6. Confirm receipt reconciliation dashboard/report exists.
7. Confirm support/runbook exists.
8. Confirm user-facing wording avoids finality overclaims.
9. Confirm caps are tiny.
10. Confirm allowlist is explicit.
11. Confirm no public bridge UI for non-allowlisted users.
12. Confirm no exchange/staking/liquidity behavior exists.
13. Confirm no public faucet exists.
14. Add allowlisted beta checker.
```

Likely files:

```text
docs/final/ALLOWLISTED_BETA_GATE.md
scripts/check_allowlisted_beta_gate.sh
crates/rox-anchor-cli/tests/allowlisted_beta_gate.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt -p rox-anchor-cli
bash scripts/check_allowlisted_beta_gate.sh .
cargo test -p rox-anchor-cli --test allowlisted_beta_gate
cargo test --workspace
cargo check --workspace
anchor build
anchor test
```

Final checkpoint:

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
Allowlisted beta is either explicitly approved with tiny caps or blocked with documented reasons.
```

---

## Phase 21 — Final Public-Readiness Gate

Purpose:

Decide whether ROX Anchor v1 is ready for controlled public release.

Work:

```text
1. Confirm allowlisted beta ran or was explicitly deferred.
2. Confirm all critical/high findings closed.
3. Confirm both directions reconcile.
4. Confirm production caps exist.
5. Confirm halt controls exist.
6. Confirm recovery controls exist.
7. Confirm monitoring exists.
8. Confirm support/runbook exists.
9. Confirm legal/disclaimer docs exist.
10. Confirm CrabLink wording is safe.
11. Confirm no exchange/staking/liquidity behavior exists.
12. Confirm no public faucet exists unless separately authorized.
13. Confirm no unbounded bridge exists.
14. Confirm no fake finality wording exists.
15. Confirm no key material is tracked.
16. Confirm production receipts are redacted and replay-resistant.
17. Confirm public docs do not promise investment/yield/returns.
18. Confirm final codebundle is secret-safe.
19. Add public-readiness checker.
```

Likely files:

```text
docs/final/PUBLIC_READINESS_GATE.md
scripts/check_public_readiness_gate.sh
crates/rox-anchor-cli/tests/public_readiness_gate.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_public_readiness_gate.sh .
cargo test -p rox-anchor-cli --test public_readiness_gate
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

RustyOnions gate:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo test -p ron-proto
cargo test -p ron-ledger
cargo test -p svc-wallet
cargo test -p ron-accounting
cargo test -p ron-policy
cargo test -p svc-gateway
cargo test -p omnigate
```

CrabLink gate:

```bash
cd /Users/mymac/Desktop/crablink

npm run check:tauri
npm run build
```

Exit condition:

```text
ROX Anchor v1 is public-release ready only if this gate passes.
If it does not pass, public release remains blocked.
```

---

## Phase 22 — Final Project Closeout Gate

Purpose:

Close the ROX Anchor project as a complete v1 bridge implementation if and only if every required gate passes.

Work:

```text
1. Confirm BUILD_PLAN.md parked.
2. Confirm BUILD_PLAN2.md parked.
3. Confirm BUILD_PLAN3.md parked.
4. Confirm BUILD_PLAN4.md parked.
5. Confirm BUILD_PLAN5 phases complete.
6. Confirm RustyOnions bridge DTOs green.
7. Confirm ron-ledger bridge truth green.
8. Confirm svc-wallet bridge front-door green.
9. Confirm policy/economics gates green.
10. Confirm ROX Anchor production config green.
11. Confirm Anchor account/PDA review green.
12. Confirm relayer production submit gate green.
13. Confirm reconciliation green.
14. Confirm readback/finality labels green.
15. Confirm halt/recovery/rollback green.
16. Confirm audit package green.
17. Confirm mainnet deployment/readback evidence if performed.
18. Confirm internal canary evidence if performed.
19. Confirm allowlisted beta gate if performed.
20. Confirm CrabLink client boundary green if touched.
21. Confirm no unresolved critical/high findings.
22. Confirm no secrets tracked.
23. Confirm no fake finality.
24. Confirm no exchange/staking/liquidity behavior.
25. Confirm final safe codebundles generated.
```

Likely files:

```text
docs/final/ROX_ANCHOR_V1_FINAL_CLOSEOUT.md
scripts/check_rox_anchor_v1_final_closeout.sh
crates/rox-anchor-cli/tests/rox_anchor_v1_final_closeout.rs
```

Commands:

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_rox_anchor_v1_final_closeout.sh .
cargo test -p rox-anchor-cli --test rox_anchor_v1_final_closeout
cargo test --workspace
cargo check --workspace
anchor build
anchor test

cargo clippy -p rox-anchor-core --all-targets -- -D warnings
cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
cargo clippy -p rox-anchor --all-targets -- -D warnings
cargo clippy --workspace --all-targets -- -D warnings

bash scripts/make_codebundle.sh
```

RustyOnions final gate:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo fmt -p ron-proto -p ron-ledger -p svc-wallet -p ron-accounting -p ron-policy -p svc-gateway -p omnigate
cargo test -p ron-proto
cargo test -p ron-ledger
cargo test -p svc-wallet
cargo test -p ron-accounting
cargo test -p ron-policy
cargo test -p svc-gateway
cargo test -p omnigate
```

CrabLink final gate, if bridge UX was touched:

```bash
cd /Users/mymac/Desktop/crablink

npm run check:tauri
npm run build
```

Exit condition:

```text
ROX Anchor v1 is complete / green / parked.

The project has a controlled ROC ↔ ROX bridge implementation with:
  wallet/ledger ROC truth,
  Solana ROX on-chain state,
  deterministic proof review,
  replay rejection,
  mismatch rejection,
  readback verification,
  bidirectional reconciliation,
  receipt trail,
  halt/recovery controls,
  emergency runbooks,
  audit package,
  client display boundary,
  and final release gates.

Staking, liquidity, exchange-facing behavior, market making, and custody services remain out of scope unless a separate future plan authorizes them.
```

---

## Final Status For This Plan

Successful completion means:

```text
ROX Anchor v1 controlled ROC ↔ ROX bridge is complete / green / parked.
Real ROC-to-ROX path is proven through svc-wallet/ron-ledger and Solana ROX readback.
Real ROX-to-ROC path is proven through Solana burn/readback and svc-wallet/ron-ledger release.
Bridge operations are receipt-backed, capped, reconciled, haltable, and auditable.
CrabLink displays/initiates bridge actions without becoming authority.
```

Successful completion does **not** mean:

```text
staking
liquidity pools
exchange integration
market making
custody service
yield product
public validator economy
unbounded public bridge
```

Those remain separate future workstreams.

---

## Build Order Summary

```text
Phase 0  — Freeze BUILD_PLAN4 evidence baseline
Phase 1  — Final bridge threat model and scope lock
Phase 2  — Production bridge DTOs in ron-proto
Phase 3  — ron-ledger bridge truth model
Phase 4  — svc-wallet bridge front-door
Phase 5  — Policy and economics gates
Phase 6  — ROX Anchor production config and authority model
Phase 7  — Anchor program production account review
Phase 8  — Production relayer and submit path
Phase 9  — Bidirectional reconciliation engine
Phase 10 — Readback finality labels and confirmation policy
Phase 11 — Emergency halt, recovery, and rollback runbooks
Phase 12 — External audit package and remediation loop
Phase 13 — Mainnet deployment decision gate
Phase 14 — Mainnet program deployment and read-only verification
Phase 15 — Internal-owned real-value canary gate
Phase 16 — Internal ROC-to-ROX real-value canary
Phase 17 — Internal ROX-to-ROC real-value canary
Phase 18 — Backend API and gateway enforcement
Phase 19 — CrabLink controlled bridge UX
Phase 20 — Allowlisted beta gate
Phase 21 — Final public-readiness gate
Phase 22 — Final project closeout gate
```

---

## First Command For The Future BUILD_PLAN5 Session

Start only after BUILD_PLAN4 is complete / green / parked.

```bash
cd /Users/mymac/Desktop/rox-anchor

cargo fmt --all
bash scripts/check_actual_private_testnet_closeout.sh .
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/make_codebundle.sh
```

Then verify RustyOnions and CrabLink baselines before editing anything:

```bash
cd /Users/mymac/Desktop/RustyOnions

cargo test -p ron-proto
cargo test -p ron-ledger
cargo test -p svc-wallet
cargo test -p ron-accounting
cargo test -p ron-policy
```

```bash
cd /Users/mymac/Desktop/crablink

npm run check:tauri
npm run build
```

If those baselines are green, begin Phase 0.
