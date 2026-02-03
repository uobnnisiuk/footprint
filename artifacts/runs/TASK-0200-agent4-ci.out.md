`./ci.sh` を最小Diffで整理して、単一入口のまま CI ゲートを育成しました。

- `always_on` / `optional` / `guard` の責務に分割し、読みやすい構造に変更 (`ci.sh:36`, `ci.sh:70`, `ci.sh:95`)。
- 失敗メッセージを `fail_with_hint` に統一し、`run` 経由のコマンド失敗でも必ず HINT が出るように変更 (`ci.sh:12`, `ci.sh:20`)。
- 「実装があるのにテスト未実行」を防ぐ方針を明文化し、manifest があるのにツール不足なら失敗＋HINT に変更 (`ci.sh:57`, `ci.sh:72`, `ci.sh:86`)。
- no-op green 禁止は `guard_checks` で維持 (`ci.sh:95`)。
- 実行ログを作成 (`artifacts/runs/TASK-0200-agent4-ci.out.md`)。

検証結果:
- `./ci.sh` 実行: green
- 失敗時 HINT 確認: `/tmp` で実行して `missing AGENTS.md` + `HINT` を確認（ログに記録済み）