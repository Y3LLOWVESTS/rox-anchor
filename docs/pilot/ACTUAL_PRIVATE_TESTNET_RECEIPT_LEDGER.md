# Actual Private Testnet Receipt Ledger

RO:WHAT — Defines BUILD_PLAN4 Phase 9 receipt ledger reconciliation for actual private devnet/testnet evidence.
RO:WHY — Links deployment, initialization, read-only RPC, simulation, capped send, readback, and dry-run release-intent receipts without claiming production settlement.
RO:INTERACTS — Phase 3 deploy receipts, Phase 4 init receipts, Phase 5 read-only evidence, Phase 6 simulation, Phase 7 ROC-to-ROX, Phase 8 ROX-to-ROC.
RO:INVARIANTS — devnet/testnet only; receipt IDs unique; operation/idempotency/nonce bindings match; signatures redacted; readback required before reconciliation; no real ROC mutation.
RO:SECURITY — no committed receipts, no private paths, no raw RPC/provider tokens, no production settlement, no public launch, no fake finality.
RO:TEST — bash scripts/check_actual_private_testnet_receipts.sh --check-docs . and cargo test -p rox-anchor-relayer --test actual_private_testnet_receipt_ledger.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 9
Receipt Ledger Reconciliation for Actual Runs
```

This phase reconciles already-produced private devnet/testnet evidence.

The repo patch itself does not call RPC.

The checker does not call RPC.

The tests do not call RPC.

The repo patch does not submit transactions.

The checker does not submit transactions.

The tests do not submit transactions.

The ledger is not runtime authorization.

The ledger is not production settlement.

The ledger is not a real ROC mutation record.

## Receipt sources

The ledger may link these receipt roles:

```text
actual_private_testnet_deploy_receipt
actual_test_only_mint_init_receipt
private_testnet_read_only_rpc_evidence_receipt
actual_private_testnet_simulation_receipt
actual_roc_to_rox_capped_send_receipt
actual_roc_to_rox_readback_receipt
actual_rox_to_roc_capped_send_receipt
actual_rox_to_roc_readback_receipt
dry_run_internal_roc_release_intent_receipt
```

A receipt role can be:

```text
verified
not_performed
blocked
failed
```

`not_performed` is allowed for manual live actions that were not executed.

A ledger may still be useful if a manual run was blocked or failed, but it must not claim reconciliation success.

## Required local artifact paths

After actual private testnet evidence exists, create one of:

```text
.rox-anchor-private-pilot/actual-private-testnet-receipt-ledger.local.json
.rox-anchor-private-pilot/actual-private-testnet-receipt-ledger-incomplete.local.json
.rox-anchor-private-pilot/actual-private-testnet-receipt-ledger-quarantined.local.json
```

Then validate it:

```bash
bash scripts/check_actual_private_testnet_receipts.sh --check-ledger .rox-anchor-private-pilot/actual-private-testnet-receipt-ledger.local.json
```

or:

```bash
bash scripts/check_actual_private_testnet_receipts.sh --check-ledger .rox-anchor-private-pilot/actual-private-testnet-receipt-ledger-quarantined.local.json
```

## Preflight gate

Before using this ledger for operator review, run:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/check_actual_private_testnet_read_only_evidence.sh --preflight . testnet
bash scripts/check_actual_private_testnet_simulation.sh --preflight . testnet
bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --preflight . testnet
bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --preflight . testnet
bash scripts/check_actual_private_testnet_receipts.sh --preflight . testnet
cargo test -p rox-anchor-relayer --test actual_private_testnet_receipt_ledger
cargo test -p rox-anchor-cli --test actual_private_testnet_receipt_display
cargo test --workspace
cargo check --workspace
anchor build
```

The preflight gate is local-file/readiness only.

It does not call RPC.

It does not load a signer.

It does not sign.

It does not submit.

It does not mint.

It does not burn.

It does not settle.

It does not release real ROC.

It does not mutate internal ROC.

## Reconciled ledger schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-receipt-ledger.v1",
  "phase": "BUILD_PLAN4 Phase 9",
  "receipt_role": "actual_private_testnet_receipt_ledger",
  "cluster": "testnet",
  "ledger_id": "<redacted-ledger-id>",
  "ledger_outcome": "reconciled",
  "reconciliation_status": "reconciled",
  "operation_id": "actual-private-testnet-op-0001",
  "idempotency_key": "actual-private-testnet-idem-0001",
  "nonce": "actual-private-testnet-nonce-0001",
  "receipt_ids": "deploy-0001,init-0001,read-only-0001,simulation-0001,roc-to-rox-send-0001,roc-to-rox-readback-0001,rox-to-roc-send-0001,rox-to-roc-readback-0001",
  "receipt_operation_ids": "actual-private-testnet-op-0001",
  "receipt_idempotency_keys": "actual-private-testnet-idem-0001",
  "receipt_nonces": "actual-private-testnet-nonce-0001",
  "deploy_receipt_status": "verified",
  "initialization_receipt_status": "verified",
  "read_only_evidence_status": "verified",
  "simulation_receipt_status": "verified",
  "roc_to_rox_send_status": "verified",
  "roc_to_rox_readback_status": "verified",
  "rox_to_roc_send_status": "verified",
  "rox_to_roc_readback_status": "verified",
  "dry_run_release_intent_status": "verified",
  "receipt_chain_status": "linked",
  "operation_binding_status": "matched",
  "idempotency_binding_status": "matched",
  "nonce_binding_status": "matched",
  "signature_binding_status": "redacted",
  "readback_binding_status": "verified",
  "transaction_signatures_redacted": "<redacted-signature-list>",
  "readback_evidence_redacted": "<redacted-readback-evidence>",
  "operator_report_redacted": "<redacted-operator-report>",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "readback_verified": true,
  "duplicate_receipts_detected": false,
  "operation_id_mismatch_detected": false,
  "idempotency_key_mismatch_detected": false,
  "nonce_mismatch_detected": false,
  "live_submission_without_signature_detected": false,
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

## Incomplete or quarantined ledger schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-receipt-ledger.v1",
  "phase": "BUILD_PLAN4 Phase 9",
  "receipt_role": "actual_private_testnet_receipt_ledger",
  "cluster": "testnet",
  "ledger_id": "<redacted-ledger-id>",
  "ledger_outcome": "quarantined",
  "reconciliation_status": "quarantined",
  "operation_id": "actual-private-testnet-op-0001",
  "idempotency_key": "actual-private-testnet-idem-0001",
  "nonce": "actual-private-testnet-nonce-0001",
  "receipt_ids": "deploy-0001,init-0001,read-only-0001,simulation-0001",
  "receipt_operation_ids": "actual-private-testnet-op-0001",
  "receipt_idempotency_keys": "actual-private-testnet-idem-0001",
  "receipt_nonces": "actual-private-testnet-nonce-0001",
  "deploy_receipt_status": "verified",
  "initialization_receipt_status": "verified",
  "read_only_evidence_status": "verified",
  "simulation_receipt_status": "blocked",
  "roc_to_rox_send_status": "not_performed",
  "roc_to_rox_readback_status": "not_performed",
  "rox_to_roc_send_status": "not_performed",
  "rox_to_roc_readback_status": "not_performed",
  "dry_run_release_intent_status": "not_performed",
  "receipt_chain_status": "linked",
  "operation_binding_status": "matched",
  "idempotency_binding_status": "matched",
  "nonce_binding_status": "matched",
  "signature_binding_status": "redacted",
  "readback_binding_status": "not_performed",
  "quarantine_reason_redacted": "<redacted-reconciliation-blocker>",
  "transaction_signatures_redacted": "<redacted-signature-list>",
  "readback_evidence_redacted": "<redacted-readback-evidence>",
  "operator_report_redacted": "<redacted-operator-report>",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "readback_verified": false,
  "duplicate_receipts_detected": false,
  "operation_id_mismatch_detected": false,
  "idempotency_key_mismatch_detected": false,
  "nonce_mismatch_detected": false,
  "live_submission_without_signature_detected": false,
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

## Required clean reconciliation gates

A reconciled ledger must show:

```text
receipt IDs are unique
operation IDs match
idempotency keys match
nonces match
receipt chain is linked
readback is verified
signatures are redacted
readback evidence is redacted
all completed send/readback receipts are verified
private_testnet_only = true
test_only_assets_only = true
production_bridge_settlement = false
real_roc_release = false
real_roc_mutation = false
finality_claim = false
```

## Rejection rules

The checker must reject:

```text
mainnet-beta cluster
duplicate receipt IDs
operation ID mismatch
idempotency key mismatch
nonce mismatch
live submission claim without redacted transaction signature linkage
production settlement claim
public launch claim
public ROX mint/burn claim
real ROC release claim
real ROC mutation claim
fake finality claim
unredacted RPC URL
unredacted provider token
unredacted signer path
unredacted local receipt path
unredacted payer/keypair/authority filename
```

## What this may prove

A valid reconciled Phase 9 ledger may prove:

```text
private devnet/testnet evidence was linked into one operator-readable ledger
receipt IDs were unique
operation/idempotency/nonce bindings matched
readback evidence was present and redacted
completed private testnet test-only actions were receipt-backed
the ledger did not claim production settlement or real ROC mutation
```

## What this does not prove

A valid Phase 9 ledger does not prove:

```text
mainnet readiness
public launch readiness
production bridge settlement
real internal ROC release
public ROX minting
public ROX burning
exchange readiness
staking readiness
liquidity readiness
final settlement
```

Those require later plans and separate explicit authorization.

## Non-authorization lock

No real ROC release.

No real internal ROC mutation.

No public launch authorization.

No mainnet-beta authorization.

No production bridge settlement.

No public ROX mint/burn.

No staking.

No liquidity.

No exchange-facing behavior.

No fake finality.
