#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — Phase 15 private testnet readiness gate checker.
# RO:WHY — Confirms the repo has the tested safety surfaces required before private testnet pilot review.
# RO:INTERACTS — BUILD_PLAN2.md, Anchor.toml, docs/audit, scripts, and crate-local tests.
# RO:INVARIANTS — private testnet only; no mainnet, public bridge, production settlement, or fake success.
# RO:SECURITY — read-only checker; no RPC, wallet load, deploy, submit, mint, burn, or settlement.
# RO:TEST — bash scripts/check_testnet_readiness_gate.sh . and cargo test -p rox-anchor-cli --test testnet_readiness_gate.

fail() {
  printf 'FAIL: %s\n' "$*" >&2
  exit 1
}

ok() {
  printf 'ok: %s\n' "$*"
}

require_file() {
  local path="$1"
  [ -f "$path" ] || fail "required file missing: $path"
  ok "found ${path#"$ROOT"/}"
}

require_contains() {
  local file="$1"
  local needle="$2"
  local label="$3"

  if ! grep -Fq "$needle" "$file"; then
    fail "$label missing expected text: $needle"
  fi

  ok "$label"
}

usage() {
  cat <<'USAGE'
usage:
  bash scripts/check_testnet_readiness_gate.sh [repo-root]
USAGE
}

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
  usage
  exit 0
fi

ROOT="${1:-.}"
ROOT="$(cd "$ROOT" && pwd)"

BUILD_PLAN2="$ROOT/BUILD_PLAN2.md"
ANCHOR_TOML="$ROOT/Anchor.toml"

require_file "$BUILD_PLAN2"
require_file "$ANCHOR_TOML"

required_files=(
  "$ROOT/docs/audit/INVARIANT_TEST_MAP.md"
  "$ROOT/docs/audit/AUTHORITY_MODEL.md"
  "$ROOT/docs/audit/STATE_TRANSITIONS.md"
  "$ROOT/docs/audit/RPC_BOUNDARY.md"
  "$ROOT/docs/audit/RELAYER_BOUNDARY.md"
  "$ROOT/docs/audit/MINT_BURN_BOUNDARY.md"
  "$ROOT/docs/audit/HALT_RECOVERY_RUNBOOK.md"
  "$ROOT/docs/audit/KEY_ROTATION_RUNBOOK.md"
  "$ROOT/docs/audit/TESTNET_DEPLOYMENT_RUNBOOK.md"
  "$ROOT/docs/audit/KNOWN_NON_GOALS.md"
  "$ROOT/docs/audit/AUDIT_PREP_INDEX.md"
  "$ROOT/docs/audit/TESTNET_READINESS_GATE.md"
  "$ROOT/scripts/check_audit_prep.sh"
  "$ROOT/scripts/check_testnet_deploy_drill.sh"
  "$ROOT/scripts/check_testnet_readiness_gate.sh"
  "$ROOT/crates/rox-anchor-cli/tests/testnet_readiness_gate.rs"
  "$ROOT/crates/rox-anchor-cli/tests/audit_prep_docs.rs"
  "$ROOT/crates/rox-anchor-cli/tests/testnet_deploy_drill_script.rs"
  "$ROOT/crates/rox-anchor-cli/tests/kill_switch_drill_command.rs"
  "$ROOT/crates/rox-anchor-core/tests/testnet_scope_locks.rs"
  "$ROOT/crates/rox-anchor-core/tests/testnet_config_model.rs"
  "$ROOT/crates/rox-anchor-core/tests/operator_authority_model.rs"
  "$ROOT/crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs"
  "$ROOT/crates/rox-anchor-relayer/tests/capped_testnet_submission.rs"
  "$ROOT/crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs"
  "$ROOT/crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs"
  "$ROOT/programs/rox-anchor/src/state.rs"
)

for path in "${required_files[@]}"; do
  require_file "$path"
done

require_contains "$BUILD_PLAN2" "Phase 15 — Testnet Readiness Gate" "build plan phase 15"
require_contains "$BUILD_PLAN2" "This is not a public launch gate." "phase 15 non-launch boundary"
require_contains "$BUILD_PLAN2" "ROX Anchor is ready for a private testnet-only pilot." "private testnet pilot exit condition"

require_contains "$ROOT/docs/audit/TESTNET_READINESS_GATE.md" "cargo test --workspace" "readiness workspace test command"
require_contains "$ROOT/docs/audit/TESTNET_READINESS_GATE.md" "cargo check --workspace" "readiness workspace check command"
require_contains "$ROOT/docs/audit/TESTNET_READINESS_GATE.md" "scripts/check_audit_prep.sh" "readiness audit checker reference"
require_contains "$ROOT/docs/audit/TESTNET_READINESS_GATE.md" "scripts/check_testnet_deploy_drill.sh" "readiness deploy drill reference"
require_contains "$ROOT/docs/audit/TESTNET_READINESS_GATE.md" "scripts/check_testnet_readiness_gate.sh" "readiness gate checker reference"
require_contains "$ROOT/docs/audit/TESTNET_READINESS_GATE.md" "Private pilot boundary" "private pilot boundary"
require_contains "$ROOT/docs/audit/AUDIT_PREP_INDEX.md" "TESTNET_READINESS_GATE.md" "audit index phase 15 readiness doc"

require_contains "$ROOT/crates/rox-anchor-core/tests/testnet_scope_locks.rs" "mainnet_beta_cluster_is_rejected_before_config_can_use_it" "mainnet-beta rejection test"
require_contains "$ROOT/crates/rox-anchor-core/tests/testnet_scope_locks.rs" "public_launch_flags_are_not_available_modes" "no public launch flags test"
require_contains "$ROOT/crates/rox-anchor-core/tests/operator_authority_model.rs" "authority_rotation_intent_rejects_noop_and_requires_activation_slot" "authority rotation test"
require_contains "$ROOT/programs/rox-anchor/src/state.rs" "wrong_authority_cannot_halt_or_recover_config" "program wrong-authority halt/recovery test"
require_contains "$ROOT/crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs" "roc_to_rox_testnet_shadow_flow_reaches_capped_authorization_without_public_mint" "roc-to-rox shadow flow test"
require_contains "$ROOT/crates/rox-anchor-coordinator/tests/testnet_shadow_flow.rs" "rox_to_roc_testnet_shadow_flow_reaches_capped_authorization_without_roc_release" "rox-to-roc shadow flow test"
require_contains "$ROOT/crates/rox-anchor-relayer/tests/capped_testnet_submission.rs" "capped_testnet_submission_authorizes_only_after_all_gates" "capped submit all-gates test"
require_contains "$ROOT/crates/rox-anchor-relayer/tests/capped_testnet_submission.rs" "receipt_persistence_is_required_when_limit_says_so" "receipt persistence test"
require_contains "$ROOT/crates/rox-anchor-cli/tests/kill_switch_drill_command.rs" "drill_default_halt_is_accepted_and_blocks_all_unsafe_stages" "kill-switch halt drill test"

if grep -Eiq '^[[:space:]]*\[programs\.(mainnet|mainnet-beta|mainnetbeta)\][[:space:]]*$' "$ANCHOR_TOML"; then
  fail "Anchor.toml must not contain a mainnet/mainnet-beta programs section"
fi
ok "Anchor.toml has no mainnet program section"

if grep -Eiq '^[[:space:]]*cluster[[:space:]]*=[[:space:]]*"(mainnet|mainnet-beta|mainnetbeta)"[[:space:]]*$' "$ANCHOR_TOML"; then
  fail "Anchor.toml provider cluster must not default to mainnet"
fi
ok "Anchor.toml provider cluster is not mainnet"

if command -v git >/dev/null 2>&1; then
  tracked="$(git -C "$ROOT" ls-files 2>/dev/null || true)"
  if [ -n "$tracked" ] && printf '%s\n' "$tracked" | grep -Eiq '(^|/)(id|keypair|wallet|payer|authority|mint-authority|upgrade-authority|program-authority|deploy-authority|admin|owner|validator-keypair|faucet-keypair)\.json$|(\.keypair\.json$|\.wallet\.json$|\.authority\.json$|\.testnet-(keypair|wallet|authority|payer)\.json$)'; then
    fail "tracked key material-like file detected"
  fi
  ok "tracked files do not include common Solana key material names"
else
  ok "git not available; skipped tracked-file key material check"
fi

bash "$ROOT/scripts/check_audit_prep.sh" "$ROOT" >/dev/null
ok "Phase 14 audit prep checker remains green"

bash "$ROOT/scripts/check_testnet_deploy_drill.sh" "$ROOT" >/dev/null
ok "testnet deployment drill checker remains green"

ok "Phase 15 testnet readiness gate checks passed"
ok "private testnet-only pilot review surface is present"
ok "public launch, mainnet, production settlement, public mint/burn, staking, liquidity, and exchange-facing behavior remain unauthorized"
ok "this script did not deploy, submit, mint, burn, settle, call RPC, or load a wallet"
