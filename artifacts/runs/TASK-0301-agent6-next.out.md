完了しました。TASK-0300 の1サイクルを実施し、必要生成物を更新しました。

- スナップショット作成: `artifacts/packs/CP-0300-orchestrator.md`
- 次タスク3本を生成/更新: `artifacts/inbox/tasks/TASK-0200-agent4-ci.md`、`artifacts/inbox/tasks/TASK-0101-agent1-spec.md`、`artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`
- Mode B実行スクリプト生成/更新: `artifacts/outbox/RUN-0300-orchestrator.sh`
- 実行ログ更新: `artifacts/runs/TASK-0300-agent6-orchestrator.out.md`

Mode A（子プロセス codex）も試行しましたが、この環境では継続不能でした。
- 失敗1: `~/.codex/sessions` へのアクセスが permission denied
- 失敗2: workspace-local `CODEX_HOME` でも API接続が stream disconnected（＋subagents timeout）
- そのため Mode B を採用しています（内容は `artifacts/outbox/RUN-0300-orchestrator.sh`）。

検証結果:
- `./ci.sh` → green
- `bash -n artifacts/outbox/RUN-0300-orchestrator.sh` → OK
- 必須生成物（TST-0300-1〜4）を満たす状態です。

次はそのまま `bash artifacts/outbox/RUN-0300-orchestrator.sh` を実行すれば、agent4 → agent1 → agent5 の順で回せます。