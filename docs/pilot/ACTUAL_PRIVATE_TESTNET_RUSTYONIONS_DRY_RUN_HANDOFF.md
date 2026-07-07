# ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md

RO:WHAT — BUILD_PLAN4 Phase 12 runbook for RustyOnions dry-run handoff evidence.
RO:WHY — Connects actual private testnet receipt evidence to RustyOnions intent/status shapes without mutating real ROC.
RO:INTERACTS — scripts/check_actual_rustyonions_dry_run_handoff.sh, rox-anchor-core, rox-anchor-coordinator, rox-anchor-cli tests.
RO:INVARIANTS — dry-run only; svc-wallet/ron-ledger remain the only future real ROC mutation boundary; ROX Anchor never issues/releases/mutates real ROC.
RO:SECURITY — no live wallet calls, no ledger mutation, no RPC submission, no production settlement, no public mint/burn, no finality claim.
RO:TEST — cargo test -p rox-anchor-core --test actual_rustyonions_dry_run_handoff; cargo test -p rox-anchor-coordinator --test actual_rustyonions_dry_run_handoff; cargo test -p rox-anchor-cli --test actual_rustyonions_dry_run_status.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 12
RustyOnions Dry-Run Handoff Evidence
```

This phase links actual private devnet/testnet evidence to RustyOnions-facing dry-run intent/status records.

This phase does not authorize:

```text
real internal ROC mutation
real ROC burn
real ROC release
svc-wallet mutation
ron-ledger mutation
production bridge settlement
mainnet-beta
public ROX mint/burn
public launch
public bridge UI
exchange-facing behavior
staking
liquidity
fake finality
```

## Hard authority boundary

The future real ROC path remains:

```text
svc-wallet -> ron-ledger
```

ROX Anchor remains:

```text
proof/evidence/Solana program/relayer/readback/reconciliation path
```

ROX Anchor must not become:

```text
wallet truth
ledger truth
real ROC issuer
real ROC releaser
real ROC burn authority
real ROC mutation authority
paid-access truth
production bridge settlement authority
```

## Handoff directions

Supported dry-run directions:

```text
roc_to_rox
rox_to_roc
```

For `roc_to_rox`, the handoff may describe:

```text
shadow_roc_burn_intent_only
test_only_rox_observation
dry_run_status
```

It must not perform:

```text
real_roc_burn
real_roc_mutation
ron_ledger_mutation
svc_wallet_mutation
```

For `rox_to_roc`, the handoff may describe:

```text
test_only_rox_burn_observation
internal_roc_release_intent_only
dry_run_status
```

It must not perform:

```text
real_roc_release
real_roc_mutation
ron_ledger_mutation
svc_wallet_mutation
```

## Required dry-run handoff schema

Required schema:

```text
schema: rox-anchor.actual-rustyonions-dry-run-handoff.v1
phase: BUILD_PLAN4 Phase 12
report_role: actual_rustyonions_dry_run_handoff_report
```

Required fields:

```text
cluster
direction
operation_id
idempotency_key
nonce
source_receipt_ledger_status
source_private_testnet_receipts_status
proof_review_status
coordinator_decision_status
relayer_status
rustyonions_handoff_status
rustyonions_target_boundary
dry_run_only
shadow_roc_burn_intent_only
internal_roc_release_intent_only
svc_wallet_mutation
ron_ledger_mutation
real_roc_burn
real_roc_release
real_roc_mutation
production_bridge_settlement
public_rox_mint_burn
mainnet_authorized
public_launch_authorized
finality_claim
operator_report_redacted
```

## Example ROC-to-ROX dry-run handoff

```json
{
  "schema": "rox-anchor.actual-rustyonions-dry-run-handoff.v1",
  "phase": "BUILD_PLAN4 Phase 12",
  "report_role": "actual_rustyonions_dry_run_handoff_report",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "operation_id": "actual-rustyonions-dry-run-op-0001",
  "idempotency_key": "actual-rustyonions-dry-run-idem-0001",
  "nonce": "actual-rustyonions-dry-run-nonce-0001",
  "source_receipt_ledger_status": "linked",
  "source_private_testnet_receipts_status": "redacted_verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_status": "dry_run_only",
  "rustyonions_handoff_status": "dry_run_recorded",
  "rustyonions_target_boundary": "svc-wallet -> ron-ledger",
  "dry_run_only": true,
  "shadow_roc_burn_intent_only": true,
  "internal_roc_release_intent_only": false,
  "svc_wallet_mutation": false,
  "ron_ledger_mutation": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "mainnet_authorized": false,
  "public_launch_authorized": false,
  "finality_claim": false,
  "operator_report_redacted": true
}
```

## Example ROX-to-ROC dry-run handoff

```json
{
  "schema": "rox-anchor.actual-rustyonions-dry-run-handoff.v1",
  "phase": "BUILD_PLAN4 Phase 12",
  "report_role": "actual_rustyonions_dry_run_handoff_report",
  "cluster": "testnet",
  "direction": "rox_to_roc",
  "operation_id": "actual-rustyonions-dry-run-op-0002",
  "idempotency_key": "actual-rustyonions-dry-run-idem-0002",
  "nonce": "actual-rustyonions-dry-run-nonce-0002",
  "source_receipt_ledger_status": "linked",
  "source_private_testnet_receipts_status": "redacted_verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_status": "dry_run_only",
  "rustyonions_handoff_status": "dry_run_recorded",
  "rustyonions_target_boundary": "svc-wallet -> ron-ledger",
  "dry_run_only": true,
  "shadow_roc_burn_intent_only": false,
  "internal_roc_release_intent_only": true,
  "svc_wallet_mutation": false,
  "ron_ledger_mutation": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "mainnet_authorized": false,
  "public_launch_authorized": false,
  "finality_claim": false,
  "operator_report_redacted": true
}
```

## Local artifact policy

Dry-run handoff reports must remain in ignored local artifact paths such as:

```text
.rox-anchor-private-pilot/actual-rustyonions-dry-run-handoff.local.json
.rox-anchor-private-pilot/actual-rustyonions-roc-to-rox-dry-run-handoff.local.json
.rox-anchor-private-pilot/actual-rustyonions-rox-to-roc-dry-run-handoff.local.json
```

## Safety statements

No real ROC burn.

No real ROC release.

No real internal ROC mutation.

No svc-wallet mutation.

No ron-ledger mutation.

No production bridge settlement.

No public ROX mint/burn.

No public launch.

No mainnet-beta.

No fake finality.

The RustyOnions handoff remains dry-run only.
