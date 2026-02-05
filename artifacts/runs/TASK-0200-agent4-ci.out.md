`./ci.sh` を最小Diffで育成し、失敗時HINTを1箇所だけ具体化しました。

- 変更: backend の package manager 未導入時HINTを具体化（`ci.sh:101`）
- 方針維持: `always_on / optional / guard` 構造、no-op green禁止、実装がある場合のテスト非スキップ方針は維持
- 実行ログ更新: `artifacts/runs/TASK-0200-agent4-ci.out.md`

検証結果:
- `bash -n ci.sh` OK
- `./ci.sh` green（TST-0200-1/2/3 を満たす）

必要なら次に `git add ci.sh artifacts/runs/TASK-0200-agent4-ci.out.md && git commit` で確定できます。