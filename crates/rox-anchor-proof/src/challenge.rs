// RO:WHAT — Challenge-window posture and timing review for the local proof validator.
// RO:WHY — Preserves delayed, challengeable, failure-closed proof language before runtime exists.
// RO:INTERACTS — package, validate, quorum, and recovery local review modules.
// RO:INVARIANTS — Challenge labels are evidence-review posture only; local challenge review is not finality.
// RO:SECURITY — No skipped challenge window, no coordinator finality, no relayer finality, no bridge runtime, no settlement behavior.
// RO:TEST — Static Phase 4 checker and local unit-test source only.
//
// ROX-ANCHOR:FUTURE-GATED-CONTEXT
//
// This local validator does not authorize runtime.

/// Challenge gate posture labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengeGatePosture {
    NotOpened,
    Closed,
    Open,
    Challenged,
    Accepted,
    Rejected,
    Expired,
    Halted,
}

/// Challenge findings for local review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengeReviewFinding {
    EvidenceIncomplete,
    ChallengeOpen,
    QuorumDisputed,
    ReplayRejected,
    Halted,
}

/// Local-only challenge window review result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChallengeWindowReview {
    pub posture: ChallengeGatePosture,
    pub finding: Option<ChallengeReviewFinding>,
}

pub type ChallengeWindowSkeleton = ChallengeWindowReview;

impl ChallengeWindowReview {
    pub fn permits_finality_by_itself(&self) -> bool {
        false
    }

    pub fn is_runtime_authorized(&self) -> bool {
        false
    }
}

/// Local-only challenge-window timing input.
///
/// Slots are local review labels only. They are not chain truth, not runtime
/// authority, and not settlement evidence by themselves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChallengeWindowTiming {
    pub opened_at_slot: Option<u64>,
    pub observed_at_slot: u64,
    pub review_delay_slots: u64,
    pub expires_after_slots: u64,
}

impl ChallengeWindowTiming {
    pub fn unopened(observed_at_slot: u64) -> Self {
        Self {
            opened_at_slot: None,
            observed_at_slot,
            review_delay_slots: 0,
            expires_after_slots: 0,
        }
    }

    pub fn opened(
        opened_at_slot: u64,
        observed_at_slot: u64,
        review_delay_slots: u64,
        expires_after_slots: u64,
    ) -> Self {
        Self {
            opened_at_slot: Some(opened_at_slot),
            observed_at_slot,
            review_delay_slots,
            expires_after_slots,
        }
    }

    pub fn elapsed_slots(self) -> Option<u64> {
        self.opened_at_slot
            .map(|opened_at_slot| self.observed_at_slot.saturating_sub(opened_at_slot))
    }

    pub fn review_delay_elapsed(self) -> bool {
        self.elapsed_slots()
            .map(|elapsed| elapsed >= self.review_delay_slots)
            .unwrap_or(false)
    }

    pub fn is_expired(self) -> bool {
        self.elapsed_slots()
            .map(|elapsed| self.expires_after_slots > 0 && elapsed >= self.expires_after_slots)
            .unwrap_or(false)
    }
}

/// Local-only challenge-window review decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengeWindowReviewDecision {
    ValidForLocalReviewOnly,
    EvidenceIncomplete,
    ChallengeOpen,
    ReviewRejected,
    RuntimeNotAuthorized,
}

/// Challenge-window clock findings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChallengeWindowClockFinding {
    WindowNotOpened,
    WindowOpen,
    ReviewDelayNotElapsed,
    ReviewDelayElapsed,
    WindowExpired,
    ChallengeResolved,
    ChallengeAcceptedRejected,
    Halted,
    RuntimeAuthorizationRejected,
}

/// Deterministic local challenge-window clock review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChallengeWindowClockReview {
    pub posture: ChallengeGatePosture,
    pub timing: ChallengeWindowTiming,
    pub decision: ChallengeWindowReviewDecision,
    pub findings: Vec<ChallengeWindowClockFinding>,
}

impl ChallengeWindowClockReview {
    pub fn has_finding(&self, finding: ChallengeWindowClockFinding) -> bool {
        self.findings.contains(&finding)
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

    pub fn review_delay_elapsed(&self) -> bool {
        self.timing.review_delay_elapsed()
    }

    pub fn is_expired(&self) -> bool {
        self.timing.is_expired()
    }
}

pub fn review_challenge_posture(posture: ChallengeGatePosture) -> ChallengeWindowReview {
    let finding = match posture {
        ChallengeGatePosture::NotOpened => Some(ChallengeReviewFinding::EvidenceIncomplete),
        ChallengeGatePosture::Open => Some(ChallengeReviewFinding::ChallengeOpen),
        ChallengeGatePosture::Challenged => Some(ChallengeReviewFinding::QuorumDisputed),
        ChallengeGatePosture::Accepted => Some(ChallengeReviewFinding::ReplayRejected),
        ChallengeGatePosture::Halted => Some(ChallengeReviewFinding::Halted),
        ChallengeGatePosture::Closed
        | ChallengeGatePosture::Rejected
        | ChallengeGatePosture::Expired => None,
    };

    ChallengeWindowReview { posture, finding }
}

pub fn review_challenge_window_for_local_review_only(
    posture: ChallengeGatePosture,
    timing: ChallengeWindowTiming,
) -> ChallengeWindowClockReview {
    let mut findings = Vec::new();

    match posture {
        ChallengeGatePosture::NotOpened => {
            findings.push(ChallengeWindowClockFinding::WindowNotOpened);
            ChallengeWindowClockReview {
                posture,
                timing,
                decision: ChallengeWindowReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        ChallengeGatePosture::Closed | ChallengeGatePosture::Rejected => {
            findings.push(ChallengeWindowClockFinding::ChallengeResolved);
            ChallengeWindowClockReview {
                posture,
                timing,
                decision: ChallengeWindowReviewDecision::ValidForLocalReviewOnly,
                findings,
            }
        }
        ChallengeGatePosture::Open | ChallengeGatePosture::Challenged => {
            findings.push(ChallengeWindowClockFinding::WindowOpen);

            if timing.is_expired() {
                findings.push(ChallengeWindowClockFinding::WindowExpired);
                return ChallengeWindowClockReview {
                    posture,
                    timing,
                    decision: ChallengeWindowReviewDecision::EvidenceIncomplete,
                    findings,
                };
            }

            if timing.review_delay_elapsed() {
                findings.push(ChallengeWindowClockFinding::ReviewDelayElapsed);
            } else {
                findings.push(ChallengeWindowClockFinding::ReviewDelayNotElapsed);
            }

            ChallengeWindowClockReview {
                posture,
                timing,
                decision: ChallengeWindowReviewDecision::ChallengeOpen,
                findings,
            }
        }
        ChallengeGatePosture::Accepted => {
            findings.push(ChallengeWindowClockFinding::ChallengeAcceptedRejected);
            ChallengeWindowClockReview {
                posture,
                timing,
                decision: ChallengeWindowReviewDecision::ReviewRejected,
                findings,
            }
        }
        ChallengeGatePosture::Expired => {
            findings.push(ChallengeWindowClockFinding::WindowExpired);
            ChallengeWindowClockReview {
                posture,
                timing,
                decision: ChallengeWindowReviewDecision::EvidenceIncomplete,
                findings,
            }
        }
        ChallengeGatePosture::Halted => {
            findings.push(ChallengeWindowClockFinding::Halted);
            ChallengeWindowClockReview {
                posture,
                timing,
                decision: ChallengeWindowReviewDecision::ReviewRejected,
                findings,
            }
        }
    }
}

pub fn challenge_window_review_authorizes_runtime() -> bool {
    false
}

pub fn challenge_window_review_is_finality() -> bool {
    false
}

pub fn challenge_window_review_is_settlement() -> bool {
    false
}
