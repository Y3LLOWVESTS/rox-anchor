//! RO:WHAT — Command registry for the ROX Anchor local inspection CLI.
//! RO:WHY — Keeps command modules small and prevents CLI behavior from drifting into main.rs.
//! RO:INTERACTS — check, proof, status, submit, halt, and recover modules.
//! RO:INVARIANTS — default commands are inspection/reporting; Phase 4 live simulation is explicit and non-submitting.
//! RO:SECURITY — no transaction submission, deployment, mint/burn execution, staking, liquidity, or settlement; explicit Phase 4 simulation may load local pilot signers.
//! RO:TEST — exercised through rox_anchor_cli::run_from_args tests.

pub mod audit;
pub mod check;
pub mod drill;
pub mod halt;
pub mod phase4_live_executor;
pub mod phase4_live_init;
pub mod phase4_live_submit;
pub mod phase5_live_closeout;
pub mod phase5_live_quorum;
pub mod phase5_live_read_only;
mod phase5_wire_compat;
pub mod phase6_live_rpc_simulation;
pub mod phase6_live_simulation;
pub mod phase7_live_capped_sender;
pub mod phase7_live_closeout;
pub mod phase7_live_manual_execution;
pub mod phase7_live_signed_executor;
pub mod phase7_live_simulation_authorization;
pub mod phase7_live_submission_readback;
pub mod phase8_live_execution;
pub mod phase8_rox_to_roc_simulation;
pub mod pilot;
pub mod posture_audit;
pub mod proof;
pub mod receipts;
pub mod recover;
pub mod status;
pub mod submit;
pub mod test_only_init;
