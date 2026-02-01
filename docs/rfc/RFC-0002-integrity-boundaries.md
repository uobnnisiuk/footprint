# RFC-0002: L0/L1 Integrity 責務境界

## Status
Accepted
Owner: SR (human)
Date: 2026-02-01

## Context

現在の SSOT では「L0 は事実のみ」「L1 は転送のみ」と宣言しているが、署名・hash chain がどちらの責務かが曖昧だった。

Bootstrap slice（CLI + JSONL 永続化）との整合性、および鍵管理の複雑性を考慮すると、責務の明確な切り分けが必要。

## Goal

- L0/L1 の integrity 責務を SSOT に明記する
- 署名・hash chain は L1（Share Envelope）に固定することを決定する
- L0 は正規化（canonicalization）の定義までを責務とし、暗号学的 integrity は持たない

## Non-goals

- 具体的な署名アルゴリズムの選定（別途決定）
- 鍵管理（生成・ローテ・失効・バックアップ）の詳細設計
- L0 per-event signature の将来拡張（必要時に mini RFC）

## Proposal (Minimal Diff)

### 1. L0（Core Fact）の責務

| 責務 | 内容 |
|------|------|
| 正規化（canonicalization）の定義 | フィールド順・型・文字列エンコーディング・時刻表現を「ハッシュ入力としての正」として定義 |
| 順序の決定規則 | hash chain の順序となる「正しい順序（committed順）」を定義 |
| 暗号学的 integrity | **持たない** — 改ざん検出は L1 に委譲 |

**設計意図**: L1 実装者が「どれを署名対象にする？」で揺れないよう、canonical bytes を L0 が決める。

### 2. L1（Share Envelope）の責務

| 責務 | 内容 |
|------|------|
| hash chain 構築 | L0 の canonical(event) を入力に、payload_events の順序どおりに chain を作る |
| Envelope 署名 | 「この envelope は確かにこの source から出た」をオフラインでも検証可能にする |

### 3. hash chain アルゴリズム（最小定義）

```
e[i] = canonical(payload_events[i])
h[-1] = 0x00...00  (固定長ゼロ、アルゴリズム依存)
h[i] = H( h[i-1] || e[i] )
```

envelope.integrity に含める情報:
- `hash_alg`: ハッシュアルゴリズム識別子
- `chain_tail`: h[last]（最終ハッシュ）
- `event_count`: イベント数（推奨）

### 4. 署名アルゴリズム（最小定義）

```
signed_bytes = canonical(envelope_header_without_signature || integrity.chain_tail)
signature = Sign(private_key, signed_bytes)
```

envelope.integrity に含める情報:
- `sig_alg`: 署名アルゴリズム識別子
- `signer_key_id`: 公開鍵探索用識別子
- `signature`: 署名値

**ポイント**: payload_events 全体を再シリアライズせず、chain_tail に代表させることで検証を軽量化。

### 5. L0 署名を入れない理由

- 鍵管理（生成・ローテ・失効・バックアップ・端末移行）がコアの不可避な複雑性になる
- 現在の bootstrap slice（CLI + JSONL 永続化）と相性が悪い
- L1 の integrity で offline verifiable は既に達成される

### 6. 将来の拡張余地

以下の要件が出た場合、L0 per-event signature を mini RFC で追加可能:
- 端末ローカルの L0 ストレージ自体の改ざん検出を「転送前から」保証したい
- envelope を細切れで流通させても「個々の event 単体で真正性」を持たせたい

## Impact on SSOT

- Constitution: No change
- Core Fact Spec: **Change** — Canonicalization セクションを追加
- Share Envelope Spec: **Change** — Integrity 詳細セクションを追加

## Safety Check (Must)

- No guessing introduced: **Yes** — 暗号学的検証は明示的なデータのみ対象
- No claiming introduced: **Yes** — 署名は事実の転送に対してのみ
- Blank remains explicit: **Yes** — blank の扱いは変更なし

## Acceptance Criteria

- AC1: L0 が canonical(event) の定義を持つこと
- AC2: L1 が hash_chain + signature の構築責務を持つこと
- AC3: envelope.integrity に hash_alg, chain_tail, sig_alg, signer_key_id, signature が含まれること
- AC4: オフラインで envelope の真正性が検証可能なこと

## 不変条件

- **IF-INTEG-001**: L0 は暗号学的 integrity を持たない（正規化の定義までが責務）
- **IF-INTEG-002**: L1 は hash chain + signature の両方を必須とする
- **IF-INTEG-003**: chain_tail を署名対象に含めることで、payload 全体の再シリアライズを回避する

## 決定

DEC-0002 として採用。10_core_fact_spec.md および 20_share_envelope_spec.md に反映する。
