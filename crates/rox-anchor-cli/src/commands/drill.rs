//! RO:WHAT — CLI kill-switch drill report for halt and recovery authority behavior.
//! RO:WHY — BUILD_PLAN2 Phase 12 requires operator-visible halt/recovery drill output.
//! RO:INTERACTS — rox-anchor-core kill-switch review, authority map, posture model, and CLI dispatch.
//! RO:INVARIANTS — drill output is local/report-only and never claims network submission or finality.
//! RO:SECURITY — no live RPC, wallet/key loading, transaction send, mint, burn, ROC release, or settlement.
//! RO:TEST — cargo test -p rox-anchor-cli --test kill_switch_drill_command.

use rox_anchor_core::{
    review_kill_switch_drill, AnchorOperationalPosture, AuthorityAssignment, AuthorityKeyId,
    AuthorityMap, AuthoritySeparationMode, ChallengePosture, HaltPosture, KillSwitchAction,
    KillSwitchDrillRequest, KillSwitchDrillStage, OperatorRole, RecoveryPosture,
};

use crate::CliError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DrillAuthority {
    Halt,
    Recovery,
    Upgrade,
    Wrong,
}

impl DrillAuthority {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "halt" | "halt-authority" | "halt_authority" => Ok(Self::Halt),
            "recovery" | "recovery-authority" | "recovery_authority" => Ok(Self::Recovery),
            "upgrade" | "upgrade-authority" | "upgrade_authority" => Ok(Self::Upgrade),
            "wrong" | "wrong-authority" | "wrong_authority" => Ok(Self::Wrong),
            other => Err(CliError::UnknownDrillFlag(format!(
                "--authority value `{other}`; expected halt, recovery, upgrade, or wrong"
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Halt => "halt_authority",
            Self::Recovery => "recovery_authority",
            Self::Upgrade => "upgrade_authority",
            Self::Wrong => "wrong_authority",
        }
    }

    fn key(self) -> AuthorityKeyId {
        match self {
            Self::Halt => key("halt-authority-phase12-cli-key"),
            Self::Recovery => key("recovery-authority-phase12-cli-key"),
            Self::Upgrade => key("upgrade-authority-phase12-cli-key"),
            Self::Wrong => key("wrong-authority-phase12-cli-key"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DrillPosture {
    Clear,
    ChallengeOpen,
    ChallengeAccepted,
    Halted,
    RecoveryRequired,
    RecoveryInReview,
    HaltedRecoveryRequired,
    RecoveryResolved,
}

impl DrillPosture {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "clear" => Ok(Self::Clear),
            "challenge-open" | "challenge_open" => Ok(Self::ChallengeOpen),
            "challenge-accepted" | "challenge_accepted" => Ok(Self::ChallengeAccepted),
            "halted" => Ok(Self::Halted),
            "recovery-required" | "recovery_required" => Ok(Self::RecoveryRequired),
            "recovery-in-review" | "recovery_in_review" => Ok(Self::RecoveryInReview),
            "halted-recovery-required" | "halted_recovery_required" => {
                Ok(Self::HaltedRecoveryRequired)
            }
            "recovery-resolved" | "recovery_resolved" => Ok(Self::RecoveryResolved),
            other => Err(CliError::UnknownDrillFlag(format!(
                "--posture value `{other}`; expected clear, challenge-open, challenge-accepted, halted, recovery-required, recovery-in-review, halted-recovery-required, or recovery-resolved"
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::ChallengeOpen => "challenge_open",
            Self::ChallengeAccepted => "challenge_accepted",
            Self::Halted => "halted",
            Self::RecoveryRequired => "recovery_required",
            Self::RecoveryInReview => "recovery_in_review",
            Self::HaltedRecoveryRequired => "halted_recovery_required",
            Self::RecoveryResolved => "recovery_resolved",
        }
    }

    fn posture(self) -> AnchorOperationalPosture {
        match self {
            Self::Clear => AnchorOperationalPosture::clear(),
            Self::ChallengeOpen => AnchorOperationalPosture::new(
                ChallengePosture::Open,
                HaltPosture::Active,
                RecoveryPosture::NotRequired,
            ),
            Self::ChallengeAccepted => AnchorOperationalPosture::new(
                ChallengePosture::Accepted,
                HaltPosture::Active,
                RecoveryPosture::NotRequired,
            ),
            Self::Halted => AnchorOperationalPosture::halted(),
            Self::RecoveryRequired => AnchorOperationalPosture::recovery_required(),
            Self::RecoveryInReview => AnchorOperationalPosture::new(
                ChallengePosture::Clear,
                HaltPosture::Active,
                RecoveryPosture::InReview,
            ),
            Self::HaltedRecoveryRequired => AnchorOperationalPosture::halted_recovery_required(),
            Self::RecoveryResolved => AnchorOperationalPosture::recovery_resolved(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DrillArgs {
    stage: KillSwitchDrillStage,
    action: KillSwitchAction,
    posture: DrillPosture,
    authority: DrillAuthority,
    help: bool,
}

impl Default for DrillArgs {
    fn default() -> Self {
        Self {
            stage: KillSwitchDrillStage::BeforeProofAcceptance,
            action: KillSwitchAction::Halt,
            posture: DrillPosture::Clear,
            authority: DrillAuthority::Halt,
            help: false,
        }
    }
}

pub fn run_drill(args: &[String]) -> Result<String, CliError> {
    let args = parse_args(args)?;

    if args.help {
        return Ok(drill_help());
    }

    let request = KillSwitchDrillRequest::new(
        args.stage,
        args.action,
        args.posture.posture(),
        args.authority.key(),
    );

    let review = review_kill_switch_drill(&authority_map(), &request);

    let mut lines = vec![
        "rox-anchor kill-switch drill".to_string(),
        "command: drill".to_string(),
        format!("stage: {}", args.stage.as_str()),
        format!("action: {}", args.action.as_str()),
        format!("posture_fixture: {}", args.posture.as_str()),
        format!("presented_authority: {}", args.authority.as_str()),
        format!("review_accepted: {}", review.is_accepted()),
    ];

    for line in review.render_lines() {
        lines.push(line);
    }

    lines.extend([
        "runtime: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "rpc_submission: disabled".to_string(),
        "network_submitted: false".to_string(),
        "mint_burn_execution: disabled".to_string(),
        "roc_release: disabled".to_string(),
        "settlement_claim: none".to_string(),
        "public_bridge_authorization: none".to_string(),
    ]);

    Ok(lines.join("\n"))
}

fn parse_args(args: &[String]) -> Result<DrillArgs, CliError> {
    let mut parsed = DrillArgs::default();
    let mut index = 0;

    while index < args.len() {
        let arg = args[index].as_str();

        match arg {
            "--help" | "-h" => {
                parsed.help = true;
                index += 1;
            }
            "--stage" => {
                parsed.stage = parse_stage(required_value(args, index, "--stage")?)?;
                index += 2;
            }
            "--action" => {
                parsed.action = parse_action(required_value(args, index, "--action")?)?;
                index += 2;
            }
            "--posture" => {
                parsed.posture = DrillPosture::parse(required_value(args, index, "--posture")?)?;
                index += 2;
            }
            "--authority" => {
                parsed.authority =
                    DrillAuthority::parse(required_value(args, index, "--authority")?)?;
                index += 2;
            }
            _ => {
                if let Some(value) = arg.strip_prefix("--stage=") {
                    parsed.stage = parse_stage(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--action=") {
                    parsed.action = parse_action(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--posture=") {
                    parsed.posture = DrillPosture::parse(value)?;
                    index += 1;
                } else if let Some(value) = arg.strip_prefix("--authority=") {
                    parsed.authority = DrillAuthority::parse(value)?;
                    index += 1;
                } else {
                    return Err(CliError::UnknownDrillFlag(arg.to_owned()));
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
        .ok_or_else(|| CliError::UnknownDrillFlag(format!("{flag} requires a value")))
}

fn parse_stage(value: &str) -> Result<KillSwitchDrillStage, CliError> {
    match value {
        "before-proof" | "before-proof-acceptance" | "before_proof_acceptance" => {
            Ok(KillSwitchDrillStage::BeforeProofAcceptance)
        }
        "after-proof"
        | "after-proof-before-simulation"
        | "after_proof_acceptance_before_simulation" => {
            Ok(KillSwitchDrillStage::AfterProofAcceptanceBeforeSimulation)
        }
        "after-simulation"
        | "after-simulation-before-submission"
        | "after_simulation_before_submission" => {
            Ok(KillSwitchDrillStage::AfterSimulationBeforeSubmission)
        }
        "after-submit" | "after-capped-submit" | "after_capped_testnet_submission" => {
            Ok(KillSwitchDrillStage::AfterCappedTestnetSubmission)
        }
        other => Err(CliError::UnknownDrillFlag(format!(
            "--stage value `{other}`; expected before-proof, after-proof, after-simulation, or after-submit"
        ))),
    }
}

fn parse_action(value: &str) -> Result<KillSwitchAction, CliError> {
    match value {
        "halt" => Ok(KillSwitchAction::Halt),
        "recover" | "recovery" => Ok(KillSwitchAction::Recover),
        other => Err(CliError::UnknownDrillFlag(format!(
            "--action value `{other}`; expected halt or recover"
        ))),
    }
}

fn authority_map() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(
                OperatorRole::UpgradeAuthority,
                key("upgrade-authority-phase12-cli-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::MintAuthority,
                key("mint-authority-phase12-cli-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::HaltAuthority,
                key("halt-authority-phase12-cli-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-authority-phase12-cli-key"),
            ),
        ],
    )
}

fn key(value: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(value).expect("static CLI authority id should validate")
}

fn drill_help() -> String {
    [
        "rox-anchor kill-switch drill",
        "",
        "usage:",
        "  rox-anchor drill [--stage <name>] [--action <halt|recover>] [--posture <name>] [--authority <name>]",
        "",
        "stages:",
        "  before-proof",
        "  after-proof",
        "  after-simulation",
        "  after-submit",
        "",
        "actions:",
        "  halt",
        "  recover",
        "",
        "postures:",
        "  clear",
        "  challenge-open",
        "  challenge-accepted",
        "  halted",
        "  recovery-required",
        "  recovery-in-review",
        "  halted-recovery-required",
        "  recovery-resolved",
        "",
        "authorities:",
        "  halt",
        "  recovery",
        "  upgrade",
        "  wrong",
        "",
        "security:",
        "  local report only",
        "  no RPC submission",
        "  no wallet/key loading",
        "  no mint/burn execution",
        "  no ROC release",
        "  no settlement or finality claim",
    ]
    .join("\n")
}
