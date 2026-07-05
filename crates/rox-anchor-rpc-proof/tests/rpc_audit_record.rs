// RO:WHAT — Phase 11 RPC proof audit record tests.
// RO:WHY — Proves RPC quorum reports are deterministic, redacted, and safe before coordinator use.
// RO:INTERACTS — RPC observations, quorum review, signature redaction, and audit renderer.
// RO:INVARIANTS — findings and accepted counts must mirror the actual quorum review.
// RO:SECURITY — no live RPC, wallet calls, transaction submission, minting, burning, or settlement.
// RO:TEST — cargo test -p rox-anchor-rpc-proof --test rpc_audit_record.

#![forbid(unsafe_code)]

use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    review_rpc_observations, ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation,
    RpcProofAuditRecord, RpcProofConfig, RpcQuorumDecision, RpcQuorumFindingCode,
};

fn expected_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        RpcCommitmentLevel::Finalized,
    )
}

fn observation(source: &str, signature: &str, slot: u64) -> RpcObservation {
    RpcObservation::new(
        source,
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        signature,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

#[test]
fn accepted_rpc_quorum_renders_safe_redacted_audit_record() {
    let expected = expected_binding();
    let observations = vec![
        observation("audit-rpc-a", "sig-accepted-aaaaaaaaaaaa-0001", 40),
        observation("audit-rpc-b", "sig-accepted-aaaaaaaaaaaa-0001", 41),
    ];
    let review = review_rpc_observations(&observations, &expected, RpcProofConfig::new(2, 100), 50);
    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, 50);
    let report = audit.render();

    assert_eq!(review.decision, RpcQuorumDecision::Agreement);
    assert_eq!(audit.version, "rpc-proof-audit-v1");
    assert_eq!(audit.decision, "Agreement");
    assert_eq!(audit.observation_count, 2);
    assert_eq!(audit.accepted_observations, 2);
    assert_eq!(audit.required_observations, 2);
    assert!(audit
        .findings
        .iter()
        .any(|finding| finding == "SourceAccepted"));
    assert!(audit.evidence_consistent);
    assert!(audit.is_safe_for_display());

    assert!(report.contains("audit_record=rpc-proof-audit-v1"));
    assert!(report.contains("decision=Agreement"));
    assert_eq!(audit.findings.len(), 1);
    assert_eq!(audit.findings[0], "SourceAccepted");
    assert!(report.contains("findings=SourceAccepted"));
    assert!(!report.contains("findings=SourceAccepted,SourceAccepted"));
    assert!(report.contains("observation.0.source=audit-rpc-a"));
    assert!(report.contains("observation.1.source=audit-rpc-b"));
    assert!(report.contains("observation.0.signature=sig-acce...0001"));
    assert!(!report.contains("sig-accepted-aaaaaaaaaaaa-0001"));
    assert!(report.contains("evidence_consistent=true"));
    assert!(report.contains("display_safe=true"));
}

#[test]
fn disputed_rpc_quorum_renders_auditable_non_agreement_record() {
    let expected = expected_binding();
    let observations = vec![
        observation("audit-rpc-a", "sig-left-aaaaaaaaaaaa-0001", 40),
        observation("audit-rpc-b", "sig-right-bbbbbbbbbbbb-0002", 41),
    ];
    let review = review_rpc_observations(&observations, &expected, RpcProofConfig::new(2, 100), 50);
    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, 50);
    let report = audit.render();

    assert_eq!(review.decision, RpcQuorumDecision::Disputed);
    assert!(review.has_finding(RpcQuorumFindingCode::RpcEquivocation));
    assert_eq!(audit.decision, "Disputed");
    assert!(audit
        .findings
        .iter()
        .any(|finding| finding == "RpcEquivocation"));
    assert!(audit.evidence_consistent);
    assert!(audit.is_safe_for_display());

    assert!(report.contains("decision=Disputed"));
    assert!(report.contains("RpcEquivocation"));
    assert!(report.contains("display_safe=true"));
    assert!(!report.contains("sig-left-aaaaaaaaaaaa-0001"));
    assert!(!report.contains("sig-right-bbbbbbbbbbbb-0002"));
}

#[test]
fn rejected_rpc_quorum_records_binding_mismatch_findings() {
    let expected = expected_binding();
    let mut observations = vec![
        observation("audit-rpc-a", "sig-rejected-aaaaaaaaaaaa-0001", 40),
        observation("audit-rpc-b", "sig-rejected-aaaaaaaaaaaa-0001", 41),
    ];

    observations[0].token_account =
        TokenAccountId::new("WrongTokenAccount1111111111111111111").unwrap();

    let review = review_rpc_observations(&observations, &expected, RpcProofConfig::new(2, 100), 50);
    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, 50);
    let report = audit.render();

    assert_eq!(review.decision, RpcQuorumDecision::Rejected);
    assert!(review.has_finding(RpcQuorumFindingCode::TokenAccountMismatch));
    assert_eq!(audit.decision, "Rejected");
    assert!(audit
        .findings
        .iter()
        .any(|finding| finding == "TokenAccountMismatch"));
    assert!(audit.evidence_consistent);
    assert!(audit.is_safe_for_display());

    assert!(report.contains("decision=Rejected"));
    assert!(report.contains("TokenAccountMismatch"));
    assert!(report.contains("display_safe=true"));
}

#[test]
fn audit_record_exposes_inconsistent_review_counts_as_not_display_safe() {
    let expected = expected_binding();
    let observations = vec![observation(
        "audit-rpc-a",
        "sig-count-tamper-aaaaaaaaaaaa-0001",
        40,
    )];

    let mut review =
        review_rpc_observations(&observations, &expected, RpcProofConfig::new(1, 100), 50);
    review.accepted_observations = 2;

    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, 50);
    let report = audit.render();

    assert!(!audit.evidence_consistent);
    assert!(!audit.is_safe_for_display());
    assert!(report.contains("accepted_observations=2"));
    assert!(report.contains("observation_count=1"));
    assert!(report.contains("evidence_consistent=false"));
    assert!(report.contains("display_safe=false"));
}

#[test]
fn audit_record_marks_sensitive_observation_values_unsafe_for_display() {
    let expected = expected_binding();
    let observations = vec![observation(
        "secret-wallet-rpc-source",
        "sig-sensitive-aaaaaaaaaaaa-0001",
        40,
    )];
    let review = review_rpc_observations(&observations, &expected, RpcProofConfig::new(1, 100), 50);
    let audit = RpcProofAuditRecord::from_review(&expected, &observations, &review, 50);
    let report = audit.render();

    assert!(audit.evidence_consistent);
    assert!(!audit.is_safe_for_display());
    assert!(report.contains("observation.0.source=secret-wallet-rpc-source"));
    assert!(report.contains("display_safe=false"));
}
