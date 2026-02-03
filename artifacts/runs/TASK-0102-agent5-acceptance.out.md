対応しました。受け入れ条件を1本だけ具体化し、観測コマンドも追記しています。

- `docs/constitution/30_testplan.md:10` に `TST-0005` を追加  
  - 内容: `trace.schema.json` の `required` に `traceId/deviceId/capturedAt/kind` が含まれることを観測可能に定義
- `docs/constitution/30_testplan.md:11` に観測手順（1行コマンド）を追加
- 既存の `TST-0001`〜`TST-0004` は変更していません
- `./ci.sh` 実行結果は green（`[ci] green`）
- 実行ログを `artifacts/runs/TASK-0102-agent5-acceptance.out.md` に保存済みです