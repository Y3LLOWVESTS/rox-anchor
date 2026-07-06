#!/usr/bin/env bash
set -euo pipefail

fail() {
  printf 'FAIL: %s\n' "$*" >&2
  exit 1
}

ok() {
  printf 'ok: %s\n' "$*"
}

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
  cat <<'USAGE'
usage:
  bash scripts/check_private_testnet_pilot_closeout.sh [repo-root]
  bash scripts/check_private_testnet_pilot_closeout.sh --checklist
USAGE
  exit 0
fi

if [ "${1:-}" = "--checklist" ]; then
  cat <<'CHECKLIST'
ROX Anchor BUILD_PLAN3 Phase 16 — private testnet pilot closeout checklist

Local gate:
  1. cargo fmt --all
  2. bash scripts/check_private_testnet_pilot_closeout.sh .
  3. cargo test -p rox-anchor-cli --test private_testnet_pilot_closeout
  4. cargo test --workspace
  5. cargo check --workspace
  6. anchor build
  7. anchor test
  8. bash scripts/make_codebundle.sh

Final Clippy checkpoint:
  - cargo clippy -p rox-anchor-core --all-targets -- -D warnings
  - cargo clippy -p rox-anchor-proof --all-targets -- -D warnings
  - cargo clippy -p rox-anchor-cli --all-targets -- -D warnings
  - cargo clippy -p rox-anchor-rpc-proof --all-targets -- -D warnings
  - cargo clippy -p rox-anchor-coordinator --all-targets -- -D warnings
  - cargo clippy -p rox-anchor-relayer --all-targets -- -D warnings
  - cargo clippy -p rox-anchor --all-targets -- -D warnings

Forbidden by this closeout:
  - public launch authorization
  - mainnet authorization
  - production bridge settlement
  - public ROX mint/burn
  - real internal ROC release
  - public bridge UI
  - exchange readiness
  - staking readiness
  - liquidity readiness
  - fake finality or fake success
CHECKLIST
  exit 0
fi

ROOT="${1:-.}"
ROOT="$(cd "$ROOT" && pwd)"

BUILD_PLAN3="$ROOT/BUILD_PLAN3.md"
CLOSEOUT_DOC="$ROOT/docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md"

[ -f "$BUILD_PLAN3" ] || fail "BUILD_PLAN3.md is missing"
[ -f "$CLOSEOUT_DOC" ] || fail "docs/pilot/PRIVATE_TESTNET_PILOT_CLOSEOUT.md is missing"

require_file() {
  local rel="$1"
  [ -f "$ROOT/$rel" ] || fail "required file missing: $rel"
  ok "required file present: $rel"
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

require_absent() {
  local file="$1"
  local needle="$2"
  local label="$3"

  if grep -Fqi "$needle" "$file"; then
    fail "$label found disallowed text: $needle"
  fi

  ok "$label"
}

require_contains "$BUILD_PLAN3" "## Phase 16 — Private Testnet Pilot Closeout Gate" "BUILD_PLAN3 Phase 16 marker"
require_contains "$BUILD_PLAN3" "Confirm all local tests pass." "BUILD_PLAN3 local test closeout item"
require_contains "$BUILD_PLAN3" "Confirm all Anchor tests pass." "BUILD_PLAN3 Anchor test closeout item"
require_contains "$BUILD_PLAN3" "Confirm no key material is tracked." "BUILD_PLAN3 key-material closeout item"
require_contains "$BUILD_PLAN3" "Confirm no public launch behavior exists." "BUILD_PLAN3 public launch closeout item"
require_contains "$BUILD_PLAN3" "Confirm no mainnet behavior exists." "BUILD_PLAN3 mainnet closeout item"
require_contains "$BUILD_PLAN3" "Confirm no production settlement behavior exists." "BUILD_PLAN3 production settlement closeout item"
require_contains "$BUILD_PLAN3" "Confirm no real ROC release behavior exists." "BUILD_PLAN3 real ROC closeout item"
require_contains "$BUILD_PLAN3" "Confirm the next plan, if any, is separate and explicitly scoped." "BUILD_PLAN3 future-plan separation item"

require_file "scripts/check_private_pilot_hygiene.sh"
require_file "scripts/check_private_testnet_deploy.sh"
require_file "scripts/check_testnet_readiness_gate.sh"
require_file "scripts/make_codebundle.sh"
require_file "crates/rox-anchor-cli/tests/private_pilot_drill_reports.rs"
require_file "docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"
require_file "docs/pilot/PRIVATE_TESTNET_DEPLOYMENT_RUNBOOK.md"
require_file "docs/pilot/PRIVATE_TESTNET_READ_ONLY_RPC.md"
require_file "docs/pilot/SIMULATION_ONLY_PILOT_TRANSACTION_PLANS.md"
require_file "docs/pilot/EXPLICIT_CAPPED_PRIVATE_TESTNET_SENDER.md"
require_file "docs/pilot/ROC_TO_ROX_PRIVATE_PILOT.md"
require_file "docs/pilot/ROX_TO_ROC_PRIVATE_PILOT.md"

require_contains "$CLOSEOUT_DOC" "ROX Anchor BUILD_PLAN3 Phase 16" "closeout phase marker"
require_contains "$CLOSEOUT_DOC" "complete / green / parked only after required local commands pass" "conditional closeout marker"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize public launch." "public launch non-authorization"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize mainnet." "mainnet non-authorization"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize production bridge settlement." "settlement non-authorization"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize real internal ROC release." "ROC release non-authorization"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize exchange-facing behavior." "exchange non-authorization"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize staking." "staking non-authorization"
require_contains "$CLOSEOUT_DOC" "This closeout gate does not authorize liquidity." "liquidity non-authorization"
require_contains "$CLOSEOUT_DOC" "Any future plan after this closeout must be a separate explicitly scoped plan." "future plan separation marker"

require_absent "$CLOSEOUT_DOC" "public launch authorized" "no public launch authorization phrase"
require_absent "$CLOSEOUT_DOC" "mainnet-beta deployment authorized" "no mainnet authorization phrase"
require_absent "$CLOSEOUT_DOC" "production settlement authorized" "no production settlement authorization phrase"
require_absent "$CLOSEOUT_DOC" "settlement complete" "no settlement completion claim"
require_absent "$CLOSEOUT_DOC" "mint complete" "no mint completion claim"
require_absent "$CLOSEOUT_DOC" "production ready" "no production-ready claim"

if [ -d "$ROOT/.git" ] && command -v git >/dev/null 2>&1; then
  tracked_tmp="$(mktemp)"
  git -C "$ROOT" ls-files > "$tracked_tmp"

  if grep -Ei '(^|/)(id|keypair|wallet|payer|authority|upgrade-authority|program-authority|deploy-authority|mint-authority)\.json$|(\.keypair\.json|\.wallet\.json|\.authority\.json|\.pilot-keypair\.json|\.pilot-wallet\.json|\.pilot-authority\.json|\.pilot-payer\.json|\.pilot-deploy-output\.json)$' "$tracked_tmp"; then
    rm -f "$tracked_tmp"
    fail "tracked key-shaped or authority-shaped file found"
  fi

  rm -f "$tracked_tmp"
  ok "git tracked-file scan found no key-shaped files"
else
  ok "git tracked-file scan skipped"
fi

cat <<'SUMMARY'
ok: BUILD_PLAN3 Phase 16 private testnet pilot closeout checks passed
summary:
  - closeout doc is present
  - closeout completion remains conditional on local green commands
  - required pilot docs and scripts are present
  - tracked-file scan found no key-shaped files when git metadata was available
  - no public/mainnet/production/ROC-release/staking/liquidity/exchange authorization is present
  - this script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, sign, or load a wallet
SUMMARY