# Test-Only Mint Initialization Runbook

This is the BUILD_PLAN3 Phase 5 private pilot runbook for test-only ROX mint initialization.

This is **not a launch**.

This is **not public ROX availability**.

This is **not bridge settlement**.

This is **no real internal ROC mutation**.

This is **no live mint initialization**.

This runbook defines the safe shape for initialization intent review before any later private testnet action.

## Required constraints

Every initialization intent must require:

```text
explicit testnet mode
explicit test-only mint label
explicit test-only token account label
tiny supply cap
mint authority separation
halt authority
recovery authority
external operator files only
redacted reports only
```

No command in this runbook may load a wallet, call RPC, initialize a live mint, mint tokens, burn tokens, submit a transaction, settle a bridge operation, or mutate internal ROC.

## Safe local checks

Run:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
cargo test -p rox-anchor-core --test test_only_asset_harness
cargo test -p rox-anchor-core --test test_only_mint_initialization
cargo test -p rox-anchor-cli --test test_only_mint_initialization
cargo test -p rox-anchor
anchor build
anchor test
```

## Initialization intent report

A redacted test-only mint initialization report may include:

```text
test-only initialization label
requested initial supply units
maximum initial supply units
test-only mint label
test-only token account label
redacted authority key IDs
safety environment mode
safety submission mode
findings
```

A report must keep these disabled:

```text
live_mint_initialization: disabled
wallet_loading: disabled
rpc_calls: disabled
internal_roc_mutation: disabled
```

## Authority requirements

The intent must include separated or explicitly reviewed authorities for:

```text
upgrade authority
mint authority
halt authority
recovery authority
```

Strict mode must reject one shared key owning every critical authority.

## Asset requirements

The mint fixture must be test-only.

The token account fixture must be test-only.

The token account must bind to the same mint.

The requested initial supply must be above zero and no higher than the tiny cap.

## Forbidden

```text
public labels
production labels
real user funds
real ROC release
bridge settlement
live wallet loading
live RPC submission
live mint initialization
committed key material
committed deployment output
fake finality
fake success output
```

## Exit condition

This phase is complete when:

```text
the core initialization intent review is tested
the runbook is tested
the Anchor program still builds
the workspace remains green
the codebundle remains secret-safe
```
