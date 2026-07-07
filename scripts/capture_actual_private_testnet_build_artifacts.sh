#!/usr/bin/env bash
set -euo pipefail

# RO:WHAT — BUILD_PLAN4 Phase 2 Anchor build artifact manifest capture.
# RO:WHY — Hashes local Anchor build outputs before deployment while keeping paths redacted and local manifests ignored.
# RO:INTERACTS — Anchor.toml, target/deploy/rox_anchor.so, target/idl/rox_anchor.json, docs/pilot.
# RO:INVARIANTS — build evidence only; no deployment proof; no finality; no wallet/RPC/send/mint/burn/ROC mutation.
# RO:SECURITY — read-only local file hashing; no RPC, signing, deployment, submission, settlement, or key loading.
# RO:TEST — cargo test -p rox-anchor-core --test actual_testnet_artifact_manifest and cargo test -p rox-anchor-cli --test actual_testnet_artifact_manifest_status.

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
  bash scripts/capture_actual_private_testnet_build_artifacts.sh --check-docs [repo-root]
  bash scripts/capture_actual_private_testnet_build_artifacts.sh --template
  bash scripts/capture_actual_private_testnet_build_artifacts.sh --capture [repo-root] [output-json|-] [devnet|testnet]

examples:
  anchor build
  bash scripts/capture_actual_private_testnet_build_artifacts.sh --capture . .rox-anchor-private-pilot/actual-private-testnet-build-artifacts.local.json devnet
  bash scripts/capture_actual_private_testnet_build_artifacts.sh --capture . - testnet
USAGE
}

json_escape() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

file_size() {
  wc -c < "$1" | tr -d '[:space:]'
}

hash_file_sha256() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
    return
  fi

  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
    return
  fi

  fail "neither shasum nor sha256sum is available"
}

tool_version() {
  local tool="$1"
  shift

  if command -v "$tool" >/dev/null 2>&1; then
    "$tool" "$@" 2>/dev/null | head -n 1
  else
    printf 'unavailable'
  fi
}

extract_program_id() {
  local anchor_toml="$1"
  local cluster="$2"

  awk -v target="$cluster" '
    $0 ~ "^\\[programs\\." target "\\]" {
      in_scope = 1
      next
    }

    $0 ~ "^\\[" {
      if (in_scope == 1) {
        exit
      }
    }

    in_scope == 1 && $1 == "rox_anchor" {
      value = $3
      gsub(/"/, "", value)
      print value
      exit
    }
  ' "$anchor_toml"
}

redacted_anchor_path() {
  local path="$1"
  printf '<redacted-anchor-build-path>/%s' "$(basename "$path")"
}

print_template() {
  cat <<'TEMPLATE'
{
  "schema": "rox-anchor.actual-private-testnet-build-artifacts.v1",
  "phase": "BUILD_PLAN4 Phase 2",
  "artifact_role": "anchor_build_metadata_only",
  "cluster": "devnet",
  "program_name": "rox_anchor",
  "program_id": "U91owoSZLda4pZf2Qw8Xz3rS5v2vvi95kSev33KTivR",
  "expected_program_id_source": "Anchor.toml [programs.devnet]",
  "program_binary_sha256": "<sha256>",
  "program_binary_size_bytes": 0,
  "idl_sha256": "<sha256>",
  "idl_size_bytes": 0,
  "anchor_version": "<anchor --version>",
  "solana_cli_version": "<solana --version>",
  "rustc_version": "<rustc --version>",
  "program_artifact_path": "<redacted-anchor-build-path>/rox_anchor.so",
  "idl_artifact_path": "<redacted-anchor-build-path>/rox_anchor.json",
  "build_manifest_is_deployment_proof": false,
  "deployment_claim": false,
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "real_roc_mutation": false
}
TEMPLATE
}

check_docs() {
  local root="${1:-.}"
  root="$(cd "$root" && pwd)"

  local doc="$root/docs/pilot/ACTUAL_PRIVATE_TESTNET_BUILD_ARTIFACTS.md"
  local script="$root/scripts/capture_actual_private_testnet_build_artifacts.sh"
  local gitignore="$root/.gitignore"

  [ -f "$doc" ] || fail "missing docs/pilot/ACTUAL_PRIVATE_TESTNET_BUILD_ARTIFACTS.md"
  [ -f "$script" ] || fail "missing scripts/capture_actual_private_testnet_build_artifacts.sh"
  [ -f "$gitignore" ] || fail "missing .gitignore"

  for needle in \
    "ROX Anchor BUILD_PLAN4 Phase 2" \
    "anchor build" \
    "target/deploy/rox_anchor.so" \
    "target/idl/rox_anchor.json" \
    "program binary SHA-256" \
    "IDL SHA-256" \
    "build_manifest_is_deployment_proof" \
    "deployment_claim" \
    "finality_claim" \
    "No deployment proof." \
    "No finality proof." \
    "No real internal ROC release."
  do
    grep -Fq "$needle" "$doc" || fail "build artifact doc missing marker: $needle"
    ok "doc marker present: $needle"
  done

  for ignored in \
    "actual-private-testnet-build-artifacts.json" \
    "*.actual-anchor-build.local.json" \
    "*.actual-private-testnet-build.local.json" \
    "*.actual-build-manifest.local.json" \
    "*.anchor-build-manifest.local.json"
  do
    grep -Fq "$ignored" "$gitignore" || fail ".gitignore missing build artifact ignore marker: $ignored"
    ok ".gitignore covers $ignored"
  done

  for forbidden in \
    "/Users/" \
    "/home/" \
    "api-key=" \
    "apikey=" \
    "access_token=" \
    "deployment_success\": true" \
    "build_manifest_is_deployment_proof\": true" \
    "finality_claim\": true" \
    "public_launch_authorized\": true" \
    "mainnet_authorized\": true"
  do
    if grep -Fq "$forbidden" "$doc"; then
      fail "build artifact doc contains forbidden marker: $forbidden"
    fi
    ok "doc excludes forbidden marker: $forbidden"
  done

  cat <<'SUMMARY'
ok: BUILD_PLAN4 Phase 2 build artifact documentation checks passed
summary:
  - actual Anchor build artifact capture is documented
  - local manifest artifact names are ignored
  - documentation preserves build-only/non-deployment/non-finality boundaries
  - documentation uses redacted paths and contains no operator-local secret paths
SUMMARY
}

capture_manifest() {
  local root="${1:-.}"
  local output="${2:-.rox-anchor-private-pilot/actual-private-testnet-build-artifacts.local.json}"
  local cluster="${3:-devnet}"

  case "$cluster" in
    devnet|testnet) ;;
    mainnet-beta|mainnet|localnet|Localnet|"")
      fail "cluster must be devnet or testnet for BUILD_PLAN4 Phase 2 capture, got: ${cluster:-<empty>}"
      ;;
    *)
      fail "unsupported cluster for private testnet build capture: $cluster"
      ;;
  esac

  root="$(cd "$root" && pwd)"

  local anchor_toml="$root/Anchor.toml"
  local program_so="$root/target/deploy/rox_anchor.so"
  local idl_json="$root/target/idl/rox_anchor.json"

  [ -f "$anchor_toml" ] || fail "Anchor.toml not found under $root"
  [ -f "$program_so" ] || fail "program binary missing; run anchor build first: target/deploy/rox_anchor.so"
  [ -f "$idl_json" ] || fail "IDL missing; run anchor build first: target/idl/rox_anchor.json"

  local program_id
  program_id="$(extract_program_id "$anchor_toml" "$cluster")"

  [ -n "$program_id" ] || fail "could not find rox_anchor program ID in Anchor.toml [programs.$cluster]"

  case "$program_id" in
    *[[:space:]]*|"")
      fail "invalid program ID extracted from Anchor.toml [programs.$cluster]"
      ;;
  esac

  local program_hash idl_hash program_bytes idl_bytes
  program_hash="$(hash_file_sha256 "$program_so")"
  idl_hash="$(hash_file_sha256 "$idl_json")"
  program_bytes="$(file_size "$program_so")"
  idl_bytes="$(file_size "$idl_json")"

  case "$program_hash" in
    [0-9a-f][0-9a-f][0-9a-f][0-9a-f]*) ;;
    *) fail "program binary hash did not look like a SHA-256 hex string" ;;
  esac

  case "$idl_hash" in
    [0-9a-f][0-9a-f][0-9a-f][0-9a-f]*) ;;
    *) fail "IDL hash did not look like a SHA-256 hex string" ;;
  esac

  local anchor_version solana_version rustc_version
  anchor_version="$(tool_version anchor --version)"
  solana_version="$(tool_version solana --version)"
  rustc_version="$(tool_version rustc --version)"

  local program_path_redacted idl_path_redacted
  program_path_redacted="$(redacted_anchor_path "$program_so")"
  idl_path_redacted="$(redacted_anchor_path "$idl_json")"

  local manifest
  manifest="$(cat <<JSON
{
  "schema": "rox-anchor.actual-private-testnet-build-artifacts.v1",
  "phase": "BUILD_PLAN4 Phase 2",
  "artifact_role": "anchor_build_metadata_only",
  "cluster": "$(json_escape "$cluster")",
  "program_name": "rox_anchor",
  "program_id": "$(json_escape "$program_id")",
  "expected_program_id_source": "Anchor.toml [programs.$(json_escape "$cluster")]",
  "program_binary_sha256": "$(json_escape "$program_hash")",
  "program_binary_size_bytes": $program_bytes,
  "idl_sha256": "$(json_escape "$idl_hash")",
  "idl_size_bytes": $idl_bytes,
  "anchor_version": "$(json_escape "$anchor_version")",
  "solana_cli_version": "$(json_escape "$solana_version")",
  "rustc_version": "$(json_escape "$rustc_version")",
  "program_artifact_path": "$(json_escape "$program_path_redacted")",
  "idl_artifact_path": "$(json_escape "$idl_path_redacted")",
  "build_manifest_is_deployment_proof": false,
  "deployment_claim": false,
  "finality_claim": false,
  "runtime_authority": false,
  "public_launch_authorized": false,
  "mainnet_authorized": false,
  "real_roc_mutation": false
}
JSON
)"

  if printf '%s' "$manifest" | grep -Fq "$root"; then
    fail "manifest leaked absolute repo root path"
  fi

  if printf '%s' "$manifest" | grep -Eq '/Users/|/home/|api-key=|apikey=|access_token='; then
    fail "manifest leaked local path or token-shaped material"
  fi

  if [ "$output" = "-" ]; then
    printf '%s\n' "$manifest"
  else
    mkdir -p "$(dirname "$output")"
    printf '%s\n' "$manifest" > "$output"
    ok "wrote redacted build artifact manifest: $output"
  fi

  cat >&2 <<'SUMMARY'
ok: BUILD_PLAN4 Phase 2 actual Anchor build artifact capture passed
summary:
  - program binary and IDL were hashed from local Anchor build outputs
  - program ID was captured from Anchor.toml devnet/testnet binding
  - local artifact paths were redacted
  - manifest records build evidence only
  - manifest is not deployment proof, finality proof, runtime authority, public launch authorization, or ROC mutation evidence
SUMMARY
}

case "${1:-}" in
  --help|-h)
    usage
    ;;
  --template)
    print_template
    ;;
  --check-docs)
    check_docs "${2:-.}"
    ;;
  --capture)
    capture_manifest "${2:-.}" "${3:-.rox-anchor-private-pilot/actual-private-testnet-build-artifacts.local.json}" "${4:-devnet}"
    ;;
  *)
    usage
    fail "unknown command: ${1:-<missing>}"
    ;;
esac
