#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — Phase 14 audit-prep document checker.
# RO:WHY — Keeps audit docs tied to real tests and prevents non-authorized launch wording.
# RO:INTERACTS — docs/audit, scripts, crate-local tests.
# RO:INVARIANTS — audit docs must map to tests and must not authorize public/mainnet/production behavior.
# RO:SECURITY — read-only checker; no RPC, wallet, deployment, mint, burn, settlement, or submission.
# RO:TEST — bash scripts/check_audit_prep.sh . and cargo test -p rox-anchor-cli --test audit_prep_docs.

fail() {
  printf 'FAIL: %s\n' "$*" >&2
  exit 1
}

ok() {
  printf 'ok: %s\n' "$*"
}

ROOT="${1:-.}"
DOC_DIR="$ROOT/docs/audit"

[ -d "$DOC_DIR" ] || fail "docs/audit directory not found"

required_docs=(
  "INVARIANT_TEST_MAP.md"
  "AUTHORITY_MODEL.md"
  "STATE_TRANSITIONS.md"
  "RPC_BOUNDARY.md"
  "RELAYER_BOUNDARY.md"
  "MINT_BURN_BOUNDARY.md"
  "HALT_RECOVERY_RUNBOOK.md"
  "KEY_ROTATION_RUNBOOK.md"
  "TESTNET_DEPLOYMENT_RUNBOOK.md"
  "KNOWN_NON_GOALS.md"
  "AUDIT_PREP_INDEX.md"
)

require_contains() {
  local file="$1"
  local needle="$2"
  local label="$3"

  if ! grep -Fq "$needle" "$file"; then
    fail "$label missing from ${file#$ROOT/}: $needle"
  fi
}

for doc in "${required_docs[@]}"; do
  path="$DOC_DIR/$doc"
  [ -f "$path" ] || fail "missing audit doc: docs/audit/$doc"
  require_contains "$path" "ROX Anchor Phase 14" "phase marker"
  require_contains "$path" "No public launch authorization." "non-authorization marker"
  ok "found docs/audit/$doc"
done

require_contains "$DOC_DIR/INVARIANT_TEST_MAP.md" "crates/rox-anchor-core/tests/testnet_scope_locks.rs" "invariant core test map"
require_contains "$DOC_DIR/INVARIANT_TEST_MAP.md" "crates/rox-anchor-rpc-proof/tests/testnet_chaos_drills.rs" "invariant rpc chaos map"
require_contains "$DOC_DIR/INVARIANT_TEST_MAP.md" "crates/rox-anchor-relayer/tests/capped_testnet_submission.rs" "invariant relayer cap map"
require_contains "$DOC_DIR/AUTHORITY_MODEL.md" "crates/rox-anchor-core/tests/operator_authority_model.rs" "authority core tests"
require_contains "$DOC_DIR/AUTHORITY_MODEL.md" "wrong_authority_cannot_halt_or_recover_config" "program wrong-authority test"
require_contains "$DOC_DIR/STATE_TRANSITIONS.md" "finalized_operations_cannot_be_reopened" "state finalization test"
require_contains "$DOC_DIR/RPC_BOUNDARY.md" "RPC outage fails closed" "rpc outage boundary"
require_contains "$DOC_DIR/RPC_BOUNDARY.md" "crates/rox-anchor-rpc-proof/tests/rpc_equivocation_chaos.rs" "rpc equivocation test"
require_contains "$DOC_DIR/RELAYER_BOUNDARY.md" "crates/rox-anchor-relayer/tests/testnet_chaos_drills.rs" "relayer chaos test"
require_contains "$DOC_DIR/MINT_BURN_BOUNDARY.md" "token_settlement_binding_derives_roc_to_rox_intent_from_config_and_plan" "mint/burn roc-to-rox test"
require_contains "$DOC_DIR/HALT_RECOVERY_RUNBOOK.md" "crates/rox-anchor-cli/tests/kill_switch_drill_command.rs" "halt drill cli test"
require_contains "$DOC_DIR/KEY_ROTATION_RUNBOOK.md" "authority_rotation_intent_rejects_noop_and_requires_activation_slot" "key rotation intent test"
require_contains "$DOC_DIR/KEY_ROTATION_RUNBOOK.md" "wrong_authority_cannot_halt_or_recover_config" "key rotation wrong authority test"
require_contains "$DOC_DIR/TESTNET_DEPLOYMENT_RUNBOOK.md" "scripts/check_testnet_deploy_drill.sh" "deployment drill script"
require_contains "$DOC_DIR/KNOWN_NON_GOALS.md" "mainnet-beta deployment" "known non-goal mainnet"
require_contains "$DOC_DIR/KNOWN_NON_GOALS.md" "fake finality" "known non-goal fake finality"
require_contains "$DOC_DIR/AUDIT_PREP_INDEX.md" "KEY_ROTATION_RUNBOOK.md" "audit index key rotation"
require_contains "$DOC_DIR/AUDIT_PREP_INDEX.md" "Successful Phase 14 means the repo is audit-prep ready" "audit index phase conclusion"

combined="$(mktemp)"
trap 'rm -f "$combined"' EXIT
cat "$DOC_DIR"/*.md > "$combined"

for forbidden in \
  "public launch authorized" \
  "mainnet launch authorized" \
  "mainnet-beta authorized" \
  "production bridge authorized" \
  "production settlement authorized" \
  "exchange ready" \
  "staking ready" \
  "liquidity ready" \
  "fake finality allowed"
do
  if grep -Fiq "$forbidden" "$combined"; then
    fail "audit docs contain forbidden authorization wording: $forbidden"
  fi
done

ok "Phase 14 audit prep checks passed"
ok "this script did not deploy, submit, mint, burn, settle, or load a wallet"
