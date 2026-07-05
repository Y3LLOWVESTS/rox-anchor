//! RO:WHAT — Local relayer receipt records.
//! RO:WHY — Captures dry-run outcomes without claiming live submission or settlement.
//! RO:INTERACTS — submit.rs and redaction.rs.
//! RO:INVARIANTS — receipts are local records; they are not chain receipts.
//! RO:SECURITY — no live RPC, wallet, transaction, mint, burn, or settlement authority.
//! RO:TEST — covered by dry-run receipt tests.

use rox_anchor_core::{IdempotencyKey, OperationId};
use rox_anchor_proof::ReviewDecision;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelayerReceiptStatus {
    DryRunAccepted,
    ProofBlocked,
    ProofRejected,
    ChallengeBlocked,
    Halted,
    RecoveryBlocked,
    DuplicateRequest,
    ReceiptCapacityReached,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerReceipt {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub status: RelayerReceiptStatus,
    pub proof_decision: ReviewDecision,
    pub attempts_used: u8,
    pub live_submission: bool,
}

impl RelayerReceipt {
    pub fn new(
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        target: impl Into<String>,
        status: RelayerReceiptStatus,
        proof_decision: ReviewDecision,
        attempts_used: u8,
    ) -> Self {
        Self {
            operation_id,
            idempotency_key,
            target: target.into(),
            status,
            proof_decision,
            attempts_used,
            live_submission: false,
        }
    }
}
