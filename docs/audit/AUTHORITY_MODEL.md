# ROX Anchor Phase 14 — Authority Model

No public launch authorization.

This document summarizes ROX Anchor authority separation for audit review. It is descriptive only and does not authorize live keys, live RPC submission, deployment, minting, burning, settlement, staking, liquidity, or exchange-facing behavior.

## Authority roles

The core authority model separates the following roles:

```text
observer
coordinator
relayer
upgrade authority
mint authority
halt authority
recovery authority
```

The role model is tested in:

```text
crates/rox-anchor-core/tests/operator_authority_model.rs
crates/rox-anchor-coordinator/tests/authority_readiness.rs
crates/rox-anchor-relayer/tests/authority_readiness.rs
programs/rox-anchor/src/state.rs
```

## Separation invariants

| Invariant | Test coverage |
| --- | --- |
| Critical authorities are separated in strict mode. | `separated_critical_authorities_validate_in_strict_mode` |
| One key owning every critical authority is rejected unless explicitly test-only. | `strict_mode_rejects_one_key_owning_every_critical_authority`, `explicit_test_only_mode_allows_shared_critical_authority_for_drills` |
| Duplicate authority roles are rejected. | `duplicate_authority_roles_are_rejected` |
| Wrong authority reports are redacted. | `wrong_authority_rejection_uses_redacted_key_ids` |
| Authority rotation intent is explicit and non-noop. | `authority_rotation_intent_rejects_noop_and_requires_activation_slot`, `authority_rotation_intent_reports_only_redacted_key_ids` |
| Coordinator/relayer readiness rejects missing or unsafe authority shape. | `authority_aware_coordinator_readiness_*`, `authority_aware_relayer_readiness_*` |
| Program config rejects wrong halt/recovery authority. | `wrong_authority_cannot_halt_or_recover_config` |

## Operational rules

1. Authority configuration is model-checked before any testnet submission path.
2. Real keypairs are not committed to the repo.
3. CLI status and audit output must redact key identifiers.
4. Halt and recovery authorities must not be silently interchangeable.
5. Upgrade and mint authority handling remains testnet/localnet only until a later explicit plan exists.

## Non-goals

This authority model does not authorize public ROX control, public bridge operations, production ROC release, mainnet-beta deployment, exchange-facing integrations, staking, or liquidity.
