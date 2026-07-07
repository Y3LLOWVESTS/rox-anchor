#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — Generate a repo-wide ROX Anchor text/codebundle review artifact.
# RO:WHY — One-file share/review artifact for docs, scripts, scaffold, and future placeholders without running builds.
# RO:OUTPUT — Default: ./CODEBUNDLE.md, or the second argument.
# RO:INVARIANTS — stable sort; respects gitignore; no secrets; no local Solana keypairs; source of truth remains repo files.
# RO:SECURITY — skips secret/key-shaped paths, secret-shaped content, ignored local files, keypair-shaped JSON, and generated artifacts.
# RO:TEST — bash -n scripts/make_codebundle.sh && bash scripts/check_codebundle_secret_failsafe.sh.

ROOT="${1:-.}"
OUT="${2:-CODEBUNDLE.md}"

cd "$ROOT"

anchor_id() {
  echo "$1" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9 _.-]+//g; s/[[:space:]]+/-/g; s/-+/-/g; s/^-//; s/-$//'
}

record_skip() {
  local path="$1"
  local reason="$2"
  printf '%s\t%s\n' "$path" "$reason" >> "$SKIPPED"
}

bundle_fail() {
  local msg="$1"
  printf 'ERROR: %s\n' "$msg" >&2
  rm -f "$OUT"
  exit 1
}

is_excluded_path() {
  local p="$1"
  local base
  base="$(basename "$p")"

  case "$p" in
    .git/*|*/.git/*) return 0 ;;
    target/*|*/target/*) return 0 ;;
    node_modules/*|*/node_modules/*) return 0 ;;
    dist/*|*/dist/*|build/*|*/build/*|coverage/*|*/coverage/*) return 0 ;;
    .cache/*|*/.cache/*|.turbo/*|*/.turbo/*) return 0 ;;
    .anchor/*|*/.anchor/*|.anchor-cache/*|*/.anchor-cache/*|.anchor-test/*|*/.anchor-test/*|.anchor-testnet/*|*/.anchor-testnet/*) return 0 ;;
    test-ledger/*|*/test-ledger/*|ledger/*|*/ledger/*|local-ledger/*|*/local-ledger/*|validator-ledger/*|*/validator-ledger/*|testnet-ledger/*|*/testnet-ledger/*|test-validator-ledger/*|*/test-validator-ledger/*) return 0 ;;
    .idea/*|*/.idea/*|.vscode/*|*/.vscode/*) return 0 ;;
    .solana/*|*/.solana/*|solana/*|*/solana/*|.config/solana/*|*/.config/solana/*) return 0 ;;
    secrets/*|*/secrets/*|private/*|*/private/*|credentials/*|*/credentials/*|tokens/*|*/tokens/*|auth/*|*/auth/*) return 0 ;;
    api-keys/*|*/api-keys/*|apikeys/*|*/apikeys/*|keys/*|*/keys/*|keypairs/*|*/keypairs/*|wallets/*|*/wallets/*) return 0 ;;
    mnemonics/*|*/mnemonics/*|seeds/*|*/seeds/*|seed/*|*/seed/*|recovery/*|*/recovery/*|keystore/*|*/keystore/*|key-store/*|*/key-store/*|key_store/*|*/key_store/*) return 0 ;;
    deploy-drills/*|*/deploy-drills/*|testnet-deploy/*|*/testnet-deploy/*|testnet-artifacts/*|*/testnet-artifacts/*|.rox-anchor-testnet/*|*/.rox-anchor-testnet/*) return 0 ;;
    .rox-anchor-pilot/*|*/.rox-anchor-pilot/*|.rox-anchor-private-pilot/*|*/.rox-anchor-private-pilot/*|private-pilot/*|*/private-pilot/*) return 0 ;;
    pilot-artifacts/*|*/pilot-artifacts/*|pilot-rpc/*|*/pilot-rpc/*|pilot-keys/*|*/pilot-keys/*|pilot-keypairs/*|*/pilot-keypairs/*) return 0 ;;
    pilot-wallets/*|*/pilot-wallets/*|pilot-secrets/*|*/pilot-secrets/*|pilot-receipts/*|*/pilot-receipts/*|pilot-audit/*|*/pilot-audit/*) return 0 ;;
    pilot-deploy/*|*/pilot-deploy/*|pilot-ledger/*|*/pilot-ledger/*|pilot-tmp/*|*/pilot-tmp/*) return 0 ;;
  esac

  case "$base" in
    CODEBUNDLE.md|CODEBUNDLE_*.md|CODEBUNDLE*.md|CODEBUNDLE_RS.md|CODEBUNDLE_TAURI_APP.md) return 0 ;;
    NOTES.MD|NOTES.md|SESSION_NOTES.md|SESSION_NOTES.MD) return 0 ;;
    Pasted\ text.txt|Pasted\ markdown.md) return 0 ;;
    .DS_Store|Thumbs.db|Desktop.ini) return 0 ;;
  esac

  case "$p" in
    scripts/make_codebundle.sh)
      # The generator contains detector strings by design; including it creates
      # self-matching false positives and weakens the review artifact.
      return 0
      ;;
  esac

  return 1
}

is_sensitive_path() {
  local p="$1"
  local base
  base="$(basename "$p")"

  case "$base" in
    .env|.env.*|*.env|*.env.local|*.env.*.local|.envrc|.envrc.local) return 0 ;;
    .netrc|.npmrc|.pnpmrc|.yarnrc) return 0 ;;
    *.pem|*.key|*.p12|*.pfx|*.crt|*.csr|*.der) return 0 ;;
    *.secret|*.token|*.apikey|*.api-key|*.credentials|*.creds|*.passwd|*.password) return 0 ;;
    *.mnemonic|*.seed|*.seed.json|*.recovery|*.recovery-phrase|*.priv|*.private|*.private.json|*.auth|*.session|*.cookie|*.cookies) return 0 ;;
    id.json|keypair.json|wallet.json|payer.json|authority.json|mint-authority.json|halt-authority.json|recovery-authority.json|upgrade-authority.json) return 0 ;;
    program-authority.json|deploy-authority.json|admin.json|owner.json|validator-keypair.json|faucet-keypair.json) return 0 ;;
    *-keypair.json|*.keypair.json|*.wallet.json|*.authority.json|*-wallet.json|*-payer.json|*-authority.json) return 0 ;;
    *-mint-authority.json|*-halt-authority.json|*-recovery-authority.json|*-upgrade-authority.json|*-program-authority.json|*-program-keypair.json) return 0 ;;
    local-wallet*.json|local-payer*.json|local-authority*.json|local-keypair*.json|dev-wallet*.json|dev-payer*.json|dev-authority*.json|dev-keypair*.json) return 0 ;;
    testnet-wallet*.json|testnet-payer*.json|testnet-authority*.json|testnet-keypair*.json|devnet-wallet*.json|devnet-payer*.json|devnet-authority*.json|devnet-keypair*.json) return 0 ;;
    rpc-url.txt|rpc-url.local|provider-url.txt|provider-url.local|provider-token.env|provider-token.txt|alchemy*.txt|quicknode*.txt|helius*.txt|triton*.txt|ankr*.txt|infura*.txt) return 0 ;;
    private-testnet.toml|private-testnet.json|actual-private-testnet.toml|actual-private-testnet.json) return 0 ;;
    *.private-testnet.local.toml|*.private-testnet.local.json|*.actual-private-testnet.local.toml|*.actual-private-testnet.local.json) return 0 ;;
    *.pilot-config.local.toml|*.pilot-config.local.json|*.pilot-rpc.txt|*.pilot-provider.txt|*.pilot-keypair.json|*.pilot-wallet.json|*.pilot-authority.json|*.pilot-payer.json) return 0 ;;
    *.pilot-receipt.json|*.pilot-audit.json|*.pilot-deploy-output.json|*.pilot-ledger.json) return 0 ;;
    *.actual-anchor-build.local.json|*.actual-private-testnet-build.local.json|*.actual-build-manifest.local.json|*.anchor-build-manifest.local.json) return 0 ;;
    *.actual-private-testnet-deploy.local.json|*.actual-private-testnet-deploy-receipt.local.json|*.actual-private-testnet-deploy-failed.local.json) return 0 ;;
    *.actual-deploy-receipt.local.json|*.actual-deploy-failed.local.json|*.deploy-attempt.local.json) return 0 ;;
  esac

  case "$p" in
    */target/deploy/*.json|*/programs/*/target/deploy/*.json) return 0 ;;
    */target/deploy/*.so|*/programs/*/target/deploy/*.so) return 0 ;;
    */target/idl/*.json|*/programs/*/target/idl/*.json) return 0 ;;
  esac

  return 1
}

is_text_or_empty() {
  local f="$1"
  [ ! -s "$f" ] && return 0
  LC_ALL=C grep -Iq . "$f"
}

is_solana_keypair_json() {
  local f="$1"
  [ -f "$f" ] || return 1

  local compact
  compact="$(tr -d '[:space:]' < "$f" | head -c 4096 || true)"
  printf '%s' "$compact" | grep -Eq '^\[[0-9]{1,3}(,[0-9]{1,3}){63}\]$'
}

contains_actual_pem_private_key() {
  local f="$1"
  LC_ALL=C grep -Eq -- '-----BEGIN (RSA |EC |DSA |OPENSSH |PGP )?PRIVATE KEY-----|-----BEGIN PRIVATE KEY-----|OPENSSH PRIVATE KEY' "$f"
}

contains_raw_rpc_secret_url() {
  local f="$1"
  local token_url_re='https?://[^[:space:]"'\''<>]+[?&](api[-_]?key|apikey|token|access[-_]?token|auth|secret)=[A-Za-z0-9._~+/=-]{8,}'

  is_text_or_empty "$f" || return 1

  while IFS= read -r match; do
    local lower
    lower="$(printf '%s' "$match" | tr '[:upper:]' '[:lower:]')"

    case "$lower" in
      *redacted*|*placeholder*|*example-token*|*fake-token*|*dummy-token*|*test-token*|*rpc.example*|*invalid*)
        continue
        ;;
    esac

    return 0
  done < <(LC_ALL=C grep -Eio -- "$token_url_re" "$f" || true)

  return 1
}

contains_credential_assignment() {
  local f="$1"
  local assignment_re='(^|[^A-Z0-9_])(AWS_ACCESS_KEY_ID|AWS_SECRET_ACCESS_KEY|AWS_SESSION_TOKEN|GITHUB_TOKEN|GH_TOKEN|OPENAI_API_KEY|HELIUS_API_KEY|ALCHEMY_API_KEY|QUICKNODE_API_KEY|INFURA_API_KEY|PRIVATE_KEY|SECRET_KEY|API_KEY|ACCESS_TOKEN|REFRESH_TOKEN|MNEMONIC|SEED_PHRASE)[[:space:]]*[:=][[:space:]]*["'\''"]?[A-Za-z0-9_./+=:@-]{12,}'

  is_text_or_empty "$f" || return 1

  while IFS= read -r line; do
    local lower
    lower="$(printf '%s' "$line" | tr '[:upper:]' '[:lower:]')"

    case "$lower" in
      *redacted*|*placeholder*|*example*|*dummy*|*fake*|*test-token*|*test_only*|*test-only*|*grep*|*forbidden*|*needle*|*secret/path\ marker*)
        continue
        ;;
      *\<redacted*|*\<placeholder*)
        continue
        ;;
    esac

    return 0
  done < <(LC_ALL=C grep -Ein -- "$assignment_re" "$f" || true)

  return 1
}

contains_secret_content() {
  local f="$1"

  if contains_actual_pem_private_key "$f"; then
    return 0
  fi

  if is_solana_keypair_json "$f"; then
    return 0
  fi

  if contains_raw_rpc_secret_url "$f"; then
    return 0
  fi

  if contains_credential_assignment "$f"; then
    return 0
  fi

  return 1
}

lang_for_file() {
  local f="$1"
  case "$f" in
    *.rs) echo "rust" ;;
    *.toml) echo "toml" ;;
    *.md|*.MD) echo "markdown" ;;
    *.sh) echo "bash" ;;
    *.json) echo "json" ;;
    *.yml|*.yaml) echo "yaml" ;;
    *) echo "text" ;;
  esac
}

FILES="$(mktemp)"
RAW_FILES="$(mktemp)"
SKIPPED="$(mktemp)"
trap 'rm -f "$FILES" "$RAW_FILES" "$SKIPPED"' EXIT

: > "$FILES"
: > "$SKIPPED"

if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  git ls-files -co --exclude-standard | LC_ALL=C sort > "$RAW_FILES"
else
  find . -type f | sed 's#^\./##' | LC_ALL=C sort > "$RAW_FILES"
fi

while IFS= read -r f; do
  [ -z "$f" ] && continue

  if is_excluded_path "$f"; then
    record_skip "$f" "excluded-path"
    continue
  fi

  if is_sensitive_path "$f"; then
    record_skip "$f" "sensitive-path"
    continue
  fi

  if ! is_text_or_empty "$f"; then
    record_skip "$f" "non-text"
    continue
  fi

  if contains_secret_content "$f"; then
    record_skip "$f" "secret-content"
    continue
  fi

  echo "$f" >> "$FILES"
done < "$RAW_FILES"

{
  echo "<!-- Generated by scripts/make_codebundle.sh on $(date -u +%Y-%m-%dT%H:%M:%SZ) -->"
  echo "# Code Bundle — ROX Anchor"
  echo
  echo "> Generated for review/sharing. Source of truth remains the repo."
  echo "> Includes tracked/unignored text files and empty placeholders after secret screening."
  echo "> Excludes secrets, ignored local metadata, build outputs, dependency folders, local Solana keys, local pilot artifacts, and generated codebundles."
  echo "> This artifact is not runtime authorization."
  echo
  echo "- Root: \`$(pwd)\`"
  if git rev-parse --short HEAD >/dev/null 2>&1; then
    echo "- Git: \`$(git rev-parse --short HEAD)$(git diff --quiet || echo ' (dirty)')\`"
  fi
  echo "- Files: \`$(wc -l < "$FILES" | tr -d ' ')\`"
  echo
  echo "## Non-Authorization Notice"
  echo
  echo "This codebundle does not authorize ROX runtime, Solana runtime, bridge runtime, staking, liquidity, exchange-facing logic, or external settlement."
  echo
  echo "## File List"
  echo
  echo '```text'
  cat "$FILES"
  echo '```'
  echo
  echo "## Skipped Files"
  echo
  if [ -s "$SKIPPED" ]; then
    echo '```text'
    cat "$SKIPPED"
    echo '```'
  else
    echo "None."
  fi
  echo
  echo "## Table of Contents"
  while IFS= read -r f; do
    echo "- [$f](#$(anchor_id "$f"))"
  done < "$FILES"
  echo

  while IFS= read -r f; do
    echo "---"
    echo
    echo "## $f"
    echo
    echo "<a id=\"$(anchor_id "$f")\"></a>"
    echo
    if [ ! -s "$f" ]; then
      echo '```text'
      echo "(empty placeholder file)"
      echo '```'
      echo
      continue
    fi

    lang="$(lang_for_file "$f")"

    echo "\`\`\`$lang"
    cat "$f"
    echo
    echo '```'
    echo
  done < "$FILES"
} > "$OUT"

echo "== Secret failsafe scan on generated bundle =="

# Reject actual included sensitive sections or TOC entries.
if grep -nE '^## \.solana/|^- \[\.solana/|^## .*keypairs?/|^- \[.*keypairs?/|^## .*wallets?/|^- \[.*wallets?/' "$OUT"; then
  bundle_fail "generated codebundle includes a sensitive path section or TOC entry."
fi

# Reject real PEM/private-key blocks. This intentionally requires PEM block
# delimiters so scanner source strings such as "BEGIN PRIVATE KEY" do not
# create self-matching false positives.
if grep -nE -- '-----BEGIN (RSA |EC |DSA |OPENSSH |PGP )?PRIVATE KEY-----|-----BEGIN PRIVATE KEY-----|OPENSSH PRIVATE KEY' "$OUT"; then
  bundle_fail "generated codebundle contains private key material."
fi

# Reject Solana keypairs: JSON arrays of exactly 64 u8 integers.
if tr -d '[:space:]' < "$OUT" | grep -Eq '\[[0-9]{1,3}(,[0-9]{1,3}){63}\]'; then
  bundle_fail "generated codebundle appears to contain a Solana 64-byte keypair array."
fi

# Reject tokenized RPC/provider URLs.
if LC_ALL=C grep -nEio -- 'https?://[^[:space:]"'\''<>]+[?&](api[-_]?key|apikey|token|access[-_]?token|auth|secret)=[A-Za-z0-9._~+/=-]{8,}' "$OUT" \
  | grep -Eiv 'redacted|placeholder|example-token|fake-token|dummy-token|test-token|rpc.example|invalid'; then
  bundle_fail "generated codebundle contains a tokenized RPC/provider URL."
fi

# Reject likely credential assignments, excluding explicit examples/placeholders.
if LC_ALL=C grep -nEi -- '(^|[^A-Z0-9_])(AWS_ACCESS_KEY_ID|AWS_SECRET_ACCESS_KEY|AWS_SESSION_TOKEN|GITHUB_TOKEN|GH_TOKEN|OPENAI_API_KEY|HELIUS_API_KEY|ALCHEMY_API_KEY|QUICKNODE_API_KEY|INFURA_API_KEY|PRIVATE_KEY|SECRET_KEY|API_KEY|ACCESS_TOKEN|REFRESH_TOKEN|MNEMONIC|SEED_PHRASE)[[:space:]]*[:=][[:space:]]*["'\''"]?[A-Za-z0-9_./+=:@-]{12,}' "$OUT" \
  | grep -Eiv 'redacted|placeholder|example|dummy|fake|test-token|test_only|test-only|grep|forbidden|needle|secret/path marker'; then
  bundle_fail "generated codebundle contains a credential-like assignment."
fi

echo "Wrote safe codebundle: $OUT"
