#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 10 actual private testnet negative-drill receipt checker.
# RO:WHY — Validates redacted fail-safe receipts without calling RPC, signing, submitting, minting, burning, settling, or mutating ROC.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md, .gitignore, ignored local negative-drill receipts.
# RO:INVARIANTS — devnet/testnet only; expected failure true; no send authorization; no production settlement; no real ROC mutation.
# RO:SECURITY — local file checks only; no wallet load, RPC, live simulation, signing, submission, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-relayer --test actual_testnet_negative_drills.

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
  bash scripts/check_actual_private_testnet_negative_drills.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_negative_drills.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_negative_drills.sh --template-failure [devnet|testnet] [drill_name]
  bash scripts/check_actual_private_testnet_negative_drills.sh --template-matrix [devnet|testnet]
  bash scripts/check_actual_private_testnet_negative_drills.sh --check-failure-receipt <receipt-json>
USAGE
}

drill_names() {
  cat <<'DRILLS'
wrong_program_id
wrong_mint
wrong_token_account
wrong_authority
missing_config_account
missing_mint_account
stale_readback
under_quorum_rpc_evidence
rpc_provider_disagreement
duplicate_operation_id
duplicate_idempotency_key
nonce_reuse
receipt_tamper
missing_receipt
operator_approval_omitted
send_disabled
cap_exceeded
halt_before_simulation
halt_after_simulation_before_send
halt_after_send_before_readback
recovery_during_pending_operation
readback_missing_after_send
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

require_json_string() {
  local file="$1"
  local key="$2"
  local expected="$3"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ "$actual" = "$expected" ] || fail "negative drill receipt $key expected '$expected' but found '${actual:-<missing>}'"
  ok "negative drill receipt $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "negative drill receipt missing non-empty string field: $key"
  ok "negative drill receipt has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "negative drill receipt must set $key true"
  ok "negative drill receipt sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "negative drill receipt contains forbidden true boolean: $key"
  fi
  ok "negative drill receipt does not set $key true"
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

require_drill_outcome() {
  local value="$1"
  case "$value" in
    blocked|failed_safe|quarantined|not_performed) ok "drill_outcome is valid: $value" ;;
    *) fail "drill_outcome must be blocked, failed_safe, quarantined, or not_performed, got: ${value:-<missing>}" ;;
  esac
}

require_proof_status() {
  local value="$1"
  case "$value" in
    rejected|blocked|disputed|missing_evidence) ok "proof_review_status is valid: $value" ;;
    *) fail "proof_review_status must be rejected, blocked, disputed, or missing_evidence, got: ${value:-<missing>}" ;;
  esac
}

require_coordinator_status() {
  local value="$1"
  case "$value" in
    rejected|blocked) ok "coordinator_decision_status is valid: $value" ;;
    *) fail "coordinator_decision_status must be rejected or blocked, got: ${value:-<missing>}" ;;
  esac
}

require_relayer_status() {
  local value="$1"
  case "$value" in
    blocked|not_authorized) ok "relayer_status is valid: $value" ;;
    *) fail "relayer_status must be blocked or not_authorized, got: ${value:-<missing>}" ;;
  esac
}

require_readback_status() {
  local value="$1"
  case "$value" in
    missing|rejected|disputed|not_performed) ok "readback_status is valid: $value" ;;
    *) fail "readback_status must be missing, rejected, disputed, or not_performed, got: ${value:-<missing>}" ;;
  esac
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md"
  local script="$root/scripts/check_actual_private_testnet_negative_drills.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_NEGATIVE_DRILLS.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_negative_drills.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 10" \
    "Actual Negative Drills Against Deployed Testnet State" \
    "actual_private_testnet_negative_drill_receipt" \
    "rox-anchor.actual-private-testnet-negative-drill.v1" \
    "wrong program ID" \
    "wrong mint" \
    "wrong token account" \
    "wrong authority" \
    "missing config account" \
    "missing mint account" \
    "stale readback" \
    "under-quorum RPC evidence" \
    "RPC provider disagreement" \
    "duplicate operation ID" \
    "duplicate idempotency key" \
    "nonce reuse" \
    "receipt tamper" \
    "missing receipt" \
    "operator approval omitted" \
    "send disabled" \
    "cap exceeded" \
    "halt before simulation" \
    "halt after simulation before send" \
    "halt after send before readback" \
    "recovery during pending operation" \
    "readback missing after send" \
    "expected_failure" \
    "failure_reason_redacted" \
    "proof_review_status" \
    "coordinator_decision_status" \
    "relayer_status" \
    "readback_status" \
    "production_bridge_settlement" \
    "real_roc_release" \
    "real_roc_mutation" \
    "finality_claim" \
    "No real ROC release." \
    "No real internal ROC mutation." \
    "No production bridge settlement." \
    "No fake finality."
  do
    grep -Fq -- "$needle" "$doc" || fail "actual negative drill doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-negative-drill.json" \
    "actual-private-testnet-negative-drill.local.json" \
    "actual-private-testnet-negative-drill-quarantined.local.json" \
    "actual-private-testnet-negative-drill-failure-receipt.local.json" \
    "*.actual-negative-drill.local.json" \
    "*.actual-negative-drill-receipt.local.json" \
    "*.actual-negative-drill-failed.local.json" \
    "*.actual-negative-drill-quarantine.local.json" \
    "*.negative-drill-receipt.local.json" \
    "*.negative-drill-report.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing negative-drill ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "actual negative drill doc"

  for forbidden in \
    "public_mint_available\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_release\": true" \
    "real_roc_mutation\": true" \
    "finality_claim\": true" \
    "transaction_submission\": true" \
    "send_authorized\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "actual negative drill doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 10 actual negative drill documentation checks passed
summary:
  - actual private testnet negative drill runbook exists
  - local negative drill receipt artifact names are ignored
  - documentation preserves drill matrix, redaction, fail-safe, non-mainnet, and non-production boundaries
  - documentation separates failure receipts from finality, settlement, public mint/burn, real ROC release, and real ROC mutation
SUMMARY
}

status_for_drill() {
  local drill="$1"
  case "$drill" in
    stale_readback|under_quorum_rpc_evidence|rpc_provider_disagreement|readback_missing_after_send)
      printf 'disputed'
      ;;
    missing_config_account|missing_mint_account|missing_receipt)
      printf 'missing_evidence'
      ;;
    halt_before_simulation|halt_after_simulation_before_send|halt_after_send_before_readback|recovery_during_pending_operation|operator_approval_omitted|send_disabled|cap_exceeded)
      printf 'blocked'
      ;;
    *)
      printf 'rejected'
      ;;
  esac
}

readback_for_drill() {
  local drill="$1"
  case "$drill" in
    stale_readback|readback_missing_after_send)
      printf 'missing'
      ;;
    under_quorum_rpc_evidence|rpc_provider_disagreement)
      printf 'disputed'
      ;;
    *)
      printf 'not_performed'
      ;;
  esac
}

print_failure_template() {
  local cluster="${1:-testnet}"
  local drill="${2:-wrong_program_id}"

  require_valid_cluster "$cluster" >/dev/null
  require_valid_drill_name "$drill" >/dev/null

  local proof_status
  local coordinator_status
  local relayer_status
  local readback_status

  proof_status="$(status_for_drill "$drill")"
  case "$proof_status" in
    blocked|missing_evidence) coordinator_status="blocked" ;;
    *) coordinator_status="rejected" ;;
  esac

  case "$drill" in
    operator_approval_omitted|send_disabled|cap_exceeded|halt_after_simulation_before_send)
      relayer_status="not_authorized"
      ;;
    *)
      relayer_status="blocked"
      ;;
  esac

  readback_status="$(readback_for_drill "$drill")"

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-negative-drill.v1",
  "phase": "BUILD_PLAN4 Phase 10",
  "receipt_role": "actual_private_testnet_negative_drill_receipt",
  "cluster": "$cluster",
  "drill_name": "$drill",
  "drill_outcome": "blocked",
  "operation_id": "actual-negative-drill-op-0001",
  "idempotency_key": "actual-negative-drill-idem-0001",
  "nonce": "actual-negative-drill-nonce-0001",
  "expected_failure": true,
  "failure_reason_redacted": "<redacted-safe-negative-drill-failure>",
  "proof_review_status": "$proof_status",
  "coordinator_decision_status": "$coordinator_status",
  "relayer_status": "$relayer_status",
  "readback_status": "$readback_status",
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "system_returned_safe_state": true,
  "clean_operation_after_matrix_status": "not_performed",
  "transaction_submission": false,
  "send_authorized": false,
  "signature_generated": false,
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
    print_failure_template "$cluster" "$drill"
  done <<EOF_MATRIX
$(drill_names)
EOF_MATRIX
}

check_failure_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-failure-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "negative drill receipt not found: $receipt"

  reject_sensitive_text "$receipt" "actual negative drill receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-private-testnet-negative-drill.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 10"
  require_json_string "$receipt" "receipt_role" "actual_private_testnet_negative_drill_receipt"

  require_valid_cluster "$(json_string_value "$receipt" "cluster")"
  require_valid_drill_name "$(json_string_value "$receipt" "drill_name")"

  for field in \
    drill_outcome operation_id idempotency_key nonce failure_reason_redacted \
    proof_review_status coordinator_decision_status relayer_status readback_status
  do
    require_json_string_present "$receipt" "$field"
  done

  require_drill_outcome "$(json_string_value "$receipt" "drill_outcome")"
  require_redacted_value "$(json_string_value "$receipt" "failure_reason_redacted")" "failure_reason_redacted"
  require_proof_status "$(json_string_value "$receipt" "proof_review_status")"
  require_coordinator_status "$(json_string_value "$receipt" "coordinator_decision_status")"
  require_relayer_status "$(json_string_value "$receipt" "relayer_status")"
  require_readback_status "$(json_string_value "$receipt" "readback_status")"

  require_json_bool_true "$receipt" "expected_failure"
  require_json_bool_true "$receipt" "private_testnet_only"
  require_json_bool_true "$receipt" "test_only_assets_only"
  require_json_bool_true "$receipt" "system_returned_safe_state"

  for field in \
    transaction_submission send_authorized signature_generated public_mint_available \
    public_launch_authorized mainnet_authorized production_bridge_settlement \
    public_rox_mint_burn real_roc_release real_roc_mutation finality_claim
  do
    require_json_bool_false_or_absent "$receipt" "$field"
  done

  for forbidden in \
    '"drill_outcome": "success"' \
    '"proof_review_status": "accepted"' \
    '"coordinator_decision_status": "accepted"' \
    '"relayer_status": "submitted"' \
    '"readback_status": "verified"' \
    '"finality": "finalized"' \
    '"settlement": "complete"'
  do
    if grep -Fq "$forbidden" "$receipt"; then
      fail "negative drill receipt contains success-like marker: $forbidden"
    fi
    ok "negative drill receipt excludes success-like marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 10 actual negative drill failure receipt checks passed
summary:
  - receipt is devnet/testnet only
  - drill name is from the required Phase 10 matrix
  - failure reason is redacted
  - proof/coordinator/relayer/readback statuses are fail-safe statuses
  - receipt rejects submission, send authorization, signatures, public launch, mainnet, production settlement, public ROX mint/burn, real ROC release, real ROC mutation, and finality claims
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/Anchor.toml" ] || fail "Anchor.toml missing"
  [ -f "$root/BUILD_PLAN4.md" ] || fail "BUILD_PLAN4.md missing"
  [ -f "$root/scripts/check_actual_private_testnet_workspace.sh" ] || fail "Phase 1 workspace checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_read_only_evidence.sh" ] || fail "Phase 5 read-only evidence checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_simulation.sh" ] || fail "Phase 6 simulation checker missing"
  [ -f "$root/scripts/check_actual_roc_to_rox_private_testnet_run.sh" ] || fail "Phase 7 ROC-to-ROX checker missing"
  [ -f "$root/scripts/check_actual_rox_to_roc_private_testnet_run.sh" ] || fail "Phase 8 ROX-to-ROC checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_receipts.sh" ] || fail "Phase 9 receipt ledger checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-negative-drill.*\.json|.*actual-negative-drill.*\.json|.*negative-drill-receipt.*\.json)$'; then
      fail "git tracked actual negative drill receipt material found"
    fi
    ok "git tracked-file scan found no actual negative drill receipt material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 10 actual negative drill preflight passed
summary:
  - Phase 10 documentation and ignore boundaries are present
  - Phase 1 and Phase 5 through Phase 9 checkers exist
  - no tracked actual negative drill receipt material was found
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
  --template-failure)
    print_failure_template "${2:-testnet}" "${3:-wrong_program_id}"
    ;;
  --template-matrix)
    template_matrix "${2:-testnet}"
    ;;
  --check-failure-receipt)
    check_failure_receipt "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
