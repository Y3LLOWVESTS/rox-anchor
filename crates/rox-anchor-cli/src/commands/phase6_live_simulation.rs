//! BUILD_PLAN4 Phase 6 actual-address simulation gate foundation.
//!
//! This module binds the completed live Phase 5 evidence to the actual
//! deployed Devnet program/config/mint/token-account identities and runs
//! those bindings through the existing proof, coordinator, relayer, and
//! simulate-only local gate models.
//!
//! This slice deliberately does not call Solana RPC simulation yet.
//! No keypair is loaded, no signature is generated, and no transaction is
//! submitted. The subsequent Phase 6B slice adds only the live
//! `simulateTransaction` execution after this foundation is compile-tested.

#![forbid(unsafe_code)]

use std::{fs, path::Path};

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
use serde_json::Value;

use crate::CliError;

pub const PHASE6_PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";
pub const PHASE6_CONFIG_ACCOUNT: &str = "4RBTypWtrn7mwV47MJkAHtEBMYnvNhd5wdSMAUsxwFeo";
pub const PHASE6_ROX_MINT: &str = "HfHRJLswuRN3eVsiWnYi7REssDEsxxA8ewU8emhC3XA4";
pub const PHASE6_TOKEN_ACCOUNT: &str = "A3sBYMUf2N7rpkqiCnE7fKZBdnGR5goH3hFmHJvgvqsJ";
pub const PHASE6_MINT_AUTHORITY: &str = "C5jTCy4EBY5fKuRMzLv7Lau5Re1SmMXukRXosndk9hJE";
pub const PHASE6_WORKFLOW_AUTHORITY: &str = "6YYJ43KRJF6pB3jUtRQpvhVHZQHaURTSxJdLpipHU3gs";
pub const PHASE6_HALT_AUTHORITY: &str = "3aAvoLEAsCCte4gow6rheJQ3F4zeoCuMvERqyFBobGgz";
pub const PHASE6_RECOVERY_AUTHORITY: &str = "74upNee16zSKS2hSuovDaioWVsadFf8Za4CCRwJW5fqe";

pub const PHASE6_SOURCE1: &str = "solana-public-devnet-primary";
pub const PHASE6_SOURCE2: &str = "uniblock-devnet-secondary";

pub const PHASE6_OPERATION_ID: &str = "actual-simulation-op-0001";
pub const PHASE6_IDEMPOTENCY_KEY: &str = "actual-simulation-idem-0001";
pub const PHASE6_NONCE: &str = "actual-simulation-nonce-0001";

pub(crate) const PHASE4_INITIALIZATION_SIGNATURE: &str =
    "5J8cjGr3idqUff4Mh5FeMSfEDoXpn5QAh6bxyWQrmpT1q4PeFCouRbThH5JN2dtLaHC1kC4QPcMApownmXeimyK5";

pub(crate) const PHASE6_STALE_AFTER_SLOTS: u64 = 100;
pub(crate) const PHASE6_REQUIRED_OBSERVATIONS: u16 = 2;
pub(crate) const PHASE6_AMOUNT_MINOR: u64 = 1;
pub(crate) const PHASE6_MAX_AMOUNT_MINOR: u64 = 1;
pub(crate) const PHASE6_MAX_OPERATIONS: u64 = 1;

#[derive(Clone, Debug)]
struct Phase6Args {
    help: bool,
    simulate_only: bool,
    phase5_receipt: Option<String>,
    receipt_out: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Phase5Evidence {
    pub(crate) source1_metadata_slot: u64,
    pub(crate) source2_metadata_slot: u64,
    pub(crate) metadata_slot_delta: u64,
}

fn phase6_error(message: impl Into<String>) -> CliError {
    CliError::UnknownPilotFlag(format!(
        "phase6-actual-address-simulation-gate {}",
        message.into()
    ))
}

fn parse_args(args: &[String]) -> Result<Phase6Args, CliError> {
    let mut parsed = Phase6Args {
        help: false,
        simulate_only: false,
        phase5_receipt: None,
        receipt_out: None,
    };

    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];

        match arg.as_str() {
            "--help" | "-h" | "help" => {
                parsed.help = true;
                index += 1;
            }
            "--simulate-only" => {
                parsed.simulate_only = true;
                index += 1;
            }
            "--phase5-receipt" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| phase6_error("--phase5-receipt requires a value"))?;
                parsed.phase5_receipt = Some(value.clone());
                index += 2;
            }
            "--receipt-out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| phase6_error("--receipt-out requires a value"))?;
                parsed.receipt_out = Some(value.clone());
                index += 2;
            }
            _ => {
                if let Some(value) = arg.strip_prefix("--phase5-receipt=") {
                    parsed.phase5_receipt = Some(value.to_owned());
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--receipt-out=") {
                    parsed.receipt_out = Some(value.to_owned());
                    index += 1;
                } else {
                    return Err(phase6_error(format!("unknown flag `{arg}`")));
                }
            }
        }
    }

    Ok(parsed)
}

fn help_text() -> String {
    [
        "rox-anchor pilot phase6-actual-address-simulation-gate",
        "",
        "BUILD_PLAN4 Phase 6A actual-address simulation gate.",
        "",
        "required:",
        "  --simulate-only",
        "  --phase5-receipt <path>",
        "  --receipt-out <path>",
        "",
        "behavior:",
        "  - validates the real completed Phase 5 closeout receipt",
        "  - binds the exact deployed Devnet program/config/mint/token account",
        "  - runs existing proof/coordinator/relayer simulate-only gates",
        "  - does not call live RPC in this Phase 6A slice",
        "  - does not load a wallet or keypair",
        "  - does not sign",
        "  - does not submit",
        "  - does not mint or burn",
        "  - does not mutate internal ROC",
        "  - does not write the final Phase 6 simulation receipt yet",
    ]
    .join("\n")
}

fn required_str<'a>(value: &'a Value, key: &str) -> Result<&'a str, CliError> {
    value.get(key).and_then(Value::as_str).ok_or_else(|| {
        phase6_error(format!(
            "Phase 5 receipt field `{key}` is missing or not a string"
        ))
    })
}

fn required_bool(value: &Value, key: &str) -> Result<bool, CliError> {
    value.get(key).and_then(Value::as_bool).ok_or_else(|| {
        phase6_error(format!(
            "Phase 5 receipt field `{key}` is missing or not boolean"
        ))
    })
}

fn required_u64(value: &Value, key: &str) -> Result<u64, CliError> {
    value.get(key).and_then(Value::as_u64).ok_or_else(|| {
        phase6_error(format!(
            "Phase 5 receipt field `{key}` is missing or not u64"
        ))
    })
}

fn require_str(value: &Value, key: &str, expected: &str) -> Result<(), CliError> {
    let observed = required_str(value, key)?;

    if observed != expected {
        return Err(phase6_error(format!(
            "Phase 5 receipt `{key}` mismatch: expected `{expected}`, observed `{observed}`"
        )));
    }

    Ok(())
}

fn require_bool(value: &Value, key: &str, expected: bool) -> Result<(), CliError> {
    let observed = required_bool(value, key)?;

    if observed != expected {
        return Err(phase6_error(format!(
            "Phase 5 receipt `{key}` mismatch: expected `{expected}`, observed `{observed}`"
        )));
    }

    Ok(())
}

pub(crate) fn validate_phase5_receipt(path: &Path) -> Result<Phase5Evidence, CliError> {
    let raw = fs::read_to_string(path)
        .map_err(|error| phase6_error(format!("could not read Phase 5 receipt: {error}")))?;

    let value: Value = serde_json::from_str(&raw)
        .map_err(|error| phase6_error(format!("could not decode Phase 5 receipt JSON: {error}")))?;

    require_str(&value, "schema", "rox-anchor.phase5-read-only-closeout.v1")?;
    require_str(&value, "phase", "BUILD_PLAN4 Phase 5B2")?;
    require_str(&value, "cluster", "devnet")?;
    require_str(&value, "program_id", PHASE6_PROGRAM_ID)?;
    require_str(&value, "rpc_source_1", PHASE6_SOURCE1)?;
    require_str(&value, "rpc_source_2", PHASE6_SOURCE2)?;
    require_str(&value, "rpc_proof_decision", "Agreement")?;

    let observations = required_u64(&value, "rpc_proof_observation_count")?;
    let required = required_u64(&value, "rpc_proof_required_observations")?;

    if observations != PHASE6_REQUIRED_OBSERVATIONS as u64
        || required != PHASE6_REQUIRED_OBSERVATIONS as u64
    {
        return Err(phase6_error(format!(
            "Phase 5 quorum mismatch: observations={observations}, required={required}"
        )));
    }

    for key in [
        "rpc_sources_distinct",
        "fresh_state_quorum",
        "state_quorum_fresh_by_rpc_proof",
        "metadata_min_context_enforced",
        "metadata_pair_fresh",
        "programdata_pointer_multi_source_verified",
        "upgrade_authority_multi_source_verified",
        "deployment_metadata_multi_source_verified",
        "program_binary_multi_source_verified",
        "deploy_signature_multi_source_verified",
        "initialization_signature_multi_source_verified",
        "state_readback_both_sources",
        "phase5_closeout",
        "read_only_rpc",
    ] {
        require_bool(&value, key, true)?;
    }

    for key in [
        "transaction_submission",
        "keypair_loading",
        "signing",
        "simulation",
        "rox_mint_performed",
        "rox_burn_performed",
        "real_roc_mutation",
        "production_settlement",
        "finality_claim",
        "settlement_claim",
        "mainnet",
    ] {
        require_bool(&value, key, false)?;
    }

    let stale_after = required_u64(&value, "stale_after_slots")?;

    if stale_after != PHASE6_STALE_AFTER_SLOTS {
        return Err(phase6_error(format!(
            "Phase 5 stale-after policy drifted: expected {}, observed {stale_after}",
            PHASE6_STALE_AFTER_SLOTS
        )));
    }

    let source1_metadata_slot = required_u64(&value, "rpc_source_1_metadata_slot")?;
    let source2_metadata_slot = required_u64(&value, "rpc_source_2_metadata_slot")?;
    let metadata_slot_delta = required_u64(&value, "metadata_source_slot_delta")?;

    let recomputed = source1_metadata_slot.abs_diff(source2_metadata_slot);

    if metadata_slot_delta != recomputed {
        return Err(phase6_error(format!(
            "Phase 5 metadata slot delta mismatch: recorded={metadata_slot_delta}, recomputed={recomputed}"
        )));
    }

    if metadata_slot_delta > PHASE6_STALE_AFTER_SLOTS {
        return Err(phase6_error(format!(
            "Phase 5 metadata evidence is stale: delta={metadata_slot_delta}"
        )));
    }

    Ok(Phase5Evidence {
        source1_metadata_slot,
        source2_metadata_slot,
        metadata_slot_delta,
    })
}

fn core_id<T, E>(result: Result<T, E>, label: &str) -> Result<T, CliError> {
    result.map_err(|_| phase6_error(format!("static Phase 6 `{label}` identifier is invalid")))
}

fn build_phase6_gate(evidence: &Phase5Evidence) -> Result<Vec<String>, CliError> {
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
        core_id(ProgramId::new(PHASE6_PROGRAM_ID), "program-id")?,
        core_id(MintId::new(PHASE6_ROX_MINT), "mint")?,
        core_id(TokenAccountId::new(PHASE6_TOKEN_ACCOUNT), "token-account")?,
    );

    let operation_id = core_id(OperationId::new(PHASE6_OPERATION_ID), "operation-id")?;
    let idempotency_key = core_id(
        IdempotencyKey::new(PHASE6_IDEMPOTENCY_KEY),
        "idempotency-key",
    )?;
    let nonce = core_id(Nonce::new(PHASE6_NONCE), "nonce")?;

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
            AccountId::new("crablink-private-roc-burn-source-0001"),
            "source-account",
        )?,
        core_id(
            AccountId::new("actual-private-rox-token-owner-0001"),
            "recipient-account",
        )?,
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
            RpcCommitmentLevel::Finalized,
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
            RpcCommitmentLevel::Finalized,
        ),
    ];

    let current_slot = evidence
        .source1_metadata_slot
        .max(evidence.source2_metadata_slot);

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
        current_slot,
    );

    if decision.status != CoordinatorDecisionStatus::Accepted
        || decision.proof_review.decision != ReviewDecision::Accepted
        || !decision.permits_transaction_simulation()
    {
        return Err(phase6_error(format!(
            "existing proof/coordinator gate rejected Phase 6 binding: coordinator={:?}, proof={:?}",
            decision.status,
            decision.proof_review.decision
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
                "phase6-actual-address-simulation-target",
                decision.proof_review.clone(),
            )
            .with_requested_attempts(1),
        )
        .map_err(|error| {
            phase6_error(format!(
                "relayer dry-run could not create receipt: {error:?}"
            ))
        })?;

    if dry_run.status != RelayerReceiptStatus::DryRunAccepted {
        return Err(phase6_error(format!(
            "relayer dry-run rejected Phase 6 binding: {:?}",
            dry_run.status
        )));
    }

    let base_plan = TransactionSimulationPlan::from_dry_run_receipt(
        dry_run,
        decision.permits_transaction_simulation(),
        1,
    );

    let simulation_plan = PrivatePilotSimulationPlan::new(base_plan)
        .with_read_only_rpc_verified(true)
        .with_steps(vec![PrivatePilotTransactionStep::new(
            PrivatePilotTransactionKind::Halt,
            "simulate-halt-against-actual-config-without-persistence",
            1,
        )]);

    let simulation = simulate_private_pilot_transaction_plan(relayer_config, simulation_plan);

    if simulation.status != PrivatePilotSimulationStatus::Simulated
        || !simulation.is_simulated()
        || simulation.live_submission
        || simulation.wallet_key_loading
        || simulation.internal_roc_mutation
    {
        return Err(phase6_error(format!(
            "existing relayer simulation gate rejected Phase 6 plan: {:?}",
            simulation.status
        )));
    }

    Ok(vec![
        "phase6_actual_address_gate: GREEN".to_string(),
        "phase: BUILD_PLAN4 Phase 6A".to_string(),
        "cluster: devnet".to_string(),
        format!("program_id: {PHASE6_PROGRAM_ID}"),
        "config_account_binding: actual_verified_constant".to_string(),
        "test_only_mint_binding: actual_verified_constant".to_string(),
        "test_only_token_account_binding: actual_verified_constant".to_string(),
        "mint_authority_binding: actual_verified_constant".to_string(),
        "workflow_authority_binding: actual_verified_constant".to_string(),
        "halt_authority_binding: actual_verified_constant".to_string(),
        "recovery_authority_binding: actual_verified_constant".to_string(),
        format!(
            "phase5_metadata_source_slot_delta: {}",
            evidence.metadata_slot_delta
        ),
        "phase5_read_only_evidence_status: verified".to_string(),
        format!("proof_review_status: {:?}", decision.proof_review.decision),
        format!("coordinator_decision_status: {:?}", decision.status),
        "relayer_dry_run_status: accepted".to_string(),
        "local_simulate_only_gate_status: simulated".to_string(),
        "planned_live_instruction_kind: halt".to_string(),
        format!("amount_minor: {PHASE6_AMOUNT_MINOR}"),
        format!("max_amount_minor: {PHASE6_MAX_AMOUNT_MINOR}"),
        format!("max_operations: {PHASE6_MAX_OPERATIONS}"),
        "live_rpc_simulation: deferred_until_phase6b".to_string(),
        "final_phase6_receipt_written: false".to_string(),
        "receipt_promotable_to_send: false".to_string(),
        "transaction_submission: false".to_string(),
        "keypair_loading: false".to_string(),
        "signing: false".to_string(),
        "rox_mint_execution: false".to_string(),
        "rox_burn_execution: false".to_string(),
        "real_roc_mutation: false".to_string(),
        "production_settlement: false".to_string(),
        "finality_claim: false".to_string(),
        "mainnet_authorized: false".to_string(),
        "next_action: BUILD_PHASE6B_LIVE_RPC_SIMULATE_TRANSACTION".to_string(),
    ])
}

pub fn run_phase6_actual_address_simulation_gate(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_args(args)?;

    if parsed.help {
        return Ok(help_text());
    }

    if !parsed.simulate_only {
        return Err(phase6_error("requires explicit --simulate-only"));
    }

    let phase5_receipt = parsed
        .phase5_receipt
        .as_deref()
        .ok_or_else(|| phase6_error("requires --phase5-receipt <path>"))?;

    let receipt_out = parsed
        .receipt_out
        .as_deref()
        .ok_or_else(|| phase6_error("requires --receipt-out <path>"))?;

    if receipt_out.trim().is_empty() {
        return Err(phase6_error("--receipt-out must not be empty"));
    }

    let evidence = validate_phase5_receipt(Path::new(phase5_receipt))?;

    let mut lines = build_phase6_gate(&evidence)?;

    lines.push(format!("future_receipt_out: {}", redact_path(receipt_out)));

    Ok(lines.join("\n"))
}

fn redact_path(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("<redacted-local-path>/{name}"))
        .unwrap_or_else(|| "<redacted-local-path>".to_string())
}
