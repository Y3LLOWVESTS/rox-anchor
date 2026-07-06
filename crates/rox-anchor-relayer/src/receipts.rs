//! RO:WHAT — Local relayer receipt records and private-pilot receipt ledger.
//! RO:WHY — Captures dry-run outcomes and Phase 9 pilot audit trails without claiming settlement.
//! RO:INTERACTS — submit.rs, audit.rs, redaction.rs, and CLI receipt display.
//! RO:INVARIANTS — receipt IDs are unique; operation IDs match; live-send and production-settlement claims reject.
//! RO:SECURITY — no live RPC, wallet, transaction, mint, burn, ROC release, or settlement authority.
//! RO:TEST — covered by dry-run receipt tests and private-pilot receipt ledger tests.

use std::{collections::BTreeSet, fmt};

use rox_anchor_core::{IdempotencyKey, Nonce, OperationId};
use rox_anchor_proof::ReviewDecision;

const PILOT_RECEIPT_LEDGER_VERSION: &str = "pilot-receipt-ledger-v1";
const PILOT_RECEIPT_GENESIS_LINK: &str = "pilot-link-0000000000000000";
const MAX_PILOT_RECEIPT_ID_BYTES: usize = 96;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelayerReceiptStatus {
    DryRunAccepted,
    ProofBlocked,
    ProofRejected,
    ChallengeBlocked,
    Halted,
    RecoveryBlocked,
    DuplicateRequest,
    ReceiptCapacityReached,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerReceipt {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub target: String,
    pub status: RelayerReceiptStatus,
    pub proof_decision: ReviewDecision,
    pub attempts_used: u8,
    pub live_submission: bool,
}

impl RelayerReceipt {
    pub fn new(
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        target: impl Into<String>,
        status: RelayerReceiptStatus,
        proof_decision: ReviewDecision,
        attempts_used: u8,
    ) -> Self {
        Self {
            operation_id,
            idempotency_key,
            target: target.into(),
            status,
            proof_decision,
            attempts_used,
            live_submission: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PilotReceiptId(String);

impl PilotReceiptId {
    pub fn new(value: impl Into<String>) -> Result<Self, PilotReceiptLedgerError> {
        let value = value.into();
        let actual_bytes = value.len();

        if value.is_empty() {
            return Err(PilotReceiptLedgerError::InvalidReceiptId {
                reason: "empty".to_string(),
            });
        }

        if value.trim() != value {
            return Err(PilotReceiptLedgerError::InvalidReceiptId {
                reason: "outer_whitespace".to_string(),
            });
        }

        if actual_bytes > MAX_PILOT_RECEIPT_ID_BYTES {
            return Err(PilotReceiptLedgerError::InvalidReceiptId {
                reason: format!("too_long:{actual_bytes}>{MAX_PILOT_RECEIPT_ID_BYTES}"),
            });
        }

        if value.chars().any(char::is_control) {
            return Err(PilotReceiptLedgerError::InvalidReceiptId {
                reason: "control_byte".to_string(),
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PilotReceiptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PilotReceiptKind {
    ProofReview,
    RpcQuorum,
    CoordinatorDecision,
    TransactionSimulation,
    SendAuthorization,
    TransactionSignature,
    ReadbackVerification,
}

impl PilotReceiptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProofReview => "proof_review",
            Self::RpcQuorum => "rpc_quorum",
            Self::CoordinatorDecision => "coordinator_decision",
            Self::TransactionSimulation => "transaction_simulation",
            Self::SendAuthorization => "send_authorization",
            Self::TransactionSignature => "transaction_signature",
            Self::ReadbackVerification => "readback_verification",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotReceiptEntry {
    pub receipt_id: PilotReceiptId,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub kind: PilotReceiptKind,
    pub stage_label: String,
    pub outcome_label: String,
    pub target: String,
    pub prior_link: String,
    pub entry_link: String,
    pub transaction_signature: Option<String>,
    pub live_submission_claimed: bool,
    pub network_submitted: bool,
    pub production_settlement_claimed: bool,
}

impl PilotReceiptEntry {
    pub fn new<Stage, Outcome, Target, Prior>(
        receipt_id: PilotReceiptId,
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        kind: PilotReceiptKind,
        labels: (Stage, Outcome),
        target: Target,
        prior_link: Prior,
    ) -> Self
    where
        Stage: Into<String>,
        Outcome: Into<String>,
        Target: Into<String>,
        Prior: Into<String>,
    {
        let (stage_label, outcome_label) = labels;
        let mut entry = Self {
            receipt_id,
            operation_id,
            idempotency_key,
            kind,
            stage_label: stage_label.into(),
            outcome_label: outcome_label.into(),
            target: target.into(),
            prior_link: prior_link.into(),
            entry_link: String::new(),
            transaction_signature: None,
            live_submission_claimed: false,
            network_submitted: false,
            production_settlement_claimed: false,
        };
        entry.refresh_link();
        entry
    }

    pub fn with_transaction_signature(mut self, transaction_signature: impl Into<String>) -> Self {
        self.transaction_signature = Some(transaction_signature.into());
        self.refresh_link();
        self
    }

    pub fn with_live_submission_claimed(mut self, live_submission_claimed: bool) -> Self {
        self.live_submission_claimed = live_submission_claimed;
        self.refresh_link();
        self
    }

    pub fn with_network_submitted(mut self, network_submitted: bool) -> Self {
        self.network_submitted = network_submitted;
        self.refresh_link();
        self
    }

    pub fn with_production_settlement_claimed(
        mut self,
        production_settlement_claimed: bool,
    ) -> Self {
        self.production_settlement_claimed = production_settlement_claimed;
        self.refresh_link();
        self
    }

    pub fn refresh_link(&mut self) {
        self.entry_link = pilot_receipt_link(self);
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            format!("receipt_id={}", self.receipt_id),
            format!("kind={}", self.kind.as_str()),
            format!("operation_id={}", self.operation_id),
            format!("idempotency_key={}", self.idempotency_key),
            format!("stage_label={}", self.stage_label),
            format!("outcome_label={}", self.outcome_label),
            format!("target={}", redact_sensitive_value(&self.target)),
            format!("prior_link={}", self.prior_link),
            format!("entry_link={}", self.entry_link),
            format!(
                "transaction_signature={}",
                self.transaction_signature
                    .as_deref()
                    .map(redact_sensitive_value)
                    .unwrap_or_else(|| "none".to_string())
            ),
            format!("live_submission_claimed={}", self.live_submission_claimed),
            format!("network_submitted={}", self.network_submitted),
            format!(
                "production_settlement_claimed={}",
                self.production_settlement_claimed
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PilotReceiptLedgerError {
    InvalidReceiptId {
        reason: String,
    },
    DuplicateReceiptId {
        receipt_id: String,
    },
    OperationIdMismatch {
        expected: String,
        actual: String,
    },
    ChainLinkMismatch {
        expected_prior_link: String,
        actual_prior_link: String,
    },
    LiveSubmissionClaimWithoutSend {
        receipt_id: String,
    },
    ProductionSettlementClaim {
        receipt_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotReceiptLedger {
    expected_operation_id: OperationId,
    seen_receipt_ids: BTreeSet<PilotReceiptId>,
    entries: Vec<PilotReceiptEntry>,
    tip_link: String,
}

impl PilotReceiptLedger {
    pub fn new(expected_operation_id: OperationId) -> Self {
        Self {
            expected_operation_id,
            seen_receipt_ids: BTreeSet::new(),
            entries: Vec::new(),
            tip_link: PILOT_RECEIPT_GENESIS_LINK.to_string(),
        }
    }

    pub fn expected_operation_id(&self) -> &OperationId {
        &self.expected_operation_id
    }

    pub fn tip_link(&self) -> &str {
        &self.tip_link
    }

    pub fn entries(&self) -> &[PilotReceiptEntry] {
        &self.entries
    }

    pub fn append_entry(
        &mut self,
        mut entry: PilotReceiptEntry,
    ) -> Result<(), PilotReceiptLedgerError> {
        if self.seen_receipt_ids.contains(&entry.receipt_id) {
            return Err(PilotReceiptLedgerError::DuplicateReceiptId {
                receipt_id: entry.receipt_id.to_string(),
            });
        }

        if entry.operation_id != self.expected_operation_id {
            return Err(PilotReceiptLedgerError::OperationIdMismatch {
                expected: self.expected_operation_id.to_string(),
                actual: entry.operation_id.to_string(),
            });
        }

        if entry.prior_link != self.tip_link {
            return Err(PilotReceiptLedgerError::ChainLinkMismatch {
                expected_prior_link: self.tip_link.clone(),
                actual_prior_link: entry.prior_link.clone(),
            });
        }

        if entry.live_submission_claimed && !entry.network_submitted {
            return Err(PilotReceiptLedgerError::LiveSubmissionClaimWithoutSend {
                receipt_id: entry.receipt_id.to_string(),
            });
        }

        if entry.production_settlement_claimed {
            return Err(PilotReceiptLedgerError::ProductionSettlementClaim {
                receipt_id: entry.receipt_id.to_string(),
            });
        }

        entry.refresh_link();
        self.tip_link = entry.entry_link.clone();
        self.seen_receipt_ids.insert(entry.receipt_id.clone());
        self.entries.push(entry);

        Ok(())
    }

    pub fn redacted_report(&self) -> String {
        let mut lines = vec![
            format!("pilot_receipt_ledger={PILOT_RECEIPT_LEDGER_VERSION}"),
            format!("expected_operation_id={}", self.expected_operation_id),
            format!("entry_count={}", self.entries.len()),
            format!("tip_link={}", self.tip_link),
            "entries:".to_string(),
        ];

        for entry in &self.entries {
            lines.push("  - entry".to_string());
            for line in entry.redacted_report_lines() {
                lines.push(format!("    {line}"));
            }
        }

        lines.push("live_submission_default=false".to_string());
        lines.push("production_settlement_claim=false".to_string());

        lines.join("\n")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PilotIncidentReceiptStatus {
    Ready,
    MissingReceiptFile,
    ReceiptTamper,
    DuplicateReceipt,
    DuplicateOperationId,
    DuplicateIdempotencyKey,
    NonceReuse,
    MissingReadbackAfterSend,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotIncidentReceiptEvidence {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
    pub receipt_id: Option<PilotReceiptId>,
    pub receipt_file_present: bool,
    pub receipt_chain_valid: bool,
    pub duplicate_receipt: bool,
    pub duplicate_operation_id: bool,
    pub duplicate_idempotency_key: bool,
    pub nonce_reused: bool,
    pub network_submitted: bool,
    pub readback_present: bool,
}

impl PilotIncidentReceiptEvidence {
    pub fn new(operation_id: OperationId, idempotency_key: IdempotencyKey, nonce: Nonce) -> Self {
        Self {
            operation_id,
            idempotency_key,
            nonce,
            receipt_id: None,
            receipt_file_present: true,
            receipt_chain_valid: true,
            duplicate_receipt: false,
            duplicate_operation_id: false,
            duplicate_idempotency_key: false,
            nonce_reused: false,
            network_submitted: false,
            readback_present: true,
        }
    }

    pub fn with_receipt_id(mut self, receipt_id: PilotReceiptId) -> Self {
        self.receipt_id = Some(receipt_id);
        self
    }

    pub fn with_receipt_file_present(mut self, receipt_file_present: bool) -> Self {
        self.receipt_file_present = receipt_file_present;
        self
    }

    pub fn with_receipt_chain_valid(mut self, receipt_chain_valid: bool) -> Self {
        self.receipt_chain_valid = receipt_chain_valid;
        self
    }

    pub fn with_duplicate_receipt(mut self, duplicate_receipt: bool) -> Self {
        self.duplicate_receipt = duplicate_receipt;
        self
    }

    pub fn with_duplicate_operation_id(mut self, duplicate_operation_id: bool) -> Self {
        self.duplicate_operation_id = duplicate_operation_id;
        self
    }

    pub fn with_duplicate_idempotency_key(mut self, duplicate_idempotency_key: bool) -> Self {
        self.duplicate_idempotency_key = duplicate_idempotency_key;
        self
    }

    pub fn with_nonce_reused(mut self, nonce_reused: bool) -> Self {
        self.nonce_reused = nonce_reused;
        self
    }

    pub fn with_network_submitted(mut self, network_submitted: bool) -> Self {
        self.network_submitted = network_submitted;
        self
    }

    pub fn with_readback_present(mut self, readback_present: bool) -> Self {
        self.readback_present = readback_present;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PilotIncidentReceiptReview {
    pub status: PilotIncidentReceiptStatus,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
    pub receipt_id: Option<PilotReceiptId>,
    pub fail_safe: bool,
    pub receipt_file_present: bool,
    pub receipt_chain_valid: bool,
    pub duplicate_receipt: bool,
    pub duplicate_operation_id: bool,
    pub duplicate_idempotency_key: bool,
    pub nonce_reused: bool,
    pub network_submitted: bool,
    pub readback_present: bool,
    pub live_submission_claim: bool,
    pub production_settlement_claim: bool,
}

impl PilotIncidentReceiptReview {
    pub fn is_ready(&self) -> bool {
        self.status == PilotIncidentReceiptStatus::Ready
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            "phase14_incident_receipt_review: local_only".to_string(),
            format!("status: {:?}", self.status),
            format!("fail_safe: {}", self.fail_safe),
            format!("operation_id: {}", self.operation_id),
            format!("idempotency_key: {}", self.idempotency_key),
            format!("nonce: {}", self.nonce),
            format!(
                "receipt_id: {}",
                self.receipt_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<missing>".to_string())
            ),
            format!("receipt_file_present: {}", self.receipt_file_present),
            format!("receipt_chain_valid: {}", self.receipt_chain_valid),
            format!("duplicate_receipt: {}", self.duplicate_receipt),
            format!("duplicate_operation_id: {}", self.duplicate_operation_id),
            format!(
                "duplicate_idempotency_key: {}",
                self.duplicate_idempotency_key
            ),
            format!("nonce_reused: {}", self.nonce_reused),
            format!("network_submitted: {}", self.network_submitted),
            format!("readback_present: {}", self.readback_present),
            format!("live_submission_claim: {}", self.live_submission_claim),
            format!(
                "production_settlement_claim: {}",
                self.production_settlement_claim
            ),
            "operator_action: halt_or_recover_before_retry".to_string(),
            "wallet_key_loading: false".to_string(),
            "signing: false".to_string(),
            "internal_roc_mutation: disabled".to_string(),
            "settlement_claim: none".to_string(),
        ]
    }
}

pub fn review_pilot_incident_receipt(
    evidence: PilotIncidentReceiptEvidence,
) -> PilotIncidentReceiptReview {
    let status = pilot_incident_receipt_status(&evidence);
    let fail_safe = status != PilotIncidentReceiptStatus::Ready;

    PilotIncidentReceiptReview {
        status,
        operation_id: evidence.operation_id,
        idempotency_key: evidence.idempotency_key,
        nonce: evidence.nonce,
        receipt_id: evidence.receipt_id,
        fail_safe,
        receipt_file_present: evidence.receipt_file_present,
        receipt_chain_valid: evidence.receipt_chain_valid,
        duplicate_receipt: evidence.duplicate_receipt,
        duplicate_operation_id: evidence.duplicate_operation_id,
        duplicate_idempotency_key: evidence.duplicate_idempotency_key,
        nonce_reused: evidence.nonce_reused,
        network_submitted: evidence.network_submitted,
        readback_present: evidence.readback_present,
        live_submission_claim: false,
        production_settlement_claim: false,
    }
}

fn pilot_incident_receipt_status(
    evidence: &PilotIncidentReceiptEvidence,
) -> PilotIncidentReceiptStatus {
    if !evidence.receipt_file_present || evidence.receipt_id.is_none() {
        return PilotIncidentReceiptStatus::MissingReceiptFile;
    }

    if !evidence.receipt_chain_valid {
        return PilotIncidentReceiptStatus::ReceiptTamper;
    }

    if evidence.duplicate_receipt {
        return PilotIncidentReceiptStatus::DuplicateReceipt;
    }

    if evidence.duplicate_operation_id {
        return PilotIncidentReceiptStatus::DuplicateOperationId;
    }

    if evidence.duplicate_idempotency_key {
        return PilotIncidentReceiptStatus::DuplicateIdempotencyKey;
    }

    if evidence.nonce_reused {
        return PilotIncidentReceiptStatus::NonceReuse;
    }

    if evidence.network_submitted && !evidence.readback_present {
        return PilotIncidentReceiptStatus::MissingReadbackAfterSend;
    }

    PilotIncidentReceiptStatus::Ready
}

pub fn redact_sensitive_value(value: &str) -> String {
    if contains_sensitive_hint(value) {
        "[redacted-sensitive-value]".to_string()
    } else {
        value.to_string()
    }
}

fn pilot_receipt_link(entry: &PilotReceiptEntry) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;

    for part in [
        PILOT_RECEIPT_LEDGER_VERSION,
        entry.receipt_id.as_str(),
        entry.operation_id.as_str(),
        entry.idempotency_key.as_str(),
        entry.kind.as_str(),
        entry.stage_label.as_str(),
        entry.outcome_label.as_str(),
        entry.target.as_str(),
        entry.prior_link.as_str(),
        entry.transaction_signature.as_deref().unwrap_or("none"),
        bool_label(entry.live_submission_claimed),
        bool_label(entry.network_submitted),
        bool_label(entry.production_settlement_claimed),
    ] {
        hash = feed_link_hash(hash, part.as_bytes());
        hash = feed_link_hash(hash, b"\0");
    }

    format!("pilot-link-{hash:016x}")
}

fn feed_link_hash(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    hash
}

fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn contains_sensitive_hint(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    [
        "/users/",
        "/external/",
        ".json",
        ".pem",
        "keypair",
        "wallet",
        "secret",
        "mnemonic",
        "seed",
        "credential",
        "password",
        "bearer",
        "api-key",
        "private-key",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}
