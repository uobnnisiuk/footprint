`docs/constitution/30_testplan.md` のみ最小Diffで更新しました。

- 追加: 観測可能な新規Acceptance `TST-0008`（`capturedAt` が `type=string` かつ `format=date-time`）と、1行の観測コマンド例を追記しました（`docs/constitution/30_testplan.md:16`、`docs/constitution/30_testplan.md:17`）。
- 既存の `TST-0001`〜`TST-0004` はそのまま維持しています。
- `./ci.sh` を実行し、green を確認済みです。

※ 既に存在していた他ファイルの未コミット変更は触っていません。