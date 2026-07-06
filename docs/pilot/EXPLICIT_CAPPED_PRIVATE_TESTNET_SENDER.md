# Explicit Capped Private Testnet Sender

This is the BUILD_PLAN3 Phase 8 runbook for the explicit capped private testnet sender.

This is an **explicit capped private testnet sender** authorization phase.

This requires **external config**.

This requires **operator approval**.

This requires a **receipt output path**.

This requires **successful simulation**.

This requires **read-only RPC verification** before simulation.

This is **no default send path**.

This is **no wallet loading**.

This is **no signing**.

This is **no internal ROC mutation**.

## Purpose

The private pilot now has a send-capable authorization model, but the local test and CLI surfaces still do not execute network submission.

The authorization model checks:

```text
external private pilot config exists
external config validates as testnet-only
submission mode is testnet-submit-capped
operator approval phrase is exact
receipt output path is declared
proof review was accepted
coordinator gate was accepted
relayer dry-run was accepted
simulation succeeded
read-only RPC verification happened before simulation
retry cap is respected
operation cap is respected
amount cap is respected
halt, challenge, and recovery blockers are clear
```

## Local test mode

Automated tests do not call live RPC.

Automated tests do not load keys.

Automated tests do not sign.

Automated tests do not send transactions.

Automated tests do not mint tokens.

Automated tests do not burn tokens.

Automated tests do not settle bridge operations.

Automated tests do not mutate internal ROC.

## CLI shape

The CLI exposes the private-pilot submit-shaped command group without making send the default behavior.

```bash
cargo run -p rox-anchor-cli -- submit
cargo run -p rox-anchor-cli -- submit capped-testnet --help
cargo run -p rox-anchor-cli -- submit capped-testnet \
  --authorize-testnet-submit-capped \
  --receipt-persisted
```

The CLI report remains local/report-only.

## Optional future operator shape

A future manual private testnet run must remain explicit and external-config-backed.

```bash
# cargo run -p rox-anchor-cli -- submit capped-testnet \
#   --config /external/path/to/private-testnet.toml \
#   --receipt-out /external/path/to/receipt.json \
#   --operator-approval "I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT"
```

## Exit condition

This phase is complete when:

```text
sender authorization requires external config
sender authorization requires exact operator approval
sender authorization requires receipt output path
sender authorization requires successful simulation
sender authorization rejects operational blockers
sender authorization surfaces capped limit failures
CLI has no default send path
workspace tests stay green
phase 8 clippy checkpoint passes for relayer and CLI
codebundle remains secret-safe
```
