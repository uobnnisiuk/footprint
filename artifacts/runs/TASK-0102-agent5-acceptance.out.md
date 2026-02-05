`docs/constitution/30_testplan.md` に最小Diffで Acceptance を1本追加しました。

- `docs/constitution/30_testplan.md` に `TST-0010` を追加し、`traceId` / `deviceId` が `minLength=1`（空文字不可）であることを観測可能な条件として定義しました。
- 同項目に 1 行の観測コマンド例（`python3 -c ...`）を追記しました。
- 既存の `TST-0001`〜`TST-0009` は変更していません（弱体化なし）。
- `./ci.sh` を実行し、green を確認しました。  
- 追加した観測コマンドも単体実行で成功確認済みです。