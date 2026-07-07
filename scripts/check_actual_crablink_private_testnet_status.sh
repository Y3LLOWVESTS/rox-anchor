#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 13 CrabLink display-only private testnet status checker.
# RO:WHY — Validates backend-derived display payloads without granting client authority, wallet/ledger authority, Solana submission, mint/burn, paid unlock, settlement, or finality.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md, .gitignore, Phase 12 checker.
# RO:INVARIANTS — display-only; private devnet/testnet only; test-only assets; no real ROC mutation; no final settlement.
# RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, transaction submission, mint, burn, settlement, paid unlock, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-cli --test actual_crablink_private_testnet_status.

fail() {
  printf 'FAIL: %s\n' "$*" >&2
  exit 1
}

ok() {
  printf 'ok: %s\n' "$*"
}

usage() {
  cat <<'USAGE'
usage:
  bash scripts/check_actual_crablink_private_testnet_status.sh --check-docs [repo-root]
  bash scripts/check_actual_crablink_private_testnet_status.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_crablink_private_testnet_status.sh --template-status [devnet|testnet]
  bash scripts/check_actual_crablink_private_testnet_status.sh --check-status <status-json>
USAGE
}

json_string_value() {
  local file="$1"
  local key="$2"
  sed -nE 's/^[[:space:]]*"'"$key"'"[[:space:]]*:[[:space:]]*"([^"]*)".*/\1/p' "$file" | head -n 1
}

contains_json_bool_true() {
  local file="$1"
  local key="$2"
  grep -Eq '"'"$key"'"[[:space:]]*:[[:space:]]*true([,[:space:]}]|$)' "$file"
}

require_valid_cluster() {
  case "${1:-}" in
    devnet|testnet) ok "cluster is $1" ;;
    *) fail "cluster must be devnet or testnet, got: ${1:-<empty>}" ;;
  esac
}

require_json_string() {
  local file="$1"
  local key="$2"
  local expected="$3"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ "$actual" = "$expected" ] || fail "CrabLink status $key expected '$expected' but found '${actual:-<missing>}'"
  ok "CrabLink status $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "CrabLink status missing non-empty string field: $key"
  ok "CrabLink status has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "CrabLink status must set $key true"
  ok "CrabLink status sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "CrabLink status contains forbidden true boolean: $key"
  fi
  ok "CrabLink status does not set $key true"
}

reject_sensitive_text() {
  local file="$1"
  local label="$2"

  for forbidden in \
    "/Users/" \
    "/home/" \
    "api-key=" \
    "apikey=" \
    "access_token=" \
    "privateKey" \
    "secretKey" \
    "seed phrase" \
    "mnemonic" \
    "payer.json" \
    "keypair.json" \
    "wallet.json" \
    "authority.json"
  do
    if grep -Fq "$forbidden" "$file"; then
      fail "$label contains unredacted secret/path marker: $forbidden"
    fi
  done

  ok "$label excludes unredacted secret/path markers"
}

require_status_value() {
  local value="$1"
  local field="$2"
  shift 2

  for allowed in "$@"; do
    if [ "$value" = "$allowed" ]; then
      ok "$field is valid: $value"
      return 0
    fi
  done

  fail "$field has invalid value: ${value:-<missing>}"
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md"
  local script="$root/scripts/check_actual_crablink_private_testnet_status.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_CRABLINK_STATUS.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_crablink_private_testnet_status.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 13" \
    "CrabLink Display-Only Private Testnet Status" \
    "rox-anchor.actual-crablink-private-testnet-status.v1" \
    "actual_crablink_private_testnet_display_status" \
    "backend-derived" \
    "display-only" \
    "test-only assets" \
    "proof_status" \
    "read_only_rpc_status" \
    "receipt_status" \
    "halt_status" \
    "recovery_status" \
    "dry_run_internal_roc_status" \
    "rustyonions_handoff_status" \
    "No Solana submit commands in CrabLink." \
    "No ROX mint/burn authority in CrabLink." \
    "No paid content unlock from private testnet status." \
    "No wallet authority." \
    "No ledger authority." \
    "No bridge authority." \
    "No real ROC mutation." \
    "No production bridge settlement." \
    "No final settlement." \
    "CrabLink status remains display-only."
  do
    grep -Fq -- "$needle" "$doc" || fail "CrabLink status doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-crablink-private-testnet-status.json" \
    "actual-crablink-private-testnet-status.local.json" \
    "actual-crablink-display-only-status.local.json" \
    "actual-crablink-status-report.local.json" \
    "*.actual-crablink-private-testnet-status.local.json" \
    "*.actual-crablink-display-only-status.local.json" \
    "*.actual-crablink-status-report.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing Phase 13 ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "CrabLink status doc"

  for forbidden in \
    "backend_derived\": false" \
    "display_only\": false" \
    "private_testnet_only\": false" \
    "test_only_assets_only\": false" \
    "dry_run_only\": false" \
    "client_authority\": true" \
    "wallet_authority\": true" \
    "ledger_authority\": true" \
    "bridge_authority\": true" \
    "solana_submit_command_available\": true" \
    "rox_mint_burn_authority\": true" \
    "paid_content_unlock\": true" \
    "real_roc_burn\": true" \
    "real_roc_release\": true" \
    "real_roc_mutation\": true" \
    "production_bridge_settlement\": true" \
    "final_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "mainnet_authorized\": true" \
    "public_launch_authorized\": true" \
    "public_bridge_ui\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "CrabLink status doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 13 CrabLink display-only status documentation checks passed
summary:
  - CrabLink display-only status runbook exists
  - local Phase 13 display status artifact names are ignored
  - documentation preserves backend-derived, display-only, test-only, private-testnet boundaries
  - documentation separates CrabLink status from client authority, wallet authority, ledger authority, bridge authority, Solana submission, ROX mint/burn authority, paid content unlock, real ROC mutation, production settlement, final settlement, mainnet, public launch, and finality
SUMMARY
}

template_status() {
  local cluster="${1:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-crablink-private-testnet-status.v1",
  "phase": "BUILD_PLAN4 Phase 13",
  "status_role": "actual_crablink_private_testnet_display_status",
  "cluster": "$cluster",
  "display_status": "display_only",
  "proof_status": "accepted",
  "read_only_rpc_status": "verified",
  "receipt_status": "linked",
  "halt_status": "not_active",
  "recovery_status": "not_required",
  "dry_run_internal_roc_status": "dry_run_only",
  "rustyonions_handoff_status": "dry_run_recorded",
  "test_only_asset_label": "TEST-ONLY ROX",
  "private_testnet_label": "PRIVATE TESTNET STATUS",
  "backend_derived": true,
  "display_only": true,
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "dry_run_only": true,
  "client_authority": false,
  "wallet_authority": false,
  "ledger_authority": false,
  "bridge_authority": false,
  "solana_submit_command_available": false,
  "rox_mint_burn_authority": false,
  "paid_content_unlock": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "production_bridge_settlement": false,
  "final_settlement": false,
  "public_rox_mint_burn": false,
  "mainnet_authorized": false,
  "public_launch_authorized": false,
  "public_bridge_ui": false,
  "finality_claim": false,
  "operator_report_redacted": true
}
TEMPLATE
}

check_status() {
  local status="${1:-}"
  [ -n "$status" ] || fail "--check-status requires a status path"
  [ -f "$status" ] || fail "CrabLink status file not found: $status"

  reject_sensitive_text "$status" "CrabLink private testnet status"

  require_json_string "$status" "schema" "rox-anchor.actual-crablink-private-testnet-status.v1"
  require_json_string "$status" "phase" "BUILD_PLAN4 Phase 13"
  require_json_string "$status" "status_role" "actual_crablink_private_testnet_display_status"

  require_valid_cluster "$(json_string_value "$status" "cluster")"

  for field in \
    display_status proof_status read_only_rpc_status receipt_status halt_status recovery_status \
    dry_run_internal_roc_status rustyonions_handoff_status test_only_asset_label private_testnet_label
  do
    require_json_string_present "$status" "$field"
  done

  require_status_value "$(json_string_value "$status" "display_status")" "display_status" display_only unavailable blocked
  require_status_value "$(json_string_value "$status" "proof_status")" "proof_status" accepted blocked rejected disputed missing_evidence unavailable
  require_status_value "$(json_string_value "$status" "read_only_rpc_status")" "read_only_rpc_status" verified blocked disputed missing_evidence unavailable
  require_status_value "$(json_string_value "$status" "receipt_status")" "receipt_status" linked quarantined missing unavailable
  require_status_value "$(json_string_value "$status" "halt_status")" "halt_status" not_active active unavailable
  require_status_value "$(json_string_value "$status" "recovery_status")" "recovery_status" not_required required validated unavailable
  require_status_value "$(json_string_value "$status" "dry_run_internal_roc_status")" "dry_run_internal_roc_status" dry_run_only blocked unavailable
  require_status_value "$(json_string_value "$status" "rustyonions_handoff_status")" "rustyonions_handoff_status" dry_run_recorded blocked quarantined unavailable

  require_json_bool_true "$status" "backend_derived"
  require_json_bool_true "$status" "display_only"
  require_json_bool_true "$status" "private_testnet_only"
  require_json_bool_true "$status" "test_only_assets_only"
  require_json_bool_true "$status" "dry_run_only"
  require_json_bool_true "$status" "operator_report_redacted"

  for field in \
    client_authority wallet_authority ledger_authority bridge_authority \
    solana_submit_command_available rox_mint_burn_authority paid_content_unlock \
    real_roc_burn real_roc_release real_roc_mutation production_bridge_settlement \
    final_settlement public_rox_mint_burn mainnet_authorized public_launch_authorized \
    public_bridge_ui finality_claim
  do
    require_json_bool_false_or_absent "$status" "$field"
  done

  for forbidden in \
    '"display_status": "final"' \
    '"display_status": "settled"' \
    '"proof_status": "finalized"' \
    '"read_only_rpc_status": "submitted"' \
    '"receipt_status": "settled"' \
    '"dry_run_internal_roc_status": "mutated"' \
    '"rustyonions_handoff_status": "settled"' \
    '"paid_access": "unlocked"' \
    '"finality": "finalized"'
  do
    if grep -Fq "$forbidden" "$status"; then
      fail "CrabLink status contains forbidden success/authority marker: $forbidden"
    fi
    ok "CrabLink status excludes success/authority marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 13 CrabLink display-only status checks passed
summary:
  - status is devnet/testnet only
  - status is backend-derived and display-only
  - status labels test-only assets and private testnet evidence
  - status rejects client authority, wallet authority, ledger authority, bridge authority, Solana submission, ROX mint/burn authority, paid content unlock, real ROC burn/release/mutation, production settlement, final settlement, public ROX mint/burn, mainnet, public launch, public bridge UI, and finality claims
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"

  require_valid_cluster "$cluster" >/dev/null
  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/BUILD_PLAN4.md" ] || fail "BUILD_PLAN4.md missing"
  [ -f "$root/scripts/check_actual_rustyonions_dry_run_handoff.sh" ] || fail "Phase 12 RustyOnions handoff checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_receipts.sh" ] || fail "Phase 9 receipt checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh" ] || fail "Phase 11 halt/recovery/authority checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-crablink-.*status.*\.json|.*crablink.*private-testnet.*status.*\.json)$'; then
      fail "git tracked Phase 13 CrabLink status material found"
    fi
    ok "git tracked-file scan found no Phase 13 CrabLink status material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 13 CrabLink display-only status preflight passed
summary:
  - Phase 13 documentation and ignore boundaries are present
  - Phase 9, Phase 11, and Phase 12 checkers exist
  - no tracked Phase 13 CrabLink status material was found
  - this preflight did not call RPC, submit, sign, load a signer, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, mutate ROC, unlock paid content, or grant client authority
SUMMARY
}

case "${1:-}" in
  --help|-h)
    usage
    ;;
  --check-docs)
    check_docs "${2:-.}"
    ;;
  --preflight)
    preflight "${2:-.}" "${3:-testnet}"
    ;;
  --template-status)
    template_status "${2:-testnet}"
    ;;
  --check-status)
    check_status "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
