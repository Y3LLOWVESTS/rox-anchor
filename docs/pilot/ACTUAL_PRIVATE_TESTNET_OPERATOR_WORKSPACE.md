# Actual Private Testnet Operator Workspace

RO:WHAT — Defines the BUILD_PLAN4 Phase 1 external operator workspace for actual private devnet/testnet evidence.
RO:WHY — Moves from private pilot readiness into real private testnet preparation without committing keys, RPC URLs, provider tokens, receipts, or local operator paths.
RO:INTERACTS — .gitignore, scripts/check_actual_private_testnet_workspace.sh, future build artifact/deploy/receipt evidence.
RO:INVARIANTS — external-only / ignored / redacted; no mainnet-beta; no public launch; no real ROC mutation; no fake finality.
RO:SECURITY — No wallet/key loading, RPC calls, deployment, submission, minting, burning, bridge settlement, staking, liquidity, or exchange-facing behavior.
RO:TEST — bash scripts/check_actual_private_testnet_workspace.sh . and cargo test -p rox-anchor-cli --test actual_private_testnet_workspace.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 1
External Operator Workspace Actualization
```

This phase prepares the operator workspace shape for actual private devnet/testnet work.

It does not authorize deployment.

It does not authorize transaction submission.

It does not authorize public ROX minting or burning.

It does not authorize production bridge settlement.

It does not authorize real internal ROC release.

It does not authorize mainnet-beta.

## Required external posture

Actual private testnet operator artifacts must remain:

```text
external-only / ignored / redacted
```

The preferred workspace is outside the repo:

```text
<external-private-workspace>/
  private-testnet.toml

  keys/
    testnet-payer.json
    rox-anchor-program-keypair.json
    mint-authority.json
    halt-authority.json
    recovery-authority.json
    upgrade-authority.json

  rpc/
    rpc-url.pilot-rpc.txt
    provider-token.pilot-provider.txt

  deploy/
    anchor-build-output.pilot-deploy-output.json
    program-artifact-manifest.pilot-deploy-output.json

  receipts/
    phase-02-build-artifacts.pilot-receipt.json
    phase-03-deploy-attempt.pilot-receipt.json
    phase-04-init-mint.pilot-receipt.json

  audit/
    redaction-review.pilot-audit.json

  ledger/
    pilot-local-ledger.pilot-ledger.json

  tmp/
    scratch files safe to delete
```

If the operator chooses to place a temporary workspace inside the repo, it must be under ignored paths such as:

```text
.rox-anchor-private-pilot/
.rox-anchor-pilot/
private-pilot/
pilot-artifacts/
pilot-rpc/
pilot-keys/
pilot-keypairs/
pilot-wallets/
pilot-secrets/
pilot-receipts/
pilot-audit/
pilot-deploy/
pilot-ledger/
pilot-tmp/
```

## External local config shape

The real local config must not be committed.

The safe source-controlled docs may show a redacted shape only:

```toml
# <external-private-workspace>/private-testnet.toml
# local-only; never commit

cluster = "testnet"
program_id = "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR"

payer_keypair_path = "<external-private-workspace>/keys/testnet-payer.json"
program_keypair_path = "<external-private-workspace>/keys/rox-anchor-program-keypair.json"
mint_authority_path = "<external-private-workspace>/keys/mint-authority.json"
halt_authority_path = "<external-private-workspace>/keys/halt-authority.json"
recovery_authority_path = "<external-private-workspace>/keys/recovery-authority.json"
upgrade_authority_path = "<external-private-workspace>/keys/upgrade-authority.json"

rpc_url_file = "<external-private-workspace>/rpc/rpc-url.pilot-rpc.txt"
provider_token_file = "<external-private-workspace>/rpc/provider-token.pilot-provider.txt"

receipt_dir = "<external-private-workspace>/receipts"
audit_dir = "<external-private-workspace>/audit"

max_test_only_amount_minor = "1000000"
max_operation_count = 1
max_retry_count = 0
require_operator_approval = true
```

Safe environment variable names:

```text
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_CONFIG
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_RECEIPT_DIR
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_AUDIT_DIR
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_RPC_URL_FILE
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_PROVIDER_TOKEN_FILE
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_PAYER
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_PROGRAM_KEYPAIR
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_MINT_AUTHORITY
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_HALT_AUTHORITY
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_RECOVERY_AUTHORITY
ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_UPGRADE_AUTHORITY
```

These environment variables must point to external or ignored local files.

They must not contain raw private keys, seed phrases, provider tokens, or tokenized RPC URLs directly.

## Redaction rules

Before any actual private testnet artifact is promoted into source-controlled docs or audit material, redact:

```text
private key arrays
seed phrases
mnemonics
payer keypair contents
authority keypair contents
provider tokens
tokenized RPC URLs
operator usernames in absolute paths
raw local filesystem paths
unreviewed receipt payloads
unreviewed deploy output
```

Safe replacements:

```text
<external-private-workspace>/keys/testnet-payer.json
<redacted-rpc-url>
<redacted-provider-token>
<redacted-signature>
<redacted-local-path>
```

## Phase 1 checker

Run:

```bash
bash scripts/check_actual_private_testnet_workspace.sh .
```

The checker confirms:

```text
.gitignore covers local private testnet artifacts
this document exists and is redacted
no key-shaped actual private testnet files exist in source paths
no raw tokenized RPC/provider URLs exist in source paths
git-tracked files do not include private testnet key/RPC/receipt artifacts
```

The checker is read-only.

It does not call RPC.

It does not load a wallet.

It does not deploy.

It does not submit.

It does not mint.

It does not burn.

It does not settle.

It does not mutate internal ROC.

## Non-authorization lock

No public launch authorization.

No mainnet-beta deployment.

No production bridge settlement.

No public ROX mint/burn.

No real internal ROC release.

No staking.

No liquidity.

No exchange-facing behavior.

No fake finality.

No fake success output.
