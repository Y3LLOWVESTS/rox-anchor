#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 5 actual private testnet read-only RPC evidence receipt checker.
# RO:WHY — Validates redacted read-only evidence receipts without calling RPC, signing, submitting, minting, burning, or mutating ROC.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_READ_ONLY_EVIDENCE.md, .gitignore, ignored local receipts.
# RO:INVARIANTS — devnet/testnet only; read-only RPC true; transaction submission false; quorum-shaped evidence; no public/mainnet/finality/real ROC claims.
# RO:SECURITY — local file checks only; no wallet load, RPC, signing, submission, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-rpc-proof --test actual_private_testnet_read_only_rpc and cargo test -p rox-anchor-cli --test actual_private_testnet_read_only_command.

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
  bash scripts/check_actual_private_testnet_read_only_evidence.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_read_only_evidence.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_read_only_evidence.sh --template-verified [devnet|testnet]
  bash scripts/check_actual_private_testnet_read_only_evidence.sh --template-failed [devnet|testnet]
  bash scripts/check_actual_private_testnet_read_only_evidence.sh --check-evidence-receipt <receipt-json>
USAGE
}

valid_cluster() {
  case "${1:-}" in
    devnet|testnet) return 0 ;;
    *) return 1 ;;
  esac
}

require_valid_cluster() {
  local cluster="${1:-}"
  valid_cluster "$cluster" || fail "cluster must be devnet or testnet, got: ${cluster:-<empty>}"
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

require_json_string() {
  local file="$1"
  local key="$2"
  local expected="$3"

  local actual
  actual="$(json_string_value "$file" "$key")"

  if [ "$actual" = "$expected" ]; then
    ok "receipt $key = $expected"
  else
    fail "receipt $key expected '$expected' but found '${actual:-<missing>}'"
  fi
}

require_json_string_present() {
  local file="$1"
  local key="$2"

  local actual
  actual="$(json_string_value "$file" "$key")"

  [ -n "$actual" ] || fail "receipt missing non-empty string field: $key"
  ok "receipt has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"

  contains_json_bool_true "$file" "$key" || fail "receipt must set $key true"
  ok "receipt sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"

  if contains_json_bool_true "$file" "$key"; then
    fail "receipt contains forbidden true boolean: $key"
  fi

  ok "receipt does not set $key true"
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
    "BEGIN PRIVATE KEY" \
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

require_integer_string() {
  local value="$1"
  local field="$2"

  printf '%s' "$value" | grep -Eq '^[0-9]+$' || fail "$field must be an integer string"
  ok "$field is an integer string"
}

require_redacted_value() {
  local value="$1"
  local field="$2"

  case "$value" in
    *redacted*|"<redacted-"*) ok "$field is redacted" ;;
    *) fail "$field must be a redacted placeholder" ;;
  esac
}

require_status_value() {
  local value="$1"
  local field="$2"

  case "$value" in
    exists|exists-executable|confirmed|not_checked|missing|mismatched|stale|disputed|failed)
      ok "$field has accepted status"
      ;;
    *)
      fail "$field has unsupported status: ${value:-<missing>}"
      ;;
  esac
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_READ_ONLY_EVIDENCE.md"
  local script="$root/scripts/check_actual_private_testnet_read_only_evidence.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_READ_ONLY_EVIDENCE.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_read_only_evidence.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 5" \
    "Live Read-Only RPC Evidence Against Deployed Accounts" \
    "private_testnet_read_only_rpc_evidence_receipt" \
    "read_only_rpc" \
    "transaction_submission" \
    "rpc_sources_count" \
    "rpc_quorum_threshold" \
    "rpc_matching_sources_count" \
    "program_account_status" \
    "config_account_status" \
    "mint_account_status" \
    "token_account_status" \
    "No transaction submission." \
    "No public launch authorization." \
    "No mainnet-beta authorization." \
    "No real internal ROC release."
  do
    grep -Fq "$needle" "$doc" || fail "read-only evidence doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-read-only-evidence.json" \
    "actual-private-testnet-read-only-evidence.local.json" \
    "actual-private-testnet-read-only-evidence-failed.local.json" \
    "*.actual-private-testnet-read-only-evidence.local.json" \
    "*.actual-private-testnet-read-only-evidence-failed.local.json" \
    "*.actual-read-only-rpc-evidence.local.json" \
    "*.actual-rpc-readback.local.json" \
    "*.actual-program-readback.local.json" \
    "*.actual-config-readback.local.json" \
    "*.actual-mint-readback.local.json" \
    "*.actual-token-account-readback.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing read-only evidence ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "read-only evidence doc"

  for forbidden in \
    "read_only_rpc\": false" \
    "transaction_submission\": true" \
    "wallet_loaded\": true" \
    "signature_generated\": true" \
    "public_mint_available\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_mutation\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "read-only evidence doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 5 read-only RPC evidence documentation checks passed
summary:
  - actual private testnet read-only RPC evidence runbook exists
  - local read-only evidence receipt names are ignored
  - documentation preserves read-only, redacted, non-submitting, non-mainnet boundaries
  - documentation separates read-only evidence from transaction submission, finality, settlement, and public mint availability
SUMMARY
}

print_template() {
  local outcome="$1"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster"

  local program_status="exists-executable"
  local config_status="exists"
  local mint_status="exists"
  local token_status="exists"
  local deploy_status="confirmed"
  local init_status="confirmed"
  local matches="2"
  local disputed="0"
  local failure_line=""

  if [ "$outcome" != "verified" ]; then
    program_status="missing"
    config_status="not_checked"
    mint_status="not_checked"
    token_status="not_checked"
    deploy_status="not_checked"
    init_status="not_checked"
    matches="0"
    failure_line='  "failure_reason_redacted": "<redacted-safe-read-only-failure-reason>",'
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-read-only-evidence.v1",
  "phase": "BUILD_PLAN4 Phase 5",
  "receipt_role": "private_testnet_read_only_rpc_evidence_receipt",
  "cluster": "$cluster",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "evidence_outcome": "$outcome",
  "current_slot": "1000",
  "program_account": "<redacted-program-account>",
  "program_account_status": "$program_status",
  "program_account_slot": "1000",
  "config_account": "<redacted-program-config-account>",
  "config_account_status": "$config_status",
  "config_account_slot": "1000",
  "test_only_mint": "<redacted-test-only-mint>",
  "mint_account_status": "$mint_status",
  "mint_account_slot": "1000",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "token_account_status": "$token_status",
  "token_account_slot": "1000",
  "deploy_signature_status": "$deploy_status",
  "initialization_signature_status": "$init_status",
  "rpc_sources_count": "2",
  "rpc_quorum_threshold": "2",
  "rpc_matching_sources_count": "$matches",
  "rpc_disputed_sources_count": "$disputed",
  "max_observation_lag_slots": "150",
$failure_line
  "rpc_provider_labels_redacted": "<redacted-rpc-provider-labels>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
TEMPLATE
}

check_evidence_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-evidence-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "read-only evidence receipt not found: $receipt"

  reject_sensitive_text "$receipt" "read-only evidence receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-private-testnet-read-only-evidence.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 5"
  require_json_string "$receipt" "receipt_role" "private_testnet_read_only_rpc_evidence_receipt"
  require_json_string "$receipt" "program_name" "rox_anchor"

  local cluster
  cluster="$(json_string_value "$receipt" "cluster")"
  require_valid_cluster "$cluster"
  ok "receipt cluster is $cluster"

  local outcome
  outcome="$(json_string_value "$receipt" "evidence_outcome")"
  case "$outcome" in
    verified|failed|disputed|stale|missing) ok "receipt evidence_outcome = $outcome" ;;
    *) fail "evidence_outcome must be verified, failed, disputed, stale, or missing, got: ${outcome:-<missing>}" ;;
  esac

  for field in \
    program_id \
    current_slot \
    program_account \
    program_account_status \
    program_account_slot \
    config_account \
    config_account_status \
    config_account_slot \
    test_only_mint \
    mint_account_status \
    mint_account_slot \
    test_only_token_account \
    token_account_status \
    token_account_slot \
    deploy_signature_status \
    initialization_signature_status \
    rpc_sources_count \
    rpc_quorum_threshold \
    rpc_matching_sources_count \
    rpc_disputed_sources_count \
    max_observation_lag_slots \
    rpc_provider_labels_redacted
  do
    require_json_string_present "$receipt" "$field"
  done

  require_redacted_value "$(json_string_value "$receipt" "program_account")" "program_account"
  require_redacted_value "$(json_string_value "$receipt" "config_account")" "config_account"
  require_redacted_value "$(json_string_value "$receipt" "test_only_mint")" "test_only_mint"
  require_redacted_value "$(json_string_value "$receipt" "test_only_token_account")" "test_only_token_account"
  require_redacted_value "$(json_string_value "$receipt" "rpc_provider_labels_redacted")" "rpc_provider_labels_redacted"

  for field in \
    current_slot \
    program_account_slot \
    config_account_slot \
    mint_account_slot \
    token_account_slot \
    rpc_sources_count \
    rpc_quorum_threshold \
    rpc_matching_sources_count \
    rpc_disputed_sources_count \
    max_observation_lag_slots
  do
    require_integer_string "$(json_string_value "$receipt" "$field")" "$field"
  done

  require_status_value "$(json_string_value "$receipt" "program_account_status")" "program_account_status"
  require_status_value "$(json_string_value "$receipt" "config_account_status")" "config_account_status"
  require_status_value "$(json_string_value "$receipt" "mint_account_status")" "mint_account_status"
  require_status_value "$(json_string_value "$receipt" "token_account_status")" "token_account_status"
  require_status_value "$(json_string_value "$receipt" "deploy_signature_status")" "deploy_signature_status"
  require_status_value "$(json_string_value "$receipt" "initialization_signature_status")" "initialization_signature_status"

  local sources threshold matches disputed lag
  sources="$(json_string_value "$receipt" "rpc_sources_count")"
  threshold="$(json_string_value "$receipt" "rpc_quorum_threshold")"
  matches="$(json_string_value "$receipt" "rpc_matching_sources_count")"
  disputed="$(json_string_value "$receipt" "rpc_disputed_sources_count")"
  lag="$(json_string_value "$receipt" "max_observation_lag_slots")"

  [ "$sources" -ge 1 ] || fail "rpc_sources_count must be positive"
  [ "$threshold" -ge 1 ] || fail "rpc_quorum_threshold must be positive"
  [ "$threshold" -le "$sources" ] || fail "rpc_quorum_threshold cannot exceed rpc_sources_count"
  [ "$matches" -le "$sources" ] || fail "rpc_matching_sources_count cannot exceed rpc_sources_count"
  [ "$disputed" -le "$sources" ] || fail "rpc_disputed_sources_count cannot exceed rpc_sources_count"
  [ "$lag" -le 10000 ] || fail "max_observation_lag_slots is too large"
  ok "RPC quorum counts are bounded"

  if [ "$outcome" = "verified" ]; then
    [ "$matches" -ge "$threshold" ] || fail "verified evidence requires matching sources to meet quorum threshold"
    [ "$(json_string_value "$receipt" "program_account_status")" = "exists-executable" ] || fail "verified evidence requires executable program account"
    [ "$(json_string_value "$receipt" "config_account_status")" = "exists" ] || fail "verified evidence requires config account"
    [ "$(json_string_value "$receipt" "mint_account_status")" = "exists" ] || fail "verified evidence requires test-only mint account"
    [ "$(json_string_value "$receipt" "token_account_status")" = "exists" ] || fail "verified evidence requires test-only token account"
    ok "verified evidence satisfies account and quorum requirements"
  else
    [ "$matches" -lt "$threshold" ] || fail "non-verified evidence must not meet quorum threshold"
    require_json_string_present "$receipt" "failure_reason_redacted"
    require_redacted_value "$(json_string_value "$receipt" "failure_reason_redacted")" "failure_reason_redacted"
  fi

  require_json_bool_true "$receipt" "read_only_rpc"
  require_json_bool_false_or_absent "$receipt" "transaction_submission"
  require_json_bool_false_or_absent "$receipt" "wallet_loaded"
  require_json_bool_false_or_absent "$receipt" "signature_generated"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 5 read-only RPC evidence receipt checks passed
summary:
  - receipt is devnet/testnet only
  - receipt is explicitly read-only RPC evidence
  - account identifiers and provider labels are redacted
  - RPC quorum counts are bounded
  - verified evidence requires executable program, config, mint, token account, and quorum
  - receipt does not claim submission, wallet load, signing, public mint availability, public launch, mainnet, production settlement, public ROX mint/burn, real ROC mutation, or finality
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"

  require_valid_cluster "$cluster"
  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/Anchor.toml" ] || fail "Anchor.toml missing"
  [ -f "$root/target/deploy/rox_anchor.so" ] || fail "target/deploy/rox_anchor.so missing; run anchor build first"
  [ -f "$root/target/idl/rox_anchor.json" ] || fail "target/idl/rox_anchor.json missing; run anchor build first"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-read-only-evidence.*\.json|.*actual-rpc-readback.*\.json|.*actual-program-readback.*\.json|.*actual-config-readback.*\.json|.*actual-mint-readback.*\.json|.*actual-token-account-readback.*\.json)$'; then
      fail "git tracked read-only RPC evidence material found"
    fi
    ok "git tracked-file scan found no actual read-only RPC evidence material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 5 read-only RPC evidence preflight passed
summary:
  - read-only evidence documentation and ignore boundaries are present
  - Anchor build outputs exist
  - no tracked read-only RPC evidence material was found
  - this preflight did not call RPC, submit, sign, load a wallet, initialize, mint, burn, settle, or mutate ROC
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
  --template-verified)
    print_template "verified" "${2:-testnet}"
    ;;
  --template-failed)
    print_template "failed" "${2:-testnet}"
    ;;
  --check-evidence-receipt)
    check_evidence_receipt "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
