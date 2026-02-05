# DEC-0005: 通知内容の最小セット（OPEN-011 解決）

## 日付
2026-02-05

## 決定内容（SR）

**通知（Notified）本文の最小セットは `occurred_at` / `action_kind` / `target_ref` / `accountability_token` の4点のみとする（抑止に十分・悪用に不足）。**

### Notification Minimal Set（固定）

| # | フィールド | 内容 |
|---|-----------|------|
| 1 | `occurred_at` | 発生時刻（UTC, ms, ISO8601 `Z`） |
| 2 | `action_kind` | `REVEAL` / `LINK`（詳細分類は後続で拡張可） |
| 3 | `target_ref` | 「どの対象か」を示す不透明ID（`case_id` / `candidate_id` / `subject_ref` 等。型は `target_ref` で統一） |
| 4 | `accountability_token` | 後で監査で追える責任トークン（公開しない不透明トークン） |

### 明示的に禁止（固定）

- 閲覧者/実行者の個人情報（氏名・電話・メール・住所・アカウントID・端末ID・IP 等）を含めない
- 痕跡の内容（`payload` / `location` / `state` 等）を含めない
- 精密位置や自由記述を含めない

### 不変条件

- **IF-NOTIFY-CONTENT-001**: 通知本文は `occurred_at` / `action_kind` / `target_ref` / `accountability_token` の4点のみを最小セットとして扱う。閲覧者/実行者の個人情報、痕跡内容、精密位置、自由記述を含めない

## 理由

1. **抑止成立**: 「いつ」「何を」「どの対象に」「責任トークン付きで」が観測できるため、reveal/link の抑止要件を満たす
2. **漏洩抑制**: 通知本文自体に PII や痕跡内容を入れず、通知が新たな漏洩源になることを防ぐ
3. **既存仕様との整合**: DEC-0002 の Notified 成立条件（送達キュー永続登録）を変更せず、本文最小化のみを確定できる
4. **論点分離**: OPEN-010（Authorized Rescue の定義）と独立して決定できる

## 関連

- RFC-0005: `docs/rfc/RFC-0005-notification-minimal-set.md`
- 解消する OPEN: OPEN-011（通知内容の最小セット）
- 継続 OPEN: OPEN-010（Authorized Rescue の定義）
- 既存前提: IF-NOTIFIED-001（DEC-0002）
