# TASK-0101: agent1 — P-L1 OPEN-011 を DEC で確定（通知内容の最小セット）

## 目的（SR）
P-L1 の未決 OPEN のうち **OPEN-011（通知内容の最小セット）**を、mini RFC + DEC で確定し、SSOT（OPEN index / risks / behavior）へ最小Diffで反映する。

## SR（結論を先に固定）
**SR**: 通知（Notified）の本文に含める最小セットは以下の4点のみとする（“抑止に十分・悪用に不足”）。

### Notification Minimal Set（固定）
- **occurred_at**: 発生時刻（UTC, ms, ISO8601 Z）
- **action_kind**: `REVEAL` / `LINK`（詳細分類は後続だが最低2値は固定）
- **target_ref**: 「どの対象か」を示す **不透明ID**（例: case_id / candidate_id / subject_ref のいずれか。型は `target_ref` で統一）
- **accountability_token**: 後で監査で追える責任トークン（公開されない不透明トークン）

### 明示的に禁止（固定）
- 閲覧者/実行者の **個人情報（氏名・電話・メール・住所・アカウントID・端末ID・IP等）**を含めない
- 痕跡の **内容（payload/location/state 等）** を含めない
- **精密位置**や **自由記述**を含めない（悪用・炎上リスク）

## 前提（不変条件）
- IF-001: SSOT は `docs/constitution/*` と `docs/rfc/*`
- IF-NOTIFIED-001: Notified は「送達キューへの永続登録（クラッシュ復旧）」で成立（DEC-0002/RFC-0002 は変更しない）
- “通すために弱める変更”は禁止（既存TSTの弱体化禁止）
- OPEN-010（Authorized Rescue 定義）は未決のまま維持（本タスクで確定しない）

## 変更範囲（Diff最小）
- `docs/rfc/RFC-0005-notification-minimal-set.md`（新規）
- `docs/rfc/DEC-0005-notification-minimal-set.md`（新規）
- `docs/constitution/15_behavior_spec.md`（OPEN-011 を解決済みにし、DEC参照を追加：最小）
- `docs/constitution/10_core_fact_spec.md`（OPEN index から OPEN-011 を解決済みに移し、DEC参照を追加）
- `docs/constitution/80_risks.md`（OPEN-011 を解決済みにし、C-006 の通知最小セットを DEC 参照へ）
- 実施ログ: `artifacts/runs/TASK-0101-agent1-spec.out.md`

## 実施内容（順序固定）
1) 既存根拠を確認:
   - `docs/constitution/80_risks.md` C-006 の「通知は最小（いつ／どの候補／責任トークン）」記述
   - `docs/rfc/RFC-0002-notification-queue-persistence.md`（内容は非対象、永続性のみ）
2) mini RFC を作る（1〜2ページ）
   - なぜこの4点が「抑止に十分」で「悪用に不足」なのか
   - 禁止事項（PII/内容/精密位置/自由記述）を明文化
   - OPEN-010 との境界（権限救助者の定義は別件）
3) DEC を作る（SRを1行で固定）
   - “通知最小セット4点” と “禁止事項” を必ず含む
4) SSOT 反映（最小Diff）
   - 15_behavior_spec / 10_core_fact_spec OPEN index / 80_risks を参照整合
5) `./ci.sh` green を確認し、実施ログを runs に残す

## 受け入れ条件
- OPEN-011 が DEC により確定し、SSOT から参照可能
- 既存の Notified 永続性（DEC-0002）と矛盾しない
- `./ci.sh` green


