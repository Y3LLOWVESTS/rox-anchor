//! RO:WHAT — Deterministic local audit records for relayer testnet-shaped actions.
//! RO:WHY — Phase 11 requires inspectable receipt/simulation/submission reports before broader testnet use.
//! RO:INTERACTS — RelayerReceipt, TransactionSimulationResult, and CappedTestnetSubmissionResult.
//! RO:INVARIANTS — audit records expose consistency and never claim live submission.
//! RO:SECURITY — render output is local-only and flags sensitive-looking targets as unsafe for display.
//! RO:TEST — covered by testnet audit record tests.

use crate::{CappedTestnetSubmissionResult, RelayerReceipt, TransactionSimulationResult};

const AUDIT_RECORD_VERSION: &str = "relayer-testnet-audit-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestnetRelayerAuditRecord {
    pub version: &'static str,
    pub operation_id: String,
    pub idempotency_key: String,
    pub target: String,
    pub relayer_status: String,
    pub proof_decision: String,
    pub attempts_used: u8,
    pub simulation_status: String,
    pub instruction_count: u16,
    pub capped_submission_status: String,
    pub requested_attempts: u8,
    pub requested_operations: u16,
    pub amount_units: u64,
    pub receipt_persisted: bool,
    pub authorized: bool,
    pub live_submission_permitted: bool,
    pub live_submission_attempted: bool,
    pub network_submitted: bool,
    pub pipeline_consistent: bool,
}

impl TestnetRelayerAuditRecord {
    pub fn from_pipeline(
        receipt: &RelayerReceipt,
        simulation: &TransactionSimulationResult,
        capped: &CappedTestnetSubmissionResult,
        receipt_persisted: bool,
    ) -> Self {
        let pipeline_consistent = receipt.operation_id == simulation.operation_id
            && receipt.operation_id == capped.operation_id
            && receipt.idempotency_key == simulation.idempotency_key
            && receipt.idempotency_key == capped.idempotency_key
            && receipt.target == simulation.target
            && receipt.target == capped.target
            && receipt.status == simulation.relayer_status
            && simulation.relayer_status == capped.relayer_status
            && simulation.status == capped.simulation_status
            && simulation.proof_decision == capped.proof_decision;

        Self {
            version: AUDIT_RECORD_VERSION,
            operation_id: capped.operation_id.to_string(),
            idempotency_key: capped.idempotency_key.to_string(),
            target: capped.target.clone(),
            relayer_status: format!("{:?}", capped.relayer_status),
            proof_decision: format!("{:?}", capped.proof_decision),
            attempts_used: receipt.attempts_used,
            simulation_status: format!("{:?}", capped.simulation_status),
            instruction_count: simulation.instruction_count,
            capped_submission_status: format!("{:?}", capped.status),
            requested_attempts: capped.requested_attempts,
            requested_operations: capped.requested_operations,
            amount_units: capped.amount_units,
            receipt_persisted,
            authorized: capped.authorized,
            live_submission_permitted: capped.live_submission_permitted,
            live_submission_attempted: capped.live_submission_attempted,
            network_submitted: capped.network_submitted,
            pipeline_consistent,
        }
    }

    pub fn is_safe_for_display(&self) -> bool {
        self.pipeline_consistent
            && !self.live_submission_attempted
            && !self.network_submitted
            && !contains_sensitive_target_hint(&self.target)
    }

    pub fn render(&self) -> String {
        [
            format!("audit_record={}", self.version),
            format!("operation_id={}", self.operation_id),
            format!("idempotency_key={}", self.idempotency_key),
            format!("target={}", self.target),
            format!("relayer_status={}", self.relayer_status),
            format!("proof_decision={}", self.proof_decision),
            format!("attempts_used={}", self.attempts_used),
            format!("simulation_status={}", self.simulation_status),
            format!("instruction_count={}", self.instruction_count),
            format!("capped_submission_status={}", self.capped_submission_status),
            format!("requested_attempts={}", self.requested_attempts),
            format!("requested_operations={}", self.requested_operations),
            format!("amount_units={}", self.amount_units),
            format!("receipt_persisted={}", self.receipt_persisted),
            format!("authorized={}", self.authorized),
            format!(
                "live_submission_permitted={}",
                self.live_submission_permitted
            ),
            format!(
                "live_submission_attempted={}",
                self.live_submission_attempted
            ),
            format!("network_submitted={}", self.network_submitted),
            format!("pipeline_consistent={}", self.pipeline_consistent),
            format!("display_safe={}", self.is_safe_for_display()),
        ]
        .join("\n")
    }
}

fn contains_sensitive_target_hint(target: &str) -> bool {
    let lower = target.to_ascii_lowercase();

    [
        "secret",
        "token",
        "keypair",
        "wallet",
        "mnemonic",
        "seed",
        "private",
        "credential",
        "password",
        "rpc-url",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}
