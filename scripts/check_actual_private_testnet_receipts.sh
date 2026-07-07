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
  bash scripts/check_actual_private_testnet_receipts.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_receipts.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_receipts.sh --template-reconciled [devnet|testnet]
  bash scripts/check_actual_private_testnet_receipts.sh --template-quarantined [devnet|testnet]
  bash scripts/check_actual_private_testnet_receipts.sh --check-ledger <ledger-json>
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
  [ "$actual" = "$expected" ] || fail "ledger $key expected '$expected' but found '${actual:-<missing>}'"
  ok "ledger $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "ledger missing non-empty string field: $key"
  ok "ledger has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "ledger must set $key true"
  ok "ledger sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "ledger contains forbidden true boolean: $key"
  fi
  ok "ledger does not set $key true"
}

require_valid_cluster() {
  case "${1:-}" in
    devnet|testnet) ok "ledger cluster is $1" ;;
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

require_redacted_value() {
  local value="$1"
  local field="$2"

  case "$value" in
    *redacted*|"<redacted-"*) ok "$field is redacted" ;;
    none|not_performed) ok "$field is $value" ;;
    *) fail "$field must be a redacted placeholder, none, or not_performed" ;;
  esac
}

require_receipt_status() {
  local value="$1"
  local field="$2"
  case "$value" in
    verified|not_performed|blocked|failed) ok "$field is valid: $value" ;;
    *) fail "$field must be verified, not_performed, blocked, or failed, got: ${value:-<missing>}" ;;
  esac
}

require_unique_receipt_ids() {
  local receipt_ids="$1"

  [ -n "$receipt_ids" ] || fail "receipt_ids must not be empty"

  local duplicate
  duplicate="$(
    printf '%s' "$receipt_ids" |
      awk -v RS=',' '
        {
          gsub(/^[[:space:]]+/, "", $0)
          gsub(/[[:space:]]+$/, "", $0)
          if ($0 != "") {
            total += 1
            seen[$0] += 1
            if (seen[$0] == 2 && duplicate == "") {
              duplicate = $0
            }
          }
        }
        END {
          if (total == 0) {
            print "__EMPTY__"
          } else if (duplicate != "") {
            print duplicate
          }
        }
      '
  )"

  [ "$duplicate" != "__EMPTY__" ] || fail "receipt_ids must include at least one receipt ID"
  [ -z "$duplicate" ] || fail "receipt_ids contain duplicate values: $duplicate"
  ok "receipt_ids are unique"
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_RECEIPT_LEDGER.md"
  local script="$root/scripts/check_actual_private_testnet_receipts.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_RECEIPT_LEDGER.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_receipts.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 9" \
    "Receipt Ledger Reconciliation for Actual Runs" \
    "actual_private_testnet_receipt_ledger" \
    "receipt_ids" \
    "operation_binding_status" \
    "idempotency_binding_status" \
    "nonce_binding_status" \
    "readback_binding_status" \
    "production_bridge_settlement" \
    "real_roc_release" \
    "real_roc_mutation" \
    "No real ROC release." \
    "No real internal ROC mutation." \
    "No production bridge settlement." \
    "No fake finality."
  do
    grep -Fq -- "$needle" "$doc" || fail "actual receipt ledger doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-receipt-ledger.json" \
    "actual-private-testnet-receipt-ledger.local.json" \
    "actual-private-testnet-receipt-ledger-incomplete.local.json" \
    "actual-private-testnet-receipt-ledger-quarantined.local.json" \
    "actual-private-testnet-reconciliation.local.json" \
    "*.actual-private-testnet-receipt-ledger.local.json" \
    "*.actual-receipt-ledger.local.json" \
    "*.actual-ledger-reconciliation.local.json" \
    "*.actual-reconciliation-quarantine.local.json" \
    "*.actual-reconciliation-report.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing receipt-ledger ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "actual receipt ledger doc"

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
      fail "actual receipt ledger doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 9 actual receipt ledger documentation checks passed
summary:
  - actual private testnet receipt ledger runbook exists
  - local receipt ledger artifact names are ignored
  - documentation preserves unique receipt, binding, redaction, readback, non-mainnet, and non-production boundaries
  - documentation separates private testnet reconciliation evidence from production settlement, real ROC release, real ROC mutation, and finality
SUMMARY
}

print_template() {
  local outcome="$1"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  local reconciliation="$outcome"
  local receipt_ids="deploy-0001,init-0001,read-only-0001,simulation-0001,roc-to-rox-send-0001,roc-to-rox-readback-0001,rox-to-roc-send-0001,rox-to-roc-readback-0001"
  local simulation_status="verified"
  local fwd_send="verified"
  local fwd_readback="verified"
  local rev_send="verified"
  local rev_readback="verified"
  local release_status="verified"
  local readback_binding="verified"
  local readback_verified="true"
  local reason_line=""

  if [ "$outcome" != "reconciled" ]; then
    receipt_ids="deploy-0001,init-0001,read-only-0001,simulation-0001"
    simulation_status="blocked"
    fwd_send="not_performed"
    fwd_readback="not_performed"
    rev_send="not_performed"
    rev_readback="not_performed"
    release_status="not_performed"
    readback_binding="not_performed"
    readback_verified="false"
    reason_line='  "quarantine_reason_redacted": "<redacted-reconciliation-blocker>",'
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-receipt-ledger.v1",
  "phase": "BUILD_PLAN4 Phase 9",
  "receipt_role": "actual_private_testnet_receipt_ledger",
  "cluster": "$cluster",
  "ledger_id": "<redacted-ledger-id>",
  "ledger_outcome": "$outcome",
  "reconciliation_status": "$reconciliation",
  "operation_id": "actual-private-testnet-op-0001",
  "idempotency_key": "actual-private-testnet-idem-0001",
  "nonce": "actual-private-testnet-nonce-0001",
  "receipt_ids": "$receipt_ids",
  "receipt_operation_ids": "actual-private-testnet-op-0001",
  "receipt_idempotency_keys": "actual-private-testnet-idem-0001",
  "receipt_nonces": "actual-private-testnet-nonce-0001",
  "deploy_receipt_status": "verified",
  "initialization_receipt_status": "verified",
  "read_only_evidence_status": "verified",
  "simulation_receipt_status": "$simulation_status",
  "roc_to_rox_send_status": "$fwd_send",
  "roc_to_rox_readback_status": "$fwd_readback",
  "rox_to_roc_send_status": "$rev_send",
  "rox_to_roc_readback_status": "$rev_readback",
  "dry_run_release_intent_status": "$release_status",
  "receipt_chain_status": "linked",
  "operation_binding_status": "matched",
  "idempotency_binding_status": "matched",
  "nonce_binding_status": "matched",
  "signature_binding_status": "redacted",
  "readback_binding_status": "$readback_binding",
$reason_line
  "transaction_signatures_redacted": "<redacted-signature-list>",
  "readback_evidence_redacted": "<redacted-readback-evidence>",
  "operator_report_redacted": "<redacted-operator-report>",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "readback_verified": $readback_verified,
  "duplicate_receipts_detected": false,
  "operation_id_mismatch_detected": false,
  "idempotency_key_mismatch_detected": false,
  "nonce_mismatch_detected": false,
  "live_submission_without_signature_detected": false,
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

check_ledger() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-ledger requires a ledger path"
  [ -f "$receipt" ] || fail "ledger not found: $receipt"

  reject_sensitive_text "$receipt" "actual receipt ledger"

  require_json_string "$receipt" "schema" "rox-anchor.actual-private-testnet-receipt-ledger.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 9"
  require_json_string "$receipt" "receipt_role" "actual_private_testnet_receipt_ledger"

  require_valid_cluster "$(json_string_value "$receipt" "cluster")"

  local outcome
  local reconciliation
  outcome="$(json_string_value "$receipt" "ledger_outcome")"
  reconciliation="$(json_string_value "$receipt" "reconciliation_status")"

  case "$outcome" in
    reconciled|incomplete|quarantined) ok "ledger_outcome is valid: $outcome" ;;
    *) fail "ledger_outcome must be reconciled, incomplete, or quarantined, got: ${outcome:-<missing>}" ;;
  esac

  [ "$outcome" = "$reconciliation" ] || fail "ledger_outcome and reconciliation_status must match"
  ok "ledger_outcome matches reconciliation_status"

  for field in \
    ledger_id operation_id idempotency_key nonce receipt_ids receipt_operation_ids \
    receipt_idempotency_keys receipt_nonces deploy_receipt_status initialization_receipt_status \
    read_only_evidence_status simulation_receipt_status roc_to_rox_send_status \
    roc_to_rox_readback_status rox_to_roc_send_status rox_to_roc_readback_status \
    dry_run_release_intent_status receipt_chain_status operation_binding_status \
    idempotency_binding_status nonce_binding_status signature_binding_status readback_binding_status \
    transaction_signatures_redacted readback_evidence_redacted operator_report_redacted
  do
    require_json_string_present "$receipt" "$field"
  done

  require_redacted_value "$(json_string_value "$receipt" "ledger_id")" "ledger_id"
  require_redacted_value "$(json_string_value "$receipt" "transaction_signatures_redacted")" "transaction_signatures_redacted"
  require_redacted_value "$(json_string_value "$receipt" "readback_evidence_redacted")" "readback_evidence_redacted"
  require_redacted_value "$(json_string_value "$receipt" "operator_report_redacted")" "operator_report_redacted"

  require_unique_receipt_ids "$(json_string_value "$receipt" "receipt_ids")"

  for field in \
    deploy_receipt_status initialization_receipt_status read_only_evidence_status simulation_receipt_status \
    roc_to_rox_send_status roc_to_rox_readback_status rox_to_roc_send_status rox_to_roc_readback_status \
    dry_run_release_intent_status
  do
    require_receipt_status "$(json_string_value "$receipt" "$field")" "$field"
  done

  require_json_bool_true "$receipt" "private_testnet_only"
  require_json_bool_true "$receipt" "test_only_assets_only"

  require_json_bool_false_or_absent "$receipt" "duplicate_receipts_detected"
  require_json_bool_false_or_absent "$receipt" "operation_id_mismatch_detected"
  require_json_bool_false_or_absent "$receipt" "idempotency_key_mismatch_detected"
  require_json_bool_false_or_absent "$receipt" "nonce_mismatch_detected"
  require_json_bool_false_or_absent "$receipt" "live_submission_without_signature_detected"
  require_json_bool_false_or_absent "$receipt" "public_mint_available"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_release"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"
  require_json_bool_false_or_absent "$receipt" "finality_claim"

  if [ "$outcome" = "reconciled" ]; then
    require_json_string "$receipt" "deploy_receipt_status" "verified"
    require_json_string "$receipt" "initialization_receipt_status" "verified"
    require_json_string "$receipt" "read_only_evidence_status" "verified"
    require_json_string "$receipt" "simulation_receipt_status" "verified"
    require_json_string "$receipt" "receipt_chain_status" "linked"
    require_json_string "$receipt" "operation_binding_status" "matched"
    require_json_string "$receipt" "idempotency_binding_status" "matched"
    require_json_string "$receipt" "nonce_binding_status" "matched"
    require_json_string "$receipt" "signature_binding_status" "redacted"
    require_json_string "$receipt" "readback_binding_status" "verified"
    require_json_bool_true "$receipt" "readback_verified"
    ok "reconciled ledger satisfies receipt linkage gates"
  else
    require_json_string_present "$receipt" "quarantine_reason_redacted"
    require_redacted_value "$(json_string_value "$receipt" "quarantine_reason_redacted")" "quarantine_reason_redacted"
    require_json_bool_false_or_absent "$receipt" "readback_verified"
    ok "incomplete/quarantined ledger remains non-success evidence"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 9 actual receipt ledger checks passed
summary:
  - ledger is devnet/testnet only
  - receipt IDs are unique
  - operation, idempotency, nonce, signature, and readback bindings are checked
  - sensitive values are redacted
  - production settlement, public launch, mainnet, public ROX mint/burn, real ROC release, real ROC mutation, and finality claims are rejected
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
  [ -f "$root/scripts/check_actual_private_testnet_deploy_receipt.sh" ] || fail "Phase 3 deploy receipt checker missing"
  [ -f "$root/scripts/check_actual_test_only_mint_initialization.sh" ] || fail "Phase 4 initialization checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_read_only_evidence.sh" ] || fail "Phase 5 read-only evidence checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_simulation.sh" ] || fail "Phase 6 simulation checker missing"
  [ -f "$root/scripts/check_actual_roc_to_rox_private_testnet_run.sh" ] || fail "Phase 7 ROC-to-ROX checker missing"
  [ -f "$root/scripts/check_actual_rox_to_roc_private_testnet_run.sh" ] || fail "Phase 8 ROX-to-ROC checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-receipt-ledger.*\.json|.*actual-ledger-reconciliation.*\.json|.*actual-reconciliation.*\.json)$'; then
      fail "git tracked actual receipt ledger material found"
    fi
    ok "git tracked-file scan found no actual receipt ledger material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 9 actual receipt ledger preflight passed
summary:
  - receipt ledger documentation and ignore boundaries are present
  - Anchor build outputs exist
  - Phase 3 through Phase 8 checkers exist
  - no tracked actual receipt ledger material was found
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
  --template-reconciled)
    print_template "reconciled" "${2:-testnet}"
    ;;
  --template-quarantined)
    print_template "quarantined" "${2:-testnet}"
    ;;
  --check-ledger)
    check_ledger "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
