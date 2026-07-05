//! RO:WHAT — Tests BUILD_PLAN2 Phase 8 capped testnet submission authorization.
//! RO:WHY — Proves submit-shaped authorization requires testnet scope, simulation, caps, approval, and receipt persistence.
//! RO:INTERACTS — RelayerDryRun, TransactionSimulationResult, capped submit model, and core safety profile.
//! RO:INVARIANTS — default modes cannot authorize submission; authorized results still do not send transactions.
//! RO:SECURITY — no RPC, key loading, wallet, mint, burn, settlement, or network submission.
//! RO:TEST — run with cargo test -p rox-anchor-relayer --test capped_testnet_submission.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, EvidenceBundle, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, CappedTestnetSubmissionStatus, RelayerConfig, RelayerDryRun,
    RelayerReceiptStatus, RelayerSubmissionRequest, TransactionSimulationPlan,
    TransactionSimulationStatus,
};

fn accepted_review() -> rox_anchor_proof::ProofReview {
    review_proof_package(
        &fixtures::valid_package(),
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn blocked_review() -> rox_anchor_proof::ProofReview {
    let mut package = fixtures::valid_package();
    package.evidence = EvidenceBundle::new(0, 2, 0);

    review_proof_package(
        &package,
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn request_with_review(review: rox_anchor_proof::ProofReview) -> RelayerSubmissionRequest {
    let package = fixtures::valid_package();

    RelayerSubmissionRequest::new(
        package.operation_id,
        package.idempotency_key,
        "capped-testnet-submit-target",
        review,
    )
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

fn simulation_config() -> RelayerConfig {
    RelayerConfig::new(3, 16)
}

fn limits() -> CappedTestnetSubmissionLimits {
    CappedTestnetSubmissionLimits::new(2, 2, 100, true)
}

fn accepted_simulation_result() -> rox_anchor_relayer::TransactionSimulationResult {
    let mut relayer = RelayerDryRun::new(simulation_config());
    let receipt = relayer
        .submit_dry_run(request_with_review(accepted_review()))
        .expect("accepted proof should produce a dry-run receipt");
    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);

    simulate_transaction_plan(simulation_config(), simulation_plan)
}

fn approved_capped_plan() -> CappedTestnetSubmissionPlan {
    CappedTestnetSubmissionPlan::from_simulation_result(accepted_simulation_result())
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true)
}

#[test]
fn capped_testnet_submission_authorizes_only_after_all_gates() {
    let result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_capped_plan(),
    );

    assert_eq!(result.status, CappedTestnetSubmissionStatus::Authorized);
    assert!(result.is_authorized());
    assert!(result.authorized);
    assert!(result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
    assert_eq!(result.proof_decision, ReviewDecision::Accepted);
    assert_eq!(result.relayer_status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(
        result.simulation_status,
        TransactionSimulationStatus::Simulated
    );
}

#[test]
fn default_non_submitting_scope_cannot_authorize_capped_submit() {
    let result = authorize_capped_testnet_submission(
        RelayerConfig::new(3, 16),
        limits(),
        approved_capped_plan(),
    );

    assert_eq!(result.status, CappedTestnetSubmissionStatus::UnsafeScope);
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
}

#[test]
fn blocked_or_unsimulated_result_cannot_authorize_capped_submit() {
    let mut relayer = RelayerDryRun::new(simulation_config());
    let receipt = relayer
        .submit_dry_run(request_with_review(blocked_review()))
        .expect("blocked proof should still create a non-attempt receipt");
    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);
    let simulation_result = simulate_transaction_plan(simulation_config(), simulation_plan);

    assert_eq!(
        simulation_result.status,
        TransactionSimulationStatus::ProofNotAccepted
    );

    let plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation_result)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let result = authorize_capped_testnet_submission(capped_testnet_config(), limits(), plan);

    assert_eq!(
        result.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!result.authorized);
    assert!(!result.network_submitted);
}

#[test]
fn explicit_operator_approval_is_required() {
    let plan = approved_capped_plan().with_explicit_operator_approval(false);

    let result = authorize_capped_testnet_submission(capped_testnet_config(), limits(), plan);

    assert_eq!(
        result.status,
        CappedTestnetSubmissionStatus::MissingExplicitOperatorApproval
    );
    assert!(!result.authorized);
}

#[test]
fn retry_operation_and_amount_caps_are_enforced() {
    let retry_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_capped_plan().with_requested_attempts(3),
    );
    assert_eq!(
        retry_result.status,
        CappedTestnetSubmissionStatus::RetryCapExceeded
    );

    let operation_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_capped_plan().with_requested_operations(3),
    );
    assert_eq!(
        operation_result.status,
        CappedTestnetSubmissionStatus::OperationCapExceeded
    );

    let amount_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_capped_plan().with_amount_units(101),
    );
    assert_eq!(
        amount_result.status,
        CappedTestnetSubmissionStatus::AmountCapExceeded
    );
}

#[test]
fn receipt_persistence_is_required_when_limit_says_so() {
    let plan = approved_capped_plan().with_receipt_persisted(false);

    let result = authorize_capped_testnet_submission(capped_testnet_config(), limits(), plan);

    assert_eq!(
        result.status,
        CappedTestnetSubmissionStatus::ReceiptPersistenceMissing
    );
    assert!(!result.authorized);
    assert!(!result.network_submitted);
}

fn approved_plan_from_simulation(
    simulation_result: rox_anchor_relayer::TransactionSimulationResult,
) -> CappedTestnetSubmissionPlan {
    CappedTestnetSubmissionPlan::from_simulation_result(simulation_result)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true)
}

#[test]
fn capped_submit_rejects_tampered_simulated_proof_decision() {
    let mut simulation = accepted_simulation_result();
    simulation.proof_decision = ReviewDecision::Rejected;

    let result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_plan_from_simulation(simulation),
    );

    assert_eq!(
        result.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_eq!(result.proof_decision, ReviewDecision::Rejected);
    assert_eq!(result.relayer_status, RelayerReceiptStatus::DryRunAccepted);
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
}

#[test]
fn capped_submit_rejects_tampered_simulated_relayer_status() {
    let mut simulation = accepted_simulation_result();
    simulation.relayer_status = RelayerReceiptStatus::DuplicateRequest;

    let result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_plan_from_simulation(simulation),
    );

    assert_eq!(
        result.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_eq!(result.proof_decision, ReviewDecision::Accepted);
    assert_eq!(
        result.relayer_status,
        RelayerReceiptStatus::DuplicateRequest
    );
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
}

#[test]
fn capped_submit_rejects_tampered_live_submission_flag_even_when_other_fields_look_accepted() {
    let mut simulation = accepted_simulation_result();
    simulation.live_submission = true;

    let result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        limits(),
        approved_plan_from_simulation(simulation),
    );

    assert_eq!(
        result.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_eq!(result.proof_decision, ReviewDecision::Accepted);
    assert_eq!(result.relayer_status, RelayerReceiptStatus::DryRunAccepted);
    assert!(!result.authorized);
    assert!(!result.live_submission_permitted);
    assert!(!result.live_submission_attempted);
    assert!(!result.network_submitted);
}

#[test]
fn capped_submit_rejects_combined_simulation_tamper_matrix() {
    for (proof_decision, relayer_status, live_submission) in [
        (
            ReviewDecision::Blocked,
            RelayerReceiptStatus::DryRunAccepted,
            false,
        ),
        (
            ReviewDecision::Rejected,
            RelayerReceiptStatus::DryRunAccepted,
            false,
        ),
        (
            ReviewDecision::Accepted,
            RelayerReceiptStatus::ProofBlocked,
            false,
        ),
        (
            ReviewDecision::Accepted,
            RelayerReceiptStatus::ProofRejected,
            false,
        ),
        (
            ReviewDecision::Accepted,
            RelayerReceiptStatus::DuplicateRequest,
            false,
        ),
        (
            ReviewDecision::Accepted,
            RelayerReceiptStatus::DryRunAccepted,
            true,
        ),
    ] {
        let mut simulation = accepted_simulation_result();
        simulation.proof_decision = proof_decision;
        simulation.relayer_status = relayer_status;
        simulation.live_submission = live_submission;

        let result = authorize_capped_testnet_submission(
            capped_testnet_config(),
            limits(),
            approved_plan_from_simulation(simulation),
        );

        assert_eq!(
            result.status,
            CappedTestnetSubmissionStatus::SimulationNotAccepted
        );
        assert!(!result.authorized);
        assert!(!result.live_submission_permitted);
        assert!(!result.live_submission_attempted);
        assert!(!result.network_submitted);
    }
}
