//! RO:WHAT — Tests BUILD_PLAN3 Phase 6 private testnet read-only RPC verification.
//! RO:WHY — Proves private pilot readback checks program/config/mint/token accounts and quorum without submission.
//! RO:INTERACTS — ReadOnlyRpcAdapter, PrivateTestnetReadOnlyRpcTarget, and existing RPC quorum review.
//! RO:INVARIANTS — missing, stale, mismatched, and disputed observations fail closed.
//! RO:SECURITY — fake adapters only; no live RPC, key loading, wallet, transaction, mint, burn, settlement, or ROC mutation.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test private_testnet_read_only_rpc.

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    verify_private_testnet_read_only_rpc, ExpectedRpcBinding, PrivateTestnetReadOnlyRpcFinding,
    PrivateTestnetReadOnlyRpcTarget, ReadOnlyAccountStatus, ReadOnlyRpcAdapter, ReadOnlyRpcError,
    ReadOnlySignatureStatus, RpcCommitmentLevel, RpcProofConfig, RpcQuorumDecision,
};

const SIGNATURE: &str = "sig-private-testnet-read-only-111111111111";

#[derive(Clone, Debug)]
struct FakePrivateTestnetReadOnlyRpc {
    source: &'static str,
    current_slot: u64,
    program_status: ReadOnlyAccountStatus,
    config_status: ReadOnlyAccountStatus,
    mint_status: ReadOnlyAccountStatus,
    token_status: ReadOnlyAccountStatus,
    signature_status: Option<ReadOnlySignatureStatus>,
}

impl FakePrivateTestnetReadOnlyRpc {
    fn healthy(source: &'static str, current_slot: u64, signature: &str) -> Self {
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
                signature,
                current_slot,
                RpcCommitmentLevel::Finalized,
            )),
        }
    }
}

impl ReadOnlyRpcAdapter for FakePrivateTestnetReadOnlyRpc {
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
    ClusterId::new("devnet").expect("cluster should validate")
}

fn program_id() -> ProgramId {
    ProgramId::new("RoxAnchorProgram111111111111111111111111").expect("program id should validate")
}

fn token_program_id() -> ProgramId {
    ProgramId::new("SplTokenProgram1111111111111111111111111")
        .expect("token program id should validate")
}

fn program_account() -> AccountId {
    AccountId::new("RoxProgramAccount1111111111111111111111").expect("account should validate")
}

fn config_account() -> AccountId {
    AccountId::new("RoxConfigAccount11111111111111111111111").expect("account should validate")
}

fn mint_account() -> AccountId {
    AccountId::new("RoxMintAccount1111111111111111111111111").expect("account should validate")
}

fn token_account_account() -> AccountId {
    AccountId::new("RoxTokenAccountReadOnly111111111111111").expect("account should validate")
}

fn mint_id() -> MintId {
    MintId::new("RoxMint111111111111111111111111111111111").expect("mint should validate")
}

fn token_account_id() -> TokenAccountId {
    TokenAccountId::new("RoxTokenAccount1111111111111111111111")
        .expect("token account should validate")
}

fn operation_id() -> OperationId {
    OperationId::new("op-private-testnet-read-only-0001").expect("operation should validate")
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
    PrivateTestnetReadOnlyRpcTarget::new(program_account(), Some(program_id()), 5)
        .with_config_account(config_account(), Some(program_id()))
        .with_mint_account(mint_account(), Some(token_program_id()))
        .with_token_account(token_account_account(), Some(token_program_id()))
}

#[test]
fn private_testnet_read_only_rpc_accepts_complete_fake_readback() {
    let adapters = vec![
        FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, SIGNATURE),
        FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, SIGNATURE),
    ];

    let review = verify_private_testnet_read_only_rpc(
        &adapters,
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::Ready));
    assert_eq!(review.current_slot, 101);
    assert!(review.program_account_checked);
    assert!(review.config_account_checked);
    assert!(review.mint_account_checked);
    assert!(review.token_account_checked);
    assert_eq!(review.quorum.quorum.decision, RpcQuorumDecision::Agreement);

    let report = review.redacted_report_lines().join("\n");

    assert!(report.contains("private_testnet_read_only_rpc: local_verification"));
    assert!(report.contains("ready: true"));
    assert!(report.contains("program_account_checked: true"));
    assert!(report.contains("config_account_checked: true"));
    assert!(report.contains("mint_account_checked: true"));
    assert!(report.contains("token_account_checked: true"));
    assert!(report.contains("transaction_submission: disabled"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));

    for forbidden in [
        "rpc submitted",
        "loaded wallet",
        "loaded keypair",
        "mint complete",
        "burn complete",
        "settlement complete",
        "access granted",
        "roc released",
    ] {
        assert!(
            !report.to_ascii_lowercase().contains(forbidden),
            "report must not contain unsafe wording: {forbidden}\n{report}"
        );
    }
}

#[test]
fn private_testnet_read_only_rpc_rejects_missing_program_account() {
    let mut rpc_a = FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, SIGNATURE);
    let mut rpc_b = FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, SIGNATURE);
    rpc_a.program_status.exists = false;
    rpc_b.program_status.exists = false;

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::ProgramAccountMissing));
}

#[test]
fn private_testnet_read_only_rpc_rejects_mint_owner_mismatch() {
    let mut rpc_a = FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, SIGNATURE);
    let mut rpc_b = FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, SIGNATURE);
    rpc_a.mint_status.owner_program_id = Some(program_id());
    rpc_b.mint_status.owner_program_id = Some(program_id());

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::MintOwnerMismatch));
}

#[test]
fn private_testnet_read_only_rpc_rejects_stale_token_account_readback() {
    let mut rpc_a = FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, SIGNATURE);
    let mut rpc_b = FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, SIGNATURE);
    rpc_a.token_status.slot = 90;
    rpc_b.token_status.slot = 90;

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::StaleTokenAccount));
}

#[test]
fn private_testnet_read_only_rpc_rejects_disputed_signature_quorum() {
    let rpc_a = FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, "sig-a-111111111111");
    let rpc_b = FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, "sig-b-222222222222");

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::RpcQuorumBlocked));
    assert_ne!(review.quorum.quorum.decision, RpcQuorumDecision::Agreement);
}

#[test]
fn phase14_private_testnet_read_only_rpc_rejects_missing_config_account_with_inspectable_report() {
    let mut rpc_a = FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, SIGNATURE);
    let mut rpc_b = FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, SIGNATURE);
    rpc_a.config_status.exists = false;
    rpc_a.config_status.owner_program_id = None;
    rpc_b.config_status.exists = false;
    rpc_b.config_status.owner_program_id = None;

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::ConfigAccountMissing));
    assert!(review.config_account_checked);
    assert_ne!(review.quorum.quorum.decision, RpcQuorumDecision::Rejected);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("ready: false"));
    assert!(report.contains("config_account_checked: true"));
    assert!(report.contains("config_account_missing"));
    assert!(report.contains("transaction_submission: disabled"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));

    for forbidden in [
        "settlement complete",
        "mint complete",
        "burn complete",
        "access granted",
        "roc released",
        "loaded wallet",
        "loaded keypair",
    ] {
        assert!(
            !report.to_ascii_lowercase().contains(forbidden),
            "report must not contain unsafe wording: {forbidden}\n{report}"
        );
    }
}

#[test]
fn phase14_private_testnet_read_only_rpc_rejects_token_account_owner_mismatch() {
    let mut rpc_a = FakePrivateTestnetReadOnlyRpc::healthy("rpc-a", 100, SIGNATURE);
    let mut rpc_b = FakePrivateTestnetReadOnlyRpc::healthy("rpc-b", 101, SIGNATURE);
    rpc_a.token_status.owner_program_id = Some(program_id());
    rpc_b.token_status.owner_program_id = Some(program_id());

    let review = verify_private_testnet_read_only_rpc(
        &[rpc_a, rpc_b],
        &target(),
        SIGNATURE,
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("fake read-only verification should not error");

    assert!(!review.ready);
    assert!(review.has_finding(PrivateTestnetReadOnlyRpcFinding::TokenAccountOwnerMismatch));

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("token_account_owner_mismatch"));
    assert!(report.contains("transaction_submission: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
}
