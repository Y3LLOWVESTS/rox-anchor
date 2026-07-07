# Actual Private Testnet Build Artifacts

RO:WHAT — Defines BUILD_PLAN4 Phase 2 actual Anchor build artifact capture.
RO:WHY — Captures non-secret program/IDL build metadata before private devnet/testnet deployment.
RO:INTERACTS — Anchor.toml, target/deploy/rox_anchor.so, target/idl/rox_anchor.json, scripts/capture_actual_private_testnet_build_artifacts.sh.
RO:INVARIANTS — build evidence only; redacted paths; no deployment proof; no finality; no public launch.
RO:SECURITY — no wallet loading, RPC calls, signing, deployment, submission, minting, burning, bridge settlement, staking, liquidity, or ROC mutation.
RO:TEST — cargo test -p rox-anchor-core --test actual_testnet_artifact_manifest and cargo test -p rox-anchor-cli --test actual_testnet_artifact_manifest_status.

## Status

This document covers:

```text
ROX Anchor BUILD_PLAN4 Phase 2
Actual Anchor Build Artifact Capture
```

This phase is still non-deploying.

It may run:

```bash
anchor build
```

It must not run:

```bash
anchor deploy
solana program deploy
solana transfer
spl-token mint
spl-token burn
```

## Purpose

The purpose is to capture real local build metadata from the Anchor build outputs:

```text
target/deploy/rox_anchor.so
target/idl/rox_anchor.json
```

The captured metadata is useful before Phase 3 deployment because it pins:

```text
program ID expected by Anchor.toml
cluster binding used for the private testnet path
program binary SHA-256
IDL SHA-256
program binary size
IDL size
Anchor CLI version
Solana CLI version
Rust compiler version
redacted local artifact paths
```

## Required build command

Run:

```bash
anchor build
```

Then capture the manifest:

```bash
bash scripts/capture_actual_private_testnet_build_artifacts.sh --capture . .rox-anchor-private-pilot/actual-private-testnet-build-artifacts.local.json devnet
```

or for testnet:

```bash
bash scripts/capture_actual_private_testnet_build_artifacts.sh --capture . .rox-anchor-private-pilot/actual-private-testnet-build-artifacts.local.json testnet
```

The output path should stay ignored/local unless a redacted summary is intentionally promoted.

## Manifest schema

The capture script emits JSON with this shape:

```json
{
  "schema": "rox-anchor.actual-private-testnet-build-artifacts.v1",
  "phase": "BUILD_PLAN4 Phase 2",
  "artifact_role": "anchor_build_metadata_only",
  "cluster": "devnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "expected_program_id_source": "Anchor.toml [programs.devnet]",
  "program_binary_sha256": "<sha256>",
  "program_binary_size_bytes": 0,
  "idl_sha256": "<sha256>",
  "idl_size_bytes": 0,
  "anchor_version": "<anchor --version>",
  "solana_cli_version": "<solana --version>",
  "rustc_version": "<rustc --version>",
  "program_artifact_path": "<redacted-anchor-build-path>/rox_anchor.so",
  "idl_artifact_path": "<redacted-anchor-build-path>/rox_anchor.json",
  "build_manifest_is_deployment_proof": false,
  "deployment_claim": false,
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "real_roc_mutation": false
}
```

## What this proves

This proves:

```text
Anchor build artifacts exist.
The program ID expected for devnet/testnet is captured from Anchor.toml.
The program binary has a concrete hash.
The IDL has a concrete hash.
Tool versions are recorded.
Local build paths are redacted.
```

## What this does not prove

This does not prove:

```text
deployment success
program account existence
program account ownership
program executable status
IDL account upload
test-only mint initialization
transaction simulation success
transaction submission success
readback verification
bridge settlement
finality
public launch readiness
mainnet readiness
production readiness
```

## Required local checks

Run:

```bash
cargo fmt -p rox-anchor-core -p rox-anchor-cli
anchor build
bash scripts/capture_actual_private_testnet_build_artifacts.sh --check-docs .
bash scripts/capture_actual_private_testnet_build_artifacts.sh --capture . .rox-anchor-private-pilot/actual-private-testnet-build-artifacts.local.json devnet
cargo test -p rox-anchor-core --test actual_testnet_artifact_manifest
cargo test -p rox-anchor-cli --test actual_testnet_artifact_manifest_status
cargo check -p rox-anchor
```

## Non-authorization lock

No deployment proof.

No finality proof.

No public launch authorization.

No mainnet-beta authorization.

No production bridge settlement.

No public ROX mint/burn.

No real internal ROC release.

No staking.

No liquidity.

No exchange-facing behavior.

No fake success output.
