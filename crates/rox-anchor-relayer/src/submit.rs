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

use crate::{
    RelayerConfig, RelayerPrivatePilotConfig, RelayerReceipt, RelayerReceiptStatus, RetryPolicy,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivatePilotTransactionKind {
    Initialize,
    Observe,
    OpenChallenge,
    ResolveChallenge,
    Halt,
    Recover,
    Finalize,
}

impl PrivatePilotTransactionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::Observe => "observe",
            Self::OpenChallenge => "open_challenge",
            Self::ResolveChallenge => "resolve_challenge",
            Self::Halt => "halt",
            Self::Recover => "recover",
            Self::Finalize => "finalize",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivatePilotTransactionStep {
    pub kind: PrivatePilotTransactionKind,
    pub label: String,
    pub instruction_count: u16,
}

impl PrivatePilotTransactionStep {
    pub fn new(
        kind: PrivatePilotTransactionKind,
        label: impl Into<String>,
        instruction_count: u16,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            instruction_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivatePilotSimulationPlan {
    pub base_plan: TransactionSimulationPlan,
    pub read_only_rpc_verified: bool,
    pub steps: Vec<PrivatePilotTransactionStep>,
}

impl PrivatePilotSimulationPlan {
    pub fn new(base_plan: TransactionSimulationPlan) -> Self {
        Self {
            base_plan,
            read_only_rpc_verified: false,
            steps: Vec::new(),
        }
    }

    pub fn with_read_only_rpc_verified(mut self, verified: bool) -> Self {
        self.read_only_rpc_verified = verified;
        self
    }

    pub fn with_steps(mut self, steps: Vec<PrivatePilotTransactionStep>) -> Self {
        self.steps = steps;
        self
    }

    pub fn planned_instruction_count(&self) -> u16 {
        self.steps
            .iter()
            .map(|step| step.instruction_count)
            .sum::<u16>()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivatePilotSimulationStatus {
    Simulated,
    ReadOnlyRpcNotVerified,
    MissingTransactionSteps,
    InstructionCountMismatch,
    UnsafeScope,
    EmptyInstructionPlan,
    CoordinatorNotAccepted,
    ProofNotAccepted,
    RelayerDryRunNotAccepted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivatePilotSimulationResult {
    pub status: PrivatePilotSimulationStatus,
    pub base_result: Option<TransactionSimulationResult>,
    pub read_only_rpc_verified: bool,
    pub step_count: usize,
    pub planned_instruction_count: u16,
    pub live_submission: bool,
    pub wallet_key_loading: bool,
    pub internal_roc_mutation: bool,
}

impl PrivatePilotSimulationResult {
    pub fn is_simulated(&self) -> bool {
        self.status == PrivatePilotSimulationStatus::Simulated
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        let mut lines = vec![
            "private_pilot_simulation: local_only".to_string(),
            format!("status: {:?}", self.status),
            format!("simulated: {}", self.is_simulated()),
            format!("read_only_rpc_verified: {}", self.read_only_rpc_verified),
            format!("step_count: {}", self.step_count),
            format!(
                "planned_instruction_count: {}",
                self.planned_instruction_count
            ),
            format!("live_submission: {}", self.live_submission),
            "network_submission: disabled".to_string(),
            format!("wallet_key_loading: {}", self.wallet_key_loading),
            format!("internal_roc_mutation: {}", self.internal_roc_mutation),
            "settlement_claim: none".to_string(),
        ];

        if let Some(base) = self.base_result.as_ref() {
            lines.push(format!("operation_id: {}", base.operation_id));
            lines.push(format!("target: {}", base.target));
            lines.push(format!("base_simulation_status: {:?}", base.status));
            lines.push(format!("proof_decision: {:?}", base.proof_decision));
            lines.push(format!("relayer_status: {:?}", base.relayer_status));
            lines.push(format!("base_live_submission: {}", base.live_submission));
        }

        lines
    }
}

pub fn simulate_private_pilot_transaction_plan(
    config: RelayerConfig,
    plan: PrivatePilotSimulationPlan,
) -> PrivatePilotSimulationResult {
    let step_count = plan.steps.len();
    let planned_instruction_count = plan.planned_instruction_count();

    if !plan.read_only_rpc_verified {
        return private_pilot_simulation_result(
            PrivatePilotSimulationStatus::ReadOnlyRpcNotVerified,
            None,
            false,
            step_count,
            planned_instruction_count,
        );
    }

    if plan.steps.is_empty() || planned_instruction_count == 0 {
        return private_pilot_simulation_result(
            PrivatePilotSimulationStatus::MissingTransactionSteps,
            None,
            true,
            step_count,
            planned_instruction_count,
        );
    }

    if planned_instruction_count != plan.base_plan.instruction_count {
        return private_pilot_simulation_result(
            PrivatePilotSimulationStatus::InstructionCountMismatch,
            None,
            true,
            step_count,
            planned_instruction_count,
        );
    }

    let base_result = simulate_transaction_plan(config, plan.base_plan);
    let status = match base_result.status {
        TransactionSimulationStatus::Simulated => PrivatePilotSimulationStatus::Simulated,
        TransactionSimulationStatus::UnsafeScope => PrivatePilotSimulationStatus::UnsafeScope,
        TransactionSimulationStatus::EmptyInstructionPlan => {
            PrivatePilotSimulationStatus::EmptyInstructionPlan
        }
        TransactionSimulationStatus::CoordinatorNotAccepted => {
            PrivatePilotSimulationStatus::CoordinatorNotAccepted
        }
        TransactionSimulationStatus::ProofNotAccepted => {
            PrivatePilotSimulationStatus::ProofNotAccepted
        }
        TransactionSimulationStatus::RelayerDryRunNotAccepted => {
            PrivatePilotSimulationStatus::RelayerDryRunNotAccepted
        }
    };

    private_pilot_simulation_result(
        status,
        Some(base_result),
        true,
        step_count,
        planned_instruction_count,
    )
}

fn private_pilot_simulation_result(
    status: PrivatePilotSimulationStatus,
    base_result: Option<TransactionSimulationResult>,
    read_only_rpc_verified: bool,
    step_count: usize,
    planned_instruction_count: u16,
) -> PrivatePilotSimulationResult {
    PrivatePilotSimulationResult {
        status,
        base_result,
        read_only_rpc_verified,
        step_count,
        planned_instruction_count,
        live_submission: false,
        wallet_key_loading: false,
        internal_roc_mutation: false,
    }
}

pub const PRIVATE_TESTNET_CAPPED_SEND_APPROVAL: &str = "I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivateTestnetSenderStatus {
    Authorized,
    MissingExternalConfig,
    UnsafeExternalConfig,
    MissingReceiptOutputPath,
    PendingOperationalBlocker,
    MissingOperatorApproval,
    SimulationNotAccepted,
    CappedAuthorizationRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateTestnetSenderRequest {
    pub external_config: Option<RelayerPrivatePilotConfig>,
    pub simulation_result: PrivatePilotSimulationResult,
    pub limits: CappedTestnetSubmissionLimits,
    pub requested_attempts: u8,
    pub requested_operations: u16,
    pub amount_units: u64,
    pub operator_approval: Option<String>,
    pub receipt_output_path_declared: bool,
    pub operational_posture: AnchorOperationalPosture,
}

impl PrivateTestnetSenderRequest {
    pub fn new(simulation_result: PrivatePilotSimulationResult) -> Self {
        Self {
            external_config: None,
            simulation_result,
            limits: CappedTestnetSubmissionLimits::new(2, 2, 100, true),
            requested_attempts: 1,
            requested_operations: 1,
            amount_units: 1,
            operator_approval: None,
            receipt_output_path_declared: false,
            operational_posture: AnchorOperationalPosture::clear(),
        }
    }

    pub fn with_external_config(mut self, external_config: RelayerPrivatePilotConfig) -> Self {
        self.external_config = Some(external_config);
        self
    }

    pub fn with_limits(mut self, limits: CappedTestnetSubmissionLimits) -> Self {
        self.limits = limits;
        self
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

    pub fn with_operator_approval(mut self, operator_approval: impl Into<String>) -> Self {
        self.operator_approval = Some(operator_approval.into());
        self
    }

    pub fn with_receipt_output_path_declared(mut self, receipt_output_path_declared: bool) -> Self {
        self.receipt_output_path_declared = receipt_output_path_declared;
        self
    }

    pub fn with_operational_posture(
        mut self,
        operational_posture: AnchorOperationalPosture,
    ) -> Self {
        self.operational_posture = operational_posture;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateTestnetSenderAuthorization {
    pub status: PrivateTestnetSenderStatus,
    pub capped_result: Option<CappedTestnetSubmissionResult>,
    pub authorized: bool,
    pub live_submission_permitted: bool,
    pub live_submission_attempted: bool,
    pub network_submitted: bool,
    pub wallet_key_loading: bool,
    pub signing: bool,
    pub external_config_validated: bool,
    pub receipt_output_path_redacted: Option<String>,
}

impl PrivateTestnetSenderAuthorization {
    pub fn is_authorized(&self) -> bool {
        self.status == PrivateTestnetSenderStatus::Authorized
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        let mut lines = vec![
            "private_testnet_sender: explicit_capped_authorization".to_string(),
            format!("status: {:?}", self.status),
            format!("authorized: {}", self.authorized),
            format!(
                "live_submission_permitted: {}",
                self.live_submission_permitted
            ),
            format!(
                "live_submission_attempted: {}",
                self.live_submission_attempted
            ),
            format!("network_submitted: {}", self.network_submitted),
            format!("wallet_key_loading: {}", self.wallet_key_loading),
            format!("signing: {}", self.signing),
            format!(
                "external_config_validated: {}",
                self.external_config_validated
            ),
            format!(
                "receipt_output_path: {}",
                self.receipt_output_path_redacted
                    .clone()
                    .unwrap_or_else(|| "<missing>".to_owned())
            ),
            "rpc_submission: disabled_in_local_authorization_model".to_string(),
            "mint_burn_execution: disabled_in_local_authorization_model".to_string(),
            "internal_roc_mutation: disabled".to_string(),
            "settlement_claim: none".to_string(),
        ];

        if let Some(capped) = self.capped_result.as_ref() {
            lines.extend([
                format!("capped_submit_status: {:?}", capped.status),
                format!("proof_decision: {:?}", capped.proof_decision),
                format!("relayer_status: {:?}", capped.relayer_status),
                format!("simulation_status: {:?}", capped.simulation_status),
                format!("requested_attempts: {}", capped.requested_attempts),
                format!("requested_operations: {}", capped.requested_operations),
                format!("amount_units: {}", capped.amount_units),
            ]);
        }

        lines
    }
}

pub fn authorize_private_testnet_sender(
    request: PrivateTestnetSenderRequest,
) -> PrivateTestnetSenderAuthorization {
    let Some(external_config) = request.external_config else {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::MissingExternalConfig,
            None,
            false,
            false,
            None,
        );
    };

    let redacted_config = external_config.redacted_report();
    let receipt_output_path = Some(redacted_config.receipt_output_path.clone());

    if external_config.validate().is_err()
        || external_config.relayer.safety.submission_mode != SubmissionMode::TestnetSubmitCapped
        || external_config.pilot.testnet.submission_mode != SubmissionMode::TestnetSubmitCapped
    {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::UnsafeExternalConfig,
            None,
            false,
            false,
            receipt_output_path,
        );
    }

    if !request.receipt_output_path_declared {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::MissingReceiptOutputPath,
            None,
            true,
            false,
            receipt_output_path,
        );
    }

    if request.operational_posture.blocks_submission() {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::PendingOperationalBlocker,
            None,
            true,
            false,
            receipt_output_path,
        );
    }

    if request.operator_approval.as_deref() != Some(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL) {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::MissingOperatorApproval,
            None,
            true,
            false,
            receipt_output_path,
        );
    }

    if !request.simulation_result.is_simulated()
        || request.simulation_result.live_submission
        || !request.simulation_result.read_only_rpc_verified
    {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::SimulationNotAccepted,
            None,
            true,
            false,
            receipt_output_path,
        );
    }

    let Some(base_simulation) = request.simulation_result.base_result else {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::SimulationNotAccepted,
            None,
            true,
            false,
            receipt_output_path,
        );
    };

    if !base_simulation.is_simulated() || base_simulation.live_submission {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::SimulationNotAccepted,
            None,
            true,
            false,
            receipt_output_path,
        );
    }

    let capped_plan = CappedTestnetSubmissionPlan::from_simulation_result(base_simulation)
        .with_requested_attempts(request.requested_attempts)
        .with_requested_operations(request.requested_operations)
        .with_amount_units(request.amount_units)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let capped_result =
        authorize_capped_testnet_submission(external_config.relayer, request.limits, capped_plan);

    if !capped_result.is_authorized() {
        return private_testnet_sender_authorization(
            PrivateTestnetSenderStatus::CappedAuthorizationRejected,
            Some(capped_result),
            true,
            false,
            receipt_output_path,
        );
    }

    private_testnet_sender_authorization(
        PrivateTestnetSenderStatus::Authorized,
        Some(capped_result),
        true,
        true,
        receipt_output_path,
    )
}

fn private_testnet_sender_authorization(
    status: PrivateTestnetSenderStatus,
    capped_result: Option<CappedTestnetSubmissionResult>,
    external_config_validated: bool,
    live_submission_permitted: bool,
    receipt_output_path_redacted: Option<String>,
) -> PrivateTestnetSenderAuthorization {
    let authorized = status == PrivateTestnetSenderStatus::Authorized;

    PrivateTestnetSenderAuthorization {
        status,
        capped_result,
        authorized,
        live_submission_permitted,
        live_submission_attempted: false,
        network_submitted: false,
        wallet_key_loading: false,
        signing: false,
        external_config_validated,
        receipt_output_path_redacted,
    }
}
