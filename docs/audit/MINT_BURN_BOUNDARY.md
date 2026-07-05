# ROX Anchor Phase 14 — Mint/Burn Boundary

No public launch authorization.

This document summarizes the ROC ↔ ROX mint/burn boundary as currently implemented and tested. The current surface is local/testnet-only and does not authorize public ROX minting, public ROX burning, production bridge settlement, production ROC release, or mainnet use.

## Boundary files

```text
programs/rox-anchor/src/state.rs
programs/rox-anchor/src/instructions/finalize.rs
programs/rox-anchor/src/instructions/observe_burn.rs
crates/rox-anchor-core/tests/test_only_asset_harness.rs
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
crates/rox-anchor-relayer/tests/capped_testnet_submission.rs
```

## Directional model

| Direction | Local meaning | Required guardrails |
| --- | --- | --- |
| ROC -> ROX | Local/testnet-shaped ROC burn observation can map to ROX mint intent. | Proof binding, operation replay rejection, mint authority binding, token account binding, challenge/halt/recovery clear state, test-only asset cap. |
| ROX -> ROC | Local/testnet-shaped ROX burn observation can map to internal ROC release intent. | Proof binding, burn source binding, no real ROC release, challenge/halt/recovery clear state, test-only asset cap. |

## Test coverage

```text
crates/rox-anchor-core/tests/test_only_asset_harness.rs
crates/rox-anchor-coordinator/tests/local_nonvalue_bidirectional_flow.rs
crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs
programs/rox-anchor/src/state.rs tests:
  roc_to_rox_finalization_classifies_rox_mint_output
  rox_to_roc_finalization_classifies_internal_roc_release
  token_settlement_binding_derives_roc_to_rox_intent_from_config_and_plan
  token_settlement_binding_derives_rox_to_roc_intent_from_config_and_plan
  token_cpi_readiness_accepts_roc_to_rox_planned_event
  token_cpi_readiness_accepts_rox_to_roc_planned_event
  token_cpi_execution_receipt_accepts_live_roc_to_rox_mint_delta
  token_cpi_execution_receipt_accepts_live_rox_to_roc_burn_delta
  token_settlement_execution_receipt_* tamper checks
```

## Required rejects

The boundary must reject:

```text
wrong mint
wrong token account
wrong mint authority
wrong direction
wrong program/config key
short ROX balance for burn-shaped path
plan tamper
event tamper
receipt tamper
pre-finalized operation
finalized operation reopen
public/production mint labels
zero and over-cap test amounts
```

## Audit note

The model proves shape and guardrails. Production value movement and public bridge behavior remain outside this plan.
