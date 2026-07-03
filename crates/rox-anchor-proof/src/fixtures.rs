//! RO:WHAT — Small deterministic proof fixtures for ROX Anchor tests.
//! RO:WHY — Provides local vectors until external fixture files are intentionally reintroduced.
//! RO:INTERACTS — package, validate, quorum, replay tests.
//! RO:INVARIANTS — fixtures are local test data, not runtime proof or finality.
//! RO:SECURITY — no secrets, live RPC data, wallet calls, or value movement.
//! RO:TEST — used by rox-anchor-proof unit tests.

use rox_anchor_core::{
    AccountId, AnchorBinding, AnchorDirection, ChallengePosture, ClusterId, DomainId, HaltPosture,
    IdempotencyKey, MintId, Nonce, OperationId, ProgramId, RecoveryPosture, TokenAccountId,
};

use crate::{EvidenceBundle, ExpectedProofBinding, ProofPackage};

pub fn anchor_binding() -> AnchorBinding {
    AnchorBinding::new(
        DomainId::new("internal-roc").unwrap(),
        DomainId::new("solana-localnet").unwrap(),
        AnchorDirection::RocToRox,
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
    )
}

pub fn expected_proof_binding() -> ExpectedProofBinding {
    ExpectedProofBinding::new(
        anchor_binding(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        IdempotencyKey::new("idem-roc-to-rox-0001").unwrap(),
        Nonce::new("nonce-roc-to-rox-0001").unwrap(),
    )
}

pub fn valid_package() -> ProofPackage {
    let expected = expected_proof_binding();

    ProofPackage::new(
        expected.binding,
        expected.operation_id,
        expected.idempotency_key,
        expected.nonce,
        AccountId::new("roc-account-creator-0001").unwrap(),
        AccountId::new("rox-token-owner-0001").unwrap(),
        EvidenceBundle::satisfied(2),
        ChallengePosture::Clear,
        HaltPosture::Active,
        RecoveryPosture::NotRequired,
    )
}
