//! RO:WHAT — Shared halt, recovery, and kill-switch drill review for ROX Anchor.
//! RO:WHY — BUILD_PLAN2 Phase 12 needs one core-owned model for stopping unsafe progress.
//! RO:INTERACTS — AuthorityMap, operator roles, challenge/halt/recovery postures, coordinator, relayer, and CLI reports.
//! RO:INVARIANTS — halted, challenged, or recovery-blocked posture prevents acceptance, simulation, submission, and finalization.
//! RO:SECURITY — local/testnet drill model only; no keypair loading, signing, wallet, RPC, mint, burn, or settlement.
//! RO:TEST — covered by core kill-switch drill tests plus coordinator/relayer/CLI Phase 12 tests.

use crate::{
    AuthorityKeyId, AuthorityMap, ChallengePosture, HaltPosture, OperatorRole, RecoveryPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AnchorOperationalBlocker {
    None,
    Challenge,
    Halt,
    Recovery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AnchorOperationalPosture {
    pub challenge: ChallengePosture,
    pub halt: HaltPosture,
    pub recovery: RecoveryPosture,
}

impl AnchorOperationalPosture {
    pub fn new(challenge: ChallengePosture, halt: HaltPosture, recovery: RecoveryPosture) -> Self {
        Self {
            challenge,
            halt,
            recovery,
        }
    }

    pub fn clear() -> Self {
        Self::new(
            ChallengePosture::Clear,
            HaltPosture::Active,
            RecoveryPosture::NotRequired,
        )
    }

    pub fn halted() -> Self {
        Self::new(
            ChallengePosture::Clear,
            HaltPosture::Halted,
            RecoveryPosture::NotRequired,
        )
    }

    pub fn recovery_required() -> Self {
        Self::new(
            ChallengePosture::Clear,
            HaltPosture::Active,
            RecoveryPosture::Required,
        )
    }

    pub fn halted_recovery_required() -> Self {
        Self::new(
            ChallengePosture::Clear,
            HaltPosture::Halted,
            RecoveryPosture::Required,
        )
    }

    pub fn recovery_resolved() -> Self {
        Self::new(
            ChallengePosture::Clear,
            HaltPosture::ResumeEligible,
            RecoveryPosture::Resolved,
        )
    }

    pub fn primary_blocker(self) -> AnchorOperationalBlocker {
        if self.challenge.blocks_acceptance() {
            AnchorOperationalBlocker::Challenge
        } else if self.halt.blocks_acceptance() {
            AnchorOperationalBlocker::Halt
        } else if self.recovery.blocks_acceptance() {
            AnchorOperationalBlocker::Recovery
        } else {
            AnchorOperationalBlocker::None
        }
    }

    pub fn blocks_acceptance(self) -> bool {
        self.primary_blocker() != AnchorOperationalBlocker::None
    }

    pub fn blocks_simulation(self) -> bool {
        self.blocks_acceptance()
    }

    pub fn blocks_submission(self) -> bool {
        self.blocks_acceptance()
    }

    pub fn blocks_finalization(self) -> bool {
        self.blocks_acceptance()
    }
}

impl Default for AnchorOperationalPosture {
    fn default() -> Self {
        Self::clear()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KillSwitchDrillStage {
    BeforeProofAcceptance,
    AfterProofAcceptanceBeforeSimulation,
    AfterSimulationBeforeSubmission,
    AfterCappedTestnetSubmission,
}

impl KillSwitchDrillStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BeforeProofAcceptance => "before_proof_acceptance",
            Self::AfterProofAcceptanceBeforeSimulation => {
                "after_proof_acceptance_before_simulation"
            }
            Self::AfterSimulationBeforeSubmission => "after_simulation_before_submission",
            Self::AfterCappedTestnetSubmission => "after_capped_testnet_submission",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KillSwitchAction {
    Halt,
    Recover,
}

impl KillSwitchAction {
    pub fn required_role(self) -> OperatorRole {
        match self {
            Self::Halt => OperatorRole::HaltAuthority,
            Self::Recover => OperatorRole::RecoveryAuthority,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Halt => "halt",
            Self::Recover => "recover",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum KillSwitchDrillStatus {
    HaltAccepted,
    RecoveryAccepted,
    WrongAuthority,
    RecoveryRequiresHaltedState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KillSwitchDrillRequest {
    pub stage: KillSwitchDrillStage,
    pub action: KillSwitchAction,
    pub posture: AnchorOperationalPosture,
    pub presented_authority: AuthorityKeyId,
}

impl KillSwitchDrillRequest {
    pub fn new(
        stage: KillSwitchDrillStage,
        action: KillSwitchAction,
        posture: AnchorOperationalPosture,
        presented_authority: AuthorityKeyId,
    ) -> Self {
        Self {
            stage,
            action,
            posture,
            presented_authority,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KillSwitchDrillReview {
    pub stage: KillSwitchDrillStage,
    pub action: KillSwitchAction,
    pub status: KillSwitchDrillStatus,
    pub required_role: OperatorRole,
    pub action_permitted: bool,
    pub effective_posture: AnchorOperationalPosture,
    pub blocks_acceptance: bool,
    pub blocks_simulation: bool,
    pub blocks_submission: bool,
    pub blocks_finalization: bool,
}

impl KillSwitchDrillReview {
    pub fn is_accepted(&self) -> bool {
        matches!(
            self.status,
            KillSwitchDrillStatus::HaltAccepted | KillSwitchDrillStatus::RecoveryAccepted
        )
    }

    pub fn render_lines(&self) -> Vec<String> {
        vec![
            format!("stage={}", self.stage.as_str()),
            format!("action={}", self.action.as_str()),
            format!("status={:?}", self.status),
            format!("required_role={}", self.required_role.as_str()),
            format!("action_permitted={}", self.action_permitted),
            format!("challenge_posture={:?}", self.effective_posture.challenge),
            format!("halt_posture={:?}", self.effective_posture.halt),
            format!("recovery_posture={:?}", self.effective_posture.recovery),
            format!("blocks_acceptance={}", self.blocks_acceptance),
            format!("blocks_simulation={}", self.blocks_simulation),
            format!("blocks_submission={}", self.blocks_submission),
            format!("blocks_finalization={}", self.blocks_finalization),
        ]
    }
}

pub fn review_kill_switch_drill(
    authorities: &AuthorityMap,
    request: &KillSwitchDrillRequest,
) -> KillSwitchDrillReview {
    let required_role = request.action.required_role();

    if authorities
        .require_authority(required_role, &request.presented_authority)
        .is_err()
    {
        return kill_switch_review(
            request,
            required_role,
            KillSwitchDrillStatus::WrongAuthority,
            false,
            request.posture,
        );
    }

    match request.action {
        KillSwitchAction::Halt => {
            let mut posture = request.posture;
            posture.halt = HaltPosture::Halted;

            kill_switch_review(
                request,
                required_role,
                KillSwitchDrillStatus::HaltAccepted,
                true,
                posture,
            )
        }
        KillSwitchAction::Recover => {
            if !request.posture.halt.blocks_acceptance() {
                return kill_switch_review(
                    request,
                    required_role,
                    KillSwitchDrillStatus::RecoveryRequiresHaltedState,
                    false,
                    request.posture,
                );
            }

            let mut posture = request.posture;
            posture.halt = HaltPosture::ResumeEligible;
            posture.recovery = RecoveryPosture::Resolved;

            kill_switch_review(
                request,
                required_role,
                KillSwitchDrillStatus::RecoveryAccepted,
                true,
                posture,
            )
        }
    }
}

fn kill_switch_review(
    request: &KillSwitchDrillRequest,
    required_role: OperatorRole,
    status: KillSwitchDrillStatus,
    action_permitted: bool,
    effective_posture: AnchorOperationalPosture,
) -> KillSwitchDrillReview {
    KillSwitchDrillReview {
        stage: request.stage,
        action: request.action,
        status,
        required_role,
        action_permitted,
        blocks_acceptance: effective_posture.blocks_acceptance(),
        blocks_simulation: effective_posture.blocks_simulation(),
        blocks_submission: effective_posture.blocks_submission(),
        blocks_finalization: effective_posture.blocks_finalization(),
        effective_posture,
    }
}
