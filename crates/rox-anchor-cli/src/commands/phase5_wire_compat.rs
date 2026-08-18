//! Phase 5 read-only account-wire compatibility.
//!
//! Solana uses `u64::MAX` as the rent-epoch sentinel for rent-exempt accounts.
//! The qualified Uniblock Devnet endpoint repeatedly returned that sentinel as
//! the out-of-range JSON number `18446744073709552000`.
//!
//! The exception remains deliberately narrow:
//! - only the fixed Uniblock Phase 5 source,
//! - only the `rentEpoch` field,
//! - only the exact observed floating representation,
//! - normalized only to `u64::MAX`.
//!
//! Lamports, owner, executable, base64 account data, context slots, and normal
//! rent epochs remain strictly decoded. This module performs read-only RPC only.

#![forbid(unsafe_code)]

use std::str::FromStr;

use anchor_client::{
    solana_client::{rpc_client::RpcClient, rpc_request::RpcRequest},
    solana_sdk::{account::Account, pubkey::Pubkey},
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};

const UNIBLOCK_PHASE5_SOURCE: &str = "uniblock-devnet-secondary";

const UNIBLOCK_RENT_EPOCH_WIRE_FLOAT: f64 = 18_446_744_073_709_552_000.0;

/// A raw Phase 5 account response with the actual RPC context slot preserved.
#[derive(Debug)]
pub(super) struct Phase5AccountBatch {
    pub(super) context_slot: u64,
    pub(super) accounts: Vec<Option<Account>>,
}

/// B1 convenience wrapper.
///
/// B1 does not consume the context slot from this particular account request,
/// so it returns only the decoded account vector.
pub(super) fn get_multiple_accounts_compat(
    rpc: &RpcClient,
    source: &str,
    pubkeys: &[Pubkey],
) -> Result<Vec<Option<Account>>, String> {
    get_multiple_accounts_with_context_compat(rpc, source, pubkeys, None)
        .map(|batch| batch.accounts)
}

/// Read multiple accounts through raw JSON while preserving the actual RPC
/// context slot and optionally enforcing Solana `minContextSlot`.
///
/// B2 uses this form so its existing common-B1-min-context invariant remains
/// tied to provider-returned RPC context rather than a local clock or slot.
pub(super) fn get_multiple_accounts_with_context_compat(
    rpc: &RpcClient,
    source: &str,
    pubkeys: &[Pubkey],
    minimum_context_slot: Option<u64>,
) -> Result<Phase5AccountBatch, String> {
    let keys = pubkeys.iter().map(ToString::to_string).collect::<Vec<_>>();

    let mut config = json!({
        "encoding": "base64",
        "commitment": "confirmed",
    });

    if let Some(minimum_context_slot) = minimum_context_slot {
        config["minContextSlot"] = json!(minimum_context_slot);
    }

    let params = json!([keys, config,]);

    let raw = rpc
        .send::<Value>(RpcRequest::GetMultipleAccounts, params)
        .map_err(|error| format!("raw getMultipleAccounts request failed: {error}",))?;

    decode_account_batch(&raw, source)
}

fn decode_account_batch(result: &Value, source: &str) -> Result<Phase5AccountBatch, String> {
    let context_slot = result
        .get("context")
        .and_then(|context| context.get("slot"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            "getMultipleAccounts result is missing an exact u64 context slot".to_string()
        })?;

    let accounts = decode_account_values(result, source)?;

    Ok(Phase5AccountBatch {
        context_slot,
        accounts,
    })
}

fn decode_account_values(result: &Value, source: &str) -> Result<Vec<Option<Account>>, String> {
    let values = result
        .get("value")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            "getMultipleAccounts result is missing the account value array".to_string()
        })?;

    values
        .iter()
        .enumerate()
        .map(|(index, value)| decode_account(value, source, index))
        .collect()
}

fn decode_account(value: &Value, source: &str, index: usize) -> Result<Option<Account>, String> {
    if value.is_null() {
        return Ok(None);
    }

    let object = value
        .as_object()
        .ok_or_else(|| format!("account {index} is not a JSON object",))?;

    let lamports = object
        .get("lamports")
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("account {index} lamports is not an exact u64",))?;

    let owner = object
        .get("owner")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("account {index} owner is not a string",))?;

    let owner = Pubkey::from_str(owner)
        .map_err(|error| format!("account {index} owner is not a valid public key: {error}",))?;

    let executable = object
        .get("executable")
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("account {index} executable is not a boolean",))?;

    let rent_epoch = decode_rent_epoch(
        object
            .get("rentEpoch")
            .ok_or_else(|| format!("account {index} is missing rentEpoch",))?,
        source,
    )
    .map_err(|error| format!("account {index} rentEpoch rejected: {error}",))?;

    let data = object
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("account {index} data is not the required base64 tuple",))?;

    if data.len() != 2 {
        return Err(format!(
            "account {index} data must contain exactly payload and encoding",
        ));
    }

    let encoded = data[0]
        .as_str()
        .ok_or_else(|| format!("account {index} base64 payload is not a string",))?;

    let encoding = data[1]
        .as_str()
        .ok_or_else(|| format!("account {index} data encoding is not a string",))?;

    if encoding != "base64" {
        return Err(format!(
            "account {index} data encoding must be exactly base64",
        ));
    }

    let decoded = STANDARD
        .decode(encoded.as_bytes())
        .map_err(|error| format!("account {index} base64 payload is invalid: {error}",))?;

    Ok(Some(Account {
        lamports,
        data: decoded,
        owner,
        executable,
        rent_epoch,
    }))
}

fn decode_rent_epoch(value: &Value, source: &str) -> Result<u64, String> {
    if let Some(exact) = value.as_u64() {
        return Ok(exact);
    }

    let observed = value.as_f64().ok_or_else(|| {
        "rentEpoch is neither an exact u64 nor the allowed Uniblock sentinel".to_string()
    })?;

    let allowed_uniblock_sentinel = source == UNIBLOCK_PHASE5_SOURCE
        && observed.to_bits() == UNIBLOCK_RENT_EPOCH_WIRE_FLOAT.to_bits();

    if allowed_uniblock_sentinel {
        return Ok(u64::MAX);
    }

    Err(format!(
        "floating or out-of-range rentEpoch is forbidden for source {source}",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase5_wire_compat_normal_u64_rent_epoch_is_unchanged() {
        let value = serde_json::json!(42_u64);

        assert_eq!(
            decode_rent_epoch(&value, "solana-public-devnet-primary",)
                .expect("ordinary u64 rentEpoch should decode",),
            42,
        );
    }

    #[test]
    fn phase5_wire_compat_accepts_only_observed_uniblock_sentinel() {
        let observed: Value = serde_json::from_str("18446744073709552000")
            .expect("observed Uniblock wire fixture should parse");

        assert!(
            observed.as_u64().is_none(),
            "observed Uniblock value must remain outside u64 range",
        );

        assert_eq!(
            decode_rent_epoch(&observed, UNIBLOCK_PHASE5_SOURCE,)
                .expect("observed Uniblock sentinel should normalize",),
            u64::MAX,
        );

        assert!(
            decode_rent_epoch(&observed, "solana-public-devnet-primary",).is_err(),
            "same malformed number from another provider must fail closed",
        );
    }

    #[test]
    fn phase5_wire_compat_rejects_other_floating_rent_epochs() {
        let value = serde_json::json!(7.5_f64);

        assert!(decode_rent_epoch(&value, UNIBLOCK_PHASE5_SOURCE,).is_err(),);
    }

    #[test]
    fn phase5_wire_compat_decodes_account_without_relaxing_other_fields() {
        let response = serde_json::json!({
            "context": {
                "slot": 123_u64
            },
            "value": [
                {
                    "lamports": 123_u64,
                    "data": ["", "base64"],
                    "owner": "11111111111111111111111111111111",
                    "executable": false,
                    "rentEpoch": 1.8446744073709552e19_f64,
                    "space": 0
                }
            ]
        });

        let batch = decode_account_batch(&response, UNIBLOCK_PHASE5_SOURCE)
            .expect("fixture account batch should decode");

        assert_eq!(batch.context_slot, 123,);

        let account = batch.accounts[0]
            .as_ref()
            .expect("fixture account should exist");

        assert_eq!(account.lamports, 123,);

        assert_eq!(account.owner, Pubkey::default(),);

        assert!(!account.executable);

        assert_eq!(account.rent_epoch, u64::MAX,);

        assert!(account.data.is_empty());
    }

    #[test]
    fn phase5_wire_compat_rejects_non_integer_context_slot() {
        let response = serde_json::json!({
            "context": {
                "slot": 123.5_f64
            },
            "value": []
        });

        assert!(decode_account_batch(&response, UNIBLOCK_PHASE5_SOURCE,).is_err(),);
    }
}
