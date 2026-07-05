//! RO:WHAT — Tests BUILD_PLAN2 Phase 4 read-only RPC adapter boundary.
//! RO:WHY — Proves read-only-shaped slot/account/signature reads feed existing RPC quorum review.
//! RO:INTERACTS — ReadOnlyRpcAdapter, RpcObservation, ExpectedRpcBinding, and review_rpc_observations.
//! RO:INVARIANTS — fake adapter has no submit/sign/mint/burn behavior and cannot bypass quorum rules.
//! RO:SECURITY — tests use fake in-memory reads only; no live RPC, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-rpc-proof --test read_only_rpc_adapter.

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    read_only_account_status, review_read_only_rpc_adapters, ExpectedRpcBinding,
    ReadOnlyAccountStatus, ReadOnlyRpcAdapter, ReadOnlyRpcError, ReadOnlySignatureStatus,
    RpcCommitmentLevel, RpcProofConfig, RpcQuorumDecision, RpcQuorumFindingCode,
};

#[derive(Clone, Debug)]
struct FakeReadOnlyRpc {
    source: &'static str,
    current_slot: u64,
    account: ReadOnlyAccountStatus,
    signature_status: Option<ReadOnlySignatureStatus>,
}

impl FakeReadOnlyRpc {
    fn new(
        source: &'static str,
        current_slot: u64,
        signature_status: Option<ReadOnlySignatureStatus>,
    ) -> Self {
        Self {
            source,
            current_slot,
            account: ReadOnlyAccountStatus::new(
                account_id(),
                true,
                Some(program_id()),
                current_slot,
            ),
            signature_status,
        }
    }
}

impl ReadOnlyRpcAdapter for FakeReadOnlyRpc {
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
        let mut status = self.account.clone();
        status.account = account.clone();
        Ok(status)
    }

    fn signature_status(
        &self,
        _signature: &str,
    ) -> Result<Option<ReadOnlySignatureStatus>, ReadOnlyRpcError> {
        Ok(self.signature_status.clone())
    }
}

fn cluster() -> ClusterId {
    ClusterId::new("devnet").expect("cluster should validate")
}

fn program_id() -> ProgramId {
    ProgramId::new("RoxAnchorProgram111111111111111111111111").expect("program id should validate")
}

fn mint() -> MintId {
    MintId::new("RoxMint111111111111111111111111111111111").expect("mint should validate")
}

fn token_account() -> TokenAccountId {
    TokenAccountId::new("RoxTokenAccount1111111111111111111111")
        .expect("token account should validate")
}

fn operation_id() -> OperationId {
    OperationId::new("op-read-only-rpc-0001").expect("operation id should validate")
}

fn account_id() -> AccountId {
    AccountId::new("program-config-account-read-only-rpc").expect("account id should validate")
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

fn signature_status(signature: &str, slot: u64) -> ReadOnlySignatureStatus {
    ReadOnlySignatureStatus::new(
        cluster(),
        program_id(),
        mint(),
        token_account(),
        operation_id(),
        signature,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

#[test]
fn read_only_adapter_fetches_current_slot_and_account_status() {
    let adapter = FakeReadOnlyRpc::new(
        "rpc-a",
        50,
        Some(signature_status("sig-readonly-111111111111", 48)),
    );

    assert_eq!(adapter.current_slot().expect("slot should read"), 50);

    let status =
        read_only_account_status(&adapter, &account_id()).expect("account status should read");

    assert!(status.exists);
    assert_eq!(status.owner_program_id, Some(program_id()));
    assert_eq!(status.slot, 50);
}

#[test]
fn missing_read_only_signature_status_becomes_missing_evidence() {
    let adapters = vec![
        FakeReadOnlyRpc::new("rpc-a", 50, None),
        FakeReadOnlyRpc::new("rpc-b", 51, None),
    ];

    let review = review_read_only_rpc_adapters(
        &adapters,
        "sig-missing-readonly-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("read-only review should not fail");

    assert_eq!(review.current_slot, 51);
    assert_eq!(review.observations_checked, 0);
    assert_eq!(review.quorum.decision, RpcQuorumDecision::MissingEvidence);
    assert!(review
        .quorum
        .has_finding(RpcQuorumFindingCode::MissingEvidence));
}

#[test]
fn stale_read_only_signature_status_is_rejected_by_existing_quorum_rules() {
    let adapters = vec![
        FakeReadOnlyRpc::new(
            "rpc-a",
            50,
            Some(signature_status("sig-stale-111111111111", 10)),
        ),
        FakeReadOnlyRpc::new(
            "rpc-b",
            50,
            Some(signature_status("sig-stale-111111111111", 11)),
        ),
    ];

    let review = review_read_only_rpc_adapters(
        &adapters,
        "sig-stale-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 5),
    )
    .expect("read-only review should not fail");

    assert_eq!(review.quorum.decision, RpcQuorumDecision::Rejected);
    assert!(review
        .quorum
        .has_finding(RpcQuorumFindingCode::StaleEvidence));
}

#[test]
fn mismatched_read_only_signature_status_is_rejected_by_existing_quorum_rules() {
    let mut bad = signature_status("sig-mismatch-111111111111", 48);
    bad.cluster = ClusterId::new("wrong-devnet").expect("cluster should validate");

    let adapters = vec![FakeReadOnlyRpc::new("rpc-a", 50, Some(bad))];

    let review = review_read_only_rpc_adapters(
        &adapters,
        "sig-mismatch-111111111111",
        &expected_binding(),
        RpcProofConfig::new(1, 100),
    )
    .expect("read-only review should not fail");

    assert_eq!(review.quorum.decision, RpcQuorumDecision::Rejected);
    assert!(review
        .quorum
        .has_finding(RpcQuorumFindingCode::ClusterMismatch));
}

#[test]
fn disputed_read_only_signature_status_is_disputed_by_existing_quorum_rules() {
    let adapters = vec![
        FakeReadOnlyRpc::new(
            "rpc-a",
            50,
            Some(signature_status("sig-left-111111111111", 48)),
        ),
        FakeReadOnlyRpc::new(
            "rpc-b",
            51,
            Some(signature_status("sig-right-2222222222", 49)),
        ),
    ];

    let review = review_read_only_rpc_adapters(
        &adapters,
        "sig-disputed-readonly-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("read-only review should not fail");

    assert_eq!(review.current_slot, 51);
    assert_eq!(review.observations_checked, 2);
    assert_eq!(review.quorum.decision, RpcQuorumDecision::Disputed);
    assert!(review
        .quorum
        .has_finding(RpcQuorumFindingCode::RpcEquivocation));
}
