//! RO:WHAT — Replay-set helpers for ROX Anchor proof review.
//! RO:WHY — Reused operation IDs, idempotency keys, or nonces must be rejected deterministically.
//! RO:INTERACTS — ProofPackage and validate.rs.
//! RO:INVARIANTS — replay findings are rejection findings, not soft warnings.
//! RO:SECURITY — local replay memory only; no durable ledger or chain mutation.
//! RO:TEST — covered by replay rejection tests.

use std::collections::BTreeSet;

use rox_anchor_core::{IdempotencyKey, Nonce, OperationId};

use crate::ProofPackage;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReplaySet {
    operation_ids: BTreeSet<OperationId>,
    idempotency_keys: BTreeSet<IdempotencyKey>,
    nonces: BTreeSet<Nonce>,
}

impl ReplaySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_package(package: &ProofPackage) -> Self {
        let mut replay = Self::new();
        replay.insert_operation_id(package.operation_id.clone());
        replay.insert_idempotency_key(package.idempotency_key.clone());
        replay.insert_nonce(package.nonce.clone());
        replay
    }

    pub fn insert_operation_id(&mut self, operation_id: OperationId) {
        self.operation_ids.insert(operation_id);
    }

    pub fn insert_idempotency_key(&mut self, idempotency_key: IdempotencyKey) {
        self.idempotency_keys.insert(idempotency_key);
    }

    pub fn insert_nonce(&mut self, nonce: Nonce) {
        self.nonces.insert(nonce);
    }

    pub fn contains_operation_id(&self, operation_id: &OperationId) -> bool {
        self.operation_ids.contains(operation_id)
    }

    pub fn contains_idempotency_key(&self, idempotency_key: &IdempotencyKey) -> bool {
        self.idempotency_keys.contains(idempotency_key)
    }

    pub fn contains_nonce(&self, nonce: &Nonce) -> bool {
        self.nonces.contains(nonce)
    }
}
