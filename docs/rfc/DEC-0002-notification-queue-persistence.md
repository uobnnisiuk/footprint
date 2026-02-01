# DEC-0002: 送達キュー永続性仕様（Notification Queue Durability）

## 日付
2026-02-01

## 決定内容（SR）

**「送達キューの永続性（端末クラッシュ後に復旧できること）」は"実装の工夫"ではなく、Notified の意味そのものとして仕様（SSOT）に含める。**

### Notified の成立条件

Notified = 宛先（本人＋プラットフォーム）への通知が **送達キューに"永続的に"登録された** こと。

**永続的**とは:
- enqueue が成功を返した通知は、**アプリ/OSクラッシュ後の再起動で必ずキューに残っている**（ロストしない）
- "表示された/受信した"（Delivered）は成立条件ではない（観測/メトリクス仕様扱い）

### 「仕様」vs「実装」の切り分け

- **仕様（must）**: クラッシュ復旧性（durability）を満たすこと
- **実装（自由）**: durability の実現方法（SQLite WAL、追記ログ＋fsync、OSトランザクション等）

### 失敗時の挙動

永続登録を保証できない場合（例：ストレージ書込不能）は **Notified 不成立** → 既存仕様どおり **reveal/link は禁止**。

### 新規不変条件

**IF-NOTIFIED-001**: Notified の成立には送達キューへの永続登録（クラッシュ復旧可能）が必須である。

## 理由

1. **セキュリティ境界としての一貫性**: Notified は reveal/link の許可条件であり、「登録されたつもり」が起きると仕様破綻する
2. **既存仕様との整合**: 「送達キューにはローカル永続Outboxを含める」と書いており、「確実」はクラッシュ耐性を含むのが自然な一貫性
3. **L0 との設計原則の統一**: L0 の "durably committed" と同じく、成立条件は耐障害性込みで定義すべき

## 関連
- RFC-0002: 送達キュー永続性仕様（詳細）
- 反映先:
  - docs/constitution/15_behavior_spec.md (3.2)
  - docs/constitution/20_share_envelope_spec.md (Outbox Rules)
  - docs/constitution/30_testplan.md (TST-0004)
