#!/usr/bin/env bash
set -euo pipefail

# Always run from repo root (works even if invoked from a subdir)
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

echo "[ci] start"

ran=0

fail_with_hint() {
  local message="$1"
  local hint="$2"
  echo "[ci] ERROR: ${message}"
  echo "[ci] HINT : ${hint}"
  exit 1
}

run() {
  ran=1
  if ! "$@"; then
    local cmd
    cmd="$(printf '%q ' "$@")"
    cmd="${cmd% }"
    fail_with_hint "command failed: ${cmd}" "rerun exactly: ${cmd}; apply a minimal diff fix, then rerun ./ci.sh"
  fi
}

require_file() {
  local path="$1"
  local hint="$2"
  ran=1
  if ! test -f "$path"; then
    fail_with_hint "missing ${path}" "${hint}"
  fi
}

always_on_checks() {
  echo "[ci] phase: always_on"
  local required_files=(
    "AGENTS.md|restore AGENTS.md (operation SSOT)"
    "docs/constitution/00_constitution.md|restore constitution SSOT docs"
    "docs/constitution/10_core_fact_spec.md|restore core fact SSOT docs"
    "docs/constitution/20_share_envelope_spec.md|restore share envelope SSOT docs"
    "docs/constitution/30_testplan.md|create or restore Acceptance/TST SSOT"
    "docs/rfc/RFC-0000-template.md|restore RFC template SSOT"
    "artifacts/inbox/tasks/TASK-0000-template.md|restore task template"
    "artifacts/packs/CP-0000-template.md|restore context-pack template"
    "docs/constitution/contracts/trace.schema.json|create or restore trace contract schema"
  )
  local entry path hint
  for entry in "${required_files[@]}"; do
    path="${entry%%|*}"
    hint="${entry#*|}"
    require_file "$path" "$hint"
  done

  if command -v python3 >/dev/null 2>&1; then
    run python3 -m json.tool docs/constitution/contracts/trace.schema.json >/dev/null
  elif command -v node >/dev/null 2>&1; then
    run node -e "JSON.parse(require('fs').readFileSync('docs/constitution/contracts/trace.schema.json','utf8'));"
  else
    fail_with_hint "need python3 or node to validate docs/constitution/contracts/trace.schema.json" "install python3 or node, then rerun ./ci.sh"
  fi
}

run_rust_tests_if_present() {
  local manifest="$1"
  if ! test -f "$manifest"; then
    return 0
  fi

  if ! command -v cargo >/dev/null 2>&1; then
    fail_with_hint "${manifest} exists but cargo is unavailable (tests would be skipped)" "install Rust toolchain (cargo) so implementation tests can run"
  fi

  run cargo test --manifest-path "$manifest"
}

run_backend_tests_if_present() {
  if ! test -f backend/package.json; then
    return 0
  fi

  if command -v pnpm >/dev/null 2>&1; then
    run pnpm -C backend test
    return 0
  fi

  if command -v npm >/dev/null 2>&1; then
    run bash -lc "cd backend && npm test"
    return 0
  fi

  if command -v yarn >/dev/null 2>&1; then
    run bash -lc "cd backend && yarn test"
    return 0
  fi

  fail_with_hint "backend/package.json exists but no package manager is available (tests would be skipped)" "install pnpm (recommended), npm, or yarn, then rerun ./ci.sh"
}

optional_checks() {
  echo "[ci] phase: optional"
  # Policy: if implementation exists, corresponding tests must run (skip is not allowed).
  local rust_manifests=(
    core/Cargo.toml
    share/Cargo.toml
    aggregate/Cargo.toml
    platform/Cargo.toml
  )
  local manifest
  for manifest in "${rust_manifests[@]}"; do
    run_rust_tests_if_present "$manifest"
  done

  run_backend_tests_if_present

  if test -f android/gradlew; then
    if ! command -v java >/dev/null 2>&1; then
      fail_with_hint "android/gradlew exists but java is unavailable (tests would fail)" "install a JDK and rerun ./ci.sh"
    fi
    run bash -lc "cd android && ./gradlew test"
  fi
}

guard_checks() {
  echo "[ci] phase: guard"
  if test "$ran" -eq 0; then
    fail_with_hint "no-op green is forbidden" "configure at least one real check target in ./ci.sh"
  fi
}

main() {
  always_on_checks
  optional_checks
  guard_checks
  echo "[ci] green"
}

main "$@"
