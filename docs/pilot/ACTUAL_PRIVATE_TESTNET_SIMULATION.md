# Actual Private Testnet Simulation

RO:WHAT — Defines BUILD_PLAN4 Phase 6 simulation receipts against actual private devnet/testnet deployed addresses.
RO:WHY — Proves simulation is gated by read-only evidence, proof review, coordinator decision, and relayer dry-run before any capped send.
RO:INTERACTS — external private testnet config, read-only RPC evidence receipts, proof/coordinator/relayer decisions, scripts/check_actual_private_testnet_simulation.sh.
RO:INVARIANTS — devnet/testnet only; simulate-only; test-only labels; tiny caps; redacted account references; no signing, no send, no finality, no real ROC mutation.
RO:SECURITY — no silent wallet usage, no RPC in tests, no committed keys/provider tokens, no transaction submission, no public mint/burn, no bridge settlement.
RO:TEST — bash scripts/check_actual_private_testnet_simulation.sh --check-docs . and cargo test -p rox-anchor-relayer --test actual_private_testnet_simulation.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 6
Simulation Against Actual Deployed Testnet Addresses
```

This phase may describe manual transaction simulation against actual private devnet/testnet addresses.

The repo patch itself does not call RPC.

The checker does not call RPC.

The tests do not call RPC.

The repo patch does not simulate live transactions.

The checker does not simulate live transactions.

The tests do not simulate live transactions.

The CLI default behavior remains read-only or simulation-only.

## Required pre-manual gate

Before any manual simulation against actual deployed addresses, run:

```bash
cargo fmt --all
bash scripts/check_private_pilot_hygiene.sh .
bash scripts/check_actual_private_testnet_workspace.sh .
bash scripts/check_actual_private_testnet_read_only_evidence.sh --preflight . testnet
bash scripts/check_actual_private_testnet_simulation.sh --preflight . testnet
cargo test -p rox-anchor-relayer --test actual_private_testnet_simulation
cargo test -p rox-anchor-coordinator --test actual_private_testnet_simulation_gate
cargo test -p rox-anchor-cli --test actual_private_testnet_simulation_command
cargo test --workspace
cargo check --workspace
anchor build
```

The preflight gate is local-file/readiness only.

It does not call RPC.

It does not load a wallet.

It does not sign.

It does not submit.

It does not mint.

It does not burn.

It does not settle.

It does not mutate internal ROC.

## Manual command shape

Only after all gates are green and external config/evidence exists:

```bash
cargo run -p rox-anchor-cli -- pilot simulate \
  --config /external/private/<redacted-private-testnet-config> \
  --receipt-out /external/private/<redacted-receipts-dir>/simulation.pilot-receipt.json \
  --simulate-only
```

The config path must remain external or ignored.

The receipt output path must remain external or ignored.

The simulation must be explicitly `--simulate-only`.

The simulation receipt must not be promotable into a send receipt.

## Required receipt

After a manual simulation attempt, create one of:

```text
.rox-anchor-private-pilot/actual-private-testnet-simulation-receipt.local.json
.rox-anchor-private-pilot/actual-private-testnet-simulation-failed.local.json
```

Then validate it:

```bash
bash scripts/check_actual_private_testnet_simulation.sh --check-simulation-receipt .rox-anchor-private-pilot/actual-private-testnet-simulation-receipt.local.json
```

or:

```bash
bash scripts/check_actual_private_testnet_simulation.sh --check-simulation-receipt .rox-anchor-private-pilot/actual-private-testnet-simulation-failed.local.json
```

Simulation evidence is not transaction submission.

Simulation evidence is not finality.

Simulation evidence is not settlement.

Simulation evidence is not public mint availability.

## Successful simulation receipt schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-simulation.v1",
  "phase": "BUILD_PLAN4 Phase 6",
  "receipt_role": "actual_private_testnet_simulation_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "simulation_outcome": "simulated",
  "operation_id": "actual-simulation-op-0001",
  "idempotency_key": "actual-simulation-idem-0001",
  "nonce": "actual-simulation-nonce-0001",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "read_only_evidence_status": "verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_dry_run_status": "accepted",
  "simulation_result": "passed",
  "simulation_log_redacted": "<redacted-simulation-log>",
  "read_only_evidence_required": true,
  "read_only_evidence_verified": true,
  "simulate_only": true,
  "transaction_submission": false,
  "send_authorized": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "receipt_promotable_to_send": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Blocked simulation receipt schema

```json
{
  "schema": "rox-anchor.actual-private-testnet-simulation.v1",
  "phase": "BUILD_PLAN4 Phase 6",
  "receipt_role": "actual_private_testnet_simulation_receipt",
  "cluster": "testnet",
  "direction": "roc_to_rox",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "simulation_outcome": "blocked",
  "operation_id": "actual-simulation-op-0001",
  "idempotency_key": "actual-simulation-idem-0001",
  "nonce": "actual-simulation-nonce-0001",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "read_only_evidence_status": "missing",
  "proof_review_status": "not_run",
  "coordinator_decision_status": "not_run",
  "relayer_dry_run_status": "not_run",
  "simulation_result": "not_run",
  "failure_reason_redacted": "<redacted-safe-simulation-blocker>",
  "simulation_log_redacted": "<redacted-simulation-log>",
  "read_only_evidence_required": true,
  "read_only_evidence_verified": false,
  "simulate_only": true,
  "transaction_submission": false,
  "send_authorized": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "receipt_promotable_to_send": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
```

## Accepted directions

```text
roc_to_rox
rox_to_roc
```

## Accepted simulation outcomes

```text
simulated
failed
blocked
```

Only `simulated` may be treated as a successful simulation receipt.

`failed` and `blocked` remain non-sendable evidence.

## Required gates for successful simulation

A successful simulation receipt must show:

```text
read_only_evidence_status = verified
proof_review_status = accepted
coordinator_decision_status = accepted
relayer_dry_run_status = accepted
simulation_result = passed
simulate_only = true
transaction_submission = false
send_authorized = false
receipt_promotable_to_send = false
```

## What this may prove

A valid Phase 6 simulation receipt may prove:

```text
the operator prepared a simulation plan against actual private devnet/testnet addresses
the plan used deployed program/config/mint/token-account bindings
the plan used test-only labels
the amount was tiny and capped
read-only evidence, proof review, coordinator decision, and relayer dry-run gates were recorded
the result was simulated, failed, or blocked
```

## What this does not prove

A valid Phase 6 simulation receipt does not prove:

```text
transaction submission
signature generation
wallet loading
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
committed simulation receipt
send-authorized simulation receipts
promotable simulation receipts
fake finality
fake success output
staking
liquidity
exchange-facing behavior
```

## Non-authorization lock

No transaction submission.

No wallet loading.

No signature generation.

No public launch authorization.

No mainnet-beta authorization.

No production bridge settlement.

No public ROX mint/burn.

No real internal ROC release.

No staking.

No liquidity.

No exchange-facing behavior.

No fake finality.
