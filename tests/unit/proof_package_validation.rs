// RO:WHAT — Local proof package validation unit tests for rox-anchor-proof.
// RO:WHY — Exercises dependency-free local review behavior without runtime, RPC, wallet, or settlement authority.
// RO:INTERACTS — crates/rox-anchor-proof local modules through path-based test imports.
// RO:INVARIANTS — Proof packages are evidence only; local validation is not finality.
// RO:SECURITY — No network, no wallet, no Solana/Anchor runtime, no bridge runtime, no value movement.
// RO:TEST — Future targeted local rustc/cargo test gate only when explicitly authorized.

#![forbid(unsafe_code)]

#[path = "../../crates/rox-anchor-proof/src/challenge.rs"]
mod challenge;
#[path = "../../crates/rox-anchor-proof/src/package.rs"]
mod package;
#[path = "../../crates/rox-anchor-proof/src/quorum.rs"]
mod quorum;
#[path = "../../crates/rox-anchor-proof/src/recovery.rs"]
mod recovery;
#[path = "../../crates/rox-anchor-proof/src/replay.rs"]
mod replay;
#[path = "../../crates/rox-anchor-proof/src/validate.rs"]
mod validate;

use challenge::ChallengeGatePosture;
use package::{CommitmentReviewLevel, EvidencePosture, ProofDirection, ProofPackageShape};
use quorum::QuorumObservationPosture;
use recovery::{HaltPosture, RecoveryPosture};
use replay::ExpectedProofBinding;
use validate::{
    review_package_for_local_review_only, LocalProofReviewDecision, ProofReviewFinding,
};

fn valid_package() -> ProofPackageShape {
    ProofPackageShape {
        schema_version: "rox-anchor-proof-package-fixture-v1".to_string(),
        source_domain: "internal-roc-local-fixture".to_string(),
        target_domain: "rox-anchor-local-fixture".to_string(),
        direction: ProofDirection::RocToRox,
        operation_id: "op_fixture_valid_0001".to_string(),
        idempotency_key: "idem_fixture_valid_0001".to_string(),
        nonce: "nonce_fixture_valid_0001".to_string(),
        cluster: "local-fixture-cluster".to_string(),
        program_id: "local-fixture-program".to_string(),
        mint: "local-fixture-mint".to_string(),
        token_account: "local-fixture-token-account".to_string(),
        commitment_level: CommitmentReviewLevel::ReviewOnly,
        evidence_posture: EvidencePosture::ConsistentForLocalReviewOnly,
        quorum_posture: QuorumObservationPosture::EvidencePresent,
        challenge_status: ChallengeGatePosture::Closed,
        halt_status: HaltPosture::NotHalted,
        recovery_status: RecoveryPosture::NotRequired,
    }
}

#[test]
fn valid_package_is_local_review_only() {
    let package = valid_package();
    let expected = ExpectedProofBinding::from_package_identity(&package);

    let review = review_package_for_local_review_only(&package, &expected);

    assert_eq!(
        review.decision,
        LocalProofReviewDecision::ValidForLocalReviewOnly
    );
    assert!(!review.is_runtime_authorized());
    assert!(!review.is_finality_claim());
    assert!(!review.is_settlement_claim());
}

#[test]
fn missing_token_account_is_evidence_incomplete() {
    let mut package = valid_package();
    package.token_account.clear();
    let expected = ExpectedProofBinding::from_package_identity(&package);

    let review = review_package_for_local_review_only(&package, &expected);

    assert_eq!(review.decision, LocalProofReviewDecision::EvidenceIncomplete);
    assert!(review.has_finding(ProofReviewFinding::MissingTokenAccount));
}

#[test]
fn disputed_quorum_does_not_become_finality() {
    let mut package = valid_package();
    package.quorum_posture = QuorumObservationPosture::Disputed;
    let expected = ExpectedProofBinding::from_package_identity(&package);

    let review = review_package_for_local_review_only(&package, &expected);

    assert_eq!(review.decision, LocalProofReviewDecision::EvidenceIncomplete);
    assert!(review.has_finding(ProofReviewFinding::QuorumDisputed));
    assert!(!review.is_runtime_authorized());
    assert!(!review.is_finality_claim());
}

// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local Phase 4 unit-test source does not authorize runtime.

#[cfg(test)]
mod composite_context_review_tests {
    use super::*;

    #[test]
    fn composite_context_accepts_fully_consistent_local_review() {
        let package = valid_package();
        let expected = ExpectedProofBinding::from_package_identity(&package);
        let inputs = validate::LocalProofEvidenceInputs::new(
            challenge::ChallengeWindowTiming::unopened(100),
            quorum::QuorumEvidenceCount::new(3, 0, 0, 2),
            recovery::RecoveryCaseKind::NotRequired,
        );

        let review = validate::review_composite_local_proof_for_local_review_only(
            &package,
            &expected,
            &[],
            inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::ValidForLocalReviewOnly
        );
        assert!(review.has_finding(
            validate::CompositeLocalProofReviewFinding::PackageReviewAccepted
        ));
        assert!(review.has_finding(
            validate::CompositeLocalProofReviewFinding::QuorumEvidencePresent
        ));
        assert!(review.has_finding(
            validate::CompositeLocalProofReviewFinding::HaltRecoveryClear
        ));
        assert!(!review.is_runtime_authorized());
        assert!(!review.calls_rpc());
        assert!(!review.calls_wallet());
        assert!(!review.is_finality_claim());
        assert!(!review.is_settlement_claim());
    }

    #[test]
    fn composite_context_keeps_disputed_quorum_incomplete() {
        let package = valid_package();
        let expected = ExpectedProofBinding::from_package_identity(&package);
        let inputs = validate::LocalProofEvidenceInputs::new(
            challenge::ChallengeWindowTiming::unopened(100),
            quorum::QuorumEvidenceCount::new(2, 1, 0, 2),
            recovery::RecoveryCaseKind::NotRequired,
        );

        let review = validate::review_composite_local_proof_for_local_review_only(
            &package,
            &expected,
            &[],
            inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::EvidenceIncomplete
        );
        assert!(review.has_finding(validate::CompositeLocalProofReviewFinding::QuorumDisputed));
        assert!(!review.is_finality_claim());
    }

    #[test]
    fn composite_context_rejects_accepted_challenge() {
        let mut package = valid_package();
        package.challenge_status = challenge::ChallengeGatePosture::Accepted;
        let expected = ExpectedProofBinding::from_package_identity(&package);
        let inputs = validate::LocalProofEvidenceInputs::new(
            challenge::ChallengeWindowTiming::opened(10, 15, 5, 20),
            quorum::QuorumEvidenceCount::new(3, 0, 0, 2),
            recovery::RecoveryCaseKind::ChallengeAccepted,
        );

        let review = validate::review_composite_local_proof_for_local_review_only(
            &package,
            &expected,
            &[],
            inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::ReviewRejected
        );
        assert!(review.has_finding(validate::CompositeLocalProofReviewFinding::ChallengeRejected));
        assert!(review.has_finding(validate::CompositeLocalProofReviewFinding::PackageRejected));
        assert!(!review.is_settlement_claim());
    }
}

#[path = "../../crates/rox-anchor-proof/src/fixtures.rs"]
mod fixtures;

#[cfg(test)]
mod fixture_helper_tests {
    use super::*;

    #[test]
    fn fixture_helpers_build_valid_composite_review() {
        let fixture = fixtures::valid_fixture_for_local_review_only();

        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::ValidForLocalReviewOnly
        );
        assert_eq!(fixture.case.case_id(), "proof-package.valid");
        assert!(!fixture.authorizes_runtime());
        assert!(!fixture.calls_rpc());
        assert!(!fixture.calls_wallet());
        assert!(!fixture.is_finality_claim());
        assert!(!fixture.is_settlement_claim());
    }

    #[test]
    fn fixture_helpers_build_replay_rejected_review() {
        let fixture = fixtures::replay_rejected_fixture_for_local_review_only();

        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::ReviewRejected
        );
        assert!(review.has_finding(validate::CompositeLocalProofReviewFinding::PackageRejected));
        assert_eq!(fixture.case.case_id(), "proof-package.replay-rejected");
    }

    #[test]
    fn fixture_helpers_build_cluster_mismatch_review() {
        let fixture = fixtures::cluster_mismatch_fixture_for_local_review_only();

        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::ReviewRejected
        );
        assert!(review.has_finding(validate::CompositeLocalProofReviewFinding::PackageRejected));
        assert_eq!(fixture.case.case_id(), "proof-package.cluster-mismatch");
    }

    #[test]
    fn fixture_helpers_build_recovery_required_review() {
        let fixture = fixtures::recovery_required_fixture_for_local_review_only();

        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );

        assert_eq!(
            review.decision,
            validate::CompositeLocalProofReviewDecision::EvidenceIncomplete
        );
        assert!(review.has_finding(validate::CompositeLocalProofReviewFinding::HaltRecoveryRequired));
        assert_eq!(fixture.case.case_id(), "recovery.case.valid");
    }

    #[test]
    fn fixture_helper_inventory_is_local_only() {
        assert_eq!(
            fixtures::all_local_fixture_cases_for_local_review_only().len(),
            7
        );
        assert!(!fixtures::fixture_helpers_authorize_runtime());
        assert!(!fixtures::fixture_helpers_read_files());
        assert!(!fixtures::fixture_helpers_parse_json());
        assert!(!fixtures::fixture_helpers_call_rpc());
        assert!(!fixtures::fixture_helpers_call_wallet());
        assert!(!fixtures::fixture_helpers_are_finality());
        assert!(!fixtures::fixture_helpers_are_settlement());
    }
}

#[cfg(test)]
mod fixture_corpus_runner_tests {
    use super::*;

    #[test]
    fn fixture_corpus_runner_reviews_all_cases() {
        let entries = fixtures::review_all_fixture_cases_for_local_review_only();

        assert_eq!(
            entries.len(),
            fixtures::all_local_fixture_cases_for_local_review_only().len()
        );
        assert_eq!(entries.len(), 7);
        assert!(entries.iter().all(|entry| entry.matched_expectation));
        assert!(entries.iter().all(|entry| !entry.authorizes_runtime()));
        assert!(entries.iter().all(|entry| !entry.calls_rpc()));
        assert!(entries.iter().all(|entry| !entry.calls_wallet()));
        assert!(entries.iter().all(|entry| !entry.is_finality_claim()));
        assert!(entries.iter().all(|entry| !entry.is_settlement_claim()));
    }

    #[test]
    fn fixture_corpus_summary_is_failure_closed_and_local_only() {
        let summary = fixtures::review_fixture_corpus_for_local_review_only();

        assert_eq!(summary.total_cases, 7);
        assert_eq!(summary.valid_for_local_review_only, 1);
        assert_eq!(summary.evidence_incomplete, 2);
        assert_eq!(summary.review_rejected, 4);
        assert_eq!(summary.runtime_not_authorized, 0);
        assert_eq!(summary.matched_expectations, 7);
        assert!(summary.all_matched_expectations);
        assert!(summary.all_runtime_not_authorized);
        assert!(summary.all_not_finality);
        assert!(summary.all_not_settlement);
        assert!(!summary.authorizes_runtime());
        assert!(!summary.calls_rpc());
        assert!(!summary.calls_wallet());
        assert!(!summary.is_finality_claim());
        assert!(!summary.is_settlement_claim());
    }

    #[test]
    fn fixture_case_expected_decisions_are_explicit() {
        use validate::CompositeLocalProofReviewDecision::{
            EvidenceIncomplete, ReviewRejected, ValidForLocalReviewOnly,
        };

        assert_eq!(
            fixtures::expected_review_for_fixture_case_for_local_review_only(
                fixtures::LocalProofFixtureCase::Valid
            )
            .expected_decision,
            ValidForLocalReviewOnly
        );

        assert_eq!(
            fixtures::expected_review_for_fixture_case_for_local_review_only(
                fixtures::LocalProofFixtureCase::ReplayRejected
            )
            .expected_decision,
            ReviewRejected
        );

        assert_eq!(
            fixtures::expected_review_for_fixture_case_for_local_review_only(
                fixtures::LocalProofFixtureCase::QuorumDisputed
            )
            .expected_decision,
            EvidenceIncomplete
        );
    }

    #[test]
    fn fixture_corpus_runner_never_claims_authority() {
        assert!(!fixtures::fixture_corpus_runner_authorizes_runtime());
        assert!(!fixtures::fixture_corpus_runner_reads_files());
        assert!(!fixtures::fixture_corpus_runner_parses_json());
        assert!(!fixtures::fixture_corpus_runner_calls_rpc());
        assert!(!fixtures::fixture_corpus_runner_calls_wallet());
        assert!(!fixtures::fixture_corpus_runner_is_finality());
        assert!(!fixtures::fixture_corpus_runner_is_settlement());
    }
}

#[cfg(test)]
mod local_review_report_tests {
    use super::*;

    #[test]
    fn local_review_report_labels_are_stable() {
        let fixture = fixtures::valid_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let report = validate::report_for_composite_local_proof_review(&review);

        assert_eq!(report.decision_label, "ValidForLocalReviewOnly");
        assert_eq!(report.severity_label, "ValidForLocalReviewOnly");
        assert!(report.composite_finding_count >= 3);
        assert!(report.is_clean_local_review_only());
        assert!(!report.authorizes_runtime());
        assert!(!report.calls_rpc());
        assert!(!report.calls_wallet());
        assert!(!report.is_finality_claim());
        assert!(!report.is_settlement_claim());
    }

    #[test]
    fn local_review_report_marks_rejected_fixture() {
        let fixture = fixtures::replay_rejected_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let report = validate::report_for_composite_local_proof_review(&review);

        assert_eq!(report.decision_label, "ReviewRejected");
        assert_eq!(report.severity_label, "ReviewRejected");
        assert_eq!(
            report.severity,
            validate::LocalReviewReportSeverity::ReviewRejected
        );
        assert!(report.is_clean_local_review_only());
    }

    #[test]
    fn fixture_corpus_report_summarizes_all_local_cases() {
        let report = fixtures::report_fixture_corpus_for_local_review_only();

        assert_eq!(report.summary.total_cases, 7);
        assert_eq!(report.entry_reports.len(), 7);
        assert!(report.summary.all_matched_expectations);
        assert!(report.all_reports_clean);
        assert!(report.all_runtime_not_authorized);
        assert!(report.all_not_finality);
        assert!(report.all_not_settlement);
        assert!(!report.authorizes_runtime());
        assert!(!report.calls_rpc());
        assert!(!report.calls_wallet());
        assert!(!report.is_finality_claim());
        assert!(!report.is_settlement_claim());
    }

    #[test]
    fn local_review_report_helpers_never_claim_authority() {
        assert!(!validate::local_review_report_authorizes_runtime());
        assert!(!validate::local_review_report_reads_files());
        assert!(!validate::local_review_report_parses_json());
        assert!(!validate::local_review_report_calls_rpc());
        assert!(!validate::local_review_report_calls_wallet());
        assert!(!validate::local_review_report_is_finality());
        assert!(!validate::local_review_report_is_settlement());

        assert!(!fixtures::fixture_corpus_report_authorizes_runtime());
        assert!(!fixtures::fixture_corpus_report_reads_files());
        assert!(!fixtures::fixture_corpus_report_parses_json());
        assert!(!fixtures::fixture_corpus_report_calls_rpc());
        assert!(!fixtures::fixture_corpus_report_calls_wallet());
        assert!(!fixtures::fixture_corpus_report_is_finality());
        assert!(!fixtures::fixture_corpus_report_is_settlement());
    }
}

#[cfg(test)]
mod local_trace_status_tests {
    use super::*;

    #[test]
    fn local_trace_contains_ordered_review_steps() {
        let fixture = fixtures::valid_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let trace = validate::trace_for_composite_local_proof_review(&review);

        assert_eq!(trace.step_count(), 5);
        assert_eq!(trace.decision_label, "ValidForLocalReviewOnly");
        assert!(trace.has_step_kind(validate::LocalReviewTraceStepKind::PackageReview));
        assert!(trace.has_step_kind(validate::LocalReviewTraceStepKind::ChallengeWindowReview));
        assert!(trace.has_step_kind(validate::LocalReviewTraceStepKind::QuorumEvidenceReview));
        assert!(trace.has_step_kind(validate::LocalReviewTraceStepKind::HaltRecoveryReview));
        assert!(trace.has_step_kind(validate::LocalReviewTraceStepKind::AuthorityPostureReview));
        assert!(trace.is_clean_local_review_only());
        assert!(!trace.authorizes_runtime());
        assert!(!trace.calls_rpc());
        assert!(!trace.calls_wallet());
        assert!(!trace.is_finality_claim());
        assert!(!trace.is_settlement_claim());
    }

    #[test]
    fn local_status_projection_maps_quorum_dispute() {
        let fixture = fixtures::quorum_disputed_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let projection = validate::status_projection_for_composite_local_proof_review(&review);

        assert_eq!(
            projection.primary,
            validate::LocalReviewStatusLabel::QuorumDisputed
        );
        assert_eq!(projection.primary_label, "QuorumDisputed");
        assert_eq!(projection.detail_label, "QuorumDisputed");
        assert_eq!(projection.decision_label, "EvidenceIncomplete");
        assert!(projection.is_clean_local_review_only());
        assert!(!projection.is_display_authority());
    }

    #[test]
    fn local_status_projection_maps_recovery_required() {
        let fixture = fixtures::recovery_required_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let projection = validate::status_projection_for_composite_local_proof_review(&review);

        assert_eq!(
            projection.primary,
            validate::LocalReviewStatusLabel::RecoveryReviewRequired
        );
        assert_eq!(projection.primary_label, "RecoveryReviewRequired");
        assert_eq!(projection.detail_label, "HaltRecoveryRequired");
        assert_eq!(projection.decision_label, "EvidenceIncomplete");
        assert!(projection.stale_safe);
        assert!(projection.local_review_only);
    }

    #[test]
    fn local_trace_status_helpers_never_claim_authority() {
        assert!(!validate::local_review_trace_authorizes_runtime());
        assert!(!validate::local_review_trace_reads_files());
        assert!(!validate::local_review_trace_parses_json());
        assert!(!validate::local_review_trace_calls_rpc());
        assert!(!validate::local_review_trace_calls_wallet());
        assert!(!validate::local_review_trace_is_finality());
        assert!(!validate::local_review_trace_is_settlement());

        assert!(!validate::local_review_status_projection_authorizes_runtime());
        assert!(!validate::local_review_status_projection_calls_rpc());
        assert!(!validate::local_review_status_projection_calls_wallet());
        assert!(!validate::local_review_status_projection_is_finality());
        assert!(!validate::local_review_status_projection_is_settlement());
        assert!(!validate::local_review_status_projection_is_display_authority());
    }
}

#[cfg(test)]
mod local_decision_gate_guard_tests {
    use super::*;

    #[test]
    fn local_decision_gate_accepts_valid_review_only() {
        let fixture = fixtures::valid_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let gate = validate::review_local_decision_gate_for_local_review_only(&review);

        assert_eq!(
            gate.posture,
            validate::LocalDecisionGatePosture::AcceptLocalReviewOnly
        );
        assert_eq!(gate.posture_label, "AcceptLocalReviewOnly");
        assert_eq!(gate.composite_decision_label, "ValidForLocalReviewOnly");
        assert_eq!(gate.status_label, "ValidForLocalReviewOnly");
        assert!(gate.passes_local_acceptance());
        assert!(gate.has_finding(validate::LocalDecisionGateFinding::CompositeReviewAccepted));
        assert!(gate.has_finding(validate::LocalDecisionGateFinding::LocalReviewOnly));
        assert!(!gate.authorizes_runtime());
        assert!(!gate.calls_rpc());
        assert!(!gate.calls_wallet());
        assert!(!gate.is_finality_claim());
        assert!(!gate.is_settlement_claim());
        assert!(!gate.is_display_authority());
    }

    #[test]
    fn local_decision_gate_blocks_incomplete_review() {
        let fixture = fixtures::quorum_disputed_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let gate = validate::review_local_decision_gate_for_local_review_only(&review);

        assert_eq!(
            gate.posture,
            validate::LocalDecisionGatePosture::EvidenceIncomplete
        );
        assert_eq!(gate.status_label, "QuorumDisputed");
        assert!(!gate.passes_local_acceptance());
        assert!(gate.has_finding(
            validate::LocalDecisionGateFinding::CompositeReviewEvidenceIncomplete
        ));
        assert!(gate.is_clean_local_review_only());
    }

    #[test]
    fn local_decision_gate_blocks_rejected_review() {
        let fixture = fixtures::replay_rejected_fixture_for_local_review_only();
        let review = validate::review_composite_local_proof_for_local_review_only(
            &fixture.package,
            &fixture.expected,
            &fixture.previously_seen_nonces,
            fixture.inputs,
        );
        let gate = validate::review_local_decision_gate_for_local_review_only(&review);

        assert_eq!(
            gate.posture,
            validate::LocalDecisionGatePosture::ReviewRejected
        );
        assert_eq!(gate.posture_label, "ReviewRejected");
        assert!(!gate.passes_local_acceptance());
        assert!(gate.has_finding(validate::LocalDecisionGateFinding::CompositeReviewRejected));
        assert!(gate.has_finding(validate::LocalDecisionGateFinding::AuthorityPostureClean));
    }

    #[test]
    fn local_decision_gate_guard_helpers_never_claim_authority() {
        assert!(!validate::local_decision_gate_guard_authorizes_runtime());
        assert!(!validate::local_decision_gate_guard_reads_files());
        assert!(!validate::local_decision_gate_guard_parses_json());
        assert!(!validate::local_decision_gate_guard_calls_rpc());
        assert!(!validate::local_decision_gate_guard_calls_wallet());
        assert!(!validate::local_decision_gate_guard_is_finality());
        assert!(!validate::local_decision_gate_guard_is_settlement());
        assert!(!validate::local_decision_gate_guard_is_display_authority());
    }
}

#[cfg(test)]
mod fixture_gate_runner_tests {
    use super::*;

    #[test]
    fn fixture_gate_runner_evaluates_all_cases() {
        let evaluations = fixtures::evaluate_all_fixture_decision_gates_for_local_review_only();

        assert_eq!(evaluations.len(), 7);
        assert!(evaluations.iter().all(|evaluation| {
            evaluation.matched_composite_expectation
                && evaluation.clean_local_review_only
                && !evaluation.authorizes_runtime()
                && !evaluation.calls_rpc()
                && !evaluation.calls_wallet()
                && !evaluation.is_finality_claim()
                && !evaluation.is_settlement_claim()
                && !evaluation.is_display_authority()
        }));
    }

    #[test]
    fn fixture_gate_runner_summary_is_failure_closed() {
        let summary = fixtures::evaluate_fixture_decision_gate_corpus_for_local_review_only();

        assert_eq!(summary.total_cases, 7);
        assert_eq!(summary.matched_composite_expectations, 7);
        assert_eq!(summary.accepted_for_local_review_only, 1);
        assert_eq!(summary.evidence_incomplete, 2);
        assert_eq!(summary.review_rejected, 4);
        assert_eq!(summary.runtime_not_authorized, 0);
        assert!(summary.all_matched_composite_expectations);
        assert!(summary.all_clean_local_review_only);
        assert!(summary.all_runtime_not_authorized);
        assert!(summary.all_not_finality);
        assert!(summary.all_not_settlement);
        assert!(summary.all_not_display_authority);
        assert!(!summary.authorizes_runtime());
        assert!(!summary.calls_rpc());
        assert!(!summary.calls_wallet());
        assert!(!summary.is_finality_claim());
        assert!(!summary.is_settlement_claim());
        assert!(!summary.is_display_authority());
    }

    #[test]
    fn fixture_gate_runner_valid_case_passes_local_acceptance_only() {
        let evaluation = fixtures::evaluate_fixture_decision_gate_for_local_review_only(
            fixtures::LocalProofFixtureCase::Valid,
        );

        assert_eq!(
            evaluation.gate_posture,
            validate::LocalDecisionGatePosture::AcceptLocalReviewOnly
        );
        assert_eq!(evaluation.gate_posture_label, "AcceptLocalReviewOnly");
        assert_eq!(evaluation.status_label, "ValidForLocalReviewOnly");
        assert!(evaluation.accepted_for_local_review_only);
        assert!(evaluation.gate.passes_local_acceptance());
        assert!(!evaluation.gate.is_finality_claim());
        assert!(!evaluation.gate.is_settlement_claim());
    }

    #[test]
    fn fixture_gate_runner_helpers_never_claim_authority() {
        assert!(!fixtures::fixture_gate_runner_authorizes_runtime());
        assert!(!fixtures::fixture_gate_runner_reads_files());
        assert!(!fixtures::fixture_gate_runner_parses_json());
        assert!(!fixtures::fixture_gate_runner_calls_rpc());
        assert!(!fixtures::fixture_gate_runner_calls_wallet());
        assert!(!fixtures::fixture_gate_runner_is_finality());
        assert!(!fixtures::fixture_gate_runner_is_settlement());
        assert!(!fixtures::fixture_gate_runner_is_display_authority());
    }
}
