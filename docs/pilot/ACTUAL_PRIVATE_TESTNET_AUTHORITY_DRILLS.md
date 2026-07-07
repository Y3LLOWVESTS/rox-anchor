# ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md

RO:WHAT — BUILD_PLAN4 Phase 11 runbook for actual private testnet authority drills.
RO:WHY — Proves wrong-authority, key-rotation-intent, and upgrade-authority checklist procedures are inspectable and safe.
RO:INTERACTS — scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh, authority model tests, CLI drill reports.
RO:INVARIANTS — separated authorities; wrong authority attempts fail safe; key rotation intent is not execution; upgrade checklist is not deployment/upgrade proof.
RO:SECURITY — no committed authority key material, no raw authority paths, no wallet/key loading in tests, no production/mainnet behavior.
RO:TEST — cargo test -p rox-anchor-core --test actual_private_testnet_authority_drills and cargo test -p rox-anchor-cli --test actual_private_testnet_drill_reports.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 11
Actual Authority Drills
```

This phase records authority safety procedure reports for private devnet/testnet only.

Authority drill reports are not runtime authorization.

Authority drill reports are not upgrade execution proof.

Authority drill reports are not key-rotation execution proof.

Authority drill reports are not production settlement.

Authority drill reports are not real ROC mutation records.

## Authority drill matrix

Required authority drills:

```text
wrong-authority halt attempt
wrong-authority recovery attempt
key rotation intent drill
upgrade authority checklist
separated authority status report
```

Canonical `drill_name` values:

```text
wrong_authority_halt_attempt
wrong_authority_recovery_attempt
key_rotation_intent
upgrade_authority_checklist
separated_authority_status
```

## Required authority behavior

The system must prove:

```text
wrong halt authority cannot halt
wrong recovery authority cannot recover
key rotation intent can be recorded without executing key rotation
upgrade authority checklist can be reviewed without executing upgrade authority change
authority roles remain separated and redacted
operator reports do not leak raw key paths
operator reports do not leak raw private keys
operator reports do not leak RPC/provider tokens
```

The system must not claim:

```text
key rotation executed
upgrade authority changed
wallet loaded
authority key loaded
signature generated
transaction submitted
production settlement
public launch
mainnet authorization
public ROX mint/burn
real ROC release
real ROC mutation
fake finality
```

## Authority report schema

Required schema:

```text
schema: rox-anchor.actual-private-testnet-authority-drill.v1
phase: BUILD_PLAN4 Phase 11
receipt_role: actual_private_testnet_authority_drill_report
```

Required authority status values:

```text
validated
rejected
intent_recorded
reviewed
separated
```

Required non-authorizing booleans:

```text
transaction_submission: false
send_authorized: false
wallet_loaded: false
signature_generated: false
authority_key_loaded: false
key_rotation_executed: false
upgrade_authority_changed: false
production_bridge_settlement: false
real_roc_release: false
real_roc_mutation: false
finality_claim: false
```

## Example wrong-authority halt report

```json
{
  "schema": "rox-anchor.actual-private-testnet-authority-drill.v1",
  "phase": "BUILD_PLAN4 Phase 11",
  "receipt_role": "actual_private_testnet_authority_drill_report",
  "cluster": "testnet",
  "drill_name": "wrong_authority_halt_attempt",
  "drill_outcome": "blocked",
  "operation_id": "actual-authority-drill-op-0002",
  "idempotency_key": "actual-authority-drill-idem-0002",
  "nonce": "actual-authority-drill-nonce-0002",
  "expected_drill": true,
  "action_reason_redacted": "<redacted-safe-authority-drill-action>",
  "halt_status": "attempt_rejected",
  "recovery_status": "not_required",
  "authority_status": "rejected",
  "clean_flow_resume_status": "not_tested",
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

## Safety statements

No key material in repo.

No raw authority paths in committed files.

No wallet loading in tests.

No authority key loading in tests.

No key rotation execution in this patch.

No upgrade authority change in this patch.

No real ROC release.

No real internal ROC mutation.

No production bridge settlement.

No public ROX mint/burn.

No mainnet-beta.

No fake finality.
