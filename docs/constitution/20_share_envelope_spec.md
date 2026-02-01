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
- integrity:
  - hash_chain + signature（両方必須を推奨）
  - 根拠: 15_behavior_spec.md / IDEA-0003 の「署名＋hash chain」設計方針と整合
  - semantics fixed: offline verifiable

### Optional
- chunking/compression metadata

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
