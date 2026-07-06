//! RO:WHAT — Tests BUILD_PLAN3 Phase 13 private ROX-to-ROC read-only RPC burn evidence.
//! RO:WHY — Proves reverse pilot verifies test ROX burn evidence without releasing internal ROC.
//! RO:INTERACTS — read-only RPC adapter trait, expected RPC binding, private readback verification.
//! RO:INVARIANTS — wrong mint/token account, stale evidence, and missing evidence block readiness.
//! RO:SECURITY — read-only only; no send, signing, wallet loading, ROC mutation, or settlement.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test private_rox_to_roc_readback.

#![forbid(unsafe_code)]

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    verify_private_testnet_read_only_rpc, ExpectedRpcBinding, PrivateTestnetReadOnlyRpcFinding,
    PrivateTestnetReadOnlyRpcTarget, ReadOnlyAccountStatus, ReadOnlyRpcAdapter, ReadOnlyRpcError,
    ReadOnlySignatureStatus, RpcCommitmentLevel, RpcProofConfig, RpcQuorumDecision,
};

const SIGNATURE: &str = "privaterox-to-roc-burn-signature-111111111111111111111";

#[derive(Clone, Debug)]
struct FakeRoxToRocReadback {
    source: &'static str,
    current_slot: u64,
    program_status: ReadOnlyAccountStatus,
    config_status: ReadOnlyAccountStatus,
    mint_status: ReadOnlyAccountStatus,
    token_status: ReadOnlyAccountStatus,
    signature_status: Option<ReadOnlySignatureStatus>,
}

impl FakeRoxToRocReadback {
    fn healthy(source: &'static str, current_slot: u64) -> Self {
        Self {
            source,
            current_slot,
            program_status: ReadOnlyAccountStatus::new(
                program_account(),
                true,
                Some(program_id()),
                current_slot,
            ),
            config_status: ReadOnlyAccountStatus::new(
                config_account(),
                true,
                Some(program_id()),
                current_slot,
            ),
            mint_status: ReadOnlyAccountStatus::new(
                mint_account(),
                true,
                Some(token_program_id()),
                current_slot,
            ),
            token_status: ReadOnlyAccountStatus::new(
                token_account_account(),
                true,
                Some(token_program_id()),
                current_slot,
            ),
            signature_status: Some(ReadOnlySignatureStatus::new(
                cluster(),
                program_id(),
                mint_id(),
                token_account_id(),
                operation_id(),
                SIGNATURE,
                current_slot,
                RpcCommitmentLevel::Finalized,
            )),
        }
    }
}

impl ReadOnlyRpcAdapter for FakeRoxToRocReadback {
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
    AccountId::new("PrivatePilotRoxBurnSourceAccount111").unwrap()
}

fn mint_id() -> MintId {
    MintId::new("TestOnlyPrivatePilotRoxMint111111111").unwrap()
}

fn token_account_id() -> TokenAccountId {
    TokenAccountId::new("PrivatePilotRoxBurnSourceToken111111").unwrap()
}

fn operation_id() -> OperationId {
    OperationId::new("private-rox-to-roc-op-0001").unwrap()
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
fn private_rox_to_roc_readback_accepts_matching_test_rox_burn_evidence_shape() {
    let adapters = vec![
        FakeRoxToRocReadback::healthy("pilot-rpc-a", 300),
        FakeRoxToRocReadback::healthy("pilot-rpc-b", 301),
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
fn private_rox_to_roc_readback_rejects_wrong_mint_or_token_account() {
    let mut rpc_a = FakeRoxToRocReadback::healthy("pilot-rpc-a", 300);
    let mut rpc_b = FakeRoxToRocReadback::healthy("pilot-rpc-b", 301);

    rpc_a.signature_status = Some(ReadOnlySignatureStatus::new(
        cluster(),
        program_id(),
        MintId::new("WrongPrivatePilotRoxBurnMint111111").unwrap(),
        token_account_id(),
        operation_id(),
        SIGNATURE,
        299,
        RpcCommitmentLevel::Finalized,
    ));
    rpc_b.signature_status = Some(ReadOnlySignatureStatus::new(
        cluster(),
        program_id(),
        mint_id(),
        TokenAccountId::new("WrongPrivatePilotBurnToken111111").unwrap(),
        operation_id(),
        SIGNATURE,
        300,
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
fn private_rox_to_roc_readback_rejects_stale_or_missing_evidence() {
    let mut stale_a = FakeRoxToRocReadback::healthy("pilot-rpc-a", 300);
    let mut stale_b = FakeRoxToRocReadback::healthy("pilot-rpc-b", 301);
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

    let mut missing_a = FakeRoxToRocReadback::healthy("pilot-rpc-a", 300);
    let mut missing_b = FakeRoxToRocReadback::healthy("pilot-rpc-b", 301);
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
