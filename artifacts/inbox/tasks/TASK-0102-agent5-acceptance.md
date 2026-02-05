## TASK-0102: agent5 — acceptance強化（OPEN-007 確定を観測可能に）

## 目的（SR）
`docs/constitution/30_testplan.md` に、**OPEN-007（遭遇カプセル鍵）の確定が観測可能**な Acceptance を1本だけ追加し、最小Diffで強化する。

## 前提（不変条件）
- IF-001: AcceptanceのSSOTは `docs/constitution/30_testplan.md`
- IF-002: “通すために弱める変更”は禁止
- IF-003: 実装内部ではなく観測可能な条件で定義する

## 変更範囲（Diff最小）
- 対象は `docs/constitution/30_testplan.md` のみ

## 実施内容（順序固定）
1) 既存TSTを維持（弱体化しない）
2) 新規TSTを1本追加:
   - **TST-0011**: `docs/rfc/DEC-0004-encounter-capsule-key.md` において、遭遇カプセル鍵の SR が **救助機関鍵**であることを観測できる
3) 観測手順（1行コマンド例）を追記（例: `grep -n`）
4) `./ci.sh` 実行で green を確認

## 受け入れ条件（TST）
- TST-0102-1: `30_testplan.md` に観測可能な新規Acceptanceが1本追加される
- TST-0102-2: 既存Acceptanceを弱めない
- TST-0102-3: `./ci.sh` が green

## 実行コマンド（固定）
`codex exec --profile agent5 --color never --output-last-message artifacts/runs/TASK-0102-agent5-acceptance.out.md - < artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`

## 生成物
- `artifacts/runs/TASK-0102-agent5-acceptance.out.md`
- 変更ファイル（最小Diff）
 TASK-0102: agent5 — acceptance強化（OPEN-007 確定を観測可能に）

## 目的（SR）
`docs/constitution/30_testplan.md` に、**OPEN-007（遭遇カプセル鍵）の確定が観測可能**な Acceptance を1本だけ追加し、最小Diffで強化する。

## 前提（不変条件）
- IF-001: AcceptanceのSSOTは `docs/constitution/30_testplan.md`
- IF-002: “通すために弱める変更”は禁止
- IF-003: 実装内部ではなく観測可能な条件で定義する

## 変更範囲（Diff最小）
- 対象は `docs/constitution/30_testplan.md` のみ

## 実施内容（順序固定）
1) 既存TSTを維持（弱体化しない）
2) 新規TSTを1本追加:
   - **TST-0011**: `docs/rfc/DEC-0004-encounter-capsule-key.md` において、遭遇カプセル鍵の SR が **救助機関鍵**であることを観測できる
3) 観測手順（1行コマンド例）を追記（例: `grep -n`）
4) `./ci.sh` 実行で green を確認

## 受け入れ条件（TST）
- TST-0102-1: `30_testplan.md` に観測可能な新規Acceptanceが1本追加される
- TST-0102-2: 既存Acceptanceを弱めない
- TST-0102-3: `./ci.sh` が green

## 実行コマンド（固定）
`codex exec --profile agent5 --color never --output-last-message artifacts/runs/TASK-0102-agent5-acceptance.out.md - < artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`

## 生成物
- `artifacts/runs/TASK-0102-agent5-acceptance.out.md`
- 変更ファイル（最小Diff）

