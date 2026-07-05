// RO:WHAT — Phase 11 coordinator audit record tests.
// RO:WHY — Proves coordinator reports include RPC quorum, proof decision, findings, and simulation permission.
// RO:INTERACTS — coordinator decision, proof fixtures, RPC quorum, and audit renderer.
// RO:INVARIANTS — report status must match RPC/proof outcome before it is display-safe.
// RO:SECURITY — no live RPC, wallet calls, transaction submission, minting, burning, or settlement.
// RO:TEST — cargo test -p rox-anchor-coordinator --test coordinator_audit_record.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::{
    review_coordinator_request, CoordinatorAuditRecord, CoordinatorConfig,
    CoordinatorDecisionStatus, CoordinatorReviewRequest,
};
use rox_anchor_core::TokenAccountId;
use rox_anchor_proof::{fixtures, ReplaySet, ReviewDecision};
use rox_anchor_rpc_proof::{
    ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation, RpcQuorumDecision,
};

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
            "audit-rpc-a",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "audit-same-signature-0001",
            100,
            RpcCommitmentLevel::Finalized,
        ),
        RpcObservation::new(
            "audit-rpc-b",
            expected.cluster.clone(),
            expected.program_id.clone(),
            expected.mint.clone(),
            expected.token_account.clone(),
            expected.operation_id.clone(),
            "audit-same-signature-0001",
            100,
            RpcCommitmentLevel::Finalized,
        ),
    ]
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

#[test]
fn accepted_coordinator_review_renders_safe_deterministic_audit_record() {
    let request = accepted_request();
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);
    let audit = CoordinatorAuditRecord::from_review(&request, &decision, 100);
    let report = audit.render();

    assert_eq!(audit.version, "coordinator-testnet-audit-v1");
    assert_eq!(decision.status, CoordinatorDecisionStatus::Accepted);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Accepted);
    assert_eq!(audit.rpc_decision, "Agreement");
    assert_eq!(audit.proof_decision, "Accepted");
    assert_eq!(audit.coordinator_status, "Accepted");
    assert_eq!(audit.accepted_observations, 2);
    assert_eq!(audit.required_observations, 2);
    assert_eq!(audit.observation_count, 2);
    assert!(audit
        .rpc_findings
        .iter()
        .any(|finding| finding == "SourceAccepted"));
    assert!(audit
        .proof_findings
        .iter()
        .any(|finding| finding == "Info:PackageAccepted"));
    assert!(audit.permits_simulation);
    assert!(audit.status_consistent);
    assert!(audit.is_safe_for_display());

    assert!(report.contains("audit_record=coordinator-testnet-audit-v1"));
    assert!(report.contains("rpc_decision=Agreement"));
    assert!(report.contains("proof_decision=Accepted"));
    assert!(report.contains("coordinator_status=Accepted"));
    assert!(report.contains("permits_simulation=true"));
    assert!(report.contains("status_consistent=true"));
    assert!(report.contains("display_safe=true"));
}

#[test]
fn rejected_rpc_evidence_renders_auditable_non_simulating_record() {
    let mut request = accepted_request();

    request.observations[0].token_account =
        TokenAccountId::new("audit-wrong-token-account-0001").unwrap();

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);
    let audit = CoordinatorAuditRecord::from_review(&request, &decision, 100);
    let report = audit.render();

    assert_eq!(decision.status, CoordinatorDecisionStatus::RejectedEvidence);
    assert_eq!(decision.rpc_review.decision, RpcQuorumDecision::Rejected);
    assert_eq!(decision.proof_review.decision, ReviewDecision::Blocked);
    assert_eq!(audit.rpc_decision, "Rejected");
    assert_eq!(audit.proof_decision, "Blocked");
    assert_eq!(audit.coordinator_status, "RejectedEvidence");
    assert!(audit
        .rpc_findings
        .iter()
        .any(|finding| finding == "TokenAccountMismatch"));
    assert!(audit
        .proof_findings
        .iter()
        .any(|finding| finding == "Block:QuorumDisputed"));
    assert!(!audit.permits_simulation);
    assert!(audit.status_consistent);
    assert!(audit.is_safe_for_display());

    assert!(report.contains("rpc_findings=TokenAccountMismatch"));
    assert!(report.contains("proof_findings=Block:QuorumDisputed"));
    assert!(report.contains("permits_simulation=false"));
    assert!(report.contains("display_safe=true"));
}

#[test]
fn audit_record_exposes_coordinator_status_tamper_as_not_display_safe() {
    let request = accepted_request();
    let mut decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);

    decision.status = CoordinatorDecisionStatus::Accepted;
    decision.proof_review.decision = ReviewDecision::Rejected;

    let audit = CoordinatorAuditRecord::from_review(&request, &decision, 100);
    let report = audit.render();

    assert_eq!(audit.coordinator_status, "Accepted");
    assert_eq!(audit.proof_decision, "Rejected");
    assert!(audit.permits_simulation);
    assert!(!audit.status_consistent);
    assert!(!audit.is_safe_for_display());
    assert!(report.contains("status_consistent=false"));
    assert!(report.contains("display_safe=false"));
}

#[test]
fn audit_record_marks_sensitive_binding_values_unsafe_for_display() {
    let mut request = accepted_request();

    request.package.binding.token_account =
        TokenAccountId::new("local-secret-wallet-token-account-0001").unwrap();

    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);
    let audit = CoordinatorAuditRecord::from_review(&request, &decision, 100);
    let report = audit.render();

    assert!(audit.status_consistent);
    assert!(!audit.is_safe_for_display());
    assert!(report.contains("token_account=local-secret-wallet-token-account-0001"));
    assert!(report.contains("display_safe=false"));
}

#[test]
fn audit_record_allows_normal_public_token_binding_terms() {
    let request = accepted_request();
    let decision = review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 8), 100);
    let audit = CoordinatorAuditRecord::from_review(&request, &decision, 100);
    let report = audit.render();

    assert!(audit.status_consistent);
    assert!(audit.is_safe_for_display());
    assert!(report.contains("token_account="));
    assert!(report.contains("display_safe=true"));
}
