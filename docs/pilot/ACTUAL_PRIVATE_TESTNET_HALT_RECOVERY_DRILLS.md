# ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md

RO:WHAT — BUILD_PLAN4 Phase 11 runbook for actual private testnet halt/recovery drills.
RO:WHY — Proves operator halt and recovery procedures fail closed and resume only after valid recovery.
RO:INTERACTS — scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh, rox-anchor-core tests, rox-anchor-cli reports.
RO:INVARIANTS — devnet/testnet only; test-only assets only; halted states block unsafe progress; recovery must be explicit; no production settlement; no real ROC mutation.
RO:SECURITY — no committed keys, no raw authority paths, no key loading in tests, no live send by default, no public/mainnet behavior.
RO:TEST — cargo test -p rox-anchor-core --test actual_private_testnet_authority_drills and cargo test -p rox-anchor-cli --test actual_private_testnet_drill_reports.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 11
Actual Halt, Recovery, and Authority Drills
```

This phase practices operator safety procedures against deployed private devnet/testnet state.

This repo patch does not call RPC.

This repo patch does not load wallets.

This repo patch does not load authority keys.

This repo patch does not sign transactions.

This repo patch does not submit transactions.

This repo patch does not mint or burn.

This repo patch does not settle.

This repo patch does not release real ROC.

This repo patch does not mutate real ROC.

## Halt/recovery drill matrix

Required halt/recovery drills:

```text
halt before simulation
halt after simulation before send
halt after capped send before readback
valid recovery after halt
clean flow after valid recovery
```

Canonical `drill_name` values:

```text
halt_before_simulation
halt_after_simulation_before_send
halt_after_capped_send_before_readback
valid_recovery_after_halt
clean_flow_after_valid_recovery
```

## Required halt/recovery behavior

The system must prove:

```text
halt before simulation blocks simulation
halt after simulation before send blocks send
halt after capped send before readback blocks finalization/readback promotion
valid recovery clears halt only after explicit recovery authority validation
clean flow resumes only after valid recovery
```

The system must not claim:

```text
production settlement
public launch
mainnet authorization
public ROX mint/burn
real ROC release
real ROC mutation
fake finality
```

## Halt/recovery drill receipt schema

Required schema:

```text
schema: rox-anchor.actual-private-testnet-authority-drill.v1
phase: BUILD_PLAN4 Phase 11
receipt_role: actual_private_testnet_authority_drill_report
```

Required fields:

```text
cluster
drill_name
drill_outcome
operation_id
idempotency_key
nonce
expected_drill
action_reason_redacted
halt_status
recovery_status
authority_status
clean_flow_resume_status
private_testnet_only
test_only_assets_only
system_returned_safe_state
operator_report_redacted
transaction_submission
send_authorized
wallet_loaded
signature_generated
authority_key_loaded
key_rotation_executed
upgrade_authority_changed
public_mint_available
public_launch_authorized
mainnet_authorized
production_bridge_settlement
public_rox_mint_burn
real_roc_release
real_roc_mutation
finality_claim
```

## Example valid recovery report

```json
{
  "schema": "rox-anchor.actual-private-testnet-authority-drill.v1",
  "phase": "BUILD_PLAN4 Phase 11",
  "receipt_role": "actual_private_testnet_authority_drill_report",
  "cluster": "testnet",
  "drill_name": "valid_recovery_after_halt",
  "drill_outcome": "recovered",
  "operation_id": "actual-authority-drill-op-0001",
  "idempotency_key": "actual-authority-drill-idem-0001",
  "nonce": "actual-authority-drill-nonce-0001",
  "expected_drill": true,
  "action_reason_redacted": "<redacted-safe-authority-drill-action>",
  "halt_status": "cleared",
  "recovery_status": "validated",
  "authority_status": "validated",
  "clean_flow_resume_status": "allowed_after_valid_recovery",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "system_returned_safe_state": true,
  "operator_report_redacted": true,
  "transaction_submission": false,
  "send_authorized": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "authority_key_loaded": false,
  "key_rotation_executed": false,
  "upgrade_authority_changed": false,
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

## Required local artifact paths

Actual halt/recovery drill reports must remain in ignored local artifact paths such as:

```text
.rox-anchor-private-pilot/actual-private-testnet-halt-recovery-drill.local.json
.rox-anchor-private-pilot/actual-private-testnet-authority-drill.local.json
.rox-anchor-private-pilot/actual-private-testnet-authority-report.local.json
```

## Safety statements

No real ROC release.

No real internal ROC mutation.

No production bridge settlement.

No public ROX mint/burn.

No public launch.

No mainnet-beta.

No fake finality.

The halt/recovery drill report is not runtime authorization.

The halt/recovery drill report is not production settlement.

The halt/recovery drill report is not a real ROC mutation record.
