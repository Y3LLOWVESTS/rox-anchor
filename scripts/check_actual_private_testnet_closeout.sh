#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 15 closeout checker.
# RO:WHY — Validates a redacted closeout gate report without authorizing runtime, public launch, mainnet, production settlement, or real ROC mutation.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md, BUILD_PLAN4.md, BUILD_PLAN5.md, prior Phase 1-14 checkers.
# RO:INVARIANTS — closeout gate only; BUILD_PLAN4 complete/green/parked; BUILD_PLAN5 separate/future; no production/mainnet/public/real-ROC behavior.
# RO:SECURITY — local file checks only; no RPC, signer load, wallet call, ledger call, transaction submission, mint, burn, settlement, paid unlock, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_closeout.

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
  bash scripts/check_actual_private_testnet_closeout.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_closeout.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_closeout.sh --template-closeout [devnet|testnet]
  bash scripts/check_actual_private_testnet_closeout.sh --check-closeout <closeout-json>
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
  [ "$actual" = "$expected" ] || fail "closeout report $key expected '$expected' but found '${actual:-<missing>}'"
  ok "closeout report $key = $expected"
}

require_json_string_present() {
  local file="$1"
  local key="$2"
  local actual
  actual="$(json_string_value "$file" "$key")"
  [ -n "$actual" ] || fail "closeout report missing non-empty string field: $key"
  ok "closeout report has $key"
}

require_json_bool_true() {
  local file="$1"
  local key="$2"
  contains_json_bool_true "$file" "$key" || fail "closeout report must set $key true"
  ok "closeout report sets $key true"
}

require_json_bool_false_or_absent() {
  local file="$1"
  local key="$2"
  if contains_json_bool_true "$file" "$key"; then
    fail "closeout report contains forbidden true boolean: $key"
  fi
  ok "closeout report does not set $key true"
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

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md"
  local script="$root/scripts/check_actual_private_testnet_closeout.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_CLOSEOUT.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_closeout.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 15" \
    "BUILD_PLAN4 Closeout Gate" \
    "rox-anchor.actual-private-testnet-closeout.v1" \
    "actual_private_testnet_closeout_gate" \
    "complete / green / parked" \
    "BUILD_PLAN5 remains separate and future" \
    "all local Rust tests pass" \
    "all actual private testnet checks pass" \
    "RustyOnions handoff remains dry-run only" \
    "CrabLink status remains display-only" \
    "no key material is tracked" \
    "no mainnet behavior exists" \
    "no public launch behavior exists" \
    "no production settlement behavior exists" \
    "no real internal ROC mutation exists" \
    "no exchange/staking/liquidity behavior exists" \
    "No runtime authorization." \
    "No transaction submission." \
    "No production bridge settlement." \
    "No real internal ROC mutation." \
    "No final settlement." \
    "No fake finality."
  do
    grep -Fq -- "$needle" "$doc" || fail "closeout doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-closeout.json" \
    "actual-private-testnet-closeout.local.json" \
    "actual-private-testnet-closeout-report.local.json" \
    "actual-build-plan4-closeout.local.json" \
    "*.actual-private-testnet-closeout.local.json" \
    "*.actual-private-testnet-closeout-report.local.json" \
    "*.actual-build-plan4-closeout.local.json" \
    "*.actual-closeout-report.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing Phase 15 ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "actual private testnet closeout doc"

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
      fail "closeout doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 15 closeout documentation checks passed
summary:
  - actual private testnet closeout runbook exists
  - local Phase 15 closeout artifact names are ignored
  - documentation preserves closeout-gate-only, private-testnet, test-only, and BUILD_PLAN5-separate boundaries
  - documentation separates BUILD_PLAN4 completion from runtime authorization, wallet authority, ledger authority, bridge authority, transaction submission, public launch, mainnet, production settlement, public ROX mint/burn, real ROC burn/release/mutation, final settlement, and finality
SUMMARY
}

template_closeout() {
  local cluster="${1:-testnet}"
  require_valid_cluster "$cluster" >/dev/null

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-closeout.v1",
  "phase": "BUILD_PLAN4 Phase 15",
  "closeout_role": "actual_private_testnet_closeout_gate",
  "cluster": "$cluster",
  "closeout_status": "complete_green_parked",
  "build_plan4_status": "complete_green_parked",
  "build_plan5_status": "separate_future_plan",
  "local_rust_tests_status": "passed",
  "anchor_build_status": "operator_verified_or_not_performed",
  "anchor_test_status": "operator_verified_or_not_performed",
  "actual_private_testnet_checks_status": "passed",
  "deploy_receipt_status": "linked_or_not_performed",
  "test_only_mint_init_status": "linked_or_not_performed",
  "live_read_only_rpc_status": "linked_or_not_performed",
  "simulation_receipts_status": "linked_or_not_performed",
  "capped_send_receipts_status": "linked_or_not_performed",
  "readback_receipts_status": "linked_or_not_performed",
  "negative_drill_failure_receipts_status": "linked",
  "halt_recovery_drills_status": "linked",
  "authority_drills_status": "linked",
  "rustyonions_handoff_status": "dry_run_only",
  "crablink_display_status": "display_only",
  "tracked_key_material_status": "none_tracked",
  "mainnet_behavior_status": "absent",
  "public_launch_status": "absent",
  "production_settlement_status": "absent",
  "real_internal_roc_mutation_status": "absent",
  "exchange_staking_liquidity_status": "absent",
  "known_pilot_failures_status": "none_observed_or_documented",
  "operator_report_redacted": true,
  "private_testnet_only": true,
  "test_only_assets_only": true,
  "closeout_gate_only": true,
  "future_build_plan5_required": true,
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

check_closeout() {
  local report="${1:-}"
  [ -n "$report" ] || fail "--check-closeout requires a report path"
  [ -f "$report" ] || fail "closeout report not found: $report"

  reject_sensitive_text "$report" "actual private testnet closeout report"

  require_json_string "$report" "schema" "rox-anchor.actual-private-testnet-closeout.v1"
  require_json_string "$report" "phase" "BUILD_PLAN4 Phase 15"
  require_json_string "$report" "closeout_role" "actual_private_testnet_closeout_gate"

  require_valid_cluster "$(json_string_value "$report" "cluster")"

  for field in \
    closeout_status build_plan4_status build_plan5_status local_rust_tests_status \
    anchor_build_status anchor_test_status actual_private_testnet_checks_status \
    deploy_receipt_status test_only_mint_init_status live_read_only_rpc_status \
    simulation_receipts_status capped_send_receipts_status readback_receipts_status \
    negative_drill_failure_receipts_status halt_recovery_drills_status authority_drills_status \
    rustyonions_handoff_status crablink_display_status tracked_key_material_status \
    mainnet_behavior_status public_launch_status production_settlement_status \
    real_internal_roc_mutation_status exchange_staking_liquidity_status known_pilot_failures_status
  do
    require_json_string_present "$report" "$field"
  done

  require_status_value "$(json_string_value "$report" "closeout_status")" "closeout_status" complete_green_parked incomplete quarantined
  require_status_value "$(json_string_value "$report" "build_plan4_status")" "build_plan4_status" complete_green_parked incomplete quarantined
  require_status_value "$(json_string_value "$report" "build_plan5_status")" "build_plan5_status" separate_future_plan absent blocked
  require_status_value "$(json_string_value "$report" "local_rust_tests_status")" "local_rust_tests_status" passed not_performed failed
  require_status_value "$(json_string_value "$report" "anchor_build_status")" "anchor_build_status" passed operator_verified_or_not_performed not_performed failed
  require_status_value "$(json_string_value "$report" "anchor_test_status")" "anchor_test_status" passed operator_verified_or_not_performed not_performed failed
  require_status_value "$(json_string_value "$report" "actual_private_testnet_checks_status")" "actual_private_testnet_checks_status" passed incomplete quarantined
  require_status_value "$(json_string_value "$report" "deploy_receipt_status")" "deploy_receipt_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$report" "test_only_mint_init_status")" "test_only_mint_init_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$report" "live_read_only_rpc_status")" "live_read_only_rpc_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$report" "simulation_receipts_status")" "simulation_receipts_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$report" "capped_send_receipts_status")" "capped_send_receipts_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$report" "readback_receipts_status")" "readback_receipts_status" linked linked_or_not_performed not_performed failed_safe missing quarantined
  require_status_value "$(json_string_value "$report" "negative_drill_failure_receipts_status")" "negative_drill_failure_receipts_status" linked missing quarantined
  require_status_value "$(json_string_value "$report" "halt_recovery_drills_status")" "halt_recovery_drills_status" linked simulated missing quarantined
  require_status_value "$(json_string_value "$report" "authority_drills_status")" "authority_drills_status" linked simulated missing quarantined
  require_status_value "$(json_string_value "$report" "rustyonions_handoff_status")" "rustyonions_handoff_status" dry_run_only linked missing quarantined
  require_status_value "$(json_string_value "$report" "crablink_display_status")" "crablink_display_status" display_only linked not_performed missing quarantined
  require_status_value "$(json_string_value "$report" "tracked_key_material_status")" "tracked_key_material_status" none_tracked failed
  require_status_value "$(json_string_value "$report" "mainnet_behavior_status")" "mainnet_behavior_status" absent failed
  require_status_value "$(json_string_value "$report" "public_launch_status")" "public_launch_status" absent failed
  require_status_value "$(json_string_value "$report" "production_settlement_status")" "production_settlement_status" absent failed
  require_status_value "$(json_string_value "$report" "real_internal_roc_mutation_status")" "real_internal_roc_mutation_status" absent failed
  require_status_value "$(json_string_value "$report" "exchange_staking_liquidity_status")" "exchange_staking_liquidity_status" absent failed
  require_status_value "$(json_string_value "$report" "known_pilot_failures_status")" "known_pilot_failures_status" none_observed_or_documented documented quarantined

  require_json_bool_true "$report" "operator_report_redacted"
  require_json_bool_true "$report" "private_testnet_only"
  require_json_bool_true "$report" "test_only_assets_only"
  require_json_bool_true "$report" "closeout_gate_only"
  require_json_bool_true "$report" "future_build_plan5_required"

  for field in \
    runtime_authorization wallet_authority ledger_authority bridge_authority transaction_submission \
    public_launch_authorized mainnet_authorized production_bridge_settlement public_rox_mint_burn \
    real_roc_burn real_roc_release real_roc_mutation final_settlement finality_claim
  do
    require_json_bool_false_or_absent "$report" "$field"
  done

  for forbidden in \
    '"build_plan5_status": "active_production"' \
    '"closeout_status": "mainnet_ready"' \
    '"closeout_status": "production_ready"' \
    '"rustyonions_handoff_status": "mutated"' \
    '"crablink_display_status": "unlocked"' \
    '"tracked_key_material_status": "tracked"' \
    '"mainnet_behavior_status": "present"' \
    '"public_launch_status": "authorized"' \
    '"production_settlement_status": "present"' \
    '"real_internal_roc_mutation_status": "present"' \
    '"exchange_staking_liquidity_status": "present"' \
    '"settlement": "complete"' \
    '"finality": "finalized"'
  do
    if grep -Fq "$forbidden" "$report"; then
      fail "closeout report contains forbidden success/authority marker: $forbidden"
    fi
    ok "closeout report excludes success/authority marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 15 actual private testnet closeout checks passed
summary:
  - closeout report is devnet/testnet only
  - closeout report is redacted and closeout-gate-only
  - BUILD_PLAN4 is allowed to be complete/green/parked while BUILD_PLAN5 remains separate/future
  - RustyOnions handoff remains dry-run only and CrabLink status remains display-only
  - closeout rejects runtime authorization, wallet authority, ledger authority, bridge authority, transaction submission, public launch, mainnet, production settlement, public ROX mint/burn, real ROC burn/release/mutation, final settlement, exchange/staking/liquidity, and finality claims
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"

  require_valid_cluster "$cluster" >/dev/null
  root="$(cd "$root" && pwd)"

  check_docs "$root"

  [ -f "$root/BUILD_PLAN4.md" ] || fail "BUILD_PLAN4.md missing"
  [ -f "$root/BUILD_PLAN5.md" ] || fail "BUILD_PLAN5.md missing; BUILD_PLAN5 must remain separate/future"
  [ -f "$root/scripts/check_private_pilot_hygiene.sh" ] || fail "private pilot hygiene checker missing"
  [ -f "$root/scripts/check_private_testnet_pilot_closeout.sh" ] || fail "prior private pilot closeout checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_workspace.sh" ] || fail "Phase 1 actual workspace checker missing"
  [ -f "$root/scripts/capture_actual_private_testnet_build_artifacts.sh" ] || fail "Phase 2 build artifact capture script missing"
  [ -f "$root/scripts/check_actual_private_testnet_deploy_receipt.sh" ] || fail "Phase 3 deploy receipt checker missing"
  [ -f "$root/scripts/check_actual_test_only_mint_initialization.sh" ] || fail "Phase 4 test-only mint initialization checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_read_only_evidence.sh" ] || fail "Phase 5 read-only evidence checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_simulation.sh" ] || fail "Phase 6 simulation checker missing"
  [ -f "$root/scripts/check_actual_roc_to_rox_private_testnet_run.sh" ] || fail "Phase 7 ROC-to-ROX checker missing"
  [ -f "$root/scripts/check_actual_rox_to_roc_private_testnet_run.sh" ] || fail "Phase 8 ROX-to-ROC checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_receipts.sh" ] || fail "Phase 9 receipt checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_negative_drills.sh" ] || fail "Phase 10 negative drill checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_halt_recovery_authority_drills.sh" ] || fail "Phase 11 halt/recovery/authority checker missing"
  [ -f "$root/scripts/check_actual_rustyonions_dry_run_handoff.sh" ] || fail "Phase 12 RustyOnions handoff checker missing"
  [ -f "$root/scripts/check_actual_crablink_private_testnet_status.sh" ] || fail "Phase 13 CrabLink status checker missing"
  [ -f "$root/scripts/check_actual_private_testnet_evidence_package.sh" ] || fail "Phase 14 evidence package checker missing"

  if grep -Fq "BUILD_PLAN5" "$root/BUILD_PLAN4.md" \
    && grep -Fq "mainnet" "$root/BUILD_PLAN4.md" \
    && grep -Fq "production bridge settlement" "$root/BUILD_PLAN4.md" \
    && grep -Fq "public ROX mint/burn" "$root/BUILD_PLAN4.md"; then
    ok "BUILD_PLAN4 preserves BUILD_PLAN5 future boundary"
  else
    fail "BUILD_PLAN4 must preserve BUILD_PLAN5 future boundary markers"
  fi

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-closeout.*\.json|actual-build-plan4-closeout.*\.json|.*actual-closeout-report.*\.json)$'; then
      fail "git tracked Phase 15 closeout material found"
    fi
    ok "git tracked-file scan found no Phase 15 closeout material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 15 actual private testnet closeout preflight passed
summary:
  - Phase 15 documentation and ignore boundaries are present
  - Phase 1 through Phase 14 actual private testnet checkers exist
  - BUILD_PLAN5 exists and remains separate/future
  - no tracked Phase 15 closeout material was found
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
  --template-closeout)
    template_closeout "${2:-testnet}"
    ;;
  --check-closeout)
    check_closeout "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
