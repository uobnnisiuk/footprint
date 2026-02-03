`./ci.sh` の責務構造（always_on / optional / guard）はそのままに、最小Diffで失敗時HINTを1箇所だけ具体化しました。

- `ci.sh:23` 付近で失敗コマンドを `printf '%q'` で整形し、HINTを `rerun exactly: ...` に変更（復旧の初動を明確化）。
- no-op green 禁止ガード（`ci.sh:120`）と「実装があるのにテストをスキップしない」方針は維持。
- 実行ログを `artifacts/runs/TASK-0200-agent4-ci.out.md` に更新。
- 検証として `./ci.sh` を実行し、green を確認済み。