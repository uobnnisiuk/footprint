```md
# TASK-0200: agent4 — CIゲート育成（PoC cycle-1）

## 目的（SR）
`./ci.sh` を単一入口のまま、実装追加に追随しやすい構造へ最小Diffで整備する。

## 前提（不変条件）
- IF-001: CIの入口は `./ci.sh` のみ
- IF-002: no-op green 禁止
- IF-003: SSOT構造（docs/constitution, docs/rfc, artifacts）を壊さない
- IF-004: Acceptance弱体化は禁止（必要なら mini RFC + DEC）

## 変更範囲（Diff最小）
- 原則 `./ci.sh` のみ
- 必要時のみ補助ドキュメント1枚（例: `docs/constitution/90_devex_ci.md`）

## 実施内容（順序固定）
1) `./ci.sh` を `always_on` / `optional` / `guard` の責務で読みやすく整理する
2) 失敗メッセージに HINT を統一し、次アクションが即分かる形にする
3) 「実装が存在するのにテスト未実行」を避ける方針を明文化する
4) `./ci.sh` を実行し green を確認する

## 受け入れ条件（TST）
- TST-0200-1: `./ci.sh` が green
- TST-0200-2: 失敗時に HINT が出る
- TST-0200-3: no-op green が引き続き禁止される

## 生成物
- 実行ログ: `artifacts/runs/TASK-0200-agent4-ci.out.md`
- 変更ファイル（最小Diff）
```
