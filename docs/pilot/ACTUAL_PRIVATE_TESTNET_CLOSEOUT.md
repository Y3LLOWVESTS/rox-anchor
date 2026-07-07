# ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md

RO:WHAT — BUILD_PLAN4 Phase 15 actual private testnet/test-only bridge closeout gate.
RO:WHY — Decides whether BUILD_PLAN4 can be parked while preserving the boundary that public/mainnet/production/real-ROC behavior remains future BUILD_PLAN5 work.
RO:INTERACTS — scripts/check_actual_private_testnet_closeout.sh, BUILD_PLAN4.md, BUILD_PLAN5.md, prior Phase 1-14 checkers, rox-anchor-cli tests.
RO:INVARIANTS — closeout gate only; private devnet/testnet evidence only; test-only assets only; no public launch; no mainnet; no production settlement; no real ROC mutation.
RO:SECURITY — no wallet load, no signer load, no authority-key load, no RPC call, no transaction submission, no mint, no burn, no settlement, no real ROC mutation.
RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_closeout.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 15
BUILD_PLAN4 Closeout Gate
```

This phase decides whether the private testnet / test-only bridge evidence goal is:

```text
complete / green / parked
```

This phase does not authorize:

```text
mainnet launch
public launch
production bridge settlement
public ROX mint/burn
real internal ROC release
real internal ROC mutation
public bridge UI
exchange readiness
staking readiness
liquidity readiness
real user funds
```

Those require BUILD_PLAN5 or a later explicitly authorized plan.

## Required closeout confirmations

The closeout gate confirms:

```text
all local Rust tests pass
all Anchor tests pass or are explicitly recorded as not performed in this local software-only closeout report
all actual private testnet checks pass
actual deploy receipt exists if deployment was performed
actual test-only mint initialization receipt exists if initialization was performed
live read-only RPC evidence exists or is recorded as not performed
simulation receipts exist or are recorded as not performed
capped send receipts exist if capped sends were performed
readback receipts exist for every capped send
failure receipts exist for negative drills
halt/recovery drills were performed or simulated
authority drills were performed or simulated
RustyOnions handoff remains dry-run only
CrabLink status remains display-only
no key material is tracked
no mainnet behavior exists
no public launch behavior exists
no production settlement behavior exists
no real internal ROC mutation exists
no exchange/staking/liquidity behavior exists
known pilot failures are documented or marked none_observed
BUILD_PLAN5 remains separate and future
```

## Required schema

Required schema:

```text
schema: rox-anchor.actual-private-testnet-closeout.v1
phase: BUILD_PLAN4 Phase 15
closeout_role: actual_private_testnet_closeout_gate
```

Required fields:

```text
cluster
closeout_status
build_plan4_status
build_plan5_status
local_rust_tests_status
anchor_build_status
anchor_test_status
actual_private_testnet_checks_status
deploy_receipt_status
test_only_mint_init_status
live_read_only_rpc_status
simulation_receipts_status
capped_send_receipts_status
readback_receipts_status
negative_drill_failure_receipts_status
halt_recovery_drills_status
authority_drills_status
rustyonions_handoff_status
crablink_display_status
tracked_key_material_status
mainnet_behavior_status
public_launch_status
production_settlement_status
real_internal_roc_mutation_status
exchange_staking_liquidity_status
known_pilot_failures_status
operator_report_redacted
private_testnet_only
test_only_assets_only
closeout_gate_only
future_build_plan5_required
runtime_authorization
wallet_authority
ledger_authority
bridge_authority
transaction_submission
public_launch_authorized
mainnet_authorized
production_bridge_settlement
public_rox_mint_burn
real_roc_burn
real_roc_release
real_roc_mutation
final_settlement
finality_claim
```

## Example closeout report

```json
{
  "schema": "rox-anchor.actual-private-testnet-closeout.v1",
  "phase": "BUILD_PLAN4 Phase 15",
  "closeout_role": "actual_private_testnet_closeout_gate",
  "cluster": "testnet",
  "closeout_status": "complete_green_parked",
  "build_plan4_status": "complete_green_parked",
  "build_plan5_status": "separate_future_plan",
  "local_rust_tests_status": "passed",
  "anchor_build_status": "operator_verified_or_not_performed",
  "anchor_test_status": "operator_verified_or_not_performed",
  "actual_private_testnet_checks_status": "passed",
  "deploy_receipt_status": "linked_or_not_performed",
  "test_only_mint_init_status": "linked_or_not_performed",
  "live_read_only_rpc_status": "linked_or_not_performed",
  "simulation_receipts_status": "linked_or_not_performed",
  "capped_send_receipts_status": "linked_or_not_performed",
  "readback_receipts_status": "linked_or_not_performed",
  "negative_drill_failure_receipts_status": "linked",
  "halt_recovery_drills_status": "linked",
  "authority_drills_status": "linked",
  "rustyonions_handoff_status": "dry_run_only",
  "crablink_display_status": "display_only",
  "tracked_key_material_status": "none_tracked",
  "mainnet_behavior_status": "absent",
  "public_launch_status": "absent",
  "production_settlement_status": "absent",
  "real_internal_roc_mutation_status": "absent",
  "exchange_staking_liquidity_status": "absent",
  "known_pilot_failures_status": "none_observed_or_documented",
  "operator_report_redacted": true,
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "closeout_gate_only": true,
  "future_build_plan5_required": true,
  "runtime_authorization": false,
  "wallet_authority": false,
  "ledger_authority": false,
  "bridge_authority": false,
  "transaction_submission": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "final_settlement": false,
  "finality_claim": false
}
```

## Final BUILD_PLAN4 status

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

Successful completion does not mean:

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

## Local artifact policy

Full closeout reports must remain ignored/local:

```text
.rox-anchor-private-pilot/actual-private-testnet-closeout.local.json
.rox-anchor-private-pilot/actual-private-testnet-closeout-report.local.json
.rox-anchor-private-pilot/actual-build-plan4-closeout.local.json
```

Only redacted summaries should be promoted into tracked docs.

## Safety statements

No runtime authorization.

No wallet authority.

No ledger authority.

No bridge authority.

No transaction submission.

No public launch.

No mainnet-beta.

No production bridge settlement.

No public ROX mint/burn.

No real ROC burn.

No real ROC release.

No real internal ROC mutation.

No final settlement.

No fake finality.

BUILD_PLAN5 remains separate and future.
