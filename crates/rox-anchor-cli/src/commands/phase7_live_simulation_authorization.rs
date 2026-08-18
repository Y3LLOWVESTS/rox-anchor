//! BUILD_PLAN4 Phase 7B.
//!
//! Reuses the exact Phase 7A ROC-to-ROX candidate, re-establishes fresh
//! read-only/proof/coordinator/relayer evidence, simulates that exact
//! transaction on Solana Devnet, proves simulation produced no persistent
//! account mutation, and then asks the existing capped private-testnet sender
//! model whether that operation is authorized.
//!
//! This module has no keypair loader, signing path, or transaction submission
//! API. An Authorized sender result means the later execution boundary may be
//! built; it does not mean a transaction was sent.

#![forbid(unsafe_code)]

use std::{fs, path::Path};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, message::Message, signature::Signature,
        transaction::Transaction,
    },
};
use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorOperationalPosture, AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId,
    DomainId, HaltPosture, IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture,
    SubmissionMode, TokenAccountId,
};
use rox_anchor_proof::{
    EvidenceBundle, ExpectedProofBinding, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_relayer::{
    authorize_private_testnet_sender, simulate_private_pilot_transaction_plan,
    CappedTestnetSubmissionLimits, PrivatePilotSimulationPlan, PrivatePilotSimulationResult,
    PrivatePilotSimulationStatus, PrivatePilotTransactionKind, PrivatePilotTransactionStep,
    PrivateTestnetSenderRequest, PrivateTestnetSenderStatus, RelayerConfig, RelayerDryRun,
    RelayerPrivatePilotConfig, RelayerReceiptStatus, RelayerSubmissionRequest,
    TransactionSimulationPlan, PRIVATE_TESTNET_CAPPED_SEND_APPROVAL,
};
use rox_anchor_rpc_proof::{ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation};
use serde_json::{json, Value};

use crate::{
    commands::{
        phase6_live_simulation::{
            validate_phase5_receipt, Phase5Evidence, PHASE4_INITIALIZATION_SIGNATURE,
            PHASE6_REQUIRED_OBSERVATIONS, PHASE6_SOURCE1, PHASE6_SOURCE2, PHASE6_STALE_AFTER_SLOTS,
        },
        phase7_live_capped_sender::{
            build_phase7_capped_roc_to_rox_plan, validate_phase6_forward_receipt,
            Phase7CappedRocToRoxPlan, PHASE7_AMOUNT_MINOR, PHASE7_IDEMPOTENCY_KEY,
            PHASE7_MAX_AMOUNT_MINOR, PHASE7_MAX_OPERATIONS, PHASE7_NONCE, PHASE7_OPERATION_ID,
            PHASE7_OPERATOR_APPROVAL, PHASE7_PROGRAM_ID, PHASE7_RETRY_CAP,
            PHASE7_TEST_ONLY_ROX_MINT, PHASE7_TEST_ONLY_TOKEN_ACCOUNT,
        },
    },
    CliError,
};

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

const PHASE7B_SCHEMA: &str = "rox-anchor.phase7-simulation-authorization.v1";

#[derive(Clone, Debug, Default)]
struct Phase7BArgs {
    config_path: Option<String>,
    phase5_receipt_path: Option<String>,
    phase6_receipt_path: Option<String>,
    authorization_receipt_out: Option<String>,
    operator_approval: Option<String>,
    max_operations: Option<u16>,
    max_amount_minor: Option<u64>,
    retry_cap: Option<u8>,
    simulate_and_authorize_only: bool,
}

#[derive(Clone, Debug)]
struct PersistentStateSnapshot {
    config_data: Vec<u8>,
    mint_data: Vec<u8>,
    token_data: Vec<u8>,
}

#[derive(Clone, Debug)]
struct LiveSimulationEvidence {
    context_slot: u64,
    log_count: usize,
}

pub fn run_phase7_simulate_and_authorize(args: &[String]) -> Result<String, CliError> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        return Ok(help_text());
    }

    let args = parse_args(args)?;

    if !args.simulate_and_authorize_only {
        return Err(phase7b_error("--simulate-and-authorize-only is required"));
    }

    require_exact_caps(&args)?;

    if args.operator_approval.as_deref() != Some(PHASE7_OPERATOR_APPROVAL) {
        return Err(phase7b_error("exact Phase 7 operator approval is required"));
    }

    let config_path = required_arg(args.config_path.as_deref(), "--config")?;

    let phase5_path = required_arg(args.phase5_receipt_path.as_deref(), "--phase5-receipt")?;

    let phase6_path = required_arg(args.phase6_receipt_path.as_deref(), "--phase6-receipt")?;

    let authorization_receipt_out = required_arg(
        args.authorization_receipt_out.as_deref(),
        "--authorization-receipt-out",
    )?;

    require_ignored_or_absolute_path(config_path, "--config")?;

    require_ignored_or_absolute_path(phase5_path, "--phase5-receipt")?;

    require_ignored_or_absolute_path(phase6_path, "--phase6-receipt")?;

    require_ignored_or_absolute_path(authorization_receipt_out, "--authorization-receipt-out")?;

    let authorization_receipt_path = Path::new(authorization_receipt_out);

    if authorization_receipt_path.exists() {
        return Err(phase7b_error("authorization receipt already exists"));
    }

    let phase6_text = fs::read_to_string(phase6_path)
        .map_err(|_| phase7b_error("could not read Phase 6 forward simulation receipt"))?;

    let phase6_receipt: Value = serde_json::from_str(&phase6_text)
        .map_err(|_| phase7b_error("Phase 6 receipt is not valid JSON"))?;

    validate_phase6_forward_receipt(&phase6_receipt)?;

    let phase5_evidence = validate_phase5_receipt(Path::new(phase5_path))?;

    let config_text = fs::read_to_string(config_path)
        .map_err(|_| phase7b_error("could not read Phase 7 capped-submit config"))?;

    let submit_config = build_submit_config(&config_text)?;

    let candidate = build_phase7_capped_roc_to_rox_plan()?;

    if !candidate.is_exact_phase7_shape() {
        return Err(phase7b_error(
            "Phase 7A exact transaction candidate failed revalidation",
        ));
    }

    let rpc =
        RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), CommitmentConfig::confirmed());

    let live_slot = rpc
        .get_slot()
        .map_err(|error| phase7b_error(format!("could not query current Devnet slot: {error}")))?;

    let review_slot = verify_phase5_freshness(&phase5_evidence, live_slot)?;

    let local_simulation = build_local_phase7_simulation(&phase5_evidence, review_slot)?;

    let before = capture_persistent_state(&rpc, &candidate, true)?;

    let live_simulation = simulate_exact_candidate(&rpc, &candidate)?;

    let after = capture_persistent_state(&rpc, &candidate, true)?;

    if before.config_data != after.config_data {
        return Err(phase7b_error("config bytes changed after simulation"));
    }

    if before.mint_data != after.mint_data {
        return Err(phase7b_error("mint bytes changed after simulation"));
    }

    if before.token_data != after.token_data {
        return Err(phase7b_error(
            "token-account bytes changed after simulation",
        ));
    }

    let sender = authorize_private_testnet_sender(
        PrivateTestnetSenderRequest::new(local_simulation)
            .with_external_config(submit_config)
            .with_limits(CappedTestnetSubmissionLimits::new(
                PHASE7_RETRY_CAP,
                PHASE7_MAX_OPERATIONS,
                PHASE7_MAX_AMOUNT_MINOR,
                true,
            ))
            .with_requested_attempts(PHASE7_RETRY_CAP)
            .with_requested_operations(PHASE7_MAX_OPERATIONS)
            .with_amount_units(PHASE7_AMOUNT_MINOR)
            // Phase 7 validates the BUILD_PLAN4 user-facing SEND phrase
            // first. The existing relayer sender retains its older internal
            // SUBMIT approval vocabulary; translating here preserves that
            // existing tested contract without weakening the outer gate.
            .with_operator_approval(PRIVATE_TESTNET_CAPPED_SEND_APPROVAL)
            .with_receipt_output_path_declared(true)
            .with_operational_posture(AnchorOperationalPosture::clear()),
    );

    if sender.status != PrivateTestnetSenderStatus::Authorized
        || !sender.authorized
        || !sender.live_submission_permitted
        || sender.live_submission_attempted
        || sender.network_submitted
        || sender.wallet_key_loading
        || sender.signing
    {
        return Err(phase7b_error(format!(
            "existing capped sender authorization rejected or crossed execution boundary: {:?}",
            sender.status
        )));
    }

    write_authorization_receipt(
        authorization_receipt_path,
        review_slot,
        &candidate,
        &live_simulation,
        &sender,
    )?;

    Ok([
        "phase7b_exact_simulation_authorization: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 7B".to_string(),
        "cluster: devnet".to_string(),
        "direction: roc_to_rox".to_string(),
        "phase5_freshness: GREEN".to_string(),
        format!(
            "phase5_source_slot_delta: {}",
            phase5_evidence.metadata_slot_delta,
        ),
        format!("phase7_review_slot: {review_slot}",),
        "phase6_forward_receipt: verified_non_promotable".to_string(),
        "exact_phase7a_candidate_reused: true".to_string(),
        "candidate_instruction_count: 2".to_string(),
        "candidate_instruction_1: observe_burn".to_string(),
        "candidate_instruction_2: finalize_roc_to_rox_mint".to_string(),
        "candidate_amount_minor: 1".to_string(),
        "local_proof_coordinator_relayer_gate: GREEN".to_string(),
        "live_devnet_exact_candidate_simulation: GREEN".to_string(),
        format!(
            "live_simulation_context_slot: {}",
            live_simulation.context_slot,
        ),
        format!("live_simulation_log_count: {}", live_simulation.log_count,),
        "persistent_operation_after_simulation: false".to_string(),
        "persistent_config_change_after_simulation: false".to_string(),
        "persistent_mint_change_after_simulation: false".to_string(),
        "persistent_token_account_change_after_simulation: false".to_string(),
        "simulated_value_instruction_amount_minor: 1".to_string(),
        "simulated_account_delta_claim: not_invented".to_string(),
        "existing_private_sender_status: Authorized".to_string(),
        "existing_private_sender_live_submission_permitted: true".to_string(),
        "existing_private_sender_live_submission_attempted: false".to_string(),
        "existing_private_sender_network_submitted: false".to_string(),
        "existing_private_sender_wallet_key_loading: false".to_string(),
        "existing_private_sender_signing: false".to_string(),
        "approval_boundary_user_facing: I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND".to_string(),
        "approval_boundary_existing_relayer_internal: I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT"
            .to_string(),
        "keypair_loading: false".to_string(),
        "signature_generation: false".to_string(),
        "transaction_submission: false".to_string(),
        "rox_mint_persisted: false".to_string(),
        "real_roc_burn: false".to_string(),
        "real_roc_mutation: false".to_string(),
        format!(
            "authorization_receipt: {}",
            redact_path(authorization_receipt_path,),
        ),
        "next_action: PHASE7C_BUILD_LIVE_SIGNED_EXECUTOR_WITHOUT_RUNNING_IT".to_string(),
    ]
    .join("\n"))
}

fn build_local_phase7_simulation(
    evidence: &Phase5Evidence,
    review_slot: u64,
) -> Result<PrivatePilotSimulationResult, CliError> {
    let operation_id = core_id(OperationId::new(PHASE7_OPERATION_ID), "operation-id")?;

    let idempotency_key = core_id(
        IdempotencyKey::new(PHASE7_IDEMPOTENCY_KEY),
        "idempotency-key",
    )?;

    let nonce = core_id(Nonce::new(PHASE7_NONCE), "nonce")?;

    let binding = AnchorBinding::new(
        core_id(
            DomainId::new("internal-roc-private-pilot-test"),
            "source-domain",
        )?,
        core_id(
            DomainId::new("solana-devnet-rox-private-pilot-test"),
            "destination-domain",
        )?,
        AnchorDirection::RocToRox,
        core_id(ClusterId::new("devnet"), "cluster")?,
        core_id(ProgramId::new(PHASE7_PROGRAM_ID), "program-id")?,
        core_id(MintId::new(PHASE7_TEST_ONLY_ROX_MINT), "mint")?,
        core_id(
            TokenAccountId::new(PHASE7_TEST_ONLY_TOKEN_ACCOUNT),
            "token-account",
        )?,
    );

    let expected = ExpectedProofBinding::new(
        binding.clone(),
        operation_id.clone(),
        idempotency_key.clone(),
        nonce.clone(),
    );

    let package = ProofPackage::new(
        binding,
        operation_id.clone(),
        idempotency_key.clone(),
        nonce,
        core_id(
            AccountId::new("shadow-roc-burn-source-phase7"),
            "source-account",
        )?,
        core_id(
            AccountId::new("actual-private-rox-recipient-phase7"),
            "recipient-account",
        )?,
        EvidenceBundle::satisfied(PHASE6_REQUIRED_OBSERVATIONS),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );

    let expected_rpc = ExpectedRpcBinding::new(
        core_id(ClusterId::new("devnet"), "rpc-cluster")?,
        core_id(ProgramId::new(PHASE7_PROGRAM_ID), "rpc-program")?,
        core_id(MintId::new(PHASE7_TEST_ONLY_ROX_MINT), "rpc-mint")?,
        core_id(
            TokenAccountId::new(PHASE7_TEST_ONLY_TOKEN_ACCOUNT),
            "rpc-token-account",
        )?,
        operation_id.clone(),
        RpcCommitmentLevel::Confirmed,
    );

    let observations = vec![
        RpcObservation::new(
            PHASE6_SOURCE1,
            core_id(ClusterId::new("devnet"), "source1-cluster")?,
            core_id(ProgramId::new(PHASE7_PROGRAM_ID), "source1-program")?,
            core_id(MintId::new(PHASE7_TEST_ONLY_ROX_MINT), "source1-mint")?,
            core_id(
                TokenAccountId::new(PHASE7_TEST_ONLY_TOKEN_ACCOUNT),
                "source1-token",
            )?,
            operation_id.clone(),
            PHASE4_INITIALIZATION_SIGNATURE,
            evidence.source1_metadata_slot,
            RpcCommitmentLevel::Confirmed,
        ),
        RpcObservation::new(
            PHASE6_SOURCE2,
            core_id(ClusterId::new("devnet"), "source2-cluster")?,
            core_id(ProgramId::new(PHASE7_PROGRAM_ID), "source2-program")?,
            core_id(MintId::new(PHASE7_TEST_ONLY_ROX_MINT), "source2-mint")?,
            core_id(
                TokenAccountId::new(PHASE7_TEST_ONLY_TOKEN_ACCOUNT),
                "source2-token",
            )?,
            operation_id.clone(),
            PHASE4_INITIALIZATION_SIGNATURE,
            evidence.source2_metadata_slot,
            RpcCommitmentLevel::Confirmed,
        ),
    ];

    let decision = review_coordinator_request(
        &CoordinatorReviewRequest::new(
            package,
            expected,
            expected_rpc,
            observations,
            ReplaySet::default(),
        ),
        CoordinatorConfig::new(PHASE6_REQUIRED_OBSERVATIONS, PHASE6_STALE_AFTER_SLOTS, 4),
        review_slot,
    );

    if decision.status != CoordinatorDecisionStatus::Accepted
        || decision.proof_review.decision != ReviewDecision::Accepted
        || !decision.permits_transaction_simulation()
    {
        return Err(phase7b_error(format!(
            "Phase 7 proof/coordinator gate rejected: coordinator={:?}, proof={:?}",
            decision.status, decision.proof_review.decision,
        )));
    }

    let simulate_config = RelayerConfig::new_with_safety(
        1,
        16,
        AnchorSafetyProfile::new(
            AnchorEnvironmentMode::TestnetOnly,
            AnchorCluster::Devnet,
            ClusterAllowlist::testnet_experiments(),
            SubmissionMode::SimulateOnly,
        ),
    );

    let mut relayer = RelayerDryRun::new(simulate_config);

    let dry_run = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                operation_id,
                idempotency_key,
                "phase7-exact-roc-to-rox-candidate",
                decision.proof_review,
            )
            .with_requested_attempts(1),
        )
        .map_err(|error| phase7b_error(format!("Phase 7 relayer dry-run failed: {error:?}")))?;

    if dry_run.status != RelayerReceiptStatus::DryRunAccepted {
        return Err(phase7b_error(format!(
            "Phase 7 relayer dry-run rejected: {:?}",
            dry_run.status
        )));
    }

    let base = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);

    let simulation = simulate_private_pilot_transaction_plan(
        simulate_config,
        PrivatePilotSimulationPlan::new(base)
            .with_read_only_rpc_verified(true)
            .with_steps(vec![
                PrivatePilotTransactionStep::new(
                    PrivatePilotTransactionKind::Observe,
                    "observe-phase7-shadow-roc-burn",
                    1,
                ),
                PrivatePilotTransactionStep::new(
                    PrivatePilotTransactionKind::Finalize,
                    "finalize-phase7-test-only-rox-mint",
                    1,
                ),
            ]),
    );

    if simulation.status != PrivatePilotSimulationStatus::Simulated
        || !simulation.is_simulated()
        || simulation.live_submission
        || simulation.wallet_key_loading
        || simulation.internal_roc_mutation
    {
        return Err(phase7b_error(format!(
            "Phase 7 local simulation gate rejected: {:?}",
            simulation.status
        )));
    }

    Ok(simulation)
}

fn simulate_exact_candidate(
    rpc: &RpcClient,
    plan: &Phase7CappedRocToRoxPlan,
) -> Result<LiveSimulationEvidence, CliError> {
    let blockhash = rpc
        .get_latest_blockhash()
        .map_err(|error| phase7b_error(format!("could not fetch Devnet blockhash: {error}")))?;

    let message = Message::new(&plan.instructions, Some(&plan.workflow_authority));

    let mut transaction = Transaction::new_unsigned(message);

    transaction.message.recent_blockhash = blockhash;

    if transaction
        .signatures
        .iter()
        .any(|signature| signature != &Signature::default())
    {
        return Err(phase7b_error(
            "unsigned Phase 7 simulation unexpectedly contains a generated signature",
        ));
    }

    let result = rpc.simulate_transaction(&transaction).map_err(|error| {
        phase7b_error(format!(
            "Phase 7 Devnet simulateTransaction failed: {error}"
        ))
    })?;

    if let Some(error) = result.value.err.as_ref() {
        return Err(phase7b_error(format!(
            "exact Phase 7 candidate simulation rejected: {error:?}"
        )));
    }

    Ok(LiveSimulationEvidence {
        context_slot: result.context.slot,

        log_count: result.value.logs.as_ref().map_or(0, Vec::len),
    })
}

fn capture_persistent_state(
    rpc: &RpcClient,
    plan: &Phase7CappedRocToRoxPlan,
    require_operation_absent: bool,
) -> Result<PersistentStateSnapshot, CliError> {
    let accounts = rpc
        .get_multiple_accounts(&[
            plan.config,
            plan.test_only_rox_mint,
            plan.test_only_token_account,
            plan.operation,
        ])
        .map_err(|error| {
            phase7b_error(format!("Phase 7 persistent-state readback failed: {error}"))
        })?;

    if accounts.len() != 4 {
        return Err(phase7b_error(
            "Phase 7 persistent-state readback returned wrong account count",
        ));
    }

    let config = accounts[0]
        .as_ref()
        .ok_or_else(|| phase7b_error("Phase 7 config account is missing"))?;

    let mint = accounts[1]
        .as_ref()
        .ok_or_else(|| phase7b_error("Phase 7 test-only mint is missing"))?;

    let token = accounts[2]
        .as_ref()
        .ok_or_else(|| phase7b_error("Phase 7 token account is missing"))?;

    if require_operation_absent && accounts[3].is_some() {
        return Err(phase7b_error(
            "Phase 7 operation PDA unexpectedly exists persistently",
        ));
    }

    Ok(PersistentStateSnapshot {
        config_data: config.data.clone(),

        mint_data: mint.data.clone(),

        token_data: token.data.clone(),
    })
}

fn build_submit_config(text: &str) -> Result<RelayerPrivatePilotConfig, CliError> {
    let config = RelayerPrivatePilotConfig::from_external_config_text(
        RelayerConfig::new_with_safety(
            1,
            16,
            AnchorSafetyProfile::new(
                AnchorEnvironmentMode::TestnetOnly,
                AnchorCluster::Devnet,
                ClusterAllowlist::testnet_experiments(),
                SubmissionMode::TestnetSubmitCapped,
            ),
        ),
        text,
    )
    .map_err(|error| phase7b_error(format!("Phase 7 capped-submit config rejected: {error}")))?;

    config.validate().map_err(|error| {
        phase7b_error(format!(
            "Phase 7 capped-submit config validation failed: {error}"
        ))
    })?;

    if config.pilot.testnet.environment_mode != AnchorEnvironmentMode::TestnetOnly
        || config.pilot.testnet.cluster != AnchorCluster::Devnet
        || config.pilot.testnet.submission_mode != SubmissionMode::TestnetSubmitCapped
    {
        return Err(phase7b_error(
            "Phase 7 capped-submit config mode/cluster/submission binding mismatch",
        ));
    }

    Ok(config)
}

fn verify_phase5_freshness(evidence: &Phase5Evidence, live_slot: u64) -> Result<u64, CliError> {
    let review_slot = live_slot
        .max(evidence.source1_metadata_slot)
        .max(evidence.source2_metadata_slot);

    for (source, slot) in [
        (PHASE6_SOURCE1, evidence.source1_metadata_slot),
        (PHASE6_SOURCE2, evidence.source2_metadata_slot),
    ] {
        let age = review_slot.saturating_sub(slot);

        if age > PHASE6_STALE_AFTER_SLOTS {
            return Err(phase7b_error(format!(
                "Phase 5 evidence from {source} is stale for Phase 7B: age_slots={age}, limit={PHASE6_STALE_AFTER_SLOTS}"
            )));
        }
    }

    Ok(review_slot)
}

fn write_authorization_receipt(
    path: &Path,
    review_slot: u64,
    plan: &Phase7CappedRocToRoxPlan,
    live: &LiveSimulationEvidence,
    sender: &rox_anchor_relayer::PrivateTestnetSenderAuthorization,
) -> Result<(), CliError> {
    let receipt = json!({
        "schema":
            PHASE7B_SCHEMA,

        "phase":
            "BUILD_PLAN4 Phase 7B",

        "receipt_role":
            "simulation_and_sender_authorization_evidence",

        "cluster":
            "devnet",

        "direction":
            "roc_to_rox",

        "program_id":
            PHASE7_PROGRAM_ID,

        "program_account":
            "<redacted-program-account>",

        "config_account":
            "<redacted-program-config-account>",

        "test_only_rox_mint":
            "<redacted-test-only-mint>",

        "test_only_token_account":
            "<redacted-test-only-token-account>",

        "operation_pda":
            "<redacted-phase7-operation-pda>",

        "operation_id":
            PHASE7_OPERATION_ID,

        "idempotency_key":
            PHASE7_IDEMPOTENCY_KEY,

        "nonce":
            PHASE7_NONCE,

        "amount_minor":
            PHASE7_AMOUNT_MINOR.to_string(),

        "max_amount_minor":
            PHASE7_MAX_AMOUNT_MINOR.to_string(),

        "max_operations":
            PHASE7_MAX_OPERATIONS.to_string(),

        "retry_cap":
            PHASE7_RETRY_CAP.to_string(),

        "instruction_count":
            plan.instructions.len(),

        "instruction_sequence": [
            "observe_burn",
            "finalize_roc_to_rox_mint"
        ],

        "phase5_read_only_evidence":
            "fresh_verified",

        "phase6_forward_simulation_evidence":
            "verified_non_promotable",

        "phase7_local_proof_review":
            "accepted",

        "phase7_coordinator_decision":
            "accepted",

        "phase7_relayer_dry_run":
            "accepted",

        "phase7_live_devnet_simulation":
            "passed",

        "phase7_review_slot":
            review_slot,

        "live_simulation_context_slot":
            live.context_slot,

        "simulation_log_redacted":
            format!(
                "<redacted-simulation-log:{}-entries>",
                live.log_count,
            ),

        "simulated_value_instruction_amount_minor":
            "1",

        "simulated_account_delta_claim":
            "not_invented",

        "persistent_operation_after_simulation":
            false,

        "persistent_config_change_after_simulation":
            false,

        "persistent_mint_change_after_simulation":
            false,

        "persistent_token_account_change_after_simulation":
            false,

        "sender_authorized_by_existing_model":
            sender.authorized,

        "live_submission_permitted_by_existing_model":
            sender.live_submission_permitted,

        "live_submission_attempted":
            sender.live_submission_attempted,

        "network_submitted":
            sender.network_submitted,

        "wallet_key_loading":
            sender.wallet_key_loading,

        "signing":
            sender.signing,

        "user_facing_operator_approval":
            "I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",

        "existing_relayer_internal_approval":
            "I_APPROVE_PRIVATE_TESTNET_CAPPED_SUBMIT",

        "approval_translation_explicit":
            true,

        "send_receipt":
            false,

        "transaction_submission":
            false,

        "signature_generated":
            false,

        "rox_mint_persisted":
            false,

        "real_roc_burn":
            false,

        "real_roc_mutation":
            false,

        "production_settlement":
            false,

        "public_launch_authorized":
            false,

        "mainnet_authorized":
            false,

        "finality_claim":
            false,

        "next_action":
            "PHASE7C_BUILD_LIVE_SIGNED_EXECUTOR_WITHOUT_RUNNING_IT"
    });

    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&receipt,).map_err(|error| {
                phase7b_error(format!("could not encode Phase 7B receipt: {error}"))
            })?,
        ),
    )
    .map_err(|error| phase7b_error(format!("could not write Phase 7B receipt: {error}")))
}

fn core_id<T, E>(result: Result<T, E>, label: &str) -> Result<T, CliError> {
    result.map_err(|_| phase7b_error(format!("invalid Phase 7 `{label}` identifier")))
}

fn required_arg<'a>(value: Option<&'a str>, flag: &str) -> Result<&'a str, CliError> {
    value.ok_or_else(|| phase7b_error(format!("{flag} is required")))
}

fn require_exact_caps(args: &Phase7BArgs) -> Result<(), CliError> {
    if args.max_operations != Some(PHASE7_MAX_OPERATIONS) {
        return Err(phase7b_error("--max-operations must be exactly 1"));
    }

    if args.max_amount_minor != Some(PHASE7_MAX_AMOUNT_MINOR) {
        return Err(phase7b_error("--max-amount-minor must be exactly 1"));
    }

    if args.retry_cap != Some(PHASE7_RETRY_CAP) {
        return Err(phase7b_error("--retry-cap must be exactly 1"));
    }

    Ok(())
}

fn require_ignored_or_absolute_path(path: &str, flag: &str) -> Result<(), CliError> {
    if path.trim().is_empty() {
        return Err(phase7b_error(format!("{flag} may not be empty")));
    }

    if !path.starts_with(".rox-anchor-private-pilot/") && !Path::new(path).is_absolute() {
        return Err(phase7b_error(format!(
            "{flag} must be ignored-local or absolute"
        )));
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<Phase7BArgs, CliError> {
    let mut parsed = Phase7BArgs::default();

    let mut index = 0_usize;

    while index < args.len() {
        match args[index].as_str() {
            "--config" => {
                parsed.config_path = Some(next_value(args, index, "--config")?);
                index += 2;
            }

            "--phase5-receipt" => {
                parsed.phase5_receipt_path = Some(next_value(args, index, "--phase5-receipt")?);
                index += 2;
            }

            "--phase6-receipt" => {
                parsed.phase6_receipt_path = Some(next_value(args, index, "--phase6-receipt")?);
                index += 2;
            }

            "--authorization-receipt-out" => {
                parsed.authorization_receipt_out =
                    Some(next_value(args, index, "--authorization-receipt-out")?);
                index += 2;
            }

            "--operator-approval" => {
                parsed.operator_approval = Some(next_value(args, index, "--operator-approval")?);
                index += 2;
            }

            "--max-operations" => {
                parsed.max_operations = Some(
                    next_value(args, index, "--max-operations")?
                        .parse::<u16>()
                        .map_err(|_| phase7b_error("--max-operations must be u16"))?,
                );
                index += 2;
            }

            "--max-amount-minor" => {
                parsed.max_amount_minor = Some(
                    next_value(args, index, "--max-amount-minor")?
                        .parse::<u64>()
                        .map_err(|_| phase7b_error("--max-amount-minor must be u64"))?,
                );
                index += 2;
            }

            "--retry-cap" => {
                parsed.retry_cap = Some(
                    next_value(args, index, "--retry-cap")?
                        .parse::<u8>()
                        .map_err(|_| phase7b_error("--retry-cap must be u8"))?,
                );
                index += 2;
            }

            "--simulate-and-authorize-only" => {
                parsed.simulate_and_authorize_only = true;
                index += 1;
            }

            other => {
                return Err(phase7b_error(format!("unknown argument `{other}`")));
            }
        }
    }

    Ok(parsed)
}

fn next_value(args: &[String], index: usize, flag: &str) -> Result<String, CliError> {
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| phase7b_error(format!("{flag} requires a value")))
}

fn redact_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("<redacted-local-path>/{name}"))
        .unwrap_or_else(|| "<redacted-local-path>".to_string())
}

fn help_text() -> String {
    [
        "BUILD_PLAN4 Phase 7B exact simulation and sender authorization",
        "",
        "required:",
        "  --config <ignored-or-absolute-capped-submit-config>",
        "  --phase5-receipt <fresh-phase5-closeout>",
        "  --phase6-receipt <phase6-roc-to-rox-simulation>",
        "  --authorization-receipt-out <ignored-local-path>",
        "  --operator-approval I_APPROVE_PRIVATE_TESTNET_CAPPED_SEND",
        "  --max-operations 1",
        "  --max-amount-minor 1",
        "  --retry-cap 1",
        "  --simulate-and-authorize-only",
        "",
        "effects:",
        "  live Devnet simulateTransaction: YES",
        "  exact Phase 7A candidate: YES",
        "  keypair loading: NO",
        "  signature generation: NO",
        "  transaction submission: NO",
        "  persistent ROX mint: NO",
        "  real ROC burn/mutation: NO",
    ]
    .join("\n")
}

fn phase7b_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase7-simulate-and-authorize-roc-to-rox {}",
        message.as_ref(),
    ))
}
