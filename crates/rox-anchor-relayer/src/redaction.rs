//! RO:WHAT — Redacted local relayer receipt reports.
//! RO:WHY — Gives future CLI/service output deterministic dry-run summaries.
//! RO:INTERACTS — RelayerReceipt.
//! RO:INVARIANTS — report rendering does not change receipt meaning.
//! RO:SECURITY — no secrets, wallet calls, RPC calls, or settlement behavior.
//! RO:TEST — covered by redacted report tests.

use crate::RelayerReceipt;

pub fn redacted_receipt_report(receipt: &RelayerReceipt) -> String {
    [
        format!("relayer_status={:?}", receipt.status),
        format!("operation_id={}", receipt.operation_id),
        format!("target={}", receipt.target),
        format!("proof_decision={:?}", receipt.proof_decision),
        format!("attempts_used={}", receipt.attempts_used),
        format!("live_submission={}", receipt.live_submission),
    ]
    .join("\n")
}
