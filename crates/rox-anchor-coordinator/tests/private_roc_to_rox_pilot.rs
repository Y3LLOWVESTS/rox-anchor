//! RO:WHAT — Tests BUILD_PLAN3 Phase 12 private ROC-to-ROX pilot coordinator path.
//! RO:WHY — Proves test-only CrabLink/internal ROC burn intent can feed proof/RPC/coordinator review.
//! RO:INTERACTS — core dry-run burn intent, RPC quorum evidence, coordinator decision, proof validation.
//! RO:INVARIANTS — operation ID, idempotency key, nonce, mint, token account, and RPC evidence must match.
//! RO:SECURITY — no real ROC burn, wallet call, ron-ledger mutation, paid unlock, public mint, or settlement.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test private_roc_to_rox_pilot.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorInternalRocDryRunObservation, CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, InternalRocDryRunBurnIntent, MintId, Nonce, OperationId, ProgramId,
    RecoveryPosture, SubmissionMode, TokenAccountId,
};
use rox_anchor_proof::{
    EvidenceBundle, ExpectedProofBinding, ProofFindingCode, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision, RpcQuorumFindingCode,
};

#[derive(Clone, Debug)]
struct ForwardFixture {
    package: ProofPackage,
    expected: ExpectedProofBinding,
    expected_rpc: ExpectedRpcBinding,
    observations: Vec<RpcObservation>,
    burn_intent: InternalRocDryRunBurnIntent,
    current_slot: u64,
}

fn safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    )
}

fn binding() -> AnchorBinding {
    AnchorBinding::new(
        DomainId::new("internal-roc-private-pilot-test").unwrap(),
        DomainId::new("solana-devnet-rox-private-pilot-test").unwrap(),
        AnchorDirection::RocToRox,
        ClusterId::new("devnet").unwrap(),
        ProgramId::new("PrivatePilotRoxAnchorProgram11111111").unwrap(),
        MintId::new("TestOnlyPrivatePilotRoxMint111111111").unwrap(),
        TokenAccountId::new("PrivatePilotRoxRecipientToken1111111").unwrap(),
    )
}

fn fixture() -> ForwardFixture {
    let binding = binding();
    let operation_id = OperationId::new("private-roc-to-rox-op-0001").unwrap();
    let idempotency_key = IdempotencyKey::new("private-roc-to-rox-idem-0001").unwrap();
    let nonce = Nonce::new("private-roc-to-rox-nonce-0001").unwrap();
    let source_account = AccountId::new("crablink-private-roc-burn-source-0001").unwrap();

    let package = ProofPackage::new(
        binding.clone(),
        operation_id.clone(),
        idempotency_key.clone(),
        nonce.clone(),
        source_account.clone(),
        AccountId::new("private-rox-token-owner-0001").unwrap(),
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    );

    let expected = ExpectedProofBinding::new(
        binding.clone(),
        operation_id.clone(),
        idempotency_key.clone(),
        nonce.clone(),
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
        observation("pilot-rpc-a", &binding, &operation_id, 120),
        observation("pilot-rpc-b", &binding, &operation_id, 121),
    ];

    let burn_intent = InternalRocDryRunBurnIntent::new(
        safety(),
        operation_id,
        idempotency_key,
        nonce,
        source_account,
        "test-only-private-roc-to-rox-burn-intent",
        10,
    )
    .expect("static private ROC-to-ROX burn intent should validate");

    ForwardFixture {
        package,
        expected,
        expected_rpc,
        observations,
        burn_intent,
        current_slot: 125,
    }
}

fn observation(
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
        "private-roc-to-rox-signature-same-0001",
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn proof_has_finding(review: &rox_anchor_proof::ProofReview, code: ProofFindingCode) -> bool {
    review.findings.iter().any(|finding| finding.code == code)
}

fn request(fixture: &ForwardFixture, replay: ReplaySet) -> CoordinatorReviewRequest {
    CoordinatorReviewRequest::new(
        fixture.package.clone(),
        fixture.expected.clone(),
        fixture.expected_rpc.clone(),
        fixture.observations.clone(),
        replay,
    )
}

#[test]
fn private_roc_to_rox_burn_intent_reaches_accepted_coordinator_review() {
    let fixture = fixture();

    let burn_observation =
        CoordinatorInternalRocDryRunObservation::from_burn_intent(&fixture.burn_intent)
            .expect("burn intent should become coordinator dry-run observation");
    let burn_report = burn_observation.redacted_report();

    assert!(burn_report.contains("coordinator_internal_roc_dry_run_observation: accepted"));
    assert!(burn_report.contains("kind: burn_intent_input"));
    assert!(burn_report.contains("real_internal_roc_burn: disabled"));
    assert!(burn_report.contains("ron_ledger_mutation: disabled"));
    assert!(burn_report.contains("paid_content_unlock: disabled"));
    assert!(burn_report.contains("settlement_claim: none"));

    let decision = review_coordinator_request(
        &request(&fixture, ReplaySet::default()),
        CoordinatorConfig::new(2, 100, 8),
        fixture.current_slot,
    );

    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);
    assert!(decision.permits_transaction_simulation());
}

#[test]
fn replayed_private_roc_to_rox_ids_are_rejected_before_relayer_work() {
    let fixture = fixture();
    let decision = review_coordinator_request(
        &request(&fixture, ReplaySet::from_package(&fixture.package)),
        CoordinatorConfig::new(2, 100, 8),
        fixture.current_slot,
    );

    assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedProof);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Rejected);
    assert!(proof_has_finding(
        &decision.proof_review,
        ProofFindingCode::ReplayOperationId
    ));
    assert!(proof_has_finding(
        &decision.proof_review,
        ProofFindingCode::ReplayIdempotencyKey
    ));
    assert!(proof_has_finding(
        &decision.proof_review,
        ProofFindingCode::ReplayNonce
    ));
    assert!(!decision.permits_transaction_simulation());
}

#[test]
fn private_roc_to_rox_mint_and_token_account_mismatches_block_rpc_quorum() {
    let mut fixture = fixture();
    fixture.observations[0].mint = MintId::new("WrongPrivatePilotRoxMint111111111").unwrap();
    fixture.observations[1].token_account =
        TokenAccountId::new("WrongPrivatePilotTokenAccount11111").unwrap();

    let decision = review_coordinator_request(
        &request(&fixture, ReplaySet::default()),
        CoordinatorConfig::new(2, 100, 8),
        fixture.current_slot,
    );

    assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
    assert!(decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::MintMismatch));
    assert!(decision
        .rpc_review
        .has_finding(RpcQuorumFindingCode::TokenAccountMismatch));
    assert!(!decision.permits_transaction_simulation());
}

#[test]
fn halted_private_roc_to_rox_proof_does_not_permit_simulation() {
    let mut fixture = fixture();
    fixture.package.halt_posture = HaltPosture::Halted;

    let decision = review_coordinator_request(
        &request(&fixture, ReplaySet::default()),
        CoordinatorConfig::new(2, 100, 8),
        fixture.current_slot,
    );

    assert_eq!(decision.status, CoordinatorDecisionStatus::BlockedProof);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Blocked);
    assert!(!decision.permits_transaction_simulation());
}
