//! RO:WHAT — Tests BUILD_PLAN3 Phase 12 private ROC-to-ROX read-only readback.
//! RO:WHY — Proves forward pilot receipt/readback evidence is verified without transaction submission.
//! RO:INTERACTS — read-only RPC adapters, account status, signature status, and quorum review.
//! RO:INVARIANTS — readback must match cluster, program ID, mint, token account, operation ID, and freshness.
//! RO:SECURITY — read-only only; no signing, wallet loading, RPC submission, mint, burn, ROC mutation, or settlement.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test private_roc_to_rox_readback.

#![forbid(unsafe_code)]

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    verify_private_testnet_read_only_rpc, ExpectedRpcBinding, PrivateTestnetReadOnlyRpcFinding,
    PrivateTestnetReadOnlyRpcTarget, ReadOnlyAccountStatus, ReadOnlyRpcAdapter, ReadOnlyRpcError,
    ReadOnlySignatureStatus, RpcCommitmentLevel, RpcProofConfig, RpcQuorumDecision,
};

const SIGNATURE: &str = "private-roc-to-rox-readback-signature-0001";

#[derive(Clone, Debug)]
struct FakeRocToRoxReadback {
    source: String,
    current_slot: u64,
    program_status: ReadOnlyAccountStatus,
    config_status: ReadOnlyAccountStatus,
    mint_status: ReadOnlyAccountStatus,
    token_status: ReadOnlyAccountStatus,
    signature_status: Option<ReadOnlySignatureStatus>,
}

impl FakeRocToRoxReadback {
    fn healthy(source: &str, current_slot: u64) -> Self {
        Self {
            source: source.to_owned(),
            current_slot,
            program_status: ReadOnlyAccountStatus::new(
                program_account(),
                true,
                Some(program_id()),
                current_slot - 1,
            ),
            config_status: ReadOnlyAccountStatus::new(
                config_account(),
                true,
                Some(program_id()),
                current_slot - 1,
            ),
            mint_status: ReadOnlyAccountStatus::new(
                mint_account(),
                true,
                Some(token_program_id()),
                current_slot - 1,
            ),
            token_status: ReadOnlyAccountStatus::new(
                token_account_account(),
                true,
                Some(token_program_id()),
                current_slot - 1,
            ),
            signature_status: Some(ReadOnlySignatureStatus::new(
                cluster(),
                program_id(),
                mint_id(),
                token_account_id(),
                operation_id(),
                SIGNATURE,
                current_slot - 1,
                RpcCommitmentLevel::Finalized,
            )),
        }
    }
}

impl ReadOnlyRpcAdapter for FakeRocToRoxReadback {
    fn source_name(&self) -> &str {
        &self.source
    }

    fn current_slot(&self) -> Result<u64, ReadOnlyRpcError> {
        Ok(self.current_slot)
    }

    fn account_status(
        &self,
        account: &AccountId,
    ) -> Result<ReadOnlyAccountStatus, ReadOnlyRpcError> {
        if account == &program_account() {
            Ok(self.program_status.clone())
        } else if account == &config_account() {
            Ok(self.config_status.clone())
        } else if account == &mint_account() {
            Ok(self.mint_status.clone())
        } else if account == &token_account_account() {
            Ok(self.token_status.clone())
        } else {
            Ok(ReadOnlyAccountStatus::new(
                account.clone(),
                false,
                None,
                self.current_slot,
            ))
        }
    }

    fn signature_status(
        &self,
        _signature: &str,
    ) -> Result<Option<ReadOnlySignatureStatus>, ReadOnlyRpcError> {
        Ok(self.signature_status.clone())
    }
}

fn cluster() -> ClusterId {
    ClusterId::new("devnet").unwrap()
}

fn program_id() -> ProgramId {
    ProgramId::new("PrivatePilotRoxAnchorProgram11111111").unwrap()
}

fn token_program_id() -> ProgramId {
    ProgramId::new("SplTokenProgram1111111111111111111111111").unwrap()
}

fn program_account() -> AccountId {
    AccountId::new("PrivatePilotRoxProgramAccount111111").unwrap()
}

fn config_account() -> AccountId {
    AccountId::new("PrivatePilotRoxConfigAccount111111").unwrap()
}

fn mint_account() -> AccountId {
    AccountId::new("PrivatePilotTestRoxMintAccount1111").unwrap()
}

fn token_account_account() -> AccountId {
    AccountId::new("PrivatePilotRoxRecipientAccount1111").unwrap()
}

fn mint_id() -> MintId {
    MintId::new("TestOnlyPrivatePilotRoxMint111111111").unwrap()
}

fn token_account_id() -> TokenAccountId {
    TokenAccountId::new("PrivatePilotRoxRecipientToken1111111").unwrap()
}

fn operation_id() -> OperationId {
    OperationId::new("private-roc-to-rox-op-0001").unwrap()
}

fn expected_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        cluster(),
        program_id(),
        mint_id(),
        token_account_id(),
        operation_id(),
        RpcCommitmentLevel::Confirmed,
    )
}

fn target() -> PrivateTestnetReadOnlyRpcTarget {
    PrivateTestnetReadOnlyRpcTarget::new(program_account(), Some(program_id()), 10)
        .with_config_account(config_account(), Some(program_id()))
        .with_mint_account(mint_account(), Some(token_program_id()))
        .with_token_account(token_account_account(), Some(token_program_id()))
}

#[test]
fn private_roc_to_rox_readback_accepts_matching_test_rox_receipt_shape() {
    let adapters = vec![
        FakeRocToRoxReadback::healthy("pilot-rpc-a", 200),
        FakeRocToRoxReadback::healthy("pilot-rpc-b", 201),
    ];

    let review = verify_private_testnet_read_only_rpc(
        &adapters,
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake readback should not error");

    assert!(review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::Ready));
    assert_eq!(review.quorum.quorum.decision, RpcQuorumDecision::Agreement);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("private_testnet_read_only_rpc: local_verification"));
    assert!(report.contains("ready: true"));
    assert!(report.contains("quorum_decision: Agreement"));
    assert!(report.contains("transaction_submission: disabled"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
}

#[test]
fn private_roc_to_rox_readback_rejects_wrong_mint_or_token_account() {
    let mut rpc_a = FakeRocToRoxReadback::healthy("pilot-rpc-a", 200);
    let mut rpc_b = FakeRocToRoxReadback::healthy("pilot-rpc-b", 201);

    rpc_a.signature_status = Some(ReadOnlySignatureStatus::new(
        cluster(),
        program_id(),
        MintId::new("WrongPrivatePilotRoxMint111111111").unwrap(),
        token_account_id(),
        operation_id(),
        SIGNATURE,
        199,
        RpcCommitmentLevel::Finalized,
    ));
    rpc_b.signature_status = Some(ReadOnlySignatureStatus::new(
        cluster(),
        program_id(),
        mint_id(),
        TokenAccountId::new("WrongPrivatePilotTokenAccount11111").unwrap(),
        operation_id(),
        SIGNATURE,
        200,
        RpcCommitmentLevel::Finalized,
    ));

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake readback should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::RpcQuorumBlocked));
    assert_eq!(review.quorum.quorum.decision, RpcQuorumDecision::Rejected);
}

#[test]
fn private_roc_to_rox_readback_rejects_stale_or_missing_evidence() {
    let mut stale_a = FakeRocToRoxReadback::healthy("pilot-rpc-a", 200);
    let mut stale_b = FakeRocToRoxReadback::healthy("pilot-rpc-b", 201);
    stale_a.signature_status.as_mut().unwrap().slot = 1;
    stale_b.signature_status.as_mut().unwrap().slot = 1;

    let stale = verify_private_testnet_read_only_rpc(
        &[stale_a, stale_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 10),
    )
    .expect("fake stale readback should not error");

    assert!(!stale.ready);
    assert!(stale.has_finding(PrivateTestnetReadOnlyRpcFinding::RpcQuorumBlocked));
    assert_eq!(stale.quorum.quorum.decision, RpcQuorumDecision::Rejected);

    let mut missing_a = FakeRocToRoxReadback::healthy("pilot-rpc-a", 200);
    let mut missing_b = FakeRocToRoxReadback::healthy("pilot-rpc-b", 201);
    missing_a.signature_status = None;
    missing_b.signature_status = None;

    let missing = verify_private_testnet_read_only_rpc(
        &[missing_a, missing_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake missing readback should not error");

    assert!(!missing.ready);
    assert!(missing.has_finding(PrivateTestnetReadOnlyRpcFinding::RpcQuorumBlocked));
    assert_eq!(
        missing.quorum.quorum.decision,
        RpcQuorumDecision::MissingEvidence
    );
}
