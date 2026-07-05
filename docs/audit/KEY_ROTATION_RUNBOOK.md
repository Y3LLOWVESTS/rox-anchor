# ROX Anchor Phase 14 — Key Rotation Runbook

No public launch authorization.

This runbook describes local/testnet-only key rotation procedure for audit review. It does not authorize live mainnet operation, public ROX minting, public ROX burning, production bridge settlement, production ROC release, staking, liquidity, exchange-facing behavior, or public bridge access.

## Rotation scope

The authority model separates these roles:

```text
observer
coordinator
relayer
upgrade authority
mint authority
halt authority
recovery authority
```

Rotation is modeled and tested by:

```text
crates/rox-anchor-core/tests/operator_authority_model.rs
crates/rox-anchor-coordinator/tests/authority_readiness.rs
crates/rox-anchor-relayer/tests/authority_readiness.rs
programs/rox-anchor/src/state.rs
```

## Pre-rotation checks

Before rotating any testnet/localnet authority:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
bash scripts/check_audit_prep.sh .
bash scripts/check_testnet_deploy_drill.sh .
```

If Anchor tooling is available:

```bash
anchor build
anchor test
```

## Required rotation invariants

| Invariant | Test coverage |
| --- | --- |
| Rotation cannot be a no-op. | `authority_rotation_intent_rejects_noop_and_requires_activation_slot` |
| Rotation intent reports only redacted key IDs. | `authority_rotation_intent_reports_only_redacted_key_ids` |
| Wrong authority is rejected with redacted output. | `wrong_authority_rejection_uses_redacted_key_ids` |
| One key cannot own all critical roles in strict mode. | `strict_mode_rejects_one_key_owning_every_critical_authority` |
| Explicit test-only shared authority remains limited to drill mode. | `explicit_test_only_mode_allows_shared_critical_authority_for_drills` |
| Coordinator readiness rejects missing/unsafe authority model. | `authority_aware_coordinator_readiness_*` |
| Relayer readiness rejects missing/unsafe authority model. | `authority_aware_relayer_readiness_*` |
| Program config rejects wrong halt/recovery authority. | `wrong_authority_cannot_halt_or_recover_config` |

## Safe rotation process

1. Confirm current role map is complete and separated.
2. Confirm the replacement key is external to the repo.
3. Confirm no keypair JSON, payer file, authority file, wallet file, seed, mnemonic, or RPC secret is committed.
4. Create an explicit rotation intent for exactly one role or planned role set.
5. Require a future activation slot or explicit activation marker.
6. Re-run authority readiness checks.
7. Re-run halt/recovery drill checks.
8. Re-run testnet deployment drill checks.
9. Preserve redacted audit receipt/report output.
10. Do not claim public finality, production settlement, or public launch completion.

## Emergency rotation posture

If a testnet/localnet key is suspected compromised:

```text
1. Halt first.
2. Stop capped submit mode.
3. Preserve local receipts and audit reports.
4. Rotate halt/recovery authority first if the safety authority is affected.
5. Rotate relayer/coordinator authority before resuming simulation or capped testnet submit.
6. Re-run read-only RPC proof checks.
7. Re-run kill-switch drill command.
8. Re-run full workspace tests before resuming private testnet drills.
```

## Forbidden during rotation

```text
mainnet-beta deployment
production bridge settlement
production ROC release
public ROX minting
public ROX burning
public bridge UI enablement
silent wallet/key loading
silent RPC submission
exchange-facing behavior
staking
liquidity
fake finality
fake success output
```

## Audit note

A rotation runbook is not a rotation execution. It is a tested safety model and operator procedure. Real key use remains external, explicit, and testnet/localnet-only under this plan.
