// RO:WHAT — Command-shape index for the disabled rox-anchor-cli inspection skeleton.
// RO:WHY — Names future local inspection surfaces without implementing runtime behavior.
// RO:INTERACTS — check, proof, status, recover, and halt command-shape modules.
// RO:INVARIANTS — Command-shape labels are not authority and do not authorize runtime.
// RO:SECURITY — No network, wallet, Solana/Anchor, bridge runtime, deployment, or settlement behavior.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

pub mod check;
pub mod halt;
pub mod proof;
pub mod recover;
pub mod status;

pub use check::CheckCommandSkeleton;
pub use halt::HaltCommandSkeleton;
pub use proof::ProofCommandSkeleton;
pub use recover::RecoverCommandSkeleton;
pub use status::StatusCommandSkeleton;

/// Non-authorization marker for all command-shape modules.
pub const DISABLED_CLI_NON_AUTHORIZATION: &str =
    "CLI command shapes are local inspection labels only and do not authorize runtime";

/// Disabled CLI posture labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CliDisabledPosture {
    DisabledByDefault,
    LocalOnly,
    NonValue,
    NoNetwork,
    NoWallet,
    NoDeployment,
    RuntimeNotAuthorized,
}

/// Future-gated command-shape labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CliInspectionCommand {
    Check,
    Proof,
    Status,
    Recover,
    Halt,
}

pub fn disabled_cli_posture() -> CliDisabledPosture {
    CliDisabledPosture::RuntimeNotAuthorized
}
