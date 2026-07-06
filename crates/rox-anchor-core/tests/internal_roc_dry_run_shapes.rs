//! RO:WHAT — Tests internal ROC dry-run intent shapes for CrabLink/private pilot handoff.
//! RO:WHY — BUILD_PLAN3 Phase 11 requires burn/release intent shapes without real ROC mutation.
//! RO:INTERACTS — rox-anchor-core safety profiles, IDs, and internal ROC dry-run reports.
//! RO:INVARIANTS — only explicit non-production, non-submitting, test-only amounts/labels validate.
//! RO:SECURITY — no svc-wallet call, ron-ledger mutation, paid-content unlock, or settlement claim.
//! RO:TEST — cargo test -p rox-anchor-core --test internal_roc_dry_run_shapes.

#![forbid(unsafe_code)]

use rox_anchor_core::{
    AccountId, AnchorCluster, AnchorCoreError, AnchorEnvironmentMode, AnchorSafetyProfile,
    ClusterAllowlist, IdempotencyKey, InternalRocDryRunBurnIntent, InternalRocDryRunReleaseIntent,
    Nonce, OperationId, SubmissionMode, MAX_INTERNAL_ROC_DRY_RUN_AMOUNT,
};

fn testnet_simulation_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    )
}

fn local_dry_run_safety() -> AnchorSafetyProfile {
    AnchorSafetyProfile::new(
        AnchorEnvironmentMode::LocalOnly,
        AnchorCluster::Localnet,
        ClusterAllowlist::localnet_only(),
        SubmissionMode::DryRunOnly,
    )
}

fn operation_id() -> OperationId {
    OperationId::new("op-internal-roc-dry-run-0001").unwrap()
}

fn idempotency_key() -> IdempotencyKey {
    IdempotencyKey::new("idem-internal-roc-dry-run-0001").unwrap()
}

fn nonce() -> Nonce {
    Nonce::new("nonce-internal-roc-dry-run-0001").unwrap()
}

fn account() -> AccountId {
    AccountId::new("crablink-test-account-0001").unwrap()
}

#[test]
fn burn_intent_accepts_only_test_only_non_submitting_shape() {
    let intent = InternalRocDryRunBurnIntent::new(
        testnet_simulation_safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-internal-roc-burn-intent",
        50,
    )
    .expect("test-only burn intent should validate");

    assert_eq!(intent.test_amount, 50);

    let report = intent.redacted_report_lines().join("\n");
    assert!(report.contains("internal_roc_burn_intent: dry_run_input"));
    assert!(report.contains("direction: roc_to_rox"));
    assert!(report.contains("svc_wallet_call: disabled"));
    assert!(report.contains("ron_ledger_mutation: disabled"));
    assert!(report.contains("paid_content_unlock: disabled"));
    assert!(report.contains("real_internal_roc_burn: disabled"));
    assert!(report.contains("settlement_claim: none"));
    assert!(report.contains("crablink_final_settlement_display: disabled"));
    assert!(!report.contains("crablink-test-account-0001"));
}

#[test]
fn release_intent_accepts_local_dry_run_without_real_roc_release() {
    let intent = InternalRocDryRunReleaseIntent::new(
        local_dry_run_safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-internal-roc-release-intent",
        25,
    )
    .expect("test-only release intent should validate");

    let report = intent.redacted_report_lines().join("\n");
    assert!(report.contains("internal_roc_release_intent: dry_run_output"));
    assert!(report.contains("direction: rox_to_roc"));
    assert!(report.contains("real_internal_roc_release: disabled"));
    assert!(report.contains("future_real_roc_path: svc-wallet -> ron-ledger only"));
    assert!(report.contains("settlement_claim: none"));
}

#[test]
fn internal_roc_dry_run_rejects_default_production_disabled_mode() {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::ProductionDisabled,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );

    let error = InternalRocDryRunBurnIntent::new(
        safety,
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-internal-roc-burn-intent",
        1,
    )
    .unwrap_err();

    assert!(matches!(
        error,
        AnchorCoreError::InternalRocDryRunRequiresExplicitNonProductionMode { .. }
    ));
}

#[test]
fn internal_roc_dry_run_rejects_submitting_mode() {
    let safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::TestnetSubmitCapped,
    );

    let error = InternalRocDryRunReleaseIntent::new(
        safety,
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-internal-roc-release-intent",
        1,
    )
    .unwrap_err();

    assert!(matches!(
        error,
        AnchorCoreError::InternalRocDryRunRequiresNonSubmittingMode { .. }
    ));
}

#[test]
fn internal_roc_dry_run_rejects_zero_over_cap_and_non_test_labels() {
    let zero = InternalRocDryRunBurnIntent::new(
        testnet_simulation_safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-internal-roc-burn-intent",
        0,
    )
    .unwrap_err();

    assert!(matches!(
        zero,
        AnchorCoreError::InvalidInternalRocDryRunAmount { amount: 0, .. }
    ));

    let over_cap = InternalRocDryRunBurnIntent::new(
        testnet_simulation_safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-only-internal-roc-burn-intent",
        MAX_INTERNAL_ROC_DRY_RUN_AMOUNT + 1,
    )
    .unwrap_err();

    assert!(matches!(
        over_cap,
        AnchorCoreError::InvalidInternalRocDryRunAmount { .. }
    ));

    let not_test = InternalRocDryRunBurnIntent::new(
        testnet_simulation_safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "shadow-internal-roc",
        1,
    )
    .unwrap_err();

    assert!(matches!(
        not_test,
        AnchorCoreError::MissingTestOnlyInternalRocLabel { .. }
    ));

    let public = InternalRocDryRunBurnIntent::new(
        testnet_simulation_safety(),
        operation_id(),
        idempotency_key(),
        nonce(),
        account(),
        "test-production-internal-roc",
        1,
    )
    .unwrap_err();

    assert!(matches!(
        public,
        AnchorCoreError::PublicOrProductionInternalRocDryRunLabel { .. }
    ));
}
