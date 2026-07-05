#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — Phase 7 local/testnet deployment drill safety checker.
# RO:WHY — Confirms Anchor testnet drill shape without deploying, loading keys, or touching value.
# RO:INTERACTS — Anchor.toml, .gitignore, local filesystem, optional git tracked-file view.
# RO:INVARIANTS — no mainnet-beta; no committed keypairs; deploy keys remain external/ignored.
# RO:SECURITY — read-only checker; no RPC, wallet load, deploy, mint, burn, settlement, or submission.
# RO:TEST — bash scripts/check_testnet_deploy_drill.sh . and cargo test -p rox-anchor-cli --test testnet_deploy_drill_script.

print_checklist() {
  cat <<'CHECKLIST'
ROX Anchor Phase 7 — testnet deployment drill checklist

Safe preflight:
  1. cargo check --workspace
  2. cargo test --workspace
  3. anchor build
  4. anchor test
  5. bash scripts/check_testnet_deploy_drill.sh .

Local-only inspection:
  6. anchor keys list
  7. anchor idl parse programs/rox-anchor/src/lib.rs >/tmp/rox-anchor-idl.json

Optional testnet-only drill, only after explicit operator decision:
  8. export ROX_ANCHOR_TESTNET_WALLET=/external/non-repo/path/testnet-payer.json
  9. export ROX_ANCHOR_TESTNET_PROGRAM_KEYPAIR=/external/non-repo/path/rox-anchor-program-keypair.json
 10. anchor build
 11. anchor deploy --provider.cluster testnet --provider.wallet "$ROX_ANCHOR_TESTNET_WALLET"

Forbidden in this drill:
  - mainnet-beta deployment
  - production ROX minting or burning
  - production ROC release
  - live value movement with real user funds
  - committed operator keys
  - fake finality or fake success output
CHECKLIST
}

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
  bash scripts/check_testnet_deploy_drill.sh [repo-root]
  bash scripts/check_testnet_deploy_drill.sh --checklist
USAGE
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
ANCHOR_TOML="$ROOT/Anchor.toml"
GITIGNORE="$ROOT/.gitignore"

[ -f "$ANCHOR_TOML" ] || fail "Anchor.toml not found at $ANCHOR_TOML"
[ -f "$GITIGNORE" ] || fail ".gitignore not found at $GITIGNORE"

if grep -Eiq '^[[:space:]]*\[programs\.(mainnet|mainnet-beta|mainnetbeta)\][[:space:]]*$' "$ANCHOR_TOML"; then
  fail "Anchor.toml must not contain a mainnet/mainnet-beta programs section"
fi

if grep -Eiq '^[[:space:]]*cluster[[:space:]]*=[[:space:]]*"(mainnet|mainnet-beta|mainnetbeta)"[[:space:]]*$' "$ANCHOR_TOML"; then
  fail "Anchor.toml provider.cluster must not point at mainnet/mainnet-beta"
fi

if grep -Eiq '^[[:space:]]*wallet[[:space:]]*=[[:space:]]*"[^"]*(mainnet|production|prod)[^"]*\.json"[[:space:]]*$' "$ANCHOR_TOML"; then
  fail "Anchor.toml provider.wallet must not look like a production/mainnet wallet path"
fi

extract_program_id() {
  local cluster="$1"
  awk -v section="programs.${cluster}" '
    $0 ~ "^[[:space:]]*\\[" section "\\][[:space:]]*$" { in_section=1; next }
    in_section && $0 ~ "^[[:space:]]*\\[" { exit }
    in_section && $0 ~ "^[[:space:]]*rox_anchor[[:space:]]*=" { print; exit }
  ' "$ANCHOR_TOML" | sed -E 's/.*=[[:space:]]*"([^"]+)".*/\1/'
}

require_program_id() {
  local cluster="$1"
  local program_id
  program_id="$(extract_program_id "$cluster")"

  [ -n "$program_id" ] || fail "missing [programs.$cluster] rox_anchor program id"

  case "$program_id" in
    REPLACE_ME|TODO|placeholder|11111111111111111111111111111111)
      fail "[programs.$cluster] rox_anchor program id is a placeholder"
      ;;
  esac

  if ! printf '%s' "$program_id" | grep -Eq '^[1-9A-HJ-NP-Za-km-z]{32,44}$'; then
    fail "[programs.$cluster] rox_anchor program id is not a plausible Solana pubkey: $program_id"
  fi

  ok "[programs.$cluster] rox_anchor program id is present"
}

require_program_id "localnet"
require_program_id "devnet"
require_program_id "testnet"

wallet_path="$(
  awk '
    /^[[:space:]]*\[provider\][[:space:]]*$/ { in_provider=1; next }
    in_provider && /^[[:space:]]*\[/ { exit }
    in_provider && /^[[:space:]]*wallet[[:space:]]*=/ { print; exit }
  ' "$ANCHOR_TOML" | sed -E 's/.*=[[:space:]]*"([^"]+)".*/\1/'
)"

[ -n "$wallet_path" ] || fail "Anchor.toml [provider] wallet path is missing"

case "$wallet_path" in
  *.json) ok "provider wallet path is JSON-shaped and must stay ignored/external: $wallet_path" ;;
  *) fail "provider wallet path must be JSON-shaped so local key material is obvious and ignorable" ;;
esac

require_gitignore_pattern() {
  local pattern="$1"
  local label="$2"

  if grep -Eq "$pattern" "$GITIGNORE"; then
    ok ".gitignore covers $label"
  else
    fail ".gitignore is missing coverage for $label"
  fi
}

require_gitignore_pattern '(^|/)\.solana/' '.solana local key directory'
require_gitignore_pattern '(^|/)target/deploy/' 'Anchor target/deploy key output'
require_gitignore_pattern '\*-keypair\.json|[.][*]keypair[.]json|[*][.]keypair[.]json' 'generic keypair JSON files'
require_gitignore_pattern 'deploy-drills/' 'deployment drill artifact directory'
require_gitignore_pattern 'testnet-deploy/' 'testnet deployment artifact directory'
require_gitignore_pattern '[*][.]testnet-keypair[.]json' 'testnet keypair JSON files'

is_ignored_scan_path() {
  case "$1" in
    */.git/*|*/target/*|*/node_modules/*|*/.anchor/*|*/.anchor-cache/*|*/.anchor-test/*|*/.anchor-testnet/*) return 0 ;;
    */test-ledger/*|*/ledger/*|*/local-ledger/*|*/validator-ledger/*|*/testnet-ledger/*|*/test-validator-ledger/*) return 0 ;;
    */.solana/*|*/solana/*|*/.config/solana/*) return 0 ;;
    */secrets/*|*/private/*|*/credentials/*|*/tokens/*|*/auth/*) return 0 ;;
    */keys/*|*/keypairs/*|*/wallets/*|*/mnemonics/*|*/seeds/*|*/seed/*|*/recovery/*|*/keystore/*) return 0 ;;
    */deploy-drills/*|*/testnet-deploy/*|*/testnet-artifacts/*|*/.rox-anchor-testnet/*) return 0 ;;
    */CODEBUNDLE*.md) return 0 ;;
  esac

  return 1
}

is_forbidden_key_filename() {
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
  esac

  return 1
}

forbidden_files=()

while IFS= read -r -d '' file; do
  is_ignored_scan_path "$file" && continue

  base="${file##*/}"
  if is_forbidden_key_filename "$base"; then
    rel="${file#"$ROOT"/}"
    forbidden_files+=("$rel")
  fi
done < <(find "$ROOT" -type f -print0)

if [ "${#forbidden_files[@]}" -gt 0 ]; then
  printf 'Forbidden key-shaped files found outside ignored local artifact directories:\n' >&2
  printf '  %s\n' "${forbidden_files[@]}" >&2
  fail "remove, relocate, or ignore local key material before deployment drill"
fi

ok "no forbidden key-shaped files found outside ignored local artifact directories"

if command -v git >/dev/null 2>&1 && git -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  tracked_forbidden=()

  while IFS= read -r tracked; do
    base="${tracked##*/}"
    if is_forbidden_key_filename "$base"; then
      tracked_forbidden+=("$tracked")
    fi
  done < <(git -C "$ROOT" ls-files)

  if [ "${#tracked_forbidden[@]}" -gt 0 ]; then
    printf 'Forbidden tracked key-shaped files found:\n' >&2
    printf '  %s\n' "${tracked_forbidden[@]}" >&2
    fail "git-tracked key material is forbidden"
  fi

  ok "git tracked-file scan found no key-shaped files"
else
  ok "git tracked-file scan skipped because git metadata is unavailable"
fi

cat <<'SUMMARY'
ok: Phase 7 testnet deployment drill safety checks passed
summary:
  - Anchor.toml has localnet/devnet/testnet program id entries
  - provider cluster is not mainnet-beta
  - provider wallet path is local/external JSON-shaped material
  - .gitignore covers Solana/Anchor key and deploy artifacts
  - forbidden key-shaped files were not found in tracked/source paths
  - this script did not deploy, submit, mint, burn, settle, or load a wallet
SUMMARY
