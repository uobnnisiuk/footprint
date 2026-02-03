観測可能なAcceptanceを1本、最小Diffで追加しました。

- 追加: `docs/constitution/30_testplan.md:14` に `TST-0007` を追加（`kind` の enum が `beacon` / `manual` / `sensor` / `network` を含むことを観測）。
- 観測手順: `docs/constitution/30_testplan.md:15` に 1行コマンド例を追記。
- 維持: `TST-0001`〜`TST-0004`（既存 `TST-0005`,`TST-0006` 含む）に弱体化変更なし。
- 検証: `./ci.sh` 実行済み、green を確認しました。