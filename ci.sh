#!/usr/bin/env bash
set -euo pipefail

# Always run from repo root (works even if invoked from a subdir)
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

echo "[ci] start"

ran=0
run() {
  ran=1
  "$@"
}

# --- Always-on minimal checks (bootstrapでも必ず何か走る) ---
run test -f AGENTS.md

run test -f docs/constitution/00_constitution.md
run test -f docs/constitution/10_core_fact_spec.md
run test -f docs/constitution/20_share_envelope_spec.md
run test -f docs/rfc/RFC-0000-template.md

run test -f artifacts/inbox/tasks/TASK-0000-template.md
run test -f artifacts/packs/CP-0000-template.md

# Required SSOT for autonomous mode (bootstrapで追加する前提)
if test -f docs/constitution/30_testplan.md; then
  run true
else
  echo "[ci] ERROR: missing docs/constitution/30_testplan.md"
  echo "[ci] HINT : create the bootstrap template for Acceptance/TST SSOT"
  exit 1
fi

if test -f docs/constitution/contracts/trace.schema.json; then
  if command -v python3 >/dev/null 2>&1; then
    run python3 -m json.tool docs/constitution/contracts/trace.schema.json >/dev/null
  elif command -v node >/dev/null 2>&1; then
    run node -e "JSON.parse(require('fs').readFileSync('docs/constitution/contracts/trace.schema.json','utf8'));"
  else
    echo "[ci] ERROR: need python3 or node to validate JSON (trace.schema.json)"
    exit 1
  fi
else
  echo "[ci] ERROR: missing docs/constitution/contracts/trace.schema.json"
  echo "[ci] HINT : create the bootstrap contract schema (SSOT)"
  exit 1
fi

# --- Optional project checks (存在するものだけ実行) ---
# Rust
if test -f core/Cargo.toml; then run cargo test --manifest-path core/Cargo.toml; fi
if test -f share/Cargo.toml; then run cargo test --manifest-path share/Cargo.toml; fi
if test -f aggregate/Cargo.toml; then run cargo test --manifest-path aggregate/Cargo.toml; fi
if test -f platform/Cargo.toml; then run cargo test --manifest-path platform/Cargo.toml; fi

# Node backend
if test -f backend/package.json; then
  if command -v pnpm >/dev/null 2>&1; then
    run pnpm -C backend test
  elif command -v npm >/dev/null 2>&1; then
    run bash -lc "cd backend && npm test"
  elif command -v yarn >/dev/null 2>&1; then
    run bash -lc "cd backend && yarn test"
  else
    echo "[ci] ERROR: backend/package.json exists but no package manager found (pnpm/npm/yarn)"
    exit 1
  fi
fi

# Android
if test -f android/gradlew; then
  run bash -lc "cd android && ./gradlew test"
fi

# --- No-op green guard ---
if test "$ran" -eq 0; then
  echo "[ci] ERROR: no-op green is forbidden. Add at least one real check."
  exit 1
fi

echo "[ci] green"
