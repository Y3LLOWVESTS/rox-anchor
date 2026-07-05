//! RO:WHAT — Tests BUILD_PLAN2 Phase 3 operator key and authority safety model.
//! RO:WHY — Proves authority roles, separation, wrong-authority rejection, redaction, and rotation intent.
//! RO:INTERACTS — OperatorRole, AuthorityKeyId, AuthorityMap, and AuthorityRotationIntent.
//! RO:INVARIANTS — critical authority sharing is rejected unless explicitly test-only; no keypairs are loaded.
//! RO:SECURITY — identifier-only tests; no wallet, RPC, key file, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-core --test operator_authority_model.

use rox_anchor_core::{
    AnchorCoreError, AuthorityAssignment, AuthorityKeyId, AuthorityMap, AuthorityRotationIntent,
    AuthoritySeparationMode, OperatorRole,
};

fn key(label: &str) -> AuthorityKeyId {
    AuthorityKeyId::new(label).expect("test key id should validate")
}

#[test]
fn operator_roles_have_stable_display_labels() {
    let labels: Vec<&str> = OperatorRole::ALL.iter().map(|role| role.as_str()).collect();

    assert_eq!(
        labels,
        vec![
            "observer",
            "coordinator",
            "relayer",
            "upgrade_authority",
            "mint_authority",
            "halt_authority",
            "recovery_authority",
        ]
    );

    assert!(OperatorRole::UpgradeAuthority.is_critical_authority());
    assert!(!OperatorRole::Observer.is_critical_authority());
}

#[test]
fn strict_mode_rejects_one_key_owning_every_critical_authority() {
    let shared = key("authority-shared-critical-key-00000001");
    let map = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::MintAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::RecoveryAuthority, shared),
        ],
    );

    assert!(matches!(
        map.validate_critical_authorities(),
        Err(AnchorCoreError::CriticalAuthoritySharedWithoutTestOnly { .. })
    ));
}

#[test]
fn explicit_test_only_mode_allows_shared_critical_authority_for_drills() {
    let shared = key("authority-shared-test-only-key-00000001");
    let map = AuthorityMap::new(
        AuthoritySeparationMode::ExplicitTestOnlyShared,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::MintAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, shared.clone()),
            AuthorityAssignment::new(OperatorRole::RecoveryAuthority, shared),
        ],
    );

    assert!(map.validate_critical_authorities().is_ok());
}

#[test]
fn separated_critical_authorities_validate_in_strict_mode() {
    let map = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::UpgradeAuthority, key("upgrade-key-00000001")),
            AuthorityAssignment::new(OperatorRole::MintAuthority, key("mint-key-00000002")),
            AuthorityAssignment::new(OperatorRole::HaltAuthority, key("halt-key-00000003")),
            AuthorityAssignment::new(
                OperatorRole::RecoveryAuthority,
                key("recovery-key-00000004"),
            ),
        ],
    );

    assert!(map.validate_critical_authorities().is_ok());
}

#[test]
fn wrong_authority_rejection_uses_redacted_key_ids() {
    let expected = key("halt-authority-expected-key-abcdef123456");
    let presented = key("halt-authority-presented-key-fedcba654321");

    let map = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![AuthorityAssignment::new(
            OperatorRole::HaltAuthority,
            expected,
        )],
    );

    let err = map
        .require_authority(OperatorRole::HaltAuthority, &presented)
        .unwrap_err();

    let rendered = err.to_string();

    assert!(rendered.contains("halt_authority"));
    assert!(rendered.contains("halt…3456"));
    assert!(rendered.contains("halt…4321"));
    assert!(!rendered.contains("abcdef123456"));
    assert!(!rendered.contains("fedcba654321"));
}

#[test]
fn duplicate_authority_roles_are_rejected() {
    let map = AuthorityMap::new(
        AuthoritySeparationMode::Strict,
        vec![
            AuthorityAssignment::new(OperatorRole::Relayer, key("relayer-key-00000001")),
            AuthorityAssignment::new(OperatorRole::Relayer, key("relayer-key-00000002")),
        ],
    );

    assert_eq!(
        map.validate_shape(),
        Err(AnchorCoreError::DuplicateAuthorityRole { role: "relayer" })
    );
}

#[test]
fn authority_rotation_intent_rejects_noop_and_requires_activation_slot() {
    let current = key("mint-authority-current-key-00000001");

    let no_op = AuthorityRotationIntent::new(
        OperatorRole::MintAuthority,
        current.clone(),
        current.clone(),
        Some(25),
    );

    assert!(matches!(
        no_op.validate(),
        Err(AnchorCoreError::RotationNoOp {
            role: "mint_authority",
            ..
        })
    ));

    let missing_slot = AuthorityRotationIntent::new(
        OperatorRole::MintAuthority,
        current,
        key("mint-authority-next-key-00000002"),
        None,
    );

    assert_eq!(
        missing_slot.validate(),
        Err(AnchorCoreError::MissingRotationActivation {
            role: "mint_authority",
        })
    );
}

#[test]
fn authority_rotation_intent_reports_only_redacted_key_ids() {
    let intent = AuthorityRotationIntent::new(
        OperatorRole::RecoveryAuthority,
        key("recovery-authority-current-key-11112222"),
        key("recovery-authority-next-key-33334444"),
        Some(100),
    );

    assert!(intent.validate().is_ok());

    let report = intent.redacted_report_lines().join("\n");

    assert!(report.contains("rotation_role: recovery_authority"));
    assert!(report.contains("activate_at_slot: 100"));
    assert!(report.contains("reco…2222"));
    assert!(report.contains("reco…4444"));
    assert!(!report.contains("11112222"));
    assert!(!report.contains("33334444"));
}
