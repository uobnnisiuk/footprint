OPEN index を最小Diffで整備しました。未決は未決のまま維持しています。

- `docs/constitution/10_core_fact_spec.md:115` 以降で OPEN Index を更新し、各 `OPEN-001,003,004,005,006,007,008,010,011,012,013` に「一次参照（`docs/constitution/*`）/補助参照（`docs/rfc/*`）」を明記しました。
- `docs/constitution/15_behavior_spec.md` / `docs/constitution/80_risks.md` は参照ズレがなかったため未変更です。
- 実行ログを `artifacts/runs/TASK-0101-agent1-spec.out.md` に反映しました。
- `./ci.sh` を実行し、green を確認しました。  

※ 既存の未コミット変更（`ci.sh`, `artifacts/runs/TASK-0200-agent4-ci.out.md`）は触っていません。