//! RO:WHAT — Shared error type for ROX Anchor core validation.
//! RO:WHY — Gives all crates one reusable error vocabulary for IDs and core bindings.
//! RO:INTERACTS — ids, types, proof validation, CLI display, and local service models.
//! RO:INVARIANTS — errors are deterministic and do not imply finality or runtime authorization.
//! RO:SECURITY — validation-only; no wallet/RPC/deployment side effects.
//! RO:TEST — covered by rox-anchor-core identifier tests.

use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnchorCoreError {
    EmptyIdentifier {
        kind: &'static str,
    },
    IdentifierHasOuterWhitespace {
        kind: &'static str,
    },
    IdentifierTooLong {
        kind: &'static str,
        max_bytes: usize,
        actual_bytes: usize,
    },
    IdentifierHasControlByte {
        kind: &'static str,
    },
}

impl fmt::Display for AnchorCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => write!(f, "{kind} is empty"),
            Self::IdentifierHasOuterWhitespace { kind } => {
                write!(f, "{kind} has leading or trailing whitespace")
            }
            Self::IdentifierTooLong {
                kind,
                max_bytes,
                actual_bytes,
            } => write!(
                f,
                "{kind} is too long: {actual_bytes} bytes exceeds {max_bytes} bytes"
            ),
            Self::IdentifierHasControlByte { kind } => {
                write!(f, "{kind} contains a control byte")
            }
        }
    }
}

impl std::error::Error for AnchorCoreError {}
