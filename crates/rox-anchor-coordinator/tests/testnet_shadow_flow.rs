//! RO:WHAT — BUILD_PLAN2 Phase 9 end-to-end testnet shadow-flow tests.
//! RO:WHY — Proves ROC ↔ ROX testnet-shaped fixtures can cross RPC proof, coordinator, relayer, simulation, and capped authorization.
//! RO:INTERACTS — rox-anchor-core, rox-anchor-proof, rox-anchor-rpc-proof, rox-anchor-coordinator, and rox-anchor-relayer.
//! RO:INVARIANTS — replay, mismatches, challenge, halt, and recovery blockers prevent authorization.
//! RO:SECURITY — no live RPC, wallet/key loading, public ROX mint/burn, real ROC release, settlement, or network submission.
//! RO:TEST — run with cargo test -p rox-anchor-coordinator --test testnet_shadow_flow.

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecision, CoordinatorDecisionStatus,
    CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture, SubmissionMode,
    TokenAccountId,
};
use rox_anchor_proof::{
    EvidenceBundle, ExpectedProofBinding, ProofFindingCode, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_relayer::{
    authorize_capped_testnet_submission, simulate_transaction_plan, CappedTestnetSubmissionLimits,
    CappedTestnetSubmissionPlan, CappedTestnetSubmissionResult, CappedTestnetSubmissionStatus,
    RelayerConfig, RelayerDryRun, RelayerReceipt, RelayerReceiptStatus, RelayerSubmissionRequest,
    TransactionSimulationPlan, TransactionSimulationResult, TransactionSimulationStatus,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision, RpcQuorumFindingCode,
};

#[derive(Clone, Debug)]
struct ShadowFixture {
    package: ProofPackage,
    expected: ExpectedProofBinding,
    expected_rpc: ExpectedRpcBinding,
    observations: Vec<RpcObservation>,
    current_slot: u64,
}

#[derive(Clone, Debug)]
struct ShadowFlowResult {
    decision: CoordinatorDecision,
    receipt: RelayerReceipt,
    simulation: TransactionSimulationResult,
    capped: CappedTestnetSubmissionResult,
}

fn coordinator_config() -> CoordinatorConfig {
    CoordinatorConfig::new(2, 100, 8)
}

fn simulation_config() -> RelayerConfig {
    RelayerConfig::new(3, 16)
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

fn capped_limits() -> CappedTestnetSubmissionLimits {
    CappedTestnetSubmissionLimits::new(2, 2, 100, true)
}

fn shadow_fixture(direction: AnchorDirection) -> ShadowFixture {
    let suffix = direction.as_str();
    let binding = shadow_binding(direction);
    let operation_id = OperationId::new(format!("testnet-shadow-{suffix}-0001")).unwrap();
    let idempotency_key =
        IdempotencyKey::new(format!("idem-testnet-shadow-{suffix}-0001")).unwrap();
    let nonce = Nonce::new(format!("nonce-testnet-shadow-{suffix}-0001")).unwrap();

    let package = ProofPackage::new(
        binding.clone(),
        operation_id.clone(),
        idempotency_key.clone(),
        nonce.clone(),
        AccountId::new(format!("source-account-{suffix}-0001")).unwrap(),
        AccountId::new(format!("target-account-{suffix}-0001")).unwrap(),
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );

    let expected = ExpectedProofBinding::new(
        binding.clone(),
        operation_id.clone(),
        idempotency_key,
        nonce,
    );

    let expected_rpc = ExpectedRpcBinding::new(
        binding.cluster.clone(),
        binding.program_id.clone(),
        binding.mint.clone(),
        binding.token_account.clone(),
        operation_id.clone(),
        RpcCommitmentLevel::Confirmed,
    );

    let observations = vec![
        shadow_observation("testnet-rpc-a", &binding, &operation_id, 90),
        shadow_observation("testnet-rpc-b", &binding, &operation_id, 91),
    ];

    ShadowFixture {
        package,
        expected,
        expected_rpc,
        observations,
        current_slot: 100,
    }
}

fn shadow_binding(direction: AnchorDirection) -> AnchorBinding {
    let (source_domain, target_domain, token_account) = match direction {
        AnchorDirection::RocToRox => (
            "internal-roc-test-fixture",
            "solana-testnet-rox-fixture",
            "testnet-rox-recipient-token-account-0001",
        ),
        AnchorDirection::RoxToRoc => (
            "solana-testnet-rox-fixture",
            "internal-roc-test-fixture",
            "testnet-rox-burn-source-token-account-0001",
        ),
    };

    AnchorBinding::new(
        DomainId::new(source_domain).unwrap(),
        DomainId::new(target_domain).unwrap(),
        direction,
        ClusterId::new("testnet").unwrap(),
        ProgramId::new("TestnetRoxAnchorProgram111111111111111").unwrap(),
        MintId::new("TestOnlyRoxMint111111111111111111111111").unwrap(),
        TokenAccountId::new(token_account).unwrap(),
    )
}

fn shadow_observation(
    source: &str,
    binding: &AnchorBinding,
    operation_id: &OperationId,
    slot: u64,
) -> RpcObservation {
    RpcObservation::new(
        source,
        binding.cluster.clone(),
        binding.program_id.clone(),
        binding.mint.clone(),
        binding.token_account.clone(),
        operation_id.clone(),
        "testnet-shadow-signature-same-0001",
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn request_from_fixture(fixture: &ShadowFixture, replay: ReplaySet) -> CoordinatorReviewRequest {
    CoordinatorReviewRequest::new(
        fixture.package.clone(),
        fixture.expected.clone(),
        fixture.expected_rpc.clone(),
        fixture.observations.clone(),
        replay,
    )
}

fn run_shadow_flow(fixture: ShadowFixture, replay: ReplaySet) -> ShadowFlowResult {
    let request = request_from_fixture(&fixture, replay);
    let decision = review_coordinator_request(&request, coordinator_config(), fixture.current_slot);

    let mut relayer = RelayerDryRun::new(simulation_config());
    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            fixture.package.operation_id.clone(),
            fixture.package.idempotency_key.clone(),
            format!(
                "testnet-shadow-target-{}",
                fixture.package.binding.direction.as_str()
            ),
            decision.proof_review.clone(),
        ))
        .expect("testnet shadow-flow receipt capacity should be available");

    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(
        receipt.clone(),
        decision.permits_transaction_simulation(),
        2,
    );
    let simulation = simulate_transaction_plan(simulation_config(), simulation_plan);

    let capped_plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let capped =
        authorize_capped_testnet_submission(capped_testnet_config(), capped_limits(), capped_plan);

    ShadowFlowResult {
        decision,
        receipt,
        simulation,
        capped,
    }
}

fn assert_shadow_flow_authorized_without_network_submission(flow: &ShadowFlowResult) {
    assert_eq!(flow.decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Agreement
    );
    assert_eq!(
        flow.decision.proof_review.decision,
        ReviewDecision::Accepted
    );

    assert_eq!(flow.receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(flow.receipt.attempts_used, 1);

    assert_eq!(
        flow.simulation.status,
        TransactionSimulationStatus::Simulated
    );
    assert!(flow.simulation.simulated);
    assert!(!flow.simulation.live_submission);

    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::Authorized
    );
    assert!(flow.capped.authorized);
    assert!(flow.capped.live_submission_permitted);
    assert!(!flow.capped.live_submission_attempted);
    assert!(!flow.capped.network_submitted);
}

#[test]
fn roc_to_rox_testnet_shadow_flow_reaches_capped_authorization_without_public_mint() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    assert_eq!(fixture.package.binding.direction, AnchorDirection::RocToRox);
    assert_eq!(fixture.package.binding.cluster.as_str(), "testnet");
    assert_eq!(
        fixture.package.binding.source_domain.as_str(),
        "internal-roc-test-fixture"
    );
    assert_eq!(
        fixture.package.binding.target_domain.as_str(),
        "solana-testnet-rox-fixture"
    );

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_shadow_flow_authorized_without_network_submission(&flow);
}

#[test]
fn rox_to_roc_testnet_shadow_flow_reaches_capped_authorization_without_roc_release() {
    let fixture = shadow_fixture(AnchorDirection::RoxToRoc);
    assert_eq!(fixture.package.binding.direction, AnchorDirection::RoxToRoc);
    assert_eq!(fixture.package.binding.cluster.as_str(), "testnet");
    assert_eq!(
        fixture.package.binding.source_domain.as_str(),
        "solana-testnet-rox-fixture"
    );
    assert_eq!(
        fixture.package.binding.target_domain.as_str(),
        "internal-roc-test-fixture"
    );

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_shadow_flow_authorized_without_network_submission(&flow);
}

#[test]
fn replayed_testnet_shadow_operation_is_rejected_before_simulation_or_submission() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    let replay = ReplaySet::from_package(&fixture.package);

    let flow = run_shadow_flow(fixture, replay);

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::RejectedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Agreement
    );
    assert_eq!(
        flow.decision.proof_review.decision,
        ReviewDecision::Rejected
    );
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::ReplayOperationId));
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::ReplayIdempotencyKey));
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::ReplayNonce));

    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofRejected);
    assert_eq!(flow.receipt.attempts_used, 0);
    assert_eq!(
        flow.simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!flow.capped.authorized);
    assert!(!flow.capped.network_submitted);
}

#[test]
fn mismatched_testnet_shadow_mint_is_rejected_before_relayer_acceptance() {
    let mut fixture = shadow_fixture(AnchorDirection::RocToRox);
    fixture.observations[0].mint = MintId::new("WrongTestnetRoxMint111111111111111111").unwrap();

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::RejectedEvidence
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Rejected
    );
    assert!(flow
        .decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::MintMismatch));

    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(flow.receipt.attempts_used, 0);
    assert_eq!(
        flow.simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!flow.capped.authorized);
    assert!(!flow.capped.network_submitted);
}

#[test]
fn challenge_halt_and_recovery_blockers_stop_testnet_shadow_authorization() {
    let mut challenged = shadow_fixture(AnchorDirection::RocToRox);
    challenged.package.challenge_posture = ChallengePosture::Open;
    let challenged_flow = run_shadow_flow(challenged, ReplaySet::default());

    assert_eq!(
        challenged_flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        challenged_flow.receipt.status,
        RelayerReceiptStatus::ProofBlocked
    );
    assert_eq!(
        challenged_flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!challenged_flow.capped.network_submitted);

    let mut halted = shadow_fixture(AnchorDirection::RocToRox);
    halted.package.halt_posture = HaltPosture::Halted;
    let halted_flow = run_shadow_flow(halted, ReplaySet::default());

    assert_eq!(
        halted_flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        halted_flow.receipt.status,
        RelayerReceiptStatus::ProofBlocked
    );
    assert_eq!(
        halted_flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!halted_flow.capped.network_submitted);

    let mut recovery = shadow_fixture(AnchorDirection::RoxToRoc);
    recovery.package.recovery_posture = RecoveryPosture::Required;
    let recovery_flow = run_shadow_flow(recovery, ReplaySet::default());

    assert_eq!(
        recovery_flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        recovery_flow.receipt.status,
        RelayerReceiptStatus::ProofBlocked
    );
    assert_eq!(
        recovery_flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!recovery_flow.capped.network_submitted);
}

#[test]
fn capped_authorization_requires_receipt_persisted_from_relayer_inventory() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    let request = request_from_fixture(&fixture, ReplaySet::default());
    let decision = review_coordinator_request(&request, coordinator_config(), fixture.current_slot);

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);

    let mut relayer = RelayerDryRun::new(simulation_config());
    let receipt = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            fixture.package.operation_id.clone(),
            fixture.package.idempotency_key.clone(),
            "testnet-shadow-receipt-persistence-target",
            decision.proof_review.clone(),
        ))
        .expect("accepted shadow proof should create a relayer receipt");

    assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(relayer.receipts().len(), 1);

    let receipt_persisted = relayer.receipts().iter().any(|stored| {
        stored.operation_id == receipt.operation_id
            && stored.idempotency_key == receipt.idempotency_key
            && stored.status == RelayerReceiptStatus::DryRunAccepted
            && !stored.live_submission
    });

    assert!(receipt_persisted);

    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(
        receipt,
        decision.permits_transaction_simulation(),
        2,
    );
    let simulation = simulate_transaction_plan(simulation_config(), simulation_plan);

    assert_eq!(simulation.status, TransactionSimulationStatus::Simulated);
    assert!(!simulation.live_submission);

    let missing_receipt_plan =
        CappedTestnetSubmissionPlan::from_simulation_result(simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(true)
            .with_receipt_persisted(false);

    let missing_receipt = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        missing_receipt_plan,
    );

    assert_eq!(
        missing_receipt.status,
        CappedTestnetSubmissionStatus::ReceiptPersistenceMissing
    );
    assert!(!missing_receipt.authorized);
    assert!(!missing_receipt.live_submission_attempted);
    assert!(!missing_receipt.network_submitted);

    let persisted_receipt_plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(receipt_persisted);

    let persisted_receipt = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        persisted_receipt_plan,
    );

    assert_eq!(
        persisted_receipt.status,
        CappedTestnetSubmissionStatus::Authorized
    );
    assert!(persisted_receipt.authorized);
    assert!(persisted_receipt.live_submission_permitted);
    assert!(!persisted_receipt.live_submission_attempted);
    assert!(!persisted_receipt.network_submitted);
}

#[test]
fn duplicate_shadow_idempotency_receipt_blocks_simulation_and_capped_authorization() {
    let fixture = shadow_fixture(AnchorDirection::RoxToRoc);
    let request = request_from_fixture(&fixture, ReplaySet::default());
    let decision = review_coordinator_request(&request, coordinator_config(), fixture.current_slot);

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);

    let mut relayer = RelayerDryRun::new(simulation_config());

    let first_request = RelayerSubmissionRequest::new(
        fixture.package.operation_id.clone(),
        fixture.package.idempotency_key.clone(),
        "testnet-shadow-duplicate-first",
        decision.proof_review.clone(),
    );

    let duplicate_request = RelayerSubmissionRequest::new(
        fixture.package.operation_id.clone(),
        fixture.package.idempotency_key.clone(),
        "testnet-shadow-duplicate-second",
        decision.proof_review.clone(),
    );

    let first = relayer
        .submit_dry_run(first_request)
        .expect("first accepted shadow request should create a receipt");
    let duplicate = relayer
        .submit_dry_run(duplicate_request)
        .expect("duplicate idempotency should create a rejection receipt");

    assert_eq!(first.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(duplicate.status, RelayerReceiptStatus::DuplicateRequest);
    assert_eq!(duplicate.attempts_used, 0);
    assert_eq!(relayer.receipts().len(), 2);

    let duplicate_receipt_persisted = relayer.receipts().iter().any(|stored| {
        stored.operation_id == duplicate.operation_id
            && stored.idempotency_key == duplicate.idempotency_key
            && stored.status == RelayerReceiptStatus::DuplicateRequest
            && !stored.live_submission
    });

    assert!(duplicate_receipt_persisted);

    let simulation_plan = TransactionSimulationPlan::from_dry_run_receipt(
        duplicate,
        decision.permits_transaction_simulation(),
        2,
    );
    let simulation = simulate_transaction_plan(simulation_config(), simulation_plan);

    assert_eq!(
        simulation.status,
        TransactionSimulationStatus::RelayerDryRunNotAccepted
    );
    assert!(!simulation.simulated);
    assert!(!simulation.live_submission);

    let capped_plan = CappedTestnetSubmissionPlan::from_simulation_result(simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(duplicate_receipt_persisted);

    let capped =
        authorize_capped_testnet_submission(capped_testnet_config(), capped_limits(), capped_plan);

    assert_eq!(
        capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!capped.authorized);
    assert!(!capped.live_submission_attempted);
    assert!(!capped.network_submitted);
}

#[test]
fn receipt_capacity_pressure_blocks_shadow_flow_before_simulation() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    let request = request_from_fixture(&fixture, ReplaySet::default());
    let decision = review_coordinator_request(&request, coordinator_config(), fixture.current_slot);

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);

    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 1));

    let first = relayer
        .submit_dry_run(RelayerSubmissionRequest::new(
            OperationId::new("testnet-shadow-capacity-first").unwrap(),
            IdempotencyKey::new("idem-testnet-shadow-capacity-first").unwrap(),
            "testnet-shadow-capacity-first-target",
            decision.proof_review.clone(),
        ))
        .expect("first receipt should fit capacity");

    assert_eq!(first.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(relayer.receipts().len(), 1);

    let overflow = relayer.submit_dry_run(RelayerSubmissionRequest::new(
        fixture.package.operation_id.clone(),
        fixture.package.idempotency_key.clone(),
        "testnet-shadow-capacity-overflow-target",
        decision.proof_review,
    ));

    assert!(overflow.is_err());
    assert_eq!(relayer.receipts().len(), 1);

    let persisted_shadow_receipt = relayer.receipts().iter().any(|stored| {
        stored.operation_id == fixture.package.operation_id
            && stored.idempotency_key == fixture.package.idempotency_key
    });

    assert!(!persisted_shadow_receipt);
}

fn assert_shadow_flow_blocked_without_network_submission(flow: &ShadowFlowResult) {
    assert!(!flow.capped.authorized);
    assert!(!flow.capped.live_submission_attempted);
    assert!(!flow.capped.network_submitted);
}

fn assert_rpc_rejection_blocks_shadow_flow(
    mut fixture: ShadowFixture,
    finding: RpcQuorumFindingCode,
) {
    let flow = run_shadow_flow(fixture.clone(), ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::RejectedEvidence
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Rejected
    );
    assert!(flow.decision.rpc_review.has_finding(finding));
    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);

    fixture.current_slot += 1;
    let repeated = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(repeated.decision.status, flow.decision.status);
    assert_eq!(
        repeated.decision.rpc_review.decision,
        flow.decision.rpc_review.decision
    );
    assert!(repeated.decision.rpc_review.has_finding(finding));
    assert_eq!(repeated.capped.status, flow.capped.status);
    assert_shadow_flow_blocked_without_network_submission(&repeated);
}

fn assert_proof_rejection_blocks_shadow_flow(fixture: ShadowFixture, finding: ProofFindingCode) {
    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::RejectedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Agreement
    );
    assert_eq!(
        flow.decision.proof_review.decision,
        ReviewDecision::Rejected
    );
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|proof_finding| proof_finding.code == finding));
    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofRejected);
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);
}

fn plan_from_shadow_simulation(
    simulation: TransactionSimulationResult,
) -> CappedTestnetSubmissionPlan {
    CappedTestnetSubmissionPlan::from_simulation_result(simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true)
}

#[test]
fn rpc_provider_disagreement_blocks_shadow_flow_before_relayer_acceptance() {
    let mut fixture = shadow_fixture(AnchorDirection::RocToRox);
    let binding = fixture.package.binding.clone();
    let operation_id = fixture.package.operation_id.clone();

    fixture.observations = vec![
        RpcObservation::new(
            "testnet-rpc-a",
            binding.cluster.clone(),
            binding.program_id.clone(),
            binding.mint.clone(),
            binding.token_account.clone(),
            operation_id.clone(),
            "testnet-shadow-signature-disputed-a",
            90,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "testnet-rpc-b",
            binding.cluster.clone(),
            binding.program_id.clone(),
            binding.mint.clone(),
            binding.token_account.clone(),
            operation_id,
            "testnet-shadow-signature-disputed-b",
            91,
            RpcCommitmentLevel::Finalized,
        ),
    ];

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Disputed
    );
    assert!(flow
        .decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::RpcEquivocation));
    assert_eq!(flow.decision.proof_review.decision, ReviewDecision::Blocked);
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::QuorumDisputed));
    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);
}

#[test]
fn rpc_binding_mismatch_matrix_blocks_shadow_flow_before_simulation() {
    let mut cluster = shadow_fixture(AnchorDirection::RocToRox);
    cluster.observations[0].cluster = ClusterId::new("devnet").unwrap();
    assert_rpc_rejection_blocks_shadow_flow(cluster, RpcQuorumFindingCode::ClusterMismatch);

    let mut program = shadow_fixture(AnchorDirection::RocToRox);
    program.observations[0].program_id =
        ProgramId::new("WrongTestnetRoxAnchorProgram111111111").unwrap();
    assert_rpc_rejection_blocks_shadow_flow(program, RpcQuorumFindingCode::ProgramIdMismatch);

    let mut token_account = shadow_fixture(AnchorDirection::RoxToRoc);
    token_account.observations[0].token_account =
        TokenAccountId::new("wrong-testnet-rox-token-account-0001").unwrap();
    assert_rpc_rejection_blocks_shadow_flow(
        token_account,
        RpcQuorumFindingCode::TokenAccountMismatch,
    );

    let mut operation = shadow_fixture(AnchorDirection::RoxToRoc);
    operation.observations[0].operation_id =
        OperationId::new("wrong-testnet-shadow-operation-0001").unwrap();
    assert_rpc_rejection_blocks_shadow_flow(operation, RpcQuorumFindingCode::OperationIdMismatch);
}

#[test]
fn proof_binding_tamper_matrix_blocks_shadow_flow_before_capped_authorization() {
    let mut wrong_direction = shadow_fixture(AnchorDirection::RocToRox);
    wrong_direction.package.binding.direction = wrong_direction.package.binding.direction.reverse();
    assert_proof_rejection_blocks_shadow_flow(wrong_direction, ProofFindingCode::DirectionMismatch);

    let mut wrong_source_domain = shadow_fixture(AnchorDirection::RocToRox);
    wrong_source_domain.package.binding.source_domain =
        DomainId::new("tampered-internal-roc-test-fixture").unwrap();
    assert_proof_rejection_blocks_shadow_flow(
        wrong_source_domain,
        ProofFindingCode::SourceDomainMismatch,
    );

    let mut wrong_target_domain = shadow_fixture(AnchorDirection::RoxToRoc);
    wrong_target_domain.package.binding.target_domain =
        DomainId::new("tampered-internal-roc-release-fixture").unwrap();
    assert_proof_rejection_blocks_shadow_flow(
        wrong_target_domain,
        ProofFindingCode::TargetDomainMismatch,
    );

    let mut wrong_idempotency = shadow_fixture(AnchorDirection::RoxToRoc);
    wrong_idempotency.package.idempotency_key =
        IdempotencyKey::new("tampered-idem-testnet-shadow-rox-to-roc-0001").unwrap();
    assert_proof_rejection_blocks_shadow_flow(
        wrong_idempotency,
        ProofFindingCode::IdempotencyKeyMismatch,
    );

    let mut wrong_nonce = shadow_fixture(AnchorDirection::RocToRox);
    wrong_nonce.package.nonce =
        Nonce::new("tampered-nonce-testnet-shadow-roc-to-rox-0001").unwrap();
    assert_proof_rejection_blocks_shadow_flow(wrong_nonce, ProofFindingCode::NonceMismatch);
}

#[test]
fn capped_submit_bypass_attempt_matrix_is_rejected_after_clean_shadow_simulation() {
    let flow = run_shadow_flow(
        shadow_fixture(AnchorDirection::RocToRox),
        ReplaySet::default(),
    );

    assert_eq!(flow.decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(
        flow.simulation.status,
        TransactionSimulationStatus::Simulated
    );
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::Authorized
    );
    assert!(!flow.capped.live_submission_attempted);
    assert!(!flow.capped.network_submitted);

    let unsafe_local_scope = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::TestnetSubmitCapped,
    );
    let unsafe_local = authorize_capped_testnet_submission(
        RelayerConfig::new_with_safety(3, 16, unsafe_local_scope),
        capped_limits(),
        plan_from_shadow_simulation(flow.simulation.clone()),
    );

    assert_eq!(
        unsafe_local.status,
        CappedTestnetSubmissionStatus::UnsafeScope
    );
    assert!(!unsafe_local.authorized);
    assert!(!unsafe_local.network_submitted);

    let simulate_only_scope = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Testnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );
    let simulate_only = authorize_capped_testnet_submission(
        RelayerConfig::new_with_safety(3, 16, simulate_only_scope),
        capped_limits(),
        plan_from_shadow_simulation(flow.simulation.clone()),
    );

    assert_eq!(
        simulate_only.status,
        CappedTestnetSubmissionStatus::UnsafeScope
    );
    assert!(!simulate_only.authorized);
    assert!(!simulate_only.network_submitted);

    let no_operator_approval =
        CappedTestnetSubmissionPlan::from_simulation_result(flow.simulation.clone())
            .with_requested_attempts(1)
            .with_requested_operations(1)
            .with_amount_units(10)
            .with_explicit_operator_approval(false)
            .with_receipt_persisted(true);
    let no_operator = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        no_operator_approval,
    );

    assert_eq!(
        no_operator.status,
        CappedTestnetSubmissionStatus::MissingExplicitOperatorApproval
    );
    assert!(!no_operator.authorized);
    assert!(!no_operator.network_submitted);

    let zero_operations =
        plan_from_shadow_simulation(flow.simulation.clone()).with_requested_operations(0);
    let zero_operations_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        zero_operations,
    );

    assert_eq!(
        zero_operations_result.status,
        CappedTestnetSubmissionStatus::EmptyOperationRun
    );
    assert!(!zero_operations_result.authorized);
    assert!(!zero_operations_result.network_submitted);

    let retry_over_cap =
        plan_from_shadow_simulation(flow.simulation.clone()).with_requested_attempts(3);
    let retry_over_cap_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        retry_over_cap,
    );

    assert_eq!(
        retry_over_cap_result.status,
        CappedTestnetSubmissionStatus::RetryCapExceeded
    );
    assert!(!retry_over_cap_result.authorized);
    assert!(!retry_over_cap_result.network_submitted);

    let amount_over_cap = plan_from_shadow_simulation(flow.simulation).with_amount_units(101);
    let amount_over_cap_result = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        amount_over_cap,
    );

    assert_eq!(
        amount_over_cap_result.status,
        CappedTestnetSubmissionStatus::AmountCapExceeded
    );
    assert!(!amount_over_cap_result.authorized);
    assert!(!amount_over_cap_result.network_submitted);
}

#[test]
fn missing_rpc_evidence_blocks_shadow_flow_without_relayer_attempts() {
    let mut fixture = shadow_fixture(AnchorDirection::RocToRox);
    fixture.observations.clear();

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::MissingEvidence
    );
    assert!(flow
        .decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::MissingEvidence));

    assert_eq!(flow.decision.proof_review.decision, ReviewDecision::Blocked);
    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(flow.receipt.attempts_used, 0);
    assert_eq!(
        flow.simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);
}

#[test]
fn under_quorum_rpc_evidence_blocks_shadow_flow_without_capped_authorization() {
    let mut fixture = shadow_fixture(AnchorDirection::RoxToRoc);
    fixture.observations.truncate(1);

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::MissingEvidence
    );
    assert!(flow
        .decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::MissingEvidence));

    assert_eq!(flow.decision.proof_review.decision, ReviewDecision::Blocked);
    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(flow.receipt.attempts_used, 0);
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);
}

#[test]
fn stale_rpc_evidence_blocks_shadow_flow_but_fresh_evidence_can_recover() {
    let mut stale_fixture = shadow_fixture(AnchorDirection::RocToRox);
    stale_fixture.current_slot = 1_000;

    let stale = run_shadow_flow(stale_fixture, ReplaySet::default());

    assert_eq!(
        stale.decision.status,
        CoordinatorDecisionStatus::RejectedEvidence
    );
    assert_eq!(
        stale.decision.rpc_review.decision,
        RpcQuorumDecision::Rejected
    );
    assert!(stale
        .decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::StaleEvidence));
    assert_eq!(stale.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(
        stale.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&stale);

    let fresh = run_shadow_flow(
        shadow_fixture(AnchorDirection::RocToRox),
        ReplaySet::default(),
    );

    assert_eq!(fresh.decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(
        fresh.decision.rpc_review.decision,
        RpcQuorumDecision::Agreement
    );
    assert_eq!(
        fresh.decision.proof_review.decision,
        ReviewDecision::Accepted
    );
    assert_eq!(fresh.receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(
        fresh.simulation.status,
        TransactionSimulationStatus::Simulated
    );
    assert_eq!(
        fresh.capped.status,
        CappedTestnetSubmissionStatus::Authorized
    );
    assert!(!fresh.capped.live_submission_attempted);
    assert!(!fresh.capped.network_submitted);
}

#[test]
fn same_source_rpc_equivocation_blocks_shadow_flow_before_simulation() {
    let mut fixture = shadow_fixture(AnchorDirection::RoxToRoc);
    let binding = fixture.package.binding.clone();
    let operation_id = fixture.package.operation_id.clone();

    fixture.observations = vec![
        RpcObservation::new(
            "testnet-rpc-equivocating-source",
            binding.cluster.clone(),
            binding.program_id.clone(),
            binding.mint.clone(),
            binding.token_account.clone(),
            operation_id.clone(),
            "testnet-shadow-same-source-signature-a",
            90,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "testnet-rpc-equivocating-source",
            binding.cluster.clone(),
            binding.program_id.clone(),
            binding.mint.clone(),
            binding.token_account.clone(),
            operation_id,
            "testnet-shadow-same-source-signature-b",
            91,
            RpcCommitmentLevel::Finalized,
        ),
    ];

    let flow = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Disputed
    );
    assert!(flow
        .decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::SourceEquivocation));
    assert_eq!(flow.decision.proof_review.decision, ReviewDecision::Blocked);
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|finding| finding.code == ProofFindingCode::QuorumDisputed));

    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(flow.receipt.attempts_used, 0);
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);
}

#[test]
fn blocked_shadow_simulation_cannot_be_rescued_by_submit_flags() {
    let mut fixture = shadow_fixture(AnchorDirection::RocToRox);
    fixture.package.challenge_posture = ChallengePosture::Accepted;

    let blocked = run_shadow_flow(fixture, ReplaySet::default());

    assert_eq!(
        blocked.decision.status,
        CoordinatorDecisionStatus::BlockedProof
    );
    assert_eq!(
        blocked.decision.proof_review.decision,
        ReviewDecision::Blocked
    );
    assert_eq!(blocked.receipt.status, RelayerReceiptStatus::ProofBlocked);
    assert_eq!(
        blocked.simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert_eq!(
        blocked.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&blocked);

    let bypass_attempt = CappedTestnetSubmissionPlan::from_simulation_result(blocked.simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let capped = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        bypass_attempt,
    );

    assert_eq!(
        capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!capped.authorized);
    assert!(!capped.live_submission_permitted);
    assert!(!capped.live_submission_attempted);
    assert!(!capped.network_submitted);
}

fn replay_set_with_only_operation_id(fixture: &ShadowFixture) -> ReplaySet {
    let mut replay = ReplaySet::new();
    replay.insert_operation_id(fixture.package.operation_id.clone());
    replay
}

fn replay_set_with_only_idempotency_key(fixture: &ShadowFixture) -> ReplaySet {
    let mut replay = ReplaySet::new();
    replay.insert_idempotency_key(fixture.package.idempotency_key.clone());
    replay
}

fn replay_set_with_only_nonce(fixture: &ShadowFixture) -> ReplaySet {
    let mut replay = ReplaySet::new();
    replay.insert_nonce(fixture.package.nonce.clone());
    replay
}

fn assert_single_replay_finding_blocks_shadow_flow(
    fixture: ShadowFixture,
    replay: ReplaySet,
    expected_finding: ProofFindingCode,
) {
    let flow = run_shadow_flow(fixture, replay);

    assert_eq!(
        flow.decision.status,
        CoordinatorDecisionStatus::RejectedProof
    );
    assert_eq!(
        flow.decision.rpc_review.decision,
        RpcQuorumDecision::Agreement
    );
    assert_eq!(
        flow.decision.proof_review.decision,
        ReviewDecision::Rejected
    );
    assert!(flow
        .decision
        .proof_review
        .findings
        .iter()
        .any(|finding| finding.code == expected_finding));

    assert_eq!(flow.receipt.status, RelayerReceiptStatus::ProofRejected);
    assert_eq!(flow.receipt.attempts_used, 0);
    assert_eq!(
        flow.simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );
    assert_eq!(
        flow.capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert_shadow_flow_blocked_without_network_submission(&flow);
}

#[test]
fn isolated_operation_id_replay_blocks_shadow_flow_without_relayer_attempts() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    let replay = replay_set_with_only_operation_id(&fixture);

    assert_single_replay_finding_blocks_shadow_flow(
        fixture,
        replay,
        ProofFindingCode::ReplayOperationId,
    );
}

#[test]
fn isolated_idempotency_key_replay_blocks_shadow_flow_without_relayer_attempts() {
    let fixture = shadow_fixture(AnchorDirection::RoxToRoc);
    let replay = replay_set_with_only_idempotency_key(&fixture);

    assert_single_replay_finding_blocks_shadow_flow(
        fixture,
        replay,
        ProofFindingCode::ReplayIdempotencyKey,
    );
}

#[test]
fn isolated_nonce_replay_blocks_shadow_flow_without_relayer_attempts() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    let replay = replay_set_with_only_nonce(&fixture);

    assert_single_replay_finding_blocks_shadow_flow(fixture, replay, ProofFindingCode::ReplayNonce);
}

#[test]
fn replay_storm_matrix_is_deterministic_and_does_not_poison_clean_review() {
    let operation_fixture = shadow_fixture(AnchorDirection::RocToRox);
    let idempotency_fixture = shadow_fixture(AnchorDirection::RoxToRoc);
    let nonce_fixture = shadow_fixture(AnchorDirection::RocToRox);

    for (fixture, replay, finding) in [
        (
            operation_fixture.clone(),
            replay_set_with_only_operation_id(&operation_fixture),
            ProofFindingCode::ReplayOperationId,
        ),
        (
            idempotency_fixture.clone(),
            replay_set_with_only_idempotency_key(&idempotency_fixture),
            ProofFindingCode::ReplayIdempotencyKey,
        ),
        (
            nonce_fixture.clone(),
            replay_set_with_only_nonce(&nonce_fixture),
            ProofFindingCode::ReplayNonce,
        ),
    ] {
        let first = run_shadow_flow(fixture.clone(), replay.clone());
        let second = run_shadow_flow(fixture, replay);

        assert_eq!(
            first.decision.status,
            CoordinatorDecisionStatus::RejectedProof
        );
        assert_eq!(
            second.decision.status,
            CoordinatorDecisionStatus::RejectedProof
        );
        assert_eq!(
            first.decision.proof_review.decision,
            ReviewDecision::Rejected
        );
        assert_eq!(
            second.decision.proof_review.decision,
            ReviewDecision::Rejected
        );
        assert!(first
            .decision
            .proof_review
            .findings
            .iter()
            .any(|proof_finding| proof_finding.code == finding));
        assert!(second
            .decision
            .proof_review
            .findings
            .iter()
            .any(|proof_finding| proof_finding.code == finding));
        assert_eq!(
            first.capped.status,
            CappedTestnetSubmissionStatus::SimulationNotAccepted
        );
        assert_eq!(
            second.capped.status,
            CappedTestnetSubmissionStatus::SimulationNotAccepted
        );
        assert_shadow_flow_blocked_without_network_submission(&first);
        assert_shadow_flow_blocked_without_network_submission(&second);
    }

    let clean = run_shadow_flow(
        shadow_fixture(AnchorDirection::RoxToRoc),
        ReplaySet::default(),
    );

    assert_eq!(clean.decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(
        clean.decision.rpc_review.decision,
        RpcQuorumDecision::Agreement
    );
    assert_eq!(
        clean.decision.proof_review.decision,
        ReviewDecision::Accepted
    );
    assert_eq!(clean.receipt.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(
        clean.simulation.status,
        TransactionSimulationStatus::Simulated
    );
    assert_eq!(
        clean.capped.status,
        CappedTestnetSubmissionStatus::Authorized
    );
    assert!(clean.capped.authorized);
    assert!(clean.capped.live_submission_permitted);
    assert!(!clean.capped.live_submission_attempted);
    assert!(!clean.capped.network_submitted);
}

#[test]
fn replay_rejection_cannot_be_rescued_by_persisted_receipt_or_operator_approval() {
    let fixture = shadow_fixture(AnchorDirection::RocToRox);
    let replay = replay_set_with_only_operation_id(&fixture);
    let blocked = run_shadow_flow(fixture, replay);

    assert_eq!(
        blocked.decision.status,
        CoordinatorDecisionStatus::RejectedProof
    );
    assert_eq!(blocked.receipt.status, RelayerReceiptStatus::ProofRejected);
    assert_eq!(
        blocked.simulation.status,
        TransactionSimulationStatus::CoordinatorNotAccepted
    );

    let bypass_attempt = CappedTestnetSubmissionPlan::from_simulation_result(blocked.simulation)
        .with_requested_attempts(1)
        .with_requested_operations(1)
        .with_amount_units(10)
        .with_explicit_operator_approval(true)
        .with_receipt_persisted(true);

    let capped = authorize_capped_testnet_submission(
        capped_testnet_config(),
        capped_limits(),
        bypass_attempt,
    );

    assert_eq!(
        capped.status,
        CappedTestnetSubmissionStatus::SimulationNotAccepted
    );
    assert!(!capped.authorized);
    assert!(!capped.live_submission_permitted);
    assert!(!capped.live_submission_attempted);
    assert!(!capped.network_submitted);
}
