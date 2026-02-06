Aligned the share envelope Notification Reference with DEC-0005’s four-field minimal set to remove the remaining mismatch in OPEN‑011 resolution; RFC/DEC and the behavior/core/risk SSOT entries already matched and needed no edits.

- `docs/constitution/20_share_envelope_spec.md`: renamed the Notification Reference fields to `occurred_at`/`action_kind`/`target_ref`/`accountability_token`, added a short deferment note for the extra refs, and tightened the prohibited content list to match DEC‑0005.
- `artifacts/runs/TASK-0101-agent1-spec.out.md`: added a 2026‑02‑06 re‑verify entry with the CI result.

Tests:
- `./ci.sh`

Next steps:
1. Commit the changes if this matches the intended resolution.