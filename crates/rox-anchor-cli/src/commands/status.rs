//! RO:WHAT — Status-label, testnet-config, private-pilot-config, and authority-model inspection for the ROX Anchor CLI.
//! RO:WHY — Exposes display-safe labels and redacted hardening surfaces without inventing finality.
//! RO:INTERACTS — rox-anchor-core labels, lifecycle states, config reports, and authority model.
//! RO:INVARIANTS — labels/config/authority reports are display strings only; not runtime authority.
//! RO:SECURITY — no wallet, RPC call, key loading, deployment, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — covered through CLI command dispatch, status display tests, authority status tests, and private-pilot config tests.

use rox_anchor_core::{
    label_for_lifecycle_state, AccountId, AnchorCluster, AnchorEnvironmentMode,
    AnchorLifecycleState, AnchorOperationalPosture, AnchorSafetyProfile, AuthorityAssignment,
    AuthorityKeyId, AuthorityMap, AuthorityRotationIntent, AuthoritySeparationMode,
    ClusterAllowlist, IdempotencyKey, InternalRocDryRunBurnIntent, InternalRocDryRunReleaseIntent,
    Nonce, OperationId, OperatorRole, PrivatePilotConfig, SubmissionMode, TestnetConfig,
    TestnetProgramArtifactManifest, SAFE_STATUS_LABELS,
};

const PRIVATE_PILOT_STATUS_CONFIG: &str = r#"
environment_mode = "testnet_only"
cluster = "devnet"
submission_mode = "simulate_only"
rpc_url = "https://private-devnet.invalid/status-provider-token"
payer_keypair_path = "/external/pilot-keys/status-payer.json"
operator_label = "private-pilot-status-operator"
asset_label = "test-only-rox-status-asset"
receipt_output_path = "/external/pilot-receipts/status-receipt.json"
observed_signature = "5JstatusPrivatePilotSignature1111222233334444"
"#;

const TESTNET_PROGRAM_ID: &str = "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk";

pub fn status_report() -> String {
    let mut lines = vec![
        "rox-anchor status labels".to_string(),
        format!(
            "finality_candidate_label: {}",
            label_for_lifecycle_state(AnchorLifecycleState::FinalityEligible)
        ),
        "safe_labels:".to_string(),
    ];

    for label in SAFE_STATUS_LABELS {
        lines.push(format!("  - {label}"));
    }

    lines.push("testnet_config_surface: redacted_non_secret_shape".to_string());

    let example = TestnetConfig::require_explicit(
        Some(AnchorEnvironmentMode::TestnetOnly),
        AnchorCluster::Devnet,
        SubmissionMode::SimulateOnly,
        Some("https://api.devnet.solana.com/example-token"),
        Some("/Users/operator/.config/solana/testnet-payer.json"),
    )
    .expect("static CLI status example should be a safe devnet config");

    for line in example.redacted_report().lines() {
        lines.push(format!("  {line}"));
    }

    lines.push("private_pilot_config_surface: redacted_external_config_loader".to_string());

    let private_pilot = PrivatePilotConfig::parse_external_config(PRIVATE_PILOT_STATUS_CONFIG)
        .expect("static private pilot status config should validate");

    for line in private_pilot.redacted_report().lines() {
        lines.push(format!("  {line}"));
    }

    lines.push("private_pilot_config_runtime_effects: disabled".to_string());
    lines.push("private_pilot_config_wallet_loading: disabled".to_string());
    lines.push("private_pilot_config_rpc_calls: disabled".to_string());

    lines.push("crablink_internal_roc_dry_run_surface: display_safe_intent_shapes".to_string());

    let dry_run_safety = AnchorSafetyProfile::new(
        AnchorEnvironmentMode::TestnetOnly,
        AnchorCluster::Devnet,
        ClusterAllowlist::testnet_experiments(),
        SubmissionMode::SimulateOnly,
    );

    let burn_intent = InternalRocDryRunBurnIntent::new(
        dry_run_safety,
        OperationId::new("op-crablink-status-burn-0001")
            .expect("static operation id should validate"),
        IdempotencyKey::new("idem-crablink-status-burn-0001")
            .expect("static idempotency key should validate"),
        Nonce::new("nonce-crablink-status-burn-0001").expect("static nonce should validate"),
        AccountId::new("crablink-status-test-account-0001")
            .expect("static account should validate"),
        "test-only-crablink-status-burn-intent",
        50,
    )
    .expect("static CrabLink burn dry-run intent should validate");

    for line in burn_intent.redacted_report_lines() {
        lines.push(format!("  {line}"));
    }

    let release_intent = InternalRocDryRunReleaseIntent::new(
        dry_run_safety,
        OperationId::new("op-crablink-status-release-0001")
            .expect("static operation id should validate"),
        IdempotencyKey::new("idem-crablink-status-release-0001")
            .expect("static idempotency key should validate"),
        Nonce::new("nonce-crablink-status-release-0001").expect("static nonce should validate"),
        AccountId::new("crablink-status-test-account-0002")
            .expect("static account should validate"),
        "test-only-crablink-status-release-intent",
        25,
    )
    .expect("static CrabLink release dry-run intent should validate");

    for line in release_intent.redacted_report_lines() {
        lines.push(format!("  {line}"));
    }

    lines.push("crablink_internal_roc_adapter_runtime_effects: disabled".to_string());
    lines.push("crablink_internal_roc_adapter_wallet_calls: disabled".to_string());
    lines.push("crablink_internal_roc_adapter_ledger_mutation: disabled".to_string());
    lines.push("crablink_internal_roc_adapter_paid_unlock: disabled".to_string());
    lines.push("crablink_internal_roc_adapter_settlement_claim: none".to_string());

    lines.push("testnet_program_manifest_surface: redacted_non_secret_artifact_shape".to_string());

    let manifest = TestnetProgramArtifactManifest::from_labels(
        "devnet",
        TESTNET_PROGRAM_ID,
        TESTNET_PROGRAM_ID,
        "build-hash-private-status-0001",
        "idl-hash-private-status-0001",
        Some(123_456),
        "private-pilot-program-status-operator",
        "test-only-rox-program-status-artifact",
        "/external/pilot-deploy/status/rox_anchor.so",
        "/external/pilot-deploy/status/rox_anchor.json",
    )
    .expect("static CLI status manifest should validate");

    for line in manifest.redacted_report().lines() {
        let status_line = if line == "production_finality_claim: false" {
            "private_pilot_finality_claim: false".to_string()
        } else {
            line
        };

        lines.push(format!("  {status_line}"));
    }

    lines.push("testnet_program_manifest_runtime_effects: disabled".to_string());
    lines.push("testnet_program_manifest_deployment_claims: disabled".to_string());

    lines.push("phase12_kill_switch_surface: local_drill_only".to_string());

    let halted_posture = AnchorOperationalPosture::halted();
    lines.push(format!(
        "halted_blocks_acceptance: {}",
        halted_posture.blocks_acceptance()
    ));
    lines.push(format!(
        "halted_blocks_simulation: {}",
        halted_posture.blocks_simulation()
    ));
    lines.push(format!(
        "halted_blocks_submission: {}",
        halted_posture.blocks_submission()
    ));
    lines.push(format!(
        "halted_blocks_finalization: {}",
        halted_posture.blocks_finalization()
    ));

    let recovered_posture = AnchorOperationalPosture::recovery_resolved();
    lines.push(format!(
        "recovery_resolved_blocks_submission: {}",
        recovered_posture.blocks_submission()
    ));
    lines.push(format!(
        "recovery_resolved_blocks_finalization: {}",
        recovered_posture.blocks_finalization()
    ));

    lines.push("authority_model_surface: redacted_identifier_only".to_string());
    lines.push("real_key_loading: disabled".to_string());

    let authority_map = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(
                OperatorRole::Observer,
                AuthorityKeyId::new("observer-authority-status-key-00000001")
                    .expect("static authority id should validate"),
            ),
            AuthorityAssignment::new(
                OperatorRole::Coordinator,
                AuthorityKeyId::new("coordinator-authority-status-key-00000002")
                    .expect("static authority id should validate"),
            ),
            AuthorityAssignment::new(
                OperatorRole::Relayer,
                AuthorityKeyId::new("relayer-authority-status-key-00000003")
                    .expect("static authority id should validate"),
            ),
            AuthorityAssignment::new(
                OperatorRole::UpgradeAuthority,
                AuthorityKeyId::new("upgrade-authority-status-key-00000004")
                    .expect("static authority id should validate"),
            ),
            AuthorityAssignment::new(
                OperatorRole::MintAuthority,
                AuthorityKeyId::new("mint-authority-status-key-00000005")
                    .expect("static authority id should validate"),
            ),
            AuthorityAssignment::new(
                OperatorRole::HaltAuthority,
                AuthorityKeyId::new("halt-authority-status-key-00000006")
                    .expect("static authority id should validate"),
            ),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                AuthorityKeyId::new("recovery-authority-status-key-00000007")
                    .expect("static authority id should validate"),
            ),
        ],
    );

    for line in authority_map.redacted_report_lines() {
        lines.push(format!("  {line}"));
    }

    let rotation = AuthorityRotationIntent::new(
        OperatorRole::HaltAuthority,
        AuthorityKeyId::new("halt-authority-current-status-key-11112222")
            .expect("static authority id should validate"),
        AuthorityKeyId::new("halt-authority-next-status-key-33334444")
            .expect("static authority id should validate"),
        Some(100),
    );

    lines.push("authority_rotation_surface: redacted_intent_only".to_string());
    for line in rotation.redacted_report_lines() {
        lines.push(format!("  {line}"));
    }

    lines.join("\n")
}
