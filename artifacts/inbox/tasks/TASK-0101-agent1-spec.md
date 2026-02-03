```md
# TASK-0101: agent1 — spec最小更新（OPEN index整備）

## 目的（SR）
`docs/constitution/10_core_fact_spec.md` の OPEN index を最小Diffで整備し、未決の参照先を一目で追える状態にする。

## 前提（不変条件）
- IF-001: 仕様SSOTは `docs/constitution/` と `docs/rfc/`
- IF-002: 未決を勝手に確定しない（未決は OPEN のまま明示）
- IF-003: 新規の実装要求は増やさない
- IF-004: 仕様確定が必要な変更は mini RFC + DEC

## 変更範囲（Diff最小）
- 主対象: `docs/constitution/10_core_fact_spec.md`
- 参照整合が必要な場合のみ: `docs/constitution/15_behavior_spec.md`, `docs/constitution/80_risks.md`

## 実施内容（順序固定）
1) 未解決OPEN（OPEN-001,003,004,005,006,007,008,010,011,012,013）を再確認
2) `10_core_fact_spec.md` の OPEN index を更新（各OPENに参照先を明記）
3) 必要なら `15_behavior_spec.md` / `80_risks.md` の参照ズレだけ最小修正
4) `./ci.sh` 実行で green を確認

## 受け入れ条件（TST）
- TST-0101-1: `10_core_fact_spec.md` から未決OPENの所在が追える
- TST-0101-2: 未決を勝手に確定しない
- TST-0101-3: `./ci.sh` が green

## 実行コマンド（固定）
`codex exec --profile agent1 --color never --output-last-message artifacts/runs/TASK-0101-agent1-spec.out.md - < artifacts/inbox/tasks/TASK-0101-agent1-spec.md`

## 生成物
- `artifacts/runs/TASK-0101-agent1-spec.out.md`
- 変更ファイル（最小Diff）
```
