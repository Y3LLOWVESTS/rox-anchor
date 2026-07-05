//! RO:WHAT — CLI audit report commands.
//! RO:WHY — Phase 11 requires terminal-visible audit output for RPC, proof, coordinator, relayer, simulation, and capped submission decisions.
//! RO:INTERACTS — rox-anchor-coordinator audit records, rox-anchor-relayer audit records, proof fixtures, and RPC observations.
//! RO:INVARIANTS — output is deterministic, display-safe, and never claims settlement or completed network submission.
//! RO:SECURITY — no live RPC, wallet/key loading, signing, transaction submission, minting, burning, ROC release, or settlement.
//! RO:TEST — covered by CLI coordinator and relayer audit report tests.

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorAuditRecord, CoordinatorConfig, CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AnchorCluster, AnchorEnvironmentMode, AnchorSafetyProfile, ClusterAllowlist, SubmissionMode,
};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, RelayerConfig, RelayerDryRun, RelayerSubmissionRequest,
    TestnetRelayerAuditRecord, TransactionSimulationPlan,
};
use rox_anchor_rpc_proof::{ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation};

pub fn audit_report() -> String {
    let request = accepted_request();
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);
    let audit = CoordinatorAuditRecord::from_review(&request, &decision, 100);

    let mut lines = vec![
        "rox-anchor audit".to_string(),
        "status: coordinator_audit_report".to_string(),
        "submission: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "network_client: not_enabled".to_string(),
        "runtime_authority: not_enabled".to_string(),
        format!("coordinator_status: {}", audit.coordinator_status),
        format!("rpc_decision: {}", audit.rpc_decision),
        format!("proof_decision: {}", audit.proof_decision),
        format!("permits_simulation: {}", audit.permits_simulation),
        "audit:".to_string(),
    ];

    lines.extend(audit.render().lines().map(|line| format!("  {line}")));

    lines.extend([
        "security: report-only; no RPC submission, wallet/key loading, mint/burn, ROC release, or settlement".to_string(),
        "next: inspect `rox-anchor proof` for RPC-only audit details".to_string(),
    ]);

    lines.join("\n")
}

pub fn relayer_audit_report() -> String {
    let audit = accepted_relayer_pipeline();

    let mut lines = vec![
        "rox-anchor audit-relayer".to_string(),
        "status: relayer_simulation_capped_audit_report".to_string(),
        "submission: capped_testnet_report_only".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "network_client: not_enabled".to_string(),
        "runtime_authority: not_enabled".to_string(),
        format!("relayer_status: {}", audit.relayer_status),
        format!("proof_decision: {}", audit.proof_decision),
        format!("simulation_status: {}", audit.simulation_status),
        format!(
            "capped_submission_status: {}",
            audit.capped_submission_status
        ),
        format!("receipt_persisted: {}", audit.receipt_persisted),
        format!("authorized: {}", audit.authorized),
        format!(
            "live_submission_attempted: {}",
            audit.live_submission_attempted
        ),
        format!("network_submitted: {}", audit.network_submitted),
        "audit:".to_string(),
    ];

    lines.extend(audit.render().lines().map(|line| format!("  {line}")));

    lines.extend([
        "security: report-only; no RPC submission, wallet/key loading, mint/burn execution, ROC release, or settlement".to_string(),
        "next: inspect `rox-anchor audit` for coordinator audit details".to_string(),
    ]);

    lines.join("\n")
}

fn accepted_request() -> CoordinatorReviewRequest {
    let package = fixtures::valid_package();
    let expected = package.expected_binding_snapshot();
    let expected_rpc = expected_rpc_binding();
    let observations = matching_observations(&expected_rpc);

    CoordinatorReviewRequest::new(
        package,
        expected,
        expected_rpc,
        observations,
        ReplaySet::default(),
    )
}

fn expected_rpc_binding() -> ExpectedRpcBinding {
    let expected = fixtures::expected_proof_binding();
    let binding = expected.binding.clone();

    ExpectedRpcBinding::new(
        binding.cluster,
        binding.program_id,
        binding.mint,
        binding.token_account,
        expected.operation_id,
        RpcCommitmentLevel::Finalized,
    )
}

fn matching_observations(expected: &ExpectedRpcBinding) -> Vec<RpcObservation> {
    vec![
        RpcObservation::new(
            "audit-cli-rpc-a",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "audit-cli-same-signature-0001",
            100,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "audit-cli-rpc-b",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "audit-cli-same-signature-0001",
            100,
            RpcCommitmentLevel::Finalized,
        ),
    ]
}

fn accepted_relayer_pipeline() -> TestnetRelayerAuditRecord {
    let package = fixtures::valid_package();
    let mut relayer = RelayerDryRun::new(simulation_config());

    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "audit-cli-relayer-target",
            accepted_review(),
        ))
        .expect("static CLI relayer audit receipt should fit capacity");

    let simulation = simulate_transaction_plan(
        simulation_config(),
        TransactionSimulationPlan::from_dry_run_receipt(receipt.clone(), true, 1),
    );

    let capped = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(true),
    );

    TestnetRelayerAuditRecord::from_pipeline(&receipt, &simulation, &capped, true)
}

fn accepted_review() -> rox_anchor_proof::ProofReview {
    review_proof_package(
        &fixtures::valid_package(),
        &fixtures::expected_proof_binding(),
        &ReplaySet::default(),
    )
}

fn simulation_config() -> RelayerConfig {
    RelayerConfig::new(2, 8)
}

fn capped_testnet_config() -> RelayerConfig {
    RelayerConfig::new_with_safety(
        2,
        8,
        AnchorSafetyProfile::new(
            AnchorEnvironmentMode::TestnetOnly,
            AnchorCluster::Testnet,
            ClusterAllowlist::testnet_experiments(),
            SubmissionMode::TestnetSubmitCapped,
        ),
    )
}

fn capped_limits() -> CappedTestnetSubmissionLimits {
    CappedTestnetSubmissionLimits::new(2, 2, 100, true)
}
