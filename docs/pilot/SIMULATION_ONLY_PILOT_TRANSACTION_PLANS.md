# Simulation-Only Pilot Transaction Plans

This is the BUILD_PLAN3 Phase 7 runbook for simulation-only private pilot transaction plans.

This is **simulation-only**.

This is a **private pilot** phase.

This requires **accepted proof review**.

This requires **accepted coordinator decision**.

This requires **relayer dry-run acceptance**.

This requires **read-only RPC verification** before simulation.

This is **no transaction submission**.

This is **no wallet loading**.

This is **no internal ROC mutation**.

This is **no live mint**.

This is **no live burn**.

## Purpose

The private pilot needs transaction-shaped plans before any send-capable path exists.

The simulation-only plan may describe these local instruction groups:

```text
initialize
observe
open challenge
resolve challenge
halt
recover
finalize
```

The plan must remain local and non-submitting.

## Required gates

A simulation-only pilot plan is allowed only after:

```text
proof review is accepted
coordinator decision is accepted
relayer dry-run receipt is accepted
read-only RPC verification has passed
instruction plan is non-empty
instruction count matches the declared local plan
safety scope is non-submitting
```

## Local test mode

Automated tests use local proof fixtures and local relayer dry-run receipts.

They do not call live RPC.

They do not load keys.

They do not sign.

They do not send.

They do not initialize live state.

They do not mint tokens.

They do not burn tokens.

They do not settle bridge operations.

They do not mutate internal ROC.

## Required local checks

Run:

```bash
cargo fmt -p rox-anchor-relayer -p rox-anchor-cli
cargo test -p rox-anchor-relayer --test private_pilot_simulation
cargo test -p rox-anchor-relayer --test transaction_simulation
cargo test -p rox-anchor-cli --test private_pilot_simulation
cargo test --workspace
```

## Exit condition

This phase is complete when:

```text
simulation accepts only after read-only RPC verification
simulation accepts only after proof/coordinator/relayer gates
missing read-only verification fails closed
missing instruction steps fail closed
instruction-count mismatch fails closed
blocked proof fails closed
unsafe submitting scope fails closed
workspace tests stay green
codebundle remains secret-safe
```
