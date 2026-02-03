未決OPENの追跡導線を、最小Diffで整理しました（仕様の新規確定はしていません）。

- `docs/constitution/10_core_fact_spec.md:115` に `OPEN Index (Unresolved Tracking)` を追加し、対象11件（OPEN-001,003,004,005,006,007,008,010,011,012,013）の参照先を明示。
- `docs/constitution/15_behavior_spec.md:401` の「残OPEN一覧」を同じ11件に揃え、各IDの追跡先（本文節 or `docs/risks.md`）を追加。
- `docs/risks.md:135` に索引参照を追記し、`10_core_fact_spec.md` / `15_behavior_spec.md` との参照関係を明確化。
- 実行ログを `artifacts/runs/TASK-0101-agent1-spec.out.md` に作成。

検証:
- `./ci.sh` 実行結果: green