# Constitution (SSOT)

## Product
We fight "information void."
Leave a trustworthy **Last Known Good** point + explicit **Blank** after it.
No guessing. No claiming. Just facts + blank.

## Outcome
Reduce early decision mistakes in rescue by providing:
- Last Known Good (time-first)
- Blank (what we do NOT know after that)

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

## SSOT Rule
This folder is the source of truth:
- docs/constitution/00_constitution.md
- docs/constitution/10_core_fact_spec.md
- docs/constitution/20_share_envelope_spec.md

Everything else is explanatory or derived.

## Change Control
- Any change to SSOT requires RFC under docs/rfc/
- Workers must NOT change SSOT inside implementation tasks

## Related (Non-SSOT docs)
<!-- TODO: 以下は将来作成予定。作成後にこのコメントを削除 -->
- docs/philosophy.md (未作成)
- docs/non-goals.md (未作成)
- docs/use-cases.md (未作成)
- docs/adr/0001-layering.md (未作成)
