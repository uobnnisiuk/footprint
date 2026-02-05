`docs/constitution/30_testplan.md` に最小Diffで Acceptance を1本追加し、OPEN-007 の確定を観測可能化しました。

- `docs/constitution/30_testplan.md:22` に **TST-0011** を追加（`docs/rfc/DEC-0004-encounter-capsule-key.md` の SR が救助機関鍵 / Authorized Rescue 公開鍵であることを確認可能）。
- `docs/constitution/30_testplan.md:23` に 1行の観測コマンド例を追加：`grep -nE "決定内容（SR）|救助機関|Authorized Rescue 公開鍵" docs/rfc/DEC-0004-encounter-capsule-key.md`
- 既存 Acceptance（TST-0001〜TST-0010）は変更していません（弱体化なし）。
- `./ci.sh` を実行し、green を確認しました。