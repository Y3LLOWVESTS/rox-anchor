// RO:WHAT — Disabled proof command-shape labels for rox-anchor-cli.
// RO:WHY — Names future local proof inspection without implementing proof validation, RPC, or finality.
// RO:INTERACTS — commands module and future local proof package review.
// RO:INVARIANTS — Proof command shape is evidence inspection only and does not authorize runtime.
// RO:SECURITY — No RPC, wallet, Solana/Anchor runtime, bridge runtime, deployment, minting, burning, settlement, staking, liquidity, or external settlement.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

/// Disabled local proof command shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofCommandSkeleton {
    pub proof_label: String,
    pub evidence_only: bool,
}

impl ProofCommandSkeleton {
    pub fn disabled(proof_label: impl Into<String>) -> Self {
        Self {
            proof_label: proof_label.into(),
            evidence_only: true,
        }
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn touches_rpc(&self) -> bool {
        false
    }
}
