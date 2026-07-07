# Actual ROC-to-ROX Private Testnet Run

RO:WHAT — Defines BUILD_PLAN4 Phase 7 capped private testnet ROC-to-ROX test-only send/readback evidence.
RO:WHY — Captures the forward bridge shape from shadow ROC burn intent to test-only ROX evidence without burning real ROC.
RO:INTERACTS — external private testnet config, Phase 5 read-only evidence, Phase 6 simulation receipts, capped sender, readback receipt checker.
RO:INVARIANTS — devnet/testnet only; shadow ROC burn intent only; explicit approval; tiny caps; test-only ROX; readback required; no public launch, no mainnet, no real ROC mutation.
RO:SECURITY — external signer only, redacted paths, no committed keys/provider tokens, no silent submission, no production settlement, no public mint/burn.
RO:TEST — bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --check-docs . and cargo test -p rox-anchor-relayer --test actual_roc_to_rox_capped_send.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 7
Actual Capped Testnet ROC-to-ROX Flow
```

This phase may describe one manually approved capped private devnet/testnet transaction.

The repo patch itself does not call RPC.

The checker does not call RPC.

The tests do not call RPC.

The repo patch does not submit transactions.

The checker does not submit transactions.

The tests do not submit transactions.

A successful Phase 7 receipt is only valid for a test-only private devnet/testnet run.

## Forward flow shape

```text
shadow internal ROC burn intent
→ proof review
→ read-only RPC evidence
→ coordinator decision
→ relayer dry-run
→ simulation
→ explicit capped private testnet send
→ test-only ROX mint/finalize evidence
→ read-only RPC readback
→ receipt
```

The shadow ROC burn intent is not a real ROC burn.

ROX Anchor must not mutate real internal ROC.

Any future real ROC mutation must go through:

```text
svc-wallet -> ron-ledger
```

## Required pre-manual gate

Before any manual capped ROC-to-ROX testnet send, run:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/check_actual_private_testnet_read_only_evidence.sh --preflight . testnet
bash scripts/check_actual_private_testnet_simulation.sh --preflight . testnet
bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --preflight . testnet
cargo test -p rox-anchor-relayer --test actual_roc_to_rox_capped_send
cargo test -p rox-anchor-rpc-proof --test actual_roc_to_rox_readback
cargo test -p rox-anchor-coordinator --test actual_roc_to_rox_decision
cargo test -p rox-anchor-cli --test actual_roc_to_rox_command
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

It does not mutate internal ROC.

## Manual command shape

Only after all gates are green and external config/evidence exists:

```bash
cargo run -p rox-anchor-cli -- pilot roc-to-rox \
  --config /external/private/<redacted-private-testnet-config> \
  --receipt-out /external/private/<redacted-receipts-dir>/roc-to-rox-send.pilot-receipt.json \
  --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND" \
  --max-operations 1 \
  --max-amount-minor 1
```

The config path must remain external or ignored.

The receipt output path must remain external or ignored.

The signer path must remain external or ignored.

The provider URL/token must remain external or ignored.

## Required send receipt

After a manual capped send attempt, create one of:

```text
.rox-anchor-private-pilot/actual-roc-to-rox-capped-send-receipt.local.json
.rox-anchor-private-pilot/actual-roc-to-rox-capped-send-failed.local.json
```

Then validate it:

```bash
bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --check-send-receipt .rox-anchor-private-pilot/actual-roc-to-rox-capped-send-receipt.local.json
```

or:

```bash
bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --check-send-receipt .rox-anchor-private-pilot/actual-roc-to-rox-capped-send-failed.local.json
```

## Required readback receipt

After a successful capped send, create:

```text
.rox-anchor-private-pilot/actual-roc-to-rox-readback.local.json
```

Then validate it:

```bash
bash scripts/check_actual_roc_to_rox_private_testnet_run.sh --check-readback-receipt .rox-anchor-private-pilot/actual-roc-to-rox-readback.local.json
```

Readback is required before any operator report may call the test-only run readback-verified.

## Successful capped send receipt schema

```json
{
  "schema": "rox-anchor.actual-roc-to-rox-capped-send.v1",
  "phase": "BUILD_PLAN4 Phase 7",
  "receipt_role": "actual_roc_to_rox_capped_send_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "send_outcome": "sent",
  "operation_id": "actual-roc-to-rox-op-0001",
  "idempotency_key": "actual-roc-to-rox-idem-0001",
  "nonce": "actual-roc-to-rox-nonce-0001",
  "shadow_roc_burn_intent_id": "shadow-roc-burn-intent-0001",
  "shadow_roc_burn_only": true,
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "retry_cap": "1",
  "read_only_evidence_status": "verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_dry_run_status": "accepted",
  "simulation_result": "passed",
  "operator_approval": "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
  "external_signer_used": true,
  "signer_path_redacted": "<redacted-external-signer-path>",
  "receipt_out_redacted": "<redacted-external-receipt-path>",
  "transaction_submission": true,
  "send_authorized": true,
  "signature_generated": true,
  "transaction_signature": "<redacted-testnet-signature>",
  "send_slot": "0",
  "test_only_rox_delta_minor": "1",
  "readback_required": true,
  "readback_verified": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Blocked capped send receipt schema

```json
{
  "schema": "rox-anchor.actual-roc-to-rox-capped-send.v1",
  "phase": "BUILD_PLAN4 Phase 7",
  "receipt_role": "actual_roc_to_rox_capped_send_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "send_outcome": "blocked",
  "operation_id": "actual-roc-to-rox-op-0001",
  "idempotency_key": "actual-roc-to-rox-idem-0001",
  "nonce": "actual-roc-to-rox-nonce-0001",
  "shadow_roc_burn_intent_id": "shadow-roc-burn-intent-0001",
  "shadow_roc_burn_only": true,
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "retry_cap": "1",
  "read_only_evidence_status": "verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_dry_run_status": "accepted",
  "simulation_result": "blocked",
  "failure_reason_redacted": "<redacted-safe-capped-send-blocker>",
  "operator_approval": "missing",
  "external_signer_used": false,
  "signer_path_redacted": "<redacted-external-signer-path>",
  "receipt_out_redacted": "<redacted-external-receipt-path>",
  "transaction_submission": false,
  "send_authorized": false,
  "signature_generated": false,
  "transaction_signature": "none",
  "send_slot": "none",
  "test_only_rox_delta_minor": "0",
  "readback_required": false,
  "readback_verified": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Readback receipt schema

```json
{
  "schema": "rox-anchor.actual-roc-to-rox-readback.v1",
  "phase": "BUILD_PLAN4 Phase 7",
  "receipt_role": "actual_roc_to_rox_readback_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "readback_outcome": "verified",
  "operation_id": "actual-roc-to-rox-op-0001",
  "idempotency_key": "actual-roc-to-rox-idem-0001",
  "nonce": "actual-roc-to-rox-nonce-0001",
  "transaction_signature": "<redacted-testnet-signature>",
  "send_receipt_id": "<redacted-send-receipt-id>",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "expected_test_only_rox_delta_minor": "1",
  "observed_test_only_rox_delta_minor": "1",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Accepted send outcomes

```text
sent
blocked
failed
```

Only `sent` may claim transaction submission, send authorization, signature generation, and external signer use.

`blocked` and `failed` must remain non-submitting evidence.

## Required gates for successful sent receipt

A successful capped send receipt must show:

```text
read_only_evidence_status = verified
proof_review_status = accepted
coordinator_decision_status = accepted
relayer_dry_run_status = accepted
simulation_result = passed
operator_approval = I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND
shadow_roc_burn_only = true
transaction_submission = true
send_authorized = true
external_signer_used = true
signature_generated = true
readback_required = true
readback_verified = false
```

## What this may prove

A valid Phase 7 sent receipt may prove:

```text
the operator manually approved one capped private devnet/testnet test transaction
the flow used a shadow ROC burn intent only
the flow used test-only ROX labels
the amount and operation count were tiny and capped
the required gates were recorded as accepted
the transaction signature and signer path were redacted
readback is required before claiming readback verification
```

A valid Phase 7 readback receipt may prove:

```text
read-only RPC observed the expected test-only ROX delta
the observed delta matched the expected delta
the evidence remained read-only and redacted
```

## What this does not prove

A valid Phase 7 receipt does not prove:

```text
real ROC burn
real internal ROC mutation
public ROX minting
public bridge settlement
production settlement
public launch readiness
mainnet readiness
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
real internal ROC burn
real internal ROC mutation
real user funds
unredacted RPC URL
unredacted provider token
unredacted payer path
unredacted authority path
committed capped-send receipt
fake finality
fake success output
staking
liquidity
exchange-facing behavior
```

## Non-authorization lock

No real ROC burn.

No real internal ROC mutation.

No public launch authorization.

No mainnet-beta authorization.

No production bridge settlement.

No public ROX mint/burn.

No staking.

No liquidity.

No exchange-facing behavior.

No fake finality.
