#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

mkdir -p artifacts/runs artifacts/outbox

echo "[run-0300] start $(date -Iseconds)"

if ! command -v codex >/dev/null 2>&1; then
  echo "[run-0300] ERROR: codex command not found"
  exit 1
fi

run_task() {
  local profile="$1"
  local task_file="$2"
  local out_file="$3"

  echo "[run-0300] run ${task_file} with ${profile}"
  codex exec --profile "${profile}" --color never \
    --output-last-message "${out_file}" \
    - < "${task_file}"

  echo "[run-0300] verify ./ci.sh after ${task_file}"
  ./ci.sh
}

# Preflight
./ci.sh

# Ordered orchestration cycle (agent4 -> agent1 -> agent5)
run_task agent4 artifacts/inbox/tasks/TASK-0200-agent4-ci.md artifacts/runs/TASK-0200-agent4-ci.out.md
run_task agent1 artifacts/inbox/tasks/TASK-0101-agent1-spec.md artifacts/runs/TASK-0101-agent1-spec.out.md
run_task agent5 artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md artifacts/runs/TASK-0102-agent5-acceptance.out.md

echo "[run-0300] done $(date -Iseconds)"
