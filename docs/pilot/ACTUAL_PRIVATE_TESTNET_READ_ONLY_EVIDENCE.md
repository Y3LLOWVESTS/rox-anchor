# Actual Private Testnet Read-Only RPC Evidence

RO:WHAT — Defines BUILD_PLAN4 Phase 5 live read-only RPC evidence receipts against deployed private devnet/testnet accounts.
RO:WHY — Captures program/config/mint/token-account readback evidence without submission, signing, minting, burning, settlement, or real ROC mutation.
RO:INTERACTS — external private testnet config, deployed program ID, test-only mint/config receipts, scripts/check_actual_private_testnet_read_only_evidence.sh.
RO:INVARIANTS — devnet/testnet only; read-only RPC only; quorum-shaped evidence; redacted RPC/provider/account details; no public/mainnet/production/finality/real ROC claim.
RO:SECURITY — no silent wallet usage, no default live submission, no committed provider tokens, no private keys, no public mint/burn, no bridge settlement, no internal ROC mutation.
RO:TEST — bash scripts/check_actual_private_testnet_read_only_evidence.sh --check-docs . and cargo test -p rox-anchor-rpc-proof --test actual_private_testnet_read_only_rpc.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 5
Live Read-Only RPC Evidence Against Deployed Accounts
```

This phase may describe manual live read-only RPC checks.

The repo patch itself does not call RPC.

The checker does not call RPC.

The tests do not call RPC.

The CLI default behavior remains read-only or simulation-only.

## Required pre-manual gate

Before any manual live read-only RPC evidence capture, run:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/check_actual_private_testnet_deploy_receipt.sh --preflight . testnet
bash scripts/check_actual_test_only_mint_initialization.sh --preflight . testnet
bash scripts/check_actual_private_testnet_read_only_evidence.sh --preflight . testnet
cargo test -p rox-anchor-rpc-proof --test actual_private_testnet_read_only_rpc
cargo test -p rox-anchor-cli --test actual_private_testnet_read_only_command
cargo test --workspace
cargo check --workspace
anchor build
```

The preflight gate is local-file/readiness only.

It does not call RPC.

It does not load a wallet.

It does not sign.

It does not submit.

It does not initialize a mint.

It does not mint.

It does not burn.

It does not settle.

It does not mutate internal ROC.

## Manual command shape

Only after all gates are green and external config exists:

```bash
cargo run -p rox-anchor-cli -- pilot proof read-only \
  --config /external/private/<redacted-private-testnet-config> \
  --receipt-out /external/private/<redacted-receipts-dir>/read-only-evidence.pilot-receipt.json
```

The config path must remain external or ignored.

The receipt output path must remain external or ignored.

The RPC endpoint or provider token must not be committed.

The receipt must use redacted provider labels and redacted account references.

## Required receipt

After a manual read-only RPC evidence capture, create one of:

```text
.rox-anchor-private-pilot/actual-private-testnet-read-only-evidence.local.json
.rox-anchor-private-pilot/actual-private-testnet-read-only-evidence-failed.local.json
```

Then validate it:

```bash
bash scripts/check_actual_private_testnet_read_only_evidence.sh --check-evidence-receipt .rox-anchor-private-pilot/actual-private-testnet-read-only-evidence.local.json
```

or:

```bash
bash scripts/check_actual_private_testnet_read_only_evidence.sh --check-evidence-receipt .rox-anchor-private-pilot/actual-private-testnet-read-only-evidence-failed.local.json
```

Read-only evidence is not transaction submission.

Read-only evidence is not finality.

Read-only evidence is not settlement.

Read-only evidence is not public mint availability.

## Receipt schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-read-only-evidence.v1",
  "phase": "BUILD_PLAN4 Phase 5",
  "receipt_role": "private_testnet_read_only_rpc_evidence_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
  "evidence_outcome": "verified",
  "current_slot": "0",
  "program_account": "<redacted-program-account>",
  "program_account_status": "exists-executable",
  "program_account_slot": "0",
  "config_account": "<redacted-program-config-account>",
  "config_account_status": "exists",
  "config_account_slot": "0",
  "test_only_mint": "<redacted-test-only-mint>",
  "mint_account_status": "exists",
  "mint_account_slot": "0",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "token_account_status": "exists",
  "token_account_slot": "0",
  "deploy_signature_status": "confirmed",
  "initialization_signature_status": "confirmed",
  "rpc_sources_count": "2",
  "rpc_quorum_threshold": "2",
  "rpc_matching_sources_count": "2",
  "rpc_disputed_sources_count": "0",
  "max_observation_lag_slots": "150",
  "rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Failed or disputed receipt shape

```json
{
  "schema": "rox-anchor.actual-private-testnet-read-only-evidence.v1",
  "phase": "BUILD_PLAN4 Phase 5",
  "receipt_role": "private_testnet_read_only_rpc_evidence_receipt",
  "cluster": "testnet",
  "program_name": "rox_anchor",
  "program_id": "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
  "evidence_outcome": "failed",
  "current_slot": "0",
  "program_account": "<redacted-program-account>",
  "program_account_status": "missing",
  "program_account_slot": "0",
  "config_account": "<redacted-program-config-account>",
  "config_account_status": "not_checked",
  "config_account_slot": "0",
  "test_only_mint": "<redacted-test-only-mint>",
  "mint_account_status": "not_checked",
  "mint_account_slot": "0",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "token_account_status": "not_checked",
  "token_account_slot": "0",
  "deploy_signature_status": "not_checked",
  "initialization_signature_status": "not_checked",
  "rpc_sources_count": "2",
  "rpc_quorum_threshold": "2",
  "rpc_matching_sources_count": "0",
  "rpc_disputed_sources_count": "0",
  "max_observation_lag_slots": "150",
  "failure_reason_redacted": "<redacted-safe-read-only-failure-reason>",
  "rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Accepted evidence outcomes

```text
verified
failed
disputed
stale
missing
```

Only `verified` may be treated as passing read-only evidence.

All other outcomes must remain non-successful evidence.

## What this may prove

A valid Phase 5 read-only RPC evidence receipt may prove:

```text
the operator queried deployed private devnet/testnet accounts
the evidence was read-only
the observed program/config/mint/token-account statuses were recorded
the observed RPC provider labels were redacted
the observed quorum counts were recorded
the outcome was verified, failed, disputed, stale, or missing
```

## What this does not prove

A valid Phase 5 receipt does not prove:

```text
transaction submission
minting
burning
bridge settlement
real internal ROC mutation
public mint availability
public launch readiness
mainnet readiness
production readiness
exchange readiness
staking readiness
liquidity readiness
final settlement
```

Those require later phases and separate gates.

## Forbidden

```text
mainnet-beta
public launch claims
public ROX mint/burn claims
production bridge settlement
real internal ROC release
real user funds
unredacted RPC URL
unredacted provider token
unredacted payer path
unredacted authority path
committed read-only evidence receipt
fake finality
fake success output
staking
liquidity
exchange-facing behavior
```

## Non-authorization lock

No transaction submission.

No public launch authorization.

No mainnet-beta authorization.

No production bridge settlement.

No public ROX mint/burn.

No real internal ROC release.

No staking.

No liquidity.

No exchange-facing behavior.

No fake finality.
