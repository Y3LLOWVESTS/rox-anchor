# Actual Test-Only ROX Mint Initialization

RO:WHAT — Defines BUILD_PLAN4 Phase 4 actual test-only ROX mint and program-config initialization evidence.
RO:WHY — Captures manual initialization receipts for private devnet/testnet test-only assets without claiming public mint availability.
RO:INTERACTS — Anchor.toml, external private testnet config, external authority material, scripts/check_actual_test_only_mint_initialization.sh.
RO:INVARIANTS — devnet/testnet only; test-only labels; tiny caps; separated authorities; redacted receipts; no public/mainnet/production/real ROC claim.
RO:SECURITY — no silent wallet usage, no default live submission, no committed keys, no raw provider tokens, no public mint/burn, no bridge settlement, no internal ROC mutation.
RO:TEST — bash scripts/check_actual_test_only_mint_initialization.sh --check-docs . and cargo test -p rox-anchor-cli --test actual_test_only_mint_initialization.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 4
Actual Test-Only ROX Mint and Program Config Initialization
```

This phase may describe a manual private devnet/testnet initialization attempt.

The repo patch itself does not initialize a mint.

The checker does not initialize a mint.

The tests do not initialize a mint.

The CLI default behavior remains read-only or simulation-only.

## Required pre-manual gate

Before any manual test-only initialization attempt, run:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/capture_actual_private_testnet_build_artifacts.sh --check-docs .
bash scripts/check_actual_private_testnet_deploy_receipt.sh --preflight . testnet
bash scripts/check_actual_test_only_mint_initialization.sh --preflight . testnet
cargo test -p rox-anchor-cli --test actual_test_only_mint_initialization
cargo test -p rox-anchor-rpc-proof --test actual_test_only_mint_readback
cargo test --workspace
cargo check --workspace
anchor build
```

The preflight gate is read-only.

It does not call RPC.

It does not load a wallet.

It does not sign.

It does not initialize a mint.

It does not mint.

It does not burn.

It does not settle.

It does not mutate internal ROC.

## Manual command shape

Only after all gates are green and external keys/config exist:

```bash
cargo run -p rox-anchor-cli -- pilot initialize-test-only-mint \
  --config /external/private/<redacted-private-testnet-config> \
  --receipt-out /external/private/<redacted-receipts-dir>/init-mint.pilot-receipt.json \
  --operator-approval "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT"
```

The external config path must not be committed.

The receipt output path must not be committed.

The payer, mint authority, halt authority, recovery authority, and upgrade authority must be external or ignored.

## Required initialization receipt

After a manual initialization attempt, create one of:

```text
.rox-anchor-private-pilot/actual-test-only-mint-init-receipt.local.json
.rox-anchor-private-pilot/actual-test-only-mint-init-failed.local.json
```

Then validate it:

```bash
bash scripts/check_actual_test_only_mint_initialization.sh --check-init-receipt .rox-anchor-private-pilot/actual-test-only-mint-init-receipt.local.json
```

or:

```bash
bash scripts/check_actual_test_only_mint_initialization.sh --check-init-receipt .rox-anchor-private-pilot/actual-test-only-mint-init-failed.local.json
```

A successful initialization receipt is not public mint availability.

A failed initialization receipt is useful if it is redacted and fails safely.

## Required readback receipt

After successful initialization, create a read-only RPC readback receipt:

```text
.rox-anchor-private-pilot/actual-test-only-mint-readback.local.json
```

Then validate it:

```bash
bash scripts/check_actual_test_only_mint_initialization.sh --check-readback-receipt .rox-anchor-private-pilot/actual-test-only-mint-readback.local.json
```

Readback validation is separate from initialization attempt evidence.

## Initialization receipt schema

```json
{
  "schema": "rox-anchor.actual-test-only-mint-initialization.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_initialization_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "initialization_outcome": "succeeded",
  "operation_id": "actual-test-only-init-0001",
  "idempotency_key": "actual-test-only-init-idem-0001",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "program_config_account": "<redacted-program-config-account>",
  "max_supply_units": "1000",
  "max_amount_units_per_operation": "1",
  "mint_authority_redacted": "<redacted-external-mint-authority>",
  "halt_authority_redacted": "<redacted-external-halt-authority>",
  "recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "init_signature": "<redacted-signature>",
  "init_slot": "0",
  "failure_reason_redacted": "not_applicable",
  "operator_approval": "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT",
  "manual_operator_action": true,
  "preflight_passed": true,
  "readback_required": true,
  "readback_verified": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Failed initialization receipt schema

```json
{
  "schema": "rox-anchor.actual-test-only-mint-initialization.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_initialization_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "initialization_outcome": "failed",
  "operation_id": "actual-test-only-init-0001",
  "idempotency_key": "actual-test-only-init-idem-0001",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "program_config_account": "<redacted-program-config-account>",
  "max_supply_units": "1000",
  "max_amount_units_per_operation": "1",
  "mint_authority_redacted": "<redacted-external-mint-authority>",
  "halt_authority_redacted": "<redacted-external-halt-authority>",
  "recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "init_signature": "none",
  "init_slot": "none",
  "failure_reason_redacted": "<redacted-safe-failure-reason>",
  "operator_approval": "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT",
  "manual_operator_action": true,
  "preflight_passed": true,
  "readback_required": false,
  "readback_verified": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Readback receipt schema

```json
{
  "schema": "rox-anchor.actual-test-only-mint-readback.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_readback_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "readback_outcome": "verified",
  "readback_slot": "0",
  "program_config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "observed_test_only_mint_label": "test-only-rox-private-testnet",
  "observed_token_account_label": "test-only-rox-token-account-private-testnet",
  "observed_max_supply_units": "1000",
  "observed_max_amount_units_per_operation": "1",
  "observed_mint_authority_redacted": "<redacted-external-mint-authority>",
  "observed_halt_authority_redacted": "<redacted-external-halt-authority>",
  "observed_recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## What this may prove

A valid Phase 4 initialization receipt may prove:

```text
a private devnet/testnet test-only initialization attempt occurred
the attempt used test-only labels
the configured caps were tiny
the authorities were represented as separated external authorities
the result was succeeded or failed
the receipt was redacted
```

A valid Phase 4 readback receipt may prove:

```text
read-only RPC observed the deployed account shape
the observed labels remained test-only
the observed caps remained tiny
the observed authority references were redacted
```

## What this does not prove

A valid Phase 4 receipt does not prove:

```text
public mint availability
public ROX minting
public ROX burning
production bridge settlement
real internal ROC mutation
mainnet readiness
public launch readiness
exchange readiness
staking readiness
liquidity readiness
final settlement
```

Those require later plans and gates.

## Forbidden

```text
mainnet-beta
public mint labels
production mint labels
public launch claims
public ROX mint/burn claims
production bridge settlement
real internal ROC release
real user funds
unredacted payer path
unredacted authority path
raw RPC provider token
committed initialization receipt
fake finality
fake success output
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
