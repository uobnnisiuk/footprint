# TASK-0102: agent5 — acceptance強化（OPEN-011: 通知最小セットを観測可能に）

## 目的（SR）
`docs/constitution/30_testplan.md` に、**OPEN-011（通知内容の最小セット）が確定していることを観測可能**な Acceptance を1本だけ追加する（最小Diff）。

## 前提（不変条件）
- 既存TSTを弱めない
- 観測は「実装内部」ではなく「ファイル上の確定」を対象にする

## 変更範囲（Diff最小）
- `docs/constitution/30_testplan.md` のみ

## 追加するTST（SR）
- **TST-0012**: `docs/rfc/DEC-0005-notification-minimal-set.md` において、
  - Notification Minimal Set として **occurred_at / action_kind / target_ref / accountability_token** が明記されていること
  - **禁止事項（閲覧者PII/内容/精密位置/自由記述を含めない）**が明記されていること
  を観測できる

## 観測手順（1行コマンド例）
- `grep -n "occurred_at\\|action_kind\\|target_ref\\|accountability_token" docs/rfc/DEC-0005-notification-minimal-set.md && grep -n "個人情報\\|payload\\|精密位置\\|自由記述" docs/rfc/DEC-0005-notification-minimal-set.md`

## 受け入れ条件
- `30_testplan.md` に TST-0012 が追加される
- 既存TSTは弱体化されない
- `./ci.sh` green


