//! RO:WHAT — Instruction module map for the ROX Anchor Anchor program.
//! RO:WHY — Keeps one-file-per-transition ownership while exposing account contexts cleanly to #[program].
//! RO:INTERACTS — initialize, observe_burn, challenge, halt, recover, finalize, and local SPL token CPI finalize handlers and ROX-to-ROC release review and burn CPI handlers.
//! RO:INVARIANTS — handlers enforce halt/challenge/recovery blockers before finalization, token CPI, or release review/burn CPI.
//! RO:SECURITY — token CPI is explicit and account-constrained; no hidden production bridge behavior.
//! RO:TEST — cargo check -p rox-anchor and cargo test -p rox-anchor.

pub mod finalize;
pub mod halt;
pub mod initialize;
pub mod observe_burn;
pub mod open_challenge;
pub mod recover;
pub mod resolve_challenge;

pub use finalize::{
    Finalize, FinalizeRocToRoxMint, FinalizeRoxToRocBurn, FinalizeRoxToRocReleaseReview,
};
pub use halt::Halt;
pub use initialize::Initialize;
pub use observe_burn::ObserveBurn;
pub use open_challenge::OpenChallenge;
pub use recover::Recover;
pub use resolve_challenge::ResolveChallenge;
