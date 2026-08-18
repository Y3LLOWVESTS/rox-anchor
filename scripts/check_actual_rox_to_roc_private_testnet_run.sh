#!/usr/bin/env bash
set -euo pipefail

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
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --check-docs [repo-root]
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --template-send-sent [devnet|testnet]
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --template-send-blocked [devnet|testnet]
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --template-readback [devnet|testnet]
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --check-send-receipt <receipt-json>
  bash scripts/check_actual_rox_to_roc_private_testnet_run.sh --check-readback-receipt <receipt-json>
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

require_json_string() {
  local file="$1"
  local key="$2"
  local expected="$3"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ "$actual" = "$expected" ] || fail "receipt $key expected '$expected' but found '${actual:-<missing>}'"
  ok "receipt $key = $expected"
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

require_valid_cluster() {
  case "${1:-}" in
    devnet|testnet) ok "receipt cluster is $1" ;;
    *) fail "cluster must be devnet or testnet, got: ${1:-<empty>}" ;;
  esac
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
    none) ok "$field is none" ;;
    *) fail "$field must be a redacted placeholder or none" ;;
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
    *test-only*|*private-testnet*|*private-test*) ok "$field is test-only/private-testnet" ;;
    *) fail "$field must contain test-only/private-testnet marker, got: $value" ;;
  esac
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_ROX_TO_ROC_PRIVATE_TESTNET_RUN.md"
  local script="$root/scripts/check_actual_rox_to_roc_private_testnet_run.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_ROX_TO_ROC_PRIVATE_TESTNET_RUN.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_rox_to_roc_private_testnet_run.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 8" \
    "Actual Capped Testnet ROX-to-ROC Flow" \
    "actual_rox_to_roc_capped_send_receipt" \
    "actual_rox_to_roc_readback_receipt" \
    "I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN" \
    "test_only_rox_burn_only" \
    "internal_roc_release_intent_only" \
    "dry_run_release_intent_id" \
    "test_only_rox_burn_delta_minor" \
    "No real ROC release." \
    "No real internal ROC mutation." \
    "svc-wallet -> ron-ledger" \
    "No public launch authorization." \
    "No mainnet-beta authorization."
  do
    grep -Fq -- "$needle" "$doc" || fail "ROX-to-ROC runbook missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-rox-to-roc-capped-send-receipt.json" \
    "actual-rox-to-roc-capped-send-receipt.local.json" \
    "actual-rox-to-roc-capped-send-failed.local.json" \
    "actual-rox-to-roc-readback.local.json" \
    "actual-rox-to-roc-readback-failed.local.json" \
    "actual-rox-to-roc-release-intent.local.json" \
    "*.actual-rox-to-roc-capped-send.local.json" \
    "*.actual-rox-to-roc-capped-send-receipt.local.json" \
    "*.actual-rox-to-roc-capped-send-failed.local.json" \
    "*.actual-rox-to-roc-readback.local.json" \
    "*.actual-reverse-flow.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing ROX-to-ROC receipt ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "ROX-to-ROC runbook"

  for forbidden in \
    "public_mint_available\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_release\": true" \
    "real_roc_mutation\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "ROX-to-ROC runbook contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 8 ROX-to-ROC documentation checks passed
summary:
  - actual capped ROX-to-ROC private testnet runbook exists
  - local capped-send/readback/release-intent receipt names are ignored
  - documentation preserves test-only-ROX, dry-run-ROC-release-intent-only, redacted, capped, non-mainnet boundaries
  - documentation separates capped send evidence from readback verification, real ROC release, real ROC mutation, production settlement, and finality
SUMMARY
}

print_send_template() {
  local outcome="$1"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  local approval="I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN"
  local sim="passed"
  local signer="true"
  local submitted="true"
  local authorized="true"
  local signature_generated="true"
  local tx_sig="<redacted-testnet-signature>"
  local slot="0"
  local burn_delta="1"
  local release_intent_amount="1"
  local readback_required="true"
  local failure_line=""

  if [ "$outcome" != "sent" ]; then
    approval="missing"
    sim="blocked"
    signer="false"
    submitted="false"
    authorized="false"
    signature_generated="false"
    tx_sig="none"
    slot="none"
    burn_delta="0"
    release_intent_amount="0"
    readback_required="false"
    failure_line='  "failure_reason_redacted": "<redacted-safe-capped-reverse-blocker>",'
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-rox-to-roc-capped-send.v1",
  "phase": "BUILD_PLAN4 Phase 8",
  "receipt_role": "actual_rox_to_roc_capped_send_receipt",
  "cluster": "$cluster",
  "direction": "rox_to_roc",
  "program_name": "rox_anchor",
  "program_id": "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
  "send_outcome": "$outcome",
  "operation_id": "actual-rox-to-roc-op-0001",
  "idempotency_key": "actual-rox-to-roc-idem-0001",
  "nonce": "actual-rox-to-roc-nonce-0001",
  "test_only_rox_burn_evidence_id": "test-only-rox-burn-evidence-0001",
  "test_only_rox_burn_only": true,
  "internal_roc_release_intent_only": true,
  "dry_run_release_intent_id": "<redacted-dry-run-roc-release-intent-id>",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "test_only_mint_label": "test-only-rox-private-testnet",
  "test_only_token_account_label": "test-only-rox-token-account-private-testnet",
  "amount_minor": "1",
  "max_amount_minor": "1",
  "max_operations": "1",
  "retry_cap": "1",
  "read_only_evidence_status": "verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_dry_run_status": "accepted",
  "simulation_result": "$sim",
$failure_line
  "operator_approval": "$approval",
  "external_signer_used": $signer,
  "signer_path_redacted": "<redacted-external-signer-path>",
  "receipt_out_redacted": "<redacted-external-receipt-path>",
  "transaction_submission": $submitted,
  "send_authorized": $authorized,
  "signature_generated": $signature_generated,
  "transaction_signature": "$tx_sig",
  "send_slot": "$slot",
  "test_only_rox_burn_delta_minor": "$burn_delta",
  "expected_internal_roc_release_intent_minor": "$release_intent_amount",
  "readback_required": $readback_required,
  "readback_verified": false,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
TEMPLATE
}

print_readback_template() {
  local cluster="${1:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-rox-to-roc-readback.v1",
  "phase": "BUILD_PLAN4 Phase 8",
  "receipt_role": "actual_rox_to_roc_readback_receipt",
  "cluster": "$cluster",
  "direction": "rox_to_roc",
  "program_name": "rox_anchor",
  "program_id": "FiUY5M3a8xRHCgCfNzqNe5qATKUa3fk2chHFsJGdEitk",
  "readback_outcome": "verified",
  "operation_id": "actual-rox-to-roc-op-0001",
  "idempotency_key": "actual-rox-to-roc-idem-0001",
  "nonce": "actual-rox-to-roc-nonce-0001",
  "transaction_signature": "<redacted-testnet-signature>",
  "send_receipt_id": "<redacted-send-receipt-id>",
  "program_account": "<redacted-program-account>",
  "config_account": "<redacted-program-config-account>",
  "test_only_mint": "<redacted-test-only-mint>",
  "test_only_token_account": "<redacted-test-only-token-account>",
  "expected_test_only_rox_burn_delta_minor": "1",
  "observed_test_only_rox_burn_delta_minor": "1",
  "dry_run_release_intent_id": "<redacted-dry-run-roc-release-intent-id>",
  "expected_internal_roc_release_intent_minor": "1",
  "observed_internal_roc_release_intent_minor": "1",
  "rpc_evidence_redacted": "<redacted-read-only-rpc-evidence>",
  "read_only_rpc": true,
  "transaction_submission": false,
  "internal_roc_release_intent_only": true,
  "public_mint_available": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "finality_claim": false
}
TEMPLATE
}

check_send_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-send-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "send receipt not found: $receipt"

  reject_sensitive_text "$receipt" "ROX-to-ROC send receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-rox-to-roc-capped-send.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 8"
  require_json_string "$receipt" "receipt_role" "actual_rox_to_roc_capped_send_receipt"
  require_json_string "$receipt" "direction" "rox_to_roc"
  require_json_string "$receipt" "program_name" "rox_anchor"

  require_valid_cluster "$(json_string_value "$receipt" "cluster")"

  local outcome
  outcome="$(json_string_value "$receipt" "send_outcome")"
  case "$outcome" in
    sent|blocked|failed) ok "receipt send_outcome = $outcome" ;;
    *) fail "send_outcome must be sent, blocked, or failed, got: ${outcome:-<missing>}" ;;
  esac

  for field in \
    program_id operation_id idempotency_key nonce test_only_rox_burn_evidence_id \
    dry_run_release_intent_id program_account config_account test_only_mint test_only_token_account \
    test_only_mint_label test_only_token_account_label amount_minor max_amount_minor \
    max_operations retry_cap read_only_evidence_status proof_review_status \
    coordinator_decision_status relayer_dry_run_status simulation_result operator_approval \
    signer_path_redacted receipt_out_redacted transaction_signature send_slot \
    test_only_rox_burn_delta_minor expected_internal_roc_release_intent_minor
  do
    require_json_string_present "$receipt" "$field"
  done

  require_redacted_value "$(json_string_value "$receipt" "dry_run_release_intent_id")" "dry_run_release_intent_id"
  require_redacted_value "$(json_string_value "$receipt" "program_account")" "program_account"
  require_redacted_value "$(json_string_value "$receipt" "config_account")" "config_account"
  require_redacted_value "$(json_string_value "$receipt" "test_only_mint")" "test_only_mint"
  require_redacted_value "$(json_string_value "$receipt" "test_only_token_account")" "test_only_token_account"
  require_redacted_value "$(json_string_value "$receipt" "signer_path_redacted")" "signer_path_redacted"
  require_redacted_value "$(json_string_value "$receipt" "receipt_out_redacted")" "receipt_out_redacted"
  require_redacted_value "$(json_string_value "$receipt" "transaction_signature")" "transaction_signature"

  reject_public_or_production_label "$(json_string_value "$receipt" "test_only_mint_label")" "test_only_mint_label"
  reject_public_or_production_label "$(json_string_value "$receipt" "test_only_token_account_label")" "test_only_token_account_label"

  local amount max_amount max_operations retry_cap burn_delta release_intent_amount
  amount="$(json_string_value "$receipt" "amount_minor")"
  max_amount="$(json_string_value "$receipt" "max_amount_minor")"
  max_operations="$(json_string_value "$receipt" "max_operations")"
  retry_cap="$(json_string_value "$receipt" "retry_cap")"
  burn_delta="$(json_string_value "$receipt" "test_only_rox_burn_delta_minor")"
  release_intent_amount="$(json_string_value "$receipt" "expected_internal_roc_release_intent_minor")"

  require_tiny_positive_amount "$amount" "amount_minor" 1000
  require_tiny_positive_amount "$max_amount" "max_amount_minor" 1000
  require_tiny_positive_amount "$max_operations" "max_operations" 10
  require_tiny_positive_amount "$retry_cap" "retry_cap" 10
  require_integer_string "$burn_delta" "test_only_rox_burn_delta_minor"
  require_integer_string "$release_intent_amount" "expected_internal_roc_release_intent_minor"
  [ "$amount" -le "$max_amount" ] || fail "amount_minor cannot exceed max_amount_minor"
  ok "amount cap relation is valid"

  require_json_bool_true "$receipt" "test_only_rox_burn_only"
  require_json_bool_true "$receipt" "internal_roc_release_intent_only"
  require_json_bool_false_or_absent "$receipt" "readback_verified"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_release"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  if [ "$outcome" = "sent" ]; then
    require_json_string "$receipt" "read_only_evidence_status" "verified"
    require_json_string "$receipt" "proof_review_status" "accepted"
    require_json_string "$receipt" "coordinator_decision_status" "accepted"
    require_json_string "$receipt" "relayer_dry_run_status" "accepted"
    require_json_string "$receipt" "simulation_result" "passed"
    require_json_string "$receipt" "operator_approval" "I_APPROVE_PRIVATE_TESTNET_CAPPED_ROX_TO_ROC_BURN"
    require_json_bool_true "$receipt" "external_signer_used"
    require_json_bool_true "$receipt" "transaction_submission"
    require_json_bool_true "$receipt" "send_authorized"
    require_json_bool_true "$receipt" "signature_generated"
    require_json_bool_true "$receipt" "readback_required"
    [ "$(json_string_value "$receipt" "transaction_signature")" != "none" ] || fail "sent receipt requires transaction_signature"
    [ "$(json_string_value "$receipt" "send_slot")" != "none" ] || fail "sent receipt requires send_slot"
    [ "$burn_delta" = "$amount" ] || fail "sent receipt test_only_rox_burn_delta_minor must equal amount_minor"
    [ "$release_intent_amount" = "$amount" ] || fail "sent receipt dry-run release intent amount must equal amount_minor"
    ok "sent receipt satisfies capped reverse-flow gates"
  else
    require_json_string_present "$receipt" "failure_reason_redacted"
    require_redacted_value "$(json_string_value "$receipt" "failure_reason_redacted")" "failure_reason_redacted"
    require_json_bool_false_or_absent "$receipt" "external_signer_used"
    require_json_bool_false_or_absent "$receipt" "transaction_submission"
    require_json_bool_false_or_absent "$receipt" "send_authorized"
    require_json_bool_false_or_absent "$receipt" "signature_generated"
    require_json_bool_false_or_absent "$receipt" "readback_required"
    require_json_string "$receipt" "transaction_signature" "none"
    require_json_string "$receipt" "send_slot" "none"
    [ "$burn_delta" = "0" ] || fail "blocked/failed receipt must not claim test-only ROX burn delta"
    [ "$release_intent_amount" = "0" ] || fail "blocked/failed receipt must not claim internal ROC release intent amount"
    ok "blocked/failed receipt remains non-submitting evidence"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 8 ROX-to-ROC send receipt checks passed
summary:
  - receipt is devnet/testnet only
  - receipt is ROX-to-ROC only
  - test-only ROX burn evidence is explicitly test-only
  - internal ROC release remains dry-run intent only
  - test-only labels and tiny caps are enforced
  - sent receipt requires read-only evidence, proof, coordinator, relayer dry-run, simulation, explicit approval, signer use, submission, signature, and readback-required markers
  - receipt does not claim public launch, mainnet, production settlement, public ROX mint/burn, real ROC release, real ROC mutation, or finality
SUMMARY
}

check_readback_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-readback-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "readback receipt not found: $receipt"

  reject_sensitive_text "$receipt" "ROX-to-ROC readback receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-rox-to-roc-readback.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 8"
  require_json_string "$receipt" "receipt_role" "actual_rox_to_roc_readback_receipt"
  require_json_string "$receipt" "direction" "rox_to_roc"
  require_json_string "$receipt" "program_name" "rox_anchor"
  require_json_string "$receipt" "readback_outcome" "verified"

  require_valid_cluster "$(json_string_value "$receipt" "cluster")"

  for field in \
    program_id operation_id idempotency_key nonce transaction_signature send_receipt_id \
    program_account config_account test_only_mint test_only_token_account \
    expected_test_only_rox_burn_delta_minor observed_test_only_rox_burn_delta_minor \
    dry_run_release_intent_id expected_internal_roc_release_intent_minor \
    observed_internal_roc_release_intent_minor rpc_evidence_redacted
  do
    require_json_string_present "$receipt" "$field"
  done

  require_redacted_value "$(json_string_value "$receipt" "transaction_signature")" "transaction_signature"
  require_redacted_value "$(json_string_value "$receipt" "send_receipt_id")" "send_receipt_id"
  require_redacted_value "$(json_string_value "$receipt" "program_account")" "program_account"
  require_redacted_value "$(json_string_value "$receipt" "config_account")" "config_account"
  require_redacted_value "$(json_string_value "$receipt" "test_only_mint")" "test_only_mint"
  require_redacted_value "$(json_string_value "$receipt" "test_only_token_account")" "test_only_token_account"
  require_redacted_value "$(json_string_value "$receipt" "dry_run_release_intent_id")" "dry_run_release_intent_id"
  require_redacted_value "$(json_string_value "$receipt" "rpc_evidence_redacted")" "rpc_evidence_redacted"

  local expected_burn observed_burn expected_release observed_release
  expected_burn="$(json_string_value "$receipt" "expected_test_only_rox_burn_delta_minor")"
  observed_burn="$(json_string_value "$receipt" "observed_test_only_rox_burn_delta_minor")"
  expected_release="$(json_string_value "$receipt" "expected_internal_roc_release_intent_minor")"
  observed_release="$(json_string_value "$receipt" "observed_internal_roc_release_intent_minor")"

  require_tiny_positive_amount "$expected_burn" "expected_test_only_rox_burn_delta_minor" 1000
  require_tiny_positive_amount "$observed_burn" "observed_test_only_rox_burn_delta_minor" 1000
  require_tiny_positive_amount "$expected_release" "expected_internal_roc_release_intent_minor" 1000
  require_tiny_positive_amount "$observed_release" "observed_internal_roc_release_intent_minor" 1000
  [ "$expected_burn" = "$observed_burn" ] || fail "observed test-only ROX burn delta must match expected delta"
  [ "$expected_release" = "$observed_release" ] || fail "observed dry-run release intent must match expected amount"
  [ "$observed_burn" = "$observed_release" ] || fail "dry-run release intent amount must match observed test-only ROX burn delta"
  ok "readback burn and dry-run release-intent deltas match"

  require_json_bool_true "$receipt" "read_only_rpc"
  require_json_bool_true "$receipt" "internal_roc_release_intent_only"
  require_json_bool_false_or_absent "$receipt" "transaction_submission"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_release"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 8 ROX-to-ROC readback receipt checks passed
summary:
  - readback receipt is devnet/testnet only
  - readback evidence is read-only RPC
  - redacted accounts, signature, receipt ID, release intent ID, and RPC evidence are enforced
  - observed test-only ROX burn delta matches expected delta
  - dry-run internal ROC release intent amount matches the observed test-only ROX burn delta
  - receipt does not claim submission, public launch, mainnet, production settlement, public ROX mint/burn, real ROC release, real ROC mutation, or finality
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/Anchor.toml" ] || fail "Anchor.toml missing"
  [ -f "$root/target/deploy/rox_anchor.so" ] || fail "target/deploy/rox_anchor.so missing; run anchor build first"
  [ -f "$root/target/idl/rox_anchor.json" ] || fail "target/idl/rox_anchor.json missing; run anchor build first"
  [ -f "$root/scripts/check_actual_private_testnet_read_only_evidence.sh" ] || fail "Phase 5 read-only evidence checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_simulation.sh" ] || fail "Phase 6 simulation checker missing"
  [ -f "$root/scripts/check_actual_roc_to_rox_private_testnet_run.sh" ] || fail "Phase 7 ROC-to-ROX checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-rox-to-roc.*\.json|.*actual-reverse-.*\.json)$'; then
      fail "git tracked actual ROX-to-ROC receipt material found"
    fi
    ok "git tracked-file scan found no actual ROX-to-ROC receipt material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 8 ROX-to-ROC preflight passed
summary:
  - ROX-to-ROC documentation and ignore boundaries are present
  - Anchor build outputs exist
  - Phase 5 read-only evidence checker exists
  - Phase 6 simulation checker exists
  - Phase 7 ROC-to-ROX checker exists
  - no tracked ROX-to-ROC receipt material was found
  - this preflight did not call RPC, submit, sign, load a signer, mint, burn, settle, release ROC, or mutate ROC
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
  --template-send-sent)
    print_send_template "sent" "${2:-testnet}"
    ;;
  --template-send-blocked)
    print_send_template "blocked" "${2:-testnet}"
    ;;
  --template-readback)
    print_readback_template "${2:-testnet}"
    ;;
  --check-send-receipt)
    check_send_receipt "${2:-}"
    ;;
  --check-readback-receipt)
    check_readback_receipt "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
