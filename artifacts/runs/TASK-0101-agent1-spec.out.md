OPEN index を最小Diffで更新しました。未決は未決のまま維持しています。

- `docs/constitution/10_core_fact_spec.md:121` 以降の OPEN Index を整理し、各 OPEN の参照先を「一次参照（constitution）」と「補助参照（rfc/dec）」に分離して明記しました。
- 対象の未決IDは指定どおり `OPEN-001,003,004,005,006,007,008,010,011,012,013` のままです（勝手な確定なし）。
- `docs/constitution/15_behavior_spec.md` と `docs/constitution/80_risks.md` は参照ズレがなかったため未変更です。
- `./ci.sh` を実行し、green を確認しました。  

（既存の未コミット変更 `ci.sh` と `artifacts/runs/TASK-0200-agent4-ci.out.md` には触れていません。）