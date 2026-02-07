# Constitution (SSOT)

## Product
We fight "information void."
Primary evidence is **LS (Last Seen / third-party observation)** — works even when the victim's device is inoperable (DEC-0007).
**LKG (Last Known Good / self-record)** is valuable but not a prerequisite.
No guessing. No claiming. Just facts + blank.

## Outcome
Reduce early decision mistakes in rescue by providing:
- **LS (Last Seen)**: third-party observation — primary evidence (DEC-0007)
- **LKG (Last Known Good)**: self-record — auxiliary evidence
- **Blank**: what we do NOT know after that
- **MEE (Minimum External Evidence)**: 「外部で観測できる最小の手がかり」（例: OS組み込みの"アイテム発見"による位置1点）でよい
- footprint の差別点は「位置そのもの」ではなく、救助判断に効く **Rescue Evidence Package**（LS + LKG + explicit Blank + 観測された状態）を改ざん耐性と共有境界（relay/reveal）付きで渡せること。

## Non-goals (Hard)
- No current location claiming
- No route / behavior inference
- No survival probability estimation
- No automatic rescue prioritization
- No AI as decision maker

## Design Principles (Hard)
- Prefer certain facts over uncertain inference
- Prefer robustness over many features
- Prefer explicit blank over false certainty
- Prefer habit-friendly use (normal routine)
- **IF-LOSSLESS-001**: 観測された痕跡は、UI/集約/フィルタの都合で到達不能にしない（ドリルダウン経路を必ず残す）
- **IF-SEARCH-001**: 探索モードの情報開示レベル
  - オープン探索 → 「存在のみ」（同一性・特定に至る情報は返さない）
  - 対象キー探索 → 「同一性・特定あり得る」（キー保持者のみ到達可能）

## Layering (Hard boundaries)
- L0: Footprint Core (facts + blank, append-only, SSOT)
- L1: Share Engine (envelope + outbox; transport adapters only)
- L2: Aggregation Engine (derived views without inventing facts)
- L3: Platform (access control, audit, sync; no guessing)

## Invariant Index (IF)

全不変条件の索引。定義本体は各仕様書を参照。

| IF | 概要 | 定義元 |
|----|------|--------|
| IF-LOSSLESS-001 | 観測痕跡は到達不能にしない | 本文 Design Principles |
| IF-SEARCH-001 | 探索モードの情報開示レベル | 本文 Design Principles |
| IF-CANON-001 | フィールド順序はアルファベット順固定 | 10_core_fact_spec.md |
| IF-CANON-002 | 時刻は UTC・ミリ秒精度固定 | 10_core_fact_spec.md |
| IF-INTEG-001 | L0 は暗号学的 integrity を持たない | 10_core_fact_spec.md |
| IF-INTEG-002 | L1 は hash chain + signature 必須 | 20_share_envelope_spec.md |
| IF-INTEG-003 | chain_tail を署名対象に含める | 20_share_envelope_spec.md |
| IF-T3-001 | 手動フラグは最上位の確定トリガ | 10_core_fact_spec.md |
| IF-T3-002 | 公的アラート取得時のみ自動ON許可 | 10_core_fact_spec.md |
| IF-T3-003 | ヒューリスティック単独ではONにしない | 10_core_fact_spec.md |
| IF-T3-004 | 自動ONはTTLで失効 | 10_core_fact_spec.md |
| IF-IMPL-001 | CLI (stdin JSON → stdout JSON) で開始 | 10_core_fact_spec.md |
| IF-IMPL-002 | ローカルファイル (JSON Lines) で開始 | 10_core_fact_spec.md |
| IF-IMPL-003 | 実装言語は Rust | 10_core_fact_spec.md |
| IF-REVEAL-001 | reveal はサイレント不可 | 15_behavior_spec.md |
| IF-NOTIFIED-001 | 通知成立は送達キュー永続登録 | 15_behavior_spec.md |
| IF-RELAY-001 | サイレント遭遇中継 | 15_behavior_spec.md |
| IF-BOUNDARY-001 | relay/reveal 境界は sealed payload 復号で定義 | 20_share_envelope_spec.md |
| IF-CAPSULE-KEY-001 | カプセル鍵は Authorized Rescue 公開鍵 | 20_share_envelope_spec.md |
| IF-NOTIFY-CONTENT-001 | 通知は最小4点のみ、PII等禁止 | 15_behavior_spec.md |

## SSOT Rule
This folder is the source of truth:
- docs/constitution/00_constitution.md
- docs/constitution/10_core_fact_spec.md
- docs/constitution/15_behavior_spec.md
- docs/constitution/20_share_envelope_spec.md
- docs/constitution/30_testplan.md（Acceptance SSOT）
- docs/constitution/80_risks.md（RISK/OPEN 管理）

Everything else is explanatory or derived.

## Change Control
- Any change to SSOT requires RFC under docs/rfc/
- Workers must NOT change SSOT inside implementation tasks

