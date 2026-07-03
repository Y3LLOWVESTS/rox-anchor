// RO:WHAT — Disabled recovery command-shape labels for rox-anchor-cli.
// RO:WHY — Names future local recovery-case inspection without creating hidden value or authority paths.
// RO:INTERACTS — commands module and future recovery-case review.
// RO:INVARIANTS — Recovery command shape is review posture only and does not authorize runtime.
// RO:SECURITY — No direct ledger mutation, no wallet call, no bridge runtime, no deployment, no staking, no liquidity, no external settlement.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

/// Disabled local recovery command shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoverCommandSkeleton {
    pub recovery_label: String,
    pub review_only: bool,
}

impl RecoverCommandSkeleton {
    pub fn disabled(recovery_label: impl Into<String>) -> Self {
        Self {
            recovery_label: recovery_label.into(),
            review_only: true,
        }
    }

    pub fn is_hidden_value_path(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn touches_ledger(&self) -> bool {
        false
    }
}
