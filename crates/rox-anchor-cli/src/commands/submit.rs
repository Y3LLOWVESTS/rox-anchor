//! RO:WHAT — CLI report for capped testnet submission authorization.
//! RO:WHY — Exposes BUILD_PLAN2 Phase 8 guard results without executing live submission.
//! RO:INTERACTS — rox-anchor-proof fixtures and rox-anchor-relayer capped submission model.
//! RO:INVARIANTS — explicit approval, persisted receipt, simulation, proof, dry-run, and caps are required.
//! RO:SECURITY — no RPC, wallet/key loading, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — covered by CLI capped-submit report tests.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, EvidenceBundle, ReplaySet};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, CappedTestnetSubmissionResult, RelayerConfig, RelayerDryRun,
    RelayerSubmissionRequest, TransactionSimulationPlan,
};

use crate::CliError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SubmitFixture {
    Accepted,
    Blocked,
}

impl SubmitFixture {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "accepted" | "valid" => Ok(Self::Accepted),
            "blocked" | "missing-evidence" => Ok(Self::Blocked),
            other => Err(CliError::UnknownSubmitFlag(format!(
                "--fixture value `{other}`; expected accepted or blocked"
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SubmitCappedArgs {
    fixture: SubmitFixture,
    explicit_operator_approval: bool,
    receipt_persisted: bool,
    requested_attempts: u8,
    requested_operations: u16,
    amount_units: u64,
    help: bool,
}

impl Default for SubmitCappedArgs {
    fn default() -> Self {
        Self {
            fixture: SubmitFixture::Accepted,
            explicit_operator_approval: false,
            receipt_persisted: false,
            requested_attempts: 1,
            requested_operations: 1,
            amount_units: 10,
            help: false,
        }
    }
}

pub fn run_submit_capped(args: &[String]) -> Result<String, CliError> {
    let args = parse_args(args)?;

    if args.help {
        return Ok(submit_capped_help());
    }

    let review = proof_review(args.fixture);
    let package = fixtures::valid_package();

    let mut relayer = RelayerDryRun::new(simulation_config());
    let dry_run_receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "cli-capped-testnet-submit-report",
            review,
        ))
        .expect("static CLI capped-submit dry-run should fit receipt capacity");

    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(dry_run_receipt, true, 2);
    let simulation_result = simulate_transaction_plan(simulation_config(), simulation_plan);

    let capped_plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation_result)
        .with_requested_attempts(args.requested_attempts)
        .with_requested_operations(args.requested_operations)
        .with_amount_units(args.amount_units)
        .with_explicit_operator_approval(args.explicit_operator_approval)
        .with_receipt_persisted(args.receipt_persisted);

    let result =
        authorize_capped_testnet_submission(capped_testnet_config(), limits(), capped_plan);

    Ok(render_submit_capped_report(args, &result))
}

fn parse_args(args: &[String]) -> Result<SubmitCappedArgs, CliError> {
    let mut parsed = SubmitCappedArgs::default();
    let mut index = 0;

    while index < args.len() {
        let arg = args[index].as_str();

        match arg {
            "--help" | "-h" => {
                parsed.help = true;
                index += 1;
            }
            "--authorize-testnet-submit-capped" | "--explicit-operator-approval" => {
                parsed.explicit_operator_approval = true;
                index += 1;
            }
            "--receipt-persisted" => {
                parsed.receipt_persisted = true;
                index += 1;
            }
            "--fixture" => {
                let value = required_value(args, index, "--fixture")?;
                parsed.fixture = SubmitFixture::parse(value)?;
                index += 2;
            }
            "--attempts" => {
                let value = required_value(args, index, "--attempts")?;
                parsed.requested_attempts = parse_u8_flag("--attempts", value)?;
                index += 2;
            }
            "--operations" => {
                let value = required_value(args, index, "--operations")?;
                parsed.requested_operations = parse_u16_flag("--operations", value)?;
                index += 2;
            }
            "--amount-units" => {
                let value = required_value(args, index, "--amount-units")?;
                parsed.amount_units = parse_u64_flag("--amount-units", value)?;
                index += 2;
            }
            _ => {
                if let Some(value) = arg.strip_prefix("--fixture=") {
                    parsed.fixture = SubmitFixture::parse(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--attempts=") {
                    parsed.requested_attempts = parse_u8_flag("--attempts", value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--operations=") {
                    parsed.requested_operations = parse_u16_flag("--operations", value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--amount-units=") {
                    parsed.amount_units = parse_u64_flag("--amount-units", value)?;
                    index += 1;
                } else {
                    return Err(CliError::UnknownSubmitFlag(arg.to_owned()));
                }
            }
        }
    }

    Ok(parsed)
}

fn required_value<'a>(
    args: &'a [String],
    index: usize,
    flag: &'static str,
) -> Result<&'a str, CliError> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| CliError::UnknownSubmitFlag(format!("{flag} requires a value")))
}

fn parse_u8_flag(flag: &'static str, value: &str) -> Result<u8, CliError> {
    value
        .parse::<u8>()
        .map_err(|_| CliError::UnknownSubmitFlag(format!("{flag} value `{value}` is not u8")))
}

fn parse_u16_flag(flag: &'static str, value: &str) -> Result<u16, CliError> {
    value
        .parse::<u16>()
        .map_err(|_| CliError::UnknownSubmitFlag(format!("{flag} value `{value}` is not u16")))
}

fn parse_u64_flag(flag: &'static str, value: &str) -> Result<u64, CliError> {
    value
        .parse::<u64>()
        .map_err(|_| CliError::UnknownSubmitFlag(format!("{flag} value `{value}` is not u64")))
}

fn proof_review(fixture: SubmitFixture) -> rox_anchor_proof::ProofReview {
    let mut package = fixtures::valid_package();

    if fixture == SubmitFixture::Blocked {
        package.evidence = EvidenceBundle::new(0, 2, 0);
    }

    review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn simulation_config() -> RelayerConfig {
    RelayerConfig::new(3, 16)
}

fn capped_testnet_config() -> RelayerConfig {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Testnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    );

    RelayerConfig::new_with_safety(3, 16, safety)
}

fn limits() -> CappedTestnetSubmissionLimits {
    CappedTestnetSubmissionLimits::new(2, 2, 100, true)
}

fn render_submit_capped_report(
    args: SubmitCappedArgs,
    result: &CappedTestnetSubmissionResult,
) -> String {
    [
        "rox-anchor capped testnet submission report".to_string(),
        "command: submit-capped".to_string(),
        "scope: testnet_only".to_string(),
        "mode: TestnetSubmitCapped".to_string(),
        "report_only: true".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "rpc_submission: disabled_in_cli_report".to_string(),
        "mint_burn_execution: disabled_in_cli_report".to_string(),
        "roc_release: disabled_in_cli_report".to_string(),
        format!("fixture: {}", args.fixture.as_str()),
        format!(
            "explicit_operator_approval: {}",
            args.explicit_operator_approval
        ),
        format!("receipt_persisted: {}", args.receipt_persisted),
        format!("requested_attempts: {}", result.requested_attempts),
        format!("requested_operations: {}", result.requested_operations),
        format!("amount_units: {}", result.amount_units),
        format!("proof_decision: {:?}", result.proof_decision),
        format!("relayer_status: {:?}", result.relayer_status),
        format!("simulation_status: {:?}", result.simulation_status),
        format!("capped_submit_status: {:?}", result.status),
        format!("authorized: {}", result.authorized),
        format!(
            "live_submission_permitted: {}",
            result.live_submission_permitted
        ),
        format!(
            "live_submission_attempted: {}",
            result.live_submission_attempted
        ),
        format!("network_submitted: {}", result.network_submitted),
        "finality_claim: none".to_string(),
        "settlement_claim: none".to_string(),
    ]
    .join("\n")
}

fn submit_capped_help() -> String {
    [
        "rox-anchor submit-capped",
        "",
        "Report-only capped testnet submission authorization.",
        "",
        "flags:",
        "  --authorize-testnet-submit-capped   explicit operator approval gate",
        "  --receipt-persisted                 prove local receipt persistence gate",
        "  --fixture <accepted|blocked>         choose deterministic proof fixture",
        "  --attempts <n>                       requested attempts, cap is 2",
        "  --operations <n>                     requested operations, cap is 2",
        "  --amount-units <n>                   requested amount units, cap is 100",
        "",
        "security:",
        "  no RPC submission",
        "  no wallet/key loading",
        "  no mint/burn execution",
        "  no ROC release",
        "  no settlement or finality claim",
    ]
    .join("\n")
}
