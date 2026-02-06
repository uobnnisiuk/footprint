# footprint

災害・通信断・端末破損・電池切れ等でも、**「デジタル痕跡を意地でも残す」**ことで、人命救助の初動判断に効く手がかりを残すためのプロジェクトです。
目的は「現在地を当てる」ことではなく、**救助判断に効く証拠パッケージ**（**Last Seen/第三者観測** + Last Known Good + 明示的な Blank）を、改ざん耐性と共有境界つきで扱えるようにすることです。

> ⚠️ 本リポジトリは設計・検証・実装の途中段階です。実運用の救助用途にそのまま使えることを保証しません。

---

## 何を作るのか（最重要）

footprint は「情報の空白（information void）」と戦います。
**推測しない / 断定しない / 事実と Blank のみ**を SSOT（単一の真実）として保持し、救助側が「最初に何を疑うべきか」の誤判断を減らすことを狙います。

### 目標アウトカム（優先順）
- **Last Seen（LS）**：第三者に観測された一次証拠（端末操作不能でも成立することを最優先）
- **Last Known Good（LKG）**：自己記録の確実点（価値は高いが前提には置かない）
- **Blank**：その後に *何が分からないか* の明示（「不明」を不明のまま残す）
- **最小外部手がかり（MEE）**は許容（OS組み込み群衆NW等で得られる位置1点レベル）
  ただし差別点は「位置そのもの」ではなく、**救助に効く証拠パッケージ**の扱いにある

---

## Non-goals（やらないこと）

以下は **ハードに禁止**です。
- 現在地の断定
- 行動経路・行動推定
- 生存確率の推定
- 救助優先度の自動決定
- AI を意思決定者にすること

---

## 設計の芯（原則と境界）

### 0) 一次証拠は LS（Observer-first）
一次証拠（primary evidence）は **LS（Last Seen / 第三者観測）**。
被災者端末が操作不能でも成立する思想を採用します。LKG（自己記録）は有益だが前提には置きません。

### 1) レイヤ境界（L0〜L3）
- **L0: Core Fact（SSOT）** … 事実 + Blank、append-only、推測なし
- **L1: Share Engine** … 転送可能なエンベロープ + Outbox（Transport依存はアダプタに隔離）
- **L2: Aggregation** … 事実を捏造しない派生ビュー（順位付け/集約/畳み込み）
- **L3: Platform** … アクセス制御・監査・同期（推測なし）

### 2) relay と reveal の分離（読める/運べるを分ける）
- **relay**: 運ぶ（誰でも可能）
- **reveal**: 読む（危険なので制御対象）
境界は **sealed payload の復号**で定義します。

### 3) 「読んだら相手に伝わる」＝通知が成立しないなら reveal/link 禁止
Notified（通知成立）は「送達キューへの**クラッシュ復旧可能な永続登録**」で定義します。
永続登録を保証できない状況では reveal/link は禁止。

通知本文は **最小4点のみ**（抑止に十分・悪用に不足）：
- `occurred_at`
- `action_kind`（REVEAL/LINK）
- `target_ref`
- `accountability_token`

### 4) Encounter（遭遇）の保証線：FG=MUST / BG=BE
- MUST: ローカル永続（Capture）
- MUST: Foreground Encounter（前景での遭遇）
- BE: Background Encounter（すれ違い即時は上積み）

### 5) 遭遇カプセルの鍵：Authorized Rescue 公開鍵
遭遇カプセル（Encounter Capsule）は **Authorized Rescue 公開鍵で暗号化**し、通行人は復号できない（relay-only）。

### 6) L0/L1 の integrity 責務：hash chain + signature は L1
- L0 は canonicalization（ハッシュ入力の正）を定義する
- L1 が hash chain + signature を必須として持つ

### 7) T3（災害モード）発動ルール
手動フラグを最上位トリガに、公的アラートによる自動ON等を組み合わせ、OFF / RECOMMENDED / ON の三状態で管理。

---

## SSOT（単一の真実）

**このフォルダが SSOT です：**
- `docs/constitution/00_constitution.md`
- `docs/constitution/10_core_fact_spec.md`
- `docs/constitution/15_behavior_spec.md`
- `docs/constitution/20_share_envelope_spec.md`
- `docs/constitution/30_testplan.md`（Acceptance SSOT）
- `docs/constitution/80_risks.md`（RISK/OPEN 管理）

**SSOT を変更する場合は必ず `docs/rfc/` に RFC を置き、DEC で決定を残します。**

---

## リポジトリの進め方（Mode B）

単一のエントリポイントで「タスク実行 → CI green」を回します：
```bash
bash artifacts/outbox/RUN-0300-orchestrator.sh
```

---

## Quickstart（最低限）

```bash
# 1) まずはCI
./ci.sh

# 2) オーケストレータで1サイクル回す（タスク→CI）
bash artifacts/outbox/RUN-0300-orchestrator.sh
```

---

## OPEN / SPIKE（未解決と検証）

未解決の設計判断（OPEN）は `docs/constitution/10_core_fact_spec.md` の OPEN Index と `docs/constitution/80_risks.md` で追跡します。
検証（SPIKE）の一覧と優先度は `docs/backlog/spike-backlog.md` を参照。

---

## ライセンス

[LICENSE](LICENSE) を参照してください。
