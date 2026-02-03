1サイクル分の Orchestrator PoC を実施して、必要生成物を作成しました。  
Mode A（子プロセス `codex` 実行）はこの sandbox では失敗したため、Mode B を必須出力として整備しています。

- スナップショット作成: `artifacts/packs/CP-0300-orchestrator.md`
- 次タスク生成:
  - `artifacts/inbox/tasks/TASK-0200-agent4-ci.md`（再生成）
  - `artifacts/inbox/tasks/TASK-0101-agent1-spec.md`
  - `artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`
- Mode B 実行スクリプト作成: `artifacts/outbox/RUN-0300-orchestrator.sh`（実行権限付与済み）
- 実行ログ作成: `artifacts/runs/TASK-0300-agent6-orchestrator.out.md`

Mode A 試行結果（要点）:
- `codex` は検出できたが、`~/.codex/sessions` 権限制約とネットワーク切断で実行継続不可。

検証:
- `./ci.sh` 実行: green
- `bash -n artifacts/outbox/RUN-0300-orchestrator.sh`: syntax OK

次にやること（判断不要）:
1. `bash artifacts/outbox/RUN-0300-orchestrator.sh` を実行  
2. 生成される各ログ（`artifacts/runs/TASK-0200...`, `TASK-0101...`, `TASK-0102...`）を確認