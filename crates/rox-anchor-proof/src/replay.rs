// RO:WHAT — Replay, operation identity, and nonce review helpers for the local proof validator.
// RO:WHY — Checks deterministic source/target/direction/operation/nonce/cluster/program_id/mint/token_account binding without proof finality.
// RO:INTERACTS — package and validate local review modules.
// RO:INVARIANTS — Replay binding and nonce review are local-only evidence review and do not authorize runtime.
// RO:SECURITY — No RPC, wallet, Solana/Anchor runtime, bridge runtime, deployment, staking, liquidity, or external settlement.
// RO:TEST — Static Phase 4 checker only for this round.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local validator does not authorize runtime.

use crate::package::{
    BridgeOperationIdentity, OperationIdentityField, ProofDirection, ProofPackageShape,
};

/// Expected local binding used to review a proof package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedProofBinding {
    pub source_domain: String,
    pub target_domain: String,
    pub direction: ProofDirection,
    pub operation_id: String,
    pub idempotency_key: String,
    pub nonce: String,
    pub cluster: String,
    pub program_id: String,
    pub mint: String,
    pub token_account: String,
}

impl ExpectedProofBinding {
    pub fn from_package_identity(package: &ProofPackageShape) -> Self {
        Self {
            source_domain: package.source_domain.clone(),
            target_domain: package.target_domain.clone(),
            direction: package.direction,
            operation_id: package.operation_id.clone(),
            idempotency_key: package.idempotency_key.clone(),
            nonce: package.nonce.clone(),
            cluster: package.cluster.clone(),
            program_id: package.program_id.clone(),
            mint: package.mint.clone(),
            token_account: package.token_account.clone(),
        }
    }

    pub fn operation_identity(&self) -> BridgeOperationIdentity {
        BridgeOperationIdentity {
            source_domain: self.source_domain.clone(),
            target_domain: self.target_domain.clone(),
            direction: self.direction,
            operation_id: self.operation_id.clone(),
            idempotency_key: self.idempotency_key.clone(),
            nonce: self.nonce.clone(),
            cluster: self.cluster.clone(),
            program_id: self.program_id.clone(),
            mint: self.mint.clone(),
            token_account: self.token_account.clone(),
        }
    }
}

/// Replay posture labels for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayPosture {
    Unchecked,
    MissingBinding,
    BoundForLocalReviewOnly,
    ReplayRejected,
    DomainMismatch,
    DirectionMismatch,
    ClusterMismatch,
    ProgramMismatch,
    MintMismatch,
    TokenAccountMismatch,
    NonceMismatch,
}

/// Nonce findings for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonceReviewFinding {
    MissingNonce,
    ReusedNonce,
    NonceAcceptedForLocalReviewOnly,
}

/// Local-only nonce review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonceReview {
    pub nonce: String,
    pub finding: NonceReviewFinding,
}

/// Operation identity findings for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationIdentityReviewFinding {
    CompleteForLocalReviewOnly,
    OperationIdentityIncomplete,
    OperationIdentityMismatch,
    MissingIdentityField,
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
    ReusedNonce,
    IdempotencyKeyAuthorityMisuse,
}

/// Local-only operation identity review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationIdentityReview {
    pub finding: OperationIdentityReviewFinding,
    pub missing_fields: Vec<OperationIdentityField>,
    pub replay_review: ReplayBindingReview,
    pub nonce_review: NonceReview,
}

impl OperationIdentityReview {
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

/// Result of local replay-binding review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayBindingReview {
    pub posture: ReplayPosture,
    pub mismatches: Vec<ReplayPosture>,
}

pub type ReplayBindingSkeleton = ReplayBindingReview;

impl ReplayBindingReview {
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

pub fn review_static_nonce_for_local_review_only(
    nonce: &str,
    previously_seen_nonces: &[&str],
) -> NonceReview {
    let trimmed = nonce.trim();

    if trimmed.is_empty() {
        return NonceReview {
            nonce: String::new(),
            finding: NonceReviewFinding::MissingNonce,
        };
    }

    if previously_seen_nonces.iter().any(|seen| seen.trim() == trimmed) {
        return NonceReview {
            nonce: trimmed.to_string(),
            finding: NonceReviewFinding::ReusedNonce,
        };
    }

    NonceReview {
        nonce: trimmed.to_string(),
        finding: NonceReviewFinding::NonceAcceptedForLocalReviewOnly,
    }
}

pub fn review_operation_identity_for_local_review_only(
    package: &ProofPackageShape,
    expected: &ExpectedProofBinding,
    previously_seen_nonces: &[&str],
) -> OperationIdentityReview {
    let identity = package.operation_identity();
    let missing_fields = identity.missing_identity_fields();
    let replay_review = review_replay_binding(package, expected);
    let nonce_review =
        review_static_nonce_for_local_review_only(&package.nonce, previously_seen_nonces);

    let finding = if !missing_fields.is_empty() {
        OperationIdentityReviewFinding::OperationIdentityIncomplete
    } else if identity.idempotency_key_is_authority() {
        OperationIdentityReviewFinding::IdempotencyKeyAuthorityMisuse
    } else if nonce_review.finding == NonceReviewFinding::ReusedNonce {
        OperationIdentityReviewFinding::ReusedNonce
    } else if replay_review.posture != ReplayPosture::BoundForLocalReviewOnly {
        OperationIdentityReviewFinding::OperationIdentityMismatch
    } else {
        OperationIdentityReviewFinding::CompleteForLocalReviewOnly
    };

    OperationIdentityReview {
        finding,
        missing_fields,
        replay_review,
        nonce_review,
    }
}

pub fn review_replay_binding(
    package: &ProofPackageShape,
    expected: &ExpectedProofBinding,
) -> ReplayBindingReview {
    let mut mismatches = Vec::new();

    if package.source_domain != expected.source_domain {
        push_unique(&mut mismatches, ReplayPosture::DomainMismatch);
    }
    if package.target_domain != expected.target_domain {
        push_unique(&mut mismatches, ReplayPosture::DomainMismatch);
    }
    if package.direction != expected.direction {
        push_unique(&mut mismatches, ReplayPosture::DirectionMismatch);
    }
    if package.operation_id != expected.operation_id {
        push_unique(&mut mismatches, ReplayPosture::ReplayRejected);
    }
    if package.idempotency_key != expected.idempotency_key {
        push_unique(&mut mismatches, ReplayPosture::ReplayRejected);
    }
    if package.nonce != expected.nonce {
        push_unique(&mut mismatches, ReplayPosture::NonceMismatch);
    }
    if package.cluster != expected.cluster {
        push_unique(&mut mismatches, ReplayPosture::ClusterMismatch);
    }
    if package.program_id != expected.program_id {
        push_unique(&mut mismatches, ReplayPosture::ProgramMismatch);
    }
    if package.mint != expected.mint {
        push_unique(&mut mismatches, ReplayPosture::MintMismatch);
    }
    if package.token_account != expected.token_account {
        push_unique(&mut mismatches, ReplayPosture::TokenAccountMismatch);
    }

    let posture = if mismatches.is_empty() {
        ReplayPosture::BoundForLocalReviewOnly
    } else if mismatches.contains(&ReplayPosture::ReplayRejected) {
        ReplayPosture::ReplayRejected
    } else {
        mismatches[0]
    };

    ReplayBindingReview { posture, mismatches }
}

fn push_unique(findings: &mut Vec<ReplayPosture>, finding: ReplayPosture) {
    if !findings.contains(&finding) {
        findings.push(finding);
    }
}
