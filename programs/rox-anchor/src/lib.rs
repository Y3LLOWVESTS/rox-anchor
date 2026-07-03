//! RO:WHAT — Compile-tested Anchor program surface for ROX Anchor.
//! RO:WHY — Defines program-owned state transitions before mint/burn logic is added.
//! RO:INTERACTS — instruction handlers, program state, errors, and events.
//! RO:INVARIANTS — challenge, halt, and recovery blockers are enforced by handlers.
//! RO:SECURITY — local compile/test surface only; no production deployment or value movement.
//! RO:TEST — cargo check -p rox-anchor and cargo test -p rox-anchor.

#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![forbid(unsafe_code)]

use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

// Anchor's #[program] macro expects generated client account modules at the
// crate root. Because account contexts live in instruction submodules, re-export
// those generated modules here while keeping the one-file-per-instruction layout.
pub(crate) use instructions::finalize::__client_accounts_finalize;
pub(crate) use instructions::halt::__client_accounts_halt;
pub(crate) use instructions::initialize::__client_accounts_initialize;
pub(crate) use instructions::observe_burn::__client_accounts_observe_burn;
pub(crate) use instructions::open_challenge::__client_accounts_open_challenge;
pub(crate) use instructions::recover::__client_accounts_recover;
pub(crate) use instructions::resolve_challenge::__client_accounts_resolve_challenge;

declare_id!("U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR");

#[allow(deprecated)]
#[program]
pub mod rox_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitializeConfigArgs) -> Result<()> {
        instructions::initialize::handler(ctx, args)
    }

    pub fn observe_burn(ctx: Context<ObserveBurn>, args: OperationBindingArgs) -> Result<()> {
        instructions::observe_burn::handler(ctx, args)
    }

    pub fn open_challenge(ctx: Context<OpenChallenge>) -> Result<()> {
        instructions::open_challenge::handler(ctx)
    }

    pub fn resolve_challenge(ctx: Context<ResolveChallenge>, accepted: bool) -> Result<()> {
        instructions::resolve_challenge::handler(ctx, accepted)
    }

    pub fn halt(ctx: Context<Halt>) -> Result<()> {
        instructions::halt::handler(ctx)
    }

    pub fn recover(ctx: Context<Recover>) -> Result<()> {
        instructions::recover::handler(ctx)
    }

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        instructions::finalize::handler(ctx)
    }
}
