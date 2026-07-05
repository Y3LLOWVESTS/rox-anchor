# ROX Anchor Phase 14 — RPC Trust Boundary

No public launch authorization.

This document summarizes the read-only RPC evidence boundary. RPC observations may inform local proof review, but RPC reads do not authorize settlement, public minting, public burning, ROC release, deployment, wallet use, or finality claims.

## RPC boundary files

```text
crates/rox-anchor-rpc-proof/src/rpc.rs
crates/rox-anchor-rpc-proof/src/quorum.rs
crates/rox-anchor-rpc-proof/src/commitment.rs
crates/rox-anchor-rpc-proof/src/audit.rs
crates/rox-anchor-rpc-proof/src/redaction.rs
```

## Core invariants

| Invariant | Test coverage |
| --- | --- |
| Read-only adapter can read slot/account/signature-shaped evidence without submitting. | `crates/rox-anchor-rpc-proof/tests/read_only_rpc_adapter.rs` |
| Missing evidence blocks proof acceptance. | `crates/rox-anchor-rpc-proof/tests/rpc_to_proof_boundary.rs` |
| Disputed evidence blocks proof acceptance. | `crates/rox-anchor-rpc-proof/tests/rpc_to_proof_boundary.rs` |
| Same-source equivocation is detected. | `crates/rox-anchor-rpc-proof/tests/rpc_equivocation_chaos.rs` |
| RPC outage fails closed and does not fabricate quorum. | `crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs` |
| Missing program account is visible but not proof evidence. | `crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs` |
| Reorg-like stale signature evidence is rejected. | `crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs` |
| Wrong program, mint, or token account is binding tamper. | `crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs` |

## Failure modes

RPC evidence must fail safely for:

```text
RPC outage
RPC stale slots
RPC equivocation
RPC provider disagreement
missing program account
missing signature status
wrong cluster
wrong program ID
wrong mint
wrong token account
wrong operation ID
under-quorum evidence
```

## Audit note

RPC is an evidence input, not an authority. It cannot bypass proof review, coordinator review, relayer dry-run, simulation gates, capped submission gates, halt posture, recovery posture, challenge posture, or Anchor program state rules.
