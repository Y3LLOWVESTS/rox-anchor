//! RO:WHAT — Tests core-owned halt/recovery kill-switch drill decisions.
//! RO:WHY — BUILD_PLAN2 Phase 12 requires halt and recovery behavior across all pipeline stages.
//! RO:INTERACTS — AnchorOperationalPosture, AuthorityMap, KillSwitchDrillReview.
//! RO:INVARIANTS — wrong authority is rejected; halted posture blocks unsafe progress; recovery clears halt blockers.
//! RO:SECURITY — local decision tests only; no keypair loading, RPC, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-core --test kill_switch_drills.

use rox_anchor_core::{
    AnchorOperationalPosture, AuthorityAssignment, AuthorityKeyId, AuthorityMap,
    AuthoritySeparationMode, KillSwitchAction, KillSwitchDrillRequest, KillSwitchDrillStage,
    KillSwitchDrillStatus, OperatorRole,
};

fn key(value: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(value).expect("static authority id should validate")
}

fn authorities() -> AuthorityMap {
    AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(
                OperatorRole::HaltAuthority,
                key("halt-authority-phase12-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-authority-phase12-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::UpgradeAuthority,
                key("upgrade-authority-phase12-key"),
            ),
            AuthorityAssignment::new(
                OperatorRole::MintAuthority,
                key("mint-authority-phase12-key"),
            ),
        ],
    )
}

#[test]
fn halt_is_accepted_and_blocks_every_phase12_stage() {
    for stage in [
        KillSwitchDrillStage::BeforeProofAcceptance,
        KillSwitchDrillStage::AfterProofAcceptanceBeforeSimulation,
        KillSwitchDrillStage::AfterSimulationBeforeSubmission,
        KillSwitchDrillStage::AfterCappedTestnetSubmission,
    ] {
        let request = KillSwitchDrillRequest::new(
            stage,
            KillSwitchAction::Halt,
            AnchorOperationalPosture::clear(),
            key("halt-authority-phase12-key"),
        );

        let review = rox_anchor_core::review_kill_switch_drill(&authorities(), &request);

        assert_eq!(review.status, KillSwitchDrillStatus::HaltAccepted);
        assert!(review.action_permitted);
        assert!(review.blocks_acceptance);
        assert!(review.blocks_simulation);
        assert!(review.blocks_submission);
        assert!(review.blocks_finalization);
        assert!(review.is_accepted());
    }
}

#[test]
fn wrong_authority_cannot_halt_or_recover() {
    for action in [KillSwitchAction::Halt, KillSwitchAction::Recover] {
        let request = KillSwitchDrillRequest::new(
            KillSwitchDrillStage::AfterSimulationBeforeSubmission,
            action,
            AnchorOperationalPosture::recovery_required(),
            key("wrong-authority-phase12-key"),
        );

        let review = rox_anchor_core::review_kill_switch_drill(&authorities(), &request);

        assert_eq!(review.status, KillSwitchDrillStatus::WrongAuthority);
        assert!(!review.action_permitted);
        assert!(!review.is_accepted());
    }
}

#[test]
fn recovery_requires_halted_state_and_then_unblocks_progress() {
    let not_halted_request = KillSwitchDrillRequest::new(
        KillSwitchDrillStage::AfterCappedTestnetSubmission,
        KillSwitchAction::Recover,
        AnchorOperationalPosture::clear(),
        key("recovery-authority-phase12-key"),
    );

    let not_halted = rox_anchor_core::review_kill_switch_drill(&authorities(), &not_halted_request);

    assert_eq!(
        not_halted.status,
        KillSwitchDrillStatus::RecoveryRequiresHaltedState
    );
    assert!(!not_halted.action_permitted);

    let recovery_request = KillSwitchDrillRequest::new(
        KillSwitchDrillStage::AfterCappedTestnetSubmission,
        KillSwitchAction::Recover,
        AnchorOperationalPosture::halted_recovery_required(),
        key("recovery-authority-phase12-key"),
    );

    let recovered = rox_anchor_core::review_kill_switch_drill(&authorities(), &recovery_request);

    assert_eq!(recovered.status, KillSwitchDrillStatus::RecoveryAccepted);
    assert!(recovered.action_permitted);
    assert!(!recovered.blocks_acceptance);
    assert!(!recovered.blocks_simulation);
    assert!(!recovered.blocks_submission);
    assert!(!recovered.blocks_finalization);
    assert!(recovered.is_accepted());
}
