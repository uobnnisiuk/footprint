```md
# TASK-0102: agent5 — acceptance強化（観測可能な1本追加）

## 目的（SR）
`docs/constitution/30_testplan.md` に「観測可能なAcceptance」を最低1本追加し、受け入れ条件を一段だけ具体化する。

## 前提（不変条件）
- IF-001: AcceptanceのSSOTは `docs/constitution/30_testplan.md`
- IF-002: “通すために弱める変更”は禁止
- IF-003: 実装内部ではなく観測可能な挙動で書く

## 変更範囲（Diff最小）
- 対象: `docs/constitution/30_testplan.md` のみ

## 実施内容
1) 既存TST（TST-0001〜0004）を維持
2) 新規TSTを1本追加（例: schema必須項目の観測可能チェック）
   - 例: `trace.schema.json` の `required` に `traceId/deviceId/capturedAt/kind` が含まれることをコマンドで観測できる形で記述
3) 観測手順（コマンド例）を1〜2行で追記
4) `./ci.sh` を実行し green を確認

## 受け入れ条件（TST）
- TST-0102-1: `30_testplan.md` に観測可能な新規Acceptanceが1本追加される
- TST-0102-2: 既存Acceptanceを弱めない
- TST-0102-3: `./ci.sh` が green

## 生成物
- 実行ログ: `artifacts/runs/TASK-0102-agent5-acceptance.out.md`
- 変更ファイル（最小Diff）
```
