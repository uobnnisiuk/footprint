# プロジェクトレビュー (2026-02-05)

## 概要

**プロジェクト名**: footprint - デジタル痕跡SSOTシステム

**目的**: 災害時の捜索ヒントとなるデジタル痕跡を残し、後から追跡・共有・集計できること

**主要ユースケース**:
- Primary: 災害時の捜索ヒント（通信断・端末故障・電池切れの現実を前提）
- Secondary: 登山・釣り等の平時利用（平時から痕跡が溜まる運用を重視）

---

## 1. SSOT状況 (docs/constitution/)

### 1.1 仕様ドキュメント

| ファイル | 状態 | 内容 |
|---------|------|------|
| `00_constitution.md` | 存在 | 運用原則・不変条件の定義 |
| `10_core_fact_spec.md` | 充実 | L0（Core Fact）データモデル、Event Model、Blank定義、Canonical化ルール、T3 Activation Rules |
| `15_behavior_spec.md` | 充実 | 抽象挙動仕様、relay/reveal分離、能力表、link条件、通知仕様、サイレント遭遇中継 |
| `20_share_envelope_spec.md` | 充実 | L1（Share Envelope）転送仕様、Integrity（hash chain + signature）、Notification Reference、Outbox定義 |
| `30_testplan.md` | 充実 | 受け入れ条件（TST-0001〜TST-0012） |
| `80_risks.md` | 未確認 | 懸念事項・RISK・OPENの管理 |
| `contracts/trace.schema.json` | 存在 | Trace schema（JSON Schema） |

### 1.2 RFC/DEC状況 (docs/rfc/)

#### DEC（決定済み）

| ID | タイトル | 状態 |
|----|----------|------|
| DEC-0001 | T3 Activation Rules | 完了 |
| DEC-0002 | Notification Queue Persistence | 完了 |
| DEC-0003 | Relay Reveal Boundary | 完了（relayとrevealの境界をsealed payload復号の有無で定義） |
| DEC-0004 | Encounter Capsule Key | 完了（遭遇カプセル鍵はAuthorized Rescue公開鍵） |
| DEC-0005 | Notification Minimal Set | 完了（通知最小セット4点：occurred_at/action_kind/target_ref/accountability_token） |

#### RFC（提案）

| ID | タイトル | 状態 |
|----|----------|------|
| RFC-0001 | T3 Activation Rules | 完了（DEC-0001で決定） |
| RFC-0002 | Integrity Boundaries | 完了（hash chain + signature必須） |
| RFC-0002 | Notification Queue Persistence | 完了（DEC-0002で決定） |
| RFC-0004 | Encounter Capsule Key | 完了（DEC-0004で決定） |
| RFC-0005 | Notification Minimal Set | 完了（DEC-0005で決定） |

### 1.3 OPEN項目状況

`docs/constitution/10_core_fact_spec.md` のOPEN Indexに基づく追跡：

| ID | 未決テーマ | 状態 |
|----|------------|------|
| OPEN-001 | オープン探索の「存在」の粒度（エリア/時間窓） | 未解決 |
| OPEN-003 | 密/疎の判定方法（自動/手動/状況タグ） | 未解決 |
| OPEN-004 | 確度（スコア）の表示上の意味 | 未解決 |
| OPEN-005 | オープン探索で履歴/パターン/ベースラインを返すか否か | 未解決 |
| ~~OPEN-006~~ | ~~relay と reveal の境界定義~~ | **解決済み**（DEC-0003） |
| ~~OPEN-007~~ | ~~遭遇カプセルの暗号化鍵は誰が持つか~~ | **解決済み**（DEC-0004） |
| OPEN-008 | サイレント遭遇中継のスパム対策 | 未解決 |
| OPEN-010 | 権限救助者（Authorized Rescue）の定義（組織・運用主体） | 未解決（一部解決済み） |
| ~~OPEN-011~~ | ~~通知内容の最小セット~~ | **解決済み**（DEC-0005） |
| OPEN-012 | プラットフォーム不在/通信断時の T2 補完方法 | 未解決 |
| OPEN-013 | T3=ON でオープン探索粒度が変わる場合の影響 | 未解決 |

**解決済み**: OPEN-006, 007, 011
**未解決**: OPEN-001, 003, 004, 005, 008, 010, 012, 013（8件）

---

## 2. ci.sh 品質ゲート状況

### 2.1 ci.sh 構造

```
always_on_checks:
  - 必須ファイル存在チェック（AGENTS.md, constitution/*, テンプレート）
  - trace.schema.json のJSON妥当性検証

optional_checks:
  - core/share/aggregate/platform の cargo test
  - backend の pnpm/npm/yarn test
  - android の gradlew test

guard_checks:
  - no-op green 禁止チェック
```

### 2.2 直近実行結果（2026-02-05）

```
[ci] start
[ci] phase: always_on
[ci] phase: optional
[ci] phase: guard
[ci] green
```

**ステータス**: GREEN

---

## 3. 実装状況

### 3.1 ディレクトリ構成

| ディレクトリ | 状態 |
|-------------|------|
| `core/` | スケルトン（Cargo.toml存在、実装未定義） |
| `share/` | スケルトン（Cargo.toml未確認） |
| `aggregate/` | スケルトン（Cargo.toml未確認） |
| `platform/` | スケルトン（Cargo.toml未確認） |
| `backend/` | 存在せず |
| `android/` | 存在せず |

### 3.2 実装進捗

- **core/footprint-core**: バージョン0.1.0、エディション2021、依存関係なし
- 実際の実装コードはスケルトン状態
- ci.shが通る最低限の構造は整備済み

---

## 4. タスク・実行ログ状況

### 4.1 完了タスク

| ID | エージェント | 内容 | 状態 |
|----|------------|------|------|
| TASK-0101 | agent1 | spec更新 | 完了 |
| TASK-0102 | agent5 | acceptance更新 | 完了 |
| TASK-0110 | agent2 | core bootstrap | 完了 |
| TASK-0200 | agent4 | ci整備 | 完了 |
| TASK-0300 | agent6 | orchestrator | 完了 |
| TASK-0301 | agent6 | next tasks生成 | 完了 |

### 4.2 実行ログ

すべてのタスク出力が `artifacts/runs/` に保存済み。

---

## 5. バックログ状況 (docs/backlog/)

| ファイル | 状態 | 内容 |
|---------|------|------|
| idea.md | 存在 | アイデア記録 |
| research-ble-mesh.md | 存在 | BLEメッシュ調査 |
| research-ble-rescue-beacons.md | 存在 | BLE救助ビーコン調査 |
| research-footprint-integrity.md | 存在 | footprint整合性調査 |
| spike-backlog.md | 存在 | spikeバックログ |

---

## 6. 評価

### 6.1 強み

1. **SSOT明確**: AGENTS.mdで運用SSOTが明確化され、constitution/rfc/artifactsの役割分担が固定されている
2. **仕様充実**: L0（Core Fact）、L1（Share Envelope）、挙動仕様（Behavior Spec）が詳細に記述されている
3. **決定管理**: RFC-DECプロセスで重要な設計判断が記録されている（relay/reveal境界、遭遇カプセル鍵、通知最小セット）
4. **品質ゲート統一**: ci.shが単一エントリポイントとして機能し、no-op green禁止で品質を保証
5. **完全自走定義**: 「人間の追加指示なしにci.shがgreenになるまで作業を継続できる」目標が明確

### 6.2 課題

1. **実装遅延**: core/はスケルトン状態で、実際の機能実装が進んでいない
2. **OPEN未解決**: 8件のOPENが未解決（特にOPEN-001, 003, 004, 005はオープン探索の根幹）
3. **ドキュメント未確認**: `docs/constitution/80_risks.md` の内容を未確認
4. **多層構築**: L0/L1/挙動仕様の整合性を担保する実装が始まっていない

### 6.3 リスク

1. **実装と仕様の乖離**: 詳細な仕様が記述されているが、実装が追いついていない
2. **OPEN項目の影響**: OPEN-001（オープン探索粒度）などの根幹的未解決が実装に影響する可能性
3. **複雑性**: 多層仕様（L0/L1/挙動/通知）が複雑で、実装者が全体像を把握しづらい可能性

---

## 7. 次のステップ推奨

### 7.1 優先度高（実装開始）

1. **core/ 実装**: TASK-0110でbootstrap済みだが、実装の充実が必要
   - Event Modelの実装
   - JSON Lines永続化の実装（IF-IMPL-002）
   - CLIインターフェースの実装（IF-IMPL-001）
   - Canonical化ルールの実装（IF-CANON-001, 002）

2. **TST-0003追加**: 「痕跡を1件保存でき、保存結果を観測できる」テストの実装

### 7.2 優先中（OPEN解決）

1. **OPEN-001**: オープン探索の「存在」の粒度（エリア/時間窓）の確定
2. **OPEN-003**: 密/疎の判定方法（自動/手動/状況タグ）の確定
3. **OPEN-008**: サイレント遭遇中継のスパム対策の確定

### 7.3 優先低（補完）

1. **docs/constitution/80_risks.md** の内容確認・更新
2. **share/aggregate/platform/** の実装計画策定
3. **backend/** の実装計画策定

---

## 8. まとめ

**現状**: 仕様・運用定義・品質ゲートの基盤が整備されている段階。実装がスケルトン状態から本格的な機能実装へ移行するフェーズ。

**成功の鍵**:
- 仕様の詳細さと実装の進捗をバランスさせる
- OPEN項目の優先順位を明確にし、実装ブロッカーを解消する
- ci.shのgreenを維持しながら、段階的に機能を積み上げる

**総合評価**: 良好な開始状態。次は実装フェーズへ。
