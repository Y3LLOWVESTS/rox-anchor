// RO:WHAT — Deterministic local proof package review for the rox-anchor-proof validator.
// RO:WHY — Aggregates missing-field, operation identity, nonce, replay, domain, quorum, challenge, halt, and recovery checks without finality or runtime authority.
// RO:INTERACTS — package, replay, quorum, challenge, and recovery local review modules.
// RO:INVARIANTS — Local review can return ValidForLocalReviewOnly, never finality, settlement, bridge completion, or value movement.
// RO:SECURITY — No RPC, wallet, Solana/Anchor runtime, bridge runtime, deployment, settlement, staking, liquidity, or external settlement.
// RO:TEST — Static Phase 4 checker only for this round.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local validator does not authorize runtime.

use crate::challenge::{review_challenge_posture, ChallengeReviewFinding};
use crate::package::{
    CommitmentReviewLevel, EvidencePosture, OperationIdentityField, ProofPackageShape,
    RequiredProofField,
};
use crate::quorum::{review_quorum_posture, QuorumReviewFinding};
use crate::recovery::{review_halt_posture, review_recovery_posture, RecoveryReviewFinding};
use crate::replay::{
    review_operation_identity_for_local_review_only, ExpectedProofBinding, NonceReviewFinding,
    OperationIdentityReviewFinding, ReplayPosture,
};

/// Conservative local proof-review findings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofReviewFinding {
    ValidForLocalReviewOnly,
    MissingRequiredField,
    MissingSchemaVersion,
    MissingSourceDomain,
    MissingTargetDomain,
    MissingDirection,
    MissingOperationId,
    MissingIdempotencyKey,
    MissingNonce,
    MissingCluster,
    MissingProgramId,
    MissingMint,
    MissingTokenAccount,
    MissingCommitmentLevel,
    MissingChallengeStatus,
    MissingHaltStatus,
    MissingRecoveryStatus,
    OperationIdentityIncomplete,
    OperationIdentityMismatch,
    IdempotencyKeyAuthorityMisuse,
    ReusedNonce,
    NonceAcceptedForLocalReviewOnly,
    EvidenceIncomplete,
    ReplayRejected,
    DomainMismatch,
    DirectionMismatch,
    ClusterMismatch,
    ProgramMismatch,
    MintMismatch,
    TokenAccountMismatch,
    NonceMismatch,
    CommitmentInsufficient,
    QuorumDisputed,
    ChallengeOpen,
    Halted,
    RecoveryReviewRequired,
    RuntimeNotAuthorized,
}

/// Local-only review decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalProofReviewDecision {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    RuntimeNotAuthorized,
}

/// Deterministic local proof-review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofReview {
    pub decision: LocalProofReviewDecision,
    pub findings: Vec<ProofReviewFinding>,
}

pub type ProofReviewSkeleton = ProofReview;

impl ProofReview {
    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn has_finding(&self, finding: ProofReviewFinding) -> bool {
        self.findings.contains(&finding)
    }
}

pub fn review_required_fields(package: &ProofPackageShape) -> Vec<ProofReviewFinding> {
    package
        .missing_required_fields()
        .into_iter()
        .map(map_required_field)
        .collect()
}

pub fn review_package_for_local_review_only(
    package: &ProofPackageShape,
    expected: &ExpectedProofBinding,
) -> ProofReview {
    review_package_with_seen_nonces_for_local_review_only(package, expected, &[])
}

pub fn review_package_with_seen_nonces_for_local_review_only(
    package: &ProofPackageShape,
    expected: &ExpectedProofBinding,
    previously_seen_nonces: &[&str],
) -> ProofReview {
    let mut findings = review_required_fields(package);

    let identity_review =
        review_operation_identity_for_local_review_only(package, expected, previously_seen_nonces);

    map_identity_missing_fields(&identity_review.missing_fields, &mut findings);

    match identity_review.finding {
        OperationIdentityReviewFinding::CompleteForLocalReviewOnly => {}
        OperationIdentityReviewFinding::OperationIdentityIncomplete
        | OperationIdentityReviewFinding::MissingIdentityField => {
            push_unique(&mut findings, ProofReviewFinding::OperationIdentityIncomplete)
        }
        OperationIdentityReviewFinding::OperationIdentityMismatch => {
            push_unique(&mut findings, ProofReviewFinding::OperationIdentityMismatch)
        }
        OperationIdentityReviewFinding::IdempotencyKeyAuthorityMisuse => {
            push_unique(&mut findings, ProofReviewFinding::IdempotencyKeyAuthorityMisuse)
        }
        OperationIdentityReviewFinding::ReusedNonce => {
            push_unique(&mut findings, ProofReviewFinding::ReusedNonce)
        }
        OperationIdentityReviewFinding::MissingSourceDomain => {
            push_unique(&mut findings, ProofReviewFinding::MissingSourceDomain)
        }
        OperationIdentityReviewFinding::MissingTargetDomain => {
            push_unique(&mut findings, ProofReviewFinding::MissingTargetDomain)
        }
        OperationIdentityReviewFinding::MissingDirection => {
            push_unique(&mut findings, ProofReviewFinding::MissingDirection)
        }
        OperationIdentityReviewFinding::MissingOperationId => {
            push_unique(&mut findings, ProofReviewFinding::MissingOperationId)
        }
        OperationIdentityReviewFinding::MissingIdempotencyKey => {
            push_unique(&mut findings, ProofReviewFinding::MissingIdempotencyKey)
        }
        OperationIdentityReviewFinding::MissingNonce => {
            push_unique(&mut findings, ProofReviewFinding::MissingNonce)
        }
        OperationIdentityReviewFinding::MissingCluster => {
            push_unique(&mut findings, ProofReviewFinding::MissingCluster)
        }
        OperationIdentityReviewFinding::MissingProgramId => {
            push_unique(&mut findings, ProofReviewFinding::MissingProgramId)
        }
        OperationIdentityReviewFinding::MissingMint => {
            push_unique(&mut findings, ProofReviewFinding::MissingMint)
        }
        OperationIdentityReviewFinding::MissingTokenAccount => {
            push_unique(&mut findings, ProofReviewFinding::MissingTokenAccount)
        }
    }

    match identity_review.nonce_review.finding {
        NonceReviewFinding::MissingNonce => {
            push_unique(&mut findings, ProofReviewFinding::MissingNonce)
        }
        NonceReviewFinding::ReusedNonce => {
            push_unique(&mut findings, ProofReviewFinding::ReusedNonce)
        }
        NonceReviewFinding::NonceAcceptedForLocalReviewOnly => {}
    }

    for mismatch in identity_review.replay_review.mismatches {
        match mismatch {
            ReplayPosture::ReplayRejected => {
                push_unique(&mut findings, ProofReviewFinding::ReplayRejected)
            }
            ReplayPosture::DomainMismatch => {
                push_unique(&mut findings, ProofReviewFinding::DomainMismatch)
            }
            ReplayPosture::DirectionMismatch => {
                push_unique(&mut findings, ProofReviewFinding::DirectionMismatch)
            }
            ReplayPosture::ClusterMismatch => {
                push_unique(&mut findings, ProofReviewFinding::ClusterMismatch)
            }
            ReplayPosture::ProgramMismatch => {
                push_unique(&mut findings, ProofReviewFinding::ProgramMismatch)
            }
            ReplayPosture::MintMismatch => {
                push_unique(&mut findings, ProofReviewFinding::MintMismatch)
            }
            ReplayPosture::TokenAccountMismatch => {
                push_unique(&mut findings, ProofReviewFinding::TokenAccountMismatch)
            }
            ReplayPosture::NonceMismatch => {
                push_unique(&mut findings, ProofReviewFinding::NonceMismatch)
            }
            ReplayPosture::Unchecked
            | ReplayPosture::MissingBinding
            | ReplayPosture::BoundForLocalReviewOnly => {}
        }
    }

    review_evidence_posture(package.evidence_posture, &mut findings);
    review_commitment_level(package.commitment_level, &mut findings);

    if let Some(finding) = review_quorum_posture(package.quorum_posture).finding {
        match finding {
            QuorumReviewFinding::EvidenceIncomplete => {
                push_unique(&mut findings, ProofReviewFinding::EvidenceIncomplete)
            }
            QuorumReviewFinding::QuorumDisputed => {
                push_unique(&mut findings, ProofReviewFinding::QuorumDisputed)
            }
        }
    }

    if let Some(finding) = review_challenge_posture(package.challenge_status).finding {
        match finding {
            ChallengeReviewFinding::EvidenceIncomplete => {
                push_unique(&mut findings, ProofReviewFinding::EvidenceIncomplete)
            }
            ChallengeReviewFinding::ChallengeOpen => {
                push_unique(&mut findings, ProofReviewFinding::ChallengeOpen)
            }
            ChallengeReviewFinding::QuorumDisputed => {
                push_unique(&mut findings, ProofReviewFinding::QuorumDisputed)
            }
            ChallengeReviewFinding::ReplayRejected => {
                push_unique(&mut findings, ProofReviewFinding::ReplayRejected)
            }
            ChallengeReviewFinding::Halted => {
                push_unique(&mut findings, ProofReviewFinding::Halted)
            }
        }
    }

    if let Some(finding) = review_halt_posture(package.halt_status) {
        map_recovery_finding(finding, &mut findings);
    }

    if let Some(finding) = review_recovery_posture(package.recovery_status) {
        map_recovery_finding(finding, &mut findings);
    }

    if package.is_runtime_authorized() {
        push_unique(&mut findings, ProofReviewFinding::RuntimeNotAuthorized);
    }

    remove_success_only_markers_when_any_problem_exists(&mut findings);

    let decision = decision_for_findings(&findings);

    ProofReview { decision, findings }
}

fn map_identity_missing_fields(
    missing_fields: &[OperationIdentityField],
    findings: &mut Vec<ProofReviewFinding>,
) {
    for field in missing_fields {
        match field {
            OperationIdentityField::SourceDomain => {
                push_unique(findings, ProofReviewFinding::MissingSourceDomain)
            }
            OperationIdentityField::TargetDomain => {
                push_unique(findings, ProofReviewFinding::MissingTargetDomain)
            }
            OperationIdentityField::Direction => {
                push_unique(findings, ProofReviewFinding::MissingDirection)
            }
            OperationIdentityField::OperationId => {
                push_unique(findings, ProofReviewFinding::MissingOperationId)
            }
            OperationIdentityField::IdempotencyKey => {
                push_unique(findings, ProofReviewFinding::MissingIdempotencyKey)
            }
            OperationIdentityField::Nonce => {
                push_unique(findings, ProofReviewFinding::MissingNonce)
            }
            OperationIdentityField::Cluster => {
                push_unique(findings, ProofReviewFinding::MissingCluster)
            }
            OperationIdentityField::ProgramId => {
                push_unique(findings, ProofReviewFinding::MissingProgramId)
            }
            OperationIdentityField::Mint => {
                push_unique(findings, ProofReviewFinding::MissingMint)
            }
            OperationIdentityField::TokenAccount => {
                push_unique(findings, ProofReviewFinding::MissingTokenAccount)
            }
        }
    }
}

fn review_evidence_posture(posture: EvidencePosture, findings: &mut Vec<ProofReviewFinding>) {
    match posture {
        EvidencePosture::Draft | EvidencePosture::Incomplete => {
            push_unique(findings, ProofReviewFinding::EvidenceIncomplete)
        }
        EvidencePosture::QuorumDisputed => {
            push_unique(findings, ProofReviewFinding::QuorumDisputed)
        }
        EvidencePosture::ChallengeOpen => {
            push_unique(findings, ProofReviewFinding::ChallengeOpen)
        }
        EvidencePosture::ChallengeAccepted | EvidencePosture::FailedClosed => {
            push_unique(findings, ProofReviewFinding::ReplayRejected)
        }
        EvidencePosture::Halted => push_unique(findings, ProofReviewFinding::Halted),
        EvidencePosture::RecoveryReviewRequired => {
            push_unique(findings, ProofReviewFinding::RecoveryReviewRequired)
        }
        EvidencePosture::ChallengeRejected | EvidencePosture::ConsistentForLocalReviewOnly => {}
    }
}

fn review_commitment_level(
    level: CommitmentReviewLevel,
    findings: &mut Vec<ProofReviewFinding>,
) {
    match level {
        CommitmentReviewLevel::Missing | CommitmentReviewLevel::Insufficient => {
            push_unique(findings, ProofReviewFinding::CommitmentInsufficient)
        }
        CommitmentReviewLevel::ReviewOnly => {}
    }
}

fn map_recovery_finding(
    finding: RecoveryReviewFinding,
    findings: &mut Vec<ProofReviewFinding>,
) {
    match finding {
        RecoveryReviewFinding::EvidenceIncomplete => {
            push_unique(findings, ProofReviewFinding::EvidenceIncomplete)
        }
        RecoveryReviewFinding::Halted => push_unique(findings, ProofReviewFinding::Halted),
        RecoveryReviewFinding::RecoveryReviewRequired => {
            push_unique(findings, ProofReviewFinding::RecoveryReviewRequired)
        }
    }
}

fn map_required_field(field: RequiredProofField) -> ProofReviewFinding {
    match field {
        RequiredProofField::SchemaVersion => ProofReviewFinding::MissingSchemaVersion,
        RequiredProofField::SourceDomain => ProofReviewFinding::MissingSourceDomain,
        RequiredProofField::TargetDomain => ProofReviewFinding::MissingTargetDomain,
        RequiredProofField::Direction => ProofReviewFinding::MissingDirection,
        RequiredProofField::OperationId => ProofReviewFinding::MissingOperationId,
        RequiredProofField::IdempotencyKey => ProofReviewFinding::MissingIdempotencyKey,
        RequiredProofField::Nonce => ProofReviewFinding::MissingNonce,
        RequiredProofField::Cluster => ProofReviewFinding::MissingCluster,
        RequiredProofField::ProgramId => ProofReviewFinding::MissingProgramId,
        RequiredProofField::Mint => ProofReviewFinding::MissingMint,
        RequiredProofField::TokenAccount => ProofReviewFinding::MissingTokenAccount,
        RequiredProofField::CommitmentLevel => ProofReviewFinding::MissingCommitmentLevel,
        RequiredProofField::ChallengeStatus => ProofReviewFinding::MissingChallengeStatus,
        RequiredProofField::HaltStatus => ProofReviewFinding::MissingHaltStatus,
        RequiredProofField::RecoveryStatus => ProofReviewFinding::MissingRecoveryStatus,
    }
}

fn decision_for_findings(findings: &[ProofReviewFinding]) -> LocalProofReviewDecision {
    if findings.contains(&ProofReviewFinding::RuntimeNotAuthorized) {
        return LocalProofReviewDecision::RuntimeNotAuthorized;
    }

    if findings.iter().any(|finding| finding_is_rejection(*finding)) {
        return LocalProofReviewDecision::ReviewRejected;
    }

    if findings.is_empty() {
        LocalProofReviewDecision::ValidForLocalReviewOnly
    } else {
        LocalProofReviewDecision::EvidenceIncomplete
    }
}

fn finding_is_rejection(finding: ProofReviewFinding) -> bool {
    matches!(
        finding,
        ProofReviewFinding::ReplayRejected
            | ProofReviewFinding::DomainMismatch
            | ProofReviewFinding::DirectionMismatch
            | ProofReviewFinding::ClusterMismatch
            | ProofReviewFinding::ProgramMismatch
            | ProofReviewFinding::MintMismatch
            | ProofReviewFinding::TokenAccountMismatch
            | ProofReviewFinding::NonceMismatch
            | ProofReviewFinding::OperationIdentityMismatch
            | ProofReviewFinding::ReusedNonce
            | ProofReviewFinding::IdempotencyKeyAuthorityMisuse
    )
}

fn remove_success_only_markers_when_any_problem_exists(findings: &mut Vec<ProofReviewFinding>) {
    let has_problem = findings.iter().any(|finding| {
        !matches!(
            finding,
            ProofReviewFinding::ValidForLocalReviewOnly
                | ProofReviewFinding::NonceAcceptedForLocalReviewOnly
        )
    });

    if has_problem {
        findings.retain(|finding| *finding != ProofReviewFinding::NonceAcceptedForLocalReviewOnly);
    }
}

fn push_unique(findings: &mut Vec<ProofReviewFinding>, finding: ProofReviewFinding) {
    if !findings.contains(&finding) {
        findings.push(finding);
    }
}

// ROX-ANCHOR:PHASE4-ROUND-4-4-STATIC-VECTOR-INVENTORY
//
// Static vector inventory is dependency-free local review metadata only.
// It does not read files.
// It does not parse JSON.
// It does not authorize runtime.

/// Compile-time marker proving Round 4.4 remains static vector inventory only.
pub const PHASE4_ROUND_4_4_STATIC_VECTOR_INVENTORY_ONLY: bool = true;

/// Static fixture vector category used by local review inventory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticFixtureVectorKind {
    ProofPackage,
    Challenge,
    Recovery,
}

/// Dependency-free static inventory entry for an authorized Phase 4 vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticFixtureVectorInventoryEntry {
    pub path: &'static str,
    pub case_id: &'static str,
    pub kind: StaticFixtureVectorKind,
    pub expected_decision: &'static str,
    pub required_markers: &'static [&'static str],
}

pub const PHASE4_PROOF_PACKAGE_STATIC_VECTOR_REQUIRED_MARKERS: &[&str] = &[
    "x_rox_anchor_fixture",
    "phase4_round_4_3_static_vector",
    "x_rox_anchor_non_authorization",
    "design_only_not_runtime",
    "does not authorize runtime",
    "local-only",
    "fixture-bound",
    "non-value-bearing",
    "proof packages are evidence only",
    "local validation is not finality",
    "schema_version",
    "source_domain",
    "target_domain",
    "direction",
    "operation_id",
    "idempotency_key",
    "nonce",
    "cluster",
    "program_id",
    "mint",
    "token_account",
];

pub const PHASE4_CHALLENGE_STATIC_VECTOR_REQUIRED_MARKERS: &[&str] = &[
    "x_rox_anchor_fixture",
    "phase4_round_4_3_static_vector",
    "x_rox_anchor_non_authorization",
    "design_only_not_runtime",
    "does not authorize runtime",
    "local-only",
    "fixture-bound",
    "non-value-bearing",
    "proof packages are evidence only",
    "local validation is not finality",
    "challenge_status",
    "operation_id",
    "nonce",
];

pub const PHASE4_RECOVERY_STATIC_VECTOR_REQUIRED_MARKERS: &[&str] = &[
    "x_rox_anchor_fixture",
    "phase4_round_4_3_static_vector",
    "x_rox_anchor_non_authorization",
    "design_only_not_runtime",
    "does not authorize runtime",
    "local-only",
    "fixture-bound",
    "non-value-bearing",
    "proof packages are evidence only",
    "local validation is not finality",
    "recovery_status",
    "halt_status",
    "operation_id",
    "nonce",
];

pub const PHASE4_STATIC_FIXTURE_VECTOR_INVENTORY: &[StaticFixtureVectorInventoryEntry] = &[
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/proof-package.valid.json",
        case_id: "proof-package.valid",
        kind: StaticFixtureVectorKind::ProofPackage,
        expected_decision: "ValidForLocalReviewOnly",
        required_markers: PHASE4_PROOF_PACKAGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/proof-package.replay-rejected.json",
        case_id: "proof-package.replay-rejected",
        kind: StaticFixtureVectorKind::ProofPackage,
        expected_decision: "ReviewRejected",
        required_markers: PHASE4_PROOF_PACKAGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/proof-package.cluster-mismatch.json",
        case_id: "proof-package.cluster-mismatch",
        kind: StaticFixtureVectorKind::ProofPackage,
        expected_decision: "ReviewRejected",
        required_markers: PHASE4_PROOF_PACKAGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/proof-package.mint-mismatch.json",
        case_id: "proof-package.mint-mismatch",
        kind: StaticFixtureVectorKind::ProofPackage,
        expected_decision: "ReviewRejected",
        required_markers: PHASE4_PROOF_PACKAGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/proof-package.rpc-disagreement.json",
        case_id: "proof-package.rpc-disagreement",
        kind: StaticFixtureVectorKind::ProofPackage,
        expected_decision: "EvidenceIncomplete",
        required_markers: PHASE4_PROOF_PACKAGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/challenge.accepted.json",
        case_id: "challenge.accepted",
        kind: StaticFixtureVectorKind::Challenge,
        expected_decision: "ReviewRejected",
        required_markers: PHASE4_CHALLENGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/challenge.rejected.json",
        case_id: "challenge.rejected",
        kind: StaticFixtureVectorKind::Challenge,
        expected_decision: "ValidForLocalReviewOnly",
        required_markers: PHASE4_CHALLENGE_STATIC_VECTOR_REQUIRED_MARKERS,
    },
    StaticFixtureVectorInventoryEntry {
        path: "tests/vectors/recovery.case.valid.json",
        case_id: "recovery.case.valid",
        kind: StaticFixtureVectorKind::Recovery,
        expected_decision: "ValidForLocalReviewOnly",
        required_markers: PHASE4_RECOVERY_STATIC_VECTOR_REQUIRED_MARKERS,
    },
];

pub fn authorized_static_fixture_vector_inventory_for_local_review_only(
) -> &'static [StaticFixtureVectorInventoryEntry] {
    PHASE4_STATIC_FIXTURE_VECTOR_INVENTORY
}

pub fn find_static_fixture_vector_inventory_entry_for_local_review_only(
    path: &str,
) -> Option<&'static StaticFixtureVectorInventoryEntry> {
    PHASE4_STATIC_FIXTURE_VECTOR_INVENTORY
        .iter()
        .find(|entry| entry.path == path)
}

pub fn is_authorized_static_fixture_vector_path_for_local_review_only(path: &str) -> bool {
    find_static_fixture_vector_inventory_entry_for_local_review_only(path).is_some()
}

pub fn static_fixture_vector_inventory_authorizes_runtime() -> bool {
    false
}

pub fn static_fixture_vector_inventory_is_finality() -> bool {
    false
}

pub fn static_fixture_vector_inventory_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-ROUND-4-5-FIXTURE-EXPECTATION-MATRIX
//
// Local review result matrix is dependency-free code metadata only.
// It does not read files.
// It does not parse JSON.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving Round 4.5 remains local fixture expectation mapping only.
pub const PHASE4_ROUND_4_5_FIXTURE_EXPECTATION_MATRIX_ONLY: bool = true;

/// Static expectation category for each authorized Phase 4 vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureExpectationKind {
    ProofPackageValid,
    ProofPackageReplayRejected,
    ProofPackageClusterMismatch,
    ProofPackageMintMismatch,
    ProofPackageRpcDisagreement,
    ChallengeAccepted,
    ChallengeRejected,
    RecoveryCaseValid,
}

/// Dependency-free expected local review result for a fixture.
///
/// This is not a test runner.
/// This is not file IO.
/// This is not JSON parsing.
/// This is not runtime authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpectedFindingSet {
    pub expected_decision: LocalProofReviewDecision,
    pub expected_findings: &'static [ProofReviewFinding],
    pub forbidden_interpretation_markers: &'static [&'static str],
}

/// Static expectation entry for an authorized fixture vector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureExpectationMatrixEntry {
    pub path: &'static str,
    pub case_id: &'static str,
    pub kind: FixtureExpectationKind,
    pub expectation: ExpectedFindingSet,
}

pub const EXPECTED_FINDINGS_PROOF_PACKAGE_VALID: &[ProofReviewFinding] = &[
    ProofReviewFinding::ValidForLocalReviewOnly,
];

pub const EXPECTED_FINDINGS_REPLAY_REJECTED: &[ProofReviewFinding] = &[
    ProofReviewFinding::ReplayRejected,
    ProofReviewFinding::ReusedNonce,
];

pub const EXPECTED_FINDINGS_CLUSTER_MISMATCH: &[ProofReviewFinding] = &[
    ProofReviewFinding::ClusterMismatch,
];

pub const EXPECTED_FINDINGS_MINT_MISMATCH: &[ProofReviewFinding] = &[
    ProofReviewFinding::MintMismatch,
];

pub const EXPECTED_FINDINGS_RPC_DISAGREEMENT: &[ProofReviewFinding] = &[
    ProofReviewFinding::QuorumDisputed,
];

pub const EXPECTED_FINDINGS_CHALLENGE_ACCEPTED: &[ProofReviewFinding] = &[
    ProofReviewFinding::ReplayRejected,
];

pub const EXPECTED_FINDINGS_CHALLENGE_REJECTED: &[ProofReviewFinding] = &[
    ProofReviewFinding::ValidForLocalReviewOnly,
];

pub const EXPECTED_FINDINGS_RECOVERY_CASE_VALID: &[ProofReviewFinding] = &[
    ProofReviewFinding::ValidForLocalReviewOnly,
];

pub const FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS: &[&str] = &[
    "not finality",
    "not settlement",
    "not bridge completion",
    "not runtime authorization",
    "not staking",
    "not liquidity",
    "not exchange-facing logic",
];

pub const PHASE4_FIXTURE_EXPECTATION_MATRIX: &[FixtureExpectationMatrixEntry] = &[
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/proof-package.valid.json",
        case_id: "proof-package.valid",
        kind: FixtureExpectationKind::ProofPackageValid,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ValidForLocalReviewOnly,
            expected_findings: EXPECTED_FINDINGS_PROOF_PACKAGE_VALID,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/proof-package.replay-rejected.json",
        case_id: "proof-package.replay-rejected",
        kind: FixtureExpectationKind::ProofPackageReplayRejected,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ReviewRejected,
            expected_findings: EXPECTED_FINDINGS_REPLAY_REJECTED,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/proof-package.cluster-mismatch.json",
        case_id: "proof-package.cluster-mismatch",
        kind: FixtureExpectationKind::ProofPackageClusterMismatch,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ReviewRejected,
            expected_findings: EXPECTED_FINDINGS_CLUSTER_MISMATCH,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/proof-package.mint-mismatch.json",
        case_id: "proof-package.mint-mismatch",
        kind: FixtureExpectationKind::ProofPackageMintMismatch,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ReviewRejected,
            expected_findings: EXPECTED_FINDINGS_MINT_MISMATCH,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/proof-package.rpc-disagreement.json",
        case_id: "proof-package.rpc-disagreement",
        kind: FixtureExpectationKind::ProofPackageRpcDisagreement,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::EvidenceIncomplete,
            expected_findings: EXPECTED_FINDINGS_RPC_DISAGREEMENT,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/challenge.accepted.json",
        case_id: "challenge.accepted",
        kind: FixtureExpectationKind::ChallengeAccepted,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ReviewRejected,
            expected_findings: EXPECTED_FINDINGS_CHALLENGE_ACCEPTED,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/challenge.rejected.json",
        case_id: "challenge.rejected",
        kind: FixtureExpectationKind::ChallengeRejected,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ValidForLocalReviewOnly,
            expected_findings: EXPECTED_FINDINGS_CHALLENGE_REJECTED,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
    FixtureExpectationMatrixEntry {
        path: "tests/vectors/recovery.case.valid.json",
        case_id: "recovery.case.valid",
        kind: FixtureExpectationKind::RecoveryCaseValid,
        expectation: ExpectedFindingSet {
            expected_decision: LocalProofReviewDecision::ValidForLocalReviewOnly,
            expected_findings: EXPECTED_FINDINGS_RECOVERY_CASE_VALID,
            forbidden_interpretation_markers: FORBIDDEN_FIXTURE_INTERPRETATION_MARKERS,
        },
    },
];

pub fn phase4_fixture_expectation_matrix_for_local_review_only(
) -> &'static [FixtureExpectationMatrixEntry] {
    PHASE4_FIXTURE_EXPECTATION_MATRIX
}

pub fn find_fixture_expectation_by_case_id_for_local_review_only(
    case_id: &str,
) -> Option<&'static FixtureExpectationMatrixEntry> {
    PHASE4_FIXTURE_EXPECTATION_MATRIX
        .iter()
        .find(|entry| entry.case_id == case_id)
}

pub fn find_fixture_expectation_by_path_for_local_review_only(
    path: &str,
) -> Option<&'static FixtureExpectationMatrixEntry> {
    PHASE4_FIXTURE_EXPECTATION_MATRIX
        .iter()
        .find(|entry| entry.path == path)
}

pub fn fixture_expectation_matrix_contains_case_id_for_local_review_only(case_id: &str) -> bool {
    find_fixture_expectation_by_case_id_for_local_review_only(case_id).is_some()
}

pub fn fixture_expectation_matrix_contains_path_for_local_review_only(path: &str) -> bool {
    find_fixture_expectation_by_path_for_local_review_only(path).is_some()
}

pub fn fixture_expectation_accepts_review_for_local_review_only(
    entry: &FixtureExpectationMatrixEntry,
    review: &ProofReview,
) -> bool {
    if review.decision != entry.expectation.expected_decision {
        return false;
    }

    entry.expectation.expected_findings.iter().all(|expected| {
        review.findings.contains(expected)
            || (*expected == ProofReviewFinding::ValidForLocalReviewOnly
                && review.findings.is_empty()
                && review.decision == LocalProofReviewDecision::ValidForLocalReviewOnly)
    })
}

pub fn fixture_expectation_matrix_authorizes_runtime() -> bool {
    false
}

pub fn fixture_expectation_matrix_reads_files() -> bool {
    false
}

pub fn fixture_expectation_matrix_parses_json() -> bool {
    false
}

pub fn fixture_expectation_matrix_is_finality() -> bool {
    false
}

pub fn fixture_expectation_matrix_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-A-STATE-TRANSITION-REVIEW
//
// State transition review is dependency-free local code only.
// It does not read files.
// It does not parse JSON.
// It does not call RPC.
// It does not call wallets.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving this batch remains local state-transition review only.
pub const PHASE4_CODE_BATCH_A_STATE_TRANSITION_REVIEW_ONLY: bool = true;

/// Local proof states used by the non-value state-transition reviewer.
///
/// These are review states only. They are not bridge states, not settlement states,
/// and not runtime authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalProofState {
    Draft,
    Requested,
    Observed,
    ProofPackaged,
    EvidenceInsufficient,
    QuorumDisputed,
    ChallengeOpen,
    Challenged,
    Expired,
    LocalReviewConsistent,
    DecisionGateRequired,
    Failed,
    RecoveryQueued,
    Halted,
    Abandoned,
}

impl LocalProofState {
    pub fn as_label(self) -> &'static str {
        match self {
            LocalProofState::Draft => "Draft",
            LocalProofState::Requested => "Requested",
            LocalProofState::Observed => "Observed",
            LocalProofState::ProofPackaged => "ProofPackaged",
            LocalProofState::EvidenceInsufficient => "EvidenceInsufficient",
            LocalProofState::QuorumDisputed => "QuorumDisputed",
            LocalProofState::ChallengeOpen => "ChallengeOpen",
            LocalProofState::Challenged => "Challenged",
            LocalProofState::Expired => "Expired",
            LocalProofState::LocalReviewConsistent => "LocalReviewConsistent",
            LocalProofState::DecisionGateRequired => "DecisionGateRequired",
            LocalProofState::Failed => "Failed",
            LocalProofState::RecoveryQueued => "RecoveryQueued",
            LocalProofState::Halted => "Halted",
            LocalProofState::Abandoned => "Abandoned",
        }
    }

    pub fn is_terminal_for_local_review_only(self) -> bool {
        matches!(
            self,
            LocalProofState::DecisionGateRequired
                | LocalProofState::Failed
                | LocalProofState::Halted
                | LocalProofState::Abandoned
        )
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Local transition intent reviewed by the proof engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateTransitionIntent {
    pub from: LocalProofState,
    pub to: LocalProofState,
}

impl StateTransitionIntent {
    pub fn new(from: LocalProofState, to: LocalProofState) -> Self {
        Self { from, to }
    }

    pub fn is_noop(self) -> bool {
        self.from == self.to
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Local-only state transition decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateTransitionReviewDecision {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    RuntimeNotAuthorized,
}

/// Findings emitted by the local state-transition reviewer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateTransitionReviewFinding {
    LocalReviewTransitionAccepted,
    UnsupportedStateTransition,
    EvidenceIncompleteTransition,
    ProofReviewRejected,
    ReplayRejectedTransition,
    QuorumDisputedTransitionRejected,
    ChallengeOpenTransitionRejected,
    HaltedStateTransitionRejected,
    RecoveryBypassRejected,
    RuntimeAuthorizationRejected,
    FinalityClaimRejected,
    SettlementClaimRejected,
    ChallengeWindowSkipped,
}

/// Deterministic local state-transition review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateTransitionReview {
    pub intent: StateTransitionIntent,
    pub decision: StateTransitionReviewDecision,
    pub findings: Vec<StateTransitionReviewFinding>,
}

impl StateTransitionReview {
    pub fn has_finding(&self, finding: StateTransitionReviewFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub type StateTransitionReviewSkeleton = StateTransitionReview;

pub fn state_transition_is_supported_for_local_review_only(
    intent: StateTransitionIntent,
) -> bool {
    if intent.is_noop() {
        return true;
    }

    matches!(
        (intent.from, intent.to),
        (LocalProofState::Draft, LocalProofState::Requested)
            | (LocalProofState::Requested, LocalProofState::Observed)
            | (LocalProofState::Observed, LocalProofState::ProofPackaged)
            | (LocalProofState::ProofPackaged, LocalProofState::EvidenceInsufficient)
            | (LocalProofState::ProofPackaged, LocalProofState::QuorumDisputed)
            | (LocalProofState::ProofPackaged, LocalProofState::ChallengeOpen)
            | (LocalProofState::ProofPackaged, LocalProofState::LocalReviewConsistent)
            | (LocalProofState::ProofPackaged, LocalProofState::Failed)
            | (LocalProofState::ProofPackaged, LocalProofState::Halted)
            | (LocalProofState::ProofPackaged, LocalProofState::RecoveryQueued)
            | (LocalProofState::EvidenceInsufficient, LocalProofState::Observed)
            | (LocalProofState::EvidenceInsufficient, LocalProofState::ProofPackaged)
            | (LocalProofState::EvidenceInsufficient, LocalProofState::Failed)
            | (LocalProofState::EvidenceInsufficient, LocalProofState::Abandoned)
            | (LocalProofState::QuorumDisputed, LocalProofState::ChallengeOpen)
            | (LocalProofState::QuorumDisputed, LocalProofState::Failed)
            | (LocalProofState::QuorumDisputed, LocalProofState::Abandoned)
            | (LocalProofState::ChallengeOpen, LocalProofState::Challenged)
            | (LocalProofState::ChallengeOpen, LocalProofState::Expired)
            | (LocalProofState::Challenged, LocalProofState::Failed)
            | (LocalProofState::Challenged, LocalProofState::RecoveryQueued)
            | (LocalProofState::Challenged, LocalProofState::Abandoned)
            | (LocalProofState::Expired, LocalProofState::ProofPackaged)
            | (LocalProofState::Expired, LocalProofState::LocalReviewConsistent)
            | (LocalProofState::RecoveryQueued, LocalProofState::Failed)
            | (LocalProofState::RecoveryQueued, LocalProofState::Halted)
            | (LocalProofState::RecoveryQueued, LocalProofState::Abandoned)
            | (LocalProofState::Halted, LocalProofState::RecoveryQueued)
            | (LocalProofState::Halted, LocalProofState::Abandoned)
            | (LocalProofState::LocalReviewConsistent, LocalProofState::DecisionGateRequired)
    )
}

pub fn review_state_transition_for_local_review_only(
    intent: StateTransitionIntent,
    proof_review: &ProofReview,
) -> StateTransitionReview {
    let mut findings = Vec::new();

    if proof_review.is_runtime_authorized()
        || proof_review.decision == LocalProofReviewDecision::RuntimeNotAuthorized
    {
        return state_transition_result(
            intent,
            StateTransitionReviewDecision::RuntimeNotAuthorized,
            &[StateTransitionReviewFinding::RuntimeAuthorizationRejected],
        );
    }

    if proof_review.is_finality_claim() {
        return state_transition_result(
            intent,
            StateTransitionReviewDecision::ReviewRejected,
            &[StateTransitionReviewFinding::FinalityClaimRejected],
        );
    }

    if proof_review.is_settlement_claim() {
        return state_transition_result(
            intent,
            StateTransitionReviewDecision::ReviewRejected,
            &[StateTransitionReviewFinding::SettlementClaimRejected],
        );
    }

    if intent.from == LocalProofState::Halted
        && !matches!(
            intent.to,
            LocalProofState::Halted | LocalProofState::RecoveryQueued | LocalProofState::Abandoned
        )
    {
        return state_transition_result(
            intent,
            StateTransitionReviewDecision::ReviewRejected,
            &[StateTransitionReviewFinding::HaltedStateTransitionRejected],
        );
    }

    if intent.from == LocalProofState::RecoveryQueued
        && matches!(
            intent.to,
            LocalProofState::LocalReviewConsistent | LocalProofState::DecisionGateRequired
        )
    {
        return state_transition_result(
            intent,
            StateTransitionReviewDecision::ReviewRejected,
            &[StateTransitionReviewFinding::RecoveryBypassRejected],
        );
    }

    if !state_transition_is_supported_for_local_review_only(intent) {
        return state_transition_result(
            intent,
            StateTransitionReviewDecision::ReviewRejected,
            &[StateTransitionReviewFinding::UnsupportedStateTransition],
        );
    }

    if matches!(
        intent.to,
        LocalProofState::LocalReviewConsistent | LocalProofState::DecisionGateRequired
    ) {
        return review_consistency_transition(intent, proof_review);
    }

    match intent.to {
        LocalProofState::EvidenceInsufficient => {
            findings.push(StateTransitionReviewFinding::EvidenceIncompleteTransition);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        LocalProofState::QuorumDisputed => {
            findings.push(StateTransitionReviewFinding::QuorumDisputedTransitionRejected);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        LocalProofState::ChallengeOpen | LocalProofState::Challenged => {
            findings.push(StateTransitionReviewFinding::ChallengeOpenTransitionRejected);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        LocalProofState::Expired => {
            findings.push(StateTransitionReviewFinding::ChallengeWindowSkipped);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        LocalProofState::Halted => {
            findings.push(StateTransitionReviewFinding::HaltedStateTransitionRejected);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::ReviewRejected,
                findings,
            }
        }
        LocalProofState::RecoveryQueued => {
            findings.push(StateTransitionReviewFinding::EvidenceIncompleteTransition);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        LocalProofState::Failed | LocalProofState::Abandoned => {
            findings.push(StateTransitionReviewFinding::ProofReviewRejected);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::ReviewRejected,
                findings,
            }
        }
        _ => {
            findings.push(StateTransitionReviewFinding::LocalReviewTransitionAccepted);
            StateTransitionReview {
                intent,
                decision: StateTransitionReviewDecision::ValidForLocalReviewOnly,
                findings,
            }
        }
    }
}

pub fn review_package_state_transition_for_local_review_only(
    package: &ProofPackageShape,
    expected: &ExpectedProofBinding,
    previously_seen_nonces: &[&str],
    intent: StateTransitionIntent,
) -> StateTransitionReview {
    let proof_review = review_package_with_seen_nonces_for_local_review_only(
        package,
        expected,
        previously_seen_nonces,
    );

    review_state_transition_for_local_review_only(intent, &proof_review)
}

fn review_consistency_transition(
    intent: StateTransitionIntent,
    proof_review: &ProofReview,
) -> StateTransitionReview {
    match proof_review.decision {
        LocalProofReviewDecision::ValidForLocalReviewOnly => state_transition_result(
            intent,
            StateTransitionReviewDecision::ValidForLocalReviewOnly,
            &[StateTransitionReviewFinding::LocalReviewTransitionAccepted],
        ),
        LocalProofReviewDecision::EvidenceIncomplete => state_transition_result(
            intent,
            StateTransitionReviewDecision::EvidenceIncomplete,
            &[map_evidence_incomplete_transition_finding(proof_review)],
        ),
        LocalProofReviewDecision::ReviewRejected => state_transition_result(
            intent,
            StateTransitionReviewDecision::ReviewRejected,
            &[map_rejected_transition_finding(proof_review)],
        ),
        LocalProofReviewDecision::RuntimeNotAuthorized => state_transition_result(
            intent,
            StateTransitionReviewDecision::RuntimeNotAuthorized,
            &[StateTransitionReviewFinding::RuntimeAuthorizationRejected],
        ),
    }
}

fn map_evidence_incomplete_transition_finding(
    proof_review: &ProofReview,
) -> StateTransitionReviewFinding {
    if proof_review.has_finding(ProofReviewFinding::QuorumDisputed) {
        StateTransitionReviewFinding::QuorumDisputedTransitionRejected
    } else if proof_review.has_finding(ProofReviewFinding::ChallengeOpen) {
        StateTransitionReviewFinding::ChallengeOpenTransitionRejected
    } else if proof_review.has_finding(ProofReviewFinding::Halted) {
        StateTransitionReviewFinding::HaltedStateTransitionRejected
    } else if proof_review.has_finding(ProofReviewFinding::RecoveryReviewRequired) {
        StateTransitionReviewFinding::RecoveryBypassRejected
    } else {
        StateTransitionReviewFinding::EvidenceIncompleteTransition
    }
}

fn map_rejected_transition_finding(proof_review: &ProofReview) -> StateTransitionReviewFinding {
    if proof_review.has_finding(ProofReviewFinding::ReplayRejected)
        || proof_review.has_finding(ProofReviewFinding::ReusedNonce)
        || proof_review.has_finding(ProofReviewFinding::NonceMismatch)
    {
        StateTransitionReviewFinding::ReplayRejectedTransition
    } else if proof_review.has_finding(ProofReviewFinding::QuorumDisputed) {
        StateTransitionReviewFinding::QuorumDisputedTransitionRejected
    } else if proof_review.has_finding(ProofReviewFinding::ChallengeOpen) {
        StateTransitionReviewFinding::ChallengeOpenTransitionRejected
    } else if proof_review.has_finding(ProofReviewFinding::Halted) {
        StateTransitionReviewFinding::HaltedStateTransitionRejected
    } else if proof_review.has_finding(ProofReviewFinding::RecoveryReviewRequired) {
        StateTransitionReviewFinding::RecoveryBypassRejected
    } else {
        StateTransitionReviewFinding::ProofReviewRejected
    }
}

fn state_transition_result(
    intent: StateTransitionIntent,
    decision: StateTransitionReviewDecision,
    findings: &[StateTransitionReviewFinding],
) -> StateTransitionReview {
    StateTransitionReview {
        intent,
        decision,
        findings: findings.to_vec(),
    }
}

pub fn state_transition_review_authorizes_runtime() -> bool {
    false
}

pub fn state_transition_review_reads_files() -> bool {
    false
}

pub fn state_transition_review_parses_json() -> bool {
    false
}

pub fn state_transition_review_calls_rpc() -> bool {
    false
}

pub fn state_transition_review_calls_wallet() -> bool {
    false
}

pub fn state_transition_review_is_finality() -> bool {
    false
}

pub fn state_transition_review_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-C-COMPOSITE-LOCAL-REVIEW-CONTEXT
//
// Composite local proof review is dependency-free local code only.
// It does not read files.
// It does not parse JSON.
// It does not call RPC.
// It does not call wallets.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving this batch remains composite local review only.
pub const PHASE4_CODE_BATCH_C_COMPOSITE_LOCAL_REVIEW_CONTEXT_ONLY: bool = true;

/// Local-only evidence inputs that accompany a proof package review.
///
/// These inputs are supplied by fixtures or deterministic local callers. They
/// are not RPC truth, not wallet truth, not runtime truth, not finality, and not
/// settlement authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalProofEvidenceInputs {
    pub challenge_timing: crate::challenge::ChallengeWindowTiming,
    pub quorum_counts: crate::quorum::QuorumEvidenceCount,
    pub recovery_case: crate::recovery::RecoveryCaseKind,
}

impl LocalProofEvidenceInputs {
    pub fn new(
        challenge_timing: crate::challenge::ChallengeWindowTiming,
        quorum_counts: crate::quorum::QuorumEvidenceCount,
        recovery_case: crate::recovery::RecoveryCaseKind,
    ) -> Self {
        Self {
            challenge_timing,
            quorum_counts,
            recovery_case,
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn calls_rpc(self) -> bool {
        false
    }

    pub fn calls_wallet(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Composite local review decision.
///
/// Even the successful decision is only `ValidForLocalReviewOnly`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositeLocalProofReviewDecision {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    RuntimeNotAuthorized,
}

/// Composite local review findings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositeLocalProofReviewFinding {
    PackageReviewAccepted,
    PackageEvidenceIncomplete,
    PackageRejected,
    ChallengeWindowOpen,
    ChallengeWindowExpired,
    ChallengeWindowIncomplete,
    ChallengeRejected,
    QuorumEvidencePresent,
    QuorumEvidenceIncomplete,
    QuorumDisputed,
    HaltRecoveryClear,
    HaltRecoveryRequired,
    HaltRecoveryRejected,
    Halted,
    RuntimeAuthorizationRejected,
    NotFinality,
    NotSettlement,
}

/// Composite local proof review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeLocalProofReview {
    pub package_review: ProofReview,
    pub challenge_review: crate::challenge::ChallengeWindowClockReview,
    pub quorum_review: crate::quorum::QuorumEvidenceReview,
    pub halt_recovery_review: crate::recovery::HaltRecoveryReview,
    pub decision: CompositeLocalProofReviewDecision,
    pub findings: Vec<CompositeLocalProofReviewFinding>,
}

impl CompositeLocalProofReview {
    pub fn has_finding(&self, finding: CompositeLocalProofReviewFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub fn review_composite_local_proof_for_local_review_only(
    package: &ProofPackageShape,
    expected: &ExpectedProofBinding,
    previously_seen_nonces: &[&str],
    inputs: LocalProofEvidenceInputs,
) -> CompositeLocalProofReview {
    let package_review = review_package_with_seen_nonces_for_local_review_only(
        package,
        expected,
        previously_seen_nonces,
    );

    let challenge_review = crate::challenge::review_challenge_window_for_local_review_only(
        package.challenge_status,
        inputs.challenge_timing,
    );

    let quorum_review = crate::quorum::review_quorum_evidence_counts_for_local_review_only(
        inputs.quorum_counts,
    );

    let halt_recovery_review = crate::recovery::review_halt_recovery_for_local_review_only(
        package.halt_status,
        package.recovery_status,
        inputs.recovery_case,
    );

    let mut findings = Vec::new();

    push_composite_package_findings(&mut findings, &package_review);
    push_composite_challenge_findings(&mut findings, &challenge_review);
    push_composite_quorum_findings(&mut findings, &quorum_review);
    push_composite_halt_recovery_findings(&mut findings, &halt_recovery_review);

    push_composite_unique(&mut findings, CompositeLocalProofReviewFinding::NotFinality);
    push_composite_unique(&mut findings, CompositeLocalProofReviewFinding::NotSettlement);

    let decision = decide_composite_local_proof_review(
        &package_review,
        &challenge_review,
        &quorum_review,
        &halt_recovery_review,
    );

    CompositeLocalProofReview {
        package_review,
        challenge_review,
        quorum_review,
        halt_recovery_review,
        decision,
        findings,
    }
}

fn decide_composite_local_proof_review(
    package_review: &ProofReview,
    challenge_review: &crate::challenge::ChallengeWindowClockReview,
    quorum_review: &crate::quorum::QuorumEvidenceReview,
    halt_recovery_review: &crate::recovery::HaltRecoveryReview,
) -> CompositeLocalProofReviewDecision {
    if package_review.is_runtime_authorized()
        || challenge_review.is_runtime_authorized()
        || quorum_review.is_runtime_authorized()
        || halt_recovery_review.is_runtime_authorized()
    {
        return CompositeLocalProofReviewDecision::RuntimeNotAuthorized;
    }

    if package_review.decision == LocalProofReviewDecision::RuntimeNotAuthorized {
        return CompositeLocalProofReviewDecision::RuntimeNotAuthorized;
    }

    if package_review.decision == LocalProofReviewDecision::ReviewRejected
        || challenge_review.decision
            == crate::challenge::ChallengeWindowReviewDecision::ReviewRejected
        || halt_recovery_review.decision
            == crate::recovery::HaltRecoveryReviewDecision::ReviewRejected
    {
        return CompositeLocalProofReviewDecision::ReviewRejected;
    }

    if package_review.decision == LocalProofReviewDecision::EvidenceIncomplete
        || matches!(
            challenge_review.decision,
            crate::challenge::ChallengeWindowReviewDecision::EvidenceIncomplete
                | crate::challenge::ChallengeWindowReviewDecision::ChallengeOpen
        )
        || !matches!(
            quorum_review.decision,
            crate::quorum::QuorumEvidenceReviewDecision::EvidencePresent
        )
        || matches!(
            halt_recovery_review.decision,
            crate::recovery::HaltRecoveryReviewDecision::EvidenceIncomplete
                | crate::recovery::HaltRecoveryReviewDecision::Halted
        )
    {
        return CompositeLocalProofReviewDecision::EvidenceIncomplete;
    }

    CompositeLocalProofReviewDecision::ValidForLocalReviewOnly
}

fn push_composite_package_findings(
    findings: &mut Vec<CompositeLocalProofReviewFinding>,
    package_review: &ProofReview,
) {
    match package_review.decision {
        LocalProofReviewDecision::ValidForLocalReviewOnly => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::PackageReviewAccepted,
            );
        }
        LocalProofReviewDecision::EvidenceIncomplete => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::PackageEvidenceIncomplete,
            );
        }
        LocalProofReviewDecision::ReviewRejected => {
            push_composite_unique(findings, CompositeLocalProofReviewFinding::PackageRejected);
        }
        LocalProofReviewDecision::RuntimeNotAuthorized => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::RuntimeAuthorizationRejected,
            );
        }
    }
}

fn push_composite_challenge_findings(
    findings: &mut Vec<CompositeLocalProofReviewFinding>,
    challenge_review: &crate::challenge::ChallengeWindowClockReview,
) {
    use crate::challenge::ChallengeWindowClockFinding;

    if challenge_review.has_finding(ChallengeWindowClockFinding::WindowOpen) {
        push_composite_unique(findings, CompositeLocalProofReviewFinding::ChallengeWindowOpen);
    }

    if challenge_review.has_finding(ChallengeWindowClockFinding::WindowExpired) {
        push_composite_unique(
            findings,
            CompositeLocalProofReviewFinding::ChallengeWindowExpired,
        );
    }

    if challenge_review.has_finding(ChallengeWindowClockFinding::WindowNotOpened)
        || challenge_review
            .has_finding(ChallengeWindowClockFinding::ReviewDelayNotElapsed)
    {
        push_composite_unique(
            findings,
            CompositeLocalProofReviewFinding::ChallengeWindowIncomplete,
        );
    }

    if challenge_review.has_finding(ChallengeWindowClockFinding::ChallengeAcceptedRejected)
        || challenge_review.has_finding(ChallengeWindowClockFinding::Halted)
    {
        push_composite_unique(findings, CompositeLocalProofReviewFinding::ChallengeRejected);
    }
}

fn push_composite_quorum_findings(
    findings: &mut Vec<CompositeLocalProofReviewFinding>,
    quorum_review: &crate::quorum::QuorumEvidenceReview,
) {
    match quorum_review.decision {
        crate::quorum::QuorumEvidenceReviewDecision::EvidencePresent => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::QuorumEvidencePresent,
            );
        }
        crate::quorum::QuorumEvidenceReviewDecision::EvidenceIncomplete => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::QuorumEvidenceIncomplete,
            );
        }
        crate::quorum::QuorumEvidenceReviewDecision::QuorumDisputed => {
            push_composite_unique(findings, CompositeLocalProofReviewFinding::QuorumDisputed);
        }
        crate::quorum::QuorumEvidenceReviewDecision::RuntimeNotAuthorized => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::RuntimeAuthorizationRejected,
            );
        }
    }
}

fn push_composite_halt_recovery_findings(
    findings: &mut Vec<CompositeLocalProofReviewFinding>,
    halt_recovery_review: &crate::recovery::HaltRecoveryReview,
) {
    match halt_recovery_review.decision {
        crate::recovery::HaltRecoveryReviewDecision::ValidForLocalReviewOnly => {
            push_composite_unique(findings, CompositeLocalProofReviewFinding::HaltRecoveryClear);
        }
        crate::recovery::HaltRecoveryReviewDecision::EvidenceIncomplete => {
            push_composite_unique(findings, CompositeLocalProofReviewFinding::HaltRecoveryRequired);
        }
        crate::recovery::HaltRecoveryReviewDecision::ReviewRejected => {
            push_composite_unique(findings, CompositeLocalProofReviewFinding::HaltRecoveryRejected);
        }
        crate::recovery::HaltRecoveryReviewDecision::Halted => {
            push_composite_unique(findings, CompositeLocalProofReviewFinding::Halted);
        }
        crate::recovery::HaltRecoveryReviewDecision::RuntimeNotAuthorized => {
            push_composite_unique(
                findings,
                CompositeLocalProofReviewFinding::RuntimeAuthorizationRejected,
            );
        }
    }
}

fn push_composite_unique(
    findings: &mut Vec<CompositeLocalProofReviewFinding>,
    finding: CompositeLocalProofReviewFinding,
) {
    if !findings.contains(&finding) {
        findings.push(finding);
    }
}

pub fn composite_local_proof_review_authorizes_runtime() -> bool {
    false
}

pub fn composite_local_proof_review_calls_rpc() -> bool {
    false
}

pub fn composite_local_proof_review_calls_wallet() -> bool {
    false
}

pub fn composite_local_proof_review_is_finality() -> bool {
    false
}

pub fn composite_local_proof_review_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-F-LOCAL-REVIEW-REPORTS
//
// Local review reports are dependency-free local code only.
// They do not read files.
// They do not parse JSON.
// They do not call RPC.
// They do not call wallets.
// They do not authorize runtime.
// They do not prove finality.
// They do not prove settlement.

/// Compile-time marker proving this batch remains deterministic local reports only.
pub const PHASE4_CODE_BATCH_F_LOCAL_REVIEW_REPORTS_ONLY: bool = true;

/// Conservative report severity label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalReviewReportSeverity {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    RuntimeNotAuthorized,
}

impl LocalReviewReportSeverity {
    pub fn as_label(self) -> &'static str {
        match self {
            LocalReviewReportSeverity::ValidForLocalReviewOnly => "ValidForLocalReviewOnly",
            LocalReviewReportSeverity::EvidenceIncomplete => "EvidenceIncomplete",
            LocalReviewReportSeverity::ReviewRejected => "ReviewRejected",
            LocalReviewReportSeverity::RuntimeNotAuthorized => "RuntimeNotAuthorized",
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Local authority posture captured in review reports.
///
/// All fields must remain false for Phase 4 local proof-engine reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalReviewAuthorityPosture {
    pub runtime_authorized: bool,
    pub rpc_called: bool,
    pub wallet_called: bool,
    pub finality_claimed: bool,
    pub settlement_claimed: bool,
}

impl LocalReviewAuthorityPosture {
    pub fn local_review_only() -> Self {
        Self {
            runtime_authorized: false,
            rpc_called: false,
            wallet_called: false,
            finality_claimed: false,
            settlement_claimed: false,
        }
    }

    pub fn is_clean(self) -> bool {
        !self.runtime_authorized
            && !self.rpc_called
            && !self.wallet_called
            && !self.finality_claimed
            && !self.settlement_claimed
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn calls_rpc(self) -> bool {
        false
    }

    pub fn calls_wallet(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Deterministic local report for one composite proof review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeLocalProofReviewReport {
    pub decision: CompositeLocalProofReviewDecision,
    pub decision_label: &'static str,
    pub severity: LocalReviewReportSeverity,
    pub severity_label: &'static str,
    pub package_finding_count: usize,
    pub composite_finding_count: usize,
    pub challenge_finding_count: usize,
    pub quorum_finding_count: usize,
    pub halt_recovery_finding_count: usize,
    pub authority_posture: LocalReviewAuthorityPosture,
}

impl CompositeLocalProofReviewReport {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_clean_local_review_only(&self) -> bool {
        self.authority_posture.is_clean()
            && !self.authorizes_runtime()
            && !self.calls_rpc()
            && !self.calls_wallet()
            && !self.is_finality_claim()
            && !self.is_settlement_claim()
    }
}

impl CompositeLocalProofReviewDecision {
    pub fn as_label(self) -> &'static str {
        match self {
            CompositeLocalProofReviewDecision::ValidForLocalReviewOnly => {
                "ValidForLocalReviewOnly"
            }
            CompositeLocalProofReviewDecision::EvidenceIncomplete => "EvidenceIncomplete",
            CompositeLocalProofReviewDecision::ReviewRejected => "ReviewRejected",
            CompositeLocalProofReviewDecision::RuntimeNotAuthorized => "RuntimeNotAuthorized",
        }
    }

    pub fn as_report_severity(self) -> LocalReviewReportSeverity {
        match self {
            CompositeLocalProofReviewDecision::ValidForLocalReviewOnly => {
                LocalReviewReportSeverity::ValidForLocalReviewOnly
            }
            CompositeLocalProofReviewDecision::EvidenceIncomplete => {
                LocalReviewReportSeverity::EvidenceIncomplete
            }
            CompositeLocalProofReviewDecision::ReviewRejected => {
                LocalReviewReportSeverity::ReviewRejected
            }
            CompositeLocalProofReviewDecision::RuntimeNotAuthorized => {
                LocalReviewReportSeverity::RuntimeNotAuthorized
            }
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

impl CompositeLocalProofReviewFinding {
    pub fn as_label(self) -> &'static str {
        match self {
            CompositeLocalProofReviewFinding::PackageReviewAccepted => "PackageReviewAccepted",
            CompositeLocalProofReviewFinding::PackageEvidenceIncomplete => {
                "PackageEvidenceIncomplete"
            }
            CompositeLocalProofReviewFinding::PackageRejected => "PackageRejected",
            CompositeLocalProofReviewFinding::ChallengeWindowOpen => "ChallengeWindowOpen",
            CompositeLocalProofReviewFinding::ChallengeWindowExpired => {
                "ChallengeWindowExpired"
            }
            CompositeLocalProofReviewFinding::ChallengeWindowIncomplete => {
                "ChallengeWindowIncomplete"
            }
            CompositeLocalProofReviewFinding::ChallengeRejected => "ChallengeRejected",
            CompositeLocalProofReviewFinding::QuorumEvidencePresent => {
                "QuorumEvidencePresent"
            }
            CompositeLocalProofReviewFinding::QuorumEvidenceIncomplete => {
                "QuorumEvidenceIncomplete"
            }
            CompositeLocalProofReviewFinding::QuorumDisputed => "QuorumDisputed",
            CompositeLocalProofReviewFinding::HaltRecoveryClear => "HaltRecoveryClear",
            CompositeLocalProofReviewFinding::HaltRecoveryRequired => {
                "HaltRecoveryRequired"
            }
            CompositeLocalProofReviewFinding::HaltRecoveryRejected => {
                "HaltRecoveryRejected"
            }
            CompositeLocalProofReviewFinding::Halted => "Halted",
            CompositeLocalProofReviewFinding::RuntimeAuthorizationRejected => {
                "RuntimeAuthorizationRejected"
            }
            CompositeLocalProofReviewFinding::NotFinality => "NotFinality",
            CompositeLocalProofReviewFinding::NotSettlement => "NotSettlement",
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

pub fn report_for_composite_local_proof_review(
    review: &CompositeLocalProofReview,
) -> CompositeLocalProofReviewReport {
    let decision = review.decision;
    let severity = decision.as_report_severity();

    CompositeLocalProofReviewReport {
        decision,
        decision_label: decision.as_label(),
        severity,
        severity_label: severity.as_label(),
        package_finding_count: review.package_review.findings.len(),
        composite_finding_count: review.findings.len(),
        challenge_finding_count: review.challenge_review.findings.len(),
        quorum_finding_count: review.quorum_review.findings.len(),
        halt_recovery_finding_count: review.halt_recovery_review.findings.len(),
        authority_posture: LocalReviewAuthorityPosture::local_review_only(),
    }
}

pub fn local_review_report_authorizes_runtime() -> bool {
    false
}

pub fn local_review_report_reads_files() -> bool {
    false
}

pub fn local_review_report_parses_json() -> bool {
    false
}

pub fn local_review_report_calls_rpc() -> bool {
    false
}

pub fn local_review_report_calls_wallet() -> bool {
    false
}

pub fn local_review_report_is_finality() -> bool {
    false
}

pub fn local_review_report_is_settlement() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-G-LOCAL-TRACE-STATUS
//
// Local review trace/status projection is dependency-free local code only.
// It does not read files.
// It does not parse JSON.
// It does not call RPC.
// It does not call wallets.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving this batch remains deterministic trace/status projection only.
pub const PHASE4_CODE_BATCH_G_LOCAL_TRACE_STATUS_ONLY: bool = true;

/// Ordered local review trace step kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalReviewTraceStepKind {
    PackageReview,
    ChallengeWindowReview,
    QuorumEvidenceReview,
    HaltRecoveryReview,
    AuthorityPostureReview,
}

/// One deterministic local trace step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalReviewTraceStep {
    pub ordinal: u8,
    pub kind: LocalReviewTraceStepKind,
    pub label: &'static str,
    pub severity: LocalReviewReportSeverity,
}

impl LocalReviewTraceStep {
    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn calls_rpc(self) -> bool {
        false
    }

    pub fn calls_wallet(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Deterministic local trace for one composite review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeLocalProofReviewTrace {
    pub decision: CompositeLocalProofReviewDecision,
    pub decision_label: &'static str,
    pub steps: Vec<LocalReviewTraceStep>,
    pub authority_posture: LocalReviewAuthorityPosture,
}

impl CompositeLocalProofReviewTrace {
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn has_step_kind(&self, kind: LocalReviewTraceStepKind) -> bool {
        self.steps.iter().any(|step| step.kind == kind)
    }

    pub fn is_clean_local_review_only(&self) -> bool {
        self.authority_posture.is_clean()
            && self.steps.iter().all(|step| {
                !step.authorizes_runtime()
                    && !step.calls_rpc()
                    && !step.calls_wallet()
                    && !step.is_finality()
                    && !step.is_settlement()
            })
    }

    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

/// Stable local status labels for review projection.
///
/// These labels are local review output only. They are not UI runtime,
/// not wallet truth, not ledger truth, not finality, and not settlement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalReviewStatusLabel {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    RuntimeNotAuthorized,
    ChallengeOpen,
    QuorumDisputed,
    Halted,
    RecoveryReviewRequired,
}

impl LocalReviewStatusLabel {
    pub fn as_label(self) -> &'static str {
        match self {
            LocalReviewStatusLabel::ValidForLocalReviewOnly => "ValidForLocalReviewOnly",
            LocalReviewStatusLabel::EvidenceIncomplete => "EvidenceIncomplete",
            LocalReviewStatusLabel::ReviewRejected => "ReviewRejected",
            LocalReviewStatusLabel::RuntimeNotAuthorized => "RuntimeNotAuthorized",
            LocalReviewStatusLabel::ChallengeOpen => "ChallengeOpen",
            LocalReviewStatusLabel::QuorumDisputed => "QuorumDisputed",
            LocalReviewStatusLabel::Halted => "Halted",
            LocalReviewStatusLabel::RecoveryReviewRequired => "RecoveryReviewRequired",
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }
}

/// Deterministic local status projection for one composite review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalReviewStatusProjection {
    pub primary: LocalReviewStatusLabel,
    pub primary_label: &'static str,
    pub detail_label: &'static str,
    pub decision_label: &'static str,
    pub severity: LocalReviewReportSeverity,
    pub stale_safe: bool,
    pub local_review_only: bool,
    pub authority_posture: LocalReviewAuthorityPosture,
}

impl LocalReviewStatusProjection {
    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_display_authority(&self) -> bool {
        false
    }

    pub fn is_clean_local_review_only(&self) -> bool {
        self.stale_safe
            && self.local_review_only
            && self.authority_posture.is_clean()
            && !self.authorizes_runtime()
            && !self.calls_rpc()
            && !self.calls_wallet()
            && !self.is_finality_claim()
            && !self.is_settlement_claim()
            && !self.is_display_authority()
    }
}

pub fn trace_for_composite_local_proof_review(
    review: &CompositeLocalProofReview,
) -> CompositeLocalProofReviewTrace {
    let mut steps = Vec::new();

    push_local_trace_step(
        &mut steps,
        LocalReviewTraceStepKind::PackageReview,
        package_review_trace_label(review.package_review.decision),
        review.package_review.decision.as_composite_trace_severity(),
    );

    push_local_trace_step(
        &mut steps,
        LocalReviewTraceStepKind::ChallengeWindowReview,
        challenge_review_trace_label(review.challenge_review.decision),
        challenge_decision_trace_severity(review.challenge_review.decision),
    );

    push_local_trace_step(
        &mut steps,
        LocalReviewTraceStepKind::QuorumEvidenceReview,
        quorum_review_trace_label(review.quorum_review.decision),
        quorum_decision_trace_severity(review.quorum_review.decision),
    );

    push_local_trace_step(
        &mut steps,
        LocalReviewTraceStepKind::HaltRecoveryReview,
        halt_recovery_review_trace_label(review.halt_recovery_review.decision),
        halt_recovery_decision_trace_severity(review.halt_recovery_review.decision),
    );

    push_local_trace_step(
        &mut steps,
        LocalReviewTraceStepKind::AuthorityPostureReview,
        "AuthorityPostureCleanLocalReviewOnly",
        LocalReviewReportSeverity::ValidForLocalReviewOnly,
    );

    CompositeLocalProofReviewTrace {
        decision: review.decision,
        decision_label: review.decision.as_label(),
        steps,
        authority_posture: LocalReviewAuthorityPosture::local_review_only(),
    }
}

pub fn status_projection_for_composite_local_proof_review(
    review: &CompositeLocalProofReview,
) -> LocalReviewStatusProjection {
    let primary = primary_status_label_for_composite_review(review);
    let detail_label = detail_status_label_for_composite_review(review);
    let severity = review.decision.as_report_severity();

    LocalReviewStatusProjection {
        primary,
        primary_label: primary.as_label(),
        detail_label,
        decision_label: review.decision.as_label(),
        severity,
        stale_safe: true,
        local_review_only: true,
        authority_posture: LocalReviewAuthorityPosture::local_review_only(),
    }
}

fn primary_status_label_for_composite_review(
    review: &CompositeLocalProofReview,
) -> LocalReviewStatusLabel {
    if review.decision == CompositeLocalProofReviewDecision::RuntimeNotAuthorized {
        return LocalReviewStatusLabel::RuntimeNotAuthorized;
    }

    if review.has_finding(CompositeLocalProofReviewFinding::Halted) {
        return LocalReviewStatusLabel::Halted;
    }

    if review.has_finding(CompositeLocalProofReviewFinding::QuorumDisputed) {
        return LocalReviewStatusLabel::QuorumDisputed;
    }

    if review.has_finding(CompositeLocalProofReviewFinding::ChallengeWindowOpen)
        || review.has_finding(CompositeLocalProofReviewFinding::ChallengeWindowIncomplete)
    {
        return LocalReviewStatusLabel::ChallengeOpen;
    }

    if review.has_finding(CompositeLocalProofReviewFinding::HaltRecoveryRequired) {
        return LocalReviewStatusLabel::RecoveryReviewRequired;
    }

    match review.decision {
        CompositeLocalProofReviewDecision::ValidForLocalReviewOnly => {
            LocalReviewStatusLabel::ValidForLocalReviewOnly
        }
        CompositeLocalProofReviewDecision::EvidenceIncomplete => {
            LocalReviewStatusLabel::EvidenceIncomplete
        }
        CompositeLocalProofReviewDecision::ReviewRejected => {
            LocalReviewStatusLabel::ReviewRejected
        }
        CompositeLocalProofReviewDecision::RuntimeNotAuthorized => {
            LocalReviewStatusLabel::RuntimeNotAuthorized
        }
    }
}

fn detail_status_label_for_composite_review(
    review: &CompositeLocalProofReview,
) -> &'static str {
    if review.has_finding(CompositeLocalProofReviewFinding::PackageRejected) {
        return "PackageRejected";
    }

    if review.has_finding(CompositeLocalProofReviewFinding::ChallengeRejected) {
        return "ChallengeRejected";
    }

    if review.has_finding(CompositeLocalProofReviewFinding::QuorumDisputed) {
        return "QuorumDisputed";
    }

    if review.has_finding(CompositeLocalProofReviewFinding::ChallengeWindowExpired) {
        return "ChallengeWindowExpired";
    }

    if review.has_finding(CompositeLocalProofReviewFinding::ChallengeWindowOpen) {
        return "ChallengeWindowOpen";
    }

    if review.has_finding(CompositeLocalProofReviewFinding::HaltRecoveryRequired) {
        return "HaltRecoveryRequired";
    }

    if review.has_finding(CompositeLocalProofReviewFinding::HaltRecoveryClear) {
        return "HaltRecoveryClear";
    }

    "LocalReviewOnly"
}

fn push_local_trace_step(
    steps: &mut Vec<LocalReviewTraceStep>,
    kind: LocalReviewTraceStepKind,
    label: &'static str,
    severity: LocalReviewReportSeverity,
) {
    let ordinal = steps.len().saturating_add(1) as u8;

    steps.push(LocalReviewTraceStep {
        ordinal,
        kind,
        label,
        severity,
    });
}

fn package_review_trace_label(decision: LocalProofReviewDecision) -> &'static str {
    match decision {
        LocalProofReviewDecision::ValidForLocalReviewOnly => "PackageReviewAccepted",
        LocalProofReviewDecision::EvidenceIncomplete => "PackageEvidenceIncomplete",
        LocalProofReviewDecision::ReviewRejected => "PackageReviewRejected",
        LocalProofReviewDecision::RuntimeNotAuthorized => "PackageRuntimeNotAuthorized",
    }
}

fn challenge_review_trace_label(
    decision: crate::challenge::ChallengeWindowReviewDecision,
) -> &'static str {
    match decision {
        crate::challenge::ChallengeWindowReviewDecision::ValidForLocalReviewOnly => {
            "ChallengeWindowClear"
        }
        crate::challenge::ChallengeWindowReviewDecision::EvidenceIncomplete => {
            "ChallengeWindowEvidenceIncomplete"
        }
        crate::challenge::ChallengeWindowReviewDecision::ChallengeOpen => {
            "ChallengeWindowOpen"
        }
        crate::challenge::ChallengeWindowReviewDecision::ReviewRejected => {
            "ChallengeWindowRejected"
        }
        crate::challenge::ChallengeWindowReviewDecision::RuntimeNotAuthorized => {
            "ChallengeWindowRuntimeNotAuthorized"
        }
    }
}

fn quorum_review_trace_label(
    decision: crate::quorum::QuorumEvidenceReviewDecision,
) -> &'static str {
    match decision {
        crate::quorum::QuorumEvidenceReviewDecision::EvidencePresent => {
            "QuorumEvidencePresent"
        }
        crate::quorum::QuorumEvidenceReviewDecision::EvidenceIncomplete => {
            "QuorumEvidenceIncomplete"
        }
        crate::quorum::QuorumEvidenceReviewDecision::QuorumDisputed => "QuorumDisputed",
        crate::quorum::QuorumEvidenceReviewDecision::RuntimeNotAuthorized => {
            "QuorumRuntimeNotAuthorized"
        }
    }
}

fn halt_recovery_review_trace_label(
    decision: crate::recovery::HaltRecoveryReviewDecision,
) -> &'static str {
    match decision {
        crate::recovery::HaltRecoveryReviewDecision::ValidForLocalReviewOnly => {
            "HaltRecoveryClear"
        }
        crate::recovery::HaltRecoveryReviewDecision::EvidenceIncomplete => {
            "HaltRecoveryEvidenceIncomplete"
        }
        crate::recovery::HaltRecoveryReviewDecision::ReviewRejected => {
            "HaltRecoveryRejected"
        }
        crate::recovery::HaltRecoveryReviewDecision::Halted => "Halted",
        crate::recovery::HaltRecoveryReviewDecision::RuntimeNotAuthorized => {
            "HaltRecoveryRuntimeNotAuthorized"
        }
    }
}

fn challenge_decision_trace_severity(
    decision: crate::challenge::ChallengeWindowReviewDecision,
) -> LocalReviewReportSeverity {
    match decision {
        crate::challenge::ChallengeWindowReviewDecision::ValidForLocalReviewOnly => {
            LocalReviewReportSeverity::ValidForLocalReviewOnly
        }
        crate::challenge::ChallengeWindowReviewDecision::EvidenceIncomplete
        | crate::challenge::ChallengeWindowReviewDecision::ChallengeOpen => {
            LocalReviewReportSeverity::EvidenceIncomplete
        }
        crate::challenge::ChallengeWindowReviewDecision::ReviewRejected => {
            LocalReviewReportSeverity::ReviewRejected
        }
        crate::challenge::ChallengeWindowReviewDecision::RuntimeNotAuthorized => {
            LocalReviewReportSeverity::RuntimeNotAuthorized
        }
    }
}

fn quorum_decision_trace_severity(
    decision: crate::quorum::QuorumEvidenceReviewDecision,
) -> LocalReviewReportSeverity {
    match decision {
        crate::quorum::QuorumEvidenceReviewDecision::EvidencePresent => {
            LocalReviewReportSeverity::ValidForLocalReviewOnly
        }
        crate::quorum::QuorumEvidenceReviewDecision::EvidenceIncomplete
        | crate::quorum::QuorumEvidenceReviewDecision::QuorumDisputed => {
            LocalReviewReportSeverity::EvidenceIncomplete
        }
        crate::quorum::QuorumEvidenceReviewDecision::RuntimeNotAuthorized => {
            LocalReviewReportSeverity::RuntimeNotAuthorized
        }
    }
}

fn halt_recovery_decision_trace_severity(
    decision: crate::recovery::HaltRecoveryReviewDecision,
) -> LocalReviewReportSeverity {
    match decision {
        crate::recovery::HaltRecoveryReviewDecision::ValidForLocalReviewOnly => {
            LocalReviewReportSeverity::ValidForLocalReviewOnly
        }
        crate::recovery::HaltRecoveryReviewDecision::EvidenceIncomplete
        | crate::recovery::HaltRecoveryReviewDecision::Halted => {
            LocalReviewReportSeverity::EvidenceIncomplete
        }
        crate::recovery::HaltRecoveryReviewDecision::ReviewRejected => {
            LocalReviewReportSeverity::ReviewRejected
        }
        crate::recovery::HaltRecoveryReviewDecision::RuntimeNotAuthorized => {
            LocalReviewReportSeverity::RuntimeNotAuthorized
        }
    }
}

impl LocalProofReviewDecision {
    fn as_composite_trace_severity(self) -> LocalReviewReportSeverity {
        match self {
            LocalProofReviewDecision::ValidForLocalReviewOnly => {
                LocalReviewReportSeverity::ValidForLocalReviewOnly
            }
            LocalProofReviewDecision::EvidenceIncomplete => {
                LocalReviewReportSeverity::EvidenceIncomplete
            }
            LocalProofReviewDecision::ReviewRejected => {
                LocalReviewReportSeverity::ReviewRejected
            }
            LocalProofReviewDecision::RuntimeNotAuthorized => {
                LocalReviewReportSeverity::RuntimeNotAuthorized
            }
        }
    }
}

pub fn local_review_trace_authorizes_runtime() -> bool {
    false
}

pub fn local_review_trace_reads_files() -> bool {
    false
}

pub fn local_review_trace_parses_json() -> bool {
    false
}

pub fn local_review_trace_calls_rpc() -> bool {
    false
}

pub fn local_review_trace_calls_wallet() -> bool {
    false
}

pub fn local_review_trace_is_finality() -> bool {
    false
}

pub fn local_review_trace_is_settlement() -> bool {
    false
}

pub fn local_review_status_projection_authorizes_runtime() -> bool {
    false
}

pub fn local_review_status_projection_calls_rpc() -> bool {
    false
}

pub fn local_review_status_projection_calls_wallet() -> bool {
    false
}

pub fn local_review_status_projection_is_finality() -> bool {
    false
}

pub fn local_review_status_projection_is_settlement() -> bool {
    false
}

pub fn local_review_status_projection_is_display_authority() -> bool {
    false
}

// ROX-ANCHOR:PHASE4-CODE-BATCH-H-LOCAL-DECISION-GATE-GUARD
//
// Local decision-gate guard is dependency-free local code only.
// It does not read files.
// It does not parse JSON.
// It does not call RPC.
// It does not call wallets.
// It does not authorize runtime.
// It does not prove finality.
// It does not prove settlement.

/// Compile-time marker proving this batch remains local decision-gate guard only.
pub const PHASE4_CODE_BATCH_H_LOCAL_DECISION_GATE_GUARD_ONLY: bool = true;

/// Top-level local decision-gate posture.
///
/// `AcceptLocalReviewOnly` never means finality, settlement, bridge completion,
/// runtime authorization, wallet authority, ledger authority, or display
/// authority. It only means the local deterministic review inputs were
/// consistent enough for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalDecisionGatePosture {
    AcceptLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    RuntimeNotAuthorized,
}

impl LocalDecisionGatePosture {
    pub fn as_label(self) -> &'static str {
        match self {
            LocalDecisionGatePosture::AcceptLocalReviewOnly => "AcceptLocalReviewOnly",
            LocalDecisionGatePosture::EvidenceIncomplete => "EvidenceIncomplete",
            LocalDecisionGatePosture::ReviewRejected => "ReviewRejected",
            LocalDecisionGatePosture::RuntimeNotAuthorized => "RuntimeNotAuthorized",
        }
    }

    pub fn authorizes_runtime(self) -> bool {
        false
    }

    pub fn calls_rpc(self) -> bool {
        false
    }

    pub fn calls_wallet(self) -> bool {
        false
    }

    pub fn is_finality(self) -> bool {
        false
    }

    pub fn is_settlement(self) -> bool {
        false
    }

    pub fn is_display_authority(self) -> bool {
        false
    }
}

/// Top-level local decision-gate findings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalDecisionGateFinding {
    CompositeReviewAccepted,
    CompositeReviewEvidenceIncomplete,
    CompositeReviewRejected,
    ReportCleanLocalOnly,
    TraceCleanLocalOnly,
    StatusProjectionCleanLocalOnly,
    AuthorityPostureClean,
    RuntimeAuthorizationRejected,
    RpcAuthorityRejected,
    WalletAuthorityRejected,
    FinalityClaimRejected,
    SettlementClaimRejected,
    DisplayAuthorityRejected,
    LocalReviewOnly,
}

/// Top-level local decision-gate review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDecisionGateReview {
    pub posture: LocalDecisionGatePosture,
    pub posture_label: &'static str,
    pub composite_decision_label: &'static str,
    pub status_label: &'static str,
    pub detail_label: &'static str,
    pub findings: Vec<LocalDecisionGateFinding>,
    pub report: CompositeLocalProofReviewReport,
    pub trace: CompositeLocalProofReviewTrace,
    pub status_projection: LocalReviewStatusProjection,
}

impl LocalDecisionGateReview {
    pub fn has_finding(&self, finding: LocalDecisionGateFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn passes_local_acceptance(&self) -> bool {
        self.posture == LocalDecisionGatePosture::AcceptLocalReviewOnly
            && self.is_clean_local_review_only()
    }

    pub fn is_clean_local_review_only(&self) -> bool {
        self.report.is_clean_local_review_only()
            && self.trace.is_clean_local_review_only()
            && self.status_projection.is_clean_local_review_only()
            && !self.authorizes_runtime()
            && !self.calls_rpc()
            && !self.calls_wallet()
            && !self.is_finality_claim()
            && !self.is_settlement_claim()
            && !self.is_display_authority()
    }

    pub fn authorizes_runtime(&self) -> bool {
        false
    }

    pub fn calls_rpc(&self) -> bool {
        false
    }

    pub fn calls_wallet(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_display_authority(&self) -> bool {
        false
    }
}

pub fn review_local_decision_gate_for_local_review_only(
    review: &CompositeLocalProofReview,
) -> LocalDecisionGateReview {
    let report = report_for_composite_local_proof_review(review);
    let trace = trace_for_composite_local_proof_review(review);
    let status_projection = status_projection_for_composite_local_proof_review(review);

    let mut findings = Vec::new();

    if report.is_clean_local_review_only() {
        push_decision_gate_unique(
            &mut findings,
            LocalDecisionGateFinding::ReportCleanLocalOnly,
        );
    }

    if trace.is_clean_local_review_only() {
        push_decision_gate_unique(
            &mut findings,
            LocalDecisionGateFinding::TraceCleanLocalOnly,
        );
    }

    if status_projection.is_clean_local_review_only() {
        push_decision_gate_unique(
            &mut findings,
            LocalDecisionGateFinding::StatusProjectionCleanLocalOnly,
        );
    }

    if report.authority_posture.is_clean()
        && trace.authority_posture.is_clean()
        && status_projection.authority_posture.is_clean()
    {
        push_decision_gate_unique(
            &mut findings,
            LocalDecisionGateFinding::AuthorityPostureClean,
        );
    }

    push_decision_gate_unique(&mut findings, LocalDecisionGateFinding::LocalReviewOnly);

    let posture = decide_local_decision_gate_posture(
        review,
        &report,
        &trace,
        &status_projection,
        &mut findings,
    );

    LocalDecisionGateReview {
        posture,
        posture_label: posture.as_label(),
        composite_decision_label: review.decision.as_label(),
        status_label: status_projection.primary_label,
        detail_label: status_projection.detail_label,
        findings,
        report,
        trace,
        status_projection,
    }
}

fn decide_local_decision_gate_posture(
    review: &CompositeLocalProofReview,
    report: &CompositeLocalProofReviewReport,
    trace: &CompositeLocalProofReviewTrace,
    status_projection: &LocalReviewStatusProjection,
    findings: &mut Vec<LocalDecisionGateFinding>,
) -> LocalDecisionGatePosture {
    if report.authorizes_runtime()
        || trace.authorizes_runtime()
        || status_projection.authorizes_runtime()
        || review.is_runtime_authorized()
    {
        push_decision_gate_unique(
            findings,
            LocalDecisionGateFinding::RuntimeAuthorizationRejected,
        );
        return LocalDecisionGatePosture::RuntimeNotAuthorized;
    }

    if report.calls_rpc() || trace.calls_rpc() || status_projection.calls_rpc() || review.calls_rpc()
    {
        push_decision_gate_unique(findings, LocalDecisionGateFinding::RpcAuthorityRejected);
        return LocalDecisionGatePosture::RuntimeNotAuthorized;
    }

    if report.calls_wallet()
        || trace.calls_wallet()
        || status_projection.calls_wallet()
        || review.calls_wallet()
    {
        push_decision_gate_unique(findings, LocalDecisionGateFinding::WalletAuthorityRejected);
        return LocalDecisionGatePosture::RuntimeNotAuthorized;
    }

    if report.is_finality_claim()
        || trace.is_finality_claim()
        || status_projection.is_finality_claim()
        || review.is_finality_claim()
    {
        push_decision_gate_unique(findings, LocalDecisionGateFinding::FinalityClaimRejected);
        return LocalDecisionGatePosture::RuntimeNotAuthorized;
    }

    if report.is_settlement_claim()
        || trace.is_settlement_claim()
        || status_projection.is_settlement_claim()
        || review.is_settlement_claim()
    {
        push_decision_gate_unique(findings, LocalDecisionGateFinding::SettlementClaimRejected);
        return LocalDecisionGatePosture::RuntimeNotAuthorized;
    }

    if status_projection.is_display_authority() {
        push_decision_gate_unique(findings, LocalDecisionGateFinding::DisplayAuthorityRejected);
        return LocalDecisionGatePosture::RuntimeNotAuthorized;
    }

    match review.decision {
        CompositeLocalProofReviewDecision::ValidForLocalReviewOnly => {
            push_decision_gate_unique(
                findings,
                LocalDecisionGateFinding::CompositeReviewAccepted,
            );
            LocalDecisionGatePosture::AcceptLocalReviewOnly
        }
        CompositeLocalProofReviewDecision::EvidenceIncomplete => {
            push_decision_gate_unique(
                findings,
                LocalDecisionGateFinding::CompositeReviewEvidenceIncomplete,
            );
            LocalDecisionGatePosture::EvidenceIncomplete
        }
        CompositeLocalProofReviewDecision::ReviewRejected => {
            push_decision_gate_unique(
                findings,
                LocalDecisionGateFinding::CompositeReviewRejected,
            );
            LocalDecisionGatePosture::ReviewRejected
        }
        CompositeLocalProofReviewDecision::RuntimeNotAuthorized => {
            push_decision_gate_unique(
                findings,
                LocalDecisionGateFinding::RuntimeAuthorizationRejected,
            );
            LocalDecisionGatePosture::RuntimeNotAuthorized
        }
    }
}

fn push_decision_gate_unique(
    findings: &mut Vec<LocalDecisionGateFinding>,
    finding: LocalDecisionGateFinding,
) {
    if !findings.contains(&finding) {
        findings.push(finding);
    }
}

pub fn local_decision_gate_guard_authorizes_runtime() -> bool {
    false
}

pub fn local_decision_gate_guard_reads_files() -> bool {
    false
}

pub fn local_decision_gate_guard_parses_json() -> bool {
    false
}

pub fn local_decision_gate_guard_calls_rpc() -> bool {
    false
}

pub fn local_decision_gate_guard_calls_wallet() -> bool {
    false
}

pub fn local_decision_gate_guard_is_finality() -> bool {
    false
}

pub fn local_decision_gate_guard_is_settlement() -> bool {
    false
}

pub fn local_decision_gate_guard_is_display_authority() -> bool {
    false
}
