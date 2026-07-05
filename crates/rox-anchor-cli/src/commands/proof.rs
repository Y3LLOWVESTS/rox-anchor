//! RO:WHAT — Implements `rox-anchor proof` using deterministic read-only RPC proof review.
//! RO:WHY — Surfaces Phase 11 RPC audit output through the CLI without live submission.
//! RO:INTERACTS — rox-anchor-rpc-proof read-only adapters, quorum review, and audit records.
//! RO:INVARIANTS — proof output is display-only and never claims settlement/finality/runtime authority.
//! RO:SECURITY — no live RPC, wallet calls, key loading, transaction submission, minting, burning, or settlement.
//! RO:TEST — covered by CLI read-only proof and audit display tests.

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    review_read_only_rpc_adapters, ExpectedRpcBinding, ReadOnlyAccountStatus, ReadOnlyRpcAdapter,
    ReadOnlyRpcError, ReadOnlySignatureStatus, RpcCommitmentLevel, RpcObservation,
    RpcProofAuditRecord, RpcProofConfig,
};

const READ_ONLY_SIGNATURE: &str = "sig-proof-command-readonly-111111111111";

pub fn proof_help() -> String {
    let adapters = [
        StaticReadOnlyRpc::new("rpc-a", 50),
        StaticReadOnlyRpc::new("rpc-b", 51),
    ];
    let expected = expected_binding();

    let review = review_read_only_rpc_adapters(
        &adapters,
        READ_ONLY_SIGNATURE,
        &expected,
        RpcProofConfig::new(2, 100),
    )
    .expect("static read-only proof command review should not fail");

    let observations = vec![rpc_observation("rpc-a", 49), rpc_observation("rpc-b", 50)];
    let audit = RpcProofAuditRecord::from_review(
        &expected,
        &observations,
        &review.quorum,
        review.current_slot,
    );

    let mut lines = vec![
        "rox-anchor proof".to_string(),
        "status: read_only_rpc_adapter_shape".to_string(),
        "submission: disabled".to_string(),
        "wallet_key_loading: disabled".to_string(),
        "network_client: not_enabled".to_string(),
        format!("current_slot: {}", review.current_slot),
        format!("observations_checked: {}", review.observations_checked),
        format!("quorum_decision: {:?}", review.quorum.decision),
        "audit:".to_string(),
    ];

    lines.extend(audit.render().lines().map(|line| format!("  {line}")));

    lines.extend([
        "next: use `rox-anchor check` for deterministic proof-review output".to_string(),
        "json_input: not enabled yet".to_string(),
    ]);

    lines.join("\n")
}

#[derive(Clone, Debug)]
struct StaticReadOnlyRpc {
    source: &'static str,
    current_slot: u64,
}

impl StaticReadOnlyRpc {
    fn new(source: &'static str, current_slot: u64) -> Self {
        Self {
            source,
            current_slot,
        }
    }
}

impl ReadOnlyRpcAdapter for StaticReadOnlyRpc {
    fn source_name(&self) -> &str {
        self.source
    }

    fn current_slot(&self) -> Result<u64, ReadOnlyRpcError> {
        Ok(self.current_slot)
    }

    fn account_status(
        &self,
        account: &AccountId,
    ) -> Result<ReadOnlyAccountStatus, ReadOnlyRpcError> {
        Ok(ReadOnlyAccountStatus::new(
            account.clone(),
            true,
            Some(program_id()),
            self.current_slot,
        ))
    }

    fn signature_status(
        &self,
        _signature: &str,
    ) -> Result<Option<ReadOnlySignatureStatus>, ReadOnlyRpcError> {
        Ok(Some(ReadOnlySignatureStatus::new(
            cluster(),
            program_id(),
            mint(),
            token_account(),
            operation_id(),
            READ_ONLY_SIGNATURE,
            self.current_slot.saturating_sub(1),
            RpcCommitmentLevel::Finalized,
        )))
    }
}

fn rpc_observation(source: &str, slot: u64) -> RpcObservation {
    RpcObservation::new(
        source,
        cluster(),
        program_id(),
        mint(),
        token_account(),
        operation_id(),
        READ_ONLY_SIGNATURE,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn expected_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        cluster(),
        program_id(),
        mint(),
        token_account(),
        operation_id(),
        RpcCommitmentLevel::Confirmed,
    )
}

fn cluster() -> ClusterId {
    ClusterId::new("devnet").expect("static cluster should validate")
}

fn program_id() -> ProgramId {
    ProgramId::new("RoxAnchorProgram111111111111111111111111")
        .expect("static program id should validate")
}

fn mint() -> MintId {
    MintId::new("RoxMint111111111111111111111111111111111").expect("static mint should validate")
}

fn token_account() -> TokenAccountId {
    TokenAccountId::new("RoxTokenAccount1111111111111111111111")
        .expect("static token account should validate")
}

fn operation_id() -> OperationId {
    OperationId::new("op-read-only-rpc-0001").expect("static operation id should validate")
}
