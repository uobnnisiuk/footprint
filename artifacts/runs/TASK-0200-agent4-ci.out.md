`./ci.sh` を最小Diffで整えて、責務構造（always_on / optional / guard）を維持したまま可読性と復旧ヒントを改善しました。

- `always_on_checks` の必須ファイル検証を配列+ループ化して重複を削減（`ci.sh:39`）。
- 失敗時HINTを1箇所だけ具体化し、再実行アクションを明示（`ci.sh:101`）。
- no-op green 禁止と「実装があるならテストをスキップしない」方針はそのまま維持（`ci.sh:104`, `ci.sh:128`）。
- 実行ログを更新（`artifacts/runs/TASK-0200-agent4-ci.out.md`）。

検証:
- `bash -n ci.sh` ✅
- `./ci.sh` ✅（green）

次に進めるなら、`git add ci.sh artifacts/runs/TASK-0200-agent4-ci.out.md && git commit` で確定できます。