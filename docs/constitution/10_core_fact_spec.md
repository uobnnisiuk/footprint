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
- Derived views belong to L2, not L0
- IF-LOSSLESS-001 を参照

## Canonicalization（正規化）

L0 は「ハッシュ入力としての正」を定義する。L1 が hash chain を構築する際、迷わず同じ bytes をハッシュできるようにするため。

### 正規化ルール

- **フィールド順序**: アルファベット順（ネストも再帰的に）
- **エンコーディング**: UTF-8（BOM なし）
- **時刻表現**: ISO 8601 拡張形式（例: `2026-02-01T12:34:56.789Z`）、ミリ秒精度、UTC 固定
- **数値**: 整数は符号付き64bit、浮動小数点は IEEE 754 double、不要な末尾ゼロは削除
- **null/省略**: null は省略ではなく明示的に `null` として出力
- **配列**: 順序保持

### canonical(event) の定義

```
canonical(event) = serialize_json_canonical(event)
```

L1 は `canonical(event)` を入力として hash chain を構築する。

### L0 の暗号学的 integrity

**L0 は暗号学的 integrity（署名・hash chain）を持たない。**

- append-only は「意味論（上書き禁止）」として保証
- 改ざん検出は L1 の envelope に委譲
- 根拠: RFC-0002 (IF-INTEG-001)

### 不変条件

- **IF-CANON-001**: フィールド順序はアルファベット順で固定
- **IF-CANON-002**: 時刻は UTC・ミリ秒精度で固定
- **IF-INTEG-001**: L0 は暗号学的 integrity を持たない（正規化の定義までが責務）

## T3 Activation Rules (災害モード発動ルール)

T3（災害モード）は、通常運用から災害時運用へ挙動を切り替えるためのトリガーである。
詳細は RFC-0001 を参照。

### T3状態の分離

| 状態 | 意味 | 挙動 |
|------|------|------|
| T3=OFF | 平時 | 通常運用 |
| T3=RECOMMENDED | T3推奨 | UI通知・省電力準備のみ。本格的な挙動変更は行わない |
| T3=ON | T3確定 | 災害モード挙動に本格切替 |

### 発動優先順位（上ほど強い）

1. **手動フラグ** → T3=ON（即確定）
2. **公的アラート受信/取得**（該当エリア内） → T3=ON
3. **エリア自動判別**（公的アラート由来のジオフェンス） → T3=ON
4. **端末状況ヒューリスティック**（通信断/停電っぽさ等） → T3=RECOMMENDED のみ

### TTL（Time-To-Live）

- 手動起動: 無期限（手動解除が基本）。誤操作対策として「一定時間後に確認」通知を出す
- 公的アラート起動: 6〜24時間。アラート更新で自動延長、更新なしで自動失効
- 端末ヒューリスティック起動（RECOMMENDED）: 1〜2時間。状況改善で自動解除

### 不変条件

- **IF-T3-001**: 手動フラグは最上位の確定トリガとして固定する
- **IF-T3-002**: 公的アラートを取得できた場合のみ自動ONを許可する（必須依存にはしない）
- **IF-T3-003**: 端末状況ヒューリスティック単独ではT3=ONにしない（推奨のみ）
- **IF-T3-004**: 自動ONは永遠に残らない。TTLで自動延長/自動失効する

## Implementation Constraints (Bootstrap Slice)

最初のスライスでの技術選択。後続で変更可（mini RFC で決定）。

- IF-IMPL-001: インターフェースは CLI (stdin JSON → stdout JSON) で開始。API は後続。
- IF-IMPL-002: 永続化はローカルファイル (JSON Lines) で開始。DB は後続。
- IF-IMPL-003: 実装言語は Rust (`core/` crate)。

## OPEN Index (Unresolved Tracking)

この章は OPEN の所在索引のみを定義する。ここで仕様確定は行わない。
再確認対象は `OPEN-001,003,004,005,006,007,008,010,011,012,013`。
状態は `未解決: OPEN-001,003,004,005,008,010,012,013 / 解決済み: OPEN-006,007,011` とする。
追跡は「一次参照（`docs/constitution/`）→ 補助参照（`docs/rfc/`）」の順で行う。

| ID | 未決テーマ | 一次参照（constitution） | 補助参照（rfc/dec） |
|----|------------|--------------------------|----------------------|
| OPEN-001 | オープン探索の「存在」の粒度（エリア/時間窓） | `docs/constitution/15_behavior_spec.md` Sections 1.2, 7（OPEN-001） / `docs/constitution/80_risks.md` OPEN-001 | — |
| OPEN-003 | 密/疎の判定方法（自動/手動/状況タグ） | `docs/constitution/15_behavior_spec.md` Section 7（OPEN-003） / `docs/constitution/80_risks.md` OPEN-003 | `docs/rfc/RFC-0001-t3-activation-rules.md` OPEN-003 |
| OPEN-004 | 確度（スコア）の表示上の意味 | `docs/constitution/15_behavior_spec.md` Section 7（OPEN-004） / `docs/constitution/80_risks.md` OPEN-004 | — |
| OPEN-005 | オープン探索で履歴/パターン/ベースラインを返すか否か | `docs/constitution/15_behavior_spec.md` Section 7（OPEN-005） / `docs/constitution/80_risks.md` OPEN-005 | — |
| ~~OPEN-006~~ | ~~relay と reveal の境界定義（どこまでが relay か）~~ | **解決済み**: `docs/constitution/15_behavior_spec.md` Section 1.1 / `docs/constitution/20_share_envelope_spec.md` Envelope 構造 | `docs/rfc/DEC-0003-relay-reveal-boundary.md` |
| ~~OPEN-007~~ | ~~遭遇カプセルの暗号化鍵は誰が持つか~~ | **解決済み**: `docs/constitution/15_behavior_spec.md` Section 5.1 / `docs/constitution/20_share_envelope_spec.md` sealed payload | `docs/rfc/DEC-0004-encounter-capsule-key.md` |
| OPEN-008 | サイレント遭遇中継のスパム対策 | `docs/constitution/15_behavior_spec.md` Sections 5, 7（OPEN-008） / `docs/constitution/80_risks.md` OPEN-008 | — |
| OPEN-010 | 権限救助者（Authorized Rescue）の定義（組織・運用主体） | `docs/constitution/15_behavior_spec.md` Sections 2, 7（OPEN-010） / `docs/constitution/80_risks.md` OPEN-010 | `docs/rfc/DEC-0004-encounter-capsule-key.md`（残存 OPEN-010） |
| ~~OPEN-011~~ | ~~通知内容の最小セット（抑止に十分・悪用に不足）~~ | **解決済み**: `docs/constitution/15_behavior_spec.md` Section 3.4 / `docs/constitution/20_share_envelope_spec.md` Notification Reference | `docs/rfc/DEC-0005-notification-minimum-set.md` |
| OPEN-012 | プラットフォーム不在/通信断時の T2 補完方法 | `docs/constitution/15_behavior_spec.md` Sections 2.1, 7（OPEN-012） / `docs/constitution/80_risks.md` OPEN-012 | `docs/rfc/RFC-0001-t3-activation-rules.md` OPEN-012 |
| OPEN-013 | T3=ON でオープン探索粒度が変わる場合の影響 | `docs/constitution/15_behavior_spec.md` Section 7（OPEN-013） / `docs/constitution/80_risks.md` OPEN-013 | `docs/rfc/RFC-0001-t3-activation-rules.md` / `docs/rfc/DEC-0001-t3-activation-rules.md` |
