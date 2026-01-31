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
  - hash_chain OR signature OR both (choose implementation later, semantics fixed: offline verifiable)

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
