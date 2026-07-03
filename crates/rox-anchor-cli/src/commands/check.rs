//! RO:WHAT — Implements `rox-anchor check` using deterministic in-code proof fixtures.
//! RO:WHY — Makes the CLI prove it calls real rox-anchor-proof behavior before JSON input is added.
//! RO:INTERACTS — rox-anchor-proof fixtures/review, rox-anchor-core status labels.
//! RO:INVARIANTS — prints deterministic local review findings; never prints fake finality or settlement.
//! RO:SECURITY — local-only; no live RPC, wallet, deployment, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — unit and CLI smoke tests cover valid, replay, and challenge outputs.

use rox_anchor_core::{
    label_for_lifecycle_state, AnchorDirection, ChallengePosture, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture, TokenAccountId,
};
use rox_anchor_proof::{
    fixtures, review_proof_package, EvidenceBundle, ExpectedProofBinding, ProofFindingCode,
    ProofFindingSeverity, ProofPackage, ProofReview, ReplaySet, ReviewDecision,
};

use crate::CliError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CheckFixture {
    Valid,
    Mismatch,
    Replay,
    MissingEvidence,
    Disputed,
    Challenge,
    Halt,
    Recovery,
}

impl CheckFixture {
    fn parse(value: &str) -> Result<Self, CliError> {
        match value {
            "valid" => Ok(Self::Valid),
            "mismatch" => Ok(Self::Mismatch),
            "replay" => Ok(Self::Replay),
            "missing-evidence" => Ok(Self::MissingEvidence),
            "disputed" => Ok(Self::Disputed),
            "challenge" => Ok(Self::Challenge),
            "halt" => Ok(Self::Halt),
            "recovery" => Ok(Self::Recovery),
            other => Err(CliError::UnknownCheckFixture(other.to_owned())),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Mismatch => "mismatch",
            Self::Replay => "replay",
            Self::MissingEvidence => "missing-evidence",
            Self::Disputed => "disputed",
            Self::Challenge => "challenge",
            Self::Halt => "halt",
            Self::Recovery => "recovery",
        }
    }
}

pub fn run_check(args: &[String]) -> Result<String, CliError> {
    let fixture = parse_fixture_arg(args)?;
    let expected = fixtures::expected_proof_binding();
    let mut package = fixtures::valid_package();
    let mut replay = ReplaySet::default();

    match fixture {
        CheckFixture::Valid => {}
        CheckFixture::Mismatch => apply_mismatch_fixture(&mut package),
        CheckFixture::Replay => replay = ReplaySet::from_package(&package),
        CheckFixture::MissingEvidence => package.evidence = EvidenceBundle::new(0, 2, 0),
        CheckFixture::Disputed => package.evidence = EvidenceBundle::new(2, 2, 1),
        CheckFixture::Challenge => package.challenge_posture = ChallengePosture::Open,
        CheckFixture::Halt => package.halt_posture = HaltPosture::Halted,
        CheckFixture::Recovery => package.recovery_posture = RecoveryPosture::Required,
    }

    let review = review_proof_package(&package, &expected, &replay);
    Ok(render_check_report(fixture, &review, &expected, &package))
}

fn parse_fixture_arg(args: &[String]) -> Result<CheckFixture, CliError> {
    let mut fixture = CheckFixture::Valid;
    let mut index = 0;

    while index < args.len() {
        let arg = args[index].as_str();

        if arg == "--fixture" {
            let Some(value) = args.get(index + 1) else {
                return Err(CliError::UnknownCheckFixture(String::new()));
            };
            fixture = CheckFixture::parse(value)?;
            index += 2;
            continue;
        }

        if let Some(value) = arg.strip_prefix("--fixture=") {
            fixture = CheckFixture::parse(value)?;
            index += 1;
            continue;
        }

        return Err(CliError::UnknownCheckFixture(arg.to_owned()));
    }

    Ok(fixture)
}

fn apply_mismatch_fixture(package: &mut ProofPackage) {
    package.binding.source_domain = DomainId::new("wrong-source").unwrap();
    package.binding.target_domain = DomainId::new("wrong-target").unwrap();
    package.binding.direction = AnchorDirection::RoxToRoc;
    package.binding.cluster = ClusterId::new("wrong-cluster").unwrap();
    package.binding.program_id = ProgramId::new("WrongProgram111111111111111111111111").unwrap();
    package.binding.mint = MintId::new("WrongMint111111111111111111111111111111").unwrap();
    package.binding.token_account =
        TokenAccountId::new("WrongTokenAccount111111111111111111").unwrap();
    package.operation_id = OperationId::new("wrong-operation").unwrap();
    package.idempotency_key = IdempotencyKey::new("wrong-idempotency").unwrap();
    package.nonce = Nonce::new("wrong-nonce").unwrap();
}

fn render_check_report(
    fixture: CheckFixture,
    review: &ProofReview,
    expected: &ExpectedProofBinding,
    package: &ProofPackage,
) -> String {
    let mut lines = vec![
        "rox-anchor local proof review".to_string(),
        "command: check".to_string(),
        format!("fixture: {}", fixture.as_str()),
        format!("direction: {}", expected.binding.direction.as_str()),
        format!("source_domain: {}", expected.binding.source_domain),
        format!("target_domain: {}", expected.binding.target_domain),
        format!("cluster: {}", expected.binding.cluster),
        format!("program_id: {}", expected.binding.program_id),
        format!("mint: {}", expected.binding.mint),
        format!("token_account: {}", expected.binding.token_account),
        format!("operation_id: {}", package.operation_id),
        format!("idempotency_key: {}", package.idempotency_key),
        format!("nonce: {}", package.nonce),
        format!("decision: {}", decision_label(review)),
        format!(
            "status_label: {}",
            label_for_lifecycle_state(review.lifecycle_state)
        ),
        "findings:".to_string(),
    ];

    for finding in &review.findings {
        lines.push(format!(
            "  - {} [{}]",
            finding_code_label(finding.code),
            finding_severity_label(finding.severity)
        ));
    }

    lines.join("\n")
}

fn decision_label(review: &ProofReview) -> &'static str {
    match review.decision {
        ReviewDecision::Accepted => "ValidForLocalReview",
        ReviewDecision::Rejected => "Rejected",
        ReviewDecision::Blocked => blocked_decision_label(review),
    }
}

fn blocked_decision_label(review: &ProofReview) -> &'static str {
    match review.findings.first().map(|finding| finding.code) {
        Some(ProofFindingCode::EvidenceMissing | ProofFindingCode::QuorumDisputed) => {
            "EvidenceIncomplete"
        }
        Some(ProofFindingCode::ChallengeOpen | ProofFindingCode::ChallengeAccepted) => {
            "ChallengeBlocked"
        }
        Some(ProofFindingCode::HaltRequested | ProofFindingCode::Halted) => "HaltBlocked",
        Some(
            ProofFindingCode::RecoveryRequired
            | ProofFindingCode::RecoveryInReview
            | ProofFindingCode::RecoveryRejected,
        ) => "RecoveryBlocked",
        _ => "Blocked",
    }
}

fn finding_code_label(code: ProofFindingCode) -> &'static str {
    match code {
        ProofFindingCode::PackageAccepted => "PackageAccepted",
        ProofFindingCode::SourceDomainMismatch => "SourceDomainMismatch",
        ProofFindingCode::TargetDomainMismatch => "TargetDomainMismatch",
        ProofFindingCode::DirectionMismatch => "DirectionMismatch",
        ProofFindingCode::ClusterMismatch => "ClusterMismatch",
        ProofFindingCode::ProgramIdMismatch => "ProgramIdMismatch",
        ProofFindingCode::MintMismatch => "MintMismatch",
        ProofFindingCode::TokenAccountMismatch => "TokenAccountMismatch",
        ProofFindingCode::OperationIdMismatch => "OperationIdMismatch",
        ProofFindingCode::IdempotencyKeyMismatch => "IdempotencyKeyMismatch",
        ProofFindingCode::NonceMismatch => "NonceMismatch",
        ProofFindingCode::ReplayOperationId => "ReplayOperationId",
        ProofFindingCode::ReplayIdempotencyKey => "ReplayIdempotencyKey",
        ProofFindingCode::ReplayNonce => "ReplayNonce",
        ProofFindingCode::EvidenceMissing => "EvidenceMissing",
        ProofFindingCode::QuorumDisputed => "QuorumDisputed",
        ProofFindingCode::ChallengeOpen => "ChallengeOpen",
        ProofFindingCode::ChallengeAccepted => "ChallengeAccepted",
        ProofFindingCode::HaltRequested => "HaltRequested",
        ProofFindingCode::Halted => "Halted",
        ProofFindingCode::RecoveryRequired => "RecoveryRequired",
        ProofFindingCode::RecoveryInReview => "RecoveryInReview",
        ProofFindingCode::RecoveryRejected => "RecoveryRejected",
    }
}

fn finding_severity_label(severity: ProofFindingSeverity) -> &'static str {
    match severity {
        ProofFindingSeverity::Info => "Info",
        ProofFindingSeverity::Block => "Block",
        ProofFindingSeverity::Reject => "Reject",
    }
}
