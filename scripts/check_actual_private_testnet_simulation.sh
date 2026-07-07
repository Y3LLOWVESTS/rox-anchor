#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 6 actual private testnet simulation receipt checker.
# RO:WHY — Validates redacted simulate-only receipts without calling RPC, simulating live, signing, submitting, minting, burning, or mutating ROC.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_SIMULATION.md, .gitignore, ignored local simulation receipts.
# RO:INVARIANTS — devnet/testnet only; simulate-only true; transaction submission false; send authorization false; test-only labels; tiny caps.
# RO:SECURITY — local file checks only; no wallet load, RPC, live simulation, signing, submission, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-relayer --test actual_private_testnet_simulation.

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
  bash scripts/check_actual_private_testnet_simulation.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_simulation.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_simulation.sh --template-simulated [roc_to_rox|rox_to_roc] [devnet|testnet]
  bash scripts/check_actual_private_testnet_simulation.sh --template-blocked [roc_to_rox|rox_to_roc] [devnet|testnet]
  bash scripts/check_actual_private_testnet_simulation.sh --check-simulation-receipt <receipt-json>
USAGE
}

valid_cluster() {
  case "${1:-}" in
    devnet|testnet) return 0 ;;
    *) return 1 ;;
  esac
}

valid_direction() {
  case "${1:-}" in
    roc_to_rox|rox_to_roc) return 0 ;;
    *) return 1 ;;
  esac
}

require_valid_cluster() {
  local cluster="${1:-}"
  valid_cluster "$cluster" || fail "cluster must be devnet or testnet, got: ${cluster:-<empty>}"
}

require_valid_direction() {
  local direction="${1:-}"
  valid_direction "$direction" || fail "direction must be roc_to_rox or rox_to_roc, got: ${direction:-<empty>}"
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

require_tiny_positive_amount() {
  local value="$1"
  local field="$2"
  local max="$3"

  require_integer_string "$value" "$field"
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

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_SIMULATION.md"
  local script="$root/scripts/check_actual_private_testnet_simulation.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_SIMULATION.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_simulation.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 6" \
    "Simulation Against Actual Deployed Testnet Addresses" \
    "actual_private_testnet_simulation_receipt" \
    "--simulate-only" \
    "read_only_evidence_status" \
    "proof_review_status" \
    "coordinator_decision_status" \
    "relayer_dry_run_status" \
    "simulation_result" \
    "receipt_promotable_to_send" \
    "No transaction submission." \
    "No wallet loading." \
    "No signature generation." \
    "No public launch authorization." \
    "No mainnet-beta authorization." \
    "No real internal ROC release."
  do
    grep -Fq -- "$needle" "$doc" || fail "simulation doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-simulation-receipt.json" \
    "actual-private-testnet-simulation-receipt.local.json" \
    "actual-private-testnet-simulation-failed.local.json" \
    "*.actual-private-testnet-simulation.local.json" \
    "*.actual-private-testnet-simulation-receipt.local.json" \
    "*.actual-private-testnet-simulation-failed.local.json" \
    "*.actual-simulation-receipt.local.json" \
    "*.actual-simulation-failed.local.json" \
    "*.actual-roc-to-rox-simulation.local.json" \
    "*.actual-rox-to-roc-simulation.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing simulation receipt ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "simulation doc"

  for forbidden in \
    "simulate_only\": false" \
    "transaction_submission\": true" \
    "send_authorized\": true" \
    "wallet_loaded\": true" \
    "signature_generated\": true" \
    "receipt_promotable_to_send\": true" \
    "public_mint_available\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_mutation\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "simulation doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 6 simulation documentation checks passed
summary:
  - actual private testnet simulation runbook exists
  - local simulation receipt names are ignored
  - documentation preserves simulate-only, redacted, gated, tiny-cap, non-mainnet boundaries
  - documentation separates simulation evidence from transaction submission, send authorization, finality, settlement, and public mint availability
SUMMARY
}

print_template() {
  local outcome="$1"
  local direction="${2:-roc_to_rox}"
  local cluster="${3:-testnet}"

  require_valid_direction "$direction"
  require_valid_cluster "$cluster"

  local read_only="verified"
  local proof="accepted"
  local coordinator="accepted"
  local dry_run="accepted"
  local result="passed"
  local readback_verified="true"
  local failure_line=""

  if [ "$outcome" != "simulated" ]; then
    read_only="missing"
    proof="not_run"
    coordinator="not_run"
    dry_run="not_run"
    result="not_run"
    readback_verified="false"
    failure_line='  "failure_reason_redacted": "<redacted-safe-simulation-blocker>",'
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-simulation.v1",
  "phase": "BUILD_PLAN4 Phase 6",
  "receipt_role": "actual_private_testnet_simulation_receipt",
  "cluster": "$cluster",
  "direction": "$direction",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "simulation_outcome": "$outcome",
  "operation_id": "actual-simulation-op-0001",
  "idempotency_key": "actual-simulation-idem-0001",
  "nonce": "actual-simulation-nonce-0001",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "read_only_evidence_status": "$read_only",
  "proof_review_status": "$proof",
  "coordinator_decision_status": "$coordinator",
  "relayer_dry_run_status": "$dry_run",
  "simulation_result": "$result",
$failure_line
  "simulation_log_redacted": "<redacted-simulation-log>",
  "read_only_evidence_required": true,
  "read_only_evidence_verified": $readback_verified,
  "simulate_only": true,
  "transaction_submission": false,
  "send_authorized": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "receipt_promotable_to_send": false,
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

check_simulation_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-simulation-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "simulation receipt not found: $receipt"

  reject_sensitive_text "$receipt" "simulation receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-private-testnet-simulation.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 6"
  require_json_string "$receipt" "receipt_role" "actual_private_testnet_simulation_receipt"
  require_json_string "$receipt" "program_name" "rox_anchor"

  local cluster direction outcome
  cluster="$(json_string_value "$receipt" "cluster")"
  direction="$(json_string_value "$receipt" "direction")"
  outcome="$(json_string_value "$receipt" "simulation_outcome")"

  require_valid_cluster "$cluster"
  ok "receipt cluster is $cluster"

  require_valid_direction "$direction"
  ok "receipt direction is $direction"

  case "$outcome" in
    simulated|failed|blocked) ok "receipt simulation_outcome = $outcome" ;;
    *) fail "simulation_outcome must be simulated, failed, or blocked, got: ${outcome:-<missing>}" ;;
  esac

  for field in \
    program_id \
    operation_id \
    idempotency_key \
    nonce \
    program_account \
    config_account \
    test_only_mint \
    test_only_token_account \
    test_only_mint_label \
    test_only_token_account_label \
    amount_minor \
    max_amount_minor \
    max_operations \
    read_only_evidence_status \
    proof_review_status \
    coordinator_decision_status \
    relayer_dry_run_status \
    simulation_result \
    simulation_log_redacted
  do
    require_json_string_present "$receipt" "$field"
  done

  require_redacted_value "$(json_string_value "$receipt" "program_account")" "program_account"
  require_redacted_value "$(json_string_value "$receipt" "config_account")" "config_account"
  require_redacted_value "$(json_string_value "$receipt" "test_only_mint")" "test_only_mint"
  require_redacted_value "$(json_string_value "$receipt" "test_only_token_account")" "test_only_token_account"
  require_redacted_value "$(json_string_value "$receipt" "simulation_log_redacted")" "simulation_log_redacted"

  reject_public_or_production_label "$(json_string_value "$receipt" "test_only_mint_label")" "test_only_mint_label"
  reject_public_or_production_label "$(json_string_value "$receipt" "test_only_token_account_label")" "test_only_token_account_label"

  local amount max_amount max_operations
  amount="$(json_string_value "$receipt" "amount_minor")"
  max_amount="$(json_string_value "$receipt" "max_amount_minor")"
  max_operations="$(json_string_value "$receipt" "max_operations")"

  require_tiny_positive_amount "$amount" "amount_minor" 1000
  require_tiny_positive_amount "$max_amount" "max_amount_minor" 1000
  require_tiny_positive_amount "$max_operations" "max_operations" 10

  [ "$amount" -le "$max_amount" ] || fail "amount_minor cannot exceed max_amount_minor"
  ok "amount cap relation is valid"

  require_json_bool_true "$receipt" "read_only_evidence_required"
  require_json_bool_true "$receipt" "simulate_only"
  require_json_bool_false_or_absent "$receipt" "transaction_submission"
  require_json_bool_false_or_absent "$receipt" "send_authorized"
  require_json_bool_false_or_absent "$receipt" "wallet_loaded"
  require_json_bool_false_or_absent "$receipt" "signature_generated"
  require_json_bool_false_or_absent "$receipt" "receipt_promotable_to_send"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  if [ "$outcome" = "simulated" ]; then
    require_json_bool_true "$receipt" "read_only_evidence_verified"
    require_json_string "$receipt" "read_only_evidence_status" "verified"
    require_json_string "$receipt" "proof_review_status" "accepted"
    require_json_string "$receipt" "coordinator_decision_status" "accepted"
    require_json_string "$receipt" "relayer_dry_run_status" "accepted"
    require_json_string "$receipt" "simulation_result" "passed"
    ok "simulated receipt satisfies all required gates"
  else
    require_json_bool_false_or_absent "$receipt" "read_only_evidence_verified"
    require_json_string_present "$receipt" "failure_reason_redacted"
    require_redacted_value "$(json_string_value "$receipt" "failure_reason_redacted")" "failure_reason_redacted"
    if [ "$(json_string_value "$receipt" "simulation_result")" = "passed" ]; then
      fail "non-simulated receipt must not claim passed simulation_result"
    fi
    ok "non-simulated receipt remains blocked/failed evidence"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 6 simulation receipt checks passed
summary:
  - receipt is devnet/testnet only
  - receipt is explicitly simulate-only
  - account identifiers and simulation logs are redacted
  - test-only labels and tiny caps are enforced
  - successful simulation requires read-only evidence, proof, coordinator, and relayer dry-run gates
  - receipt does not claim submission, send authorization, wallet load, signing, public mint availability, public launch, mainnet, production settlement, public ROX mint/burn, real ROC mutation, or finality
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
  [ -f "$root/scripts/check_actual_private_testnet_read_only_evidence.sh" ] || fail "Phase 5 read-only evidence checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-simulation.*\.json|.*actual-simulation.*\.json|.*actual-roc-to-rox-simulation.*\.json|.*actual-rox-to-roc-simulation.*\.json)$'; then
      fail "git tracked actual private testnet simulation receipt material found"
    fi
    ok "git tracked-file scan found no actual simulation receipt material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 6 simulation preflight passed
summary:
  - simulation documentation and ignore boundaries are present
  - Anchor build outputs exist
  - Phase 5 read-only evidence checker exists
  - no tracked simulation receipt material was found
  - this preflight did not call RPC, simulate live, submit, sign, load a wallet, mint, burn, settle, or mutate ROC
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
  --template-simulated)
    print_template "simulated" "${2:-roc_to_rox}" "${3:-testnet}"
    ;;
  --template-blocked)
    print_template "blocked" "${2:-roc_to_rox}" "${3:-testnet}"
    ;;
  --check-simulation-receipt)
    check_simulation_receipt "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
