#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 11 halt/recovery/authority drill report checker.
# RO:WHY — Validates redacted operator safety reports without RPC, signing, submission, authority-key loading, upgrades, settlement, or ROC mutation.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md, docs/pilot/ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md, .gitignore.
# RO:INVARIANTS — devnet/testnet only; operator reports are redacted; wrong authority fails safe; valid recovery does not imply finality/settlement.
# RO:SECURITY — local file checks only; no wallet load, authority key load, live RPC, signing, submission, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-core --test actual_private_testnet_authority_drills and cargo test -p rox-anchor-cli --test actual_private_testnet_drill_reports.

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
  bash scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh --template-drill [devnet|testnet] [drill_name]
  bash scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh --template-matrix [devnet|testnet]
  bash scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh --check-drill-report <report-json>
USAGE
}

drill_names() {
  cat <<'DRILLS'
halt_before_simulation
halt_after_simulation_before_send
halt_after_capped_send_before_readback
valid_recovery_after_halt
clean_flow_after_valid_recovery
wrong_authority_halt_attempt
wrong_authority_recovery_attempt
key_rotation_intent
upgrade_authority_checklist
separated_authority_status
DRILLS
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

require_valid_drill_name() {
  local name="${1:-}"
  drill_names | grep -Fxq "$name" || fail "unknown drill_name: ${name:-<empty>}"
  ok "drill_name is valid: $name"
}

require_json_string() {
  local file="$1"
  local key="$2"
  local expected="$3"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ "$actual" = "$expected" ] || fail "drill report $key expected '$expected' but found '${actual:-<missing>}'"
  ok "drill report $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "drill report missing non-empty string field: $key"
  ok "drill report has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "drill report must set $key true"
  ok "drill report sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "drill report contains forbidden true boolean: $key"
  fi
  ok "drill report does not set $key true"
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
    "authority.json" \
    "upgrade-authority.json" \
    "halt-authority.json" \
    "recovery-authority.json"
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

require_drill_outcome() {
  local value="$1"
  case "$value" in
    blocked|recovered|reviewed|not_performed) ok "drill_outcome is valid: $value" ;;
    *) fail "drill_outcome must be blocked, recovered, reviewed, or not_performed, got: ${value:-<missing>}" ;;
  esac
}

require_halt_status() {
  local value="$1"
  case "$value" in
    active|cleared|not_active|attempt_rejected|not_required) ok "halt_status is valid: $value" ;;
    *) fail "halt_status is invalid: ${value:-<missing>}" ;;
  esac
}

require_recovery_status() {
  local value="$1"
  case "$value" in
    required|validated|not_required|attempt_rejected) ok "recovery_status is valid: $value" ;;
    *) fail "recovery_status is invalid: ${value:-<missing>}" ;;
  esac
}

require_authority_status() {
  local value="$1"
  case "$value" in
    validated|rejected|intent_recorded|reviewed|separated) ok "authority_status is valid: $value" ;;
    *) fail "authority_status is invalid: ${value:-<missing>}" ;;
  esac
}

require_clean_flow_resume_status() {
  local value="$1"
  case "$value" in
    blocked|allowed_after_valid_recovery|not_tested) ok "clean_flow_resume_status is valid: $value" ;;
    *) fail "clean_flow_resume_status is invalid: ${value:-<missing>}" ;;
  esac
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local halt_doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md"
  local authority_doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md"
  local script="$root/scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh"
  local gitignore="$root/.gitignore"

  [ -f "$halt_doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_HALT_RECOVERY_DRILLS.md"
  [ -f "$authority_doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_AUTHORITY_DRILLS.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 11" \
    "Actual Halt, Recovery, and Authority Drills" \
    "actual_private_testnet_authority_drill_report" \
    "rox-anchor.actual-private-testnet-authority-drill.v1" \
    "halt_before_simulation" \
    "halt_after_simulation_before_send" \
    "halt_after_capped_send_before_readback" \
    "valid_recovery_after_halt" \
    "clean_flow_after_valid_recovery" \
    "No real ROC release." \
    "No real internal ROC mutation." \
    "No production bridge settlement." \
    "No fake finality."
  do
    grep -Fq -- "$needle" "$halt_doc" || fail "halt/recovery doc missing marker: $needle"
    ok "halt/recovery doc marker present: $needle"
  done

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 11" \
    "Actual Authority Drills" \
    "actual_private_testnet_authority_drill_report" \
    "rox-anchor.actual-private-testnet-authority-drill.v1" \
    "wrong_authority_halt_attempt" \
    "wrong_authority_recovery_attempt" \
    "key_rotation_intent" \
    "upgrade_authority_checklist" \
    "separated_authority_status" \
    "No key material in repo." \
    "No wallet loading in tests." \
    "No authority key loading in tests." \
    "No key rotation execution in this patch." \
    "No upgrade authority change in this patch." \
    "No real internal ROC mutation."
  do
    grep -Fq -- "$needle" "$authority_doc" || fail "authority doc missing marker: $needle"
    ok "authority doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-halt-recovery-drill.json" \
    "actual-private-testnet-halt-recovery-drill.local.json" \
    "actual-private-testnet-authority-drill.json" \
    "actual-private-testnet-authority-drill.local.json" \
    "actual-private-testnet-authority-report.local.json" \
    "*.actual-halt-recovery-drill.local.json" \
    "*.actual-authority-drill.local.json" \
    "*.actual-authority-report.local.json" \
    "*.actual-key-rotation-intent.local.json" \
    "*.actual-upgrade-authority-checklist.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing Phase 11 ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$halt_doc" "halt/recovery drill doc"
  reject_sensitive_text "$authority_doc" "authority drill doc"

  for doc in "$halt_doc" "$authority_doc"; do
    for forbidden in \
      "transaction_submission\": true" \
      "send_authorized\": true" \
      "wallet_loaded\": true" \
      "signature_generated\": true" \
      "authority_key_loaded\": true" \
      "key_rotation_executed\": true" \
      "upgrade_authority_changed\": true" \
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
        fail "$(basename "$doc") contains forbidden claim marker: $forbidden"
      fi
      ok "$(basename "$doc") excludes forbidden claim marker: $forbidden"
    done
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 11 halt/recovery/authority documentation checks passed
summary:
  - halt/recovery and authority drill runbooks exist
  - local Phase 11 drill report artifact names are ignored
  - documentation preserves redaction, separated authority, non-mainnet, non-production, and non-real-ROC boundaries
  - documentation separates drill reports from transaction submission, key loading, key rotation execution, upgrade authority changes, finality, settlement, and real ROC mutation
SUMMARY
}

status_for_drill() {
  local drill="$1"
  case "$drill" in
    halt_before_simulation)
      printf 'blocked|active|required|validated|blocked'
      ;;
    halt_after_simulation_before_send)
      printf 'blocked|active|required|validated|blocked'
      ;;
    halt_after_capped_send_before_readback)
      printf 'blocked|active|required|validated|blocked'
      ;;
    valid_recovery_after_halt)
      printf 'recovered|cleared|validated|validated|allowed_after_valid_recovery'
      ;;
    clean_flow_after_valid_recovery)
      printf 'reviewed|cleared|validated|validated|allowed_after_valid_recovery'
      ;;
    wrong_authority_halt_attempt)
      printf 'blocked|attempt_rejected|not_required|rejected|not_tested'
      ;;
    wrong_authority_recovery_attempt)
      printf 'blocked|active|attempt_rejected|rejected|blocked'
      ;;
    key_rotation_intent)
      printf 'reviewed|not_required|not_required|intent_recorded|not_tested'
      ;;
    upgrade_authority_checklist)
      printf 'reviewed|not_required|not_required|reviewed|not_tested'
      ;;
    separated_authority_status)
      printf 'reviewed|not_required|not_required|separated|not_tested'
      ;;
    *)
      fail "unknown drill_name: $drill"
      ;;
  esac
}

print_drill_template() {
  local cluster="${1:-testnet}"
  local drill="${2:-halt_before_simulation}"

  require_valid_cluster "$cluster" >/dev/null
  require_valid_drill_name "$drill" >/dev/null

  local packed outcome halt_status recovery_status authority_status clean_status
  packed="$(status_for_drill "$drill")"
  IFS='|' read -r outcome halt_status recovery_status authority_status clean_status <<EOF_STATUS
$packed
EOF_STATUS

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-authority-drill.v1",
  "phase": "BUILD_PLAN4 Phase 11",
  "receipt_role": "actual_private_testnet_authority_drill_report",
  "cluster": "$cluster",
  "drill_name": "$drill",
  "drill_outcome": "$outcome",
  "operation_id": "actual-authority-drill-op-0001",
  "idempotency_key": "actual-authority-drill-idem-0001",
  "nonce": "actual-authority-drill-nonce-0001",
  "expected_drill": true,
  "action_reason_redacted": "<redacted-safe-authority-drill-action>",
  "halt_status": "$halt_status",
  "recovery_status": "$recovery_status",
  "authority_status": "$authority_status",
  "clean_flow_resume_status": "$clean_status",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "system_returned_safe_state": true,
  "operator_report_redacted": true,
  "transaction_submission": false,
  "send_authorized": false,
  "wallet_loaded": false,
  "signature_generated": false,
  "authority_key_loaded": false,
  "key_rotation_executed": false,
  "upgrade_authority_changed": false,
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

template_matrix() {
  local cluster="${1:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  while IFS= read -r drill; do
    [ -n "$drill" ] || continue
    print_drill_template "$cluster" "$drill"
  done <<EOF_MATRIX
$(drill_names)
EOF_MATRIX
}

check_drill_report() {
  local report="${1:-}"
  [ -n "$report" ] || fail "--check-drill-report requires a report path"
  [ -f "$report" ] || fail "drill report not found: $report"

  reject_sensitive_text "$report" "Phase 11 drill report"

  require_json_string "$report" "schema" "rox-anchor.actual-private-testnet-authority-drill.v1"
  require_json_string "$report" "phase" "BUILD_PLAN4 Phase 11"
  require_json_string "$report" "receipt_role" "actual_private_testnet_authority_drill_report"

  require_valid_cluster "$(json_string_value "$report" "cluster")"
  require_valid_drill_name "$(json_string_value "$report" "drill_name")"

  for field in \
    drill_outcome operation_id idempotency_key nonce action_reason_redacted \
    halt_status recovery_status authority_status clean_flow_resume_status
  do
    require_json_string_present "$report" "$field"
  done

  require_drill_outcome "$(json_string_value "$report" "drill_outcome")"
  require_redacted_value "$(json_string_value "$report" "action_reason_redacted")" "action_reason_redacted"
  require_halt_status "$(json_string_value "$report" "halt_status")"
  require_recovery_status "$(json_string_value "$report" "recovery_status")"
  require_authority_status "$(json_string_value "$report" "authority_status")"
  require_clean_flow_resume_status "$(json_string_value "$report" "clean_flow_resume_status")"

  require_json_bool_true "$report" "expected_drill"
  require_json_bool_true "$report" "private_testnet_only"
  require_json_bool_true "$report" "test_only_assets_only"
  require_json_bool_true "$report" "system_returned_safe_state"
  require_json_bool_true "$report" "operator_report_redacted"

  for field in \
    transaction_submission send_authorized wallet_loaded signature_generated authority_key_loaded \
    key_rotation_executed upgrade_authority_changed public_mint_available public_launch_authorized \
    mainnet_authorized production_bridge_settlement public_rox_mint_burn real_roc_release \
    real_roc_mutation finality_claim
  do
    require_json_bool_false_or_absent "$report" "$field"
  done

  for forbidden in \
    '"drill_outcome": "success"' \
    '"halt_status": "finalized"' \
    '"recovery_status": "finalized"' \
    '"authority_status": "executed"' \
    '"clean_flow_resume_status": "settled"' \
    '"settlement": "complete"' \
    '"finality": "finalized"'
  do
    if grep -Fq "$forbidden" "$report"; then
      fail "drill report contains success-like marker: $forbidden"
    fi
    ok "drill report excludes success-like marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 11 halt/recovery/authority drill report checks passed
summary:
  - report is devnet/testnet only
  - drill name is from the required Phase 11 matrix
  - action reason is redacted
  - halt/recovery/authority/clean-flow states are inspectable
  - report rejects transaction submission, send authorization, wallet/key loading, key rotation execution, upgrade authority change, public launch, mainnet, production settlement, public ROX mint/burn, real ROC release, real ROC mutation, and finality claims
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"

  require_valid_cluster "$cluster" >/dev/null
  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/BUILD_PLAN4.md" ] || fail "BUILD_PLAN4.md missing"
  [ -f "$root/scripts/check_actual_private_testnet_negative_drills.sh" ] || fail "Phase 10 negative drill checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_receipts.sh" ] || fail "Phase 9 receipt checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_simulation.sh" ] || fail "Phase 6 simulation checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-(halt-recovery|authority)-drill.*\.json|.*actual-authority-drill.*\.json|.*actual-key-rotation.*\.json|.*actual-upgrade-authority.*\.json)$'; then
      fail "git tracked Phase 11 drill report material found"
    fi
    ok "git tracked-file scan found no Phase 11 drill report material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 11 halt/recovery/authority preflight passed
summary:
  - Phase 11 documentation and ignore boundaries are present
  - Phase 6, Phase 9, and Phase 10 checkers exist
  - no tracked Phase 11 drill report material was found
  - this preflight did not call RPC, submit, sign, load a signer, load authority keys, rotate keys, upgrade authority, mint, burn, settle, release ROC, or mutate ROC
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
  --template-drill)
    print_drill_template "${2:-testnet}" "${3:-halt_before_simulation}"
    ;;
  --template-matrix)
    template_matrix "${2:-testnet}"
    ;;
  --check-drill-report)
    check_drill_report "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
