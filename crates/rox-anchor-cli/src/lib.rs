// RO:WHAT — Local command dispatch for the ROX Anchor CLI.
// RO:WHY — Routes terminal commands into real core/proof review behavior.
// RO:INTERACTS — commands::{check, proof, status, halt, recover}, rox-anchor-proof, rox-anchor-core.
// RO:INVARIANTS — CLI reports local review only; it never claims settlement or submits transactions.
// RO:SECURITY — no live RPC, wallet calls, deployment, minting, burning, staking, liquidity, or settlement.
// RO:TEST — crate-local CLI smoke tests cover dispatch and deterministic check output.

pub mod commands;

use std::{error::Error, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliError {
    UnknownCommand(String),
    UnknownCheckFixture(String),
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
        "status" => Ok(commands::status::status_report()),
        "halt" => Ok(commands::halt::halt_report()),
        "recover" | "recovery" => Ok(commands::recover::recovery_report()),
        "--help" | "-h" | "help" => Ok(help_text()),
        other => Err(CliError::UnknownCommand(other.to_owned())),
    }
}

pub fn help_text() -> String {
    [
        "rox-anchor local inspection CLI",
        "",
        "commands:",
        "  check [--fixture <name>]   review a deterministic local proof fixture",
        "  proof                      show proof-review command notes",
        "  status                     show display-safe status labels",
        "  halt                       show halt posture notes",
        "  recover                    show recovery posture notes",
        "",
        "fixtures:",
        "  valid | mismatch | replay | missing-evidence | disputed | challenge | halt | recovery",
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
    fn unknown_command_is_error() {
        let error = run_from_args(["rox-anchor", "submit"]).unwrap_err();

        assert_eq!(error, CliError::UnknownCommand("submit".to_string()));
    }
}
