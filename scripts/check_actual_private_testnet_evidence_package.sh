#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 14 actual private testnet evidence package checker.
# RO:WHY — Validates redacted evidence-package/index reports without granting runtime authorization, settlement, mainnet, public launch, or real ROC mutation.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md, .gitignore, Phase 9-13 checkers.
# RO:INVARIANTS — evidence package only; private devnet/testnet only; test-only assets; no runtime authorization; no real ROC mutation; no finality claim.
# RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, transaction submission, mint, burn, settlement, paid unlock, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_evidence_package.

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
  bash scripts/check_actual_private_testnet_evidence_package.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_evidence_package.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_evidence_package.sh --template-package [devnet|testnet]
  bash scripts/check_actual_private_testnet_evidence_package.sh --check-package <package-json>
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
  [ "$actual" = "$expected" ] || fail "evidence package $key expected '$expected' but found '${actual:-<missing>}'"
  ok "evidence package $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "evidence package missing non-empty string field: $key"
  ok "evidence package has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "evidence package must set $key true"
  ok "evidence package sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "evidence package contains forbidden true boolean: $key"
  fi
  ok "evidence package does not set $key true"
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

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md"
  local script="$root/scripts/check_actual_private_testnet_evidence_package.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_EVIDENCE_PACKAGE.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_evidence_package.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 14" \
    "Actual Private Testnet Evidence Package" \
    "rox-anchor.actual-private-testnet-evidence-package.v1" \
    "actual_private_testnet_evidence_package" \
    "build artifact manifest" \
    "deployment receipt or safe failed-deployment receipt" \
    "test-only mint/config initialization receipt" \
    "read-only RPC evidence receipt" \
    "simulation receipts" \
    "ROC-to-ROX capped testnet receipt/readback" \
    "ROX-to-ROC capped testnet receipt/readback" \
    "negative drill failure receipts" \
    "halt/recovery drill reports" \
    "authority drill reports" \
    "RustyOnions dry-run handoff report" \
    "CrabLink display-only status report" \
    "No wallet authority." \
    "No ledger authority." \
    "No bridge authority." \
    "No runtime authorization." \
    "No transaction submission." \
    "No production bridge settlement." \
    "No real internal ROC mutation." \
    "No final settlement." \
    "The actual private testnet evidence package is an audit index only."
  do
    grep -Fq -- "$needle" "$doc" || fail "evidence package doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-evidence-package.json" \
    "actual-private-testnet-evidence-package.local.json" \
    "actual-private-testnet-evidence-index.local.json" \
    "actual-private-testnet-evidence-report.local.json" \
    "*.actual-private-testnet-evidence-package.local.json" \
    "*.actual-private-testnet-evidence-index.local.json" \
    "*.actual-private-testnet-evidence-report.local.json" \
    "*.actual-evidence-package.local.json" \
    "*.actual-evidence-index.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing Phase 14 ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "actual private testnet evidence package doc"

  for forbidden in \
    "runtime_authorization\": true" \
    "wallet_authority\": true" \
    "ledger_authority\": true" \
    "bridge_authority\": true" \
    "transaction_submission\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_burn\": true" \
    "real_roc_release\": true" \
    "real_roc_mutation\": true" \
    "final_settlement\": true" \
    "finality_claim\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "evidence package doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 14 actual private testnet evidence package documentation checks passed
summary:
  - actual private testnet evidence package runbook exists
  - local Phase 14 evidence package artifact names are ignored
  - documentation preserves redacted, evidence-index-only, private-testnet, and test-only boundaries
  - documentation separates evidence packaging from runtime authorization, wallet authority, ledger authority, bridge authority, transaction submission, public launch, mainnet, production settlement, public ROX mint/burn, real ROC burn/release/mutation, final settlement, and finality
SUMMARY
}

template_package() {
  local cluster="${1:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-evidence-package.v1",
  "phase": "BUILD_PLAN4 Phase 14",
  "package_role": "actual_private_testnet_evidence_package",
  "cluster": "$cluster",
  "package_id": "actual-private-testnet-evidence-package-0001",
  "evidence_index_status": "audit_ready",
  "build_artifact_manifest_status": "linked",
  "deploy_receipt_status": "linked_or_not_performed",
  "test_only_mint_init_status": "linked_or_not_performed",
  "read_only_rpc_evidence_status": "linked_or_not_performed",
  "simulation_receipts_status": "linked_or_not_performed",
  "roc_to_rox_receipts_status": "linked_or_not_performed",
  "rox_to_roc_receipts_status": "linked_or_not_performed",
  "receipt_ledger_status": "linked",
  "negative_drill_receipts_status": "linked",
  "halt_recovery_reports_status": "linked",
  "authority_reports_status": "linked",
  "rustyonions_handoff_status": "linked",
  "crablink_display_status": "linked",
  "operation_id_linkage_status": "validated",
  "idempotency_key_linkage_status": "validated",
  "receipt_id_linkage_status": "validated",
  "redaction_status": "redacted",
  "operator_report_redacted": true,
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "evidence_package_only": true,
  "runtime_authorization": false,
  "wallet_authority": false,
  "ledger_authority": false,
  "bridge_authority": false,
  "transaction_submission": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_burn": false,
  "real_roc_release": false,
  "real_roc_mutation": false,
  "final_settlement": false,
  "finality_claim": false
}
TEMPLATE
}

check_package() {
  local package="${1:-}"
  [ -n "$package" ] || fail "--check-package requires a package path"
  [ -f "$package" ] || fail "evidence package not found: $package"

  reject_sensitive_text "$package" "actual private testnet evidence package"

  require_json_string "$package" "schema" "rox-anchor.actual-private-testnet-evidence-package.v1"
  require_json_string "$package" "phase" "BUILD_PLAN4 Phase 14"
  require_json_string "$package" "package_role" "actual_private_testnet_evidence_package"

  require_valid_cluster "$(json_string_value "$package" "cluster")"

  for field in \
    package_id evidence_index_status build_artifact_manifest_status deploy_receipt_status \
    test_only_mint_init_status read_only_rpc_evidence_status simulation_receipts_status \
    roc_to_rox_receipts_status rox_to_roc_receipts_status receipt_ledger_status \
    negative_drill_receipts_status halt_recovery_reports_status authority_reports_status \
    rustyonions_handoff_status crablink_display_status operation_id_linkage_status \
    idempotency_key_linkage_status receipt_id_linkage_status redaction_status
  do
    require_json_string_present "$package" "$field"
  done

  require_status_value "$(json_string_value "$package" "evidence_index_status")" "evidence_index_status" audit_ready incomplete quarantined
  require_status_value "$(json_string_value "$package" "build_artifact_manifest_status")" "build_artifact_manifest_status" linked missing quarantined
  require_status_value "$(json_string_value "$package" "deploy_receipt_status")" "deploy_receipt_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$package" "test_only_mint_init_status")" "test_only_mint_init_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$package" "read_only_rpc_evidence_status")" "read_only_rpc_evidence_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$package" "simulation_receipts_status")" "simulation_receipts_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$package" "roc_to_rox_receipts_status")" "roc_to_rox_receipts_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$package" "rox_to_roc_receipts_status")" "rox_to_roc_receipts_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$package" "receipt_ledger_status")" "receipt_ledger_status" linked missing quarantined
  require_status_value "$(json_string_value "$package" "negative_drill_receipts_status")" "negative_drill_receipts_status" linked missing quarantined
  require_status_value "$(json_string_value "$package" "halt_recovery_reports_status")" "halt_recovery_reports_status" linked missing quarantined
  require_status_value "$(json_string_value "$package" "authority_reports_status")" "authority_reports_status" linked missing quarantined
  require_status_value "$(json_string_value "$package" "rustyonions_handoff_status")" "rustyonions_handoff_status" linked missing quarantined
  require_status_value "$(json_string_value "$package" "crablink_display_status")" "crablink_display_status" linked not_performed missing quarantined
  require_status_value "$(json_string_value "$package" "operation_id_linkage_status")" "operation_id_linkage_status" validated incomplete quarantined
  require_status_value "$(json_string_value "$package" "idempotency_key_linkage_status")" "idempotency_key_linkage_status" validated incomplete quarantined
  require_status_value "$(json_string_value "$package" "receipt_id_linkage_status")" "receipt_id_linkage_status" validated incomplete quarantined
  require_status_value "$(json_string_value "$package" "redaction_status")" "redaction_status" redacted incomplete quarantined

  require_json_bool_true "$package" "operator_report_redacted"
  require_json_bool_true "$package" "private_testnet_only"
  require_json_bool_true "$package" "test_only_assets_only"
  require_json_bool_true "$package" "evidence_package_only"

  for field in \
    runtime_authorization wallet_authority ledger_authority bridge_authority transaction_submission \
    public_launch_authorized mainnet_authorized production_bridge_settlement public_rox_mint_burn \
    real_roc_burn real_roc_release real_roc_mutation final_settlement finality_claim
  do
    require_json_bool_false_or_absent "$package" "$field"
  done

  for forbidden in \
    '"evidence_index_status": "finalized"' \
    '"deploy_receipt_status": "production"' \
    '"receipt_ledger_status": "settled"' \
    '"rustyonions_handoff_status": "mutated"' \
    '"crablink_display_status": "unlocked"' \
    '"settlement": "complete"' \
    '"finality": "finalized"' \
    '"mainnet": "authorized"'
  do
    if grep -Fq "$forbidden" "$package"; then
      fail "evidence package contains forbidden success/authority marker: $forbidden"
    fi
    ok "evidence package excludes success/authority marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 14 actual private testnet evidence package checks passed
summary:
  - package is devnet/testnet only
  - package is evidence-index-only and redacted
  - package covers build, deploy, initialization, read-only RPC, simulation, directional receipts, receipt ledger, negative drills, halt/recovery, authority drills, RustyOnions handoff, and CrabLink display-only status
  - package validates operation ID, idempotency key, receipt ID, and redaction status labels
  - package rejects runtime authorization, wallet authority, ledger authority, bridge authority, transaction submission, public launch, mainnet, production settlement, public ROX mint/burn, real ROC burn/release/mutation, final settlement, and finality claims
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
  [ -f "$root/scripts/check_actual_rustyonions_dry_run_handoff.sh" ] || fail "Phase 12 RustyOnions handoff checker missing"
  [ -f "$root/scripts/check_actual_crablink_private_testnet_status.sh" ] || fail "Phase 13 CrabLink status checker missing"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-evidence-(package|index|report).*\.json|.*actual-evidence-(package|index).*\.json)$'; then
      fail "git tracked Phase 14 evidence package material found"
    fi
    ok "git tracked-file scan found no Phase 14 evidence package material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 14 actual private testnet evidence package preflight passed
summary:
  - Phase 14 documentation and ignore boundaries are present
  - Phase 9, Phase 10, Phase 11, Phase 12, and Phase 13 checkers exist
  - no tracked Phase 14 evidence package material was found
  - this preflight did not call RPC, submit, sign, load a signer, load authority keys, call svc-wallet, call ron-ledger, mint, burn, settle, release ROC, mutate ROC, unlock paid content, or grant runtime authority
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
  --template-package)
    template_package "${2:-testnet}"
    ;;
  --check-package)
    check_package "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
