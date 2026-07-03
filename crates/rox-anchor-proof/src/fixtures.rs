// RO:WHAT — Local fixture helpers for rox-anchor-proof Phase 4 review code.
// RO:WHY — Provides deterministic package/evidence builders for local tests and future static vectors without duplicating setup.
// RO:INTERACTS — package, replay, challenge, quorum, recovery, and validate modules.
// RO:INVARIANTS — Fixtures are local review inputs only; fixture validity is not finality.
// RO:SECURITY — No file IO, no JSON parsing, no RPC, no wallet, no Solana/Anchor runtime, no bridge runtime, no value movement.
// RO:TEST — Static Phase 4 checker and local unit-test source only.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local fixture helper module does not authorize runtime.

use crate::challenge::{ChallengeGatePosture, ChallengeWindowTiming};
use crate::package::{
    CommitmentReviewLevel, EvidencePosture, ProofDirection, ProofPackageShape,
};
use crate::quorum::QuorumEvidenceCount;
use crate::recovery::{HaltPosture, RecoveryCaseKind, RecoveryPosture};
use crate::replay::ExpectedProofBinding;
use crate::validate::LocalProofEvidenceInputs;

/// Compile-time marker proving fixture helpers remain local review only.
pub const PHASE4_CODE_BATCH_D_FIXTURE_HELPERS_ONLY: bool = true;

pub const VALID_FIXTURE_SCHEMA_VERSION: &str = "rox-anchor-proof-package-fixture-v1";
pub const VALID_FIXTURE_SOURCE_DOMAIN: &str = "internal-roc-local-fixture";
pub const VALID_FIXTURE_TARGET_DOMAIN: &str = "rox-anchor-local-fixture";
pub const VALID_FIXTURE_OPERATION_ID: &str = "op_fixture_valid_0001";
pub const VALID_FIXTURE_IDEMPOTENCY_KEY: &str = "idem_fixture_valid_0001";
pub const VALID_FIXTURE_NONCE: &str = "nonce_fixture_valid_0001";
pub const VALID_FIXTURE_CLUSTER: &str = "local-fixture-cluster";
pub const VALID_FIXTURE_PROGRAM_ID: &str = "local-fixture-program";
pub const VALID_FIXTURE_MINT: &str = "local-fixture-mint";
pub const VALID_FIXTURE_TOKEN_ACCOUNT: &str = "local-fixture-token-account";

pub const REPLAYED_FIXTURE_NONCE: &str = VALID_FIXTURE_NONCE;
pub const CLUSTER_MISMATCH_VALUE: &str = "unexpected-local-fixture-cluster";
pub const MINT_MISMATCH_VALUE: &str = "unexpected-local-fixture-mint";

/// Local fixture case labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalProofFixtureCase {
    Valid,
    ReplayRejected,
    ClusterMismatch,
    MintMismatch,
    QuorumDisputed,
    ChallengeAccepted,
    RecoveryRequired,
}

impl LocalProofFixtureCase {
    pub fn case_id(self) -> &'static str {
        match self {
            LocalProofFixtureCase::Valid => "proof-package.valid",
            LocalProofFixtureCase::ReplayRejected => "proof-package.replay-rejected",
            LocalProofFixtureCase::ClusterMismatch => "proof-package.cluster-mismatch",
            LocalProofFixtureCase::MintMismatch => "proof-package.mint-mismatch",
            LocalProofFixtureCase::QuorumDisputed => "proof-package.rpc-disagreement",
            LocalProofFixtureCase::ChallengeAccepted => "challenge.accepted",
            LocalProofFixtureCase::RecoveryRequired => "recovery.case.valid",
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Local fixture bundle for composite proof review.
///
/// This is an in-memory fixture only. It does not read files, parse JSON, call
/// RPC, call wallets, authorize runtime, prove finality, or prove settlement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalProofFixture {
    pub case: LocalProofFixtureCase,
    pub package: ProofPackageShape,
    pub expected: ExpectedProofBinding,
    pub previously_seen_nonces: Vec<&'static str>,
    pub inputs: LocalProofEvidenceInputs,
}

impl LocalProofFixture {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub fn valid_proof_package_fixture_for_local_review_only() -> ProofPackageShape {
    ProofPackageShape {
        schema_version: VALID_FIXTURE_SCHEMA_VERSION.to_string(),
        source_domain: VALID_FIXTURE_SOURCE_DOMAIN.to_string(),
        target_domain: VALID_FIXTURE_TARGET_DOMAIN.to_string(),
        direction: ProofDirection::RocToRox,
        operation_id: VALID_FIXTURE_OPERATION_ID.to_string(),
        idempotency_key: VALID_FIXTURE_IDEMPOTENCY_KEY.to_string(),
        nonce: VALID_FIXTURE_NONCE.to_string(),
        cluster: VALID_FIXTURE_CLUSTER.to_string(),
        program_id: VALID_FIXTURE_PROGRAM_ID.to_string(),
        mint: VALID_FIXTURE_MINT.to_string(),
        token_account: VALID_FIXTURE_TOKEN_ACCOUNT.to_string(),
        commitment_level: CommitmentReviewLevel::ReviewOnly,
        evidence_posture: EvidencePosture::ConsistentForLocalReviewOnly,
        quorum_posture: crate::quorum::QuorumObservationPosture::EvidencePresent,
        challenge_status: ChallengeGatePosture::Closed,
        halt_status: HaltPosture::NotHalted,
        recovery_status: RecoveryPosture::NotRequired,
    }
}

pub fn valid_expected_binding_for_local_review_only() -> ExpectedProofBinding {
    let package = valid_proof_package_fixture_for_local_review_only();
    ExpectedProofBinding::from_package_identity(&package)
}

pub fn valid_composite_inputs_for_local_review_only() -> LocalProofEvidenceInputs {
    LocalProofEvidenceInputs::new(
        ChallengeWindowTiming::unopened(100),
        QuorumEvidenceCount::new(3, 0, 0, 2),
        RecoveryCaseKind::NotRequired,
    )
}

pub fn disputed_quorum_inputs_for_local_review_only() -> LocalProofEvidenceInputs {
    LocalProofEvidenceInputs::new(
        ChallengeWindowTiming::unopened(100),
        QuorumEvidenceCount::new(2, 1, 0, 2),
        RecoveryCaseKind::NotRequired,
    )
}

pub fn accepted_challenge_inputs_for_local_review_only() -> LocalProofEvidenceInputs {
    LocalProofEvidenceInputs::new(
        ChallengeWindowTiming::opened(10, 15, 5, 20),
        QuorumEvidenceCount::new(3, 0, 0, 2),
        RecoveryCaseKind::ChallengeAccepted,
    )
}

pub fn recovery_required_inputs_for_local_review_only() -> LocalProofEvidenceInputs {
    LocalProofEvidenceInputs::new(
        ChallengeWindowTiming::unopened(100),
        QuorumEvidenceCount::new(3, 0, 0, 2),
        RecoveryCaseKind::OperatorReviewRequired,
    )
}

pub fn valid_fixture_for_local_review_only() -> LocalProofFixture {
    let package = valid_proof_package_fixture_for_local_review_only();
    let expected = ExpectedProofBinding::from_package_identity(&package);

    LocalProofFixture {
        case: LocalProofFixtureCase::Valid,
        package,
        expected,
        previously_seen_nonces: Vec::new(),
        inputs: valid_composite_inputs_for_local_review_only(),
    }
}

pub fn replay_rejected_fixture_for_local_review_only() -> LocalProofFixture {
    let package = valid_proof_package_fixture_for_local_review_only();
    let expected = ExpectedProofBinding::from_package_identity(&package);

    LocalProofFixture {
        case: LocalProofFixtureCase::ReplayRejected,
        package,
        expected,
        previously_seen_nonces: vec![REPLAYED_FIXTURE_NONCE],
        inputs: valid_composite_inputs_for_local_review_only(),
    }
}

pub fn cluster_mismatch_fixture_for_local_review_only() -> LocalProofFixture {
    let mut package = valid_proof_package_fixture_for_local_review_only();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    package.cluster = CLUSTER_MISMATCH_VALUE.to_string();

    LocalProofFixture {
        case: LocalProofFixtureCase::ClusterMismatch,
        package,
        expected,
        previously_seen_nonces: Vec::new(),
        inputs: valid_composite_inputs_for_local_review_only(),
    }
}

pub fn mint_mismatch_fixture_for_local_review_only() -> LocalProofFixture {
    let mut package = valid_proof_package_fixture_for_local_review_only();
    let expected = ExpectedProofBinding::from_package_identity(&package);
    package.mint = MINT_MISMATCH_VALUE.to_string();

    LocalProofFixture {
        case: LocalProofFixtureCase::MintMismatch,
        package,
        expected,
        previously_seen_nonces: Vec::new(),
        inputs: valid_composite_inputs_for_local_review_only(),
    }
}

pub fn quorum_disputed_fixture_for_local_review_only() -> LocalProofFixture {
    let package = valid_proof_package_fixture_for_local_review_only();
    let expected = ExpectedProofBinding::from_package_identity(&package);

    LocalProofFixture {
        case: LocalProofFixtureCase::QuorumDisputed,
        package,
        expected,
        previously_seen_nonces: Vec::new(),
        inputs: disputed_quorum_inputs_for_local_review_only(),
    }
}

pub fn challenge_accepted_fixture_for_local_review_only() -> LocalProofFixture {
    let mut package = valid_proof_package_fixture_for_local_review_only();
    package.challenge_status = ChallengeGatePosture::Accepted;
    let expected = ExpectedProofBinding::from_package_identity(&package);

    LocalProofFixture {
        case: LocalProofFixtureCase::ChallengeAccepted,
        package,
        expected,
        previously_seen_nonces: Vec::new(),
        inputs: accepted_challenge_inputs_for_local_review_only(),
    }
}

pub fn recovery_required_fixture_for_local_review_only() -> LocalProofFixture {
    let mut package = valid_proof_package_fixture_for_local_review_only();
    package.recovery_status = RecoveryPosture::ReviewRequired;
    let expected = ExpectedProofBinding::from_package_identity(&package);

    LocalProofFixture {
        case: LocalProofFixtureCase::RecoveryRequired,
        package,
        expected,
        previously_seen_nonces: Vec::new(),
        inputs: recovery_required_inputs_for_local_review_only(),
    }
}

pub fn all_local_fixture_cases_for_local_review_only() -> &'static [LocalProofFixtureCase] {
    &[
        LocalProofFixtureCase::Valid,
        LocalProofFixtureCase::ReplayRejected,
        LocalProofFixtureCase::ClusterMismatch,
        LocalProofFixtureCase::MintMismatch,
        LocalProofFixtureCase::QuorumDisputed,
        LocalProofFixtureCase::ChallengeAccepted,
        LocalProofFixtureCase::RecoveryRequired,
    ]
}

pub fn fixture_helpers_authorize_runtime() -> bool {
    false
}

pub fn fixture_helpers_read_files() -> bool {
    false
}

pub fn fixture_helpers_parse_json() -> bool {
    false
}

pub fn fixture_helpers_call_rpc() -> bool {
    false
}

pub fn fixture_helpers_call_wallet() -> bool {
    false
}

pub fn fixture_helpers_are_finality() -> bool {
    false
}

pub fn fixture_helpers_are_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-E-FIXTURE-CORPUS-RUNNER
//
// Fixture corpus review is dependency-free local code only.
// It does not read files.
// It does not parse JSON.
// It does not call RPC.
// It does not call wallets.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving this batch remains fixture corpus review only.
pub const PHASE4_CODE_BATCH_E_FIXTURE_CORPUS_RUNNER_ONLY: bool = true;

/// Expected composite decision for a local fixture case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalFixtureExpectedReview {
    pub case: LocalProofFixtureCase,
    pub expected_decision: crate::validate::CompositeLocalProofReviewDecision,
}

impl LocalFixtureExpectedReview {
    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Composite review output for a single local fixture case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFixtureCorpusReviewEntry {
    pub case: LocalProofFixtureCase,
    pub expected_decision: crate::validate::CompositeLocalProofReviewDecision,
    pub observed_decision: crate::validate::CompositeLocalProofReviewDecision,
    pub matched_expectation: bool,
    pub review: crate::validate::CompositeLocalProofReview,
}

impl LocalFixtureCorpusReviewEntry {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

/// Summary over all local fixture corpus entries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFixtureCorpusReviewSummary {
    pub total_cases: usize,
    pub matched_expectations: usize,
    pub valid_for_local_review_only: usize,
    pub evidence_incomplete: usize,
    pub review_rejected: usize,
    pub runtime_not_authorized: usize,
    pub all_matched_expectations: bool,
    pub all_runtime_not_authorized: bool,
    pub all_not_finality: bool,
    pub all_not_settlement: bool,
}

impl LocalFixtureCorpusReviewSummary {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub fn expected_review_for_fixture_case_for_local_review_only(
    case: LocalProofFixtureCase,
) -> LocalFixtureExpectedReview {
    use crate::validate::CompositeLocalProofReviewDecision::{
        EvidenceIncomplete, ReviewRejected, ValidForLocalReviewOnly,
    };

    let expected_decision = match case {
        LocalProofFixtureCase::Valid => ValidForLocalReviewOnly,
        LocalProofFixtureCase::ReplayRejected
        | LocalProofFixtureCase::ClusterMismatch
        | LocalProofFixtureCase::MintMismatch
        | LocalProofFixtureCase::ChallengeAccepted => ReviewRejected,
        LocalProofFixtureCase::QuorumDisputed | LocalProofFixtureCase::RecoveryRequired => {
            EvidenceIncomplete
        }
    };

    LocalFixtureExpectedReview {
        case,
        expected_decision,
    }
}

pub fn fixture_for_case_for_local_review_only(case: LocalProofFixtureCase) -> LocalProofFixture {
    match case {
        LocalProofFixtureCase::Valid => valid_fixture_for_local_review_only(),
        LocalProofFixtureCase::ReplayRejected => replay_rejected_fixture_for_local_review_only(),
        LocalProofFixtureCase::ClusterMismatch => cluster_mismatch_fixture_for_local_review_only(),
        LocalProofFixtureCase::MintMismatch => mint_mismatch_fixture_for_local_review_only(),
        LocalProofFixtureCase::QuorumDisputed => quorum_disputed_fixture_for_local_review_only(),
        LocalProofFixtureCase::ChallengeAccepted => {
            challenge_accepted_fixture_for_local_review_only()
        }
        LocalProofFixtureCase::RecoveryRequired => {
            recovery_required_fixture_for_local_review_only()
        }
    }
}

pub fn review_fixture_for_local_review_only(
    fixture: &LocalProofFixture,
) -> crate::validate::CompositeLocalProofReview {
    crate::validate::review_composite_local_proof_for_local_review_only(
        &fixture.package,
        &fixture.expected,
        &fixture.previously_seen_nonces,
        fixture.inputs,
    )
}

pub fn review_fixture_case_for_local_review_only(
    case: LocalProofFixtureCase,
) -> LocalFixtureCorpusReviewEntry {
    let fixture = fixture_for_case_for_local_review_only(case);
    let expected = expected_review_for_fixture_case_for_local_review_only(case);
    let review = review_fixture_for_local_review_only(&fixture);
    let observed_decision = review.decision;
    let matched_expectation = observed_decision == expected.expected_decision;

    LocalFixtureCorpusReviewEntry {
        case,
        expected_decision: expected.expected_decision,
        observed_decision,
        matched_expectation,
        review,
    }
}

pub fn review_all_fixture_cases_for_local_review_only() -> Vec<LocalFixtureCorpusReviewEntry> {
    all_local_fixture_cases_for_local_review_only()
        .iter()
        .copied()
        .map(review_fixture_case_for_local_review_only)
        .collect()
}

pub fn summarize_fixture_corpus_for_local_review_only(
    entries: &[LocalFixtureCorpusReviewEntry],
) -> LocalFixtureCorpusReviewSummary {
    let mut valid_for_local_review_only = 0;
    let mut evidence_incomplete = 0;
    let mut review_rejected = 0;
    let mut runtime_not_authorized = 0;

    let mut matched_expectations = 0;
    let mut all_runtime_not_authorized = true;
    let mut all_not_finality = true;
    let mut all_not_settlement = true;

    for entry in entries {
        if entry.matched_expectation {
            matched_expectations += 1;
        }

        match entry.observed_decision {
            crate::validate::CompositeLocalProofReviewDecision::ValidForLocalReviewOnly => {
                valid_for_local_review_only += 1;
            }
            crate::validate::CompositeLocalProofReviewDecision::EvidenceIncomplete => {
                evidence_incomplete += 1;
            }
            crate::validate::CompositeLocalProofReviewDecision::ReviewRejected => {
                review_rejected += 1;
            }
            crate::validate::CompositeLocalProofReviewDecision::RuntimeNotAuthorized => {
                runtime_not_authorized += 1;
            }
        }

        all_runtime_not_authorized &=
            !entry.authorizes_runtime() && !entry.review.is_runtime_authorized();
        all_not_finality &= !entry.is_finality_claim() && !entry.review.is_finality_claim();
        all_not_settlement &= !entry.is_settlement_claim() && !entry.review.is_settlement_claim();
    }

    LocalFixtureCorpusReviewSummary {
        total_cases: entries.len(),
        matched_expectations,
        valid_for_local_review_only,
        evidence_incomplete,
        review_rejected,
        runtime_not_authorized,
        all_matched_expectations: matched_expectations == entries.len(),
        all_runtime_not_authorized,
        all_not_finality,
        all_not_settlement,
    }
}

pub fn review_fixture_corpus_for_local_review_only() -> LocalFixtureCorpusReviewSummary {
    let entries = review_all_fixture_cases_for_local_review_only();
    summarize_fixture_corpus_for_local_review_only(&entries)
}

pub fn fixture_corpus_runner_authorizes_runtime() -> bool {
    false
}

pub fn fixture_corpus_runner_reads_files() -> bool {
    false
}

pub fn fixture_corpus_runner_parses_json() -> bool {
    false
}

pub fn fixture_corpus_runner_calls_rpc() -> bool {
    false
}

pub fn fixture_corpus_runner_calls_wallet() -> bool {
    false
}

pub fn fixture_corpus_runner_is_finality() -> bool {
    false
}

pub fn fixture_corpus_runner_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-F-FIXTURE-CORPUS-REPORTS
//
// Fixture corpus reports are dependency-free local code only.
// They do not read files.
// They do not parse JSON.
// They do not call RPC.
// They do not call wallets.
// They do not authorize runtime.
// They do not prove finality.
// They do not prove settlement.

/// Compile-time marker proving fixture corpus reports remain local review only.
pub const PHASE4_CODE_BATCH_F_FIXTURE_CORPUS_REPORTS_ONLY: bool = true;

/// Deterministic report for the local fixture corpus.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFixtureCorpusReport {
    pub summary: LocalFixtureCorpusReviewSummary,
    pub entry_reports: Vec<crate::validate::CompositeLocalProofReviewReport>,
    pub all_reports_clean: bool,
    pub all_runtime_not_authorized: bool,
    pub all_not_finality: bool,
    pub all_not_settlement: bool,
}

impl LocalFixtureCorpusReport {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub fn report_fixture_corpus_for_local_review_only() -> LocalFixtureCorpusReport {
    let entries = review_all_fixture_cases_for_local_review_only();
    let summary = summarize_fixture_corpus_for_local_review_only(&entries);
    let entry_reports: Vec<crate::validate::CompositeLocalProofReviewReport> = entries
        .iter()
        .map(|entry| crate::validate::report_for_composite_local_proof_review(&entry.review))
        .collect();

    let all_reports_clean = entry_reports
        .iter()
        .all(crate::validate::CompositeLocalProofReviewReport::is_clean_local_review_only);

    LocalFixtureCorpusReport {
        summary,
        entry_reports,
        all_reports_clean,
        all_runtime_not_authorized: true,
        all_not_finality: true,
        all_not_settlement: true,
    }
}

pub fn fixture_corpus_report_authorizes_runtime() -> bool {
    false
}

pub fn fixture_corpus_report_reads_files() -> bool {
    false
}

pub fn fixture_corpus_report_parses_json() -> bool {
    false
}

pub fn fixture_corpus_report_calls_rpc() -> bool {
    false
}

pub fn fixture_corpus_report_calls_wallet() -> bool {
    false
}

pub fn fixture_corpus_report_is_finality() -> bool {
    false
}

pub fn fixture_corpus_report_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-I-FIXTURE-DECISION-GATE-RUNNER
//
// Fixture decision-gate evaluation is dependency-free local code only.
// It does not read files.
// It does not parse JSON.
// It does not call RPC.
// It does not call wallets.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving this batch remains fixture decision-gate evaluation only.
pub const PHASE4_CODE_BATCH_I_FIXTURE_GATE_RUNNER_ONLY: bool = true;

/// Full local evaluation for one fixture through the decision-gate guard.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFixtureDecisionGateEvaluation {
    pub case: LocalProofFixtureCase,
    pub expected_composite_decision: crate::validate::CompositeLocalProofReviewDecision,
    pub observed_composite_decision: crate::validate::CompositeLocalProofReviewDecision,
    pub gate_posture: crate::validate::LocalDecisionGatePosture,
    pub gate_posture_label: &'static str,
    pub status_label: &'static str,
    pub detail_label: &'static str,
    pub matched_composite_expectation: bool,
    pub accepted_for_local_review_only: bool,
    pub clean_local_review_only: bool,
    pub gate: crate::validate::LocalDecisionGateReview,
}

impl LocalFixtureDecisionGateEvaluation {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_display_authority(&self) -> bool {
        false
    }
}

/// Summary over all fixture decision-gate evaluations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFixtureDecisionGateSummary {
    pub total_cases: usize,
    pub matched_composite_expectations: usize,
    pub accepted_for_local_review_only: usize,
    pub evidence_incomplete: usize,
    pub review_rejected: usize,
    pub runtime_not_authorized: usize,
    pub all_matched_composite_expectations: bool,
    pub all_clean_local_review_only: bool,
    pub all_runtime_not_authorized: bool,
    pub all_not_finality: bool,
    pub all_not_settlement: bool,
    pub all_not_display_authority: bool,
}

impl LocalFixtureDecisionGateSummary {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_display_authority(&self) -> bool {
        false
    }
}

pub fn evaluate_fixture_decision_gate_for_local_review_only(
    case: LocalProofFixtureCase,
) -> LocalFixtureDecisionGateEvaluation {
    let fixture = fixture_for_case_for_local_review_only(case);
    let expected = expected_review_for_fixture_case_for_local_review_only(case);

    let composite = crate::validate::review_composite_local_proof_for_local_review_only(
        &fixture.package,
        &fixture.expected,
        &fixture.previously_seen_nonces,
        fixture.inputs,
    );

    let observed_composite_decision = composite.decision;
    let matched_composite_expectation =
        observed_composite_decision == expected.expected_decision;

    let gate = crate::validate::review_local_decision_gate_for_local_review_only(&composite);

    LocalFixtureDecisionGateEvaluation {
        case,
        expected_composite_decision: expected.expected_decision,
        observed_composite_decision,
        gate_posture: gate.posture,
        gate_posture_label: gate.posture_label,
        status_label: gate.status_label,
        detail_label: gate.detail_label,
        matched_composite_expectation,
        accepted_for_local_review_only: gate.passes_local_acceptance(),
        clean_local_review_only: gate.is_clean_local_review_only(),
        gate,
    }
}

pub fn evaluate_all_fixture_decision_gates_for_local_review_only(
) -> Vec<LocalFixtureDecisionGateEvaluation> {
    all_local_fixture_cases_for_local_review_only()
        .iter()
        .copied()
        .map(evaluate_fixture_decision_gate_for_local_review_only)
        .collect()
}

pub fn summarize_fixture_decision_gates_for_local_review_only(
    evaluations: &[LocalFixtureDecisionGateEvaluation],
) -> LocalFixtureDecisionGateSummary {
    let mut matched_composite_expectations = 0;
    let mut accepted_for_local_review_only = 0;
    let mut evidence_incomplete = 0;
    let mut review_rejected = 0;
    let mut runtime_not_authorized = 0;

    let mut all_clean_local_review_only = true;
    let mut all_runtime_not_authorized = true;
    let mut all_not_finality = true;
    let mut all_not_settlement = true;
    let mut all_not_display_authority = true;

    for evaluation in evaluations {
        if evaluation.matched_composite_expectation {
            matched_composite_expectations += 1;
        }

        if evaluation.accepted_for_local_review_only {
            accepted_for_local_review_only += 1;
        }

        match evaluation.gate_posture {
            crate::validate::LocalDecisionGatePosture::AcceptLocalReviewOnly => {}
            crate::validate::LocalDecisionGatePosture::EvidenceIncomplete => {
                evidence_incomplete += 1;
            }
            crate::validate::LocalDecisionGatePosture::ReviewRejected => {
                review_rejected += 1;
            }
            crate::validate::LocalDecisionGatePosture::RuntimeNotAuthorized => {
                runtime_not_authorized += 1;
            }
        }

        all_clean_local_review_only &=
            evaluation.clean_local_review_only && evaluation.gate.is_clean_local_review_only();
        all_runtime_not_authorized &=
            !evaluation.authorizes_runtime() && !evaluation.gate.authorizes_runtime();
        all_not_finality &=
            !evaluation.is_finality_claim() && !evaluation.gate.is_finality_claim();
        all_not_settlement &=
            !evaluation.is_settlement_claim() && !evaluation.gate.is_settlement_claim();
        all_not_display_authority &=
            !evaluation.is_display_authority() && !evaluation.gate.is_display_authority();
    }

    LocalFixtureDecisionGateSummary {
        total_cases: evaluations.len(),
        matched_composite_expectations,
        accepted_for_local_review_only,
        evidence_incomplete,
        review_rejected,
        runtime_not_authorized,
        all_matched_composite_expectations: matched_composite_expectations
            == evaluations.len(),
        all_clean_local_review_only,
        all_runtime_not_authorized,
        all_not_finality,
        all_not_settlement,
        all_not_display_authority,
    }
}

pub fn evaluate_fixture_decision_gate_corpus_for_local_review_only(
) -> LocalFixtureDecisionGateSummary {
    let evaluations = evaluate_all_fixture_decision_gates_for_local_review_only();
    summarize_fixture_decision_gates_for_local_review_only(&evaluations)
}

pub fn fixture_gate_runner_authorizes_runtime() -> bool {
    false
}

pub fn fixture_gate_runner_reads_files() -> bool {
    false
}

pub fn fixture_gate_runner_parses_json() -> bool {
    false
}

pub fn fixture_gate_runner_calls_rpc() -> bool {
    false
}

pub fn fixture_gate_runner_calls_wallet() -> bool {
    false
}

pub fn fixture_gate_runner_is_finality() -> bool {
    false
}

pub fn fixture_gate_runner_is_settlement() -> bool {
    false
}

pub fn fixture_gate_runner_is_display_authority() -> bool {
    false
}
