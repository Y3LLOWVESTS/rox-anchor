# ROC to ROX Private Pilot

This BUILD_PLAN3 Phase 12 surface models the forward ROC-to-ROX private pilot flow.

Allowed:

```text
test-only CrabLink/internal ROC burn-intent input
deterministic proof review
read-only RPC quorum/readback
coordinator decision
relayer dry-run
simulation-only transaction plan
explicit capped private testnet authorization report
receipt-backed pilot inspection
test-only ROX mint target labels
```

Forbidden:

```text
real internal ROC burn
svc-wallet call
ron-ledger mutation
paid content unlock
public ROX mint
public bridge UI
mainnet-beta
wallet/key loading by default
signing by default
silent RPC submission
production settlement
CrabLink final-settlement display before backend proof
```

The forward pilot is only a private testnet/test-only shape. It does not authorize public minting, production bridge settlement, or real internal ROC mutation.
