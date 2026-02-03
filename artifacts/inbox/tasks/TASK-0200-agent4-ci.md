```md
# TASK-0200: agent4 — CIゲート育成（cycle-2 minimal）

## 目的（SR）
`./ci.sh` を単一入口のまま最小Diffで育成し、失敗時の復旧速度と再現性を上げる。

## 前提（不変条件）
- IF-001: 入口は常に `./ci.sh` のみ
- IF-002: no-op green 禁止
- IF-003: SSOT構造（`docs/constitution`, `docs/rfc`, `artifacts`）を壊さない
- IF-004: Acceptance弱体化は禁止（必要なら mini RFC + DEC）

## 変更範囲（Diff最小）
- 原則 `./ci.sh` のみ
- 必要な場合だけ docs 1ファイル以内を追加/更新

## 実施内容（順序固定）
1) `./ci.sh` の責務（always_on / optional / guard）を崩さず、重複や読みづらさがあれば最小整理
2) 失敗時HINTが具体的か確認し、不足があれば1箇所だけ改善
3) 「実装があるのにテストをスキップしない」方針を維持（緩和しない）
4) `./ci.sh` 実行で green を確認

## 受け入れ条件（TST）
- TST-0200-1: `./ci.sh` が green
- TST-0200-2: 失敗時に次アクションが分かる HINT を維持
- TST-0200-3: no-op green 禁止が維持される

## 実行コマンド（固定）
`codex exec --profile agent4 --color never --output-last-message artifacts/runs/TASK-0200-agent4-ci.out.md - < artifacts/inbox/tasks/TASK-0200-agent4-ci.md`

## 生成物
- `artifacts/runs/TASK-0200-agent4-ci.out.md`
- 変更ファイル（最小Diff）
```
