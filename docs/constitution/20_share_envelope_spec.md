# Share Envelope Spec (SSOT)

## Purpose
Create transferable, tamper-evident copies of Core facts (L1).
Transport is adapter-only; Share Engine does not depend on BLE/HTTP/etc.

## Envelope
### Required
- envelope_id
- created_at
- source_id: privacy-safe identifier (policy decided elsewhere)
- payload_events: [Core Event]
- integrity: (詳細は下記 Integrity セクション参照)

### Optional
- chunking/compression metadata

## Integrity（改ざん検出・真正性）

L1 は hash chain + signature の両方を必須とする（IF-INTEG-002）。
根拠: RFC-0002 / 15_behavior_spec.md / IDEA-0003 の設計方針と整合。

### hash chain アルゴリズム

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
