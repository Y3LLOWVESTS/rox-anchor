# CrabLink / Internal ROC Dry-Run Adapter

This note documents the BUILD_PLAN3 private-pilot adapter shape for future CrabLink and RustyOnions handoff tests.

It is intentionally dry-run only.

Allowed in this surface:

```text
test-only ROC burn-intent input shape
test-only ROC release-intent output shape
display-safe CrabLink status labels
operation ID / idempotency key / nonce binding
coordinator observation of dry-run intent records
```

Forbidden in this surface:

```text
svc-wallet calls
ron-ledger mutation
real internal ROC burn
real internal ROC release
paid content unlock
public bridge UI
public ROX mint/burn
mainnet behavior
production settlement
CrabLink final-settlement display before backend proof
```

The future real internal ROC mutation path remains:

```text
svc-wallet -> ron-ledger
```

ROX Anchor must not directly issue, release, unlock, or mutate real internal ROC.
