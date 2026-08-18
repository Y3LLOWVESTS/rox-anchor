//! RO:WHAT — Private-pilot command group for safe pilot operation reports.
//! RO:WHY — BUILD_PLAN3 Phase 10 makes pilot status/proof/simulation/submit/receipt/drill paths usable.
//! RO:INTERACTS — status, proof, submit, receipts, drill commands plus relayer simulation models.
//! RO:INVARIANTS — every non-read-only pilot path is explicit; unknown/ambiguous flags fail closed.
//! RO:SECURITY — default pilot reports are inert; explicit Phase 4 live simulation may read RPC and load/sign pilot keys but has no transaction-send path.
//! RO:TEST — cargo test -p rox-anchor-cli --test private_pilot_cli.

use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, InternalRocDryRunBurnIntent, InternalRocDryRunReleaseIntent, MintId, Nonce,
    OperationId, ProgramId, RecoveryPosture, SubmissionMode, TokenAccountId,
};
use rox_anchor_proof::{
    fixtures, review_proof_package, EvidenceBundle, ExpectedProofBinding, ProofPackage, ReplaySet,
};
use rox_anchor_relayer::{
    simulate_private_pilot_transaction_plan, PrivatePilotSimulationPlan,
    PrivatePilotTransactionKind, PrivatePilotTransactionStep, RelayerConfig, RelayerDryRun,
    RelayerSubmissionRequest, TransactionSimulationPlan,
};

use crate::{commands, CliError};

pub fn run_pilot(args: &[String]) -> Result<String, CliError> {
    let Some((subcommand, rest)) = args.split_first() else {
        return Ok(pilot_help());
    };

    match subcommand.as_str() {
        "--help" | "-h" | "help" => Ok(pilot_help()),
        "status" => {
            reject_extra_args("status", rest)?;
            Ok(wrap_pilot_report("status", commands::status::status_report()))
        }
        "phase5-read-only-live" | "live-read-only-evidence" => {
            let report =
                commands::phase5_live_read_only::run_phase5_live_read_only(rest)?;
            Ok(wrap_pilot_report(
                "phase5-read-only-live",
                report,
            ))
        }
        "phase5-read-only-quorum" | "live-read-only-quorum" => {
            let report =
                commands::phase5_live_quorum::run_phase5_live_quorum(rest)?;
            Ok(wrap_pilot_report(
                "phase5-read-only-quorum",
                report,
            ))
        }

        "phase5-read-only-closeout" | "live-read-only-closeout" => {
            let report =
                commands::phase5_live_closeout::run_phase5_live_closeout(rest)?;
            Ok(wrap_pilot_report(
                "phase5-read-only-closeout",
                report,
            ))
        }
        "phase6-actual-address-simulation-gate" | "actual-address-simulation-gate" => {
            let report =
                commands::phase6_live_simulation::run_phase6_actual_address_simulation_gate(rest)?;
            Ok(wrap_pilot_report(
                "phase6-actual-address-simulation-gate",
                report,
            ))
        }
        "phase6-live-rpc-simulation" | "actual-address-live-simulation" => {
            let report =
                commands::phase6_live_rpc_simulation::run_phase6_live_rpc_simulation(rest)?;
            Ok(wrap_pilot_report(
                "phase6-live-rpc-simulation",
                report,
            ))
        }
        "phase7-prepare-capped-roc-to-rox" | "prepare-actual-roc-to-rox-send" => {
            let report =
                commands::phase7_live_capped_sender::run_phase7_prepare_capped_roc_to_rox(rest)?;
            Ok(wrap_pilot_report(
                "phase7-prepare-capped-roc-to-rox",
                report,
            ))
        }
        "phase7-simulate-and-authorize-roc-to-rox" | "phase7-live-simulation-authorization" => {
            let report =
                commands::phase7_live_simulation_authorization::run_phase7_simulate_and_authorize(rest)?;
            Ok(wrap_pilot_report(
                "phase7-simulate-and-authorize-roc-to-rox",
                report,
            ))
        }
        "phase7-execute-capped-roc-to-rox" | "execute-actual-roc-to-rox-send" => {
            commands::phase7_live_manual_execution::run_phase7_live_manual_execution(rest)
        }
        "proof" | "read-only-proof" | "read-only" => {
            reject_extra_args("read-only-proof", rest)?;
            Ok(wrap_pilot_report(
                "read-only-proof",
                commands::proof::proof_help(),
            ))
        }
        "phase7-post-send-closeout" | "closeout-actual-roc-to-rox-send" => {
            let report =
                commands::phase7_live_closeout::run_phase7_post_send_closeout(rest)?;
            Ok(wrap_pilot_report(
                "phase7-post-send-closeout",
                report,
            ))
        }
        "initialize-test-only-mint" | "init-test-only-mint" => {
            let report =
                commands::test_only_init::run_initialize_test_only_mint(rest)?;
            Ok(wrap_pilot_report(
                "initialize-test-only-mint",
                report,
            ))
        }
        "simulate" | "simulation" => run_pilot_simulate(rest),
        "roc-to-rox" | "roc-to-rox-pilot" => run_pilot_roc_to_rox(rest),
        "rox-to-roc" | "rox-to-roc-pilot" => run_pilot_rox_to_roc(rest),
        "phase8-execute-capped-rox-to-roc-burn"
        | "execute-actual-rox-to-roc-burn" => {
            commands::phase8_live_execution::run_phase8_live_execution(rest)
        }
        "phase8-simulate-rox-to-roc-burn"
        | "simulate-actual-rox-to-roc-burn" => {
            let report =
                commands::phase8_rox_to_roc_simulation::
                    run_phase8_rox_to_roc_simulation(rest)?;
            Ok(wrap_pilot_report(
                "phase8-simulate-rox-to-roc-burn",
                report,
            ))
        }
        "submit-capped" | "capped-submit" | "capped-testnet" => {
            let report = commands::submit::run_submit_capped(rest)?;
            Ok(wrap_pilot_report("submit-capped", report))
        }
        "receipts" | "receipt-ledger" => {
            reject_extra_args("receipts", rest)?;
            Ok(wrap_pilot_report(
                "receipts",
                commands::receipts::receipt_report(),
            ))
        }
        "drill" | "halt-recovery-drill" => {
            let report = commands::drill::run_drill(rest)?;
            Ok(wrap_pilot_report("drill", report))
        }
        "halt" => {
            reject_extra_args("halt", rest)?;
            Ok(wrap_pilot_report("halt", commands::halt::halt_report()))
        }
        "recover" | "recovery" => {
            reject_extra_args("recover", rest)?;
            Ok(wrap_pilot_report("recover", commands::recover::recovery_report()))
        }
        other => Err(CliError::UnknownPilotFlag(format!(
            "pilot subcommand `{other}`; expected status, read-only-proof, initialize-test-only-mint, simulate, roc-to-rox, rox-to-roc, submit-capped, receipts, drill, halt, or recover"
        ))),
    }
}

fn run_pilot_roc_to_rox(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_roc_to_rox_args(args)?;

    if parsed.help {
        return Ok(pilot_roc_to_rox_help());
    }

    if !parsed.explicit_simulate_only {
        return Err(CliError::UnknownPilotFlag(
            "pilot roc-to-rox requires explicit --simulate-only".to_string(),
        ));
    }

    let package = fixtures::valid_package();
    let proof_review = review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    );
    let burn_intent = InternalRocDryRunBurnIntent::new(
        non_submitting_relayer_config().safety,
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        package.nonce.clone(),
        package.source_account.clone(),
        "test-only-private-roc-to-rox-burn-intent",
        10,
    )
    .expect("static private ROC-to-ROX burn intent should validate");

    let simulation = build_simulation_report(!parsed.missing_read_only_rpc);

    let mut lines = vec![
        "rox-anchor pilot".to_string(),
        "command: pilot roc-to-rox".to_string(),
        "scope: private_roc_to_rox_testnet_pilot".to_string(),
        "pilot_subcommand: roc-to-rox".to_string(),
        "explicit_simulate_only: true".to_string(),
        format!("proof_decision: {:?}", proof_review.decision),
        "crablink_internal_roc_burn_intent: dry_run_only".to_string(),
        "real_internal_roc_burn: disabled".to_string(),
        "test_rox_mint_path: simulation_or_explicit_capped_testnet_only".to_string(),
        "public_rox_mint: disabled".to_string(),
        "read_only_rpc_gate: required".to_string(),
        format!(
            "read_only_rpc_gate_fixture: {}",
            if parsed.missing_read_only_rpc {
                "missing"
            } else {
                "verified"
            }
        ),
        "network_submission: disabled_in_cli_report".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "signing: disabled".to_string(),
        "mint_burn_execution: disabled_in_cli_report".to_string(),
        "internal_roc_mutation: disabled".to_string(),
        "private_testnet_send_attempted: false".to_string(),
        "settlement_claim: none".to_string(),
        "public_launch_authorization: none".to_string(),
        "--- burn intent ---".to_string(),
    ];

    lines.extend(burn_intent.redacted_report_lines());
    lines.push("--- simulation ---".to_string());
    lines.extend(simulation);

    if parsed.authorize_capped {
        let mut submit_args = vec!["--authorize-testnet-submit-capped".to_string()];
        if parsed.receipt_persisted {
            submit_args.push("--receipt-persisted".to_string());
        }

        lines.push("--- capped submit authorization report ---".to_string());
        lines.push(commands::submit::run_submit_capped(&submit_args)?);
    } else {
        lines.push("capped_submit_authorization: not_requested".to_string());
    }

    Ok(lines.join("\n"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct PilotRocToRoxArgs {
    explicit_simulate_only: bool,
    authorize_capped: bool,
    receipt_persisted: bool,
    missing_read_only_rpc: bool,
    help: bool,
}

fn parse_roc_to_rox_args(args: &[String]) -> Result<PilotRocToRoxArgs, CliError> {
    let mut parsed = PilotRocToRoxArgs::default();

    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => parsed.help = true,
            "--simulate-only" => parsed.explicit_simulate_only = true,
            "--authorize-testnet-submit-capped" => parsed.authorize_capped = true,
            "--receipt-persisted" => parsed.receipt_persisted = true,
            "--missing-read-only-rpc" => parsed.missing_read_only_rpc = true,
            other => {
                return Err(CliError::UnknownPilotFlag(format!(
                    "pilot roc-to-rox flag `{other}`; expected --simulate-only, --authorize-testnet-submit-capped, --receipt-persisted, --missing-read-only-rpc, or --help"
                )));
            }
        }
    }

    Ok(parsed)
}

fn run_pilot_rox_to_roc(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_rox_to_roc_args(args)?;

    if parsed.help {
        return Ok(pilot_rox_to_roc_help());
    }

    if !parsed.explicit_simulate_only {
        return Err(CliError::UnknownPilotFlag(
            "pilot rox-to-roc requires explicit --simulate-only".to_string(),
        ));
    }

    let package = rox_to_roc_package();
    let expected = package.expected_binding_snapshot();
    let proof_review = review_proof_package(&package, &expected, &ReplaySet::default());
    let release_intent = InternalRocDryRunReleaseIntent::new(
        non_submitting_relayer_config().safety,
        package.operation_id.clone(),
        package.idempotency_key.clone(),
        package.nonce.clone(),
        package.target_account.clone(),
        "test-only-private-rox-to-roc-release-intent",
        10,
    )
    .expect("static private ROX-to-ROC release intent should validate");

    let simulation = build_rox_to_roc_simulation_report(!parsed.missing_read_only_rpc);

    let mut lines = vec![
        "rox-anchor pilot".to_string(),
        "command: pilot rox-to-roc".to_string(),
        "scope: private_rox_to_roc_testnet_pilot".to_string(),
        "pilot_subcommand: rox-to-roc".to_string(),
        "explicit_simulate_only: true".to_string(),
        format!("proof_decision: {:?}", proof_review.decision),
        "test_rox_burn_evidence: read_only_rpc_verified_or_simulated_fixture".to_string(),
        "internal_roc_release_intent: dry_run_only".to_string(),
        "real_internal_roc_release: disabled".to_string(),
        "future_real_roc_path: svc-wallet -> ron-ledger only".to_string(),
        "svc_wallet_call: disabled".to_string(),
        "ron_ledger_mutation: disabled".to_string(),
        "paid_content_unlock: disabled".to_string(),
        "read_only_rpc_gate: required".to_string(),
        format!(
            "read_only_rpc_gate_fixture: {}",
            if parsed.missing_read_only_rpc {
                "missing"
            } else {
                "verified"
            }
        ),
        "network_submission: disabled_in_cli_report".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "signing: disabled".to_string(),
        "mint_burn_execution: disabled_in_cli_report".to_string(),
        "internal_roc_mutation: disabled".to_string(),
        "private_testnet_send_attempted: false".to_string(),
        "settlement_claim: none".to_string(),
        "public_launch_authorization: none".to_string(),
        "--- release intent ---".to_string(),
    ];

    lines.extend(release_intent.redacted_report_lines());
    lines.push("--- simulation ---".to_string());
    lines.extend(simulation);
    lines.push("capped_submit_authorization: not_applicable_to_internal_roc_release".to_string());

    Ok(lines.join("\n"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct PilotRoxToRocArgs {
    explicit_simulate_only: bool,
    missing_read_only_rpc: bool,
    help: bool,
}

fn parse_rox_to_roc_args(args: &[String]) -> Result<PilotRoxToRocArgs, CliError> {
    let mut parsed = PilotRoxToRocArgs::default();

    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => parsed.help = true,
            "--simulate-only" => parsed.explicit_simulate_only = true,
            "--missing-read-only-rpc" => parsed.missing_read_only_rpc = true,
            other => {
                return Err(CliError::UnknownPilotFlag(format!(
                    "pilot rox-to-roc flag `{other}`; expected --simulate-only, --missing-read-only-rpc, or --help"
                )));
            }
        }
    }

    Ok(parsed)
}

fn run_pilot_simulate(args: &[String]) -> Result<String, CliError> {
    let parsed = parse_simulate_args(args)?;

    if parsed.help {
        return Ok(pilot_simulate_help());
    }

    if !parsed.explicit_simulate_only {
        return Err(CliError::UnknownPilotFlag(
            "pilot simulate requires explicit --simulate-only".to_string(),
        ));
    }

    let simulation = build_simulation_report(!parsed.missing_read_only_rpc);

    let mut lines = vec![
        "rox-anchor pilot".to_string(),
        "command: pilot simulate".to_string(),
        "scope: private_pilot_command_surface".to_string(),
        "pilot_subcommand: simulate".to_string(),
        "explicit_simulate_only: true".to_string(),
        "read_only_rpc_gate: required".to_string(),
        format!(
            "read_only_rpc_gate_fixture: {}",
            if parsed.missing_read_only_rpc {
                "missing"
            } else {
                "verified"
            }
        ),
        "network_submission: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "signing: disabled".to_string(),
        "mint_burn_execution: disabled".to_string(),
        "internal_roc_mutation: disabled".to_string(),
        "settlement_claim: none".to_string(),
        "public_launch_authorization: none".to_string(),
    ];

    lines.extend(simulation);

    Ok(lines.join("\n"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct PilotSimulateArgs {
    explicit_simulate_only: bool,
    missing_read_only_rpc: bool,
    help: bool,
}

fn parse_simulate_args(args: &[String]) -> Result<PilotSimulateArgs, CliError> {
    let mut parsed = PilotSimulateArgs::default();

    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => parsed.help = true,
            "--simulate-only" => parsed.explicit_simulate_only = true,
            "--missing-read-only-rpc" => parsed.missing_read_only_rpc = true,
            other => {
                return Err(CliError::UnknownPilotFlag(format!(
                    "pilot simulate flag `{other}`; expected --simulate-only, --missing-read-only-rpc, or --help"
                )));
            }
        }
    }

    Ok(parsed)
}

fn rox_to_roc_binding() -> AnchorBinding {
    AnchorBinding::new(
        DomainId::new("solana-devnet-rox-private-pilot-test").unwrap(),
        DomainId::new("internal-roc-private-pilot-test").unwrap(),
        AnchorDirection::RoxToRoc,
        ClusterId::new("devnet").unwrap(),
        ProgramId::new("PrivatePilotRoxAnchorProgram11111111").unwrap(),
        MintId::new("TestOnlyPrivatePilotRoxMint111111111").unwrap(),
        TokenAccountId::new("PrivatePilotRoxBurnSourceToken111111").unwrap(),
    )
}

fn rox_to_roc_package() -> ProofPackage {
    let binding = rox_to_roc_binding();

    ProofPackage::new(
        binding,
        OperationId::new("private-rox-to-roc-op-0001").unwrap(),
        IdempotencyKey::new("private-rox-to-roc-idem-0001").unwrap(),
        Nonce::new("private-rox-to-roc-nonce-0001").unwrap(),
        AccountId::new("private-rox-burn-source-0001").unwrap(),
        AccountId::new("crablink-private-roc-release-target-0001").unwrap(),
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    )
}

fn rox_to_roc_expected_binding() -> ExpectedProofBinding {
    rox_to_roc_package().expected_binding_snapshot()
}

fn build_simulation_report(read_only_rpc_verified: bool) -> Vec<String> {
    let package = fixtures::valid_package();
    let review = review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    );

    let mut relayer = RelayerDryRun::new(non_submitting_relayer_config());
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "pilot-cli-simulation-target",
            review,
        ))
        .expect("static pilot CLI simulation dry-run should fit receipt capacity");

    let base_plan = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base_plan)
        .with_read_only_rpc_verified(read_only_rpc_verified)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Observe,
                "pilot-cli-observe-test-only",
                1,
            ),
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Finalize,
                "pilot-cli-finalize-test-only",
                1,
            ),
        ]);

    let result = simulate_private_pilot_transaction_plan(non_submitting_relayer_config(), plan);
    result.redacted_report_lines()
}

fn build_rox_to_roc_simulation_report(read_only_rpc_verified: bool) -> Vec<String> {
    let package = rox_to_roc_package();
    let expected = rox_to_roc_expected_binding();
    let review = review_proof_package(&package, &expected, &ReplaySet::default());

    let mut relayer = RelayerDryRun::new(non_submitting_relayer_config());
    let dry_run = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "pilot-cli-rox-to-roc-release-intent-target",
            review,
        ))
        .expect("static ROX-to-ROC pilot CLI dry-run should fit receipt capacity");

    let base_plan = TransactionSimulationPlan::from_dry_run_receipt(dry_run, true, 2);
    let plan = PrivatePilotSimulationPlan::new(base_plan)
        .with_read_only_rpc_verified(read_only_rpc_verified)
        .with_steps(vec![
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Observe,
                "pilot-cli-observe-test-rox-burn",
                1,
            ),
            PrivatePilotTransactionStep::new(
                PrivatePilotTransactionKind::Finalize,
                "pilot-cli-release-intent-only",
                1,
            ),
        ]);

    let result = simulate_private_pilot_transaction_plan(non_submitting_relayer_config(), plan);
    result.redacted_report_lines()
}

fn non_submitting_relayer_config() -> RelayerConfig {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );

    RelayerConfig::new_with_safety(3, 16, safety)
}

fn reject_extra_args(subcommand: &'static str, args: &[String]) -> Result<(), CliError> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(CliError::UnknownPilotFlag(format!(
            "pilot {subcommand} does not accept `{}`",
            args[0]
        )))
    }
}

fn wrap_pilot_report(subcommand: &'static str, body: String) -> String {
    [
        "rox-anchor pilot".to_string(),
        format!("command: pilot {subcommand}"),
        "scope: private_pilot_command_surface".to_string(),
        format!("pilot_subcommand: {subcommand}"),
        "unsafe_defaults: rejected".to_string(),
        "network_submission: disabled_unless_explicit_capped_submit_authorizes".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "signing: disabled_in_cli_report".to_string(),
        "mint_burn_execution: disabled_in_cli_report".to_string(),
        "internal_roc_mutation: disabled".to_string(),
        "settlement_claim: none".to_string(),
        "public_launch_authorization: none".to_string(),
        "---".to_string(),
        body,
    ]
    .join("\n")
}

fn pilot_help() -> String {
    [
        "rox-anchor pilot",
        "",
        "Private-pilot command group with safe defaults.",
        "",
        "subcommands:",
        "  status                         show private pilot status/report surfaces",
        "  read-only-proof                show read-only RPC proof report",
        "  phase5-read-only-quorum       run fixed two-source Phase 5B read-only quorum",
        "  phase5-read-only-closeout      run Phase 5B2 loader metadata closeout",
        "  initialize-test-only-mint --prepare-only|--simulate-live",
        "                                  prepare inputs or explicitly simulate the atomic init",
        "  simulate --simulate-only       run local simulation-only pilot report",
        "  roc-to-rox --simulate-only      run private forward ROC-to-ROX pilot report",
        "  rox-to-roc --simulate-only      run private reverse ROX-to-ROC pilot report",
        "  phase7-prepare-capped-roc-to-rox prepare exact Phase 7 two-instruction candidate",
        "  phase7-execute-capped-roc-to-rox LIVE DEVNET one-shot forward execution; explicit flags required",
        "  phase8-simulate-rox-to-roc-burn simulate exact one-unit reverse burn without submitting",
        "  submit-capped                  run explicit capped-submit authorization report",
        "  receipts                       inspect deterministic pilot receipt ledger",
        "  drill                          run halt/recovery drill in pilot mode",
        "  halt                           show halt posture notes",
        "  recover                        show recovery posture notes",
        "",
        "safety rules:",
        "  no default send path",
        "  no mainnet mode",
        "  no public launch mode",
        "  no production settlement mode",
        "  no wallet/key loading by default; Phase 4 live simulation is explicit",
        "  no silent RPC submission",
        "  no mint/burn execution from CLI reports",
        "  non-read-only pilot paths require explicit flags",
    ]
    .join("\n")
}

fn pilot_simulate_help() -> String {
    [
        "rox-anchor pilot simulate",
        "",
        "Simulation-only private-pilot transaction plan report.",
        "",
        "usage:",
        "  rox-anchor pilot simulate --simulate-only",
        "",
        "flags:",
        "  --simulate-only             required explicit simulation-only intent",
        "  --missing-read-only-rpc     fixture: prove read-only RPC gate blocks simulation",
        "",
        "security:",
        "  no RPC submission",
        "  no wallet/key loading by default; Phase 4 live simulation is explicit",
        "  no signing",
        "  no mint/burn execution",
        "  no internal ROC mutation",
        "  no settlement or finality claim",
    ]
    .join("\n")
}

fn pilot_roc_to_rox_help() -> String {
    [
        "rox-anchor pilot roc-to-rox",
        "",
        "Private ROC-to-ROX pilot flow report using test-only inputs.",
        "",
        "usage:",
        "  rox-anchor pilot roc-to-rox --simulate-only",
        "",
        "flags:",
        "  --simulate-only                       required explicit simulation-only intent",
        "  --authorize-testnet-submit-capped     include capped testnet authorization report",
        "  --receipt-persisted                   fixture: receipt persistence gate satisfied",
        "  --missing-read-only-rpc               fixture: prove read-only RPC gate blocks simulation",
        "",
        "security:",
        "  no real internal ROC burn",
        "  no public ROX mint",
        "  no wallet/key loading by default; Phase 4 live simulation is explicit",
        "  no signing",
        "  no CLI transaction submission",
        "  no settlement or finality claim",
    ]
    .join("\n")
}

fn pilot_rox_to_roc_help() -> String {
    [
        "rox-anchor pilot rox-to-roc",
        "",
        "Private ROX-to-ROC pilot flow report using test-only burn evidence.",
        "",
        "usage:",
        "  rox-anchor pilot rox-to-roc --simulate-only",
        "",
        "flags:",
        "  --simulate-only           required explicit simulation-only intent",
        "  --missing-read-only-rpc   fixture: prove read-only RPC gate blocks simulation",
        "",
        "security:",
        "  no real internal ROC release",
        "  no svc-wallet call",
        "  no ron-ledger mutation",
        "  no wallet/key loading by default; Phase 4 live simulation is explicit",
        "  no signing",
        "  no CLI transaction submission",
        "  no settlement or finality claim",
    ]
    .join("\n")
}
