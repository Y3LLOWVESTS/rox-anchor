// RO:WHAT — Disabled halt command-shape labels for rox-anchor-cli.
// RO:WHY — Names future local halt-state inspection without creating operational halt authority.
// RO:INTERACTS — commands module and future halt/recovery review design.
// RO:INVARIANTS — Halt command shape is local review posture only and does not authorize runtime.
// RO:SECURITY — No coordinator authority, no relayer authority, no deployment, no bridge runtime, no external settlement.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

/// Disabled local halt command shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HaltCommandSkeleton {
    pub halt_label: String,
    pub review_only: bool,
}

impl HaltCommandSkeleton {
    pub fn disabled(halt_label: impl Into<String>) -> Self {
        Self {
            halt_label: halt_label.into(),
            review_only: true,
        }
    }

    pub fn is_operational_authority(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }
}
