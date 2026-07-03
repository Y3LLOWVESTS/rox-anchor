//! RO:WHAT — Typed local RPC observation inputs for ROX Anchor.
//! RO:WHY — Represents what RPC sources claim before quorum review.
//! RO:INTERACTS — rox-anchor-core typed bindings, commitment.rs, and quorum.rs.
//! RO:INVARIANTS — observations are data only; they do not mutate state or prove final settlement.
//! RO:SECURITY — no live RPC calls, no wallet calls, no transaction submission.
//! RO:TEST — constructed by crate-local quorum tests.

use rox_anchor_core::{ClusterId, MintId, OperationId, ProgramId, TokenAccountId};

use crate::RpcCommitmentLevel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedRpcBinding {
    pub cluster: ClusterId,
    pub program_id: ProgramId,
    pub mint: MintId,
    pub token_account: TokenAccountId,
    pub operation_id: OperationId,
    pub minimum_commitment: RpcCommitmentLevel,
}

impl ExpectedRpcBinding {
    pub fn new(
        cluster: ClusterId,
        program_id: ProgramId,
        mint: MintId,
        token_account: TokenAccountId,
        operation_id: OperationId,
        minimum_commitment: RpcCommitmentLevel,
    ) -> Self {
        Self {
            cluster,
            program_id,
            mint,
            token_account,
            operation_id,
            minimum_commitment,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcObservation {
    pub source: String,
    pub cluster: ClusterId,
    pub program_id: ProgramId,
    pub mint: MintId,
    pub token_account: TokenAccountId,
    pub operation_id: OperationId,
    pub signature: String,
    pub slot: u64,
    pub commitment: RpcCommitmentLevel,
}

impl RpcObservation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: impl Into<String>,
        cluster: ClusterId,
        program_id: ProgramId,
        mint: MintId,
        token_account: TokenAccountId,
        operation_id: OperationId,
        signature: impl Into<String>,
        slot: u64,
        commitment: RpcCommitmentLevel,
    ) -> Self {
        Self {
            source: source.into(),
            cluster,
            program_id,
            mint,
            token_account,
            operation_id,
            signature: signature.into(),
            slot,
            commitment,
        }
    }
}
