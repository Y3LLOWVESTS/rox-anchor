// RO:WHAT — Error labels for the disabled rox-anchor-core skeleton.
// RO:WHY — Keeps future local-only validation names conservative and non-authoritative.
// RO:INTERACTS — ids, types, and state skeleton modules.
// RO:INVARIANTS — Error labels are not runtime decisions and do not authorize runtime.
// RO:SECURITY — No network, wallet, Solana/Anchor, bridge runtime, deployment, or settlement behavior.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

use core::fmt;

/// Non-runtime error labels for local skeleton review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoreSkeletonError {
    EmptyIdentifier,
    InvalidIdentifierWhitespace,
    UnsupportedDirection,
    UnsupportedState,
    RuntimeNotAuthorized,
    AuthorityBoundary,
}

impl fmt::Display for CoreSkeletonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::EmptyIdentifier => "empty identifier",
            Self::InvalidIdentifierWhitespace => "identifier contains leading or trailing whitespace",
            Self::UnsupportedDirection => "unsupported direction",
            Self::UnsupportedState => "unsupported state",
            Self::RuntimeNotAuthorized => "runtime is not authorized",
            Self::AuthorityBoundary => "authority boundary preserved",
        };

        f.write_str(label)
    }
}

impl std::error::Error for CoreSkeletonError {}
