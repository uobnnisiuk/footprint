# TASK-0300: agent6 — Orchestrator PoC（1サイクル回す）

## 目的（SR）
「あなた（人間）の追加判断なし」で開発を前に進めるために、agent6 をオーケストレーターとして扱い、
(1) 現状スナップショットの作成 → (2) 次タスク生成 → (3) 実行（可能なら）/ 実行手順書の生成（必須）
までを 1サイクルで完了させる。

## 前提（不変条件）
- IF-001: 入口は `./ci.sh` だけ。完了条件は常に `./ci.sh` green。
- IF-002: SSOTパスは AGENTS.md の定義に従う（docs/constitution・docs/rfc・artifacts）
- IF-003: 自走＝「次の質問をせずに前へ進む」。ブロックは Q/IF で仮定して前進。
- IF-004: “通すために弱める変更”は禁止（必要なら RFC/DEC）。

## 変更範囲（Diff最小）
- AGENTS.md の追記は必要最小限（このPoC運用の追記のみ）
- 生成物は artifacts/* のみ（既存SSOTを壊さない）

## 実施内容（この順で）
1) スナップショットを `artifacts/packs/CP-0300-orchestrator.md` として作る
   - 現在のリポ構造（ls -R の要点）
   - `./ci.sh` の内容要点
   - SSOT一覧（docs/constitution/*.md, docs/rfc/*.md）
   - 直近の実行結果（./ci.sh 実行ログの要点）
   - 既知の未決（OPEN/Qがあれば）

2) 次に回すべきタスクを生成する（artifacts/inbox/tasks/ に作成）
   - TASK-0200-agent4-ci（まだ未実施なら。CIの育成）
   - TASK-0101-agent1-spec（specの最小更新：OPEN整理）
   - TASK-0102-agent5-acceptance（testplanの強化：観測可能なAcceptance 1本追加）
   ※内容は “最小diff” で、各タスクは `./ci.sh` green を壊さない順序で。

3) オーケストレーションを試みる
   - 可能なら：この実行環境で `codex` を子プロセスとして呼べるか検出し、呼べる場合は agent4→agent1→agent5 を実行して進捗を反映する
   - 不可能なら（または危険なら）：Mode B として `artifacts/outbox/RUN-0300-orchestrator.sh` を生成し、
     人間がコピペ実行するだけで同じ順序で回るようにコマンド列を出力する（必須）

## 受け入れ条件（TST）
- TST-0300-1: `artifacts/packs/CP-0300-orchestrator.md` が生成されている
- TST-0300-2: 次タスクが `artifacts/inbox/tasks/` に生成されている（少なくとも3本）
- TST-0300-3: Mode Aが無理でも、Mode B の実行スクリプトが `artifacts/outbox/` に生成されている
- TST-0300-4: 生成物だけで「次に何を実行すれば良いか」が判断不要で分かる

## 生成物
- `artifacts/packs/CP-0300-orchestrator.md`
- `artifacts/inbox/tasks/TASK-0200-agent4-ci.md`（必要なら再生成/上書き）
- `artifacts/inbox/tasks/TASK-0101-agent1-spec.md`
- `artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`
- `artifacts/outbox/RUN-0300-orchestrator.sh`
- 実行ログ: `artifacts/runs/TASK-0300-agent6-orchestrator.out.md`
