`docs/constitution/30_testplan.md` に観測可能なAcceptanceを1本追加しました。  
- 追加: `TST-0006`（`trace.schema.json` が `additionalProperties: false` で未知フィールド拒否になることをコマンドで観測）
- 観測手順例（1行）も同ファイル内に追記済み
- 既存の `TST-0001`〜`TST-0005` は維持（弱体化なし）

検証:
- `./ci.sh` 実行結果は green
- 実行ログ: `artifacts/runs/TASK-0102-agent5-acceptance.out.md`