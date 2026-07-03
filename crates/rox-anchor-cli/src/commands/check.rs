// RO:WHAT — Disabled check command-shape labels for rox-anchor-cli.
// RO:WHY — Preserves a future local inspection surface without implementing proof validation or runtime behavior.
// RO:INTERACTS — commands module and future-gated local checker design.
// RO:INVARIANTS — Check command shape is local-only and does not authorize runtime.
// RO:SECURITY — No RPC, wallet, Solana/Anchor runtime, bridge runtime, deployment, staking, liquidity, or external settlement.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

/// Disabled local check command shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckCommandSkeleton {
    pub input_label: String,
    pub local_only: bool,
}

impl CheckCommandSkeleton {
    pub fn disabled(input_label: impl Into<String>) -> Self {
        Self {
            input_label: input_label.into(),
            local_only: true,
        }
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn touches_network(&self) -> bool {
        false
    }

    pub fn touches_wallet(&self) -> bool {
        false
    }
}
