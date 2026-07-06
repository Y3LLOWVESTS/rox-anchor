//! RO:WHAT — Tests BUILD_PLAN3 Phase 13 private ROX-to-ROC pilot coordinator path.
//! RO:WHY — Proves test ROX burn evidence can produce only a dry-run internal ROC release intent.
//! RO:INTERACTS — core dry-run release intent, RPC quorum evidence, coordinator decision, proof validation.
//! RO:INVARIANTS — operation ID, idempotency key, nonce, mint, token account, and RPC evidence must match.
//! RO:SECURITY — no real ROC release, svc-wallet call, ron-ledger mutation, paid unlock, or settlement.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test private_rox_to_roc_pilot.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorConfig, CoordinatorDecisionStatus,
    CoordinatorInternalRocDryRunObservation, CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorCluster, AnchorDirection, AnchorEnvironmentMode,
    AnchorSafetyProfile, ChallengePosture, ClusterAllowlist, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, InternalRocDryRunReleaseIntent, MintId, Nonce, OperationId, ProgramId,
    RecoveryPosture, SubmissionMode, TokenAccountId,
};
use rox_anchor_proof::{
    EvidenceBundle, ExpectedProofBinding, ProofFindingCode, ProofPackage, ReplaySet, ReviewDecision,
};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision, RpcQuorumFindingCode,
};

#[derive(Clone, Debug)]
struct ReverseFixture {
    package: ProofPackage,
    expected: ExpectedProofBinding,
    expected_rpc: ExpectedRpcBinding,
    observations: Vec<RpcObservation>,
    release_intent: InternalRocDryRunReleaseIntent,
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
        DomainId::new("solana-devnet-rox-private-pilot-test").unwrap(),
        DomainId::new("internal-roc-private-pilot-test").unwrap(),
        AnchorDirection::RoxToRoc,
        ClusterId::new("devnet").unwrap(),
        ProgramId::new("PrivatePilotRoxAnchorProgram11111111").unwrap(),
        MintId::new("TestOnlyPrivatePilotRoxMint111111111").unwrap(),
        TokenAccountId::new("PrivatePilotRoxBurnSourceToken111111").unwrap(),
    )
}

fn fixture() -> ReverseFixture {
    let binding = binding();
    let operation_id = OperationId::new("private-rox-to-roc-op-0001").unwrap();
    let idempotency_key = IdempotencyKey::new("private-rox-to-roc-idem-0001").unwrap();
    let nonce = Nonce::new("private-rox-to-roc-nonce-0001").unwrap();
    let release_account = AccountId::new("crablink-private-roc-release-target-0001").unwrap();

    let package = ProofPackage::new(
        binding.clone(),
        operation_id.clone(),
        idempotency_key.clone(),
        nonce.clone(),
        AccountId::new("private-rox-burn-source-0001").unwrap(),
        release_account.clone(),
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
        observation("pilot-rpc-a", &binding, &operation_id, 220),
        observation("pilot-rpc-b", &binding, &operation_id, 221),
    ];

    let release_intent = InternalRocDryRunReleaseIntent::new(
        safety(),
        operation_id,
        idempotency_key,
        nonce,
        release_account,
        "test-only-private-rox-to-roc-release-intent",
        10,
    )
    .expect("static private ROX-to-ROC release intent should validate");

    ReverseFixture {
        package,
        expected,
        expected_rpc,
        observations,
        release_intent,
        current_slot: 225,
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
        "private-rox-to-roc-burn-signature-same-0001",
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn proof_has_finding(review: &rox_anchor_proof::ProofReview, code: ProofFindingCode) -> bool {
    review.findings.iter().any(|finding| finding.code == code)
}

fn request(fixture: &ReverseFixture, replay: ReplaySet) -> CoordinatorReviewRequest {
    CoordinatorReviewRequest::new(
        fixture.package.clone(),
        fixture.expected.clone(),
        fixture.expected_rpc.clone(),
        fixture.observations.clone(),
        replay,
    )
}

#[test]
fn private_rox_to_roc_release_intent_reaches_accepted_coordinator_review() {
    let fixture = fixture();

    let release_observation =
        CoordinatorInternalRocDryRunObservation::from_release_intent(&fixture.release_intent)
            .expect("release intent should become coordinator dry-run observation");
    let release_report = release_observation.redacted_report();

    assert!(release_report.contains("coordinator_internal_roc_dry_run_observation: accepted"));
    assert!(release_report.contains("kind: release_intent_output"));
    assert!(release_report.contains("real_internal_roc_release: disabled"));
    assert!(release_report.contains("future_real_roc_path: svc-wallet -> ron-ledger only"));
    assert!(release_report.contains("coordinator_wallet_call: disabled"));
    assert!(release_report.contains("coordinator_ron_ledger_mutation: disabled"));
    assert!(release_report.contains("paid_content_unlock: disabled"));
    assert!(release_report.contains("settlement_claim: none"));

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
fn replayed_private_rox_to_roc_ids_are_rejected_before_release_intent_use() {
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
fn private_rox_to_roc_burn_mint_and_token_account_mismatches_block_rpc_quorum() {
    let mut fixture = fixture();
    fixture.observations[0].mint = MintId::new("WrongPrivatePilotRoxBurnMint111111").unwrap();
    fixture.observations[1].token_account =
        TokenAccountId::new("WrongPrivatePilotBurnToken111111").unwrap();

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
fn halted_private_rox_to_roc_proof_does_not_permit_release_simulation() {
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
