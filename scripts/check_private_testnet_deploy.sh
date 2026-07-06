#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN3 Phase 4 private testnet deployment drill checker.
# RO:WHY — Confirms deployment drill readiness without deploying, loading keys, or claiming launch/finality.
# RO:INTERACTS — Anchor.toml, .gitignore, pilot docs, optional git tracked-file view.
# RO:INVARIANTS — external keys only; no mainnet-beta; no committed deployment outputs; no fake success.
# RO:SECURITY — read-only checker; no RPC, wallet load, deploy, submit, mint, burn, settlement, or ROC mutation.
# RO:TEST — bash scripts/check_private_testnet_deploy.sh . and cargo test -p rox-anchor-cli --test private_testnet_deploy_drill.

print_checklist() {
  cat <<'CHECKLIST'
ROX Anchor BUILD_PLAN3 Phase 4 — private testnet deployment drill checklist

Safe local preflight:
  1. cargo fmt --all
  2. cargo test --workspace
  3. cargo check --workspace
  4. anchor build
  5. anchor test
  6. bash scripts/check_private_testnet_deploy.sh .

External-only operator inputs:
  7. export ROX_ANCHOR_PRIVATE_TESTNET_PAYER=/external/non-repo/path/private-testnet-payer.json
  8. export ROX_ANCHOR_PRIVATE_TESTNET_PROGRAM_KEYPAIR=/external/non-repo/path/rox-anchor-program-keypair.json
  9. export ROX_ANCHOR_PRIVATE_TESTNET_UPGRADE_AUTHORITY=/external/non-repo/path/upgrade-authority.json

Local-only artifact capture:
 10. write any drill notes under /external/non-repo/path or ignored pilot-deploy/
 11. capture only redacted, non-secret metadata
 12. never commit payer, program keypair, upgrade authority, RPC provider token, or raw receipt secrets

Optional private testnet drill, only after explicit operator decision:
 13. anchor build
 14. anchor deploy --provider.cluster testnet --provider.wallet "$ROX_ANCHOR_PRIVATE_TESTNET_PAYER"

Forbidden in this drill:
  - mainnet-beta
  - public launch claims
  - public ROX availability claims
  - real user funds
  - internal ROC mutation
  - committed key material
  - committed deployment outputs
  - fake finality
  - fake success output
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
  bash scripts/check_private_testnet_deploy.sh [repo-root]
  bash scripts/check_private_testnet_deploy.sh --checklist
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
PILOT_WORKSPACE_DOC="$ROOT/docs/pilot/PRIVATE_TESTNET_OPERATOR_WORKSPACE.md"
PROGRAM_MANIFEST_DOC="$ROOT/docs/pilot/TESTNET_PROGRAM_MANIFEST.md"
DEPLOY_RUNBOOK="$ROOT/docs/pilot/PRIVATE_TESTNET_DEPLOYMENT_RUNBOOK.md"

[ -f "$ANCHOR_TOML" ] || fail "Anchor.toml is missing"
[ -f "$GITIGNORE" ] || fail ".gitignore is missing"
[ -f "$PILOT_WORKSPACE_DOC" ] || fail "private pilot operator workspace doc is missing"
[ -f "$PROGRAM_MANIFEST_DOC" ] || fail "testnet program manifest doc is missing"
[ -f "$DEPLOY_RUNBOOK" ] || fail "private testnet deployment runbook is missing"

if grep -Eiq '^\[programs\.(mainnet|mainnet-beta)\]' "$ANCHOR_TOML"; then
  fail "Anchor.toml must not define a mainnet/mainnet-beta program section"
fi
ok "Anchor.toml has no mainnet/mainnet-beta program section"

if grep -Eiq 'cluster[[:space:]]*=[[:space:]]*"(mainnet|mainnet-beta)"' "$ANCHOR_TOML"; then
  fail "Anchor provider cluster must not be mainnet/mainnet-beta"
fi
ok "Anchor provider cluster is not mainnet/mainnet-beta"

grep -Eq '^\[programs\.devnet\]' "$ANCHOR_TOML" || fail "Anchor.toml must include a devnet program binding"
grep -Eq '^\[programs\.testnet\]' "$ANCHOR_TOML" || fail "Anchor.toml must include a testnet program binding"
ok "devnet/testnet program bindings are present"

grep -Fq 'rox_anchor = "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR"' "$ANCHOR_TOML" \
  || fail "expected ROX Anchor program binding is missing from Anchor.toml"
ok "expected ROX Anchor program ID binding is present"

for required_ignore in \
  ".rox-anchor-private-pilot/" \
  "private-pilot/" \
  "pilot-deploy/" \
  "pilot-artifacts/" \
  "pilot-receipts/" \
  "*.pilot-deploy-output.json" \
  "target/deploy/*.json"
do
  grep -Fq "$required_ignore" "$GITIGNORE" || fail ".gitignore missing required private deploy pattern: $required_ignore"
done
ok ".gitignore covers private pilot deploy artifacts and key-shaped outputs"

grep -Fq 'external deploy keypair path' "$DEPLOY_RUNBOOK" \
  || fail "deployment runbook must mention external deploy keypair path"
grep -Fq 'external payer path' "$DEPLOY_RUNBOOK" \
  || fail "deployment runbook must mention external payer path"
grep -Fq 'external upgrade authority path' "$DEPLOY_RUNBOOK" \
  || fail "deployment runbook must mention external upgrade authority path"
grep -Fq 'redacted deployment drill report' "$DEPLOY_RUNBOOK" \
  || fail "deployment runbook must require redacted drill reporting"
grep -Fq 'not a launch' "$DEPLOY_RUNBOOK" \
  || fail "deployment runbook must say the drill is not a launch"
ok "private testnet deployment runbook contains required operator boundaries"

if [ -d "$ROOT/.git" ] && command -v git >/dev/null 2>&1; then
  tracked_tmp="$(mktemp)"
  git -C "$ROOT" ls-files > "$tracked_tmp"

  if grep -E '(^|/)(target/deploy|pilot-deploy|pilot-keys|pilot-keypairs|pilot-wallets|pilot-secrets|private-pilot|\.rox-anchor-private-pilot)/' "$tracked_tmp"; then
    rm -f "$tracked_tmp"
    fail "tracked private deploy/key/pilot artifact path found"
  fi

  if grep -Ei '(^|/)(id|keypair|wallet|payer|authority|upgrade-authority|program-authority|deploy-authority|mint-authority)\.json$|(\.keypair\.json|\.wallet\.json|\.authority\.json|\.pilot-keypair\.json|\.pilot-wallet\.json|\.pilot-authority\.json|\.pilot-payer\.json|\.pilot-deploy-output\.json)$' "$tracked_tmp"; then
    rm -f "$tracked_tmp"
    fail "tracked key-shaped or deployment-output file found"
  fi

  rm -f "$tracked_tmp"
  ok "git tracked-file scan found no committed deploy keys or private deploy outputs"
else
  ok "git tracked-file scan skipped because git metadata is unavailable"
fi

cat <<'SUMMARY'
ok: BUILD_PLAN3 Phase 4 private testnet deployment drill checks passed
summary:
  - Anchor program bindings are devnet/testnet scoped
  - mainnet-beta is rejected by local inspection
  - external deploy keypair path is required by checklist
  - external payer path is required by checklist
  - external upgrade authority path is required by checklist
  - deployment output remains local/ignored/redacted
  - this script did not deploy, submit, mint, burn, settle, call RPC, mutate ROC, or load a wallet
SUMMARY
