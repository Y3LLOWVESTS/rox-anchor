//! RO:WHAT — Proof package and expected binding shapes for local ROX Anchor review.
//! RO:WHY — Gives validation one deterministic input model before CLI/services/program code consume it.
//! RO:INTERACTS — rox-anchor-core IDs, quorum evidence, challenge posture, halt posture, recovery posture.
//! RO:INVARIANTS — package bindings are typed; expected bindings are explicit; no inferred finality.
//! RO:SECURITY — data model only; no live RPC, wallet, mint, burn, or settlement calls.
//! RO:TEST — exercised by rox-anchor-proof package validation tests.

use rox_anchor_core::{
    AccountId, AnchorBinding, ChallengePosture, HaltPosture, IdempotencyKey, Nonce, OperationId,
    RecoveryPosture,
};

use crate::EvidenceBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedProofBinding {
    pub binding: AnchorBinding,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
}

impl ExpectedProofBinding {
    pub fn new(
        binding: AnchorBinding,
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        nonce: Nonce,
    ) -> Self {
        Self {
            binding,
            operation_id,
            idempotency_key,
            nonce,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofPackage {
    pub binding: AnchorBinding,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
    pub source_account: AccountId,
    pub target_account: AccountId,
    pub evidence: EvidenceBundle,
    pub challenge_posture: ChallengePosture,
    pub halt_posture: HaltPosture,
    pub recovery_posture: RecoveryPosture,
}

impl ProofPackage {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        binding: AnchorBinding,
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        nonce: Nonce,
        source_account: AccountId,
        target_account: AccountId,
        evidence: EvidenceBundle,
        challenge_posture: ChallengePosture,
        halt_posture: HaltPosture,
        recovery_posture: RecoveryPosture,
    ) -> Self {
        Self {
            binding,
            operation_id,
            idempotency_key,
            nonce,
            source_account,
            target_account,
            evidence,
            challenge_posture,
            halt_posture,
            recovery_posture,
        }
    }

    pub fn expected_binding_snapshot(&self) -> ExpectedProofBinding {
        ExpectedProofBinding::new(
            self.binding.clone(),
            self.operation_id.clone(),
            self.idempotency_key.clone(),
            self.nonce.clone(),
        )
    }
}
