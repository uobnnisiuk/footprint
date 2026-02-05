OPEN-007 を最小Diffで確定しました。SR は「遭遇カプセルを Authorized Rescue 公開鍵で暗号化」に固定し、OPEN-010 は未決のまま維持しています。

- mini RFC を追加: `docs/rfc/RFC-0004-encounter-capsule-key.md:1`
- DEC に RFC 参照を追加: `docs/rfc/DEC-0004-encounter-capsule-key.md:37`
- OPEN index の OPEN-007 を DEC 明示に更新: `docs/constitution/10_core_fact_spec.md:129`
- C-008 の鍵運用参照を DEC/OPEN-010 境界へ更新: `docs/constitution/80_risks.md:119` `docs/constitution/80_risks.md:121`
- 実施ログを更新: `artifacts/runs/TASK-0101-agent1-spec.out.md:1`

検証:
- `./ci.sh` 実行済み（green / exit 0）

次の自然な一手:
1. この変更をコミットする