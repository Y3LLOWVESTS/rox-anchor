# Testnet Program Artifact Manifest

This document describes the BUILD_PLAN3 Phase 3 manifest boundary.

The manifest is a **non-secret, operator-readable metadata shape** for private devnet/testnet program artifacts. It may record:

```text
cluster
program_id
expected_program_id
build_hash
idl_hash
deploy_slot, if supplied
operator_label
artifact_label
redacted program artifact path
redacted IDL artifact path
```

The manifest is **not** a deployment proof.

The manifest is **not** production finality.

The manifest is **not** public launch authorization.

The manifest must not contain keypairs, wallet material, RPC provider tokens, unredacted local paths, private keys, seed phrases, or receipt secrets.

Required safety rules:

```text
mainnet-beta is rejected
localnet is rejected for private pilot artifact manifests
empty program IDs are rejected
program ID must match the expected Anchor.toml devnet/testnet binding
public or production labels are rejected
local artifact paths are redacted in reports
```

Current expected private pilot binding:

```text
FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk
```

Allowed use:

```text
private devnet/testnet operator review
redacted CLI status display
artifact bookkeeping before a deployment drill
audit trail preparation
```

Forbidden use:

```text
mainnet deployment claim
public ROX launch claim
production bridge settlement claim
real ROC release claim
wallet/key loading
RPC submission
mint/burn execution
staking/liquidity/exchange-facing behavior
```
