#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN3 Phase 1 private pilot workspace hygiene checker.
# RO:WHY — Keeps private pilot keys, RPC URLs, receipts, and deployment outputs out of tracked source.
# RO:INTERACTS — .gitignore, docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md, optional git tracked-file view.
# RO:INVARIANTS — pilot artifacts remain local/external; no keypairs, raw provider tokens, or fake success claims.
# RO:SECURITY — read-only checker; no RPC, wallet load, deploy, submit, mint, burn, settlement, or key parsing.
# RO:TEST — bash scripts/check_private_pilot_hygiene.sh . and cargo test -p rox-anchor-cli --test private_pilot_hygiene.

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
  bash scripts/check_private_pilot_hygiene.sh [repo-root]
  bash scripts/check_private_pilot_hygiene.sh --checklist
USAGE
}

print_checklist() {
  cat <<'CHECKLIST'
ROX Anchor BUILD_PLAN3 Phase 1 — private pilot operator workspace checklist

Local-only workspace, outside tracked source by default:
  .rox-anchor-pilot/
    keys/               external non-committed testnet keypairs only
    rpc/                RPC/provider URLs and tokens only
    deploy/             local deploy outputs and program artifact captures
    receipts/           pilot receipts, signatures, and run evidence
    audit/              redacted local audit bundles before promotion
    tmp/                scratch material safe to delete

Safe operator rules:
  - keep RPC URLs and provider tokens out of tracked source
  - keep all keypairs, wallets, payers, authorities, mint authorities, and upgrade authorities out of tracked source
  - keep pilot receipts local until they are redacted and intentionally promoted
  - use explicit operator approval for every future live send path
  - distinguish simulation from any capped private testnet submission

Forbidden from this hygiene phase:
  - mainnet-beta deployment
  - public ROX minting or burning
  - production bridge settlement
  - real internal ROC release
  - silent wallet/key usage
  - raw private keys or provider tokens in logs/docs/source
  - fake finality or fake success output
CHECKLIST
}

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
  usage
  exit 0
fi

if [ "${1:-}" = "--checklist" ]; then
  print_checklist
  exit 0
fi

ROOT="${1:-.}"
ROOT="$(cd "$ROOT" && pwd)"
GITIGNORE="$ROOT/.gitignore"
DOC="$ROOT/docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"

[ -f "$GITIGNORE" ] || fail ".gitignore not found at $GITIGNORE"
[ -f "$DOC" ] || fail "pilot workspace doc missing: docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"

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

require_gitignore_literal ".rox-anchor-pilot/" "private pilot local workspace root"
require_gitignore_literal "pilot-rpc/" "private pilot RPC/provider material"
require_gitignore_literal "pilot-keys/" "private pilot key material"
require_gitignore_literal "pilot-receipts/" "private pilot receipt material"
require_gitignore_literal "pilot-deploy/" "private pilot deployment outputs"
require_gitignore_literal "pilot-artifacts/" "private pilot generated artifacts"
require_gitignore_literal "pilot-ledger/" "private pilot local ledger output"
require_gitignore_literal "*.pilot-keypair.json" "pilot keypair JSON files"
require_gitignore_literal "*.pilot-rpc.txt" "pilot RPC URL files"
require_gitignore_literal "*.pilot-receipt.json" "pilot receipt JSON files"

require_contains "$DOC" "ROX Anchor BUILD_PLAN3 Phase 1" "phase marker"
require_contains "$DOC" "No public launch authorization." "non-launch marker"
require_contains "$DOC" "No mainnet-beta deployment." "mainnet non-authorization marker"
require_contains "$DOC" "No real internal ROC release." "ROC release non-authorization marker"
require_contains "$DOC" "scripts/check_private_pilot_hygiene.sh" "checker reference"
require_contains "$DOC" "local-only / ignored / external" "local-only artifact marker"

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
    */pilot-receipts/*|*/pilot-audit/*|*/pilot-deploy/*|*/pilot-ledger/*) return 0 ;;
    */CODEBUNDLE*.md) return 0 ;;
  esac

  return 1
}

is_forbidden_pilot_filename() {
  local base="$1"

  case "$base" in
    id.json|keypair.json|wallet.json|payer.json|authority.json|mint-authority.json|upgrade-authority.json)
      return 0
      ;;
    program-authority.json|deploy-authority.json|admin.json|owner.json|validator-keypair.json|faucet-keypair.json)
      return 0
      ;;
    *-keypair.json|*.keypair.json|*.wallet.json|*.authority.json|*-wallet.json|*-payer.json|*-authority.json)
      return 0
      ;;
    *-mint-authority.json|*-upgrade-authority.json|*-program-authority.json|*-program-keypair.json)
      return 0
      ;;
    *.testnet-keypair.json|*.testnet-wallet.json|*.testnet-authority.json|*.testnet-payer.json)
      return 0
      ;;
    *.pilot-keypair.json|*.pilot-wallet.json|*.pilot-authority.json|*.pilot-payer.json)
      return 0
      ;;
    pilot-rpc-url.txt|pilot-provider-url.txt|*.pilot-rpc.txt|*.pilot-provider.txt)
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

  if is_forbidden_pilot_filename "$base"; then
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
  printf 'Forbidden pilot/key-shaped files found outside ignored local artifact directories:\n' >&2
  printf '  %s\n' "${forbidden_files[@]}" >&2
  fail "remove, relocate, or ignore private pilot key/RPC material"
fi
ok "no forbidden pilot/key-shaped files found outside ignored local artifact directories"

if [ "${#forbidden_content[@]}" -gt 0 ]; then
  printf 'Forbidden raw RPC/provider token URL content found in tracked/source paths:\n' >&2
  printf '  %s\n' "${forbidden_content[@]}" >&2
  fail "redact RPC/provider URLs before committing"
fi
ok "no raw RPC/provider token URLs found in source paths"

if command -v git >/dev/null 2>&1 && git -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  tracked_forbidden=()

  while IFS= read -r tracked; do
    base="${tracked##*/}"
    if is_forbidden_pilot_filename "$base"; then
      tracked_forbidden+=("$tracked")
    fi
  done < <(git -C "$ROOT" ls-files)

  if [ "${#tracked_forbidden[@]}" -gt 0 ]; then
    printf 'Forbidden tracked pilot/key-shaped files found:\n' >&2
    printf '  %s\n' "${tracked_forbidden[@]}" >&2
    fail "git-tracked pilot key/RPC material is forbidden"
  fi

  ok "git tracked-file scan found no pilot key/RPC material"
else
  ok "git tracked-file scan skipped because git metadata is unavailable"
fi

cat <<'SUMMARY'
ok: BUILD_PLAN3 Phase 1 private pilot hygiene checks passed
summary:
  - private pilot local workspace layout is documented
  - .gitignore covers pilot keys, RPC files, receipts, deploy artifacts, and local ledgers
  - source paths contain no key-shaped pilot files
  - source paths contain no raw RPC/provider token URLs
  - this script did not deploy, submit, mint, burn, settle, call RPC, or load a wallet
SUMMARY
