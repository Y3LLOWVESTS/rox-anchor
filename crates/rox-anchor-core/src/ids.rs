// RO:WHAT — Identifier wrappers for the disabled rox-anchor-core skeleton.
// RO:WHY — Separates operation identity, retry identity, nonce, and domain labels before any runtime exists.
// RO:INTERACTS — ProofPackageSkeleton and future-gated state/proof design docs.
// RO:INVARIANTS — Identifiers are local labels only and do not authorize runtime.
// RO:SECURITY — No RPC, wallet, Solana/Anchor, bridge runtime, deployment, or settlement behavior.
// RO:TEST — Static checker only at this phase.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This disabled skeleton does not authorize runtime.

use crate::errors::CoreSkeletonError;

fn checked_label(value: impl Into<String>) -> Result<String, CoreSkeletonError> {
    let value = value.into();

    if value.is_empty() {
        return Err(CoreSkeletonError::EmptyIdentifier);
    }

    if value.trim() != value {
        return Err(CoreSkeletonError::InvalidIdentifierWhitespace);
    }

    Ok(value)
}

/// Durable future-gated operation identity label.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AnchorId(String);

impl AnchorId {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreSkeletonError> {
        checked_label(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Retry identity label; not authority.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreSkeletonError> {
        checked_label(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Replay-defense label.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Nonce(String);

impl Nonce {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreSkeletonError> {
        checked_label(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Source or target domain label.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AnchorDomain(String);

impl AnchorDomain {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreSkeletonError> {
        checked_label(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
