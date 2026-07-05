//! RO:WHAT — Tests CLI status output for BUILD_PLAN2 Phase 3 authority safety display.
//! RO:WHY — Ensures status can expose authority model shape without leaking raw key identifiers.
//! RO:INTERACTS — status command, AuthorityMap, and AuthorityRotationIntent.
//! RO:INVARIANTS — no real key loading; output stays display-only and non-finality.
//! RO:SECURITY — no RPC, key loading, wallet, transaction, mint, burn, or settlement.
//! RO:TEST — run with cargo test -p rox-anchor-cli --test authority_status.

use rox_anchor_cli::run_from_args;

#[test]
fn status_output_includes_redacted_authority_model_shape() {
    let output = run_from_args(["rox-anchor", "status"]).expect("status should run");

    assert!(output.contains("authority_model_surface: redacted_identifier_only"));
    assert!(output.contains("real_key_loading: disabled"));
    assert!(output.contains("authority_separation_mode: strict"));

    for role in [
        "observer",
        "coordinator",
        "relayer",
        "upgrade_authority",
        "mint_authority",
        "halt_authority",
        "recovery_authority",
    ] {
        assert!(output.contains(role), "missing role {role}");
    }

    assert!(output.contains("authority_rotation_surface: redacted_intent_only"));
    assert!(output.contains("rotation_role: halt_authority"));
    assert!(output.contains("activate_at_slot: 100"));

    assert!(!output.contains("status-key-00000001"));
    assert!(!output.contains("11112222"));
    assert!(!output.contains("33334444"));
    assert!(!output.contains("loaded keypair"));
    assert!(!output.contains("settlement complete"));
}
