//! RO:WHAT — Builds and live-simulates the exact BUILD_PLAN4 Phase 8
//! test-only ROX-to-ROC burn candidate against the post-Phase-7 Devnet state.
//! RO:WHY — Phase 8 must prove the reverse one-unit burn path before any new
//! live-send approval is requested.
//! RO:INTERACTS — Solana/Uniblock read-only RPC, ROX Anchor observe/finalize
//! burn instructions, proof/coordinator/relayer gates, and dry-run ROC release.
//! RO:INVARIANTS — Devnet/test-only only; current ROX state must be exactly
//! 1/1; operation identity is fresh; simulation never persists the burn.
//! RO:SECURITY — no keypair loading, signing, transaction submission, real ROC
//! release, svc-wallet call, ron-ledger mutation, production settlement, or mainnet.
//! RO:TEST — phase8_rox_to_roc_simulation_source.rs plus reverse-path regressions.

#![forbid(unsafe_code)]

use std::{fs, fs::OpenOptions, io::Write, path::Path, str::FromStr};

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        account::Account, commitment_config::CommitmentConfig, message::Message, pubkey::Pubkey,
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
    IdempotencyKey, InternalRocDryRunReleaseIntent, MintId, Nonce, OperationId, ProgramId,
    RecoveryPosture, SubmissionMode, TokenAccountId,
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
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use solana_sdk_ids::system_program;
use spl_token::state::{Account as SplTokenAccount, Mint};

use crate::{
    commands::{
        phase5_live_quorum::PHASE5B_SOURCE2_RPC_URL,
        phase6_live_simulation::{
            PHASE4_INITIALIZATION_SIGNATURE, PHASE6_CONFIG_ACCOUNT, PHASE6_MINT_AUTHORITY,
            PHASE6_PROGRAM_ID, PHASE6_ROX_MINT, PHASE6_SOURCE1, PHASE6_SOURCE2,
            PHASE6_TOKEN_ACCOUNT, PHASE6_WORKFLOW_AUTHORITY,
        },
    },
    CliError,
};

const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

pub(crate) const PHASE8_OPERATION_ID: &str = "actual-rox-to-roc-op-0001";
pub(crate) const PHASE8_IDEMPOTENCY_KEY: &str = "actual-rox-to-roc-idem-0001";
pub(crate) const PHASE8_NONCE: &str = "actual-rox-to-roc-nonce-0001";
pub(crate) const PHASE8_BURN_EVIDENCE_LABEL: &str = "phase8-test-only-rox-burn-evidence-0001";

const PHASE8_RELEASE_TARGET: &str = "crablink-private-roc-release-target-0001";

const PHASE8_RELEASE_LABEL: &str = "test-only-internal-roc-release-intent";

pub(crate) const PHASE8_AMOUNT_MINOR: u64 = 1;
const PHASE8_REQUIRED_OBSERVATIONS: u16 = 2;
const PHASE8_STALE_AFTER_SLOTS: u64 = 100;

const PHASE7F_SCHEMA: &str = "rox-anchor.phase7-post-send-closeout.v1";

#[derive(Debug, Default)]
struct Phase8Args {
    help: bool,
    simulate_only: bool,
    phase7f_closeout: Option<String>,
    simulation_receipt_out: Option<String>,
    release_intent_receipt_out: Option<String>,
}

#[derive(Debug)]
struct ActualState {
    source1_slot: u64,
    source2_slot: u64,
    operation: Pubkey,
    config_data: Vec<u8>,
    mint_data: Vec<u8>,
    token_data: Vec<u8>,
}

#[derive(Debug)]
struct SimulationResult {
    context_slot: u64,
    log_count: usize,
}

fn phase8_error(message: impl AsRef<str>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase8-simulate-rox-to-roc-burn {}",
        message.as_ref(),
    ))
}

fn parse_args(args: &[String]) -> Result<Phase8Args, CliError> {
    let mut parsed = Phase8Args::default();
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
            "--phase7f-closeout" => {
                parsed.phase7f_closeout = Some(
                    args.get(index + 1)
                        .ok_or_else(|| phase8_error("--phase7f-closeout requires a value"))?
                        .clone(),
                );
                index += 2;
            }
            "--simulation-receipt-out" => {
                parsed.simulation_receipt_out = Some(
                    args.get(index + 1)
                        .ok_or_else(|| phase8_error("--simulation-receipt-out requires a value"))?
                        .clone(),
                );
                index += 2;
            }
            "--release-intent-receipt-out" => {
                parsed.release_intent_receipt_out = Some(
                    args.get(index + 1)
                        .ok_or_else(|| {
                            phase8_error("--release-intent-receipt-out requires a value")
                        })?
                        .clone(),
                );
                index += 2;
            }
            other => {
                return Err(phase8_error(format!("unknown flag `{other}`",)));
            }
        }
    }

    Ok(parsed)
}

fn help_text() -> String {
    [
        "rox-anchor pilot phase8-simulate-rox-to-roc-burn",
        "",
        "BUILD_PLAN4 Phase 8A exact ROX-to-ROC burn simulation.",
        "",
        "required:",
        "  --simulate-only",
        "  --phase7f-closeout <receipt>",
        "  --simulation-receipt-out <new-receipt>",
        "  --release-intent-receipt-out <new-receipt>",
        "",
        "behavior:",
        "  verifies Phase 7 is fully closed",
        "  independently reads current state from Solana + Uniblock",
        "  requires test-only ROX mint supply = 1",
        "  requires workflow ROX token amount = 1",
        "  requires the fresh Phase 8 operation PDA to be absent",
        "  runs proof/coordinator/relayer dry-run gates",
        "  simulates observe_burn + finalize_rox_to_roc_burn",
        "  verifies simulation persisted no state",
        "  creates a dry-run internal ROC release intent only",
        "",
        "security:",
        "  no keypair loading",
        "  no signing",
        "  no transaction submission",
        "  no persistent ROX burn",
        "  no svc-wallet call",
        "  no ron-ledger mutation",
        "  no real internal ROC release",
        "  no production settlement",
        "  no mainnet",
    ]
    .join("\n")
}

fn parse_pubkey(value: &str, label: &str) -> Result<Pubkey, CliError> {
    Pubkey::from_str(value)
        .map_err(|_| phase8_error(format!("{label} is not a valid Solana public key",)))
}

fn sha256(value: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(value);
    let mut output = [0_u8; 32];
    output.copy_from_slice(&digest);
    output
}

fn operation_id() -> Result<OperationId, CliError> {
    OperationId::new(PHASE8_OPERATION_ID).map_err(|_| phase8_error("invalid Phase 8 operation id"))
}

fn idempotency_key() -> Result<IdempotencyKey, CliError> {
    IdempotencyKey::new(PHASE8_IDEMPOTENCY_KEY)
        .map_err(|_| phase8_error("invalid Phase 8 idempotency key"))
}

fn nonce() -> Result<Nonce, CliError> {
    Nonce::new(PHASE8_NONCE).map_err(|_| phase8_error("invalid Phase 8 nonce"))
}

fn simulation_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    )
}

fn require_phase7f_closeout(path: &Path) -> Result<(), CliError> {
    let text = fs::read_to_string(path)
        .map_err(|error| phase8_error(format!("could not read Phase 7F closeout: {error}",)))?;

    let value: Value = serde_json::from_str(&text)
        .map_err(|_| phase8_error("Phase 7F closeout is not valid JSON"))?;

    for (field, expected) in [
        ("schema", PHASE7F_SCHEMA),
        ("phase", "BUILD_PLAN4 Phase 7F"),
        ("cluster", "devnet"),
        ("direction", "roc_to_rox"),
        ("mint_supply_minor", "1"),
        ("workflow_token_amount_minor", "1"),
        ("operation_state", "finalized"),
    ] {
        if value.get(field).and_then(Value::as_str) != Some(expected) {
            return Err(phase8_error(format!(
                "Phase 7F closeout field `{field}` is not `{expected}`",
            )));
        }
    }

    for field in [
        "two_source_account_bytes_agree",
        "config_binding_verified",
        "operation_pda_exists",
        "operation_binding_verified",
        "read_only_rpc",
        "phase7_closeout",
    ] {
        if value.get(field).and_then(Value::as_bool) != Some(true) {
            return Err(phase8_error(format!(
                "Phase 7F closeout requires `{field}=true`",
            )));
        }
    }

    for field in [
        "challenge_open",
        "recovery_required",
        "transaction_submission",
        "additional_rox_mint",
        "real_roc_mutation",
        "production_settlement",
        "mainnet_authorized",
    ] {
        if value.get(field).and_then(Value::as_bool) != Some(false) {
            return Err(phase8_error(format!(
                "Phase 7F closeout requires `{field}=false`",
            )));
        }
    }

    Ok(())
}

fn accounts_match(left: &[Option<Account>], right: &[Option<Account>]) -> bool {
    left == right
}

fn read_and_validate_two_source_state(operation: Pubkey) -> Result<ActualState, CliError> {
    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program")?;
    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config")?;
    let mint = parse_pubkey(PHASE6_ROX_MINT, "ROX mint")?;
    let token = parse_pubkey(PHASE6_TOKEN_ACCOUNT, "ROX token account")?;
    let workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;
    let mint_authority = parse_pubkey(PHASE6_MINT_AUTHORITY, "mint authority")?;

    let keys = [program, config, mint, token, operation];

    let source1 =
        RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), CommitmentConfig::confirmed());

    let source2 = RpcClient::new_with_commitment(
        PHASE5B_SOURCE2_RPC_URL.to_string(),
        CommitmentConfig::confirmed(),
    );

    let batch1 = super::phase5_wire_compat::get_multiple_accounts_with_context_compat(
        &source1,
        PHASE6_SOURCE1,
        &keys,
        None,
    )
    .map_err(|error| phase8_error(format!("Solana post-Phase-7 state read failed: {error}",)))?;

    let batch2 = super::phase5_wire_compat::get_multiple_accounts_with_context_compat(
        &source2,
        PHASE6_SOURCE2,
        &keys,
        None,
    )
    .map_err(|error| phase8_error(format!("Uniblock post-Phase-7 state read failed: {error}",)))?;

    if batch1.accounts.len() != 5 || batch2.accounts.len() != 5 {
        return Err(phase8_error(
            "two-source Phase 8 state read returned unexpected account count",
        ));
    }

    if batch1.context_slot.abs_diff(batch2.context_slot) > PHASE8_STALE_AFTER_SLOTS {
        return Err(phase8_error(
            "two-source Phase 8 observations are too far apart",
        ));
    }

    if !accounts_match(&batch1.accounts, &batch2.accounts) {
        return Err(phase8_error(
            "Solana and Uniblock disagree on exact Phase 8 pre-state",
        ));
    }

    let program_account = batch1.accounts[0]
        .as_ref()
        .ok_or_else(|| phase8_error("deployed program is missing"))?;

    if !program_account.executable {
        return Err(phase8_error(
            "deployed ROX Anchor program is not executable",
        ));
    }

    let config_account = batch1.accounts[1]
        .as_ref()
        .ok_or_else(|| phase8_error("ROX Anchor config is missing"))?;

    if config_account.owner != program {
        return Err(phase8_error("ROX Anchor config owner mismatch"));
    }

    let mut config_bytes = config_account.data.as_slice();

    let config_state = RoxAnchorConfig::try_deserialize(&mut config_bytes)
        .map_err(|error| phase8_error(format!("ROX Anchor config decode failed: {error}",)))?;

    if config_state.authority != workflow
        || config_state.rox_mint != mint
        || config_state.mint_authority != mint_authority
    {
        return Err(phase8_error(
            "ROX Anchor config authority/mint binding mismatch",
        ));
    }

    if !config_state.test_only_mode
        || config_state.max_supply_units != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_SUPPLY_UNITS
        || config_state.max_amount_units_per_operation
            != RoxAnchorConfig::PRIVATE_TEST_ONLY_MAX_AMOUNT_UNITS
    {
        return Err(phase8_error(
            "ROX Anchor config is not exact private-test-only policy",
        ));
    }

    if config_state.halted || config_state.recovery_required {
        return Err(phase8_error(
            "halt/recovery posture blocks Phase 8 simulation",
        ));
    }

    let mint_account = batch1.accounts[2]
        .as_ref()
        .ok_or_else(|| phase8_error("test-only ROX mint is missing"))?;

    if mint_account.owner != spl_token::id() {
        return Err(phase8_error("ROX mint program owner mismatch"));
    }

    let mint_state = Mint::unpack(&mint_account.data)
        .map_err(|error| phase8_error(format!("ROX mint decode failed: {error}",)))?;

    if mint_state.supply != PHASE8_AMOUNT_MINOR
        || mint_state.decimals != 0
        || mint_state.mint_authority != COption::Some(mint_authority)
        || mint_state.freeze_authority != COption::None
    {
        return Err(phase8_error(
            "Phase 8 requires exact post-Phase-7 mint state supply=1",
        ));
    }

    let token_account = batch1.accounts[3]
        .as_ref()
        .ok_or_else(|| phase8_error("test-only ROX token account is missing"))?;

    if token_account.owner != spl_token::id() {
        return Err(phase8_error("ROX token account program owner mismatch"));
    }

    let token_state = SplTokenAccount::unpack(&token_account.data)
        .map_err(|error| phase8_error(format!("ROX token account decode failed: {error}",)))?;

    if token_state.mint != mint
        || token_state.owner != workflow
        || token_state.amount != PHASE8_AMOUNT_MINOR
    {
        return Err(phase8_error(
            "Phase 8 requires exact workflow token state amount=1",
        ));
    }

    if batch1.accounts[4].is_some() {
        return Err(phase8_error("fresh Phase 8 operation PDA already exists"));
    }

    Ok(ActualState {
        source1_slot: batch1.context_slot,
        source2_slot: batch2.context_slot,
        operation,
        config_data: config_account.data.clone(),
        mint_data: mint_account.data.clone(),
        token_data: token_account.data.clone(),
    })
}

fn verify_proof_coordinator_relayer_gate(state: &ActualState) -> Result<(), CliError> {
    let op = operation_id()?;
    let idem = idempotency_key()?;
    let nonce = nonce()?;

    let binding = AnchorBinding::new(
        DomainId::new("solana-devnet-rox-private-pilot-test")
            .map_err(|_| phase8_error("invalid source domain"))?,
        DomainId::new("internal-roc-private-pilot-test")
            .map_err(|_| phase8_error("invalid destination domain"))?,
        AnchorDirection::RoxToRoc,
        ClusterId::new("devnet").map_err(|_| phase8_error("invalid cluster"))?,
        ProgramId::new(PHASE6_PROGRAM_ID).map_err(|_| phase8_error("invalid program id"))?,
        MintId::new(PHASE6_ROX_MINT).map_err(|_| phase8_error("invalid mint id"))?,
        TokenAccountId::new(PHASE6_TOKEN_ACCOUNT)
            .map_err(|_| phase8_error("invalid token account id"))?,
    );

    let expected =
        ExpectedProofBinding::new(binding.clone(), op.clone(), idem.clone(), nonce.clone());

    let package = ProofPackage::new(
        binding,
        op.clone(),
        idem.clone(),
        nonce,
        AccountId::new("actual-private-rox-burn-source-0001")
            .map_err(|_| phase8_error("invalid burn source"))?,
        AccountId::new(PHASE8_RELEASE_TARGET)
            .map_err(|_| phase8_error("invalid release target"))?,
        EvidenceBundle::satisfied(PHASE8_REQUIRED_OBSERVATIONS),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );

    let expected_rpc = ExpectedRpcBinding::new(
        ClusterId::new("devnet").map_err(|_| phase8_error("invalid RPC cluster"))?,
        ProgramId::new(PHASE6_PROGRAM_ID).map_err(|_| phase8_error("invalid RPC program"))?,
        MintId::new(PHASE6_ROX_MINT).map_err(|_| phase8_error("invalid RPC mint"))?,
        TokenAccountId::new(PHASE6_TOKEN_ACCOUNT).map_err(|_| phase8_error("invalid RPC token"))?,
        op.clone(),
        RpcCommitmentLevel::Confirmed,
    );

    let observations = vec![
        RpcObservation::new(
            PHASE6_SOURCE1,
            ClusterId::new("devnet").map_err(|_| phase8_error("invalid source1 cluster"))?,
            ProgramId::new(PHASE6_PROGRAM_ID)
                .map_err(|_| phase8_error("invalid source1 program"))?,
            MintId::new(PHASE6_ROX_MINT).map_err(|_| phase8_error("invalid source1 mint"))?,
            TokenAccountId::new(PHASE6_TOKEN_ACCOUNT)
                .map_err(|_| phase8_error("invalid source1 token"))?,
            op.clone(),
            PHASE4_INITIALIZATION_SIGNATURE,
            state.source1_slot,
            RpcCommitmentLevel::Confirmed,
        ),
        RpcObservation::new(
            PHASE6_SOURCE2,
            ClusterId::new("devnet").map_err(|_| phase8_error("invalid source2 cluster"))?,
            ProgramId::new(PHASE6_PROGRAM_ID)
                .map_err(|_| phase8_error("invalid source2 program"))?,
            MintId::new(PHASE6_ROX_MINT).map_err(|_| phase8_error("invalid source2 mint"))?,
            TokenAccountId::new(PHASE6_TOKEN_ACCOUNT)
                .map_err(|_| phase8_error("invalid source2 token"))?,
            op.clone(),
            PHASE4_INITIALIZATION_SIGNATURE,
            state.source2_slot,
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

    let review_slot = state.source1_slot.max(state.source2_slot);

    let decision = review_coordinator_request(
        &request,
        CoordinatorConfig::new(PHASE8_REQUIRED_OBSERVATIONS, PHASE8_STALE_AFTER_SLOTS, 4),
        review_slot,
    );

    if decision.status != CoordinatorDecisionStatus::Accepted
        || decision.proof_review.decision != ReviewDecision::Accepted
        || !decision.permits_transaction_simulation()
    {
        return Err(phase8_error(format!(
            "proof/coordinator gate rejected: coordinator={:?}, proof={:?}",
            decision.status, decision.proof_review.decision,
        )));
    }

    let relayer_config = RelayerConfig::new_with_safety(3, 16, simulation_safety());

    let mut relayer = RelayerDryRun::new(relayer_config);

    let dry_run = relayer
        .submit_dry_run(
            RelayerSubmissionRequest::new(
                op,
                idem,
                "phase8-rox-to-roc-burn-simulation-target",
                decision.proof_review.clone(),
            )
            .with_requested_attempts(1),
        )
        .map_err(|error| phase8_error(format!("Phase 8 relayer dry-run failed: {error:?}",)))?;

    if dry_run.status != RelayerReceiptStatus::DryRunAccepted {
        return Err(phase8_error("Phase 8 relayer dry-run was not accepted"));
    }

    let base = TransactionSimulationPlan::from_dry_run_receipt(
        dry_run,
        decision.permits_transaction_simulation(),
        2,
    );

    let plan = PrivatePilotSimulationPlan::new(base)
        .with_read_only_rpc_verified(true)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Observe,
                "phase8-observe-test-only-rox-burn",
                1,
            ),
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Finalize,
                "phase8-finalize-test-only-rox-burn",
                1,
            ),
        ]);

    let local = simulate_private_pilot_transaction_plan(relayer_config, plan);

    if local.status != PrivatePilotSimulationStatus::Simulated
        || !local.is_simulated()
        || local.live_submission
        || local.wallet_key_loading
        || local.internal_roc_mutation
    {
        return Err(phase8_error(
            "Phase 8 local relayer simulation gate rejected",
        ));
    }

    Ok(())
}

pub(crate) fn build_exact_instructions(operation: Pubkey) -> Result<Vec<Instruction>, CliError> {
    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program")?;
    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config")?;
    let mint = parse_pubkey(PHASE6_ROX_MINT, "ROX mint")?;
    let token = parse_pubkey(PHASE6_TOKEN_ACCOUNT, "ROX token")?;
    let workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;
    let mint_authority = parse_pubkey(PHASE6_MINT_AUTHORITY, "mint authority")?;

    let args = OperationBindingArgs {
        operation_id_hash: sha256(PHASE8_OPERATION_ID.as_bytes()),
        direction: AnchorTransferDirection::RoxToRoc,
        mint,
        token_account: token,
        amount_atoms: PHASE8_AMOUNT_MINOR,
        burn_evidence_hash: sha256(PHASE8_BURN_EVIDENCE_LABEL.as_bytes()),
    };

    let observe = Instruction {
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

    let burn = Instruction {
        program_id: program,
        accounts: rox_anchor::accounts::FinalizeRoxToRocBurn {
            config,
            authority: workflow,
            operation,
            rox_mint: mint,
            source_rox_token_account: token,
            source_rox_token_authority: workflow,
            mint_authority,
            token_program: spl_token::id(),
        }
        .to_account_metas(None),
        data: rox_anchor::instruction::FinalizeRoxToRocBurn {}.data(),
    };

    Ok(vec![observe, burn])
}

fn simulate_exact_burn(state: &ActualState) -> Result<SimulationResult, CliError> {
    let workflow = parse_pubkey(PHASE6_WORKFLOW_AUTHORITY, "workflow authority")?;

    let instructions = build_exact_instructions(state.operation)?;

    if instructions.len() != 2 {
        return Err(phase8_error(
            "Phase 8 candidate must contain exactly two instructions",
        ));
    }

    let rpc =
        RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), CommitmentConfig::confirmed());

    let blockhash = rpc
        .get_latest_blockhash()
        .map_err(|error| phase8_error(format!("could not fetch Devnet blockhash: {error}",)))?;

    let message = Message::new(&instructions, Some(&workflow));

    let mut transaction = Transaction::new_unsigned(message);
    transaction.message.recent_blockhash = blockhash;

    if transaction
        .signatures
        .iter()
        .any(|signature| signature != &Signature::default())
    {
        return Err(phase8_error(
            "Phase 8 unsigned simulation unexpectedly has a signature",
        ));
    }

    let simulation = rpc
        .simulate_transaction(&transaction)
        .map_err(|error| phase8_error(format!("Phase 8 simulateTransaction failed: {error}",)))?;

    if let Some(error) = simulation.value.err.as_ref() {
        return Err(phase8_error(format!(
            "exact ROX-to-ROC burn simulation rejected: {error:?}",
        )));
    }

    let log_count = simulation.value.logs.as_ref().map_or(0, Vec::len);

    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config")?;
    let mint = parse_pubkey(PHASE6_ROX_MINT, "ROX mint")?;
    let token = parse_pubkey(PHASE6_TOKEN_ACCOUNT, "ROX token")?;

    let after = rpc
        .get_multiple_accounts(&[config, mint, token, state.operation])
        .map_err(|error| phase8_error(format!("post-simulation readback failed: {error}",)))?;

    if after.len() != 4 {
        return Err(phase8_error(
            "post-simulation readback returned unexpected account count",
        ));
    }

    let config_after = after[0]
        .as_ref()
        .ok_or_else(|| phase8_error("config disappeared"))?;

    let mint_after = after[1]
        .as_ref()
        .ok_or_else(|| phase8_error("ROX mint disappeared"))?;

    let token_after = after[2]
        .as_ref()
        .ok_or_else(|| phase8_error("ROX token account disappeared"))?;

    if config_after.data != state.config_data
        || mint_after.data != state.mint_data
        || token_after.data != state.token_data
    {
        return Err(phase8_error(
            "Phase 8 simulation unexpectedly changed persistent state",
        ));
    }

    if after[3].is_some() {
        return Err(phase8_error(
            "Phase 8 simulated operation persisted unexpectedly",
        ));
    }

    let mint_state = Mint::unpack(&mint_after.data).map_err(|error| {
        phase8_error(format!("post-simulation ROX mint decode failed: {error}",))
    })?;

    let token_state = SplTokenAccount::unpack(&token_after.data)
        .map_err(|error| phase8_error(format!("post-simulation token decode failed: {error}",)))?;

    if mint_state.supply != 1 || token_state.amount != 1 {
        return Err(phase8_error(
            "persistent post-simulation ROX state must remain 1/1",
        ));
    }

    Ok(SimulationResult {
        context_slot: simulation.context.slot,
        log_count,
    })
}

fn require_new_path(path: &Path, label: &str) -> Result<(), CliError> {
    let display = path.to_string_lossy();

    if !path.is_absolute() && !display.starts_with(".rox-anchor-private-pilot/") {
        return Err(phase8_error(format!(
            "{label} must be absolute or inside .rox-anchor-private-pilot",
        )));
    }

    if path.exists() {
        return Err(phase8_error(format!(
            "{label} already exists; refusing overwrite",
        )));
    }

    let parent = path
        .parent()
        .ok_or_else(|| phase8_error(format!("{label} has no parent directory",)))?;

    if !parent.is_dir() {
        return Err(phase8_error(format!(
            "{label} parent directory does not exist",
        )));
    }

    Ok(())
}

fn write_new_json(path: &Path, value: &Value) -> Result<(), CliError> {
    let encoded = serde_json::to_string_pretty(value)
        .map_err(|error| phase8_error(format!("could not encode Phase 8 receipt: {error}",)))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| phase8_error(format!("could not create Phase 8 receipt: {error}",)))?;

    file.write_all(encoded.as_bytes())
        .map_err(|error| phase8_error(format!("could not write Phase 8 receipt: {error}",)))?;

    file.write_all(b"\n")
        .map_err(|error| phase8_error(format!("could not terminate Phase 8 receipt: {error}",)))?;

    file.sync_all()
        .map_err(|error| phase8_error(format!("could not sync Phase 8 receipt: {error}",)))
}

fn redact_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("<redacted-local-path>/{name}",))
        .unwrap_or_else(|| "<redacted-local-path>".to_string())
}

pub fn run_phase8_rox_to_roc_simulation(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help_text());
    }

    if !parsed.simulate_only {
        return Err(phase8_error("requires explicit --simulate-only"));
    }

    let phase7f = Path::new(
        parsed
            .phase7f_closeout
            .as_deref()
            .ok_or_else(|| phase8_error("--phase7f-closeout is required"))?,
    );

    let simulation_out = Path::new(
        parsed
            .simulation_receipt_out
            .as_deref()
            .ok_or_else(|| phase8_error("--simulation-receipt-out is required"))?,
    );

    let release_out = Path::new(
        parsed
            .release_intent_receipt_out
            .as_deref()
            .ok_or_else(|| phase8_error("--release-intent-receipt-out is required"))?,
    );

    if simulation_out == release_out {
        return Err(phase8_error(
            "simulation and release-intent receipts need distinct paths",
        ));
    }

    require_new_path(simulation_out, "simulation receipt")?;

    require_new_path(release_out, "release-intent receipt")?;

    require_phase7f_closeout(phase7f)?;

    let program = parse_pubkey(PHASE6_PROGRAM_ID, "program")?;
    let config = parse_pubkey(PHASE6_CONFIG_ACCOUNT, "config")?;

    let operation_hash = sha256(PHASE8_OPERATION_ID.as_bytes());

    let (operation, _) = RoxAnchorOperation::derive_address(&program, &config, &operation_hash);

    let state = read_and_validate_two_source_state(operation)?;

    verify_proof_coordinator_relayer_gate(&state)?;

    let release_intent = InternalRocDryRunReleaseIntent::new(
        simulation_safety(),
        operation_id()?,
        idempotency_key()?,
        nonce()?,
        AccountId::new(PHASE8_RELEASE_TARGET)
            .map_err(|_| phase8_error("invalid internal ROC release target"))?,
        PHASE8_RELEASE_LABEL,
        PHASE8_AMOUNT_MINOR,
    )
    .map_err(|error| phase8_error(format!("dry-run ROC release intent rejected: {error}",)))?;

    release_intent.validate().map_err(|error| {
        phase8_error(format!("dry-run ROC release validation failed: {error}",))
    })?;

    let simulation = simulate_exact_burn(&state)?;

    let simulation_receipt = json!({
        "schema":
            "rox-anchor.phase8-rox-to-roc-simulation.v1",

        "phase":
            "BUILD_PLAN4 Phase 8A",

        "receipt_role":
            "actual_rox_to_roc_exact_burn_simulation",

        "cluster":
            "devnet",

        "direction":
            "rox_to_roc",

        "operation_id":
            PHASE8_OPERATION_ID,

        "idempotency_key":
            PHASE8_IDEMPOTENCY_KEY,

        "nonce":
            PHASE8_NONCE,

        "amount_minor":
            PHASE8_AMOUNT_MINOR.to_string(),

        "pre_mint_supply_minor":
            "1",

        "pre_workflow_token_amount_minor":
            "1",

        "instruction_count":
            2,

        "instruction_1":
            "observe_burn",

        "instruction_2":
            "finalize_rox_to_roc_burn",

        "two_source_read_only_state":
            "Agreement",

        "rpc_source_1":
            PHASE6_SOURCE1,

        "rpc_source_2":
            PHASE6_SOURCE2,

        "source_1_context_slot":
            state.source1_slot,

        "source_2_context_slot":
            state.source2_slot,

        "proof_review_status":
            "accepted",

        "coordinator_decision_status":
            "accepted",

        "relayer_dry_run_status":
            "accepted",

        "live_devnet_exact_candidate_simulation":
            "passed",

        "simulation_context_slot":
            simulation.context_slot,

        "simulation_log_redacted":
            format!(
                "<redacted-simulation-log:{}-entries>",
                simulation.log_count,
            ),

        "unsigned_transaction":
            true,

        "persistent_post_simulation_mint_supply_minor":
            "1",

        "persistent_post_simulation_workflow_token_amount_minor":
            "1",

        "operation_persisted_after_simulation":
            false,

        "config_bytes_unchanged":
            true,

        "mint_bytes_unchanged":
            true,

        "token_account_bytes_unchanged":
            true,

        "dry_run_internal_roc_release_intent":
            true,

        "release_intent_amount_minor":
            "1",

        "future_real_roc_path":
            "svc-wallet -> ron-ledger only",

        "keypair_loading":
            false,

        "signature_generation":
            false,

        "transaction_submission":
            false,

        "persistent_rox_burn":
            false,

        "real_internal_roc_release":
            false,

        "svc_wallet_call":
            false,

        "ron_ledger_mutation":
            false,

        "production_settlement":
            false,

        "mainnet_authorized":
            false,

        "receipt_promotable_to_live_burn":
            false
    });

    let release_receipt = json!({
        "schema":
            "rox-anchor.phase8-internal-roc-release-intent.v1",

        "phase":
            "BUILD_PLAN4 Phase 8A",

        "receipt_role":
            "dry_run_internal_roc_release_intent",

        "cluster":
            "devnet",

        "direction":
            "rox_to_roc",

        "operation_id":
            PHASE8_OPERATION_ID,

        "idempotency_key":
            PHASE8_IDEMPOTENCY_KEY,

        "nonce":
            PHASE8_NONCE,

        "test_amount_minor":
            PHASE8_AMOUNT_MINOR.to_string(),

        "release_intent_report":
            release_intent.redacted_report_lines(),

        "real_internal_roc_release":
            false,

        "svc_wallet_call":
            false,

        "ron_ledger_mutation":
            false,

        "paid_content_unlock":
            false,

        "future_real_roc_path":
            "svc-wallet -> ron-ledger only",

        "settlement_claim":
            false
    });

    write_new_json(simulation_out, &simulation_receipt)?;

    write_new_json(release_out, &release_receipt)?;

    Ok([
        "phase8_rox_to_roc_simulation: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 8A".to_string(),
        "cluster: devnet".to_string(),
        "direction: rox_to_roc".to_string(),
        "phase7f_closeout: verified".to_string(),
        "two_source_post_phase7_state: Agreement".to_string(),
        format!("source_1_context_slot: {}", state.source1_slot,),
        format!("source_2_context_slot: {}", state.source2_slot,),
        "pre_simulation_mint_supply_minor: 1".to_string(),
        "pre_simulation_workflow_token_amount_minor: 1".to_string(),
        "fresh_phase8_operation_pda: absent".to_string(),
        "proof_review: accepted".to_string(),
        "coordinator_decision: accepted".to_string(),
        "relayer_dry_run: accepted".to_string(),
        "exact_instruction_1: observe_burn".to_string(),
        "exact_instruction_2: finalize_rox_to_roc_burn".to_string(),
        "live_devnet_simulate_transaction: GREEN".to_string(),
        format!("simulation_context_slot: {}", simulation.context_slot,),
        "post_simulation_persistent_mint_supply_minor: 1".to_string(),
        "post_simulation_persistent_workflow_token_amount_minor: 1".to_string(),
        "operation_persisted_after_simulation: false".to_string(),
        "dry_run_internal_roc_release_intent: GREEN".to_string(),
        "future_real_roc_path: svc-wallet -> ron-ledger only".to_string(),
        "keypair_loading: false".to_string(),
        "signature_generation: false".to_string(),
        "transaction_submission: false".to_string(),
        "persistent_rox_burn: false".to_string(),
        "real_internal_roc_release: false".to_string(),
        "production_settlement: false".to_string(),
        "mainnet_authorized: false".to_string(),
        format!("simulation_receipt: {}", redact_path(simulation_out),),
        format!("release_intent_receipt: {}", redact_path(release_out),),
        "next_action: PHASE8_LIVE_BURN_REQUIRES_NEW_EXPLICIT_APPROVAL".to_string(),
    ]
    .join("\n"))
}
