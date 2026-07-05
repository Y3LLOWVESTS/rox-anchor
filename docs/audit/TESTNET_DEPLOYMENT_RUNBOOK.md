# ROX Anchor Phase 14 — Testnet Deployment and Rollback Runbook

No public launch authorization.

This runbook documents testnet/localnet deployment drill boundaries for audit review. It does not authorize mainnet-beta deployment, public token launch, public bridge access, production ROC release, exchange-facing behavior, staking, liquidity, or production settlement.

## Preflight commands

Before any private testnet drill:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_testnet_deploy_drill.sh .
bash scripts/check_audit_prep.sh .
```

## Deployment drill guardrails

1. Use only localnet/devnet/testnet labels allowed by configuration.
2. Do not add a mainnet-beta section to `Anchor.toml`.
3. Keep payer, upgrade authority, deploy keypair, mint authority, halt authority, and recovery authority outside the repo.
4. Do not commit `.json` keypairs.
5. Do not treat testnet deployment as public launch.
6. Do not expose public mint/burn UI.
7. Do not connect production ROC release.
8. Do not create exchange, staking, or liquidity behavior.

## Rollback / stop procedure

For testnet/localnet drill failures:

```text
1. Stop relayer submit mode by returning to DryRunOnly or SimulateOnly.
2. Set operational posture to halted in local coordinator/relayer model.
3. Preserve receipts and audit reports.
4. Record RPC evidence source, slot, signature, and binding mismatch if present.
5. Re-run read-only RPC proof classification.
6. Re-run kill-switch drill command.
7. Do not submit a corrective transaction until capped testnet checks pass again.
8. Do not publish any public bridge/finality claim.
```

## Checked by

```text
scripts/check_testnet_deploy_drill.sh
crates/rox-anchor-cli/tests/testnet_deploy_drill_script.rs
scripts/check_audit_prep.sh
crates/rox-anchor-cli/tests/audit_prep_docs.rs
```

## Manual testnet-only note

Any optional testnet deployment command remains an operator drill only. It must be preceded by explicit operator action and external non-repo key paths.
