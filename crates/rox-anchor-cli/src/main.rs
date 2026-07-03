//! RO:WHAT — Disabled non-runtime CLI inspection skeleton for ROX Anchor planning.
//! RO:WHY — Provides local-only command-shape names for future review without enabling runtime behavior.
//! RO:INTERACTS — commands module only.
//! RO:INVARIANTS — CLI inspection labels are not bridge authority and do not authorize runtime.
//! RO:SECURITY — No RPC, no wallet, no Solana/Anchor runtime, no bridge runtime, no deployment, no value movement.
//! RO:TEST — Static checker only at this phase.
//!
//! ROX-ANCHOR:FUTURE-GATED-CONTEXT
//!
//! This disabled skeleton does not authorize runtime.

#![forbid(unsafe_code)]

pub mod commands;

pub use commands::{
    CliDisabledPosture, CliInspectionCommand, DISABLED_CLI_NON_AUTHORIZATION,
};

/// Compile-time marker proving this CLI crate is a disabled skeleton, not runtime.
pub const ROX_ANCHOR_CLI_DISABLED_SKELETON: bool = true;

/// Human-readable non-authorization marker used by static review tools.
pub const ROX_ANCHOR_CLI_NON_AUTHORIZATION: &str =
    "rox-anchor-cli is a disabled local inspection skeleton and does not authorize runtime";

/// Disabled local entry placeholder.
///
/// This function intentionally performs no bridge, wallet, RPC, deployment, or settlement behavior.
fn main() {
    let _ = commands::disabled_cli_posture();
}
