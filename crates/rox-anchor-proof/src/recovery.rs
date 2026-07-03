// RO:WHAT — Halt and recovery posture review for the local proof validator.
// RO:WHY — Names recovery review cases while preventing hidden issue, hidden value movement, or manual settlement paths.
// RO:INTERACTS — package, validate, and challenge local review modules.
// RO:INVARIANTS — Recovery review is not value movement and does not authorize runtime.
// RO:SECURITY — No direct ledger mutation, no wallet call, no bridge runtime, no deployment, no staking, no liquidity, no external settlement.
// RO:TEST — Static Phase 4 checker and local unit-test source only.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local validator does not authorize runtime.

/// Recovery case labels for non-runtime review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryCaseKind {
    NotRequired,
    EvidenceMismatch,
    ChallengeAccepted,
    HaltedForReview,
    OperatorReviewRequired,
    Abandoned,
}

/// Halt posture for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltPosture {
    Unknown,
    NotHalted,
    Halted,
}

/// Recovery posture for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryPosture {
    Unknown,
    NotRequired,
    ReviewRequired,
    Queued,
    Halted,
}

/// Recovery findings for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryReviewFinding {
    EvidenceIncomplete,
    Halted,
    RecoveryReviewRequired,
}

/// Local-only recovery review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryReview {
    pub case_kind: RecoveryCaseKind,
    pub recovery_posture: RecoveryPosture,
    pub halt_posture: HaltPosture,
    pub finding: Option<RecoveryReviewFinding>,
}

pub type RecoveryReviewSkeleton = RecoveryReview;

impl RecoveryReview {
    pub fn is_hidden_value_path(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }
}

/// Local-only recovery action intent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryActionIntent {
    NoActionRequired,
    QueueReview,
    RejectEvidence,
    AbandonReview,
    KeepHalted,
}

/// Local halt/recovery review decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltRecoveryReviewDecision {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ReviewRejected,
    Halted,
    RuntimeNotAuthorized,
}

/// Local halt/recovery findings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HaltRecoveryReviewFinding {
    HaltUnknown,
    Halted,
    RecoveryReviewRequired,
    RecoveryQueued,
    EvidenceMismatch,
    ChallengeAcceptedForReview,
    OperatorReviewRequired,
    Abandoned,
    HiddenValuePathRejected,
    RuntimeAuthorizationRejected,
}

/// Local-only halt/recovery review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HaltRecoveryReview {
    pub halt_posture: HaltPosture,
    pub recovery_posture: RecoveryPosture,
    pub case_kind: RecoveryCaseKind,
    pub action_intent: RecoveryActionIntent,
    pub decision: HaltRecoveryReviewDecision,
    pub findings: Vec<HaltRecoveryReviewFinding>,
}

impl HaltRecoveryReview {
    pub fn has_finding(&self, finding: HaltRecoveryReviewFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn is_hidden_value_path(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }

    pub fn is_finality_claim(&self) -> bool {
        false
    }

    pub fn is_settlement_claim(&self) -> bool {
        false
    }
}

pub fn review_halt_posture(posture: HaltPosture) -> Option<RecoveryReviewFinding> {
    match posture {
        HaltPosture::Unknown => Some(RecoveryReviewFinding::EvidenceIncomplete),
        HaltPosture::Halted => Some(RecoveryReviewFinding::Halted),
        HaltPosture::NotHalted => None,
    }
}

pub fn review_recovery_posture(posture: RecoveryPosture) -> Option<RecoveryReviewFinding> {
    match posture {
        RecoveryPosture::Unknown => Some(RecoveryReviewFinding::EvidenceIncomplete),
        RecoveryPosture::ReviewRequired | RecoveryPosture::Queued => {
            Some(RecoveryReviewFinding::RecoveryReviewRequired)
        }
        RecoveryPosture::Halted => Some(RecoveryReviewFinding::Halted),
        RecoveryPosture::NotRequired => None,
    }
}

pub fn review_halt_recovery_for_local_review_only(
    halt_posture: HaltPosture,
    recovery_posture: RecoveryPosture,
    case_kind: RecoveryCaseKind,
) -> HaltRecoveryReview {
    let mut findings = Vec::new();

    match halt_posture {
        HaltPosture::Unknown => findings.push(HaltRecoveryReviewFinding::HaltUnknown),
        HaltPosture::Halted => findings.push(HaltRecoveryReviewFinding::Halted),
        HaltPosture::NotHalted => {}
    }

    match recovery_posture {
        RecoveryPosture::Unknown => {
            findings.push(HaltRecoveryReviewFinding::RecoveryReviewRequired)
        }
        RecoveryPosture::ReviewRequired => {
            findings.push(HaltRecoveryReviewFinding::RecoveryReviewRequired)
        }
        RecoveryPosture::Queued => findings.push(HaltRecoveryReviewFinding::RecoveryQueued),
        RecoveryPosture::Halted => findings.push(HaltRecoveryReviewFinding::Halted),
        RecoveryPosture::NotRequired => {}
    }

    match case_kind {
        RecoveryCaseKind::EvidenceMismatch => {
            findings.push(HaltRecoveryReviewFinding::EvidenceMismatch)
        }
        RecoveryCaseKind::ChallengeAccepted => {
            findings.push(HaltRecoveryReviewFinding::ChallengeAcceptedForReview)
        }
        RecoveryCaseKind::HaltedForReview => {
            findings.push(HaltRecoveryReviewFinding::Halted)
        }
        RecoveryCaseKind::OperatorReviewRequired => {
            findings.push(HaltRecoveryReviewFinding::OperatorReviewRequired)
        }
        RecoveryCaseKind::Abandoned => findings.push(HaltRecoveryReviewFinding::Abandoned),
        RecoveryCaseKind::NotRequired => {}
    }

    let action_intent = recovery_action_intent_for_local_review_only(
        halt_posture,
        recovery_posture,
        case_kind,
    );

    let decision = match action_intent {
        RecoveryActionIntent::NoActionRequired => {
            HaltRecoveryReviewDecision::ValidForLocalReviewOnly
        }
        RecoveryActionIntent::QueueReview => HaltRecoveryReviewDecision::EvidenceIncomplete,
        RecoveryActionIntent::RejectEvidence => HaltRecoveryReviewDecision::ReviewRejected,
        RecoveryActionIntent::AbandonReview => HaltRecoveryReviewDecision::ReviewRejected,
        RecoveryActionIntent::KeepHalted => HaltRecoveryReviewDecision::Halted,
    };

    HaltRecoveryReview {
        halt_posture,
        recovery_posture,
        case_kind,
        action_intent,
        decision,
        findings,
    }
}

pub fn recovery_action_intent_for_local_review_only(
    halt_posture: HaltPosture,
    recovery_posture: RecoveryPosture,
    case_kind: RecoveryCaseKind,
) -> RecoveryActionIntent {
    if matches!(halt_posture, HaltPosture::Halted)
        || matches!(recovery_posture, RecoveryPosture::Halted)
        || matches!(case_kind, RecoveryCaseKind::HaltedForReview)
    {
        return RecoveryActionIntent::KeepHalted;
    }

    if matches!(case_kind, RecoveryCaseKind::Abandoned) {
        return RecoveryActionIntent::AbandonReview;
    }

    if matches!(
        case_kind,
        RecoveryCaseKind::EvidenceMismatch | RecoveryCaseKind::ChallengeAccepted
    ) {
        return RecoveryActionIntent::RejectEvidence;
    }

    if matches!(
        recovery_posture,
        RecoveryPosture::Unknown | RecoveryPosture::ReviewRequired | RecoveryPosture::Queued
    ) || matches!(case_kind, RecoveryCaseKind::OperatorReviewRequired)
        || matches!(halt_posture, HaltPosture::Unknown)
    {
        return RecoveryActionIntent::QueueReview;
    }

    RecoveryActionIntent::NoActionRequired
}

pub fn halt_recovery_review_authorizes_runtime() -> bool {
    false
}

pub fn halt_recovery_review_touches_wallet() -> bool {
    false
}

pub fn halt_recovery_review_touches_ledger() -> bool {
    false
}

pub fn halt_recovery_review_is_finality() -> bool {
    false
}

pub fn halt_recovery_review_is_settlement() -> bool {
    false
}
