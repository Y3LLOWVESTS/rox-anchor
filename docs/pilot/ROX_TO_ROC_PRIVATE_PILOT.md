# Private ROX-to-ROC Testnet Pilot

This document records the BUILD_PLAN3 Phase 13 reverse private pilot boundary.

The reverse pilot accepts only test ROX burn evidence and produces only a dry-run internal ROC release intent.

It does not release real ROC.

It does not call `svc-wallet`.

It does not mutate `ron-ledger`.

It does not unlock paid content.

It does not claim final settlement.

## Required gates

The reverse pilot must keep these gates in order:

1. Observe test ROX burn evidence.
2. Require explicit test operation ID.
3. Require explicit idempotency key.
4. Require nonce.
5. Review proof package.
6. Review read-only RPC evidence.
7. Run coordinator decision.
8. Run relayer dry-run.
9. Run simulation where applicable.
10. Produce internal ROC release-intent only.
11. Verify burn/readback by read-only RPC.
12. Persist receipt when pilot execution artifacts are used.

## Non-goals

The reverse pilot does not authorize:

```text
real internal ROC release
direct ron-ledger mutation
direct svc-wallet mutation
paid content unlock
mainnet deployment
public bridge UI
production settlement
exchange-facing behavior
staking
liquidity
```

Any future real ROC release path must be delegated to the RustyOnions wallet and ledger boundary:

```text
svc-wallet -> ron-ledger
```

ROX Anchor must remain evidence, coordination, simulation, and on-chain Anchor state machinery. It must not become a direct internal ROC issuer.
