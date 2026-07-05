# ROX Anchor Phase 14 — Known Non-Goals

No public launch authorization.

This document records explicit non-goals for audit review.

## Not authorized by this repo state

```text
mainnet-beta deployment
public Solana token launch
public ROX minting
public ROX burning
production bridge settlement
production ROC release
exchange-facing behavior
staking
liquidity pools
public faucet
public claim page
public bridge UI
live value movement with real user funds
silent live RPC submission
silent wallet/key usage
unbounded relayer retries
operator key material committed to repo
fake finality
fake success output
```

## Allowed scope

```text
localnet tests
devnet/testnet experiments
test-only ROX mint fixtures
test-only token account fixtures
test-only Anchor deployment drills
read-only live RPC checks
transaction simulation
strictly capped testnet submission after explicit gates
operator safety drills
key rotation drills
upgrade authority drills
halt/recovery drills
monitoring and alerting prototypes
security tests
chaos tests
audit prep
documentation of invariants proven by tests
```

## Audit interpretation

If future work needs mainnet, public ROX mint/burn, public bridge UI, production settlement, production ROC release, exchange-facing behavior, staking, or liquidity, it must become a separate build plan with new tests and explicit authorization.
