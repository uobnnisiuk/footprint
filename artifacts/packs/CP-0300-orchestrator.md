# CP-0300 Orchestrator Snapshot

Generated: 2026-02-03T17:13:18+09:00
Role: agent6 (Integrator / Editor)

## 1) Repository snapshot (ls -R key points)

- Root: `AGENTS.md`, `ci.sh`, `docs/`, `artifacts/`, `core/`, `share/`, `aggregate/`, `platform/`
- `docs/`: `adr/`, `backlog/`, `constitution/`, `rfc/`
- `docs/constitution/`: `00_constitution.md`, `10_core_fact_spec.md`, `15_behavior_spec.md`, `20_share_envelope_spec.md`, `30_testplan.md`, `80_risks.md`, `contracts/trace.schema.json`
- `docs/rfc/`: `RFC-0000-template.md`, `RFC-0001-t3-activation-rules.md`, `RFC-0002-integrity-boundaries.md`, `RFC-0002-notification-queue-persistence.md`, `DEC-0001-t3-activation-rules.md`, `DEC-0002-notification-queue-persistence.md`
- `artifacts/inbox/tasks/`: `TASK-0000-template.md`, `TASK-0101-agent1-spec.md`, `TASK-0102-agent5-acceptance.md`, `TASK-0200-agent4-ci.md`, `TASK-0300-agent6-orchestrator.md`
- `artifacts/packs/`: `CP-0000-template.md`, `CP-0300-orchestrator.md`
- `artifacts/outbox/`: `RUN-0300-orchestrator.sh`
- `artifacts/runs/`: `TASK-0101-agent1-spec.out.md`, `TASK-0102-agent5-acceptance.out.md`, `TASK-0200-agent4-ci.out.md`, `TASK-0300-agent6-orchestrator.out.md`
- 実装ディレクトリ（`core/`, `share/`, `aggregate/`, `platform/`）は現状スケルトン中心（`internal/`, `tests/`, `tools/`）

## 2) `./ci.sh` summary

`./ci.sh` は以下の順に判定する。

1. repo rootへ移動して `[ci] start`
2. always_on: SSOT必須ファイル存在チェック（`AGENTS.md`, `docs/constitution/*`, `docs/rfc/RFC-0000-template.md`, `artifacts`テンプレ/契約）
3. `docs/constitution/contracts/trace.schema.json` のJSON妥当性を `python3` または `node` で検証
4. optional: 実装が存在する場合のみテストを実行
   - Rust: `core/share/aggregate/platform` の `Cargo.toml` があれば `cargo test`
   - backend: `backend/package.json` があれば `pnpm` / `npm` / `yarn` test
   - Android: `android/gradlew` があれば test
5. guard: `ran=0` の no-op green を失敗にする
6. 成功時 `[ci] green`

## 3) SSOT inventory

### constitution
- `docs/constitution/00_constitution.md`
- `docs/constitution/10_core_fact_spec.md`
- `docs/constitution/15_behavior_spec.md`
- `docs/constitution/20_share_envelope_spec.md`
- `docs/constitution/30_testplan.md`
- `docs/constitution/80_risks.md`
- `docs/constitution/contracts/trace.schema.json`

### rfc / decision
- `docs/rfc/DEC-0001-t3-activation-rules.md`
- `docs/rfc/DEC-0002-notification-queue-persistence.md`
- `docs/rfc/RFC-0000-template.md`
- `docs/rfc/RFC-0001-t3-activation-rules.md`
- `docs/rfc/RFC-0002-integrity-boundaries.md`
- `docs/rfc/RFC-0002-notification-queue-persistence.md`

## 4) Latest `./ci.sh` result

Run date: 2026-02-03

```text
[ci] start
[ci] phase: always_on
[ci] phase: optional
[ci] phase: guard
[ci] green
```

Exit code: 0

## 5) Known unresolved OPEN / Q

### OPEN (tracked)

`docs/constitution/10_core_fact_spec.md` の OPEN index に未解決として記載あり:

- OPEN-001
- OPEN-003
- OPEN-004
- OPEN-005
- OPEN-006
- OPEN-007
- OPEN-008
- OPEN-010
- OPEN-011
- OPEN-012
- OPEN-013

### Q

- `Q-xxx` 形式の未決は `docs/constitution`, `docs/rfc`, `artifacts/inbox/tasks` では未検出

## 6) Orchestration Mode A trial result

Detection:
- `codex` path: `/home/higuchi/.nvm/versions/node/v24.13.0/bin/codex`
- version: `codex-cli 0.91.0`

Attempt-1 (default `CODEX_HOME`):
- `codex exec --profile agent4 ...` を試行
- 失敗: `~/.codex/sessions` へのアクセスが sandbox 制約で拒否（permission denied）

Attempt-2 (workspace-local `CODEX_HOME` + local config):
- 起動はするが `https://api.openai.com/v1/responses` への接続で stream disconnected を繰り返し失敗
- `subagents` MCP startup timeout も発生

Conclusion:
- この実行環境では Mode A（子プロセス codex 実行）継続不能
- Mode B を採用し、`artifacts/outbox/RUN-0300-orchestrator.sh` を生成

## 7) Next tasks generated (order fixed)

1. `artifacts/inbox/tasks/TASK-0200-agent4-ci.md`
2. `artifacts/inbox/tasks/TASK-0101-agent1-spec.md`
3. `artifacts/inbox/tasks/TASK-0102-agent5-acceptance.md`

順序は `agent4 -> agent1 -> agent5`。各タスクは `./ci.sh` green を維持する前提。
