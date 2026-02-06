# Specification Review: footprint 仕様群

**Date**: 2026-02-06
**Scope**: docs/constitution/ 全文、docs/rfc/ 全 RFC/DEC、docs/backlog/ 全研究文書
**Focus**: 仕様全体の整合性 + iOS BLE ジレンマへの視点転換

---

## Part I: 仕様全体の評価

### 1. 設計思想の一貫性

**極めて高い**。以下の設計判断が全文書を通じて一貫している。

- **「事実 + 空白」原則**: 推論しない、主張しない、空白は明示する。constitution から実装まで一貫
- **relay / reveal 分離**: DEC-0003 で「sealed payload の復号の有無」という明快な境界を定義。15_behavior_spec.md の能力表、20_share_envelope_spec.md の Envelope 構造がこの境界に忠実
- **通知の非対称性**: relay は通知不要、reveal は必ず通知。IF-REVEAL-001 がプライバシー設計の骨格になっている
- **append-only**: L0 で「上書き禁止」を意味論として保証し、改ざん検出は L1 に委譲（IF-INTEG-001）

### 2. 仕様間の整合性

| 文書ペア | 整合性 | 備考 |
|----------|--------|------|
| constitution ↔ core_fact_spec | 良好 | Layering、Non-goals が一致 |
| core_fact_spec ↔ 実装 (lib.rs) | 良好 | 正規化ルール（IF-CANON-001/002）が BTreeMap + timestamp 正規化で実現 |
| behavior_spec ↔ share_envelope_spec | 良好 | DEC-0003 の relay/reveal 境界が Envelope 構造に反映 |
| behavior_spec ↔ risks | 良好 | C-001〜C-008 が behavior_spec の OPEN と対応 |
| testplan ↔ 実装 | **要注意** | TST-0003（痕跡保存の観測）が「後で追加」のまま。実装は既にCLIで実現しているが、テストプランに反映されていない |
| trace.schema.json ↔ 実装 | **要修正** | payload の null 許容（前回レビュー指摘済み）|

### 3. OPEN / RISK の管理状況

**よく管理されている**。13 件中 5 件が解決済み（DEC で確定）、8 件が未解決。80_risks.md と 10_core_fact_spec.md の OPEN Index が二重管理だが、内容は一致しており追跡可能。

ただし、**OPEN の優先度付けが明示されていない**。spike-backlog.md で P-L1/P-L2/P-L3 のフェーズ分けがあるが、constitution 側の OPEN 一覧にはこの情報がない。フェーズと優先度を OPEN Index に反映すると、次に何を決めるべきかが明確になる。

### 4. 能力表（Actor x Capability）の完成度

15_behavior_spec.md Section 2 の能力表は **仕様のハイライト**。relay/reveal/link/audit の 4 軸で Actor を制御する発想は、「判別ロジックで解こうとせず能力設計で安全性を作る」という原則の具現。

**改善点**: 「通りすがり」の能力に **help（通報）** が「許可（任意操作）」とあるが、この help のトリガーと UI 動線が未定義。疎での価値（C-005）を最大化するなら、help の UI は最小タップ（1-2 操作）であるべき。この点は仕様として明記する価値がある。

### 5. T2/T3 発動ルールの成熟度

**十分に成熟している**。特に以下が優秀:

- T2 の 3 段階（Report → Open Case → Start Link）による段階的昇格
- Weak/Strong Independent の二段階独立性定義
- カテゴリ別ルーティングと共通ガード（G-01〜G-05）

**唯一の構造的弱点**: T2 は「運用受付（プラットフォーム受付）」を必須ゲートにしているが、OPEN-012（プラットフォーム不在/通信断時）が未解決。災害時こそプラットフォーム不在になりやすいため、T3 発動時に T2 ゲートを緩和するルール（例: 権限救助者が自己起票できる範囲の拡大）を先に決めておくべき。

---

## Part II: iOS BLE ジレンマの深堀りと視点転換

### 現状の整理

プロジェクトが到達した結論（DEC-0006/0007/0008 + IDEA-0004）は正しい:

1. **BG BLE を保証に置かない**（DEC-0008: BG Encounter = BE）
2. **被災者端末を前提にしない**（DEC-0007: LS が主、LKG は補助）
3. **マルチトランスポート保証を不採用**（DEC-0006: attempt > guarantee の劣化）
4. **OS 群衆ネットワーク活用**（IDEA-0004: Find My / Find Hub で位置 relay）

**しかし、これらの決定が「仕方がない」というトーンで書かれている**。ここに視点転換の余地がある。

---

### 視点転換 1: 「iOS 制約」ではなく「証拠設計の原理」として捉え直す

現在の仕様は、iOS の BLE 制約を「技術的障壁」として記述している。しかし、DEC-0007（Observer-first）と DEC-0008（FG=MUST / BG=BE）を組み合わせると、実は **より強い設計原理** が導かれる:

> **「被災者は何もできないことを前提にする。証拠は第三者が作る。」**

これは iOS 制約の回避策ではなく、**人命救助の本質に根ざした設計原理**。理由:

- 生き埋め、気絶、端末破損、電池切れ — いずれも「被災者端末が何かをする」前提を壊す
- **Android でも** 端末が水没したら同じ。iOS 制約はこの本質を早期に顕在化させただけ
- ココヘリ、EPIRB（船舶用非常位置表示）、ACR PLB（個人用救難ビーコン）—  **専門救助機器はすべて「被災者は操作不能」を前提に設計されている**

**提案**: constitution の Design Principles に以下を追加する:

```
- Prefer witness-generated evidence over victim-generated evidence
  (被災者起点の証拠より、第三者起点の証拠を優先する)
```

これにより、iOS 制約への対応が「妥協」ではなく「原則に基づく設計判断」になる。

---

### 視点転換 2: 「relay のジレンマ」を分解する

現在 RISK-BLE-IOS-001 は「操作ゼロの痕跡/中継をスマホ単体で保証できない」と記述しているが、ここには 2 つの異なる問題が混在している:

| 問題 | 内容 | 本当に必要か |
|------|------|--------------|
| **(A) 被災者の位置が残る** | 被災者が最後にいた場所の手がかりが、何らかの形で外部に存在する | **必須（人命に直結）** |
| **(B) 被災者の詳細状態が中継される** | 遭遇カプセル（battery%, network_state 等を含む payload）が第三者経由で運ばれる | **有益だが必須ではない（上積み）** |

現在の仕様はこの区別を research-minimum-external-evidence.md で示唆し、relay(A) と relay(B) を区別しているが、**この区別が constitution レベルに昇格していない**。

**提案**: 以下の不変条件を追加する:

```
IF-MEE-001: Minimum External Evidence（位置1点の手がかり）は、
footprint アプリの稼働を前提としない経路でも成立すること。
```

これにより:
- relay(A)（位置）: Find My / Find Hub / BLE タグで解決 → **footprint の責務外に明示的に分離**
- relay(B)（詳細状態）: footprint の FG Encounter + Store & Forward で解決 → **上積みとして明確化**
- iOS ジレンマの本質が「relay(B) を BG で保証できない」に絞られ、これは DEC-0008 で既に「BE（上積み）」と結論済み

---

### 視点転換 3: 「FG Encounter の価値」を再定義する

DEC-0008 は FG Encounter を MUST としているが、**その価値が「BG の代替手段」として矮小化されている**。FG Encounter には BG にはない固有の価値がある:

1. **意図的な観察**: FG（アプリを開いている）状態は、観察者が意図的に周囲を確認していることを意味する。これは **LS（第三者観測）の品質が最も高い瞬間**
2. **context の付与**: FG なら observer がカテゴリ選択（R-03 の 6 種類）や Confidence 付与ができる。BG の自動中継では不可能
3. **救助導線への直接接続**: FG なら help（通報）操作にワンステップで遷移できる

**提案**: FG Encounter を「BG の劣化版」ではなく、**「最高品質の LS 生成手段」** として仕様に位置づけ直す。

具体的には、15_behavior_spec.md Section 5 に以下のニュアンスを追加:

```
FG Encounter は BG Encounter の代替ではなく、observer の意図的行為による
最高品質の LS（第三者観測）である。BG Encounter が上積みである理由は、
BG では context（カテゴリ選択・確度付与・通報）が不可能だからでもある。
```

---

### 視点転換 4: 「習慣的利用」と「災害モード」の間を埋める

constitution の Design Principles に「Prefer habit-friendly use (normal routine)」がある。これは重要だが、**平時の習慣と災害時の FG 利用を結ぶ導線が仕様にない**。

**問題**: 災害が起きてから footprint を開くのでは遅い。しかし平時に毎日開く理由がない。

**提案 — 「日常の LS 蓄積」モデル**:

登山・釣り・アウトドアなど、footprint がターゲットとするユースケースでは:

1. **平時**: 仲間同士で FG Encounter を記録する。「今日のメンバーはこの 4 人、出発地点はここ」— これが LKG の seed になる
2. **異常検知**: 予定時刻に帰着しない → 信頼者が T2 Report を上げる
3. **災害時**: すでに習慣化されているアプリだから開く。T3=ON で挙動が切り替わる

この「平時 → 異常 → 災害」の連続性を仕様として明記すると:
- 「なぜ平時に使うのか」の答えが仕様レベルで出る
- 「災害時に FG で開く動機」が自然になる
- iOS 制約の影響が最小化される（そもそも FG 前提の設計だから）

---

### 視点転換 5: 「公平性の再定義」— iOS ユーザーは本当に不利か

RISK-BLE-IOS-001 の核心は「iOS ユーザーが相対的に痕跡が残りにくい → 救命機会に偏りが生じる」。しかし、DEC-0007 + IDEA-0004 の世界では:

| 証拠経路 | iOS | Android | 差異 |
|----------|-----|---------|------|
| Find My / Find Hub タグ (relay A) | Find My NW（数十億台）| Find Hub | **iOS が優位** |
| FG Encounter (relay B) | 同等 | 同等 | 差なし |
| BG Encounter (relay B) | 制限あり | 比較的自由 | **Android が優位** |
| Capture（ローカル永続）| 同等 | 同等 | 差なし |

**BG Encounter でのみ Android が優位**だが、relay(A) では iOS（Find My NW）が圧倒的に優位。つまり、**証拠設計全体で見ると、iOS と Android の公平性は確保できる可能性がある**。

**提案**: RISK-BLE-IOS-001 に以下の補足を追加:

```
iOS BLE BG 制約による relay(B) の不利は、relay(A)（Find My NW）の
優位性で相殺される可能性がある。公平性の評価は relay(A) + relay(B) +
Capture の総合で行うべきであり、relay(B) 単独で判定すべきではない。
```

---

### 視点転換 6: ハードウェアは「敗北」ではなく「救助の専門性」

RISK-BLE-IOS-001 の既存サービス比較表（ココヘリ、DRS、HITOCOCO）は「専用ハードウェアに成立条件を移している」と記述しているが、これを「footprint もハードウェアが必要」と読むのは早計。

**より正確な読み**:

> 「操作ゼロ + 常時性」の保証は、**そもそもソフトウェア単体で解ける問題ではない**。
> Apple も Google も OS レベル + 専用タグで解いている。
> footprint が解くべきは「位置の常時発信」ではなく、
> **「位置 + 状態 + 空白」の Rescue Evidence Package を改ざん耐性付きで渡すこと**。

つまり:

```
位置の手がかり（MEE）   → 専用タグ / OS群衆NW に委ねる（footprint の責務外）
Rescue Evidence Package → footprint の差別点（L0 + L1 で渡す）
```

これは constitution の Outcome セクションに既にある:

> footprint の差別点は「位置そのもの」ではなく、救助判断に効く
> Rescue Evidence Package（Last Known Good + explicit Blank + 観測された状態）を
> 改ざん耐性と共有境界（relay/reveal）付きで渡せること。

**この一文が iOS ジレンマの解を既に含んでいる**。問題は、この一文の意味が仕様全体に十分に浸透していないこと。

---

## Part III: 具体的な仕様改善提案

### 優先度: 高

| # | 提案 | 対象文書 | 理由 |
|---|------|----------|------|
| S-1 | Design Principles に「Prefer witness-generated evidence over victim-generated evidence」を追加 | 00_constitution.md | DEC-0007 の原則を constitution レベルに昇格。iOS 対応を「原則」にする |
| S-2 | IF-MEE-001 を新設し、relay(A) と relay(B) の区別を constitution レベルで明確化 | 00_constitution.md, 10_core_fact_spec.md | iOS ジレンマの問題空間を正しく分割する |
| S-3 | TST-0003 を実装済みの CLI テストに合わせて具体化する | 30_testplan.md | 実装は存在するがテストプランが追いついていない |
| S-4 | OPEN-012（T2 のオフライン補完）の解決方針を策定する | 15_behavior_spec.md | 災害時にプラットフォーム不在が最も起きやすく、T2 ゲートが詰まるのは致命的 |

### 優先度: 中

| # | 提案 | 対象文書 | 理由 |
|---|------|----------|------|
| S-5 | FG Encounter を「最高品質の LS 生成手段」として再定義 | 15_behavior_spec.md | BG の代替ではなく固有の価値として位置づけ |
| S-6 | RISK-BLE-IOS-001 に relay(A) + relay(B) 総合での公平性評価を追記 | 80_risks.md | iOS ユーザーの不利が relay(B) 限定であることを明確化 |
| S-7 | 「平時 → 異常 → 災害」の連続的利用モデルを仕様化 | 15_behavior_spec.md or 新規文書 | 習慣的利用と FG Encounter の接続を明確化 |
| S-8 | OPEN Index にフェーズ（P-L1/P-L2/P-L3）と優先度を反映 | 10_core_fact_spec.md | 次に何を決めるべきかを明確化 |

### 優先度: 低

| # | 提案 | 対象文書 | 理由 |
|---|------|----------|------|
| S-9 | help（通報）の UI 動線の最小タップ要件を仕様化 | 15_behavior_spec.md | 疎での価値最大化のため |
| S-10 | trace.schema.json の payload に null 許容を追加 | contracts/trace.schema.json | 実装との乖離解消 |

---

## Part IV: iOS ジレンマへの最終的な提言

### 「ジレンマ」は既に解消されている — ただしそのことに気づいていない

プロジェクトの既存の決定（DEC-0006/0007/0008 + IDEA-0004）と constitution の Outcome 定義を組み合わせると:

1. **位置の手がかり（MEE）** → Find My / Find Hub タグで解決（iOS/Android 公平）
2. **Rescue Evidence Package** → FG Encounter + Capture + L1 Envelope で解決（iOS/Android 同等）
3. **BG Encounter** → 上積み（BE）として明確に分離済み

**iOS だから救えない、という状況は発生しない**。ただし、この結論を導くには:

- relay(A) と relay(B) の分離が constitution レベルで明文化されていること（→ S-2）
- Find My / Find Hub タグとの連携が IDEA-0004 ではなく仕様に昇格していること
- FG Encounter の固有価値が再定義されていること（→ S-5）

が必要。

### 人命救助の観点での最重要メッセージ

**footprint が解くべき問題は「iOS でどう BLE を動かすか」ではない。**

footprint が解くべき問題は:

> 「被災者が何もできない状況で、第三者の観測がどうやって
>  救助者の手に届き、その情報がどれだけ信頼できるか」

この問題に対して、footprint は **Rescue Evidence Package（事実 + 空白 + 改ざん耐性 + relay/reveal 境界）** という独自の解を持っている。位置の常時発信は Apple と Google に任せ、footprint は **位置の先にある「救助判断に効く情報」** を届ける。

これは妥協ではない。**分業**である。

---

*Reviewed by: Claude (Opus 4.6)*
*Spec documents reviewed: 00_constitution.md, 10_core_fact_spec.md, 15_behavior_spec.md, 20_share_envelope_spec.md, 30_testplan.md, 80_risks.md, trace.schema.json, RFC-0001〜0005, DEC-0001〜0008, backlog research 5 files*
