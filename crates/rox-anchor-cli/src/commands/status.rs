// RO:WHAT — Disabled status command-shape labels for rox-anchor-cli.
// RO:WHY — Keeps status wording local, stale-safe, and non-authoritative before any UI/runtime exists.
// RO:INTERACTS — commands module and future CrabLink display-only status design.
// RO:INVARIANTS — Status command shape is display posture only and does not authorize runtime.
// RO:SECURITY — No client finality, no cache truth, no wallet/RPC authority, no Solana/Anchor runtime, no settlement behavior.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

/// Disabled local status command shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusCommandSkeleton {
    pub status_label: String,
    pub display_only: bool,
}

impl StatusCommandSkeleton {
    pub fn disabled(status_label: impl Into<String>) -> Self {
        Self {
            status_label: status_label.into(),
            display_only: true,
        }
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn is_user_facing_bridge_claim(&self) -> bool {
        false
    }
}
