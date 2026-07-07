#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff checker.
# RO:WHY — Validates dry-run-only handoff reports without wallet/ledger mutation, real ROC release, settlement, or finality claims.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md, .gitignore, Phase 9/10/11 checkers.
# RO:INVARIANTS — svc-wallet -> ron-ledger remains future real ROC boundary; ROX Anchor only records dry-run intent/status.
# RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, submission, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-core --test actual_rustyonions_dry_run_handoff; cargo test -p rox-anchor-coordinator --test actual_rustyonions_dry_run_handoff; cargo test -p rox-anchor-cli --test actual_rustyonions_dry_run_status.

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
  bash scripts/check_actual_rustyonions_dry_run_handoff.sh --check-docs [repo-root]
  bash scripts/check_actual_rustyonions_dry_run_handoff.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_rustyonions_dry_run_handoff.sh --template-report [devnet|testnet] [roc_to_rox|rox_to_roc]
  bash scripts/check_actual_rustyonions_dry_run_handoff.sh --check-report <report-json>
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

require_valid_direction() {
  case "${1:-}" in
    roc_to_rox|rox_to_roc) ok "direction is $1" ;;
    *) fail "direction must be roc_to_rox or rox_to_roc, got: ${1:-<empty>}" ;;
  esac
}

require_json_string() {
  local file="$1"
  local key="$2"
  local expected="$3"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ "$actual" = "$expected" ] || fail "handoff report $key expected '$expected' but found '${actual:-<missing>}'"
  ok "handoff report $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "handoff report missing non-empty string field: $key"
  ok "handoff report has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "handoff report must set $key true"
  ok "handoff report sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "handoff report contains forbidden true boolean: $key"
  fi
  ok "handoff report does not set $key true"
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
    "ledger-private" \
    "wallet-private"
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

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md"
  local script="$root/scripts/check_actual_rustyonions_dry_run_handoff.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_RUSTYONIONS_DRY_RUN_HANDOFF.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_rustyonions_dry_run_handoff.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 12" \
    "RustyOnions Dry-Run Handoff Evidence" \
    "rox-anchor.actual-rustyonions-dry-run-handoff.v1" \
    "actual_rustyonions_dry_run_handoff_report" \
    "svc-wallet -> ron-ledger" \
    "roc_to_rox" \
    "rox_to_roc" \
    "shadow_roc_burn_intent_only" \
    "internal_roc_release_intent_only" \
    "dry_run_only" \
    "svc_wallet_mutation" \
    "ron_ledger_mutation" \
    "real_roc_release" \
    "real_roc_mutation" \
    "No real ROC burn." \
    "No real ROC release." \
    "No real internal ROC mutation." \
    "No svc-wallet mutation." \
    "No ron-ledger mutation." \
    "No production bridge settlement." \
    "The RustyOnions handoff remains dry-run only."
  do
    grep -Fq -- "$needle" "$doc" || fail "RustyOnions dry-run handoff doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-rustyonions-dry-run-handoff.json" \
    "actual-rustyonions-dry-run-handoff.local.json" \
    "actual-rustyonions-roc-to-rox-dry-run-handoff.local.json" \
    "actual-rustyonions-rox-to-roc-dry-run-handoff.local.json" \
    "*.actual-rustyonions-dry-run-handoff.local.json" \
    "*.actual-rustyonions-roc-to-rox-handoff.local.json" \
    "*.actual-rustyonions-rox-to-roc-handoff.local.json" \
    "*.actual-rustyonions-handoff-report.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing Phase 12 ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "RustyOnions dry-run handoff doc"

  for forbidden in \
    "dry_run_only\": false" \
    "svc_wallet_mutation\": true" \
    "ron_ledger_mutation\": true" \
    "real_roc_burn\": true" \
    "real_roc_release\": true" \
    "real_roc_mutation\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "mainnet_authorized\": true" \
    "public_launch_authorized\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "RustyOnions handoff doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff documentation checks passed
summary:
  - RustyOnions dry-run handoff runbook exists
  - local Phase 12 dry-run handoff artifact names are ignored
  - documentation preserves svc-wallet -> ron-ledger as the future real ROC mutation boundary
  - documentation separates ROX Anchor evidence from wallet mutation, ledger mutation, real ROC burn/release, production settlement, public mint/burn, mainnet, launch, and finality
SUMMARY
}

template_report() {
  local cluster="${1:-testnet}"
  local direction="${2:-roc_to_rox}"

  require_valid_cluster "$cluster" >/dev/null
  require_valid_direction "$direction" >/dev/null

  local shadow_burn="false"
  local release_intent="false"

  if [ "$direction" = "roc_to_rox" ]; then
    shadow_burn="true"
  else
    release_intent="true"
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-rustyonions-dry-run-handoff.v1",
  "phase": "BUILD_PLAN4 Phase 12",
  "report_role": "actual_rustyonions_dry_run_handoff_report",
  "cluster": "$cluster",
  "direction": "$direction",
  "operation_id": "actual-rustyonions-dry-run-op-0001",
  "idempotency_key": "actual-rustyonions-dry-run-idem-0001",
  "nonce": "actual-rustyonions-dry-run-nonce-0001",
  "source_receipt_ledger_status": "linked",
  "source_private_testnet_receipts_status": "redacted_verified",
  "proof_review_status": "accepted",
  "coordinator_decision_status": "accepted",
  "relayer_status": "dry_run_only",
  "rustyonions_handoff_status": "dry_run_recorded",
  "rustyonions_target_boundary": "svc-wallet -> ron-ledger",
  "dry_run_only": true,
  "shadow_roc_burn_intent_only": $shadow_burn,
  "internal_roc_release_intent_only": $release_intent,
  "svc_wallet_mutation": false,
  "ron_ledger_mutation": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "mainnet_authorized": false,
  "public_launch_authorized": false,
  "finality_claim": false,
  "operator_report_redacted": true
}
TEMPLATE
}

check_report() {
  local report="${1:-}"
  [ -n "$report" ] || fail "--check-report requires a report path"
  [ -f "$report" ] || fail "handoff report not found: $report"

  reject_sensitive_text "$report" "RustyOnions dry-run handoff report"

  require_json_string "$report" "schema" "rox-anchor.actual-rustyonions-dry-run-handoff.v1"
  require_json_string "$report" "phase" "BUILD_PLAN4 Phase 12"
  require_json_string "$report" "report_role" "actual_rustyonions_dry_run_handoff_report"

  require_valid_cluster "$(json_string_value "$report" "cluster")"
  require_valid_direction "$(json_string_value "$report" "direction")"

  for field in \
    operation_id idempotency_key nonce source_receipt_ledger_status \
    source_private_testnet_receipts_status proof_review_status coordinator_decision_status \
    relayer_status rustyonions_handoff_status rustyonions_target_boundary
  do
    require_json_string_present "$report" "$field"
  done

  require_json_string "$report" "rustyonions_target_boundary" "svc-wallet -> ron-ledger"

  require_status_value "$(json_string_value "$report" "source_receipt_ledger_status")" "source_receipt_ledger_status" linked quarantined
  require_status_value "$(json_string_value "$report" "source_private_testnet_receipts_status")" "source_private_testnet_receipts_status" redacted_verified redacted_quarantined
  require_status_value "$(json_string_value "$report" "proof_review_status")" "proof_review_status" accepted blocked rejected disputed missing_evidence
  require_status_value "$(json_string_value "$report" "coordinator_decision_status")" "coordinator_decision_status" accepted blocked rejected
  require_status_value "$(json_string_value "$report" "relayer_status")" "relayer_status" dry_run_only blocked not_authorized
  require_status_value "$(json_string_value "$report" "rustyonions_handoff_status")" "rustyonions_handoff_status" dry_run_recorded blocked quarantined

  require_json_bool_true "$report" "dry_run_only"
  require_json_bool_true "$report" "operator_report_redacted"

  local direction
  direction="$(json_string_value "$report" "direction")"
  if [ "$direction" = "roc_to_rox" ]; then
    require_json_bool_true "$report" "shadow_roc_burn_intent_only"
    require_json_bool_false_or_absent "$report" "internal_roc_release_intent_only"
  else
    require_json_bool_true "$report" "internal_roc_release_intent_only"
    require_json_bool_false_or_absent "$report" "shadow_roc_burn_intent_only"
  fi

  for field in \
    svc_wallet_mutation ron_ledger_mutation real_roc_burn real_roc_release real_roc_mutation \
    production_bridge_settlement public_rox_mint_burn mainnet_authorized public_launch_authorized finality_claim
  do
    require_json_bool_false_or_absent "$report" "$field"
  done

  for forbidden in \
    '"rustyonions_handoff_status": "mutated"' \
    '"rustyonions_handoff_status": "settled"' \
    '"relayer_status": "submitted"' \
    '"source_receipt_ledger_status": "finalized"' \
    '"finality": "finalized"' \
    '"settlement": "complete"'
  do
    if grep -Fq "$forbidden" "$report"; then
      fail "handoff report contains forbidden success-like marker: $forbidden"
    fi
    ok "handoff report excludes success-like marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff report checks passed
summary:
  - report is devnet/testnet only
  - direction is roc_to_rox or rox_to_roc
  - svc-wallet -> ron-ledger remains the target boundary for future real ROC mutation
  - handoff is dry-run only and redacted
  - report rejects wallet mutation, ledger mutation, real ROC burn/release, real ROC mutation, production settlement, public ROX mint/burn, mainnet, launch, and finality claims
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"

  require_valid_cluster "$cluster" >/dev/null
  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/BUILD_PLAN4.md" ] || fail "BUILD_PLAN4.md missing"
  [ -f "$root/scripts/check_actual_private_testnet_receipts.sh" ] || fail "Phase 9 receipt checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_negative_drills.sh" ] || fail "Phase 10 negative drill checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh" ] || fail "Phase 11 halt/recovery/authority checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-rustyonions-.*handoff.*\.json|.*rustyonions.*dry-run.*\.json)$'; then
      fail "git tracked Phase 12 RustyOnions handoff material found"
    fi
    ok "git tracked-file scan found no Phase 12 RustyOnions handoff material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 12 RustyOnions dry-run handoff preflight passed
summary:
  - Phase 12 documentation and ignore boundaries are present
  - Phase 9, Phase 10, and Phase 11 checkers exist
  - no tracked Phase 12 handoff material was found
  - this preflight did not call RPC, submit, sign, load a signer, load authority keys, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, or mutate ROC
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
  --template-report)
    template_report "${2:-testnet}" "${3:-roc_to_rox}"
    ;;
  --check-report)
    check_report "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
