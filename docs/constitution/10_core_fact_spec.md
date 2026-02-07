# Core Fact Spec (SSOT)

## Purpose
Persist trustworthy **facts** and explicit **blank** as SSOT (L0).

## Primary Evidence (一次証拠)

footprintの一次証拠は **LS（Last Seen / 第三者観測）**を主軸とする。被災者側が操作不能/端末破損でも成立する設計思想である。被災者端末の自己記録（LKG）は有益だが必須前提ではない。

## Terminology

| 用語 | 定義 |
|------|------|
| **LKG** | 端末内自己記録の最後の確実点（Self/On-device） |
| **LS** | 第三者の観測点（Witness/External） |

LKGとLSは別ストリームであり、統合は相関（time/location）＋confidenceで扱う（暗号学的一意紐付けは前提にしない）。

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
- **ネット断絶でも機能**: ネットは上積み。ネット断絶でも **LSの記録（少なくとも観測ログがローカルに残ること）**が成立することを要件とする。平時の常時位置追跡/常時アップロードを前提にしない
- **Encounter保証線**: DEC-0008 参照。Capture（ローカル永続）= MUST、FG Encounter = MUST、BG Encounter = BE。詳細は `15_behavior_spec.md` Section 5

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

OPEN（未解決の設計判断）の一元管理は `docs/constitution/80_risks.md` を参照。
