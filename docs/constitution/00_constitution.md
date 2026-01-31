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
- docs/philosophy.md
- docs/non-goals.md
- docs/use-cases.md
- docs/adr/0001-layering.md
