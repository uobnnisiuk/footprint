#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

mkdir -p artifacts/runs artifacts/outbox

echo "[run-0300] start $(date -Iseconds)"

if ! command -v codex >/dev/null 2>&1; then
  echo "[run-0300] ERROR: codex command not found"
  echo "[run-0300] HINT : install codex CLI and rerun this script"
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

echo "[run-0300] preflight ./ci.sh"
./ci.sh

# --- Add: allow running specific tasks by args (minimal) ---
TASK_NAMES=()
if [ "$#" -gt 0 ]; then
  for t in "$@"; do
    TASK_NAMES+=("${t}")
  done
else
  # fallback to existing default tasks (keep current behavior)
  TASK_NAMES=(
    "TASK-0200-agent4-ci"
    "TASK-0101-agent1-spec"
    "TASK-0102-agent5-acceptance"
  )
fi
# --- End add ---

# run task with profile inferred from task name
run_task_inferred() {
  local task_name="$1"
  local task_file="artifacts/inbox/tasks/${task_name}.md"
  local out_file="artifacts/runs/${task_name}.out.md"
  local profile

  # infer profile from task name (e.g., TASK-0101-agent1-spec -> agent1)
  profile=$(echo "$task_name" | sed -n 's/.*-\(agent[0-9]*\)-.*/\1/p')

  if [ -z "$profile" ]; then
    echo "[run-0300] ERROR: cannot infer profile from task name: $task_name"
    exit 1
  fi

  run_task "$profile" "$task_file" "$out_file"
}

# run tasks in order
for task_name in "${TASK_NAMES[@]}"; do
  run_task_inferred "$task_name"
done

echo "[run-0300] done $(date -Iseconds)"
