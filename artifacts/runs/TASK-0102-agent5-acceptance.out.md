`docs/constitution/30_testplan.md` に最小Diffで Acceptance を1本追加しました。  
既存の TST-0001〜TST-0004（および既存TST全体）はそのまま維持しています。

- 追加: `TST-0009`  
  - 対象: `docs/constitution/contracts/trace.schema.json`  
  - 内容: `payload` が任意（`required` に含まれない）かつ `type=object` であることを**観測可能**に定義  
  - 観測手順: 1行の `python3 -c ...` コマンド例を追記
- 変更ファイル: `docs/constitution/30_testplan.md`
- 検証: `./ci.sh` 実行済みで green（全テスト通過）