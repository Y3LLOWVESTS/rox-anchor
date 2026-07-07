# Actual Private Testnet Deployment

RO:WHAT — Defines BUILD_PLAN4 Phase 3 actual private devnet/testnet deployment receipt handling.
RO:WHY — Lets an operator deploy or safely attempt deploy with external keys while capturing only redacted, non-secret evidence.
RO:INTERACTS — Anchor.toml, target/deploy/rox_anchor.so, target/idl/rox_anchor.json, external payer/keypair paths, scripts/check_actual_private_testnet_deploy_receipt.sh.
RO:INVARIANTS — manual only; devnet/testnet only; external keys only; redacted receipt only; no finality/public/mainnet/production/real ROC claim.
RO:SECURITY — no silent wallet usage, no default live submission, no committed keys, no raw provider tokens, no bridge settlement, no internal ROC mutation.
RO:TEST — bash scripts/check_actual_private_testnet_deploy_receipt.sh --check-docs . and cargo test -p rox-anchor-cli --test actual_private_testnet_deploy_receipt.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 3
Actual Private Devnet/Testnet Deployment
```

This is the first BUILD_PLAN4 phase where a manual live private devnet/testnet deploy may be performed.

The repo patch itself does not deploy.

The checker does not deploy.

The tests do not deploy.

Deployment must remain a separate operator action after the preflight gate is green.

## Required pre-manual gate

Before any actual deploy attempt, run:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_private_testnet_deploy.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/capture_actual_private_testnet_build_artifacts.sh --check-docs .
anchor build
cargo test --workspace
cargo check --workspace
bash scripts/check_actual_private_testnet_deploy_receipt.sh --preflight . testnet
```

The preflight gate is read-only.

It does not call RPC.

It does not load a wallet.

It does not deploy.

It does not submit.

It does not mint.

It does not burn.

It does not settle.

It does not mutate internal ROC.

## Manual deploy command shape

Only after the preflight gate passes and the operator explicitly chooses to proceed:

```bash
# anchor deploy \
#   --provider.cluster testnet \
#   --provider.wallet /external/private/<redacted-external-payer-file>
```

or:

```bash
# anchor deploy \
#   --provider.cluster devnet \
#   --provider.wallet /external/private/<redacted-external-payer-file>
```

The wallet path must be external or ignored.

The deploy authority and upgrade authority material must be external or ignored.

The command output must be copied only into ignored local artifact paths until redacted.

## Required receipt

After a manual deploy attempt, create one of:

```text
.rox-anchor-private-pilot/actual-private-testnet-deploy-receipt.local.json
.rox-anchor-private-pilot/actual-private-testnet-deploy-failed.local.json
```

The receipt must be checked with:

```bash
bash scripts/check_actual_private_testnet_deploy_receipt.sh --check-receipt .rox-anchor-private-pilot/actual-private-testnet-deploy-receipt.local.json
```

or:

```bash
bash scripts/check_actual_private_testnet_deploy_receipt.sh --check-receipt .rox-anchor-private-pilot/actual-private-testnet-deploy-failed.local.json
```

A successful deploy receipt is still not finality proof.

A failed deploy receipt is still useful evidence if it is redacted and explains the failure safely.

## Receipt schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-deploy-receipt.v1",
  "phase": "BUILD_PLAN4 Phase 3",
  "receipt_role": "private_testnet_deployment_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "deployment_outcome": "succeeded",
  "deploy_signature": "<redacted-signature>",
  "deploy_slot": "0",
  "program_binary_sha256": "<sha256>",
  "idl_sha256": "<sha256>",
  "build_manifest_path": "<redacted-local-build-manifest>",
  "payer_redacted": "<redacted-external-payer>",
  "deploy_authority_redacted": "<redacted-external-deploy-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "failure_reason_redacted": "not_applicable",
  "deploy_command_was_manual": true,
  "preflight_passed": true,
  "program_account_readback_verified": false,
  "idl_account_readback_verified": false,
  "deployment_success_claim_scope": "private_devnet_testnet_only",
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false
}
```

## Safe failed receipt schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-deploy-receipt.v1",
  "phase": "BUILD_PLAN4 Phase 3",
  "receipt_role": "private_testnet_deployment_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "deployment_outcome": "failed",
  "deploy_signature": "none",
  "deploy_slot": "none",
  "program_binary_sha256": "<sha256>",
  "idl_sha256": "<sha256>",
  "build_manifest_path": "<redacted-local-build-manifest>",
  "payer_redacted": "<redacted-external-payer>",
  "deploy_authority_redacted": "<redacted-external-deploy-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "failure_reason_redacted": "<redacted-safe-failure-reason>",
  "deploy_command_was_manual": true,
  "preflight_passed": true,
  "program_account_readback_verified": false,
  "idl_account_readback_verified": false,
  "deployment_success_claim_scope": "none",
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false
}
```

## What the receipt may prove

A valid Phase 3 receipt may prove:

```text
a devnet/testnet deploy attempt occurred
the attempt used a specific program ID
the attempt referenced a specific program binary hash
the attempt referenced a specific IDL hash
the result was succeeded or failed
the output was redacted
the payer/deploy authority were not committed
```

## What the receipt does not prove

A valid Phase 3 receipt does not prove:

```text
program account readback
program account executable status
IDL account readback
test-only mint initialization
transaction simulation success
capped bridge test flow success
settlement
finality beyond private devnet/testnet receipt scope
public launch readiness
mainnet readiness
production readiness
real internal ROC release
```

Those are later BUILD_PLAN4 phases.

## Forbidden

```text
mainnet-beta
public launch claims
public ROX mint/burn claims
production bridge settlement
real internal ROC release
real user funds
unredacted payer path
unredacted keypair path
unredacted deploy authority path
raw RPC provider token
committed deploy output
fake finality
fake success output
silent wallet/key usage
silent live RPC submission
staking
liquidity
exchange-facing behavior
```

## Non-authorization lock

No public launch authorization.

No mainnet-beta authorization.

No production bridge settlement.

No public ROX mint/burn.

No real internal ROC release.

No staking.

No liquidity.

No exchange-facing behavior.

No fake finality.
