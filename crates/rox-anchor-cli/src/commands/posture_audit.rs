//! RO:WHAT — CLI proof-posture audit report command.
//! RO:WHY — Phase 11 requires halt, challenge, and recovery posture to be visible in audit output.
//! RO:INTERACTS — rox-anchor-proof deterministic package review and rox-anchor-core posture enums.
//! RO:INVARIANTS — challenge, halt, and recovery blockers are reported from real proof review behavior.
//! RO:SECURITY — no live RPC, wallet/key loading, signing, transaction submission, minting, burning, ROC release, or settlement.
//! RO:TEST — covered by CLI posture audit report tests.

use rox_anchor_core::{ChallengePosture, HaltPosture, RecoveryPosture};
use rox_anchor_proof::{
    fixtures, review_proof_package, ProofFindingCode, ProofPackage, ProofReview, ReplaySet,
    ReviewDecision,
};

const AUDIT_RECORD_VERSION: &str = "proof-posture-audit-v1";

pub fn posture_audit_report() -> String {
    let cases = posture_cases();
    let accepted_cases = cases
        .iter()
        .filter(|case| is_accepted_decision(&case.review.decision))
        .count();
    let blocked_cases = cases.len().saturating_sub(accepted_cases);

    let mut lines = vec![
        "rox-anchor audit-posture".to_string(),
        "status: proof_posture_audit_report".to_string(),
        "submission: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "network_client: not_enabled".to_string(),
        "runtime_authority: not_enabled".to_string(),
        format!("audit_record={AUDIT_RECORD_VERSION}"),
        format!("case_count={}", cases.len()),
        format!("accepted_cases={accepted_cases}"),
        format!("blocked_cases={blocked_cases}"),
        "audit:".to_string(),
    ];

    for (index, case) in cases.iter().enumerate() {
        lines.extend(case.render(index).lines().map(|line| format!("  {line}")));
    }

    lines.extend([
        "display_safe=true".to_string(),
        "security: report-only; no RPC submission, wallet/key loading, mint/burn, ROC release, or settlement".to_string(),
        "next: inspect `rox-anchor audit` and `rox-anchor audit-relayer` for coordinator and relayer audit details".to_string(),
    ]);

    lines.join("\n")
}

#[derive(Clone, Debug)]
struct PostureAuditCase {
    name: &'static str,
    package: ProofPackage,
    review: ProofReview,
}

impl PostureAuditCase {
    fn new(name: &'static str, package: ProofPackage) -> Self {
        let expected = fixtures::expected_proof_binding();
        let review = review_proof_package(&package, &expected, &ReplaySet::default());

        Self {
            name,
            package,
            review,
        }
    }

    fn render(&self, index: usize) -> String {
        [
            format!("case.{index}.name={}", self.name),
            format!(
                "case.{index}.challenge_posture={:?}",
                self.package.challenge_posture
            ),
            format!("case.{index}.halt_posture={:?}", self.package.halt_posture),
            format!(
                "case.{index}.recovery_posture={:?}",
                self.package.recovery_posture
            ),
            format!("case.{index}.decision={:?}", self.review.decision),
            format!(
                "case.{index}.lifecycle_state={:?}",
                self.review.lifecycle_state
            ),
            format!(
                "case.{index}.findings={}",
                render_finding_codes(&self.review)
            ),
            format!(
                "case.{index}.permits_acceptance={}",
                is_accepted_decision(&self.review.decision)
            ),
        ]
        .join("\n")
    }
}

fn posture_cases() -> Vec<PostureAuditCase> {
    vec![
        PostureAuditCase::new("clear", fixtures::valid_package()),
        PostureAuditCase::new(
            "challenge_open",
            package_with_challenge(ChallengePosture::Open),
        ),
        PostureAuditCase::new(
            "challenge_accepted",
            package_with_challenge(ChallengePosture::Accepted),
        ),
        PostureAuditCase::new("halted", package_with_halt(HaltPosture::Halted)),
        PostureAuditCase::new(
            "recovery_required",
            package_with_recovery(RecoveryPosture::Required),
        ),
        PostureAuditCase::new(
            "recovery_in_review",
            package_with_recovery(RecoveryPosture::InReview),
        ),
    ]
}

fn package_with_challenge(posture: ChallengePosture) -> ProofPackage {
    let mut package = fixtures::valid_package();
    package.challenge_posture = posture;
    package
}

fn package_with_halt(posture: HaltPosture) -> ProofPackage {
    let mut package = fixtures::valid_package();
    package.halt_posture = posture;
    package
}

fn package_with_recovery(posture: RecoveryPosture) -> ProofPackage {
    let mut package = fixtures::valid_package();
    package.recovery_posture = posture;
    package
}

fn render_finding_codes(review: &ProofReview) -> String {
    let codes = review
        .findings
        .iter()
        .map(|finding| finding.code)
        .collect::<Vec<ProofFindingCode>>();

    if codes.is_empty() {
        return "none".to_string();
    }

    codes
        .iter()
        .map(|code| format!("{code:?}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn is_accepted_decision(decision: &ReviewDecision) -> bool {
    matches!(decision, ReviewDecision::Accepted)
}
