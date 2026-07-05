# ROX Anchor Private Testnet Operator Workspace

RO:WHAT — Defines the local-only private pilot workspace layout for BUILD_PLAN3 Phase 1.
RO:WHY — Keeps operator keys, RPC URLs, provider tokens, deployment captures, and pilot receipts outside tracked source.
RO:INTERACTS — .gitignore, scripts/check_private_pilot_hygiene.sh, future private testnet config and receipt tools.
RO:INVARIANTS — local-only / ignored / external artifacts; no committed keypairs; no raw RPC tokens; no fake finality or fake success.
RO:SECURITY — No public launch authorization. No mainnet-beta deployment. No production bridge settlement. No real internal ROC release.
RO:TEST — bash scripts/check_private_pilot_hygiene.sh . and cargo test -p rox-anchor-cli --test private_pilot_hygiene.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN3 Phase 1
Private Pilot Operator Workspace Hygiene
```

This is a local operator hygiene document.

It does not authorize deployment.

It does not authorize live RPC submission.

It does not authorize public ROX minting or burning.

It does not authorize production bridge settlement.

It does not authorize real internal ROC release.

It does not authorize mainnet-beta.

## Required local-only posture

Private pilot operator artifacts must stay:

```text
local-only / ignored / external
```

That means they must not be committed to the repo.

The expected local workspace is:

```text
.rox-anchor-pilot/
  keys/
    testnet-payer.json
    testnet-program-keypair.json
    testnet-mint-authority.json
    testnet-upgrade-authority.json

  rpc/
    testnet-rpc-url.txt
    provider-token.env

  deploy/
    anchor-build-notes.local.md
    deploy-output.local.json
    program-artifact-manifest.local.json

  receipts/
    pilot-run-0001.receipt.json
    pilot-run-0002.receipt.json

  audit/
    unredacted-local-audit.json
    redaction-review.local.md

  tmp/
    scratch files
```

The repo may contain redacted documentation and deterministic tests, but it must not contain:

```text
raw keypairs
wallet JSON
payer JSON
authority JSON
mint authority JSON
upgrade authority JSON
provider URLs with tokens
RPC URL files
unredacted pilot receipts
unredacted deployment output
unredacted audit bundles
local validator ledgers
```

## Safe source-controlled files

The following source files are safe to track:

```text
docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md
scripts/check_private_pilot_hygiene.sh
crates/rox-anchor-cli/tests/private_pilot_hygiene.rs
```

These files define and test the boundary.

They do not load wallets.

They do not call RPC.

They do not deploy.

They do not submit transactions.

They do not mint.

They do not burn.

They do not settle.

## Operator environment rules

Future pilot commands may read environment variables such as:

```text
ROX_ANCHOR_PILOT_CONFIG
ROX_ANCHOR_PILOT_RPC_URL_FILE
ROX_ANCHOR_PILOT_WALLET_PATH
ROX_ANCHOR_PILOT_PROGRAM_KEYPAIR_PATH
ROX_ANCHOR_PILOT_RECEIPT_DIR
```

Those values must point outside tracked source or into ignored local pilot directories.

Do not place raw values in committed scripts, docs, tests, logs, or codebundles.

## Redaction rules

Before any pilot artifact is intentionally promoted into docs or audit materials, it must be redacted.

Redact at least:

```text
private key arrays
seed phrases
provider tokens
RPC token query parameters
wallet filesystem paths
operator usernames in absolute paths
transaction signatures if the report is not intended to publish them
unredacted receipt payloads
```

Safe replacement examples:

```text
<redacted-keypair-path>/testnet-payer.json
https://api.testnet.solana.com/<redacted>
<redacted-signature>
<redacted-provider-token>
```

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

