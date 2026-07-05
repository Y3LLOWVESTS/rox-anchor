//! RO:WHAT — Typed local and read-only RPC observation inputs for ROX Anchor.
//! RO:WHY — Represents what RPC sources claim before quorum review and gives Phase 4 a testable read-only adapter boundary.
//! RO:INTERACTS — rox-anchor-core typed bindings, commitment.rs, quorum.rs, and CLI proof display.
//! RO:INVARIANTS — observations are data only; read-only adapters cannot submit, sign, mint, burn, or settle.
//! RO:SECURITY — no live RPC client is implemented here; tests use fake adapters and all transaction submission remains absent.
//! RO:TEST — constructed by crate-local quorum and read-only adapter tests.

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};

use crate::{review_rpc_observations, RpcCommitmentLevel, RpcProofConfig, RpcQuorumReview};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadOnlyRpcError {
    Unavailable { source: String, reason: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlyAccountStatus {
    pub account: AccountId,
    pub exists: bool,
    pub owner_program_id: Option<ProgramId>,
    pub slot: u64,
}

impl ReadOnlyAccountStatus {
    pub fn new(
        account: AccountId,
        exists: bool,
        owner_program_id: Option<ProgramId>,
        slot: u64,
    ) -> Self {
        Self {
            account,
            exists,
            owner_program_id,
            slot,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlySignatureStatus {
    pub cluster: ClusterId,
    pub program_id: ProgramId,
    pub mint: MintId,
    pub token_account: TokenAccountId,
    pub operation_id: OperationId,
    pub signature: String,
    pub slot: u64,
    pub commitment: RpcCommitmentLevel,
}

impl ReadOnlySignatureStatus {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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

    pub fn into_observation(self, source: impl Into<String>) -> RpcObservation {
        RpcObservation::new(
            source,
            self.cluster,
            self.program_id,
            self.mint,
            self.token_account,
            self.operation_id,
            self.signature,
            self.slot,
            self.commitment,
        )
    }
}

pub trait ReadOnlyRpcAdapter {
    fn source_name(&self) -> &str;

    fn current_slot(&self) -> Result<u64, ReadOnlyRpcError>;

    fn account_status(
        &self,
        account: &AccountId,
    ) -> Result<ReadOnlyAccountStatus, ReadOnlyRpcError>;

    fn signature_status(
        &self,
        signature: &str,
    ) -> Result<Option<ReadOnlySignatureStatus>, ReadOnlyRpcError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlyRpcObservationReview {
    pub current_slot: u64,
    pub observations_checked: u16,
    pub quorum: RpcQuorumReview,
}

pub fn read_only_account_status<A: ReadOnlyRpcAdapter>(
    adapter: &A,
    account: &AccountId,
) -> Result<ReadOnlyAccountStatus, ReadOnlyRpcError> {
    adapter.account_status(account)
}

pub fn review_read_only_rpc_adapters<A: ReadOnlyRpcAdapter>(
    adapters: &[A],
    signature: &str,
    expected: &ExpectedRpcBinding,
    config: RpcProofConfig,
) -> Result<ReadOnlyRpcObservationReview, ReadOnlyRpcError> {
    let mut current_slot = 0_u64;
    let mut observations = Vec::new();

    for adapter in adapters {
        current_slot = current_slot.max(adapter.current_slot()?);

        if let Some(status) = adapter.signature_status(signature)? {
            observations.push(status.into_observation(adapter.source_name()));
        }
    }

    let observations_checked = observations.len().min(u16::MAX as usize) as u16;
    let quorum = review_rpc_observations(&observations, expected, config, current_slot);

    Ok(ReadOnlyRpcObservationReview {
        current_slot,
        observations_checked,
        quorum,
    })
}
