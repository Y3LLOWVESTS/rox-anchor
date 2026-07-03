//! RO:WHAT — Local-only proof package review crate for ROX Anchor Phase 4.
//! RO:WHY — Provides deterministic evidence-review code without proof finality, runtime behavior, or value movement.
//! RO:INTERACTS — package, validate, replay, quorum, challenge, and recovery local review modules.
//! RO:INVARIANTS — Proof packages are evidence only; local review is not finality; this crate does not authorize runtime.
//! RO:SECURITY — No RPC, no wallet, no Solana/Anchor runtime, no bridge runtime, no deployment, no value movement.
//! RO:TEST — Static Phase 4 checker only for this round.
//!
//! ROX-ANCHOR:FUTURE-GATED-CONTEXT
//!
//! This local validator does not authorize runtime.

#![forbid(unsafe_code)]

pub mod challenge;
pub mod package;
pub mod fixtures;
pub mod quorum;
pub mod recovery;
pub mod replay;
pub mod validate;

pub use challenge::{
    challenge_window_review_authorizes_runtime, challenge_window_review_is_finality,
    challenge_window_review_is_settlement, review_challenge_posture,
    review_challenge_window_for_local_review_only, ChallengeGatePosture,
    ChallengeReviewFinding, ChallengeWindowClockFinding, ChallengeWindowClockReview,
    ChallengeWindowReview, ChallengeWindowReviewDecision, ChallengeWindowSkeleton,
    ChallengeWindowTiming,
};
pub use fixtures::{
    accepted_challenge_inputs_for_local_review_only,
    all_local_fixture_cases_for_local_review_only,
    challenge_accepted_fixture_for_local_review_only,
    cluster_mismatch_fixture_for_local_review_only,
    disputed_quorum_inputs_for_local_review_only,
    fixture_helpers_are_finality,
    fixture_helpers_are_settlement,
    fixture_helpers_authorize_runtime,
    fixture_helpers_call_rpc,
    fixture_helpers_call_wallet,
    fixture_helpers_parse_json,
    fixture_helpers_read_files,
    mint_mismatch_fixture_for_local_review_only,
    recovery_required_fixture_for_local_review_only,
    recovery_required_inputs_for_local_review_only,
    replay_rejected_fixture_for_local_review_only,
    valid_composite_inputs_for_local_review_only,
    valid_expected_binding_for_local_review_only,
    valid_fixture_for_local_review_only,
    valid_proof_package_fixture_for_local_review_only,
    expected_review_for_fixture_case_for_local_review_only,
    fixture_corpus_runner_authorizes_runtime,
    fixture_corpus_runner_calls_rpc,
    fixture_corpus_runner_calls_wallet,
    fixture_corpus_runner_is_finality,
    fixture_corpus_runner_is_settlement,
    fixture_corpus_runner_parses_json,
    fixture_corpus_runner_reads_files,
    fixture_for_case_for_local_review_only,
    review_all_fixture_cases_for_local_review_only,
    review_fixture_case_for_local_review_only,
    review_fixture_corpus_for_local_review_only,
    review_fixture_for_local_review_only,
    summarize_fixture_corpus_for_local_review_only,
    LocalFixtureCorpusReviewEntry, LocalFixtureCorpusReviewSummary,
    LocalFixtureExpectedReview,
    LocalFixtureCorpusReport, report_fixture_corpus_for_local_review_only,
    LocalFixtureDecisionGateEvaluation, LocalFixtureDecisionGateSummary,
    evaluate_fixture_decision_gate_for_local_review_only,
    evaluate_all_fixture_decision_gates_for_local_review_only,
    summarize_fixture_decision_gates_for_local_review_only,
    evaluate_fixture_decision_gate_corpus_for_local_review_only,
    fixture_gate_runner_authorizes_runtime,
    fixture_gate_runner_calls_rpc,
    fixture_gate_runner_calls_wallet,
    fixture_gate_runner_is_display_authority,
    fixture_gate_runner_is_finality,
    fixture_gate_runner_is_settlement,
    fixture_gate_runner_parses_json,
    fixture_gate_runner_reads_files,
    fixture_corpus_report_authorizes_runtime,
    fixture_corpus_report_calls_rpc,
    fixture_corpus_report_calls_wallet,
    fixture_corpus_report_is_finality,
    fixture_corpus_report_is_settlement,
    fixture_corpus_report_parses_json,
    fixture_corpus_report_reads_files,
    LocalProofFixture, LocalProofFixtureCase,
};
pub use package::{
    BridgeOperationIdentity, CommitmentReviewLevel, EvidencePosture, OperationIdentityField,
    OperationIdentityStatus, ProofDirection, ProofPackageShape, RequiredProofField,
};
pub use quorum::{
    quorum_evidence_review_authorizes_runtime, quorum_evidence_review_calls_rpc,
    quorum_evidence_review_is_finality, quorum_evidence_review_is_settlement,
    review_quorum_evidence_counts_for_local_review_only, review_quorum_posture,
    QuorumEvidenceCount, QuorumEvidenceReview, QuorumEvidenceReviewDecision,
    QuorumEvidenceReviewFinding, QuorumObservationPosture, QuorumReview,
    QuorumReviewFinding, QuorumReviewSkeleton,
};
pub use recovery::{
    halt_recovery_review_authorizes_runtime, halt_recovery_review_is_finality,
    halt_recovery_review_is_settlement, halt_recovery_review_touches_ledger,
    halt_recovery_review_touches_wallet, recovery_action_intent_for_local_review_only,
    review_halt_posture, review_halt_recovery_for_local_review_only,
    review_recovery_posture, HaltPosture, HaltRecoveryReview, HaltRecoveryReviewDecision,
    HaltRecoveryReviewFinding, RecoveryActionIntent, RecoveryCaseKind, RecoveryPosture,
    RecoveryReview, RecoveryReviewFinding, RecoveryReviewSkeleton,
};
pub use replay::{
    review_operation_identity_for_local_review_only, review_replay_binding,
    review_static_nonce_for_local_review_only, ExpectedProofBinding, NonceReview,
    NonceReviewFinding, OperationIdentityReview, OperationIdentityReviewFinding,
    ReplayBindingReview, ReplayBindingSkeleton, ReplayPosture,
};
pub use validate::{
    authorized_static_fixture_vector_inventory_for_local_review_only,
    composite_local_proof_review_authorizes_runtime,
    composite_local_proof_review_calls_rpc,
    composite_local_proof_review_calls_wallet,
    composite_local_proof_review_is_finality,
    composite_local_proof_review_is_settlement,
    find_fixture_expectation_by_case_id_for_local_review_only,
    find_fixture_expectation_by_path_for_local_review_only,
    find_static_fixture_vector_inventory_entry_for_local_review_only,
    fixture_expectation_accepts_review_for_local_review_only,
    fixture_expectation_matrix_authorizes_runtime,
    fixture_expectation_matrix_contains_case_id_for_local_review_only,
    fixture_expectation_matrix_contains_path_for_local_review_only,
    fixture_expectation_matrix_is_finality,
    fixture_expectation_matrix_is_settlement,
    fixture_expectation_matrix_parses_json,
    fixture_expectation_matrix_reads_files,
    is_authorized_static_fixture_vector_path_for_local_review_only,
    phase4_fixture_expectation_matrix_for_local_review_only,
    review_composite_local_proof_for_local_review_only,
    review_package_for_local_review_only,
    review_package_state_transition_for_local_review_only,
    review_package_with_seen_nonces_for_local_review_only,
    review_required_fields,
    review_state_transition_for_local_review_only,
    state_transition_is_supported_for_local_review_only,
    state_transition_review_authorizes_runtime,
    state_transition_review_calls_rpc,
    state_transition_review_calls_wallet,
    state_transition_review_is_finality,
    state_transition_review_is_settlement,
    state_transition_review_parses_json,
    state_transition_review_reads_files,
    CompositeLocalProofReview, CompositeLocalProofReviewDecision,
    CompositeLocalProofReviewReport, LocalReviewAuthorityPosture,
    LocalReviewReportSeverity, report_for_composite_local_proof_review,
    CompositeLocalProofReviewTrace, LocalReviewStatusLabel,
    LocalReviewStatusProjection, LocalReviewTraceStep,
    LocalReviewTraceStepKind, trace_for_composite_local_proof_review,
    LocalDecisionGateFinding, LocalDecisionGatePosture,
    LocalDecisionGateReview, review_local_decision_gate_for_local_review_only,
    local_decision_gate_guard_authorizes_runtime,
    local_decision_gate_guard_calls_rpc,
    local_decision_gate_guard_calls_wallet,
    local_decision_gate_guard_is_display_authority,
    local_decision_gate_guard_is_finality,
    local_decision_gate_guard_is_settlement,
    local_decision_gate_guard_parses_json,
    local_decision_gate_guard_reads_files,
    status_projection_for_composite_local_proof_review,
    local_review_trace_authorizes_runtime,
    local_review_trace_calls_rpc,
    local_review_trace_calls_wallet,
    local_review_trace_is_finality,
    local_review_trace_is_settlement,
    local_review_trace_parses_json,
    local_review_trace_reads_files,
    local_review_status_projection_authorizes_runtime,
    local_review_status_projection_calls_rpc,
    local_review_status_projection_calls_wallet,
    local_review_status_projection_is_finality,
    local_review_status_projection_is_settlement,
    local_review_status_projection_is_display_authority,
    local_review_report_authorizes_runtime,
    local_review_report_calls_rpc,
    local_review_report_calls_wallet,
    local_review_report_is_finality,
    local_review_report_is_settlement,
    local_review_report_parses_json,
    local_review_report_reads_files,
    CompositeLocalProofReviewFinding, ExpectedFindingSet, FixtureExpectationKind,
    FixtureExpectationMatrixEntry, LocalProofEvidenceInputs, LocalProofReviewDecision,
    LocalProofState, ProofReview, ProofReviewFinding, ProofReviewSkeleton,
    StateTransitionIntent, StateTransitionReview, StateTransitionReviewDecision,
    StateTransitionReviewFinding, StateTransitionReviewSkeleton,
    StaticFixtureVectorInventoryEntry, StaticFixtureVectorKind,
};

/// Compile-time marker proving this crate remains non-runtime.
pub const ROX_ANCHOR_PROOF_DISABLED_SKELETON: bool = true;

/// Compile-time marker proving this crate is only a local validator.
pub const ROX_ANCHOR_PROOF_LOCAL_VALIDATOR_ONLY: bool = true;

/// Compile-time marker proving Round 4.2 remains operation-identity and nonce review only.
pub const ROX_ANCHOR_PROOF_OPERATION_IDENTITY_NONCE_ONLY: bool = true;

/// Compile-time marker proving Code Batch B remains local challenge/quorum/recovery review only.
pub const ROX_ANCHOR_PROOF_CHALLENGE_QUORUM_RECOVERY_ONLY: bool = true;

/// Compile-time marker proving Code Batch C remains composite local proof review only.
pub const ROX_ANCHOR_PROOF_COMPOSITE_LOCAL_REVIEW_ONLY: bool = true;

/// Human-readable non-authorization marker used by static review tools.
pub const ROX_ANCHOR_PROOF_NON_AUTHORIZATION: &str =
    "rox-anchor-proof is a local-only validator and does not authorize runtime";

/// Compile-time marker proving Code Batch D remains local fixture helpers only.
pub const ROX_ANCHOR_PROOF_FIXTURE_HELPERS_ONLY: bool = true;

/// Compile-time marker proving Code Batch E remains local fixture corpus review only.
pub const ROX_ANCHOR_PROOF_FIXTURE_CORPUS_RUNNER_ONLY: bool = true;

/// Compile-time marker proving Code Batch F remains local review reports only.
pub const ROX_ANCHOR_PROOF_LOCAL_REVIEW_REPORTS_ONLY: bool = true;

/// Compile-time marker proving Code Batch G remains local trace/status projection only.
pub const ROX_ANCHOR_PROOF_LOCAL_TRACE_STATUS_ONLY: bool = true;

/// Compile-time marker proving Code Batch H remains local decision-gate guard only.
pub const ROX_ANCHOR_PROOF_LOCAL_DECISION_GATE_GUARD_ONLY: bool = true;

/// Compile-time marker proving Code Batch I remains fixture decision-gate evaluation only.
pub const ROX_ANCHOR_PROOF_FIXTURE_GATE_RUNNER_ONLY: bool = true;
