# Private Testnet Deployment Runbook

This is the BUILD_PLAN3 Phase 4 private testnet deployment drill runbook.

This drill is **not a launch**.

This drill is **not public ROX availability**.

This drill is **not bridge settlement**.

This drill is **not internal ROC mutation**.

This drill exists to prepare or rehearse a private devnet/testnet deployment using external operator-controlled files.

## Required external inputs

The operator must keep these outside tracked source:

```text
external deploy keypair path
external payer path
external upgrade authority path
external RPC/provider URL, if one is used later
```

Suggested local environment variable names:

```text
ROX_ANCHOR_PRIVATE_TESTNET_PAYER=/external/non-repo/path/private-testnet-payer.json
ROX_ANCHOR_PRIVATE_TESTNET_PROGRAM_KEYPAIR=/external/non-repo/path/rox-anchor-program-keypair.json
ROX_ANCHOR_PRIVATE_TESTNET_UPGRADE_AUTHORITY=/external/non-repo/path/upgrade-authority.json
```

Do not commit these files.

Do not paste their contents into docs, terminal logs, receipts, or codebundles.

## Required safe preflight

Run:

```bash
cargo fmt --all
cargo test --workspace
cargo check --workspace
anchor build
anchor test
bash scripts/check_private_testnet_deploy.sh .
```

The checker is read-only. It does not call RPC, load a wallet, deploy, submit, mint, burn, settle, or mutate internal ROC.

## Optional private testnet drill

Only after the safe preflight passes and the operator explicitly chooses to perform a private testnet drill:

```bash
anchor build
anchor deploy --provider.cluster testnet --provider.wallet "$ROX_ANCHOR_PRIVATE_TESTNET_PAYER"
```

Any actual drill output must stay in ignored local paths such as:

```text
pilot-deploy/
pilot-artifacts/
pilot-receipts/
.rox-anchor-private-pilot/
```

## Redacted deployment drill report

A redacted deployment drill report may record:

```text
cluster label
program ID
build hash
IDL hash
deploy slot, if supplied
operator-visible label
redacted artifact path
redacted IDL path
```

A redacted deployment drill report must not record:

```text
private key material
payer keypair contents
upgrade authority keypair contents
RPC provider token
raw wallet path if it leaks local identity
seed phrase
secret environment values
```

The report must not claim:

```text
public launch
public ROX availability
bridge settlement
real ROC release
mainnet readiness
finality beyond the private testnet drill context
```

## Forbidden

```text
mainnet-beta
public launch claims
public ROX mint/burn claims
real user funds
production bridge settlement
internal ROC release
committed operator keys
committed deployment outputs
fake finality
fake success output
silent wallet/key usage
silent live RPC submission
```

## Exit condition

This phase is complete when:

```text
anchor build passes
the private deployment drill checker passes
the checker is covered by a cargo test
the runbook is present
no key material or deploy output is tracked
the codebundle remains secret-safe
```
