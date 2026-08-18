//! BUILD_PLAN4 Phase 6B live simulation against actual deployed Devnet state.
//!
//! The command performs two real Solana RPC `simulateTransaction` calls:
//! one ROC-to-ROX observation plan and one ROX-to-ROC observation plan.
//! Both use the deployed program/config/mint/token-account bindings.
//!
//! The simulated Anchor instruction is `observe_burn`. It creates the
//! direction-bound operation PDA inside the simulated bank only; it performs
//! no SPL mint CPI, no SPL burn CPI, and no internal ROC mutation.
//!
//! No keypair is loaded. Transactions are intentionally unsigned. Solana RPC
//! simulation evaluates signer account metadata without broadcasting the
//! transaction. After each simulation, read-only RPC proves that the operation
//! PDA was not persisted and that config/mint/token-account bytes are unchanged.

#![forbid(unsafe_code)]

use std::{fs, path::Path, str::FromStr};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, message::Message, pubkey::Pubkey,
        signature::Signature, transaction::Transaction,
    },
};
use anchor_lang::{
    solana_program::{instruction::Instruction, program_option::COption, program_pack::Pack},
    AccountDeserialize, InstructionData, ToAccountMetas,
};
use rox_anchor::{
    AnchorTransferDirection, OperationBindingArgs, RoxAnchorConfig, RoxAnchorOperation,
};
use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture, SubmissionMode,
    TokenAccountId,
};
use rox_anchor_proof::{
    EvidenceBundle, ExpectedProofBinding, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_relayer::{
    simulate_private_pilot_transaction_plan, PrivatePilotSimulationPlan,
    PrivatePilotSimulationStatus, PrivatePilotTransactionKind, PrivatePilotTransactionStep,
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
    TransactionSimulationPlan,
};
use rox_anchor_rpc_proof::{ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation};
use serde_json::json;
use sha2::{Digest, Sha256};
use solana_sdk_ids::system_program;
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::phase6_live_simulation::{
        validate_phase5_receipt, Phase5Evidence, PHASE4_INITIALIZATION_SIGNATURE,
        PHASE6_AMOUNT_MINOR, PHASE6_CONFIG_ACCOUNT, PHASE6_MAX_AMOUNT_MINOR, PHASE6_MAX_OPERATIONS,
        PHASE6_MINT_AUTHORITY, PHASE6_PROGRAM_ID, PHASE6_RECOVERY_AUTHORITY,
        PHASE6_REQUIRED_OBSERVATIONS, PHASE6_ROX_MINT, PHASE6_SOURCE1, PHASE6_SOURCE2,
        PHASE6_STALE_AFTER_SLOTS, PHASE6_TOKEN_ACCOUNT, PHASE6_WORKFLOW_AUTHORITY,
    },
    CliError,
};

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

const TEST_ONLY_MINT_LABEL: &str = "test-only-rox-private-testnet";

const TEST_ONLY_TOKEN_ACCOUNT_LABEL: &str = "test-only-rox-token-account-private-testnet";

#[derive(Clone, Debug)]
struct Phase6LiveArgs {
    help: bool,
    simulate_only: bool,
    phase5_receipt: Option<String>,
    roc_to_rox_receipt_out: Option<String>,
    rox_to_roc_receipt_out: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SimulationDirection {
    RocToRox,
    RoxToRoc,
}

#[derive(Clone, Copy, Debug)]
struct DirectionSpec {
    direction: SimulationDirection,
    receipt_direction: &'static str,
    operation_id: &'static str,
    idempotency_key: &'static str,
    nonce: &'static str,
    source_domain: &'static str,
    destination_domain: &'static str,
    source_account: &'static str,
    recipient_account: &'static str,
    burn_evidence_label: &'static str,
}

#[derive(Clone, Debug)]
struct ActualStateSnapshot {
    config_data: Vec<u8>,
    mint_data: Vec<u8>,
    token_data: Vec<u8>,
}

#[derive(Clone, Debug)]
struct LiveSimulationResult {
    operation: Pubkey,
    context_slot: u64,
    log_count: usize,
}

fn phase6b_error(message: impl Into<String>) -> CliError {
    CliError::UnknownPilotFlag(format!("phase6-live-rpc-simulation {}", message.into()))
}

fn parse_args(args: &[String]) -> Result<Phase6LiveArgs, CliError> {
    let mut parsed = Phase6LiveArgs {
        help: false,
        simulate_only: false,
        phase5_receipt: None,
        roc_to_rox_receipt_out: None,
        rox_to_roc_receipt_out: None,
    };

    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" | "help" => {
                parsed.help = true;
                index += 1;
            }

            "--simulate-only" => {
                parsed.simulate_only = true;
                index += 1;
            }

            "--phase5-receipt" => {
                parsed.phase5_receipt = Some(
                    args.get(index + 1)
                        .ok_or_else(|| phase6b_error("--phase5-receipt requires a value"))?
                        .clone(),
                );

                index += 2;
            }

            "--roc-to-rox-receipt-out" => {
                parsed.roc_to_rox_receipt_out = Some(
                    args.get(index + 1)
                        .ok_or_else(|| phase6b_error("--roc-to-rox-receipt-out requires a value"))?
                        .clone(),
                );

                index += 2;
            }

            "--rox-to-roc-receipt-out" => {
                parsed.rox_to_roc_receipt_out = Some(
                    args.get(index + 1)
                        .ok_or_else(|| phase6b_error("--rox-to-roc-receipt-out requires a value"))?
                        .clone(),
                );

                index += 2;
            }

            other => {
                return Err(phase6b_error(format!("unknown flag `{other}`")));
            }
        }
    }

    Ok(parsed)
}

fn help_text() -> String {
    [
        "rox-anchor pilot phase6-live-rpc-simulation",
        "",
        "BUILD_PLAN4 Phase 6B actual Devnet simulation.",
        "",
        "required:",
        "  --simulate-only",
        "  --phase5-receipt <path>",
        "  --roc-to-rox-receipt-out <path>",
        "  --rox-to-roc-receipt-out <path>",
        "",
        "behavior:",
        "  - requires fresh Phase 5 two-source evidence",
        "  - re-reads deployed program/config/mint/token state",
        "  - runs the existing proof/coordinator/relayer gates",
        "  - builds one ROC-to-ROX observe transaction",
        "  - builds one ROX-to-ROC observe transaction",
        "  - calls Solana simulateTransaction only",
        "  - loads no keypair",
        "  - generates no cryptographic signature",
        "  - submits no transaction",
        "  - verifies operation PDAs remain absent afterward",
        "  - verifies config/mint/token bytes remain unchanged",
        "  - writes redacted non-promotable receipts",
    ]
    .join("\n")
}

fn direction_spec(direction: SimulationDirection) -> DirectionSpec {
    match direction {
        SimulationDirection::RocToRox => DirectionSpec {
            direction,
            receipt_direction: "roc_to_rox",
            operation_id: "actual-simulation-roc-to-rox-0001",
            idempotency_key: "actual-simulation-roc-to-rox-idem-0001",
            nonce: "actual-simulation-roc-to-rox-nonce-0001",
            source_domain: "internal-roc-private-pilot-test",
            destination_domain: "solana-devnet-rox-private-pilot-test",
            source_account: "shadow-internal-roc-burn-source",
            recipient_account: "actual-private-rox-token-owner",
            burn_evidence_label: "phase6-shadow-roc-burn-evidence",
        },

        SimulationDirection::RoxToRoc => DirectionSpec {
            direction,
            receipt_direction: "rox_to_roc",
            operation_id: "actual-simulation-rox-to-roc-0001",
            idempotency_key: "actual-simulation-rox-to-roc-idem-0001",
            nonce: "actual-simulation-rox-to-roc-nonce-0001",
            source_domain: "solana-devnet-rox-private-pilot-test",
            destination_domain: "internal-roc-private-pilot-test",
            source_account: "actual-private-rox-burn-source",
            recipient_account: "shadow-internal-roc-release-intent",
            burn_evidence_label: "phase6-shadow-rox-burn-evidence",
        },
    }
}

fn core_direction(direction: SimulationDirection) -> AnchorDirection {
    match direction {
        SimulationDirection::RocToRox => AnchorDirection::RocToRox,
        SimulationDirection::RoxToRoc => AnchorDirection::RoxToRoc,
    }
}

fn program_direction(direction: SimulationDirection) -> AnchorTransferDirection {
    match direction {
        SimulationDirection::RocToRox => AnchorTransferDirection::RocToRox,
        SimulationDirection::RoxToRoc => AnchorTransferDirection::RoxToRoc,
    }
}

fn hash_label(label: &str) -> [u8; 32] {
    let digest = Sha256::digest(label.as_bytes());
    let mut output = [0_u8; 32];

    output.copy_from_slice(&digest);

    output
}

fn core_id<T, E>(result: Result<T, E>, label: &str) -> Result<T, CliError> {
    result.map_err(|_| phase6b_error(format!("invalid Phase 6B `{label}` identifier")))
}

fn verify_phase5_freshness(evidence: &Phase5Evidence, live_slot: u64) -> Result<u64, CliError> {
    let review_slot = live_slot
        .max(evidence.source1_metadata_slot)
        .max(evidence.source2_metadata_slot);

    for (label, slot) in [
        (
            "solana-public-devnet-primary",
            evidence.source1_metadata_slot,
        ),
        ("uniblock-devnet-secondary", evidence.source2_metadata_slot),
    ] {
        let age = review_slot.saturating_sub(slot);

        if age > PHASE6_STALE_AFTER_SLOTS {
            return Err(
                phase6b_error(format!(
                    "Phase 5 evidence from {label} is stale for live Phase 6 simulation: age_slots={age}, limit={PHASE6_STALE_AFTER_SLOTS}"
                )),
            );
        }
    }

    Ok(review_slot)
}

fn verify_direction_local_gate(
    evidence: &Phase5Evidence,
    review_slot: u64,
    spec: DirectionSpec,
) -> Result<(), CliError> {
    let operation_id = core_id(OperationId::new(spec.operation_id), "operation-id")?;

    let idempotency_key = core_id(IdempotencyKey::new(spec.idempotency_key), "idempotency-key")?;

    let nonce = core_id(Nonce::new(spec.nonce), "nonce")?;

    let binding = AnchorBinding::new(
        core_id(DomainId::new(spec.source_domain), "source-domain")?,
        core_id(DomainId::new(spec.destination_domain), "destination-domain")?,
        core_direction(spec.direction),
        core_id(ClusterId::new("devnet"), "cluster")?,
        core_id(ProgramId::new(PHASE6_PROGRAM_ID), "program-id")?,
        core_id(MintId::new(PHASE6_ROX_MINT), "mint")?,
        core_id(TokenAccountId::new(PHASE6_TOKEN_ACCOUNT), "token-account")?,
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
        core_id(AccountId::new(spec.source_account), "source-account")?,
        core_id(AccountId::new(spec.recipient_account), "recipient-account")?,
        EvidenceBundle::satisfied(PHASE6_REQUIRED_OBSERVATIONS),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );

    let expected_rpc = ExpectedRpcBinding::new(
        core_id(ClusterId::new("devnet"), "rpc-cluster")?,
        core_id(ProgramId::new(PHASE6_PROGRAM_ID), "rpc-program-id")?,
        core_id(MintId::new(PHASE6_ROX_MINT), "rpc-mint")?,
        core_id(
            TokenAccountId::new(PHASE6_TOKEN_ACCOUNT),
            "rpc-token-account",
        )?,
        operation_id.clone(),
        RpcCommitmentLevel::Confirmed,
    );

    let observations = vec![
        RpcObservation::new(
            PHASE6_SOURCE1,
            core_id(ClusterId::new("devnet"), "source1-cluster")?,
            core_id(ProgramId::new(PHASE6_PROGRAM_ID), "source1-program")?,
            core_id(MintId::new(PHASE6_ROX_MINT), "source1-mint")?,
            core_id(TokenAccountId::new(PHASE6_TOKEN_ACCOUNT), "source1-token")?,
            operation_id.clone(),
            PHASE4_INITIALIZATION_SIGNATURE,
            evidence.source1_metadata_slot,
            RpcCommitmentLevel::Confirmed,
        ),
        RpcObservation::new(
            PHASE6_SOURCE2,
            core_id(ClusterId::new("devnet"), "source2-cluster")?,
            core_id(ProgramId::new(PHASE6_PROGRAM_ID), "source2-program")?,
            core_id(MintId::new(PHASE6_ROX_MINT), "source2-mint")?,
            core_id(TokenAccountId::new(PHASE6_TOKEN_ACCOUNT), "source2-token")?,
            operation_id.clone(),
            PHASE4_INITIALIZATION_SIGNATURE,
            evidence.source2_metadata_slot,
            RpcCommitmentLevel::Confirmed,
        ),
    ];

    let request = CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        observations,
        ReplaySet::default(),
    );

    let decision = review_coordinator_request(
        &request,
        CoordinatorConfig::new(PHASE6_REQUIRED_OBSERVATIONS, PHASE6_STALE_AFTER_SLOTS, 4),
        review_slot,
    );

    if decision.status != CoordinatorDecisionStatus::Accepted
        || decision.proof_review.decision != ReviewDecision::Accepted
        || !decision.permits_transaction_simulation()
    {
        return Err(phase6b_error(format!(
            "{} local proof/coordinator gate rejected: coordinator={:?}, proof={:?}",
            spec.receipt_direction, decision.status, decision.proof_review.decision,
        )));
    }

    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );

    let relayer_config = RelayerConfig::new_with_safety(3, 16, safety);

    let mut relayer = RelayerDryRun::new(relayer_config);

    let dry_run = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                operation_id,
                idempotency_key,
                format!("phase6-{}-simulation-target", spec.receipt_direction,),
                decision.proof_review.clone(),
            )
            .with_requested_attempts(1),
        )
        .map_err(|error| {
            phase6b_error(format!(
                "{} relayer dry-run failed: {error:?}",
                spec.receipt_direction,
            ))
        })?;

    if dry_run.status != RelayerReceiptStatus::DryRunAccepted {
        return Err(phase6b_error(format!(
            "{} relayer dry-run rejected: {:?}",
            spec.receipt_direction, dry_run.status,
        )));
    }

    let base_plan = TransactionSimulationPlan::from_dry_run_receipt(
        dry_run,
        decision.permits_transaction_simulation(),
        1,
    );

    let local_plan = PrivatePilotSimulationPlan::new(base_plan)
        .with_read_only_rpc_verified(true)
        .with_steps(vec![PrivatePilotTransactionStep::new(
            PrivatePilotTransactionKind::Observe,
            format!(
                "simulate-{}-observe-against-actual-devnet-bindings",
                spec.receipt_direction,
            ),
            1,
        )]);

    let local_simulation = simulate_private_pilot_transaction_plan(relayer_config, local_plan);

    if local_simulation.status != PrivatePilotSimulationStatus::Simulated
        || !local_simulation.is_simulated()
        || local_simulation.live_submission
        || local_simulation.wallet_key_loading
        || local_simulation.internal_roc_mutation
    {
        return Err(phase6b_error(format!(
            "{} local relayer simulation gate rejected: {:?}",
            spec.receipt_direction, local_simulation.status,
        )));
    }

    Ok(())
}

fn parse_pubkey(value: &str, label: &str) -> Result<Pubkey, CliError> {
    Pubkey::from_str(value)
        .map_err(|_| phase6b_error(format!("{label} is not a valid Solana public key")))
}

fn preflight_actual_state(rpc: &RpcClient) -> Result<ActualStateSnapshot, CliError> {
    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program id")?;

    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config account")?;

    let mint = parse_pubkey(PHASE6_ROX_MINT, "test-only ROX mint")?;

    let token = parse_pubkey(PHASE6_TOKEN_ACCOUNT, "test-only token account")?;

    let workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;

    let halt_authority = parse_pubkey(
        crate::commands::phase6_live_simulation::PHASE6_HALT_AUTHORITY,
        "halt authority",
    )?;

    let recovery_authority = parse_pubkey(PHASE6_RECOVERY_AUTHORITY, "recovery authority")?;

    let mint_authority = parse_pubkey(PHASE6_MINT_AUTHORITY, "mint authority")?;

    let accounts = rpc
        .get_multiple_accounts(&[program, config, mint, token])
        .map_err(|error| phase6b_error(format!("actual-state readback failed: {error}")))?;

    if accounts.len() != 4 {
        return Err(phase6b_error(
            "actual-state readback returned unexpected account count",
        ));
    }

    let program_account = accounts[0]
        .as_ref()
        .ok_or_else(|| phase6b_error("deployed program account is missing"))?;

    if !program_account.executable {
        return Err(phase6b_error("deployed program account is not executable"));
    }

    let config_account = accounts[1]
        .as_ref()
        .ok_or_else(|| phase6b_error("program config account is missing"))?;

    if config_account.owner != program {
        return Err(phase6b_error("program config owner mismatch"));
    }

    let mut config_data = config_account.data.as_slice();

    let config_state = RoxAnchorConfig::try_deserialize(&mut config_data)
        .map_err(|error| phase6b_error(format!("program config decode failed: {error}")))?;

    if config_state.authority != workflow
        || config_state.halt_authority != halt_authority
        || config_state.recovery_authority != recovery_authority
        || config_state.rox_mint != mint
        || config_state.mint_authority != mint_authority
    {
        return Err(phase6b_error(
            "program config authority or mint binding mismatch",
        ));
    }

    if !config_state.test_only_mode
        || config_state.max_supply_units != 1_000
        || config_state.max_amount_units_per_operation != 10
    {
        return Err(phase6b_error(
            "program config private-test-only policy mismatch",
        ));
    }

    if config_state.halted || config_state.recovery_required {
        return Err(phase6b_error(
            "program config is halted or recovery-required",
        ));
    }

    let mint_account = accounts[2]
        .as_ref()
        .ok_or_else(|| phase6b_error("test-only ROX mint account is missing"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase6b_error("test-only ROX mint owner mismatch"));
    }

    let mint_state = Mint::unpack(&mint_account.data)
        .map_err(|error| phase6b_error(format!("test-only ROX mint decode failed: {error}")))?;

    if mint_state.supply != 0
        || mint_state.decimals != 0
        || mint_state.mint_authority != COption::Some(mint_authority)
        || mint_state.freeze_authority != COption::None
    {
        return Err(phase6b_error("test-only ROX mint state mismatch"));
    }

    let token_account = accounts[3]
        .as_ref()
        .ok_or_else(|| phase6b_error("test-only token account is missing"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase6b_error(
            "test-only token account program owner mismatch",
        ));
    }

    let token_state = SplTokenAccount::unpack(&token_account.data).map_err(|error| {
        phase6b_error(format!("test-only token account decode failed: {error}"))
    })?;

    if token_state.mint != mint || token_state.owner != workflow || token_state.amount != 0 {
        return Err(phase6b_error(
            "test-only token account binding or balance mismatch",
        ));
    }

    Ok(ActualStateSnapshot {
        config_data: config_account.data.clone(),
        mint_data: mint_account.data.clone(),
        token_data: token_account.data.clone(),
    })
}

fn simulate_direction(
    rpc: &RpcClient,
    snapshot: &ActualStateSnapshot,
    spec: DirectionSpec,
) -> Result<LiveSimulationResult, CliError> {
    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program id")?;

    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config account")?;

    let mint = parse_pubkey(PHASE6_ROX_MINT, "test-only ROX mint")?;

    let token = parse_pubkey(PHASE6_TOKEN_ACCOUNT, "test-only token account")?;

    let workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;

    let operation_id_hash = hash_label(spec.operation_id);

    let burn_evidence_hash = hash_label(spec.burn_evidence_label);

    let (operation, _) = RoxAnchorOperation::derive_address(&program, &config, &operation_id_hash);

    let before = rpc.get_multiple_accounts(&[operation]).map_err(|error| {
        phase6b_error(format!(
            "{} operation preflight failed: {error}",
            spec.receipt_direction,
        ))
    })?;

    if before.len() != 1 || before[0].is_some() {
        return Err(phase6b_error(format!(
            "{} operation PDA already exists before simulation",
            spec.receipt_direction,
        )));
    }

    let args = OperationBindingArgs {
        operation_id_hash,
        direction: program_direction(spec.direction),
        mint,
        token_account: token,
        amount_atoms: PHASE6_AMOUNT_MINOR,
        burn_evidence_hash,
    };

    let instruction = Instruction {
        program_id: program,
        accounts: rox_anchor::accounts::ObserveBurn {
            config,
            operation,
            payer: workflow,
            system_program: system_program::id(),
        }
        .to_account_metas(None),
        data: rox_anchor::instruction::ObserveBurn { args }.data(),
    };

    let blockhash = rpc.get_latest_blockhash().map_err(|error| {
        phase6b_error(format!(
            "{} could not fetch recent blockhash: {error}",
            spec.receipt_direction,
        ))
    })?;

    let message = Message::new(&[instruction], Some(&workflow));

    let mut transaction = Transaction::new_unsigned(message);

    transaction.message.recent_blockhash = blockhash;

    if transaction
        .signatures
        .iter()
        .any(|signature| signature != &Signature::default())
    {
        return Err(phase6b_error(
            "unsigned simulation transaction unexpectedly contains a generated signature",
        ));
    }

    let simulation = rpc.simulate_transaction(&transaction).map_err(|error| {
        phase6b_error(format!(
            "{} Solana simulateTransaction request failed: {error}",
            spec.receipt_direction,
        ))
    })?;

    if let Some(error) = simulation.value.err.as_ref() {
        return Err(phase6b_error(format!(
            "{} simulated observe transaction was rejected: {error:?}",
            spec.receipt_direction,
        )));
    }

    let log_count = simulation.value.logs.as_ref().map_or(0, Vec::len);

    let after = rpc
        .get_multiple_accounts(&[config, mint, token, operation])
        .map_err(|error| {
            phase6b_error(format!(
                "{} post-simulation readback failed: {error}",
                spec.receipt_direction,
            ))
        })?;

    if after.len() != 4 {
        return Err(phase6b_error(
            "post-simulation readback returned unexpected account count",
        ));
    }

    let config_after = after[0]
        .as_ref()
        .ok_or_else(|| phase6b_error("config disappeared after simulation"))?;

    let mint_after = after[1]
        .as_ref()
        .ok_or_else(|| phase6b_error("mint disappeared after simulation"))?;

    let token_after = after[2]
        .as_ref()
        .ok_or_else(|| phase6b_error("token account disappeared after simulation"))?;

    if config_after.data != snapshot.config_data {
        return Err(phase6b_error("config bytes changed after simulation"));
    }

    if mint_after.data != snapshot.mint_data {
        return Err(phase6b_error("mint bytes changed after simulation"));
    }

    if token_after.data != snapshot.token_data {
        return Err(phase6b_error(
            "token-account bytes changed after simulation",
        ));
    }

    if after[3].is_some() {
        return Err(phase6b_error(format!(
            "{} simulated operation PDA persisted unexpectedly",
            spec.receipt_direction,
        )));
    }

    Ok(LiveSimulationResult {
        operation,
        context_slot: simulation.context.slot,
        log_count,
    })
}

fn require_new_receipt_path(path: &Path, label: &str) -> Result<(), CliError> {
    if path.exists() {
        return Err(phase6b_error(format!(
            "{label} already exists; refusing to overwrite"
        )));
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(phase6b_error(format!(
                "{label} parent directory does not exist"
            )));
        }
    }

    Ok(())
}

fn write_receipt(
    path: &Path,
    spec: DirectionSpec,
    result: &LiveSimulationResult,
) -> Result<(), CliError> {
    require_new_receipt_path(path, spec.receipt_direction)?;

    let receipt = json!({
        "schema":
            "rox-anchor.actual-private-testnet-simulation.v1",

        "phase":
            "BUILD_PLAN4 Phase 6",

        "receipt_role":
            "actual_private_testnet_simulation_receipt",

        "cluster":
            "devnet",

        "direction":
            spec.receipt_direction,

        "program_name":
            "rox_anchor",

        "program_id":
            PHASE6_PROGRAM_ID,

        "simulation_outcome":
            "simulated",

        "operation_id":
            spec.operation_id,

        "idempotency_key":
            spec.idempotency_key,

        "nonce":
            spec.nonce,

        "program_account":
            "<redacted-program-account>",

        "config_account":
            "<redacted-program-config-account>",

        "test_only_mint":
            "<redacted-test-only-mint>",

        "test_only_token_account":
            "<redacted-test-only-token-account>",

        "test_only_mint_label":
            TEST_ONLY_MINT_LABEL,

        "test_only_token_account_label":
            TEST_ONLY_TOKEN_ACCOUNT_LABEL,

        "amount_minor":
            PHASE6_AMOUNT_MINOR.to_string(),

        "max_amount_minor":
            PHASE6_MAX_AMOUNT_MINOR.to_string(),

        "max_operations":
            PHASE6_MAX_OPERATIONS.to_string(),

        "read_only_evidence_status":
            "verified",

        "proof_review_status":
            "accepted",

        "coordinator_decision_status":
            "accepted",

        "relayer_dry_run_status":
            "accepted",

        "simulation_result":
            "passed",

        "simulation_log_redacted":
            format!(
                "<redacted-simulation-log:{}-entries>",
                result.log_count,
            ),

        "read_only_evidence_required":
            true,

        "read_only_evidence_verified":
            true,

        "simulate_only":
            true,

        "transaction_submission":
            false,

        "send_authorized":
            false,

        "wallet_loaded":
            false,

        "signature_generated":
            false,

        "receipt_promotable_to_send":
            false,

        "public_mint_available":
            false,

        "public_launch_authorized":
            false,

        "mainnet_authorized":
            false,

        "production_bridge_settlement":
            false,

        "public_rox_mint_burn":
            false,

        "real_roc_mutation":
            false,

        "finality_claim":
            false,

        "live_rpc_simulation":
            true,

        "unsigned_transaction":
            true,

        "signature_verification":
            false,

        "simulated_instruction":
            "observe_burn",

        "shadow_evidence_only":
            true,

        "operation_pda_redacted":
            "<redacted-simulated-operation-pda>",

        "operation_persisted_after_simulation":
            false,

        "config_bytes_unchanged":
            true,

        "mint_bytes_unchanged":
            true,

        "token_account_bytes_unchanged":
            true,

        "simulation_context_slot":
            result.context_slot,

        "operation_address_display":
            "<redacted-simulated-operation-pda>"
    });

    let serialized = serde_json::to_string_pretty(&receipt)
        .map_err(|error| phase6b_error(format!("could not encode simulation receipt: {error}")))?;

    fs::write(path, format!("{serialized}\n"))
        .map_err(|error| phase6b_error(format!("could not write simulation receipt: {error}")))
}

fn redact_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("<redacted-local-path>/{name}"))
        .unwrap_or_else(|| "<redacted-local-path>".to_string())
}

pub fn run_phase6_live_rpc_simulation(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help_text());
    }

    if !parsed.simulate_only {
        return Err(phase6b_error("requires explicit --simulate-only"));
    }

    let phase5_receipt = parsed
        .phase5_receipt
        .as_deref()
        .ok_or_else(|| phase6b_error("requires --phase5-receipt <path>"))?;

    let roc_to_rox_out = parsed
        .roc_to_rox_receipt_out
        .as_deref()
        .ok_or_else(|| phase6b_error("requires --roc-to-rox-receipt-out <path>"))?;

    let rox_to_roc_out = parsed
        .rox_to_roc_receipt_out
        .as_deref()
        .ok_or_else(|| phase6b_error("requires --rox-to-roc-receipt-out <path>"))?;

    if roc_to_rox_out == rox_to_roc_out {
        return Err(phase6b_error("direction receipts must use different paths"));
    }

    let roc_to_rox_path = Path::new(roc_to_rox_out);

    let rox_to_roc_path = Path::new(rox_to_roc_out);

    require_new_receipt_path(roc_to_rox_path, "ROC-to-ROX receipt")?;

    require_new_receipt_path(rox_to_roc_path, "ROX-to-ROC receipt")?;

    let evidence = validate_phase5_receipt(Path::new(phase5_receipt))?;

    let rpc =
        RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), CommitmentConfig::confirmed());

    let live_slot = rpc
        .get_slot()
        .map_err(|error| phase6b_error(format!("could not query current Devnet slot: {error}")))?;

    let review_slot = verify_phase5_freshness(&evidence, live_slot)?;

    let snapshot = preflight_actual_state(&rpc)?;

    let roc_to_rox = direction_spec(SimulationDirection::RocToRox);

    let rox_to_roc = direction_spec(SimulationDirection::RoxToRoc);

    verify_direction_local_gate(&evidence, review_slot, roc_to_rox)?;

    verify_direction_local_gate(&evidence, review_slot, rox_to_roc)?;

    let roc_to_rox_result = simulate_direction(&rpc, &snapshot, roc_to_rox)?;

    let rox_to_roc_result = simulate_direction(&rpc, &snapshot, rox_to_roc)?;

    write_receipt(roc_to_rox_path, roc_to_rox, &roc_to_rox_result)?;

    write_receipt(rox_to_roc_path, rox_to_roc, &rox_to_roc_result)?;

    Ok([
        "phase6_live_rpc_simulation: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 6B".to_string(),
        "cluster: devnet".to_string(),
        "phase5_freshness: GREEN".to_string(),
        format!("phase5_source_slot_delta: {}", evidence.metadata_slot_delta,),
        format!("phase6_review_slot: {review_slot}",),
        "actual_state_preflight: GREEN".to_string(),
        "roc_to_rox_local_gate: GREEN".to_string(),
        "rox_to_roc_local_gate: GREEN".to_string(),
        "roc_to_rox_live_simulation: GREEN".to_string(),
        "rox_to_roc_live_simulation: GREEN".to_string(),
        format!(
            "roc_to_rox_simulation_context_slot: {}",
            roc_to_rox_result.context_slot,
        ),
        format!(
            "rox_to_roc_simulation_context_slot: {}",
            rox_to_roc_result.context_slot,
        ),
        format!(
            "roc_to_rox_operation_persistence: none:{}",
            short_pubkey(roc_to_rox_result.operation,),
        ),
        format!(
            "rox_to_roc_operation_persistence: none:{}",
            short_pubkey(rox_to_roc_result.operation,),
        ),
        "config_bytes_unchanged: true".to_string(),
        "mint_bytes_unchanged: true".to_string(),
        "token_account_bytes_unchanged: true".to_string(),
        "unsigned_transactions: true".to_string(),
        "keypair_loading: false".to_string(),
        "signature_generation: false".to_string(),
        "transaction_submission: false".to_string(),
        "rox_mint_execution: false".to_string(),
        "rox_burn_execution: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "receipt_promotable_to_send: false".to_string(),
        format!("roc_to_rox_receipt: {}", redact_path(roc_to_rox_path,),),
        format!("rox_to_roc_receipt: {}", redact_path(rox_to_roc_path,),),
        "next_action: CLOSE_BUILD_PLAN4_PHASE6".to_string(),
    ]
    .join("\n"))
}

fn short_pubkey(key: Pubkey) -> String {
    let value = key.to_string();

    if value.len() <= 12 {
        return "<redacted-pda>".to_string();
    }

    format!(
        "<redacted-pda:{}...{}>",
        &value[..4],
        &value[value.len() - 4..],
    )
}
