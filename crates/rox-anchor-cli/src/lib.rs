#![recursion_limit = "256"]
// RO:WHAT — Local command dispatch for the ROX Anchor CLI.
// RO:WHY — Routes terminal commands into real core/proof/relayer review behavior.
// RO:INTERACTS — commands::{check, proof, status, submit, drill, halt, recover}, rox-anchor-proof, rox-anchor-core.
// RO:INVARIANTS — default CLI remains report-only; explicit Phase 4 simulation never submits transactions or claims settlement.
// RO:SECURITY — no transaction submission, deployment, mint/burn execution, staking, liquidity, or settlement; Phase 4 simulation is explicit.
// RO:TEST — crate-local CLI smoke tests cover dispatch and deterministic check/submit output.

pub mod commands;

use std::{error::Error, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliError {
    UnknownCommand(String),
    UnknownCheckFixture(String),
    UnknownSubmitFlag(String),
    UnknownDrillFlag(String),
    UnknownPilotFlag(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCommand(command) => {
                write!(f, "unknown command `{command}`; try `rox-anchor check`")
            }
            Self::UnknownCheckFixture(fixture) => write!(
                f,
                "unknown check fixture `{fixture}`; expected valid, mismatch, replay, missing-evidence, disputed, challenge, halt, or recovery"
            ),
            Self::UnknownSubmitFlag(flag) => write!(
                f,
                "unknown submit-capped flag `{flag}`; try `rox-anchor submit-capped --help`"
            ),
            Self::UnknownDrillFlag(flag) => write!(
                f,
                "unknown drill flag `{flag}`; try `rox-anchor drill --help`"
            ),
            Self::UnknownPilotFlag(flag) => write!(
                f,
                "unknown pilot flag `{flag}`; try `rox-anchor pilot --help`"
            ),
        }
    }
}

impl Error for CliError {}

pub fn run_from_args<I, S>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let command = args.get(1).map(String::as_str).unwrap_or("check");
    let command_args: &[String] = if args.len() > 2 { &args[2..] } else { &[] };

    match command {
        "check" => commands::check::run_check(command_args),
        "proof" => Ok(commands::proof::proof_help()),
        "audit" => Ok(commands::audit::audit_report()),
        "audit-relayer" | "relayer-audit" => Ok(commands::audit::relayer_audit_report()),
        "audit-posture" | "posture-audit" => Ok(commands::posture_audit::posture_audit_report()),
        "status" => Ok(commands::status::status_report()),
        "pilot" => commands::pilot::run_pilot(command_args),
        "receipts" | "receipt-ledger" => Ok(commands::receipts::receipt_report()),
        "submit" => commands::submit::run_submit(command_args),
        "submit-capped" | "testnet-submit-capped" => {
            commands::submit::run_submit_capped(command_args)
        }
        "drill" | "kill-switch" | "kill-switch-drill" => commands::drill::run_drill(command_args),
        "halt" => Ok(commands::halt::halt_report()),
        "recover" | "recovery" => Ok(commands::recover::recovery_report()),
        "--help" | "-h" | "help" => Ok(help_text()),
        other => Err(CliError::UnknownCommand(other.to_owned())),
    }
}

pub fn help_text() -> String {
    [
        "rox-anchor",
        "",
        "Local ROX Anchor inspection and testnet-hardening reports.",
        "",
        "commands:",
        "  check [--fixture <name>]   review a deterministic local proof fixture",
        "  proof                      show read-only RPC proof audit report",
        "  audit                      show coordinator audit report",
        "  audit-relayer              show relayer simulation and capped submission audit report",
        "  audit-posture              show halt/challenge/recovery posture audit report",
        "  status                     show display-safe status labels",
        "  pilot                      private pilot command group",
        "  receipts                   show private pilot receipt ledger report",
        "  submit-capped              report capped testnet submit authorization",
        "  drill                      run local halt/recovery kill-switch drill",
        "  halt                       show halt posture notes",
        "  recover                    show recovery posture notes",
        "",
        "aliases:",
        "  relayer-audit              same as audit-relayer",
        "  posture-audit              same as audit-posture",
        "  receipt-ledger             same as receipts",
        "  kill-switch                same as drill",
        "  kill-switch-drill          same as drill",
        "",
        "security:",
        "  local/report-only by default",
        "  no silent RPC submission",
        "  no wallet/key loading by default; explicit Phase 4 simulation only",
        "  no mint/burn execution",
        "  no ROC release",
        "  no settlement or finality claim",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_command_runs_check_against_valid_fixture() {
        let output = run_from_args(["rox-anchor"]).unwrap();

        assert!(output.contains("command: check"));
        assert!(output.contains("fixture: valid"));
        assert!(output.contains("decision: ValidForLocalReview"));
        assert!(output.contains("status_label: Finality eligible"));
        assert!(output.contains("- PackageAccepted [Info]"));
    }

    #[test]
    fn check_replay_fixture_uses_real_replay_rejection() {
        let output = run_from_args(["rox-anchor", "check", "--fixture", "replay"]).unwrap();

        assert!(output.contains("fixture: replay"));
        assert!(output.contains("decision: Rejected"));
        assert!(output.contains("status_label: Failed"));
        assert!(output.contains("- ReplayOperationId [Reject]"));
        assert!(output.contains("- ReplayIdempotencyKey [Reject]"));
        assert!(output.contains("- ReplayNonce [Reject]"));
    }

    #[test]
    fn check_challenge_fixture_reports_blocked_status() {
        let output = run_from_args(["rox-anchor", "check", "--fixture=challenge"]).unwrap();

        assert!(output.contains("fixture: challenge"));
        assert!(output.contains("decision: ChallengeBlocked"));
        assert!(output.contains("status_label: Challenge open"));
        assert!(output.contains("- ChallengeOpen [Block]"));
    }

    #[test]
    fn submit_capped_report_defaults_to_missing_explicit_approval() {
        let output = run_from_args(["rox-anchor", "submit-capped"]).unwrap();

        assert!(output.contains("command: submit-capped"));
        assert!(output.contains("capped_submit_status: MissingExplicitOperatorApproval"));
        assert!(output.contains("authorized: false"));
        assert!(output.contains("live_submission_attempted: false"));
        assert!(output.contains("network_submitted: false"));
    }

    #[test]
    fn submit_capped_report_can_authorize_without_executing_network_submission() {
        let output = run_from_args([
            "rox-anchor",
            "submit-capped",
            "--authorize-testnet-submit-capped",
            "--receipt-persisted",
        ])
        .unwrap();

        assert!(output.contains("capped_submit_status: Authorized"));
        assert!(output.contains("authorized: true"));
        assert!(output.contains("live_submission_permitted: true"));
        assert!(output.contains("live_submission_attempted: false"));
        assert!(output.contains("network_submitted: false"));
        assert!(output.contains("wallet_key_loading: disabled"));
        assert!(output.contains("rpc_submission: disabled_in_cli_report"));
        assert!(output.contains("settlement_claim: none"));
    }

    #[test]
    fn unknown_command_is_error() {
        let error = run_from_args(["rox-anchor", "definitely-unknown"]).unwrap_err();

        assert_eq!(
            error,
            CliError::UnknownCommand("definitely-unknown".to_string())
        );
    }
}
