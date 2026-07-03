//! RO:WHAT — Typed identifier wrappers for ROX Anchor bindings.
//! RO:WHY — Prevents raw-string drift for domains, operation IDs, nonces, mints, accounts, and program IDs.
//! RO:INTERACTS — proof package validation, coordinator evidence, relayer dry-run receipts, and Anchor state.
//! RO:INVARIANTS — identifiers are non-empty, trimmed, bounded, and control-byte-free.
//! RO:SECURITY — validation-only; these types do not grant authority.
//! RO:TEST — covered by rox-anchor-core identifier and binding tests.

use core::fmt;

use crate::AnchorCoreError;

const MAX_IDENTIFIER_BYTES: usize = 128;

fn validate_identifier(
    kind: &'static str,
    value: impl Into<String>,
) -> Result<String, AnchorCoreError> {
    let value = value.into();
    let actual_bytes = value.len();

    if value.is_empty() {
        return Err(AnchorCoreError::EmptyIdentifier { kind });
    }

    if value.trim() != value {
        return Err(AnchorCoreError::IdentifierHasOuterWhitespace { kind });
    }

    if actual_bytes > MAX_IDENTIFIER_BYTES {
        return Err(AnchorCoreError::IdentifierTooLong {
            kind,
            max_bytes: MAX_IDENTIFIER_BYTES,
            actual_bytes,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(AnchorCoreError::IdentifierHasControlByte { kind });
    }

    Ok(value)
}

macro_rules! typed_id {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, AnchorCoreError> {
                validate_identifier($kind, value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

typed_id!(DomainId, "domain");
typed_id!(OperationId, "operation_id");
typed_id!(IdempotencyKey, "idempotency_key");
typed_id!(Nonce, "nonce");
typed_id!(ClusterId, "cluster");
typed_id!(ProgramId, "program_id");
typed_id!(MintId, "mint");
typed_id!(TokenAccountId, "token_account");
typed_id!(AccountId, "account");
