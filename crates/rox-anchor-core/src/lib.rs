//! RO:WHAT — Disabled non-runtime core skeleton for ROX Anchor planning.
//! RO:WHY — Provides safe, local-only type names for later review without enabling runtime behavior.
//! RO:INTERACTS — ids, types, state, errors, and labels modules inside rox-anchor-core only.
//! RO:INVARIANTS — Internal ROC truth remains svc-wallet + ron-ledger; this crate does not authorize runtime.
//! RO:SECURITY — No RPC, no wallet, no Solana/Anchor runtime, no bridge runtime, no deployment, no value movement.
//! RO:TEST — Static checker only at this phase.
//!
//! ROX-ANCHOR:FUTURE-GATED-CONTEXT
//!
//! This disabled skeleton does not authorize runtime.

#![forbid(unsafe_code)]

pub mod errors;
pub mod ids;
pub mod labels;
pub mod state;
pub mod types;

pub use errors::CoreSkeletonError;
pub use ids::{AnchorDomain, AnchorId, IdempotencyKey, Nonce};
pub use state::{AnchorState, FailureClosedPosture};
pub use types::{
    AnchorDirection, ChallengePosture, HaltPosture, ProofPackageSkeleton, RecoveryPosture,
};

/// Compile-time marker proving this crate is a disabled skeleton, not runtime.
pub const ROX_ANCHOR_CORE_DISABLED_SKELETON: bool = true;

/// Human-readable non-authorization marker used by static review tools.
pub const ROX_ANCHOR_CORE_NON_AUTHORIZATION: &str =
    "rox-anchor-core is a disabled non-runtime skeleton and does not authorize runtime";
