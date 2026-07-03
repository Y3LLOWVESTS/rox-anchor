//! RO:WHAT — Redacted local coordinator report helpers.
//! RO:WHY — Gives tests and future CLI/service output a deterministic summary without raw evidence dumps.
//! RO:INTERACTS — CoordinatorDecision.
//! RO:INVARIANTS — report rendering does not change decisions.
//! RO:SECURITY — no secrets, wallet calls, RPC calls, or settlement behavior.
//! RO:TEST — covered by redacted report tests.

use crate::CoordinatorDecision;

pub fn redacted_coordinator_report(decision: &CoordinatorDecision) -> String {
    [
        format!("coordinator_status={:?}", decision.status),
        format!("rpc_decision={:?}", decision.rpc_review.decision),
        format!("proof_decision={:?}", decision.proof_review.decision),
        format!("proof_status={:?}", decision.proof_review.lifecycle_state),
    ]
    .join("\n")
}
