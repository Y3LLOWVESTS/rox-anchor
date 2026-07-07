# ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md

RO:WHAT — BUILD_PLAN4 Phase 10 runbook for actual private testnet negative drills.
RO:WHY — Proves deployed private devnet/testnet operations fail safely before any later halt/recovery/handoff work.
RO:INTERACTS — scripts/check_actual_private_testnet_negative_drills.sh, relayer receipts, coordinator decisions, rpc-proof evidence, CLI reports.
RO:INVARIANTS — devnet/testnet only; test-only assets only; expected failures only; no live submission by default; no production settlement; no real ROC mutation; no finality claim.
RO:SECURITY — no committed keys, no raw RPC provider tokens, no key paths, no public mint/burn, no mainnet, no real internal ROC release.
RO:TEST — bash scripts/check_actual_private_testnet_negative_drills.sh --check-docs . and cargo test -p rox-anchor-relayer --test actual_testnet_negative_drills.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 10
Actual Negative Drills Against Deployed Testnet State
```

Phase 10 proves that actual private devnet/testnet operations fail safely under bad bindings, stale evidence, unsafe operator state, replay attempts, and missing receipts.

The repo patch itself does not call RPC.

The checker does not call RPC.

The tests do not call RPC.

The repo patch does not submit transactions.

The checker does not submit transactions.

The tests do not submit transactions.

The negative drill receipt is not runtime authorization.

The negative drill receipt is not production settlement.

The negative drill receipt is not a real ROC mutation record.

## Scope

Phase 10 is allowed to model and inspect failure receipts for private devnet/testnet only.

Allowed clusters:

```text
devnet
testnet
```

Forbidden clusters:

```text
mainnet-beta
mainnet
production
public
```

Allowed receipt outcomes:

```text
blocked
failed_safe
quarantined
not_performed
```

Avoid ambiguous success-like wording in negative-drill receipts. A negative drill should prove that unsafe input is blocked, quarantined, or fails safe.

## Required negative drill matrix

Every Phase 10 implementation surface must preserve this drill matrix:

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

Canonical receipt `drill_name` values:

```text
wrong_program_id
wrong_mint
wrong_token_account
wrong_authority
missing_config_account
missing_mint_account
stale_readback
under_quorum_rpc_evidence
rpc_provider_disagreement
duplicate_operation_id
duplicate_idempotency_key
nonce_reuse
receipt_tamper
missing_receipt
operator_approval_omitted
send_disabled
cap_exceeded
halt_before_simulation
halt_after_simulation_before_send
halt_after_send_before_readback
recovery_during_pending_operation
readback_missing_after_send
```

## Failure receipt schema

A Phase 10 failure receipt must use:

```text
schema: rox-anchor.actual-private-testnet-negative-drill.v1
phase: BUILD_PLAN4 Phase 10
receipt_role: actual_private_testnet_negative_drill_receipt
```

Required fields:

```text
cluster
drill_name
drill_outcome
operation_id
idempotency_key
nonce
expected_failure
failure_reason_redacted
proof_review_status
coordinator_decision_status
relayer_status
readback_status
private_testnet_only
test_only_assets_only
system_returned_safe_state
transaction_submission
send_authorized
signature_generated
public_mint_available
public_launch_authorized
mainnet_authorized
production_bridge_settlement
public_rox_mint_burn
real_roc_release
real_roc_mutation
finality_claim
```

Expected negative status values:

```text
proof_review_status: rejected / blocked / disputed / missing_evidence
coordinator_decision_status: rejected / blocked
relayer_status: blocked / not_authorized
readback_status: missing / rejected / disputed / not_performed
```

## Example blocked receipt

```json
{
  "schema": "rox-anchor.actual-private-testnet-negative-drill.v1",
  "phase": "BUILD_PLAN4 Phase 10",
  "receipt_role": "actual_private_testnet_negative_drill_receipt",
  "cluster": "testnet",
  "drill_name": "wrong_program_id",
  "drill_outcome": "blocked",
  "operation_id": "actual-negative-drill-op-0001",
  "idempotency_key": "actual-negative-drill-idem-0001",
  "nonce": "actual-negative-drill-nonce-0001",
  "expected_failure": true,
  "failure_reason_redacted": "<redacted-safe-negative-drill-failure>",
  "proof_review_status": "rejected",
  "coordinator_decision_status": "rejected",
  "relayer_status": "blocked",
  "readback_status": "not_performed",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "system_returned_safe_state": true,
  "transaction_submission": false,
  "send_authorized": false,
  "signature_generated": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Preflight requirements

Before manual negative drills:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/check_actual_private_testnet_read_only_evidence.sh --preflight . testnet
bash scripts/check_actual_private_testnet_simulation.sh --preflight . testnet
bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --preflight . testnet
bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --preflight . testnet
bash scripts/check_actual_private_testnet_receipts.sh --preflight . testnet
bash scripts/check_actual_private_testnet_negative_drills.sh --preflight . testnet
```

## Manual drill rule

Manual negative drills may only happen after fake/adapted tests pass.

Manual drills must be:

```text
tiny
explicit
operator-approved
receipt-backed
external-keyed
external-configured
ignored-local-artifact-only
devnet/testnet only
```

Manual drills must not:

```text
load keys by default
submit by default
use mainnet-beta
use public launch labels
claim settlement
claim finality
release real ROC
mutate real internal ROC
write committed local receipts
leak RPC provider tokens
leak key paths
```

## Required local artifact paths

After actual private testnet evidence exists, create receipts only in ignored local paths such as:

```text
.rox-anchor-private-pilot/actual-private-testnet-negative-drill.local.json
.rox-anchor-private-pilot/actual-private-testnet-negative-drill-quarantined.local.json
.rox-anchor-private-pilot/actual-negative-drill-report.local.json
```

## Safety statements

No real ROC release.

No real internal ROC mutation.

No production bridge settlement.

No public ROX mint/burn.

No public launch.

No mainnet-beta.

No fake finality.

The negative drill receipt is not runtime authorization.

The negative drill receipt is not production settlement.

The negative drill receipt is not a real ROC mutation record.
