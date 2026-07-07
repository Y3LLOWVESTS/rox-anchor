# ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md

RO:WHAT — BUILD_PLAN4 Phase 13 CrabLink display-only private testnet status boundary.
RO:WHY — Lets CrabLink display backend-derived private testnet evidence without giving the client wallet, ledger, bridge, Solana, mint/burn, paid-access, or finality authority.
RO:INTERACTS — scripts/check_actual_crablink_private_testnet_status.sh and rox-anchor-cli tests.
RO:INVARIANTS — display-only; backend-derived; test-only assets; no real ROC mutation; no final settlement; no paid content unlock; no Solana submit path.
RO:SECURITY — no wallet load, no signer load, no RPC call, no transaction submission, no ROX mint/burn authority, no real ROC release, no production settlement.
RO:TEST — cargo test -p rox-anchor-cli --test actual_crablink_private_testnet_status.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 13
CrabLink Display-Only Private Testnet Status
```

This phase prepares a CrabLink-facing status payload for private devnet/testnet evidence.

The status payload is:

```text
backend-derived
display-only
test-only
private devnet/testnet only
non-authorizing
non-finality
non-settlement
non-wallet
non-ledger
non-paid-access
```

This phase does not add CrabLink runtime authority.

This phase does not touch the CrabLink repo.

This phase does not add Solana submit commands to CrabLink.

This phase does not add ROX mint/burn authority to CrabLink.

This phase does not unlock paid content from private testnet status.

This phase does not make CrabLink the wallet truth.

This phase does not make CrabLink the ledger truth.

This phase does not make CrabLink the bridge authority.

## Display-only status contents

The display payload may include:

```text
proof_status
read_only_rpc_status
receipt_status
halt_status
recovery_status
dry_run_internal_roc_status
rustyonions_handoff_status
test_only_asset_label
private_testnet_label
```

The display payload must clearly label:

```text
test-only assets
private devnet/testnet evidence
no real ROC mutation
no final settlement
no production bridge settlement
no mainnet authorization
no public launch authorization
no public ROX mint/burn
no paid content unlock
```

## Required schema

Required schema:

```text
schema: rox-anchor.actual-crablink-private-testnet-status.v1
phase: BUILD_PLAN4 Phase 13
status_role: actual_crablink_private_testnet_display_status
```

Required fields:

```text
cluster
display_status
proof_status
read_only_rpc_status
receipt_status
halt_status
recovery_status
dry_run_internal_roc_status
rustyonions_handoff_status
test_only_asset_label
private_testnet_label
backend_derived
display_only
private_testnet_only
test_only_assets_only
dry_run_only
client_authority
wallet_authority
ledger_authority
bridge_authority
solana_submit_command_available
rox_mint_burn_authority
paid_content_unlock
real_roc_burn
real_roc_release
real_roc_mutation
production_bridge_settlement
final_settlement
public_rox_mint_burn
mainnet_authorized
public_launch_authorized
public_bridge_ui
finality_claim
operator_report_redacted
```

## Example display-only status payload

```json
{
  "schema": "rox-anchor.actual-crablink-private-testnet-status.v1",
  "phase": "BUILD_PLAN4 Phase 13",
  "status_role": "actual_crablink_private_testnet_display_status",
  "cluster": "testnet",
  "display_status": "display_only",
  "proof_status": "accepted",
  "read_only_rpc_status": "verified",
  "receipt_status": "linked",
  "halt_status": "not_active",
  "recovery_status": "not_required",
  "dry_run_internal_roc_status": "dry_run_only",
  "rustyonions_handoff_status": "dry_run_recorded",
  "test_only_asset_label": "TEST-ONLY ROX",
  "private_testnet_label": "PRIVATE TESTNET STATUS",
  "backend_derived": true,
  "display_only": true,
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "dry_run_only": true,
  "client_authority": false,
  "wallet_authority": false,
  "ledger_authority": false,
  "bridge_authority": false,
  "solana_submit_command_available": false,
  "rox_mint_burn_authority": false,
  "paid_content_unlock": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "production_bridge_settlement": false,
  "final_settlement": false,
  "public_rox_mint_burn": false,
  "mainnet_authorized": false,
  "public_launch_authorized": false,
  "public_bridge_ui": false,
  "finality_claim": false,
  "operator_report_redacted": true
}
```

## Local artifact policy

If an operator exports a private testnet display status report, it must remain ignored/local:

```text
.rox-anchor-private-pilot/actual-crablink-private-testnet-status.local.json
.rox-anchor-private-pilot/actual-crablink-display-only-status.local.json
.rox-anchor-private-pilot/actual-crablink-status-report.local.json
```

## Safety statements

No Solana submit commands in CrabLink.

No ROX mint/burn authority in CrabLink.

No paid content unlock from private testnet status.

No wallet authority.

No ledger authority.

No bridge authority.

No real ROC burn.

No real ROC release.

No real ROC mutation.

No production bridge settlement.

No final settlement.

No public ROX mint/burn.

No mainnet-beta.

No public launch.

No fake finality.

CrabLink status remains display-only.
