# CP-0300 Orchestrator Snapshot

Generated: 2026-02-03T16:24:29+09:00
Role: agent6 (Integrator / Editor)

## 1) Repository snapshot (ls -R key points)

- Root: `AGENTS.md`, `ci.sh`, `docs/`, `artifacts/`, `core/`, `share/`, `aggregate/`, `platform/`
- `docs/constitution/`: `00_constitution.md`, `10_core_fact_spec.md`, `15_behavior_spec.md`, `20_share_envelope_spec.md`, `30_testplan.md`, `contracts/trace.schema.json`
- `docs/rfc/`: `RFC-0000-template.md`, `RFC-0001-t3-activation-rules.md`, `RFC-0002-integrity-boundaries.md`, `RFC-0002-notification-queue-persistence.md`, `DEC-0001-t3-activation-rules.md`, `DEC-0002-notification-queue-persistence.md`
- `artifacts/inbox/tasks/`: `TASK-0000-template.md`, `TASK-0200-agent4-ci.md`, `TASK-0300-agent6-orchestrator.md`
- `artifacts/packs/`: `CP-0000-template.md`
- `artifacts/outbox/`, `artifacts/runs/`: present (initially empty)
- 実装ディレクトリ（`core/`,`share/`,`aggregate/`,`platform/`）は現在スケルトン中心（`internal/`,`tests/` 等）

## 2) `./ci.sh` summary

`./ci.sh` は次の順で判定する。

1. repo root へ移動し `[ci] start` を出力
2. Always-on 最小チェック（`AGENTS.md` / `docs/constitution/*` / `docs/rfc/RFC-0000-template.md` / `artifacts`テンプレファイル）
3. `docs/constitution/contracts/trace.schema.json` を python3 または node で JSON 妥当性検証
4. Optional checks（存在時のみ実行）
   - Rust: `core/share/aggregate/platform` の `Cargo.toml` があれば `cargo test`
   - Node backend: `backend/package.json` があれば `pnpm` / `npm` / `yarn` で test
   - Android: `android/gradlew` があれば test
5. no-op green ガード（`ran=0` なら失敗）
6. 正常時 `[ci] green`

## 3) SSOT inventory

### constitution
- `docs/constitution/00_constitution.md`
- `docs/constitution/10_core_fact_spec.md`
- `docs/constitution/15_behavior_spec.md`
- `docs/constitution/20_share_envelope_spec.md`
- `docs/constitution/30_testplan.md`
- `docs/constitution/contracts/trace.schema.json`

### rfc / decision
- `docs/rfc/RFC-0000-template.md`
- `docs/rfc/RFC-0001-t3-activation-rules.md`
- `docs/rfc/RFC-0002-integrity-boundaries.md`
- `docs/rfc/RFC-0002-notification-queue-persistence.md`
- `docs/rfc/DEC-0001-t3-activation-rules.md`
- `docs/rfc/DEC-0002-notification-queue-persistence.md`

## 4) Latest `./ci.sh` result

Run timestamp: 2026-02-03 (local)

Observed output:

```text
[ci] start
[ci] green
```

Exit code: 0

## 5) Known unresolved OPEN / Q

### OPEN (known)

- `OPEN-001`: オープン探索の存在粒度
- `OPEN-003`: 密/疎判定方法
- `OPEN-004`: 確度（スコア）の表示上の意味
- `OPEN-005`: オープン探索で履歴/パターン/ベースラインを返すか
- `OPEN-006`: relay と reveal の境界
- `OPEN-007`: 遭遇カプセル暗号化鍵モデル
- `OPEN-008`: サイレント遭遇中継のスパム対策
- `OPEN-010`: Authorized Rescue の定義
- `OPEN-011`: 通知内容の最小セット
- `OPEN-012`: プラットフォーム不在/通信断時の T2 補完
- `OPEN-013`: T3=ON時に粒度が変わる場合のプライバシー影響

### Q

- `Q-xxx` は `docs/constitution` / `docs/rfc` / `artifacts/inbox/tasks` では未検出（TASK本文中の説明文を除く）

## 6) Orchestration attempt (Mode A detection)

- `codex` binary: `/home/higuchi/.nvm/versions/node/v24.13.0/bin/codex`
- version: `codex-cli 0.91.0`

Attempt-1 (default CODEX_HOME):
- 失敗理由: `~/.codex/sessions` へのアクセス権限エラー（sandbox制約下）

Attempt-2 (workspace-local CODEX_HOME):
- 起動はしたが、`chatgpt.com/backend-api/codex/...` への接続で stream disconnected を繰り返し失敗
- MCP `subagents` startup timeout も発生

結論:
- この実行環境では Mode A（子プロセスcodex実行）は継続不能
- Mode B として `artifacts/outbox/RUN-0300-orchestrator.sh` を生成

## 7) Next tasks generated for the next cycle

実行順は `agent4 -> agent1 -> agent5` で固定。

1. `artifacts/inbox/tasks/TASK-0200-agent4-ci.md`
2. `artifacts/inbox/tasks/TASK-0101-agent1-spec.md`
3. `artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`

各タスクは `./ci.sh` green を維持する前提で記述済み。
