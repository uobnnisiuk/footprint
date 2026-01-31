# Core Fact Spec (SSOT)

## Purpose
Persist trustworthy **facts** and explicit **blank** as SSOT (L0).

## Event Model (Append-only)
An Event is immutable and appended.

### Required
- event_id: unique
- committed_at: timestamp (most important)
- time_source: enum (SYSTEM | SERVER | MANUAL)  # pick final names later, semantics fixed
- blank: enum (NONE | BLANK_START | BLANK_CONTINUES)

### Optional
- location: {lat, lon, accuracy_m}
- location_source: enum (GNSS | CELL_WIFI | MANUAL | NONE)
- state: minimal observed status only
  - battery_percent
  - power_save_enabled
  - network_state (e.g., OFFLINE/ONLINE/UNKNOWN)
  - app_state (foreground/background/unknown) if observable

## Blank (断絶)
Blank is explicit, not inferred.
- After the last committed event, the system represents "no further facts exist"
- "Reason" is recorded only as observed state, never guessed

## Commit Semantics
- Last Known Good = last durably committed event
- "last_commit_time" must be derivable without inference

## Hard Rules
- No inference fields
- No overwrites (no update-in-place)
- Derived views belong to L2, not L0）
- IF-LOSSLESS-001 を参照

## Implementation Constraints (Bootstrap Slice)

最初のスライスでの技術選択。後続で変更可（mini RFC で決定）。

- IF-IMPL-001: インターフェースは CLI (stdin JSON → stdout JSON) で開始。API は後続。
- IF-IMPL-002: 永続化はローカルファイル (JSON Lines) で開始。DB は後続。
- IF-IMPL-003: 実装言語は Rust (`core/` crate)。
