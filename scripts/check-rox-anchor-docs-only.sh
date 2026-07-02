#!/usr/bin/env bash
# RO:WHAT — Static docs-only checker for ROX Anchor Phase 0.
# RO:WHY — Enforces docs / threat-model / decision-gate scope and blocks hidden runtime drift.
# RO:INTERACTS — docs/*.md and repository file layout.
# RO:INVARIANTS — checker green is a planning gate only, not runtime authorization.
# RO:SECURITY — blocks forbidden runtime directories and hidden Solana/Anchor/coordinator/relayer implementation markers.
# RO:TEST — bash scripts/check-rox-anchor-docs-only.sh.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

failures=0

fail() {
  echo "FAIL: $*" >&2
  failures=$((failures + 1))
}

need_file() {
  local rel="$1"
  [[ -f "$rel" ]] || fail "missing required file: $rel"
}

need_text() {
  local rel="$1"
  local needle="$2"

  if [[ ! -f "$rel" ]]; then
    fail "cannot inspect missing file: $rel"
    return
  fi

  grep -Fq -- "$needle" "$rel" || fail "missing marker in $rel: $needle"
}

echo "== ROX Anchor docs-only checker: required files =="

required_docs=(
  "docs/00_IDB_ROX_ANCHOR.md"
  "docs/01_SCOPE_DECISION_GATE.md"
  "docs/02_THREAT_MODEL.md"
  "docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md"
  "docs/04_TESTPLAN_CHECKER.md"
)

for rel in "${required_docs[@]}"; do
  need_file "$rel"
done

need_file "scripts/check-rox-anchor-docs-only.sh"

echo "== ROX Anchor docs-only checker: shell syntax =="
bash -n "scripts/check-rox-anchor-docs-only.sh" || fail "bash syntax failed: scripts/check-rox-anchor-docs-only.sh"

echo "== ROX Anchor docs-only checker: required doc markers =="

for rel in "${required_docs[@]}"; do
  need_text "$rel" "North Star:"
  need_text "$rel" "RO:WHAT"
  need_text "$rel" "RO:WHY"
  need_text "$rel" "RO:INVARIANTS"
  need_text "$rel" "RO:SECURITY"
  need_text "$rel" "RO:TEST"
  need_text "$rel" "Anchor meaning used in this document:"
  need_text "$rel" "docs / threat-model / decision-gate"
done

echo "== ROX Anchor docs-only checker: safe status markers =="

need_text "docs/00_IDB_ROX_ANCHOR.md" "Internal ROC Product Beta Readiness aggregate gate: COMPLETE / GREEN / PARKED."
need_text "docs/00_IDB_ROX_ANCHOR.md" "ROX Anchor Phase 0 — Docs-Only Planning Gate:"
need_text "docs/01_SCOPE_DECISION_GATE.md" "ROX Anchor Phase 0 — Docs-Only Planning Gate:"
need_text "docs/04_TESTPLAN_CHECKER.md" "ROX Anchor Phase 0 — Docs-Only Planning Gate:"

echo "== ROX Anchor docs-only checker: forbidden runtime directories =="

for dir in \
  programs \
  anchor \
  migrations \
  app \
  src \
  relayer \
  coordinator \
  crablink-bridge-ui \
  solana-program \
  token-mint \
  mint \
  burn \
  stake \
  staking \
  liquidity \
  dex \
  cex \
  mainnet \
  devnet-deploy
do
  if [[ -d "$dir" ]]; then
    fail "forbidden runtime-shaped directory exists: $dir"
  fi
done

echo "== ROX Anchor docs-only checker: forbidden runtime-shaped files =="

runtime_files=()
while IFS= read -r -d '' file; do
  runtime_files+=("$file")
done < <(
  find . \
    -path './.git' -prune -o \
    -path './docs' -prune -o \
    -path './scripts' -prune -o \
    -type f \( \
      -name '*.rs' -o \
      -name '*.ts' -o \
      -name '*.tsx' -o \
      -name '*.js' -o \
      -name '*.jsx' -o \
      -name '*.mjs' -o \
      -name '*.cjs' -o \
      -name '*.toml' -o \
      -name '*.json' -o \
      -name '*.yaml' -o \
      -name '*.yml' \
    \) -print0
)

if (( ${#runtime_files[@]} > 0 )); then
  for file in "${runtime_files[@]}"; do
    fail "forbidden runtime-shaped file exists outside docs/scripts: ${file#./}"
  done
fi

echo "== ROX Anchor docs-only checker: hidden Solana/Anchor implementation markers =="

hidden_markers=(
  "#[program]"
  "declare_id!"
  "#[derive(Accounts)]"
  "anchor_lang"
  "anchor_spl"
  "Context<"
  "Program<"
  "Account<"
  "AccountInfo<"
  "Signer<"
  "UncheckedAccount"
  "InterfaceAccount"
  "MintTo"
  "TransferChecked"
  "set_authority"
  "spl_token"
  "spl_associated_token_account"
  "invoke_signed"
  "pub mod instructions"
  "pub mod accounts"
  "pub mod state"
)

implementation_candidates=()
while IFS= read -r -d '' file; do
  implementation_candidates+=("$file")
done < <(
  find . \
    -path './.git' -prune -o \
    -path './docs' -prune -o \
    -path './scripts' -prune -o \
    -type f -print0
)

for file in "${implementation_candidates[@]}"; do
  for marker in "${hidden_markers[@]}"; do
    if grep -Fq -- "$marker" "$file"; then
      fail "hidden implementation marker '$marker' found in ${file#./}"
    fi
  done
done

echo "== ROX Anchor docs-only checker: anti-scope language present in docs =="

need_text "docs/01_SCOPE_DECISION_GATE.md" "Forbidden current-scope behavior"
need_text "docs/02_THREAT_MODEL.md" "Threat modeling must not create"
need_text "docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md" "This document must not create"
need_text "docs/04_TESTPLAN_CHECKER.md" "The checker must not create or run"

if (( failures > 0 )); then
  echo "ROX Anchor docs-only checker failed: failures=$failures" >&2
  exit 1
fi

echo
echo "ROX Anchor Phase 0 — Docs-Only Planning Gate:"
echo "COMPLETE / GREEN / PARKED."
echo
echo "== docs-only scope preserved; no runtime-shaped files/directories detected =="
echo "== no Solana/Anchor/coordinator/relayer implementation markers detected outside docs/scripts =="
echo "== this does not authorize ROX runtime, Solana runtime, bridge runtime, staking, liquidity, external settlement, or user-facing bridge behavior =="
