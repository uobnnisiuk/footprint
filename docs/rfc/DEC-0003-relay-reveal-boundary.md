# DEC-0003: relay と reveal の境界定義

## 日付
2026-02-05

## 決定内容（SR）

**relay の定義を「sealed payload を復号せず、公開ヘッダの範囲だけで損失なく運ぶこと」に固定し、reveal は「sealed payload の平文にアクセスした瞬間」に固定する。**

### 境界の定義

- **relay（通知なし）**
  - できる: 受領・保存・再送・重複排除（envelope_id）・チャンク/圧縮（復号なしのまま）・署名の形式検査（任意）
  - 禁止: sealed payload の復号、payload_events（Core Event）の読み取り/解析、そこから導ける位置/時刻/同一性の推定
- **reveal（必ず通知が成立できることが前提）**
  - sealed payload を復号する/平文を見る/平文由来の情報を表示・検索に使う行為はすべて reveal
  - IF-REVEAL-001（サイレント不可）および IF-NOTIFIED-001（送達キュー永続登録）の制約に乗る

### L1 Envelope への反映（公開/非公開の分離）

- **公開ヘッダ（relay が触ってよい）**: `envelope_id`, `created_at`, `source_id`（privacy-safe は別ポリシー）, `integrity`
- **非公開（relay から隠す）**: `payload_events` 相当は **`sealed_payload`（暗号化コンテナ）** に封入
- `integrity` は「暗号化前の平文」に対して hash chain を構築する前提（受領者は復号後に検証）

### 新規不変条件

- **IF-BOUNDARY-001**: relay と reveal の境界は「sealed payload の復号の有無」で定義する。復号なしで触れる範囲が relay、復号した瞬間が reveal。

## 理由

1. **L1 実装の前提固定**: OPEN-006 が未解決のままだと、envelope の暗号化/同梱範囲がブレて SPIKE-0001（envelope round-trip）すら前提が揺れる
2. **実装可能な境界**: 「第三者が運べるが読めない」を「復号の有無」という機械的に検証可能な境界に落とすことで、曖昧さが消える
3. **既存不変条件との整合**: IF-RELAY-001（第三者は読めない）と IF-REVEAL-001（サイレント不可）を「復号」という単一の境界で統一的に実装できる

## 却下案

- **フィールド単位の暗号化（per-field encryption）**: 柔軟だが、どのフィールドまでが relay 可かの境界が曖昧になる。公開ヘッダ + sealed payload の二分割のほうが境界が明確
- **全フィールド暗号化（envelope 全体を暗号化）**: relay がメタデータ（重複排除用の envelope_id 等）にもアクセスできなくなり、中継機能が成立しない

## 関連
- 解消する OPEN: OPEN-006（relay と reveal の境界定義）
- 前提となる不変条件: IF-RELAY-001, IF-REVEAL-001, IF-NOTIFIED-001
- 反映先:
  - `docs/constitution/20_share_envelope_spec.md`（Envelope 構造）
  - `docs/constitution/15_behavior_spec.md`（Sections 1.1, 5, 7）
  - `docs/constitution/80_risks.md`（OPEN-006 → 解決済み）
  - `docs/constitution/10_core_fact_spec.md`（OPEN Index）
  - `docs/backlog/spike-backlog.md`（SPIKE-0001 前提更新）
