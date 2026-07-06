//! RO:WHAT — Tests the coordinator-side internal ROC dry-run adapter.
//! RO:WHY — BUILD_PLAN3 Phase 11 needs CrabLink/internal ROC intent handoff without value mutation.
//! RO:INTERACTS — coordinator observations and rox-anchor-core internal ROC dry-run intent shapes.
//! RO:INVARIANTS — burn/release intent observations are report-only and do not claim finality.
//! RO:SECURITY — no svc-wallet call, ron-ledger mutation, paid unlock, or real ROC release.
//! RO:TEST — cargo test -p rox-anchor-coordinator --test internal_roc_dry_run_adapter.

#![forbid(unsafe_code)]

use rox_anchor_coordinator::CoordinatorInternalRocDryRunObservation;
use rox_anchor_core::{
    AccountId, AnchorCluster, AnchorCoreError, AnchorEnvironmentMode, AnchorSafetyProfile,
    ClusterAllowlist, IdempotencyKey, InternalRocDryRunBurnIntent, InternalRocDryRunReleaseIntent,
    Nonce, OperationId, SubmissionMode,
};

fn safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    )
}

fn operation_id() -> OperationId {
    OperationId::new("op-crablink-internal-roc-0001").unwrap()
}

fn idempotency_key() -> IdempotencyKey {
    IdempotencyKey::new("idem-crablink-internal-roc-0001").unwrap()
}

fn nonce() -> Nonce {
    Nonce::new("nonce-crablink-internal-roc-0001").unwrap()
}

fn account() -> AccountId {
    AccountId::new("crablink-test-account-private-pilot-0001").unwrap()
}

#[test]
fn coordinator_accepts_burn_intent_as_dry_run_observation_only() {
    let intent = InternalRocDryRunBurnIntent::new(
        safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-crablink-roc-burn-intent",
        75,
    )
    .expect("burn intent should validate");

    let observation = CoordinatorInternalRocDryRunObservation::from_burn_intent(&intent)
        .expect("coordinator should accept valid dry-run burn intent");

    let report = observation.redacted_report();

    assert!(report.contains("coordinator_internal_roc_dry_run_observation: accepted"));
    assert!(report.contains("kind: burn_intent_input"));
    assert!(report.contains("internal_roc_burn_intent: dry_run_input"));
    assert!(report.contains("direction: roc_to_rox"));
    assert!(report.contains("coordinator_finality_claim: none"));
    assert!(report.contains("coordinator_wallet_call: disabled"));
    assert!(report.contains("coordinator_ron_ledger_mutation: disabled"));
    assert!(report.contains("coordinator_paid_content_unlock: disabled"));
    assert!(report.contains("svc_wallet_call: disabled"));
    assert!(report.contains("ron_ledger_mutation: disabled"));
    assert!(report.contains("settlement_claim: none"));
    assert!(!report.contains("crablink-test-account-private-pilot-0001"));
}

#[test]
fn coordinator_accepts_release_intent_without_real_roc_mutation() {
    let intent = InternalRocDryRunReleaseIntent::new(
        safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-crablink-roc-release-intent",
        33,
    )
    .expect("release intent should validate");

    let observation = CoordinatorInternalRocDryRunObservation::from_release_intent(&intent)
        .expect("coordinator should accept valid dry-run release intent");

    let report = observation.redacted_report();

    assert!(report.contains("kind: release_intent_output"));
    assert!(report.contains("internal_roc_release_intent: dry_run_output"));
    assert!(report.contains("direction: rox_to_roc"));
    assert!(report.contains("real_internal_roc_release: disabled"));
    assert!(report.contains("future_real_roc_path: svc-wallet -> ron-ledger only"));
    assert!(report.contains("settlement_claim: none"));
}

#[test]
fn coordinator_rejects_invalid_internal_roc_intent_before_observation() {
    let unsafe_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::ProductionDisabled,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );

    let error = InternalRocDryRunBurnIntent::new(
        unsafe_safety,
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-crablink-roc-burn-intent",
        1,
    )
    .unwrap_err();

    assert!(matches!(
        error,
        AnchorCoreError::InternalRocDryRunRequiresExplicitNonProductionMode { .. }
    ));
}
