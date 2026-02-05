OPEN-011 を SR どおり確定しました。通知本文は 4 点固定（`occurred_at` / `action_kind` / `target_ref` / `accountability_token`）で、禁止事項（PII・痕跡内容・精密位置・自由記述）を DEC 化しています。

- mini RFC を新規追加: `docs/rfc/RFC-0005-notification-minimal-set.md:1`
- DEC を新規追加（旧 `docs/rfc/DEC-0005-notification-minimum-set.md` は置換）: `docs/rfc/DEC-0005-notification-minimal-set.md:1`
- 挙動 SSOT を反映（Section 3.4 と IF サマリ）: `docs/constitution/15_behavior_spec.md:357`, `docs/constitution/15_behavior_spec.md:425`
- OPEN index の DEC 参照を更新: `docs/constitution/10_core_fact_spec.md:132`
- risks の C-006/OPEN-011 を DEC 参照で整合: `docs/constitution/80_risks.md:86`, `docs/constitution/80_risks.md:150`, `docs/constitution/80_risks.md:164`
- 実施ログを更新: `artifacts/runs/TASK-0101-agent1-spec.out.md:1`

検証:
- `./ci.sh` は green（成功）

次の自然な一手:
1. 必要なら `docs/constitution/20_share_envelope_spec.md` の Notification Reference も DEC-0005 の 4 点固定に合わせる
2. この差分をコミットする