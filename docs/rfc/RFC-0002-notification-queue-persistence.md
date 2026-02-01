# RFC-0002: 送達キュー永続性仕様（Notification Queue Durability）

## Status
Accepted
Owner: SR (human)
Date: 2026-02-01

## Context

Notified の定義が「送達キューに登録された時点」となっているが、キューの永続性（端末クラッシュ時の復旧）が「仕様」なのか「実装の工夫」なのかが曖昧だった。

### 問題

- Notified はセキュリティ境界（reveal/link の許可条件）であり、「登録されたつもり」が起きると仕様破綻する
- 既存仕様で「送達キューにはローカル永続Outboxを含める」と書いているが、「確実」の意味（クラッシュ耐性を含むか）が明文化されていない
- L0 の "durably committed" と同じく、成立条件は推測ではなく耐障害性込みで定義すべき

## Goal

- Notified の成立条件として「永続性＝クラッシュ復旧性」を仕様（SSOT）に含める
- 仕様（must）と実装（自由）の切り分けを明確にする
- 失敗時の挙動を定義する

## Non-goals

- 具体的な永続化の実装方式（SQLite WAL、追記ログ＋fsync、OSトランザクション等）の選定
- 通知内容の詳細（OPEN-011で別途検討）

## Proposal (Minimal Diff)

### 1. Notified の成立条件（仕様として固定）

**Notified** = 宛先（本人＋プラットフォーム）への通知が **送達キューに"永続的に"登録された** こと。

**永続的（Durable）** の定義:
- enqueue が成功を返した通知は、**アプリ/OSクラッシュ後の再起動で必ずキューに残っている**（ロストしない）
- "表示された/受信した"（Delivered）は成立条件ではない（観測/メトリクス仕様扱い）

**不変条件 IF-NOTIFIED-001**: Notified の成立には送達キューへの永続登録（クラッシュ復旧可能）が必須である。

### 2. 「仕様」vs「実装」の切り分け

| 区分 | 内容 |
|------|------|
| **仕様（must）** | クラッシュ復旧性（durability）を満たすこと |
| **実装（自由）** | durability の実現方法（SQLite WAL、追記ログ＋fsync、OSトランザクション等） |

### 3. 失敗時の挙動（仕様の帰結）

- 永続登録を保証できない場合（例：ストレージ書込不能）は **Notified 不成立**
- Notified 不成立なら、既存仕様どおり **reveal/link は禁止**

### 4. SSOT への最小 Diff

| ファイル | 変更内容 |
|----------|----------|
| `15_behavior_spec.md` (3.2) | 「永続＝クラッシュ復旧」の定義を追加 |
| `20_share_envelope_spec.md` (Outbox Rules) | 「QUEUEはクラッシュで消えない」ルールを追加 |
| `30_testplan.md` | クラッシュ復旧テスト（TST-0004）を追加 |

## Impact on SSOT

- Constitution: No change
- Core Fact Spec: No change
- Behavior Spec: **Change** — 3.2 に永続性の定義を追加
- Share Envelope Spec: **Change** — Outbox Rules にクラッシュ復旧性を追加
- Testplan: **Change** — クラッシュ復旧テストを追加

## Safety Check (Must)

- No guessing introduced: **Yes** — 永続性は明示的な成立条件
- No claiming introduced: **Yes** — enqueue 成功＝durably written のみを根拠とする
- Blank remains explicit: **Yes** — 永続登録失敗時は明示的に Notified 不成立

## Acceptance Criteria

- AC1: 15_behavior_spec.md の 3.2 に永続性の定義が追加されていること
- AC2: 20_share_envelope_spec.md の Outbox Rules にクラッシュ復旧性が明記されていること
- AC3: 30_testplan.md にクラッシュ復旧テストが追加されていること
- AC4: 既存の Notified 関連仕様と矛盾がないこと

## L0 との整合

L0（Core Fact）の "durably committed" と同じ設計原則:
- 成立条件は耐障害性込みで定義
- 「登録されたつもり」を許さない

## 決定

DEC-0002 として採用。15_behavior_spec.md / 20_share_envelope_spec.md / 30_testplan.md に反映する。
