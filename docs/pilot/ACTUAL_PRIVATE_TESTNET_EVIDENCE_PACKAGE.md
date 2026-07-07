# ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md

RO:WHAT — BUILD_PLAN4 Phase 14 actual private testnet evidence package boundary.
RO:WHY — Collects private devnet/testnet evidence into an audit-ready redacted index without authorizing public launch, settlement, mainnet, or real ROC mutation.
RO:INTERACTS — scripts/check_actual_private_testnet_evidence_package.sh and rox-anchor-cli tests.
RO:INVARIANTS — evidence package only; redacted; test-only; private devnet/testnet only; no production settlement; no real ROC mutation; no finality claim.
RO:SECURITY — no wallet load, no signer load, no authority-key load, no RPC call, no transaction submission, no mint, no burn, no settlement, no real ROC mutation.
RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_evidence_package.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 14
Actual Private Testnet Evidence Package
```

This phase creates an audit-ready evidence index for the private devnet/testnet and test-only bridge evidence surfaces.

The evidence package is:

```text
redacted
operator-readable
test-only
private devnet/testnet only
evidence-index only
non-authorizing
non-runtime
non-mainnet
non-production
non-settlement
non-finality
```

This phase does not call RPC.

This phase does not load wallets.

This phase does not load signers.

This phase does not load authority keys.

This phase does not submit transactions.

This phase does not mint or burn.

This phase does not settle.

This phase does not release real ROC.

This phase does not mutate real ROC.

## Evidence package contents

The evidence package should index the following evidence surfaces:

```text
build artifact manifest
deployment receipt or safe failed-deployment receipt
test-only mint/config initialization receipt
read-only RPC evidence receipt
simulation receipts
ROC-to-ROX capped testnet receipt/readback, if performed
ROX-to-ROC capped testnet receipt/readback, if performed
receipt ledger reconciliation
negative drill failure receipts
halt/recovery drill reports
authority drill reports
RustyOnions dry-run handoff report
CrabLink display-only status report
```

The package may mark a surface as:

```text
linked
linked_or_not_performed
not_performed
failed_safe
quarantined
missing
```

A missing or not-performed manual artifact is not a success claim.

A failed-safe artifact is not a success claim.

A quarantined artifact is not a success claim.

## Required schema

Required schema:

```text
schema: rox-anchor.actual-private-testnet-evidence-package.v1
phase: BUILD_PLAN4 Phase 14
package_role: actual_private_testnet_evidence_package
```

Required fields:

```text
cluster
package_id
evidence_index_status
build_artifact_manifest_status
deploy_receipt_status
test_only_mint_init_status
read_only_rpc_evidence_status
simulation_receipts_status
roc_to_rox_receipts_status
rox_to_roc_receipts_status
receipt_ledger_status
negative_drill_receipts_status
halt_recovery_reports_status
authority_reports_status
rustyonions_handoff_status
crablink_display_status
operation_id_linkage_status
idempotency_key_linkage_status
receipt_id_linkage_status
redaction_status
operator_report_redacted
private_testnet_only
test_only_assets_only
evidence_package_only
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

## Example evidence package

```json
{
  "schema": "rox-anchor.actual-private-testnet-evidence-package.v1",
  "phase": "BUILD_PLAN4 Phase 14",
  "package_role": "actual_private_testnet_evidence_package",
  "cluster": "testnet",
  "package_id": "actual-private-testnet-evidence-package-0001",
  "evidence_index_status": "audit_ready",
  "build_artifact_manifest_status": "linked",
  "deploy_receipt_status": "linked_or_not_performed",
  "test_only_mint_init_status": "linked_or_not_performed",
  "read_only_rpc_evidence_status": "linked_or_not_performed",
  "simulation_receipts_status": "linked_or_not_performed",
  "roc_to_rox_receipts_status": "linked_or_not_performed",
  "rox_to_roc_receipts_status": "linked_or_not_performed",
  "receipt_ledger_status": "linked",
  "negative_drill_receipts_status": "linked",
  "halt_recovery_reports_status": "linked",
  "authority_reports_status": "linked",
  "rustyonions_handoff_status": "linked",
  "crablink_display_status": "linked",
  "operation_id_linkage_status": "validated",
  "idempotency_key_linkage_status": "validated",
  "receipt_id_linkage_status": "validated",
  "redaction_status": "redacted",
  "operator_report_redacted": true,
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "evidence_package_only": true,
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

## Local artifact policy

Full evidence package exports must remain ignored/local:

```text
.rox-anchor-private-pilot/actual-private-testnet-evidence-package.local.json
.rox-anchor-private-pilot/actual-private-testnet-evidence-index.local.json
.rox-anchor-private-pilot/actual-private-testnet-evidence-report.local.json
```

Only redacted summaries should be promoted into tracked docs.

## Safety statements

No wallet authority.

No ledger authority.

No bridge authority.

No runtime authorization.

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

The actual private testnet evidence package is an audit index only.
