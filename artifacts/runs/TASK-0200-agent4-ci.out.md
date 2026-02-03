`./ci.sh` を最小Diffで育成して、責務構造を維持したまま読みやすさと復旧速度を上げました。

- `ci.sh:70` に `run_backend_tests_if_present` を追加し、optional の backend 分岐を整理（重複削減、挙動は維持）
- `ci.sh:101` で optional 側は新ヘルパー呼び出しに置換（「実装があるのにスキップしない」方針はそのまま）
- `ci.sh:23` の失敗時 HINT を1箇所だけ具体化（失敗コマンドの先行再実行→最小修正→`./ci.sh` 再実行）
- 実行ログを `artifacts/runs/TASK-0200-agent4-ci.out.md:1` に更新

検証:
- `./ci.sh` 実行結果: green（TST-0200-1）
- no-op green 禁止ガードは維持（`ci.sh:108`）