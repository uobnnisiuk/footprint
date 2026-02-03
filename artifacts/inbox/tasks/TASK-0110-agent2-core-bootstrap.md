# TASK-0110: agent2 — core bootstrap slice（CLI + JSONL 永続化）

## Role
Assignee: agent2
Role: Worker

## References (SSOT)
- docs/constitution/00_constitution.md
- docs/constitution/10_core_fact_spec.md
- docs/constitution/contracts/trace.schema.json
- docs/constitution/30_testplan.md
- docs/rfc/RFC-0002-integrity-boundaries.md（L0/L1境界）

## Goal (One sentence)
Trace(JSON) を1件受け取り、L0の事実として canonical 化して JSONL に durable append し、観測可能な結果(JSON)を stdout に返す。

## In-scope
- `core/` crate を追加/実装（存在する場合は最小diffで拡張）
- CLI（stdin JSON → stdout JSON）
- ローカルファイル JSON Lines（append-only / durable）
- 追加テスト（`cargo test --manifest-path core/Cargo.toml` で観測できること）

## Out-of-scope (Hard)
- L1（share）以降の実装・署名・hash chain（IF-INTEG-001により禁止）
- API/HTTP サーバ、DB（SQLite等）
- 仕様の再解釈や勝手な拡張（OPENを確定しない）

## Allowed Touch Area
- Files/modules:
  - core/** （新規追加を含む）
- 原則、docs/ や ci.sh は触らない（必要なら Q-xxx を残して止まらず IF-xxx で前進）

## Design Constraints (Non-negotiable)
- L0 は inference しない（観測事実のみ）。上書き禁止（append-only）。
- L0 は暗号学的 integrity を持たない（署名・hash chain は実装しない）。
- canonicalization: JSONオブジェクトキーはアルファベット順（ネストも再帰）、時刻はUTCミリ秒精度ISO8601（Z固定）。
- 入力は contracts/trace.schema.json に準拠（未知フィールドは拒否）。

## Implementation Spec (Bootstrap)
### CLI behavior
- 標準入力から Trace JSON（単一オブジェクト）を読む。
- 正常時: stdout に JSON を出力し exit 0。
- 異常時: stdout に error JSON を出力し exit 1（stderr 追加ログは可だが stdout JSON を優先）。

### Storage
- JSONL 追記（1行 = 1イベント、末尾に `\n`）。
- durable: `flush` + `sync_data`（もしくは同等）で「コミットした事実」を担保。
- 保存先指定:
  - 優先1: CLI引数 `--store <path>`
  - 優先2: 環境変数 `FOOTPRINT_CORE_STORE_PATH`
  - デフォルト: `$XDG_STATE_HOME/footprint/core/events.jsonl`（なければ `~/.local/state/footprint/core/events.jsonl`）

### L0 record shape
- 入力 Trace を、L0 の Event として以下へ正規化して保存（10_core_fact_spec に整合）:
  - `event_id` = traceId
  - `committed_at` = capturedAt（UTCミリ秒に正規化）
  - `time_source` = kind から決定（固定ルール）
    - kind == "manual" -> "MANUAL"
    - それ以外 -> "SYSTEM"
  - `blank` = "NONE"
  - `device_id` = deviceId（観測事実として保持）
  - `kind` = kind（観測事実として保持）
  - `payload` = payload（存在すればそのまま保持。null も明示可能）
- 追加の推測・補完は禁止（例: location/state を推測しない）。

### Output JSON (stdout)
例（フィールド名はこのまま固定でよい）:
```json
{
  "ok": true,
  "event_id": "...",
  "committed_at": "2026-02-01T12:34:56.789Z",
  "store_path": "...",
  "bytes_appended": 123
}
````

失敗例:

```json
{
  "ok": false,
  "error": { "code": "invalid_input", "message": "..." }
}
```

## Tests (must be observable in CI)

* integration test:

  * temp file を store に指定して CLI を起動
  * Trace JSON を stdin に渡す
  * exit 0 / stdout の `ok:true` を確認
  * store に 1行追記され、JSON が parse でき、`event_id/committed_at/blank` 等が存在すること
  * `committed_at` が `...Z` + ミリ秒（3桁）になっていること
* negative test:

  * Trace に未知フィールドを混ぜる -> exit 1 / `ok:false` になること

## Deliverables

* Code:

  * core crate（lib + bin どちらでも可。少なくとも bin が必要）
* Tests:

  * 上記 integration / negative が通ること
* Docs:

  * 追加不要（READMEが必要なら core/README.md 程度で最小）

## Definition of Done

* `cargo test --manifest-path core/Cargo.toml` が green
* `./ci.sh` が green
* SSOT（docs/constitution/*）は変更しない
* L0/L1 境界（暗号はL1）を破らない

