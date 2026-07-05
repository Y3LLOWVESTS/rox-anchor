//! RO:WHAT — Tests relayer halt/recovery submission gate behavior.
//! RO:WHY — BUILD_PLAN2 Phase 12 requires relayer refusal while halted and after unsafe recovery posture.
//! RO:INTERACTS — RelayerSubmissionRequest, RelayerDryRun, AnchorOperationalPosture, simulation, and capped submit models.
//! RO:INVARIANTS — halted or recovery-blocked posture prevents attempts, simulation, and capped submission authorization.
//! RO:SECURITY — local dry-run tests only; no RPC, keypair loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-relayer --test halt_recovery_submit_gate.

use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorOperationalPosture, AnchorSafetyProfile,
    ChallengePosture, ClusterAllowlist, HaltPosture, RecoveryPosture, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
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

fn request_with_posture(posture: AnchorOperationalPosture) -> RelayerSubmissionRequest {
    let package = fixtures::valid_package();

    RelayerSubmissionRequest::new(
        package.operation_id,
        package.idempotency_key,
        "phase12-halt-recovery-target",
        accepted_review(),
    )
    .with_operational_posture(posture)
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

#[test]
fn relayer_refuses_submission_while_halted() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(request_with_posture(AnchorOperationalPosture::halted()))
        .expect("halted posture still records a refusal receipt");

    assert_eq!(receipt.status, RelayerReceiptStatus::Halted);
    assert_eq!(receipt.attempts_used, 0);
    assert!(!receipt.live_submission);
}

#[test]
fn relayer_refuses_challenge_and_recovery_blocked_postures() {
    let challenge_open = AnchorOperationalPosture::new(
        ChallengePosture::Open,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );
    let recovery_blocked = AnchorOperationalPosture::new(
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::Required,
    );

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let challenge_receipt = relayer
        .submit_dry_run(request_with_posture(challenge_open))
        .expect("challenge-blocked posture should record a refusal receipt");
    let recovery_receipt = relayer
        .submit_dry_run(request_with_posture(recovery_blocked))
        .expect("recovery-blocked posture should record a refusal receipt");

    assert_eq!(
        challenge_receipt.status,
        RelayerReceiptStatus::ChallengeBlocked
    );
    assert_eq!(
        recovery_receipt.status,
        RelayerReceiptStatus::RecoveryBlocked
    );
    assert_eq!(challenge_receipt.attempts_used, 0);
    assert_eq!(recovery_receipt.attempts_used, 0);
}

#[test]
fn halted_receipt_cannot_be_simulated_or_capped_submitted() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(request_with_posture(AnchorOperationalPosture::halted()))
        .expect("halted posture should record receipt");

    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(receipt, true, 2);
    let simulation = simulate_transaction_plan(RelayerConfig::new(3, 16), simulation_plan);

    assert_eq!(
        simulation.status,
        TransactionSimulationStatus::RelayerDryRunNotAccepted
    );
    assert_eq!(simulation.relayer_status, RelayerReceiptStatus::Halted);
    assert!(!simulation.simulated);
    assert!(!simulation.live_submission);

    let capped_plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let capped =
        authorize_capped_testnet_submission(capped_testnet_config(), limits(), capped_plan);

    assert_eq!(
        capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!capped.authorized);
    assert!(!capped.live_submission_attempted);
    assert!(!capped.network_submitted);
}

#[test]
fn recovered_posture_allows_dry_run_again() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
    let receipt = relayer
        .submit_dry_run(request_with_posture(
            AnchorOperationalPosture::recovery_resolved(),
        ))
        .expect("resolved recovery posture should allow normal dry-run review");

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(receipt.attempts_used, 1);
    assert!(!receipt.live_submission);
}
