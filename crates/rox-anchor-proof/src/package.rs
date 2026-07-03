// RO:WHAT — Local proof package and operation identity shapes for the rox-anchor-proof validator.
// RO:WHY — Captures required evidence-review fields without executing proof finality, bridge behavior, or runtime behavior.
// RO:INTERACTS — challenge, quorum, recovery, replay, and validate local review modules.
// RO:INVARIANTS — A proof package shape is evidence only; local validation is not finality; this file does not authorize runtime.
// RO:SECURITY — No RPC, wallet, Solana/Anchor runtime, bridge runtime, deployment, minting, burning, staking, liquidity, or external settlement.
// RO:TEST — Static Phase 4 checker only for this round.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local validator does not authorize runtime.

use crate::challenge::ChallengeGatePosture;
use crate::quorum::QuorumObservationPosture;
use crate::recovery::{HaltPosture, RecoveryPosture};

/// Local review direction for a proof package.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofDirection {
    RocToRox,
    RoxToRoc,
    ObservationOnly,
    Unknown,
}

/// Evidence posture labels for non-runtime local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidencePosture {
    Draft,
    Incomplete,
    QuorumDisputed,
    ChallengeOpen,
    ChallengeAccepted,
    ChallengeRejected,
    Halted,
    RecoveryReviewRequired,
    ConsistentForLocalReviewOnly,
    FailedClosed,
}

/// Commitment posture for local review.
/// This is not finality and does not authorize runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommitmentReviewLevel {
    Missing,
    Insufficient,
    ReviewOnly,
}

/// Required proof package fields checked by the local validator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiredProofField {
    SchemaVersion,
    SourceDomain,
    TargetDomain,
    Direction,
    OperationId,
    IdempotencyKey,
    Nonce,
    Cluster,
    ProgramId,
    Mint,
    TokenAccount,
    CommitmentLevel,
    ChallengeStatus,
    HaltStatus,
    RecoveryStatus,
}

/// Fields that form the local operation identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationIdentityField {
    SourceDomain,
    TargetDomain,
    Direction,
    OperationId,
    IdempotencyKey,
    Nonce,
    Cluster,
    ProgramId,
    Mint,
    TokenAccount,
}

/// Local-only operation identity status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationIdentityStatus {
    MissingRequiredField,
    CompleteForLocalReviewOnly,
}

/// Local-only operation identity.
/// It is fixture-bound evidence metadata, not authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeOperationIdentity {
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

impl BridgeOperationIdentity {
    pub fn missing_identity_fields(&self) -> Vec<OperationIdentityField> {
        let mut missing = Vec::new();

        if !is_present(&self.source_domain) {
            missing.push(OperationIdentityField::SourceDomain);
        }
        if !is_present(&self.target_domain) {
            missing.push(OperationIdentityField::TargetDomain);
        }
        if self.direction == ProofDirection::Unknown {
            missing.push(OperationIdentityField::Direction);
        }
        if !is_present(&self.operation_id) {
            missing.push(OperationIdentityField::OperationId);
        }
        if !is_present(&self.idempotency_key) {
            missing.push(OperationIdentityField::IdempotencyKey);
        }
        if !is_present(&self.nonce) {
            missing.push(OperationIdentityField::Nonce);
        }
        if !is_present(&self.cluster) {
            missing.push(OperationIdentityField::Cluster);
        }
        if !is_present(&self.program_id) {
            missing.push(OperationIdentityField::ProgramId);
        }
        if !is_present(&self.mint) {
            missing.push(OperationIdentityField::Mint);
        }
        if !is_present(&self.token_account) {
            missing.push(OperationIdentityField::TokenAccount);
        }

        missing
    }

    pub fn status(&self) -> OperationIdentityStatus {
        if self.missing_identity_fields().is_empty() {
            OperationIdentityStatus::CompleteForLocalReviewOnly
        } else {
            OperationIdentityStatus::MissingRequiredField
        }
    }

    pub fn is_complete_for_local_review_only(&self) -> bool {
        self.status() == OperationIdentityStatus::CompleteForLocalReviewOnly
    }

    pub fn idempotency_key_is_authority(&self) -> bool {
        false
    }

    pub fn nonce_is_value_authority(&self) -> bool {
        false
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

/// Local-only proof package shape reviewed by Phase 4.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofPackageShape {
    pub schema_version: String,
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
    pub commitment_level: CommitmentReviewLevel,
    pub evidence_posture: EvidencePosture,
    pub quorum_posture: QuorumObservationPosture,
    pub challenge_status: ChallengeGatePosture,
    pub halt_status: HaltPosture,
    pub recovery_status: RecoveryPosture,
}

impl ProofPackageShape {
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

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn has_required_fields_for_local_review(&self) -> bool {
        self.missing_required_fields().is_empty()
    }

    pub fn missing_required_fields(&self) -> Vec<RequiredProofField> {
        let mut missing = Vec::new();

        if !is_present(&self.schema_version) {
            missing.push(RequiredProofField::SchemaVersion);
        }
        if !is_present(&self.source_domain) {
            missing.push(RequiredProofField::SourceDomain);
        }
        if !is_present(&self.target_domain) {
            missing.push(RequiredProofField::TargetDomain);
        }
        if self.direction == ProofDirection::Unknown {
            missing.push(RequiredProofField::Direction);
        }
        if !is_present(&self.operation_id) {
            missing.push(RequiredProofField::OperationId);
        }
        if !is_present(&self.idempotency_key) {
            missing.push(RequiredProofField::IdempotencyKey);
        }
        if !is_present(&self.nonce) {
            missing.push(RequiredProofField::Nonce);
        }
        if !is_present(&self.cluster) {
            missing.push(RequiredProofField::Cluster);
        }
        if !is_present(&self.program_id) {
            missing.push(RequiredProofField::ProgramId);
        }
        if !is_present(&self.mint) {
            missing.push(RequiredProofField::Mint);
        }
        if !is_present(&self.token_account) {
            missing.push(RequiredProofField::TokenAccount);
        }
        if self.commitment_level == CommitmentReviewLevel::Missing {
            missing.push(RequiredProofField::CommitmentLevel);
        }
        if self.challenge_status == ChallengeGatePosture::NotOpened {
            missing.push(RequiredProofField::ChallengeStatus);
        }
        if self.halt_status == HaltPosture::Unknown {
            missing.push(RequiredProofField::HaltStatus);
        }
        if self.recovery_status == RecoveryPosture::Unknown {
            missing.push(RequiredProofField::RecoveryStatus);
        }

        missing
    }
}

impl RequiredProofField {
    pub fn as_label(self) -> &'static str {
        match self {
            RequiredProofField::SchemaVersion => "schema_version",
            RequiredProofField::SourceDomain => "source_domain",
            RequiredProofField::TargetDomain => "target_domain",
            RequiredProofField::Direction => "direction",
            RequiredProofField::OperationId => "operation_id",
            RequiredProofField::IdempotencyKey => "idempotency_key",
            RequiredProofField::Nonce => "nonce",
            RequiredProofField::Cluster => "cluster",
            RequiredProofField::ProgramId => "program_id",
            RequiredProofField::Mint => "mint",
            RequiredProofField::TokenAccount => "token_account",
            RequiredProofField::CommitmentLevel => "commitment_level",
            RequiredProofField::ChallengeStatus => "challenge_status",
            RequiredProofField::HaltStatus => "halt_status",
            RequiredProofField::RecoveryStatus => "recovery_status",
        }
    }
}

impl OperationIdentityField {
    pub fn as_label(self) -> &'static str {
        match self {
            OperationIdentityField::SourceDomain => "source_domain",
            OperationIdentityField::TargetDomain => "target_domain",
            OperationIdentityField::Direction => "direction",
            OperationIdentityField::OperationId => "operation_id",
            OperationIdentityField::IdempotencyKey => "idempotency_key",
            OperationIdentityField::Nonce => "nonce",
            OperationIdentityField::Cluster => "cluster",
            OperationIdentityField::ProgramId => "program_id",
            OperationIdentityField::Mint => "mint",
            OperationIdentityField::TokenAccount => "token_account",
        }
    }
}

fn is_present(value: &str) -> bool {
    !value.trim().is_empty()
}
