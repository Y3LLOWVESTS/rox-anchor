//! RO:WHAT — Main deterministic proof-review logic for ROX Anchor.
//! RO:WHY — Combines binding, replay, evidence, challenge, halt, and recovery checks into one review.
//! RO:INTERACTS — package, replay, quorum, challenge, recovery, and rox-anchor-core lifecycle states.
//! RO:INVARIANTS — reject mismatches/replay; block unsafe evidence/challenge/halt/recovery states.
//! RO:SECURITY — returns local decisions only; never claims bridge settlement or performs value movement.
//! RO:TEST — covered by crate-local proof review tests.

use rox_anchor_core::AnchorLifecycleState;

use crate::{
    classify_quorum, review_challenge, review_halt, review_recovery, ChallengeReview,
    ExpectedProofBinding, HaltReview, ProofPackage, QuorumPosture, RecoveryReview, ReplaySet,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewDecision {
    Accepted,
    Blocked,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofFindingSeverity {
    Info,
    Block,
    Reject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofFindingCode {
    PackageAccepted,
    SourceDomainMismatch,
    TargetDomainMismatch,
    DirectionMismatch,
    ClusterMismatch,
    ProgramIdMismatch,
    MintMismatch,
    TokenAccountMismatch,
    OperationIdMismatch,
    IdempotencyKeyMismatch,
    NonceMismatch,
    ReplayOperationId,
    ReplayIdempotencyKey,
    ReplayNonce,
    EvidenceMissing,
    QuorumDisputed,
    ChallengeOpen,
    ChallengeAccepted,
    HaltRequested,
    Halted,
    RecoveryRequired,
    RecoveryInReview,
    RecoveryRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProofFinding {
    pub code: ProofFindingCode,
    pub severity: ProofFindingSeverity,
}

impl ProofFinding {
    pub fn info(code: ProofFindingCode) -> Self {
        Self {
            code,
            severity: ProofFindingSeverity::Info,
        }
    }

    pub fn block(code: ProofFindingCode) -> Self {
        Self {
            code,
            severity: ProofFindingSeverity::Block,
        }
    }

    pub fn reject(code: ProofFindingCode) -> Self {
        Self {
            code,
            severity: ProofFindingSeverity::Reject,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofReview {
    pub decision: ReviewDecision,
    pub lifecycle_state: AnchorLifecycleState,
    pub findings: Vec<ProofFinding>,
}

pub fn review_proof_package(
    package: &ProofPackage,
    expected: &ExpectedProofBinding,
    replay: &ReplaySet,
) -> ProofReview {
    let mut findings = Vec::new();

    push_binding_findings(package, expected, &mut findings);
    push_replay_findings(package, replay, &mut findings);

    if findings
        .iter()
        .any(|finding| finding.severity == ProofFindingSeverity::Reject)
    {
        return ProofReview {
            decision: ReviewDecision::Rejected,
            lifecycle_state: AnchorLifecycleState::Failed,
            findings,
        };
    }

    push_quorum_findings(package, &mut findings);
    push_challenge_findings(package, &mut findings);
    push_halt_findings(package, &mut findings);
    push_recovery_findings(package, &mut findings);

    if findings
        .iter()
        .any(|finding| finding.severity == ProofFindingSeverity::Block)
    {
        let lifecycle_state = lifecycle_for_first_blocker(&findings);
        return ProofReview {
            decision: ReviewDecision::Blocked,
            lifecycle_state,
            findings,
        };
    }

    findings.push(ProofFinding::info(ProofFindingCode::PackageAccepted));

    ProofReview {
        decision: ReviewDecision::Accepted,
        lifecycle_state: AnchorLifecycleState::FinalityEligible,
        findings,
    }
}

fn push_binding_findings(
    package: &ProofPackage,
    expected: &ExpectedProofBinding,
    findings: &mut Vec<ProofFinding>,
) {
    if package.binding.source_domain != expected.binding.source_domain {
        findings.push(ProofFinding::reject(ProofFindingCode::SourceDomainMismatch));
    }

    if package.binding.target_domain != expected.binding.target_domain {
        findings.push(ProofFinding::reject(ProofFindingCode::TargetDomainMismatch));
    }

    if package.binding.direction != expected.binding.direction {
        findings.push(ProofFinding::reject(ProofFindingCode::DirectionMismatch));
    }

    if package.binding.cluster != expected.binding.cluster {
        findings.push(ProofFinding::reject(ProofFindingCode::ClusterMismatch));
    }

    if package.binding.program_id != expected.binding.program_id {
        findings.push(ProofFinding::reject(ProofFindingCode::ProgramIdMismatch));
    }

    if package.binding.mint != expected.binding.mint {
        findings.push(ProofFinding::reject(ProofFindingCode::MintMismatch));
    }

    if package.binding.token_account != expected.binding.token_account {
        findings.push(ProofFinding::reject(ProofFindingCode::TokenAccountMismatch));
    }

    if package.operation_id != expected.operation_id {
        findings.push(ProofFinding::reject(ProofFindingCode::OperationIdMismatch));
    }

    if package.idempotency_key != expected.idempotency_key {
        findings.push(ProofFinding::reject(
            ProofFindingCode::IdempotencyKeyMismatch,
        ));
    }

    if package.nonce != expected.nonce {
        findings.push(ProofFinding::reject(ProofFindingCode::NonceMismatch));
    }
}

fn push_replay_findings(
    package: &ProofPackage,
    replay: &ReplaySet,
    findings: &mut Vec<ProofFinding>,
) {
    if replay.contains_operation_id(&package.operation_id) {
        findings.push(ProofFinding::reject(ProofFindingCode::ReplayOperationId));
    }

    if replay.contains_idempotency_key(&package.idempotency_key) {
        findings.push(ProofFinding::reject(ProofFindingCode::ReplayIdempotencyKey));
    }

    if replay.contains_nonce(&package.nonce) {
        findings.push(ProofFinding::reject(ProofFindingCode::ReplayNonce));
    }
}

fn push_quorum_findings(package: &ProofPackage, findings: &mut Vec<ProofFinding>) {
    match classify_quorum(package.evidence) {
        QuorumPosture::Satisfied => {}
        QuorumPosture::MissingEvidence => {
            findings.push(ProofFinding::block(ProofFindingCode::EvidenceMissing));
        }
        QuorumPosture::Disputed => {
            findings.push(ProofFinding::block(ProofFindingCode::QuorumDisputed));
        }
    }
}

fn push_challenge_findings(package: &ProofPackage, findings: &mut Vec<ProofFinding>) {
    match review_challenge(package.challenge_posture) {
        ChallengeReview::Clear => {}
        ChallengeReview::Open => {
            findings.push(ProofFinding::block(ProofFindingCode::ChallengeOpen));
        }
        ChallengeReview::Accepted => {
            findings.push(ProofFinding::block(ProofFindingCode::ChallengeAccepted));
        }
    }
}

fn push_halt_findings(package: &ProofPackage, findings: &mut Vec<ProofFinding>) {
    match review_halt(package.halt_posture) {
        HaltReview::Active => {}
        HaltReview::HaltRequested => {
            findings.push(ProofFinding::block(ProofFindingCode::HaltRequested));
        }
        HaltReview::Halted => {
            findings.push(ProofFinding::block(ProofFindingCode::Halted));
        }
    }
}

fn push_recovery_findings(package: &ProofPackage, findings: &mut Vec<ProofFinding>) {
    match review_recovery(package.recovery_posture) {
        RecoveryReview::Clear => {}
        RecoveryReview::Required => {
            findings.push(ProofFinding::block(ProofFindingCode::RecoveryRequired));
        }
        RecoveryReview::InReview => {
            findings.push(ProofFinding::block(ProofFindingCode::RecoveryInReview));
        }
        RecoveryReview::Rejected => {
            findings.push(ProofFinding::block(ProofFindingCode::RecoveryRejected));
        }
    }
}

fn lifecycle_for_first_blocker(findings: &[ProofFinding]) -> AnchorLifecycleState {
    for finding in findings {
        match finding.code {
            ProofFindingCode::EvidenceMissing => return AnchorLifecycleState::EvidenceIncomplete,
            ProofFindingCode::QuorumDisputed => return AnchorLifecycleState::QuorumDisputed,
            ProofFindingCode::ChallengeOpen => return AnchorLifecycleState::ChallengeOpen,
            ProofFindingCode::ChallengeAccepted => return AnchorLifecycleState::ChallengeAccepted,
            ProofFindingCode::HaltRequested => return AnchorLifecycleState::HaltRequested,
            ProofFindingCode::Halted => return AnchorLifecycleState::Halted,
            ProofFindingCode::RecoveryRequired => return AnchorLifecycleState::RecoveryRequired,
            ProofFindingCode::RecoveryInReview | ProofFindingCode::RecoveryRejected => {
                return AnchorLifecycleState::RecoveryInReview;
            }
            _ => {}
        }
    }

    AnchorLifecycleState::EvidenceIncomplete
}
