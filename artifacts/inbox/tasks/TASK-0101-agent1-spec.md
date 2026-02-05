# TASK-0101: agent1 — P-L1 OPEN-007 を DEC で確定（遭遇カプセル鍵）

## 目的（SR）
P-L1 の未決 OPEN のうち **OPEN-007（遭遇カプセルの暗号化鍵は誰が持つか）**を、mini RFC + DEC で確定し、SSOT の OPEN index / 参照整合まで最小Diffで反映する。

## SR（結論を先に固定）
- **SR**: 遭遇カプセルは **救助機関鍵（Authorized Rescue key）**で暗号化する
- 却下（短く）:
  - 地域鍵: 配布/ローテ/漏洩時影響が地域境界に依存し運用が破綻しやすい
  - 家族鍵: 家族不在・連絡不能で救助が詰まる（孤立者救助の主ルートを壊す）

## 前提（不変条件）
- IF-001: 仕様SSOTは `docs/constitution/` と `docs/rfc/`
- IF-002: OPEN-010（Authorized Rescue の定義）は **未決のまま維持**する（本タスクで確定しない）
- IF-003: 新規の実装要求は増やさない（仕様と参照整合のみ）
- IF-004: “通すために弱める変更”は禁止（CIやAcceptanceを弱めない）

## 変更範囲（Diff最小）
- 主対象:
  - `docs/rfc/DEC-0004-encounter-capsule-key.md`（存在すれば更新、無ければ新規作成）
  - `docs/constitution/10_core_fact_spec.md`（OPEN index を更新）
- 参照整合が必要な場合のみ:
  - `docs/constitution/80_risks.md`（OPEN-007 の状態と参照更新）
  - `docs/constitution/15_behavior_spec.md`（参照ズレのみ最小修正）

## 実施内容（順序固定）
1) `spike-backlog.md` の P-L1（OPEN-006/007/011）と、`80_risks.md` の OPEN-007 を確認
2) **mini RFC + DEC** を作る（最小）
   - RFC: 1〜2ページで「なぜ救助機関鍵か」「却下理由」「OPEN-010 との境界」を書く
   - DEC: SR を1行で固定し、仕様参照先（Behavior/Risk）を列挙
   - RFC/DEC のファイル名は既存規約に合わせる（DEC-0004 があるならそれを更新）
3) `10_core_fact_spec.md` の OPEN index で **OPEN-007 を解決済み**にし、参照先を DEC に固定
   - 他のOPENは “未決のまま” を維持
4) `80_risks.md` の OPEN 表で OPEN-007 を解決済みにし、C-008 の「鍵運用」箇所の参照を DEC に向ける
5) `./ci.sh` 実行で green を確認し、実施ログを `artifacts/runs/TASK-0101-agent1-spec.out.md` に残す

## 受け入れ条件（TST）
- TST-0101-1: OPEN-007 が DEC により確定し、SSOTから参照できる
- TST-0101-2: OPEN-010 は未決のまま維持される（境界が明記される）
- TST-0101-3: `./ci.sh` が green

## 実行コマンド（固定）
`codex exec --profile agent1 --color never --output-last-message artifacts/runs/TASK-0101-agent1-spec.out.md - < artifacts/inbox/tasks/TASK-0101-agent1-spec.md`

## 生成物
- `artifacts/runs/TASK-0101-agent1-spec.out.md`
- `docs/rfc/(RFC-xxxx ...).md`（最小）
- `docs/rfc/DEC-0004-encounter-capsule-key.md`
- `docs/constitution/10_core_fact_spec.md`（OPEN index 更新）
- （必要なら）`docs/constitution/80_risks.md` 参照整合

