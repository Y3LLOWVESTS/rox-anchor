//! RO:WHAT — Tests coordinator finalization gate under halt and recovery posture.
//! RO:WHY — BUILD_PLAN2 Phase 12 requires coordinator refusal while halted or recovery-blocked.
//! RO:INTERACTS — coordinator decisions and core AnchorOperationalPosture.
//! RO:INVARIANTS — accepted proof/RPC evidence is not enough when halt/recovery posture blocks finalization.
//! RO:SECURITY — local gate tests only; no live RPC, keypair, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test halt_recovery_finalization_gate.

use rox_anchor_coordinator::{
    review_coordinator_finalization_gate, review_coordinator_request, CoordinatorConfig,
    CoordinatorFinalizationGateStatus, CoordinatorReviewRequest,
};
use rox_anchor_core::{
    AnchorOperationalPosture, ClusterId, MintId, OperationId, ProgramId, TokenAccountId,
};
use rox_anchor_proof::{fixtures, ReplaySet};
use rox_anchor_rpc_proof::{ExpectedRpcBinding, RpcCommitmentLevel, RpcObservation};

fn expected_rpc_binding() -> ExpectedRpcBinding {
    ExpectedRpcBinding::new(
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        RpcCommitmentLevel::Confirmed,
    )
}

fn observation(source: &str, signature: &str, slot: u64) -> RpcObservation {
    RpcObservation::new(
        source,
        ClusterId::new("localnet").unwrap(),
        ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
        MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
        TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        OperationId::new("op-roc-to-rox-0001").unwrap(),
        signature,
        slot,
        RpcCommitmentLevel::Finalized,
    )
}

fn accepted_decision() -> rox_anchor_coordinator::CoordinatorDecision {
    let package = fixtures::valid_package();
    let request = CoordinatorReviewRequest::new(
        package,
        fixtures::expected_proof_binding(),
        expected_rpc_binding(),
        vec![
            observation("rpc-a", "sig-phase12-finalization-111111111111", 90),
            observation("rpc-b", "sig-phase12-finalization-111111111111", 91),
        ],
        ReplaySet::default(),
    );

    review_coordinator_request(&request, CoordinatorConfig::new(2, 100, 4), 100)
}

#[test]
fn accepted_decision_permits_finalization_only_when_posture_is_clear() {
    let decision = accepted_decision();
    assert!(decision.is_accepted());

    let gate = review_coordinator_finalization_gate(&decision, AnchorOperationalPosture::clear());

    assert_eq!(gate.status, CoordinatorFinalizationGateStatus::Permitted);
    assert!(gate.permits_finalization);
    assert!(gate.is_permitted());
}

#[test]
fn halted_posture_blocks_coordinator_finalization() {
    let decision = accepted_decision();
    let gate = review_coordinator_finalization_gate(&decision, AnchorOperationalPosture::halted());

    assert_eq!(gate.status, CoordinatorFinalizationGateStatus::Halted);
    assert!(!gate.permits_finalization);
    assert!(!gate.is_permitted());
}

#[test]
fn recovery_required_posture_blocks_coordinator_finalization() {
    let decision = accepted_decision();
    let gate = review_coordinator_finalization_gate(
        &decision,
        AnchorOperationalPosture::recovery_required(),
    );

    assert_eq!(
        gate.status,
        CoordinatorFinalizationGateStatus::RecoveryBlocked
    );
    assert!(!gate.permits_finalization);
}

#[test]
fn recovery_resolved_posture_allows_gate_again() {
    let decision = accepted_decision();
    let gate = review_coordinator_finalization_gate(
        &decision,
        AnchorOperationalPosture::recovery_resolved(),
    );

    assert_eq!(gate.status, CoordinatorFinalizationGateStatus::Permitted);
    assert!(gate.permits_finalization);
}
