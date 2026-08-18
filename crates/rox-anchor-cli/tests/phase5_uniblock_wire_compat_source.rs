//! Phase 5 Uniblock account-wire compatibility boundary.
//!
//! This test never calls RPC. It proves the compatibility exception remains
//! provider-specific, rentEpoch-specific, and read-only.

#![forbid(unsafe_code)]

const COMPAT: &str = include_str!("../src/commands/phase5_wire_compat.rs");

const PHASE5A: &str = include_str!("../src/commands/phase5_live_read_only.rs");

#[test]
fn phase5_uniblock_wire_exception_is_exactly_scoped() {
    for required in [
        "uniblock-devnet-secondary",
        "18_446_744_073_709_552_000.0",
        "u64::MAX",
        "rentEpoch",
        "observed.to_bits()",
        "UNIBLOCK_RENT_EPOCH_WIRE_FLOAT",
    ] {
        assert!(
            COMPAT.contains(required),
            "compatibility source missing `{required}`",
        );
    }
}

#[test]
fn phase5_uniblock_wire_decoder_keeps_other_account_fields_strict() {
    for required in [
        "\"lamports\"",
        "Value::as_u64",
        "\"owner\"",
        "Pubkey::from_str",
        "\"executable\"",
        "Value::as_bool",
        "\"base64\"",
        "STANDARD",
        ".decode(",
    ] {
        assert!(
            COMPAT.contains(required),
            "strict account decoder marker missing `{required}`",
        );
    }
}

#[test]
fn phase5_uniblock_wire_decoder_is_read_only() {
    for forbidden in [
        "read_keypair_file",
        "new_signed_with_payer",
        "send_transaction(",
        "send_and_confirm_transaction(",
        "simulate_transaction(",
        "mint_to(",
        "burn(",
    ] {
        assert!(
            !COMPAT.contains(forbidden),
            "compatibility decoder contains forbidden API `{forbidden}`",
        );
    }

    assert!(COMPAT.contains("RpcRequest::GetMultipleAccounts"));
}

#[test]
fn phase5b1_uses_wire_decoder_inside_existing_bounded_retry() {
    let retry = PHASE5A
        .find("phase5_read_only_rpc_retry")
        .expect("Phase 5 read-only retry must remain");

    let decoder = PHASE5A
        .find("phase5_wire_compat::get_multiple_accounts_compat")
        .expect("Phase 5B1 must call the compatibility decoder");

    assert!(retry < decoder);
}

#[test]
fn phase5b2_reuses_same_wire_decoder_with_common_min_context() {
    let phase5b2 = include_str!("../src/commands/phase5_live_closeout.rs");

    for required in [
        "phase5_wire_compat::get_multiple_accounts_with_context_compat",
        "Some(",
        "minimum_context_slot",
        "account_response.context_slot",
        "account_response.accounts",
        "phase5_read_only_rpc_retry",
    ] {
        assert!(
            phase5b2.contains(required),
            "Phase 5B2 compatibility source missing `{required}`",
        );
    }

    assert!(
        !phase5b2.contains("get_multiple_accounts_with_config"),
        "Phase 5B2 must no longer use the typed account decoder that rejects Uniblock rentEpoch",
    );
}
