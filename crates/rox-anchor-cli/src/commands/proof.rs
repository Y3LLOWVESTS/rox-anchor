//! RO:WHAT — Proof command notes for the ROX Anchor local inspection CLI.
//! RO:WHY — Points users toward `check` until JSON proof-package inspection is added.
//! RO:INTERACTS — check command and rox-anchor-proof.
//! RO:INVARIANTS — proof output is local review guidance, not finality or settlement.
//! RO:SECURITY — no live RPC, wallet, deployment, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — covered through CLI command dispatch tests.

pub fn proof_help() -> String {
    [
        "rox-anchor proof",
        "status: local inspection surface",
        "next: use `rox-anchor check` for deterministic proof-review output",
        "json_input: not enabled yet",
    ]
    .join("\n")
}
