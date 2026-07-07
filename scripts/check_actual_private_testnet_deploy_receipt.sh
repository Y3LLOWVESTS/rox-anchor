#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 3 actual private devnet/testnet deployment receipt checker.
# RO:WHY — Validates redacted deploy/safe-failure receipts without performing deployment or readback.
# RO:INTERACTS — docs/pilot/ACTUAL_PRIVATE_TESTNET_DEPLOYMENT.md, .gitignore, local ignored receipts.
# RO:INVARIANTS — devnet/testnet only; manual deploy only; no public/mainnet/finality/production/real ROC claims.
# RO:SECURITY — no RPC, wallet load, signing, deploy, submit, mint, burn, settlement, or ROC mutation.
# RO:TEST — cargo test -p rox-anchor-cli --test actual_private_testnet_deploy_receipt.

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
  bash scripts/check_actual_private_testnet_deploy_receipt.sh --check-docs [repo-root]
  bash scripts/check_actual_private_testnet_deploy_receipt.sh --preflight [repo-root] [devnet|testnet]
  bash scripts/check_actual_private_testnet_deploy_receipt.sh --template-success [devnet|testnet]
  bash scripts/check_actual_private_testnet_deploy_receipt.sh --template-failure [devnet|testnet]
  bash scripts/check_actual_private_testnet_deploy_receipt.sh --check-receipt <receipt-json>
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

is_hex64() {
  printf '%s' "$1" | grep -Eq '^[0-9a-f]{64}$'
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

check_doc_file() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_DEPLOYMENT.md"
  local script="$root/scripts/check_actual_private_testnet_deploy_receipt.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_DEPLOYMENT.md"
  [ -f "$script" ] || fail "missing scripts/check_actual_private_testnet_deploy_receipt.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 3" \
    "Actual Private Devnet/Testnet Deployment" \
    "anchor deploy" \
    "--provider.cluster testnet" \
    "--provider.wallet /external/private/<redacted-external-payer-file>" \
    "rox-anchor.actual-private-testnet-deploy-receipt.v1" \
    "deployment_outcome" \
    "deploy_signature" \
    "deploy_slot" \
    "program_binary_sha256" \
    "idl_sha256" \
    "failure_reason_redacted" \
    "program_account_readback_verified" \
    "public_launch_authorized" \
    "mainnet_authorized" \
    "real_roc_mutation" \
    "No public launch authorization." \
    "No mainnet-beta authorization." \
    "No real internal ROC release."
  do
    grep -Fq -- "$needle" "$doc" || fail "deployment doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-deploy-receipt.json" \
    "actual-private-testnet-deploy-receipt.local.json" \
    "actual-private-testnet-deploy-failed.local.json" \
    "*.actual-private-testnet-deploy.local.json" \
    "*.actual-private-testnet-deploy-receipt.local.json" \
    "*.actual-deploy-receipt.local.json" \
    "*.deploy-attempt.local.json"
  do
    grep -Fq -- "$ignored" "$gitignore" || fail ".gitignore missing deploy receipt ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  reject_sensitive_text "$doc" "deployment doc"

  for forbidden in \
    "finality_claim\": true" \
    "runtime_authority\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true" \
    "production_bridge_settlement\": true" \
    "public_rox_mint_burn\": true" \
    "real_roc_mutation\": true" \
    "deployment_success_claim_scope\": \"production\"" \
    "deployment_success_claim_scope\": \"mainnet\""
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "deployment doc contains forbidden claim marker: $forbidden"
    fi
    ok "doc excludes forbidden claim marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 3 deployment documentation checks passed
summary:
  - actual private devnet/testnet deployment runbook exists
  - local deploy receipt artifact names are ignored
  - documentation preserves manual-only, external-key-only, non-mainnet boundaries
  - documentation separates deployment receipt evidence from readback/finality/settlement evidence
SUMMARY
}

print_receipt_template() {
  local outcome="$1"
  local cluster="${2:-testnet}"
  require_valid_cluster "$cluster"

  local signature="none"
  local slot="none"
  local failure="not_performed"
  local scope="none"

  if [ "$outcome" = "succeeded" ]; then
    signature="<redacted-signature>"
    slot="0"
    failure="not_applicable"
    scope="private_devnet_testnet_only"
  fi

  cat <<TEMPLATE
{
  "schema": "rox-anchor.actual-private-testnet-deploy-receipt.v1",
  "phase": "BUILD_PLAN4 Phase 3",
  "receipt_role": "private_testnet_deployment_receipt",
  "cluster": "$cluster",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "deployment_outcome": "$outcome",
  "deploy_signature": "$signature",
  "deploy_slot": "$slot",
  "program_binary_sha256": "<sha256>",
  "idl_sha256": "<sha256>",
  "build_manifest_path": "<redacted-local-build-manifest>",
  "payer_redacted": "<redacted-external-payer>",
  "deploy_authority_redacted": "<redacted-external-deploy-authority>",
  "upgrade_authority_policy": "separated_external_upgrade_authority",
  "failure_reason_redacted": "$failure",
  "deploy_command_was_manual": true,
  "preflight_passed": true,
  "program_account_readback_verified": false,
  "idl_account_readback_verified": false,
  "deployment_success_claim_scope": "$scope",
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "production_bridge_settlement": false,
  "public_rox_mint_burn": false,
  "real_roc_mutation": false
}
TEMPLATE
}

check_receipt() {
  local receipt="${1:-}"
  [ -n "$receipt" ] || fail "--check-receipt requires a receipt path"
  [ -f "$receipt" ] || fail "receipt not found: $receipt"

  reject_sensitive_text "$receipt" "deploy receipt"

  require_json_string "$receipt" "schema" "rox-anchor.actual-private-testnet-deploy-receipt.v1"
  require_json_string "$receipt" "phase" "BUILD_PLAN4 Phase 3"
  require_json_string "$receipt" "receipt_role" "private_testnet_deployment_receipt"
  require_json_string "$receipt" "program_name" "rox_anchor"

  local cluster
  cluster="$(json_string_value "$receipt" "cluster")"
  require_valid_cluster "$cluster"
  ok "receipt cluster is $cluster"

  local program_id
  program_id="$(json_string_value "$receipt" "program_id")"
  [ -n "$program_id" ] || fail "receipt missing program_id"
  case "$program_id" in
    *[[:space:]]*|mainnet*|production*|public*) fail "receipt has invalid program_id: $program_id" ;;
  esac
  ok "receipt has non-empty program_id"

  local outcome
  outcome="$(json_string_value "$receipt" "deployment_outcome")"
  case "$outcome" in
    succeeded|failed) ok "receipt deployment_outcome = $outcome" ;;
    *) fail "deployment_outcome must be succeeded or failed, got: ${outcome:-<missing>}" ;;
  esac

  local program_hash idl_hash
  program_hash="$(json_string_value "$receipt" "program_binary_sha256")"
  idl_hash="$(json_string_value "$receipt" "idl_sha256")"

  is_hex64 "$program_hash" || fail "program_binary_sha256 must be 64 lowercase hex characters"
  is_hex64 "$idl_hash" || fail "idl_sha256 must be 64 lowercase hex characters"
  ok "receipt hashes are lowercase SHA-256 hex"

  require_json_string_present "$receipt" "build_manifest_path"
  require_json_string_present "$receipt" "payer_redacted"
  require_json_string_present "$receipt" "deploy_authority_redacted"
  require_json_string_present "$receipt" "upgrade_authority_policy"
  require_json_string_present "$receipt" "failure_reason_redacted"
  require_json_string_present "$receipt" "deploy_signature"
  require_json_string_present "$receipt" "deploy_slot"
  require_json_string_present "$receipt" "deployment_success_claim_scope"

  case "$(json_string_value "$receipt" "payer_redacted")" in
    *redacted*|"<redacted-"*) ok "payer is redacted" ;;
    *) fail "payer_redacted must be a redacted placeholder" ;;
  esac

  case "$(json_string_value "$receipt" "deploy_authority_redacted")" in
    *redacted*|"<redacted-"*) ok "deploy authority is redacted" ;;
    *) fail "deploy_authority_redacted must be a redacted placeholder" ;;
  esac

  if [ "$outcome" = "succeeded" ]; then
    [ "$(json_string_value "$receipt" "deploy_signature")" != "none" ] || fail "succeeded receipt requires deploy_signature"
    [ "$(json_string_value "$receipt" "deploy_slot")" != "none" ] || fail "succeeded receipt requires deploy_slot"
    require_json_string "$receipt" "deployment_success_claim_scope" "private_devnet_testnet_only"
  else
    require_json_string "$receipt" "deployment_success_claim_scope" "none"
  fi

  require_json_bool_false_or_absent "$receipt" "program_account_readback_verified"
  require_json_bool_false_or_absent "$receipt" "idl_account_readback_verified"
  require_json_bool_false_or_absent "$receipt" "finality_claim"
  require_json_bool_false_or_absent "$receipt" "runtime_authority"
  require_json_bool_false_or_absent "$receipt" "public_launch_authorized"
  require_json_bool_false_or_absent "$receipt" "mainnet_authorized"
  require_json_bool_false_or_absent "$receipt" "production_bridge_settlement"
  require_json_bool_false_or_absent "$receipt" "public_rox_mint_burn"
  require_json_bool_false_or_absent "$receipt" "real_roc_mutation"

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 3 deploy receipt checks passed
summary:
  - receipt is devnet/testnet only
  - receipt records succeeded or failed deployment attempt evidence
  - payer and deploy authority are redacted
  - program and IDL hashes are concrete lowercase SHA-256 values
  - receipt does not claim readback, finality, runtime authority, public launch, mainnet, production settlement, public ROX mint/burn, or real ROC mutation
SUMMARY
}

preflight() {
  local root="${1:-.}"
  local cluster="${2:-testnet}"

  require_valid_cluster "$cluster"

  root="$(cd "$root" && pwd)"

  check_doc_file "$root"

  [ -f "$root/Anchor.toml" ] || fail "Anchor.toml missing"
  [ -f "$root/target/deploy/rox_anchor.so" ] || fail "target/deploy/rox_anchor.so missing; run anchor build first"
  [ -f "$root/target/idl/rox_anchor.json" ] || fail "target/idl/rox_anchor.json missing; run anchor build first"

  if [ -d "$root/.git" ] && command -v git >/dev/null 2>&1; then
    if git -C "$root" ls-files | grep -Eq '(^|/)(actual-private-testnet-deploy.*\.json|.*actual-private-testnet-deploy.*\.json|.*deploy-attempt\.local\.json)$'; then
      fail "git tracked deploy receipt material found"
    fi
    ok "git tracked-file scan found no actual deploy receipt material"
  else
    ok "git tracked-file scan skipped because git metadata is unavailable"
  fi

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 3 deployment preflight passed
summary:
  - deployment documentation and ignore boundaries are present
  - Anchor build outputs exist
  - no tracked deploy receipt material was found
  - this preflight did not deploy, submit, call RPC, sign, mint, burn, settle, mutate ROC, or load a wallet
SUMMARY
}

case "${1:-}" in
  --help|-h)
    usage
    ;;
  --check-docs)
    check_doc_file "${2:-.}"
    ;;
  --preflight)
    preflight "${2:-.}" "${3:-testnet}"
    ;;
  --template-success)
    print_receipt_template "succeeded" "${2:-testnet}"
    ;;
  --template-failure)
    print_receipt_template "failed" "${2:-testnet}"
    ;;
  --check-receipt)
    check_receipt "${2:-}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
