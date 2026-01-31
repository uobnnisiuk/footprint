# TASK-0200: agent4 — CIゲート（./ci.sh）を“実装増加に追随できる形”へ整備する

## 目的（SR）
`./ci.sh` を、現状のbootstrap最小チェックを維持したまま、リポジトリに今後追加される実装（Rust/Node/Android等）に追随して段階的にゲートを増やせる形に整備する。入口は常に `./ci.sh` のみ。

## 前提（不変条件）
- IF-001: CIの単一エントリポイントは `./ci.sh`（他の入口を増やさない）
- IF-002: no-op green 禁止（実質何も走っていない green は禁止）
- IF-003: `docs/constitution/` と `docs/rfc/` と `artifacts/` のSSOT構造は維持する（勝手に移動しない）
- IF-004: 受け入れSSOTは `docs/constitution/30_testplan.md`（弱める変更は禁止。必要ならRFC/DEC）

## 変更範囲（Diff最小）
- 変更は `./ci.sh` と、必要なら “CIの使い方” を説明する短いドキュメント（例: `docs/constitution/90_devex_ci.md`）のみ
- 既存のbootstrapチェック（ファイル存在、JSON妥当性）は残す

## 実施内容（具体）
1) `./ci.sh` を読みやすく整理する
   - “Always-on minimal checks”（bootstrapで必ず走る） と “Optional project checks”（存在するものだけ） を明確に分離
   - 失敗時のメッセージに「次に何をすればよいか（HINT）」を必ず付ける

2) 「実装が生えたら自動でゲートが増える」仕組みにする
   - 例: ディレクトリ検出でテストを有効化（既にある方式を踏襲）
   - Rust/Node/Android を想定した拡張ポイントをコメントではなく関数化して追加しやすくする
     - 例: check_rust(), check_node(), check_android() のように分離（名前は任意だが最小diffで）

3) “環境不足” で詰まりにくくする
   - python3/node/pnpm 等が無い場合の扱いを明確にする
   - ただし「必要なものが無いのに黙ってgreen」は禁止。missingの場合は
     - (A) bootstrap必須チェックは hard fail
     - (B) Optional checks は WARN + ran=1 で許容（ただし実装が存在する場合は原則 hard fail）
   - どのケースをhard failにするかを `./ci.sh` 内のコメントで明文化する

4) `./ci.sh` の実行方法を最小限ドキュメント化する（任意だが推奨）
   - 追加するなら `docs/constitution/90_devex_ci.md`
   - 内容は「ローカルで ./ci.sh を回す」「失敗の典型と対処」「依存ツール（python3/node等）」程度で短く

## 受け入れ条件（TST）
- TST-0200-1: `./ci.sh` が現状のリポジトリで green のまま
- TST-0200-2: `./ci.sh` の出力が、失敗時に次の手が分かる（HINTが出る）
- TST-0200-3: no-op green は引き続き禁止されている
- TST-0200-4: “実装ディレクトリが存在するのにテストが走らない” 状態を作らない（方針を明文化）

## 生成物
- 実行ログ: `artifacts/runs/TASK-0200-agent4-ci.out.md`
- 変更ファイル（diff最小）
