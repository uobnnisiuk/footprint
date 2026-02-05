# DEC-0005: 通知内容の最小セット（OPEN-011 解決）

## 日付
2026-02-05

## 決定内容（SR）

**通知の最小セットは「行為の事実 + 責任追跡 + 粗いスコープ」だけに固定し、内容（痕跡そのもの）や精密な位置・連続追跡可能性は一切入れない。**

### 通知ペイロード（最小セット）

| # | フィールド | 内容 | 目的 |
|---|-----------|------|------|
| 1 | **event_kind** | REVEAL / LINK_START / LINK_EXTEND / LINK_STOP | 行為の種別（最低でも REVEAL と LINK の区別） |
| 2 | **performed_at** | 通知が生成された時刻（= 行為が発生した時刻） | 「いつ」の事実 |
| 3 | **actor_class** | Trusted / AuthorizedRescue / Auditor | 「誰が」の分類（実名/所属は入れない） |
| 4 | **accountability_token** | 後で監査・受付が追えるトークン（一般公開しない） | 責任追跡（抑止の芯） |
| 5 | **scope_summary** | Time Window（例: last_24h）+ Coarse Location Cell（点座標ではなくセル） | 「どの範囲を」の粗い要約 |
| 6 | **case_ref** *(任意だが強く推奨)* | T2/T3 で受付起票（Case）が存在するなら case_id | 救助/受付文脈の判断、監査線の一本化 |

### 新規不変条件

- **IF-NOTIFY-CONTENT-001**: 通知ペイロードに痕跡の内容（位置の精密座標・連続追跡可能なID・閲覧内容の要約）を含めない。通知自体が新たな漏洩源にならないことを保証する

## 「抑止に十分」な理由

- 本人から見ると「いつ・どの権限クラスが・どの範囲を・責任トークン付きで」が分かり、**"見たらバレる"** が成立する（IF-REVEAL-001 / IF-NOTIFIED-001 の趣旨に合う）
- 救助/監査側から見ると accountability_token と case_ref で追跡・審査ができる（スパム/濫用の入口が作れる）

## 「悪用に不足」な理由

- 精密位置・連続追跡可能なID・閲覧内容の要約を入れないので、通知自体が新たな漏洩にならない
- actor の実名/所属を通知本文に入れないので、逆方向の嫌がらせ誘発も抑える（必要なら監査でのみ解決）

## 却下案

- **「閲覧した痕跡のサマリを入れる」**: 通知が漏洩源になる（悪用に寄る）ので不可
- **「実名/所属を常に入れる」**: 抑止は強いが、報復・炎上・嫌がらせ誘発が大きい。責任追跡は token + 監査で十分
- **「時刻と権限クラスだけ」**: 抑止が弱い。何をどの範囲で見たかの"スコープ"が無いと本人が判断できない

## 関連
- 解消する OPEN: OPEN-011（通知内容の最小セット — 抑止に十分・悪用に不足）
- 前提となる不変条件: IF-REVEAL-001, IF-NOTIFIED-001（DEC-0002）
- 反映先:
  - `docs/constitution/15_behavior_spec.md`（Section 3, 6, 7）
  - `docs/constitution/20_share_envelope_spec.md`（通知参照情報の同梱範囲）
  - `docs/constitution/80_risks.md`（OPEN-011 → 解決済み）
  - `docs/constitution/10_core_fact_spec.md`（OPEN Index）
  - `docs/backlog/spike-backlog.md`（P-L1 OPEN-011 更新、SPIKE-0005 依存更新）
