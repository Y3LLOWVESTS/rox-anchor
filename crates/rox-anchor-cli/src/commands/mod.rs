//! RO:WHAT — Command registry for the ROX Anchor local inspection CLI.
//! RO:WHY — Keeps command modules small and prevents CLI behavior from drifting into main.rs.
//! RO:INTERACTS — check, proof, status, submit, halt, and recover modules.
//! RO:INVARIANTS — all commands are local inspection/reporting; no live runtime side effects.
//! RO:SECURITY — no RPC submission, wallet calls, deployment, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — exercised through rox_anchor_cli::run_from_args tests.

pub mod audit;
pub mod check;
pub mod drill;
pub mod halt;
pub mod posture_audit;
pub mod proof;
pub mod recover;
pub mod status;
pub mod submit;
