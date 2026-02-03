TASK-0300 (agent6) を 1サイクル実施し、以下を完了。

- `artifacts/packs/CP-0300-orchestrator.md` を更新（リポ構造、`./ci.sh`要点、SSOT一覧、最新CI結果、OPEN/Q、Mode A試行結果を記録）
- 次タスク3本を生成/更新
  - `artifacts/inbox/tasks/TASK-0200-agent4-ci.md`
  - `artifacts/inbox/tasks/TASK-0101-agent1-spec.md`
  - `artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`
- Mode B 実行スクリプトを生成/更新
  - `artifacts/outbox/RUN-0300-orchestrator.sh`

Mode A 試行（child codex）:
1) default `CODEX_HOME` + `--profile agent4`
   - 失敗: `/home/higuchi/.codex/sessions` permission denied
2) workspace local `CODEX_HOME` + local config
   - 起動はするが `https://api.openai.com/v1/responses` で stream disconnected
   - `subagents` MCP startup timeout も発生

判定:
- この実行環境では Mode A 継続不能
- Mode B（`artifacts/outbox/RUN-0300-orchestrator.sh`）を採用

検証:
- `./ci.sh` 実行結果: green
- `bash -n artifacts/outbox/RUN-0300-orchestrator.sh`: OK

次アクション（判断不要）:
1) `bash artifacts/outbox/RUN-0300-orchestrator.sh`
2) `artifacts/runs/TASK-0200-agent4-ci.out.md`
3) `artifacts/runs/TASK-0101-agent1-spec.out.md`
4) `artifacts/runs/TASK-0102-agent5-acceptance.out.md`
