# Share Envelope Spec (SSOT)

## Purpose
Create transferable, tamper-evident copies of Core facts (L1).
Transport is adapter-only; Share Engine does not depend on BLE/HTTP/etc.

## Envelope

Envelope は **公開ヘッダ**（relay が触ってよい）と **sealed payload**（復号しないと読めない）の二層で構成する（DEC-0003: IF-BOUNDARY-001）。

### 公開ヘッダ（relay 可）
- envelope_id
- created_at
- source_id: privacy-safe identifier (policy decided elsewhere)
- integrity: (詳細は下記 Integrity セクション参照)

### sealed payload（relay 不可 — 復号 = reveal）
- sealed_payload: 暗号化コンテナ（内部に payload_events: [Core Event] を含む）
- recipient_key_id: 暗号化に使用した Authorized Rescue 公開鍵の識別子（公開ヘッダ側に配置 — relay が鍵の宛先を確認可能）
- 暗号化鍵: Authorized Rescue の公開鍵（IF-CAPSULE-KEY-001 / DEC-0004）。通行人は復号不可

### Optional
- chunking/compression metadata（公開ヘッダ側。復号なしで処理できるようにする）

### 設計根拠（DEC-0003）
- relay は公開ヘッダだけで受領・保存・再送・重複排除（envelope_id）・チャンク/圧縮が可能
- relay は sealed_payload を復号できない（読めない）
- sealed_payload を復号した瞬間が reveal（IF-REVEAL-001 の通知制約に乗る）

## Integrity（改ざん検出・真正性）

L1 は hash chain + signature の両方を必須とする（IF-INTEG-002）。
根拠: RFC-0002 / 15_behavior_spec.md / IDEA-0003 の設計方針と整合。

### hash chain アルゴリズム

hash chain は **暗号化前の平文 payload_events** に対して構築する。受領者は sealed_payload を復号した後に検証する（DEC-0003）。

```
e[i] = canonical(payload_events[i])   # L0 が定義した正規化
h[-1] = 0x00...00                      # 固定長ゼロ（アルゴリズム依存）
h[i] = H( h[i-1] || e[i] )
```

envelope.integrity に含める情報:
- `hash_alg`: ハッシュアルゴリズム識別子（例: SHA-256）
- `chain_tail`: h[last]（最終ハッシュ）
- `event_count`: イベント数（推奨）

### 署名アルゴリズム

```
signed_bytes = canonical(envelope_header_without_signature || integrity.chain_tail)
signature = Sign(private_key, signed_bytes)
```

envelope.integrity に含める情報:
- `sig_alg`: 署名アルゴリズム識別子（例: Ed25519）
- `signer_key_id`: 公開鍵探索用識別子（privacy-safe であることは別ポリシー）
- `signature`: 署名値

### 設計ポイント

- **chain_tail に代表させる**: payload_events 全体を再シリアライズせず、chain_tail を署名対象に含めることで検証を軽量化
- **オフライン検証可能**: 受信者は envelope 単体で改ざん検出・出所確認が可能

### integrity フィールド構造

```json
{
  "hash_alg": "SHA-256",
  "chain_tail": "base64...",
  "event_count": 42,
  "sig_alg": "Ed25519",
  "signer_key_id": "source-key-123",
  "signature": "base64..."
}
```

### 不変条件

- **IF-INTEG-002**: L1 は hash chain + signature の両方を必須とする
- **IF-INTEG-003**: chain_tail を署名対象に含めることで、payload 全体の再シリアライズを回避する
- **IF-BOUNDARY-001**: relay と reveal の境界は sealed payload の復号の有無で定義する（DEC-0003）
- **IF-CAPSULE-KEY-001**: 遭遇カプセルの暗号化鍵は Authorized Rescue の公開鍵とする（DEC-0004）
- **IF-NOTIFY-CONTENT-001**: 通知ペイロードに痕跡内容・精密位置・追跡可能IDを含めない（DEC-0005）

## Notification Reference（通知に同梱する参照情報）

reveal / link が発生した際の通知ペイロードの最小セット（DEC-0005）。
envelope に"通知に必要な参照情報だけ"を同梱する範囲を固定する。

| フィールド | 内容 | 備考 |
|-----------|------|------|
| occurred_at | 発生時刻（UTC, ms, ISO8601 `Z`） | |
| action_kind | REVEAL / LINK（詳細分類は後続で拡張可） | 行為の種別 |
| target_ref | 対象を示す不透明ID（case_id / candidate_id / subject_ref 等） | 型は target_ref で統一 |
| accountability_token | 監査用トークン（非公開） | 責任追跡 |

注: `action_kind` の詳細分類や追加参照（例: actor_class / scope_summary / case_ref）は後続で決定する。最小セットには含めない。

**禁止**: 閲覧者/実行者の個人情報、痕跡内容（payload/location/state）、精密位置、自由記述を通知に含めない。

## Outbox
Outbox stores envelopes pending delivery.

### States (Minimum)
- QUEUED
- SENT_ACKED
- PURGED (optional policy after ACK)

### Rules
- No event invention / no mutation
- Transport-specific logic stays in adapters
- Envelope must be verifiable offline
- **QUEUED state is crash-durable**: enqueue が成功を返したエントリは、アプリ/OSクラッシュ後の再起動で必ず QUEUED 状態として残る（ロストしない）。これにより Notified（送達キュー永続登録）の成立条件を満たす。
