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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadbackAfterSendStatus {
    NotSubmitted,
    VerifiedReadback,
    MissingReadbackAfterSend,
    DisputedReadback,
    RejectedReadback,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadbackAfterSendReview {
    pub status: ReadbackAfterSendStatus,
    pub fail_safe: bool,
    pub network_submitted: bool,
    pub readback_present: bool,
    pub current_slot: u64,
    pub observations_checked: u16,
    pub quorum_decision: crate::RpcQuorumDecision,
    pub success_claim: bool,
    pub finality_claim: bool,
    pub settlement_claim: bool,
}

impl ReadbackAfterSendReview {
    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            "phase14_readback_after_send_review: local_only".to_string(),
            format!("status: {:?}", self.status),
            format!("fail_safe: {}", self.fail_safe),
            format!("network_submitted: {}", self.network_submitted),
            format!("readback_present: {}", self.readback_present),
            format!("current_slot: {}", self.current_slot),
            format!("observations_checked: {}", self.observations_checked),
            format!("quorum_decision: {:?}", self.quorum_decision),
            format!("success_claim: {}", self.success_claim),
            format!("finality_claim: {}", self.finality_claim),
            format!(
                "settlement_claim: {}",
                if self.settlement_claim {
                    "present"
                } else {
                    "none"
                }
            ),
            "transaction_submission: not_performed_by_rpc_proof".to_string(),
            "wallet_key_loading: disabled".to_string(),
            "signing: disabled".to_string(),
            "internal_roc_mutation: disabled".to_string(),
        ]
    }
}

pub fn review_readback_after_send(
    network_submitted: bool,
    review: &ReadOnlyRpcObservationReview,
) -> ReadbackAfterSendReview {
    let status = if !network_submitted {
        ReadbackAfterSendStatus::NotSubmitted
    } else {
        match review.quorum.decision {
            crate::RpcQuorumDecision::Agreement => ReadbackAfterSendStatus::VerifiedReadback,
            crate::RpcQuorumDecision::MissingEvidence => {
                ReadbackAfterSendStatus::MissingReadbackAfterSend
            }
            crate::RpcQuorumDecision::Disputed => ReadbackAfterSendStatus::DisputedReadback,
            crate::RpcQuorumDecision::Rejected => ReadbackAfterSendStatus::RejectedReadback,
        }
    };

    let fail_safe = matches!(
        status,
        ReadbackAfterSendStatus::MissingReadbackAfterSend
            | ReadbackAfterSendStatus::DisputedReadback
            | ReadbackAfterSendStatus::RejectedReadback
    );

    ReadbackAfterSendReview {
        status,
        fail_safe,
        network_submitted,
        readback_present: review.observations_checked > 0,
        current_slot: review.current_slot,
        observations_checked: review.observations_checked,
        quorum_decision: review.quorum.decision,
        success_claim: status == ReadbackAfterSendStatus::VerifiedReadback,
        finality_claim: false,
        settlement_claim: false,
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivateTestnetReadOnlyRpcFinding {
    Ready,
    ProgramAccountMissing,
    ConfigAccountMissing,
    MintAccountMissing,
    TokenAccountMissing,
    ProgramOwnerMismatch,
    ConfigOwnerMismatch,
    MintOwnerMismatch,
    TokenAccountOwnerMismatch,
    StaleProgramAccount,
    StaleConfigAccount,
    StaleMintAccount,
    StaleTokenAccount,
    RpcQuorumBlocked,
}

impl PrivateTestnetReadOnlyRpcFinding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ProgramAccountMissing => "program_account_missing",
            Self::ConfigAccountMissing => "config_account_missing",
            Self::MintAccountMissing => "mint_account_missing",
            Self::TokenAccountMissing => "token_account_missing",
            Self::ProgramOwnerMismatch => "program_owner_mismatch",
            Self::ConfigOwnerMismatch => "config_owner_mismatch",
            Self::MintOwnerMismatch => "mint_owner_mismatch",
            Self::TokenAccountOwnerMismatch => "token_account_owner_mismatch",
            Self::StaleProgramAccount => "stale_program_account",
            Self::StaleConfigAccount => "stale_config_account",
            Self::StaleMintAccount => "stale_mint_account",
            Self::StaleTokenAccount => "stale_token_account",
            Self::RpcQuorumBlocked => "rpc_quorum_blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateTestnetReadOnlyRpcTarget {
    pub program_account: AccountId,
    pub expected_program_owner: Option<ProgramId>,
    pub config_account: Option<AccountId>,
    pub expected_config_owner: Option<ProgramId>,
    pub mint_account: Option<AccountId>,
    pub expected_mint_owner: Option<ProgramId>,
    pub token_account: Option<AccountId>,
    pub expected_token_account_owner: Option<ProgramId>,
    pub max_account_age_slots: u64,
}

impl PrivateTestnetReadOnlyRpcTarget {
    pub fn new(
        program_account: AccountId,
        expected_program_owner: Option<ProgramId>,
        max_account_age_slots: u64,
    ) -> Self {
        Self {
            program_account,
            expected_program_owner,
            config_account: None,
            expected_config_owner: None,
            mint_account: None,
            expected_mint_owner: None,
            token_account: None,
            expected_token_account_owner: None,
            max_account_age_slots,
        }
    }

    pub fn with_config_account(
        mut self,
        account: AccountId,
        expected_owner: Option<ProgramId>,
    ) -> Self {
        self.config_account = Some(account);
        self.expected_config_owner = expected_owner;
        self
    }

    pub fn with_mint_account(
        mut self,
        account: AccountId,
        expected_owner: Option<ProgramId>,
    ) -> Self {
        self.mint_account = Some(account);
        self.expected_mint_owner = expected_owner;
        self
    }

    pub fn with_token_account(
        mut self,
        account: AccountId,
        expected_owner: Option<ProgramId>,
    ) -> Self {
        self.token_account = Some(account);
        self.expected_token_account_owner = expected_owner;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateTestnetReadOnlyRpcVerification {
    pub ready: bool,
    pub findings: Vec<PrivateTestnetReadOnlyRpcFinding>,
    pub current_slot: u64,
    pub program_account_checked: bool,
    pub config_account_checked: bool,
    pub mint_account_checked: bool,
    pub token_account_checked: bool,
    pub quorum: ReadOnlyRpcObservationReview,
}

impl PrivateTestnetReadOnlyRpcVerification {
    pub fn has_finding(&self, finding: PrivateTestnetReadOnlyRpcFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            "private_testnet_read_only_rpc: local_verification".to_string(),
            format!("ready: {}", self.ready),
            format!("current_slot: {}", self.current_slot),
            format!("program_account_checked: {}", self.program_account_checked),
            format!("config_account_checked: {}", self.config_account_checked),
            format!("mint_account_checked: {}", self.mint_account_checked),
            format!("token_account_checked: {}", self.token_account_checked),
            format!("observations_checked: {}", self.quorum.observations_checked),
            format!("quorum_decision: {:?}", self.quorum.quorum.decision),
            format!(
                "findings: {}",
                self.findings
                    .iter()
                    .map(|finding| finding.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            "transaction_submission: disabled".to_string(),
            "wallet_key_loading: disabled".to_string(),
            "signing: disabled".to_string(),
            "internal_roc_mutation: disabled".to_string(),
        ]
    }
}

pub fn verify_private_testnet_read_only_rpc<A: ReadOnlyRpcAdapter>(
    adapters: &[A],
    target: &PrivateTestnetReadOnlyRpcTarget,
    signature: &str,
    expected: &ExpectedRpcBinding,
    config: RpcProofConfig,
) -> Result<PrivateTestnetReadOnlyRpcVerification, ReadOnlyRpcError> {
    let mut findings = Vec::new();
    let mut current_slot = 0_u64;

    for adapter in adapters {
        let adapter_slot = adapter.current_slot()?;
        current_slot = current_slot.max(adapter_slot);

        let program_status = adapter.account_status(&target.program_account)?;
        inspect_read_only_account_status(
            &mut findings,
            adapter_slot,
            program_status,
            target.expected_program_owner.as_ref(),
            target.max_account_age_slots,
            PrivateTestnetReadOnlyRpcFinding::ProgramAccountMissing,
            PrivateTestnetReadOnlyRpcFinding::ProgramOwnerMismatch,
            PrivateTestnetReadOnlyRpcFinding::StaleProgramAccount,
        );

        if let Some(account) = target.config_account.as_ref() {
            let config_status = adapter.account_status(account)?;
            inspect_read_only_account_status(
                &mut findings,
                adapter_slot,
                config_status,
                target.expected_config_owner.as_ref(),
                target.max_account_age_slots,
                PrivateTestnetReadOnlyRpcFinding::ConfigAccountMissing,
                PrivateTestnetReadOnlyRpcFinding::ConfigOwnerMismatch,
                PrivateTestnetReadOnlyRpcFinding::StaleConfigAccount,
            );
        }

        if let Some(account) = target.mint_account.as_ref() {
            let mint_status = adapter.account_status(account)?;
            inspect_read_only_account_status(
                &mut findings,
                adapter_slot,
                mint_status,
                target.expected_mint_owner.as_ref(),
                target.max_account_age_slots,
                PrivateTestnetReadOnlyRpcFinding::MintAccountMissing,
                PrivateTestnetReadOnlyRpcFinding::MintOwnerMismatch,
                PrivateTestnetReadOnlyRpcFinding::StaleMintAccount,
            );
        }

        if let Some(account) = target.token_account.as_ref() {
            let token_status = adapter.account_status(account)?;
            inspect_read_only_account_status(
                &mut findings,
                adapter_slot,
                token_status,
                target.expected_token_account_owner.as_ref(),
                target.max_account_age_slots,
                PrivateTestnetReadOnlyRpcFinding::TokenAccountMissing,
                PrivateTestnetReadOnlyRpcFinding::TokenAccountOwnerMismatch,
                PrivateTestnetReadOnlyRpcFinding::StaleTokenAccount,
            );
        }
    }

    let quorum = review_read_only_rpc_adapters(adapters, signature, expected, config)?;
    current_slot = current_slot.max(quorum.current_slot);

    if quorum.quorum.decision != crate::RpcQuorumDecision::Agreement {
        push_private_testnet_read_only_rpc_finding(
            &mut findings,
            PrivateTestnetReadOnlyRpcFinding::RpcQuorumBlocked,
        );
    }

    if findings.is_empty() {
        findings.push(PrivateTestnetReadOnlyRpcFinding::Ready);
    }

    Ok(PrivateTestnetReadOnlyRpcVerification {
        ready: findings == vec![PrivateTestnetReadOnlyRpcFinding::Ready],
        findings,
        current_slot,
        program_account_checked: !adapters.is_empty(),
        config_account_checked: target.config_account.is_some() && !adapters.is_empty(),
        mint_account_checked: target.mint_account.is_some() && !adapters.is_empty(),
        token_account_checked: target.token_account.is_some() && !adapters.is_empty(),
        quorum,
    })
}

#[allow(clippy::too_many_arguments)]
fn inspect_read_only_account_status(
    findings: &mut Vec<PrivateTestnetReadOnlyRpcFinding>,
    adapter_slot: u64,
    status: ReadOnlyAccountStatus,
    expected_owner: Option<&ProgramId>,
    max_account_age_slots: u64,
    missing_finding: PrivateTestnetReadOnlyRpcFinding,
    owner_mismatch_finding: PrivateTestnetReadOnlyRpcFinding,
    stale_finding: PrivateTestnetReadOnlyRpcFinding,
) {
    if !status.exists {
        push_private_testnet_read_only_rpc_finding(findings, missing_finding);
        return;
    }

    if let Some(expected_owner) = expected_owner {
        if status.owner_program_id.as_ref() != Some(expected_owner) {
            push_private_testnet_read_only_rpc_finding(findings, owner_mismatch_finding);
        }
    }

    if adapter_slot.saturating_sub(status.slot) > max_account_age_slots {
        push_private_testnet_read_only_rpc_finding(findings, stale_finding);
    }
}

fn push_private_testnet_read_only_rpc_finding(
    findings: &mut Vec<PrivateTestnetReadOnlyRpcFinding>,
    finding: PrivateTestnetReadOnlyRpcFinding,
) {
    if !findings.contains(&finding) {
        findings.push(finding);
    }
}
