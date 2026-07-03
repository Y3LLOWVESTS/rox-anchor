//! RO:WHAT — Local dry-run submission model for ROX Anchor relayer.
//! RO:WHY — Accepts only proof-accepted local reviews and emits local receipts.
//! RO:INTERACTS — rox-anchor-proof ProofReview, retry policy, receipts, and config.
//! RO:INVARIANTS — duplicate idempotency is rejected; blocked/rejected proof reviews are not attempted.
//! RO:SECURITY — dry-run only; no live transaction submission, RPC, wallet, mint, burn, or settlement.
//! RO:TEST — covered by accepted, blocked, duplicate, and capacity tests.

use std::collections::BTreeSet;

use rox_anchor_core::{IdempotencyKey, OperationId};
use rox_anchor_proof::{ProofReview, ReviewDecision};

use crate::{RelayerConfig, RelayerReceipt, RelayerReceiptStatus, RetryPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerSubmissionRequest {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub proof_review: ProofReview,
    pub requested_attempts: u8,
}

impl RelayerSubmissionRequest {
    pub fn new(
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        target: impl Into<String>,
        proof_review: ProofReview,
    ) -> Self {
        Self {
            operation_id,
            idempotency_key,
            target: target.into(),
            proof_review,
            requested_attempts: 1,
        }
    }

    pub fn with_requested_attempts(mut self, requested_attempts: u8) -> Self {
        self.requested_attempts = requested_attempts;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelayerDryRunError {
    ReceiptCapacityReached,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerDryRun {
    config: RelayerConfig,
    seen_idempotency_keys: BTreeSet<IdempotencyKey>,
    receipts: Vec<RelayerReceipt>,
}

impl RelayerDryRun {
    pub fn new(config: RelayerConfig) -> Self {
        Self {
            config,
            seen_idempotency_keys: BTreeSet::new(),
            receipts: Vec::new(),
        }
    }

    pub fn submit_dry_run(
        &mut self,
        request: RelayerSubmissionRequest,
    ) -> Result<RelayerReceipt, RelayerDryRunError> {
        if self.receipts.len() >= self.config.max_receipts {
            return Err(RelayerDryRunError::ReceiptCapacityReached);
        }

        let status = self.status_for_request(&request);
        let attempts_used = self.attempts_for_status(status, request.requested_attempts);

        if status == RelayerReceiptStatus::DryRunAccepted {
            self.seen_idempotency_keys
                .insert(request.idempotency_key.clone());
        }

        let receipt = RelayerReceipt::new(
            request.operation_id,
            request.idempotency_key,
            request.target,
            status,
            request.proof_review.decision,
            attempts_used,
        );

        self.receipts.push(receipt.clone());

        Ok(receipt)
    }

    pub fn receipts(&self) -> &[RelayerReceipt] {
        &self.receipts
    }

    fn status_for_request(&self, request: &RelayerSubmissionRequest) -> RelayerReceiptStatus {
        if self
            .seen_idempotency_keys
            .contains(&request.idempotency_key)
        {
            return RelayerReceiptStatus::DuplicateRequest;
        }

        match request.proof_review.decision {
            ReviewDecision::Accepted => RelayerReceiptStatus::DryRunAccepted,
            ReviewDecision::Blocked => RelayerReceiptStatus::ProofBlocked,
            ReviewDecision::Rejected => RelayerReceiptStatus::ProofRejected,
        }
    }

    fn attempts_for_status(&self, status: RelayerReceiptStatus, requested_attempts: u8) -> u8 {
        if status != RelayerReceiptStatus::DryRunAccepted {
            return 0;
        }

        RetryPolicy::new(self.config.max_attempts)
            .plan_attempts(requested_attempts)
            .allowed_attempts
    }
}
