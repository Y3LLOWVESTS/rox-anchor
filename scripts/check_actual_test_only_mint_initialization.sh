#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 4 actual test-only mint/config initialization receipt checker.
# RO:WHY — Validates redacted initialization/readback receipts without initializing, submitting, minting, burning, or calling RPC.
# RO:INTERACTS — docs/pilot/ACTUAL_TEST_ONLY_MINT_INITIALIZATION.md, .gitignore, ignored local receipts.
# RO:INVARIANTS — devnet/testnet only; test-only labels; tiny caps; separated authorities; no public/mainnet/finality/real ROC claims.
# RO:SECURITY — read-only local file checks only; no wallet load, signing, RPC, submission, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-cli --test actual_test_only_mint_initialization and cargo test -p rox-anchor-rpc-proof --test actual_test_only_mint_readback.

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
  bash scripts/check_actual_test_only_mint_initialization.sh --check-docs [repo-root]
  bash scripts/check_actual_test_only_mint_initialization.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_test_only_mint_initialization.sh --template-init-success [devnet|testnet]
  bash scripts/check_actual_test_only_mint_initialization.sh --template-init-failure [devnet|testnet]
  bash scripts/check_actual_test_only_mint_initialization.sh --template-readback [devnet|testnet]
  bash scripts/check_actual_test_only_mint_initialization.sh --check-init-receipt <receipt-json>
  bash scripts/check_actual_test_only_mint_initialization.sh --check-readback-receipt <receipt-json>
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

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"

  if contains_json_bool_true "$file" "$key"; then
    fail "receipt contains forbidden true boolean: $key"
  fi

  ok "receipt does not set $key true"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"

  contains_json_bool_true "$file" "$key" || fail "receipt must set $key true"
  ok "receipt sets $key true"
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

reject_public_or_production_label() {
  local value="$1"
  local field="$2"

  local lower
  lower="$(printf '%s' "$value" | tr '[:upper:]' '[:lower:]')"

  case "$lower" in
    *public*|*production*|*mainnet*|*release*|*launch*)
      fail "$field must stay test-only/private-testnet, got: $value"
      ;;
  esac

  case "$lower" in
    *test-only*|*private-testnet*|*private-test*)
      ok "$field is test-only/private-testnet"
      ;;
    *)
      fail "$field must contain a test-only/private-testnet marker, got: $value"
      ;;
  esac
}

require_tiny_positive_cap() {
  local value="$1"
  local field="$2"
  local max="$3"

  printf '%s' "$value" | grep -Eq '^[0-9]+$' || fail "$field must be an integer string"
  [ "$value" -gt 0 ] || fail "$field must be positive"
  [ "$value" -le "$max" ] || fail "$field exceeds cap $max: $value"
  ok "$field is positive and capped"
}

require_redacted_value() {
  local value="$1"
  local field="$2"

  case "$value" in
    *redacted*|"<redacted-"*) ok "$field is redacted" ;;
    *) fail "$field must be a redacted placeholder" ;;
  esac
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_TEST_ONLY_MINT_INITIALIZATION.md"
  local script="$root/scripts/check_actual_test_only_mint_initialization.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_TEST_ONLY_MINT_INITIALIZATION.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_test_only_mint_initialization.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 4" \
    "Actual Test-Only ROX Mint and Program Config Initialization" \
    "test_only_mint_initialization_receipt" \
    "test_only_mint_readback_receipt" \
    "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT" \
    "max_supply_units" \
    "max_amount_units_per_operation" \
    "mint_authority_redacted" \
    "halt_authority_redacted" \
    "recovery_authority_redacted" \
    "readback_required" \
    "readback_verified" \
    "public_mint_available" \
    "No public launch authorization." \
    "No mainnet-beta authorization." \
    "No real internal ROC release."
  do
    grep -Fq "$needle" "$doc" || fail "initialization doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-test-only-mint-init-receipt.json" \
    "actual-test-only-mint-init-receipt.local.json" \
    "actual-test-only-mint-init-failed.local.json" \
    "actual-test-only-mint-readback.local.json" \
    "*.actual-test-only-mint-init.local.json" \
    "*.actual-test-only-mint-init-receipt.local.json" \
    "*.actual-test-only-mint-readback.local.json" \
    "*.actual-program-config-init.local.json" \
    "*.actual-program-config-readback.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing init receipt ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "initialization doc"

  for forbidden in \
    "public_mint_available\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_mutation\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "initialization doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 4 test-only mint initialization documentation checks passed
summary:
  - actual test-only mint/config initialization runbook exists
  - local initialization/readback receipt names are ignored
  - documentation preserves test-only, tiny-cap, external-authority, non-mainnet boundaries
  - documentation separates initialization attempt evidence from readback evidence and public availability
SUMMARY
}

print_init_template() {
  local outcome="$1"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster"

  local signature="none"
  local slot="none"
  local failure="<redacted-safe-failure-reason>"
  local readback_required="false"

  if [ "$outcome" = "succeeded" ]; then
    signature="<redacted-signature>"
    slot="0"
    failure="not_applicable"
    readback_required="true"
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-test-only-mint-initialization.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_initialization_receipt",
  "cluster": "$cluster",
  "program_name": "rox_anchor",
  "program_id": "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
  "initialization_outcome": "$outcome",
  "operation_id": "actual-test-only-init-0001",
  "idempotency_key": "actual-test-only-init-idem-0001",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "program_config_account": "<redacted-program-config-account>",
  "max_supply_units": "1000",
  "max_amount_units_per_operation": "1",
  "mint_authority_redacted": "<redacted-external-mint-authority>",
  "halt_authority_redacted": "<redacted-external-halt-authority>",
  "recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "init_signature": "$signature",
  "init_slot": "$slot",
  "failure_reason_redacted": "$failure",
  "operator_approval": "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT",
  "manual_operator_action": true,
  "preflight_passed": true,
  "readback_required": $readback_required,
  "readback_verified": false,
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

print_readback_template() {
  local cluster="${1:-testnet}"
  require_valid_cluster "$cluster"

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-test-only-mint-readback.v1",
  "phase": "BUILD_PLAN4 Phase 4",
  "receipt_role": "test_only_mint_readback_receipt",
  "cluster": "$cluster",
  "program_name": "rox_anchor",
  "program_id": "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
  "readback_outcome": "verified",
  "readback_slot": "0",
  "program_config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "observed_test_only_mint_label": "test-only-rox-private-testnet",
  "observed_token_account_label": "test-only-rox-token-account-private-testnet",
  "observed_max_supply_units": "1000",
  "observed_max_amount_units_per_operation": "1",
  "observed_mint_authority_redacted": "<redacted-external-mint-authority>",
  "observed_halt_authority_redacted": "<redacted-external-halt-authority>",
  "observed_recovery_authority_redacted": "<redacted-external-recovery-authority>",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
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

check_init_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-init-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "initialization receipt not found: $receipt"

  reject_sensitive_text "$receipt" "initialization receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-test-only-mint-initialization.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 4"
  require_json_string "$receipt" "receipt_role" "test_only_mint_initialization_receipt"
  require_json_string "$receipt" "program_name" "rox_anchor"
  require_json_string "$receipt" "operator_approval" "I_APPROVE_PRIVATE_TESTNET_TEST_ONLY_INIT"

  local cluster
  cluster="$(json_string_value "$receipt" "cluster")"
  require_valid_cluster "$cluster"
  ok "receipt cluster is $cluster"

  local outcome
  outcome="$(json_string_value "$receipt" "initialization_outcome")"
  case "$outcome" in
    succeeded|failed) ok "receipt initialization_outcome = $outcome" ;;
    *) fail "initialization_outcome must be succeeded or failed, got: ${outcome:-<missing>}" ;;
  esac

  require_json_string_present "$receipt" "program_id"
  require_json_string_present "$receipt" "operation_id"
  require_json_string_present "$receipt" "idempotency_key"
  require_json_string_present "$receipt" "test_only_mint"
  require_json_string_present "$receipt" "test_only_token_account"
  require_json_string_present "$receipt" "program_config_account"
  require_json_string_present "$receipt" "upgrade_authority_policy"
  require_json_string_present "$receipt" "init_signature"
  require_json_string_present "$receipt" "init_slot"
  require_json_string_present "$receipt" "failure_reason_redacted"

  local mint_label token_label max_supply max_amount
  mint_label="$(json_string_value "$receipt" "test_only_mint_label")"
  token_label="$(json_string_value "$receipt" "test_only_token_account_label")"
  max_supply="$(json_string_value "$receipt" "max_supply_units")"
  max_amount="$(json_string_value "$receipt" "max_amount_units_per_operation")"

  reject_public_or_production_label "$mint_label" "test_only_mint_label"
  reject_public_or_production_label "$token_label" "test_only_token_account_label"
  require_tiny_positive_cap "$max_supply" "max_supply_units" 1000000
  require_tiny_positive_cap "$max_amount" "max_amount_units_per_operation" 1000

  require_redacted_value "$(json_string_value "$receipt" "test_only_mint")" "test_only_mint"
  require_redacted_value "$(json_string_value "$receipt" "test_only_token_account")" "test_only_token_account"
  require_redacted_value "$(json_string_value "$receipt" "program_config_account")" "program_config_account"
  require_redacted_value "$(json_string_value "$receipt" "mint_authority_redacted")" "mint_authority_redacted"
  require_redacted_value "$(json_string_value "$receipt" "halt_authority_redacted")" "halt_authority_redacted"
  require_redacted_value "$(json_string_value "$receipt" "recovery_authority_redacted")" "recovery_authority_redacted"

  if [ "$outcome" = "succeeded" ]; then
    [ "$(json_string_value "$receipt" "init_signature")" != "none" ] || fail "succeeded receipt requires init_signature"
    [ "$(json_string_value "$receipt" "init_slot")" != "none" ] || fail "succeeded receipt requires init_slot"
    require_json_bool_true "$receipt" "readback_required"
  else
    require_json_string "$receipt" "init_signature" "none"
    require_json_string "$receipt" "init_slot" "none"
    require_json_bool_false_or_absent "$receipt" "readback_required"
  fi

  require_json_bool_true "$receipt" "manual_operator_action"
  require_json_bool_true "$receipt" "preflight_passed"
  require_json_bool_false_or_absent "$receipt" "readback_verified"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 4 initialization receipt checks passed
summary:
  - receipt is devnet/testnet only
  - receipt records succeeded or failed test-only initialization attempt evidence
  - labels are test-only/private-testnet
  - supply and per-operation caps are tiny
  - authorities and account identifiers are redacted
  - receipt does not claim public mint availability, public launch, mainnet, production settlement, public ROX mint/burn, real ROC mutation, or finality
SUMMARY
}

check_readback_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-readback-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "readback receipt not found: $receipt"

  reject_sensitive_text "$receipt" "readback receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-test-only-mint-readback.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 4"
  require_json_string "$receipt" "receipt_role" "test_only_mint_readback_receipt"
  require_json_string "$receipt" "program_name" "rox_anchor"
  require_json_string "$receipt" "readback_outcome" "verified"

  local cluster
  cluster="$(json_string_value "$receipt" "cluster")"
  require_valid_cluster "$cluster"
  ok "receipt cluster is $cluster"

  require_json_string_present "$receipt" "program_id"
  require_json_string_present "$receipt" "readback_slot"
  require_json_string_present "$receipt" "program_config_account"
  require_json_string_present "$receipt" "test_only_mint"
  require_json_string_present "$receipt" "test_only_token_account"
  require_json_string_present "$receipt" "rpc_evidence_redacted"

  local mint_label token_label max_supply max_amount
  mint_label="$(json_string_value "$receipt" "observed_test_only_mint_label")"
  token_label="$(json_string_value "$receipt" "observed_token_account_label")"
  max_supply="$(json_string_value "$receipt" "observed_max_supply_units")"
  max_amount="$(json_string_value "$receipt" "observed_max_amount_units_per_operation")"

  reject_public_or_production_label "$mint_label" "observed_test_only_mint_label"
  reject_public_or_production_label "$token_label" "observed_token_account_label"
  require_tiny_positive_cap "$max_supply" "observed_max_supply_units" 1000000
  require_tiny_positive_cap "$max_amount" "observed_max_amount_units_per_operation" 1000

  require_redacted_value "$(json_string_value "$receipt" "program_config_account")" "program_config_account"
  require_redacted_value "$(json_string_value "$receipt" "test_only_mint")" "test_only_mint"
  require_redacted_value "$(json_string_value "$receipt" "test_only_token_account")" "test_only_token_account"
  require_redacted_value "$(json_string_value "$receipt" "observed_mint_authority_redacted")" "observed_mint_authority_redacted"
  require_redacted_value "$(json_string_value "$receipt" "observed_halt_authority_redacted")" "observed_halt_authority_redacted"
  require_redacted_value "$(json_string_value "$receipt" "observed_recovery_authority_redacted")" "observed_recovery_authority_redacted"
  require_redacted_value "$(json_string_value "$receipt" "rpc_evidence_redacted")" "rpc_evidence_redacted"

  require_json_bool_true "$receipt" "read_only_rpc"
  require_json_bool_false_or_absent "$receipt" "transaction_submission"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 4 readback receipt checks passed
summary:
  - readback receipt is devnet/testnet only
  - readback is explicitly read-only RPC evidence
  - observed labels remain test-only/private-testnet
  - observed caps remain tiny
  - observed authorities and evidence are redacted
  - receipt does not claim submission, public mint availability, public launch, mainnet, production settlement, public ROX mint/burn, real ROC mutation, or finality
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
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-test-only-mint.*\.json|.*actual-program-config.*\.json)$'; then
      fail "git tracked test-only mint/config receipt material found"
    fi
    ok "git tracked-file scan found no actual test-only mint/config receipt material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 4 test-only mint initialization preflight passed
summary:
  - initialization documentation and ignore boundaries are present
  - Anchor build outputs exist
  - no tracked test-only mint/config receipt material was found
  - this preflight did not initialize, submit, call RPC, sign, mint, burn, settle, mutate ROC, or load a wallet
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
  --template-init-success)
    print_init_template "succeeded" "${2:-testnet}"
    ;;
  --template-init-failure)
    print_init_template "failed" "${2:-testnet}"
    ;;
  --template-readback)
    print_readback_template "${2:-testnet}"
    ;;
  --check-init-receipt)
    check_init_receipt "${2:-}"
    ;;
  --check-readback-receipt)
    check_readback_receipt "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
