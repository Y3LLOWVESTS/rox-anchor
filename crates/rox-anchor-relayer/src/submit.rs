//! RO:WHAT — Local dry-run, simulation, and capped testnet submission authorization model.
//! RO:WHY — Gates every submit-shaped path through proof, coordinator, dry-run, simulation, caps, and receipts.
//! RO:INTERACTS — rox-anchor-proof ProofReview, retry policy, receipts, config, and coordinator simulation gate.
//! RO:INVARIANTS — duplicate idempotency is rejected; blocked/rejected proof reviews cannot simulate or submit.
//! RO:SECURITY — no RPC, wallet, key loading, transaction send, mint, burn, settlement, staking, or liquidity.
//! RO:TEST — covered by accepted, blocked, duplicate, capacity, retry, simulation, and capped-submit tests.

use std::collections::BTreeSet;

use rox_anchor_core::{
    AnchorOperationalBlocker, AnchorOperationalPosture, IdempotencyKey, OperationId, SubmissionMode,
};
use rox_anchor_proof::{ProofReview, ReviewDecision};

use crate::{RelayerConfig, RelayerReceipt, RelayerReceiptStatus, RetryPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerSubmissionRequest {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub proof_review: ProofReview,
    pub requested_attempts: u8,
    pub operational_posture: AnchorOperationalPosture,
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
            operational_posture: AnchorOperationalPosture::clear(),
        }
    }

    pub fn with_requested_attempts(mut self, requested_attempts: u8) -> Self {
        self.requested_attempts = requested_attempts;
        self
    }

    pub fn with_operational_posture(mut self, posture: AnchorOperationalPosture) -> Self {
        self.operational_posture = posture;
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
        match request.operational_posture.primary_blocker() {
            AnchorOperationalBlocker::Challenge => return RelayerReceiptStatus::ChallengeBlocked,
            AnchorOperationalBlocker::Halt => return RelayerReceiptStatus::Halted,
            AnchorOperationalBlocker::Recovery => return RelayerReceiptStatus::RecoveryBlocked,
            AnchorOperationalBlocker::None => {}
        }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionSimulationPlan {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub dry_run_receipt: RelayerReceipt,
    pub coordinator_accepted: bool,
    pub instruction_count: u16,
}

impl TransactionSimulationPlan {
    pub fn from_dry_run_receipt(
        dry_run_receipt: RelayerReceipt,
        coordinator_accepted: bool,
        instruction_count: u16,
    ) -> Self {
        let operation_id = dry_run_receipt.operation_id.clone();
        let idempotency_key = dry_run_receipt.idempotency_key.clone();
        let target = dry_run_receipt.target.clone();

        Self {
            operation_id,
            idempotency_key,
            target,
            dry_run_receipt,
            coordinator_accepted,
            instruction_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionSimulationStatus {
    Simulated,
    UnsafeScope,
    EmptyInstructionPlan,
    CoordinatorNotAccepted,
    ProofNotAccepted,
    RelayerDryRunNotAccepted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionSimulationResult {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub status: TransactionSimulationStatus,
    pub proof_decision: ReviewDecision,
    pub relayer_status: RelayerReceiptStatus,
    pub instruction_count: u16,
    pub simulated: bool,
    pub live_submission: bool,
}

impl TransactionSimulationResult {
    pub fn is_simulated(&self) -> bool {
        self.status == TransactionSimulationStatus::Simulated
    }
}

pub fn simulate_transaction_plan(
    config: RelayerConfig,
    plan: TransactionSimulationPlan,
) -> TransactionSimulationResult {
    let status = simulation_status(config, &plan);

    TransactionSimulationResult {
        operation_id: plan.operation_id,
        idempotency_key: plan.idempotency_key,
        target: plan.target,
        status,
        proof_decision: plan.dry_run_receipt.proof_decision,
        relayer_status: plan.dry_run_receipt.status,
        instruction_count: plan.instruction_count,
        simulated: status == TransactionSimulationStatus::Simulated,
        live_submission: false,
    }
}

fn simulation_status(
    config: RelayerConfig,
    plan: &TransactionSimulationPlan,
) -> TransactionSimulationStatus {
    if config.safety.validate().is_err() || !config.safety.submission_mode.is_non_submitting() {
        return TransactionSimulationStatus::UnsafeScope;
    }

    if plan.instruction_count == 0 {
        return TransactionSimulationStatus::EmptyInstructionPlan;
    }

    if !plan.coordinator_accepted {
        return TransactionSimulationStatus::CoordinatorNotAccepted;
    }

    if plan.dry_run_receipt.proof_decision != ReviewDecision::Accepted {
        return TransactionSimulationStatus::ProofNotAccepted;
    }

    if plan.dry_run_receipt.status != RelayerReceiptStatus::DryRunAccepted {
        return TransactionSimulationStatus::RelayerDryRunNotAccepted;
    }

    TransactionSimulationStatus::Simulated
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CappedTestnetSubmissionLimits {
    pub max_attempts: u8,
    pub max_operations_per_run: u16,
    pub max_amount_units: u64,
    pub require_persisted_receipt: bool,
}

impl CappedTestnetSubmissionLimits {
    pub fn new(
        max_attempts: u8,
        max_operations_per_run: u16,
        max_amount_units: u64,
        require_persisted_receipt: bool,
    ) -> Self {
        Self {
            max_attempts,
            max_operations_per_run,
            max_amount_units,
            require_persisted_receipt,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CappedTestnetSubmissionPlan {
    pub simulation_result: TransactionSimulationResult,
    pub requested_attempts: u8,
    pub requested_operations: u16,
    pub amount_units: u64,
    pub explicit_operator_approval: bool,
    pub receipt_persisted: bool,
}

impl CappedTestnetSubmissionPlan {
    pub fn from_simulation_result(simulation_result: TransactionSimulationResult) -> Self {
        Self {
            simulation_result,
            requested_attempts: 1,
            requested_operations: 1,
            amount_units: 1,
            explicit_operator_approval: false,
            receipt_persisted: false,
        }
    }

    pub fn with_requested_attempts(mut self, requested_attempts: u8) -> Self {
        self.requested_attempts = requested_attempts;
        self
    }

    pub fn with_requested_operations(mut self, requested_operations: u16) -> Self {
        self.requested_operations = requested_operations;
        self
    }

    pub fn with_amount_units(mut self, amount_units: u64) -> Self {
        self.amount_units = amount_units;
        self
    }

    pub fn with_explicit_operator_approval(mut self, explicit_operator_approval: bool) -> Self {
        self.explicit_operator_approval = explicit_operator_approval;
        self
    }

    pub fn with_receipt_persisted(mut self, receipt_persisted: bool) -> Self {
        self.receipt_persisted = receipt_persisted;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CappedTestnetSubmissionStatus {
    Authorized,
    UnsafeScope,
    SimulationNotAccepted,
    MissingExplicitOperatorApproval,
    EmptyOperationRun,
    RetryCapExceeded,
    OperationCapExceeded,
    AmountCapExceeded,
    ReceiptPersistenceMissing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CappedTestnetSubmissionResult {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub status: CappedTestnetSubmissionStatus,
    pub proof_decision: ReviewDecision,
    pub relayer_status: RelayerReceiptStatus,
    pub simulation_status: TransactionSimulationStatus,
    pub requested_attempts: u8,
    pub requested_operations: u16,
    pub amount_units: u64,
    pub authorized: bool,
    pub live_submission_permitted: bool,
    pub live_submission_attempted: bool,
    pub network_submitted: bool,
}

impl CappedTestnetSubmissionResult {
    pub fn is_authorized(&self) -> bool {
        self.status == CappedTestnetSubmissionStatus::Authorized
    }
}

pub fn authorize_capped_testnet_submission(
    config: RelayerConfig,
    limits: CappedTestnetSubmissionLimits,
    plan: CappedTestnetSubmissionPlan,
) -> CappedTestnetSubmissionResult {
    let status = capped_submission_status(config, limits, &plan);
    let authorized = status == CappedTestnetSubmissionStatus::Authorized;

    CappedTestnetSubmissionResult {
        operation_id: plan.simulation_result.operation_id,
        idempotency_key: plan.simulation_result.idempotency_key,
        target: plan.simulation_result.target,
        status,
        proof_decision: plan.simulation_result.proof_decision,
        relayer_status: plan.simulation_result.relayer_status,
        simulation_status: plan.simulation_result.status,
        requested_attempts: plan.requested_attempts,
        requested_operations: plan.requested_operations,
        amount_units: plan.amount_units,
        authorized,
        live_submission_permitted: authorized,
        live_submission_attempted: false,
        network_submitted: false,
    }
}

fn capped_submission_status(
    config: RelayerConfig,
    limits: CappedTestnetSubmissionLimits,
    plan: &CappedTestnetSubmissionPlan,
) -> CappedTestnetSubmissionStatus {
    if config.safety.validate().is_err()
        || config.safety.submission_mode != SubmissionMode::TestnetSubmitCapped
    {
        return CappedTestnetSubmissionStatus::UnsafeScope;
    }

    if !plan.simulation_result.is_simulated()
        || plan.simulation_result.live_submission
        || plan.simulation_result.proof_decision != ReviewDecision::Accepted
        || plan.simulation_result.relayer_status != RelayerReceiptStatus::DryRunAccepted
    {
        return CappedTestnetSubmissionStatus::SimulationNotAccepted;
    }

    if !plan.explicit_operator_approval {
        return CappedTestnetSubmissionStatus::MissingExplicitOperatorApproval;
    }

    if plan.requested_operations == 0 {
        return CappedTestnetSubmissionStatus::EmptyOperationRun;
    }

    if plan.requested_attempts == 0 || plan.requested_attempts > limits.max_attempts {
        return CappedTestnetSubmissionStatus::RetryCapExceeded;
    }

    if plan.requested_operations > limits.max_operations_per_run {
        return CappedTestnetSubmissionStatus::OperationCapExceeded;
    }

    if plan.amount_units == 0 || plan.amount_units > limits.max_amount_units {
        return CappedTestnetSubmissionStatus::AmountCapExceeded;
    }

    if limits.require_persisted_receipt && !plan.receipt_persisted {
        return CappedTestnetSubmissionStatus::ReceiptPersistenceMissing;
    }

    CappedTestnetSubmissionStatus::Authorized
}
