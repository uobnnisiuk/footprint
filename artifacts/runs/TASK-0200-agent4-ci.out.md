`TASK-0200` を実施して、`./ci.sh` を最小Diffで育成しました。

- `ci.sh:96` で Rust の `Cargo.toml` 実行箇所を配列+ループ化し、`optional` の重複を整理（責務は維持）。
- `ci.sh:110` で Android テスト前に `java` 有無を明示チェックし、失敗時 HINT を具体化。
- no-op green 禁止ガードは維持（`ci.sh:117`）。
- 実行ログを更新（`artifacts/runs/TASK-0200-agent4-ci.out.md`）。

検証:
- `./ci.sh` は green を確認済み。