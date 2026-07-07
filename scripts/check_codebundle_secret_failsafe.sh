#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — Focused test for scripts/make_codebundle.sh secret-screening behavior.
# RO:WHY — Proves codebundle generation skips secret-shaped paths/content before producing a shareable artifact.
# RO:INTERACTS — scripts/make_codebundle.sh.
# RO:INVARIANTS — generated bundle must exclude PEM blocks, Solana keypair arrays, tokenized URLs, and credential assignments.
# RO:SECURITY — uses synthetic temp fixtures only; no real secrets, no RPC, no wallet load.
# RO:TEST — bash scripts/check_codebundle_secret_failsafe.sh.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
TMP_STDOUT="$(mktemp)"
trap 'rm -rf "$TMP"; rm -f "$TMP_STDOUT"' EXIT

mkdir -p "$TMP/src" "$TMP/secrets" "$TMP/rpc" "$TMP/docs"

cat > "$TMP/src/lib.rs" <<'RS'
pub fn safe() -> &'static str {
    "safe"
}
RS

cat > "$TMP/docs/README.md" <<'MD'
# Safe Doc

This file is safe and should be included.
MD

cat > "$TMP/secrets/private.pem" <<'PEM'
-----BEGIN PRIVATE KEY-----
not-a-real-key-but-this-shape-must-never-ship
-----END PRIVATE KEY-----
PEM

python3 - <<'PY' > "$TMP/keypair.json"
print("[" + ",".join(["1"] * 64) + "]")
PY

{
  printf '%s' 'https://'
  printf '%s' 'rpc.example.invalid/'
  printf '%s' '?api'
  printf '%s' '-'
  printf '%s' 'key'
  printf '%s' '='
  printf '%s\n' 'abcdef1234567890'
} > "$TMP/rpc/provider-url.txt"

{
  printf '%s' 'OPENAI'
  printf '%s' '_API'
  printf '%s' '_KEY'
  printf '%s' '='
  printf '%s\n' 'sk-this-is-not-real-but-is-secret-shaped'
} > "$TMP/docs/credentials.md"

bash "$ROOT/scripts/make_codebundle.sh" "$TMP" "$TMP/CODEBUNDLE.md" > "$TMP_STDOUT"

[ -f "$TMP/CODEBUNDLE.md" ] || {
  cat "$TMP_STDOUT" >&2
  echo "expected CODEBUNDLE.md to be created" >&2
  exit 1
}

grep -Fq "src/lib.rs" "$TMP/CODEBUNDLE.md" || {
  echo "safe source file was not included" >&2
  exit 1
}

for forbidden in \
  "BEGIN PRIVATE KEY" \
  "[1,1,1,1,1,1,1,1" \
  "api-key=abcdef1234567890" \
  "OPENAI_API_KEY=sk-"
do
  if grep -Fq "$forbidden" "$TMP/CODEBUNDLE.md"; then
    echo "generated synthetic bundle leaked forbidden marker: $forbidden" >&2
    exit 1
  fi
done

grep -Fq $'secrets/private.pem\texcluded-path' "$TMP/CODEBUNDLE.md" || {
  echo "expected PEM path to be skipped by excluded-path" >&2
  exit 1
}

grep -Fq $'keypair.json\tsensitive-path' "$TMP/CODEBUNDLE.md" || {
  echo "expected keypair JSON path to be skipped by sensitive-path" >&2
  exit 1
}

grep -Fq $'rpc/provider-url.txt\tsensitive-path' "$TMP/CODEBUNDLE.md" || {
  echo "expected provider URL path to be skipped by sensitive-path" >&2
  exit 1
}

grep -Fq $'docs/credentials.md\tsecret-content' "$TMP/CODEBUNDLE.md" || {
  echo "expected credential assignment content to be skipped by secret-content" >&2
  exit 1
}

echo "ok: codebundle secret failsafe synthetic test passed"
