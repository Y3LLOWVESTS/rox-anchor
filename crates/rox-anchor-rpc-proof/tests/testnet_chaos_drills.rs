//! RO:WHAT — Phase 14 testnet chaos drills for read-only RPC proof behavior.
//! RO:WHY — Proves RPC outage, missing accounts, stale slots, binding tamper, and post-send readback failures fail safely.
//! RO:INTERACTS — ReadOnlyRpcAdapter, review_read_only_rpc_adapters, review_rpc_observations.
//! RO:INVARIANTS — RPC chaos never fabricates agreement, finality, submission success, mint, burn, or settlement.
//! RO:SECURITY — fake in-memory adapters only; no live RPC, wallet, key loading, transaction send, or value movement.
//! RO:TEST — cargo test -p rox-anchor-rpc-proof --test testnet_chaos_drills.

use rox_anchor_core::{AccountId, ClusterId, MintId, OperationId, ProgramId, TokenAccountId};
use rox_anchor_rpc_proof::{
    read_only_account_status, review_read_only_rpc_adapters, review_readback_after_send,
    review_rpc_observations, ExpectedRpcBinding, ReadOnlyAccountStatus, ReadOnlyRpcAdapter,
    ReadOnlyRpcError, ReadOnlySignatureStatus, ReadbackAfterSendStatus, RpcCommitmentLevel,
    RpcObservation, RpcProofConfig, RpcQuorumDecision, RpcQuorumFindingCode,
};

#[derive(Clone, Debug)]
struct ChaosReadOnlyRpc {
    source: &'static str,
    current_slot: u64,
    slot_unavailable: bool,
    account_exists: bool,
    signature_status: Option<ReadOnlySignatureStatus>,
}

impl ChaosReadOnlyRpc {
    fn healthy(
        source: &'static str,
        current_slot: u64,
        signature_status: Option<ReadOnlySignatureStatus>,
    ) -> Self {
        Self {
            source,
            current_slot,
            slot_unavailable: false,
            account_exists: true,
            signature_status,
        }
    }

    fn outage(source: &'static str) -> Self {
        Self {
            source,
            current_slot: 0,
            slot_unavailable: true,
            account_exists: false,
            signature_status: None,
        }
    }

    fn missing_account(source: &'static str, current_slot: u64) -> Self {
        Self {
            source,
            current_slot,
            slot_unavailable: false,
            account_exists: false,
            signature_status: None,
        }
    }
}

impl ReadOnlyRpcAdapter for ChaosReadOnlyRpc {
    fn source_name(&self) -> &str {
        self.source
    }

    fn current_slot(&self) -> Result<u64, ReadOnlyRpcError> {
        if self.slot_unavailable {
            return Err(ReadOnlyRpcError::Unavailable {
                source: self.source.to_owned(),
                reason: "phase13 rpc outage drill".to_owned(),
            });
        }

        Ok(self.current_slot)
    }

    fn account_status(
        &self,
        account: &AccountId,
    ) -> Result<ReadOnlyAccountStatus, ReadOnlyRpcError> {
        Ok(ReadOnlyAccountStatus::new(
            account.clone(),
            self.account_exists,
            self.account_exists.then(program_id),
            self.current_slot,
        ))
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
    ProgramId::new("RoxAnchorProgramPhase13Chaos111111111").expect("program id should validate")
}

fn mint() -> MintId {
    MintId::new("RoxMintPhase13Chaos111111111111111111").expect("mint should validate")
}

fn token_account() -> TokenAccountId {
    TokenAccountId::new("RoxTokenPhase13Chaos111111111111111")
        .expect("token account should validate")
}

fn operation_id() -> OperationId {
    OperationId::new("op-phase13-rpc-chaos-0001").expect("operation id should validate")
}

fn account_id() -> AccountId {
    AccountId::new("program-config-account-phase13-chaos").expect("account id should validate")
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

fn observation(source: &str, signature: &str, slot: u64) -> RpcObservation {
    RpcObservation::new(
        source,
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
fn rpc_outage_fails_without_fabricating_quorum_agreement() {
    let adapters = vec![ChaosReadOnlyRpc::outage("rpc-outage-a")];

    let error = review_read_only_rpc_adapters(
        &adapters,
        "sig-phase13-outage-111111111111",
        &expected_binding(),
        RpcProofConfig::new(1, 100),
    )
    .expect_err("outage should fail closed");

    assert_eq!(
        error,
        ReadOnlyRpcError::Unavailable {
            source: "rpc-outage-a".to_owned(),
            reason: "phase13 rpc outage drill".to_owned(),
        }
    );
}

#[test]
fn missing_program_account_is_visible_and_does_not_create_evidence() {
    let adapter = ChaosReadOnlyRpc::missing_account("rpc-missing-account", 75);

    let account =
        read_only_account_status(&adapter, &account_id()).expect("account status should read");

    assert!(!account.exists);
    assert_eq!(account.owner_program_id, None);
    assert_eq!(account.slot, 75);

    let adapters = vec![adapter];
    let review = review_read_only_rpc_adapters(
        &adapters,
        "sig-phase13-missing-account-111111111111",
        &expected_binding(),
        RpcProofConfig::new(1, 100),
    )
    .expect("missing account drill should still return a local report");

    assert_eq!(review.current_slot, 75);
    assert_eq!(review.observations_checked, 0);
    assert_eq!(review.quorum.decision, RpcQuorumDecision::MissingEvidence);
    assert!(review
        .quorum
        .has_finding(RpcQuorumFindingCode::MissingEvidence));
}

#[test]
fn reorg_like_stale_read_only_signature_is_rejected() {
    let adapters = vec![
        ChaosReadOnlyRpc::healthy(
            "rpc-stale-a",
            120,
            Some(signature_status("sig-phase13-stale-111111111111", 90)),
        ),
        ChaosReadOnlyRpc::healthy(
            "rpc-stale-b",
            121,
            Some(signature_status("sig-phase13-stale-111111111111", 91)),
        ),
    ];

    let review = review_read_only_rpc_adapters(
        &adapters,
        "sig-phase13-stale-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 5),
    )
    .expect("stale read-only review should return a rejection report");

    assert_eq!(review.current_slot, 121);
    assert_eq!(review.quorum.decision, RpcQuorumDecision::Rejected);
    assert!(review
        .quorum
        .has_finding(RpcQuorumFindingCode::StaleEvidence));
}

#[test]
fn wrong_program_mint_and_token_account_are_rejected_as_binding_tamper() {
    let mut wrong_program = observation("rpc-wrong-program", "sig-phase13-bind-111111111111", 100);
    wrong_program.program_id =
        ProgramId::new("WrongProgramPhase13Chaos111111111111").expect("program id should validate");

    let mut wrong_mint = observation("rpc-wrong-mint", "sig-phase13-bind-111111111111", 100);
    wrong_mint.mint =
        MintId::new("WrongMintPhase13Chaos11111111111111").expect("mint should validate");

    let mut wrong_token = observation("rpc-wrong-token", "sig-phase13-bind-111111111111", 100);
    wrong_token.token_account = TokenAccountId::new("WrongTokenPhase13Chaos111111111111")
        .expect("token account should validate");

    let review = review_rpc_observations(
        &[wrong_program, wrong_mint, wrong_token],
        &expected_binding(),
        RpcProofConfig::new(2, 100),
        105,
    );

    assert_eq!(review.decision, RpcQuorumDecision::Rejected);
    assert!(review.has_finding(RpcQuorumFindingCode::ProgramIdMismatch));
    assert!(review.has_finding(RpcQuorumFindingCode::MintMismatch));
    assert!(review.has_finding(RpcQuorumFindingCode::TokenAccountMismatch));
}

#[test]
fn phase14_rpc_disagreement_during_readback_remains_disputed_and_non_successful() {
    let adapters = vec![
        ChaosReadOnlyRpc::healthy(
            "rpc-dispute-a",
            140,
            Some(signature_status("sig-phase14-dispute-a-111111111111", 140)),
        ),
        ChaosReadOnlyRpc::healthy(
            "rpc-dispute-b",
            141,
            Some(signature_status("sig-phase14-dispute-b-222222222222", 141)),
        ),
    ];

    let quorum = review_read_only_rpc_adapters(
        &adapters,
        "sig-phase14-dispute-request-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("disputed readback should still produce local report");

    assert_eq!(quorum.quorum.decision, RpcQuorumDecision::Disputed);

    let review = review_readback_after_send(true, &quorum);

    assert_eq!(review.status, ReadbackAfterSendStatus::DisputedReadback);
    assert!(review.fail_safe);
    assert!(review.network_submitted);
    assert!(review.readback_present);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("phase14_readback_after_send_review: local_only"));
    assert!(report.contains("status: DisputedReadback"));
    assert!(report.contains("fail_safe: true"));
    assert!(report.contains("success_claim: false"));
    assert!(report.contains("finality_claim: false"));
    assert!(report.contains("settlement_claim: none"));
    assert!(report.contains("transaction_submission: not_performed_by_rpc_proof"));
    assert!(report.contains("wallet_key_loading: disabled"));
    assert!(report.contains("signing: disabled"));
    assert!(report.contains("internal_roc_mutation: disabled"));
}

#[test]
fn phase14_readback_missing_after_send_fails_safe_without_finality_or_settlement_claim() {
    let adapters = vec![
        ChaosReadOnlyRpc::healthy("rpc-missing-readback-a", 150, None),
        ChaosReadOnlyRpc::healthy("rpc-missing-readback-b", 151, None),
    ];

    let quorum = review_read_only_rpc_adapters(
        &adapters,
        "sig-phase14-missing-readback-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 100),
    )
    .expect("missing readback should still produce local report");

    assert_eq!(quorum.observations_checked, 0);
    assert_eq!(quorum.quorum.decision, RpcQuorumDecision::MissingEvidence);

    let review = review_readback_after_send(true, &quorum);

    assert_eq!(
        review.status,
        ReadbackAfterSendStatus::MissingReadbackAfterSend
    );
    assert!(review.fail_safe);
    assert!(review.network_submitted);
    assert!(!review.readback_present);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);

    let report = review.redacted_report_lines().join("\n");
    assert!(report.contains("network_submitted: true"));
    assert!(report.contains("readback_present: false"));
    assert!(report.contains("observations_checked: 0"));
    assert!(report.contains("quorum_decision: MissingEvidence"));
    assert!(report.contains("success_claim: false"));
    assert!(report.contains("finality_claim: false"));
    assert!(report.contains("settlement_claim: none"));

    for forbidden in [
        "settlement complete",
        "finality: confirmed",
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
fn phase14_rejected_readback_after_send_fails_safe_without_settlement_claim() {
    let adapters = vec![
        ChaosReadOnlyRpc::healthy(
            "rpc-rejected-stale-a",
            200,
            Some(signature_status(
                "sig-phase14-rejected-stale-111111111111",
                100,
            )),
        ),
        ChaosReadOnlyRpc::healthy(
            "rpc-rejected-stale-b",
            201,
            Some(signature_status(
                "sig-phase14-rejected-stale-111111111111",
                101,
            )),
        ),
    ];

    let quorum = review_read_only_rpc_adapters(
        &adapters,
        "sig-phase14-rejected-stale-111111111111",
        &expected_binding(),
        RpcProofConfig::new(2, 5),
    )
    .expect("rejected stale readback should still produce local report");

    assert_eq!(quorum.quorum.decision, RpcQuorumDecision::Rejected);
    assert!(quorum
        .quorum
        .has_finding(RpcQuorumFindingCode::StaleEvidence));

    let review = review_readback_after_send(true, &quorum);

    assert_eq!(review.status, ReadbackAfterSendStatus::RejectedReadback);
    assert!(review.fail_safe);
    assert!(review.readback_present);
    assert!(!review.success_claim);
    assert!(!review.finality_claim);
    assert!(!review.settlement_claim);
}
