#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 1 actual private testnet workspace checker.
# RO:WHY — Confirms external-only operator artifacts before real private testnet evidence work begins.
# RO:INTERACTS — .gitignore, docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md, optional git tracked-file view.
# RO:INVARIANTS — keys/RPC/receipts stay external or ignored; docs stay redacted; no mainnet/public/production claims.
# RO:SECURITY — read-only checker; no RPC, wallet load, deploy, submit, mint, burn, settlement, or ROC mutation.
# RO:TEST — bash scripts/check_actual_private_testnet_workspace.sh . and cargo test -p rox-anchor-cli --test actual_private_testnet_workspace.

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
  bash scripts/check_actual_private_testnet_workspace.sh [repo-root]
  bash scripts/check_actual_private_testnet_workspace.sh --checklist
  bash scripts/check_actual_private_testnet_workspace.sh --template
USAGE
}

print_checklist() {
  cat <<'CHECKLIST'
ROX Anchor BUILD_PLAN4 Phase 1 — actual private testnet operator workspace checklist

Safe local preflight:
  1. cargo fmt --all
  2. cargo check --workspace
  3. cargo test --workspace
  4. bash scripts/check_private_pilot_hygiene.sh .
  5. bash scripts/check_private_testnet_pilot_closeout.sh .
  6. bash scripts/check_actual_private_testnet_workspace.sh .

External-only operator workspace:
  7. create <external-private-workspace> outside tracked source
  8. create private-testnet.toml outside tracked source
  9. store payer/program/mint/halt/recovery/upgrade keypairs outside tracked source
 10. store RPC URL and provider token in files outside tracked source
 11. store receipts/audit/deploy outputs outside tracked source or ignored pilot paths
 12. redact all promoted artifacts before committing any evidence document

Forbidden in Phase 1:
  - mainnet-beta
  - public launch claims
  - public ROX mint/burn claims
  - real internal ROC release
  - production bridge settlement
  - committed key material
  - committed tokenized RPC/provider URLs
  - committed unredacted receipts or deploy output
  - fake finality
  - fake success output
CHECKLIST
}

print_template() {
  cat <<'TEMPLATE'
# <external-private-workspace>/private-testnet.toml
# local-only; never commit

cluster = "testnet"
program_id = "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR"

payer_keypair_path = "<external-private-workspace>/keys/testnet-payer.json"
program_keypair_path = "<external-private-workspace>/keys/rox-anchor-program-keypair.json"
mint_authority_path = "<external-private-workspace>/keys/mint-authority.json"
halt_authority_path = "<external-private-workspace>/keys/halt-authority.json"
recovery_authority_path = "<external-private-workspace>/keys/recovery-authority.json"
upgrade_authority_path = "<external-private-workspace>/keys/upgrade-authority.json"

rpc_url_file = "<external-private-workspace>/rpc/rpc-url.pilot-rpc.txt"
provider_token_file = "<external-private-workspace>/rpc/provider-token.pilot-provider.txt"

receipt_dir = "<external-private-workspace>/receipts"
audit_dir = "<external-private-workspace>/audit"

max_test_only_amount_minor = "1000000"
max_operation_count = 1
max_retry_count = 0
require_operator_approval = true
TEMPLATE
}

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
  usage
  exit 0
fi

if [ "${1:-}" = "--checklist" ]; then
  print_checklist
  exit 0
fi

if [ "${1:-}" = "--template" ]; then
  print_template
  exit 0
fi

ROOT="${1:-.}"
ROOT="$(cd "$ROOT" && pwd)"

GITIGNORE="$ROOT/.gitignore"
ACTUAL_DOC="$ROOT/docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"

[ -f "$GITIGNORE" ] || fail ".gitignore not found at $GITIGNORE"
[ -f "$ACTUAL_DOC" ] || fail "actual private testnet workspace doc missing: docs/pilot/ACTUAL_PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"

require_gitignore_literal() {
  local literal="$1"
  local label="$2"

  if grep -Fq "$literal" "$GITIGNORE"; then
    ok ".gitignore covers $label"
  else
    fail ".gitignore missing $label: $literal"
  fi
}

require_contains() {
  local file="$1"
  local needle="$2"
  local label="$3"

  if grep -Fq "$needle" "$file"; then
    ok "$label"
  else
    fail "$label missing expected text: $needle"
  fi
}

reject_contains() {
  local file="$1"
  local needle="$2"
  local label="$3"

  if grep -Fq "$needle" "$file"; then
    fail "$label contains forbidden text: $needle"
  else
    ok "$label"
  fi
}

for required_ignore in \
  ".rox-anchor-pilot/" \
  ".rox-anchor-private-pilot/" \
  "private-pilot/" \
  "pilot-artifacts/" \
  "pilot-rpc/" \
  "pilot-keys/" \
  "pilot-keypairs/" \
  "pilot-wallets/" \
  "pilot-secrets/" \
  "pilot-receipts/" \
  "pilot-audit/" \
  "pilot-deploy/" \
  "pilot-ledger/" \
  "pilot-tmp/" \
  "*.pilot-config.local.toml" \
  "*.pilot-config.local.json" \
  "*.pilot-rpc.txt" \
  "*.pilot-provider.txt" \
  "*.pilot-keypair.json" \
  "*.pilot-wallet.json" \
  "*.pilot-authority.json" \
  "*.pilot-payer.json" \
  "*.pilot-receipt.json" \
  "*.pilot-audit.json" \
  "*.pilot-deploy-output.json" \
  "*.pilot-ledger.json" \
  "private-testnet.toml" \
  "actual-private-testnet.toml" \
  "*.private-testnet.local.toml"
do
  require_gitignore_literal "$required_ignore" "$required_ignore"
done

require_contains "$ACTUAL_DOC" "ROX Anchor BUILD_PLAN4 Phase 1" "phase marker"
require_contains "$ACTUAL_DOC" "external-only / ignored / redacted" "external-only marker"
require_contains "$ACTUAL_DOC" "<external-private-workspace>" "redacted workspace placeholder"
require_contains "$ACTUAL_DOC" "private-testnet.toml" "local config filename marker"
require_contains "$ACTUAL_DOC" "ROX_ANCHOR_ACTUAL_PRIVATE_TESTNET_CONFIG" "actual config env marker"
require_contains "$ACTUAL_DOC" "No public launch authorization." "non-launch marker"
require_contains "$ACTUAL_DOC" "No mainnet-beta deployment." "mainnet non-authorization marker"
require_contains "$ACTUAL_DOC" "No real internal ROC release." "ROC release non-authorization marker"
require_contains "$ACTUAL_DOC" "scripts/check_actual_private_testnet_workspace.sh" "checker reference"

reject_contains "$ACTUAL_DOC" "/Users/" "actual workspace doc redaction"
reject_contains "$ACTUAL_DOC" "/home/" "actual workspace doc redaction"
reject_contains "$ACTUAL_DOC" "api-key=" "actual workspace doc token redaction"
reject_contains "$ACTUAL_DOC" "apikey=" "actual workspace doc token redaction"
reject_contains "$ACTUAL_DOC" "access_token=" "actual workspace doc token redaction"

is_ignored_scan_path() {
  case "$1" in
    */.git/*|*/target/*|*/node_modules/*|*/.anchor/*|*/.anchor-cache/*|*/.anchor-test/*|*/.anchor-testnet/*) return 0 ;;
    */test-ledger/*|*/ledger/*|*/local-ledger/*|*/validator-ledger/*|*/testnet-ledger/*|*/test-validator-ledger/*) return 0 ;;
    */.solana/*|*/solana/*|*/.config/solana/*) return 0 ;;
    */secrets/*|*/private/*|*/credentials/*|*/tokens/*|*/auth/*) return 0 ;;
    */api-keys/*|*/apikeys/*|*/keys/*|*/keypairs/*|*/wallets/*|*/mnemonics/*|*/seeds/*|*/seed/*|*/recovery/*|*/keystore/*) return 0 ;;
    */deploy-drills/*|*/testnet-deploy/*|*/testnet-artifacts/*|*/.rox-anchor-testnet/*) return 0 ;;
    */.rox-anchor-pilot/*|*/.rox-anchor-private-pilot/*|*/private-pilot/*) return 0 ;;
    */pilot-artifacts/*|*/pilot-rpc/*|*/pilot-keys/*|*/pilot-keypairs/*|*/pilot-wallets/*|*/pilot-secrets/*) return 0 ;;
    */pilot-receipts/*|*/pilot-audit/*|*/pilot-deploy/*|*/pilot-ledger/*|*/pilot-tmp/*) return 0 ;;
    */CODEBUNDLE*.md) return 0 ;;
  esac

  return 1
}

is_forbidden_actual_filename() {
  local base="$1"

  case "$base" in
    private-testnet.toml|private-testnet.json|actual-private-testnet.toml|actual-private-testnet.json)
      return 0
      ;;
    authority-notes.local.txt|*.pilot-authority-notes.local.txt)
      return 0
      ;;
    id.json|keypair.json|wallet.json|payer.json|authority.json|mint-authority.json|halt-authority.json|recovery-authority.json|upgrade-authority.json)
      return 0
      ;;
    program-authority.json|deploy-authority.json|admin.json|owner.json|validator-keypair.json|faucet-keypair.json)
      return 0
      ;;
    *-keypair.json|*.keypair.json|*.wallet.json|*.authority.json|*-wallet.json|*-payer.json|*-authority.json)
      return 0
      ;;
    *-mint-authority.json|*-halt-authority.json|*-recovery-authority.json|*-upgrade-authority.json|*-program-authority.json|*-program-keypair.json)
      return 0
      ;;
    *.testnet-keypair.json|*.testnet-wallet.json|*.testnet-authority.json|*.testnet-payer.json)
      return 0
      ;;
    *.pilot-keypair.json|*.pilot-wallet.json|*.pilot-authority.json|*.pilot-payer.json)
      return 0
      ;;
    *.pilot-config.local.toml|*.pilot-config.local.json|*.private-testnet.local.toml|*.private-testnet.local.json)
      return 0
      ;;
    *.actual-private-testnet.local.toml|*.actual-private-testnet.local.json)
      return 0
      ;;
    pilot-rpc-url.txt|pilot-provider-url.txt|*.pilot-rpc.txt|*.pilot-provider.txt)
      return 0
      ;;
    *.pilot-receipt.json|*.pilot-audit.json|*.pilot-deploy-output.json|*.pilot-ledger.json)
      return 0
      ;;
  esac

  return 1
}

is_text_file() {
  local file="$1"
  [ ! -s "$file" ] && return 0
  LC_ALL=C grep -Iq . "$file"
}

is_solana_keypair_json() {
  local file="$1"
  [ -f "$file" ] || return 1

  local compact
  compact="$(tr -d '[:space:]' < "$file" | head -c 4096 || true)"
  printf '%s' "$compact" | grep -Eq '^\[[0-9]{1,3}(,[0-9]{1,3}){63}\]$'
}

contains_raw_rpc_secret_url() {
  local file="$1"
  local token_url_re="https?://[^[:space:]\"'<>]+[?&](api[-_]?key|apikey|token|access[-_]?token|auth|secret)=[A-Za-z0-9._~+/=-]{8,}"

  is_text_file "$file" || return 1

  while IFS= read -r match; do
    lower="$(printf '%s' "$match" | tr '[:upper:]' '[:lower:]')"
    case "$lower" in
      *do-not-print*|*redacted*|*placeholder*|*example-token*|*fake-token*|*dummy-token*|*test-token*|*rpc.example.dev*)
        continue
        ;;
    esac
    return 0
  done < <(LC_ALL=C grep -Eio "$token_url_re" "$file" || true)

  return 1
}

forbidden_files=()
forbidden_content=()

while IFS= read -r -d '' file; do
  is_ignored_scan_path "$file" && continue

  base="${file##*/}"
  rel="${file#"$ROOT"/}"

  if is_forbidden_actual_filename "$base"; then
    forbidden_files+=("$rel")
    continue
  fi

  if is_solana_keypair_json "$file"; then
    forbidden_files+=("$rel")
    continue
  fi

  if contains_raw_rpc_secret_url "$file"; then
    forbidden_content+=("$rel")
  fi
done < <(find "$ROOT" -type f -print0)

if [ "${#forbidden_files[@]}" -gt 0 ]; then
  printf 'Forbidden actual private testnet/key/RPC/receipt files found outside ignored local artifact directories:\n' >&2
  printf '  %s\n' "${forbidden_files[@]}" >&2
  fail "remove, relocate, or ignore actual private testnet operator material"
fi
ok "no forbidden actual private testnet file names found in source paths"

if [ "${#forbidden_content[@]}" -gt 0 ]; then
  printf 'Raw tokenized RPC/provider URLs found in source paths:\n' >&2
  printf '  %s\n' "${forbidden_content[@]}" >&2
  fail "redact RPC/provider URLs before committing"
fi
ok "no raw RPC/provider token URLs found in source paths"

if [ -d "$ROOT/.git" ] && command -v git >/dev/null 2>&1; then
  tracked_forbidden=()

  while IFS= read -r tracked; do
    base="${tracked##*/}"
    if is_forbidden_actual_filename "$base"; then
      tracked_forbidden+=("$tracked")
    fi
  done < <(git -C "$ROOT" ls-files)

  if [ "${#tracked_forbidden[@]}" -gt 0 ]; then
    printf 'Forbidden tracked actual private testnet files found:\n' >&2
    printf '  %s\n' "${tracked_forbidden[@]}" >&2
    fail "git-tracked actual private testnet operator material is forbidden"
  fi

  ok "git tracked-file scan found no actual private testnet key/RPC/receipt material"
else
  ok "git tracked-file scan skipped because git metadata is unavailable"
fi

cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 1 actual private testnet workspace checks passed
summary:
  - external operator workspace shape is documented
  - .gitignore covers actual private testnet config, key, RPC, receipt, deploy, audit, ledger, and tmp artifacts
  - source paths contain no key-shaped actual private testnet files
  - source paths contain no raw tokenized RPC/provider URLs
  - promoted documentation uses redacted placeholders rather than operator-local paths
  - this script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, sign, or load a wallet
SUMMARY
