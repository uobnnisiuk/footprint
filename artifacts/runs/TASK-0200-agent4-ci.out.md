`./ci.sh` を単一入口のまま最小Diffで育成し、可読性と失敗時復旧の具体性を少し上げました。

- 重複していた `cd <dir> && ...` 実行を `run_in_dir` に集約し、`optional` フェーズの読みやすさを改善しました（`ci.sh:30`, `ci.sh:101`, `ci.sh:106`, `ci.sh:133`）。
- 失敗時HINTは1箇所だけ具体化し、Rust未導入時に「`cargo --version` が通る状態」を明示しました（`ci.sh:84`）。
- `always_on / optional / guard` の責務分離、`no-op green` 禁止、実装存在時の非スキップ方針は維持しています（`ci.sh:48`, `ci.sh:113`, `ci.sh:137`）。
- 実行ログを更新しました（`artifacts/runs/TASK-0200-agent4-ci.out.md:1`）。

検証:
- `bash -n ci.sh` ✅
- `./ci.sh` ✅ green（coreテスト実行含む、TST-0200-1/2/3充足）

次に進めるなら:
1. `git add ci.sh artifacts/runs/TASK-0200-agent4-ci.out.md && git commit`