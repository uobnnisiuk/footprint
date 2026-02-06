# DEC-0009: SSOT Rule ファイルリスト更新

## 日付
2026-02-06

## 決定内容（SR）

**SSOT Rule のファイルリストを3ファイルから6ファイルに更新する。**

### 更新後リスト（固定）

| # | ファイル | 役割 |
|---|---------|------|
| 1 | `docs/constitution/00_constitution.md` | 憲法（原則・境界・変更管理） |
| 2 | `docs/constitution/10_core_fact_spec.md` | L0 事実モデル仕様 |
| 3 | `docs/constitution/15_behavior_spec.md` | 挙動仕様（探索・通知・reveal・遭遇） |
| 4 | `docs/constitution/20_share_envelope_spec.md` | L1 エンベロープ仕様 |
| 5 | `docs/constitution/30_testplan.md` | Acceptance SSOT（受け入れ条件） |
| 6 | `docs/constitution/80_risks.md` | RISK / OPEN 管理台帳 |

### 変更管理への影響

上記6ファイルすべてに対し、既存の Change Control ルールが適用される：
- 変更には RFC が必須
- 実装タスク内での直接変更は禁止

## 理由

1. **実態との整合**: `15_behavior_spec.md` と `30_testplan.md` は既に自身で SSOT を宣言しており、リスト未記載は形式上の漏れ
2. **Change Control の明確化**: リストに含まれないと「explanatory or derived」扱いとなり、RFC なしで変更できてしまう
3. **最小差分**: ファイル内容は変更せず、リストの追加のみ

## 関連

- RFC-0007: `docs/rfc/RFC-0007-ssot-rule-file-list.md`
- 影響先: `docs/constitution/00_constitution.md`（SSOT Rule セクション）
