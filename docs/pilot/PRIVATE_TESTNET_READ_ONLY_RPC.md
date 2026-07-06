# Private Testnet Read-Only RPC Verification

This is the BUILD_PLAN3 Phase 6 runbook for private testnet read-only RPC verification.

This is a **read-only RPC** phase.

This is a **private testnet** phase.

This requires **external config** when a real operator later points the tool at an RPC provider.

This is **no transaction submission**.

This is **no wallet loading**.

This is **no internal ROC mutation**.

## Purpose

The private pilot must be able to read testnet state without changing it.

The readback shape checks:

```text
current slot
program account status
config account status
mint account status
token account status
signature status
RPC quorum result
stale readback findings
missing readback findings
mismatched owner findings
disputed signature findings
```

## Local test mode

The automated tests use fake read-only adapters only.

They do not call live RPC.

They do not load keys.

They do not sign.

They do not send.

They do not initialize a mint.

They do not mint tokens.

They do not burn tokens.

They do not settle any bridge operation.

They do not mutate internal ROC.

## Required local checks

Run:

```bash
cargo fmt -p rox-anchor-rpc-proof -p rox-anchor-cli
cargo test -p rox-anchor-rpc-proof --test private_testnet_read_only_rpc
cargo test -p rox-anchor-rpc-proof
cargo test -p rox-anchor-cli --test private_testnet_read_only_rpc
cargo test --workspace
```

## Optional future manual shape

Only after the local tests pass and an external private testnet config exists, an operator may perform a read-only check using an explicit external path.

```bash
# cargo run -p rox-anchor-cli -- proof read-only \
#   --config /external/path/to/private-testnet.toml
```

That optional command shape is read-only. It must not submit transactions or load wallet keys.

## Exit condition

This phase is complete when:

```text
fake-adapter read-only verification passes
missing account readback fails closed
owner mismatch fails closed
stale account readback fails closed
disputed signature quorum fails closed
CLI proof output remains display-safe
workspace tests stay green
codebundle remains secret-safe
```
