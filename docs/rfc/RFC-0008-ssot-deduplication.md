# RFC-0008: SSOT 文書の重複解消と構造整理

## Status
Accepted
Owner: agent
Date: 2026-02-06

## Context

SSOT 6文書が段階的に拡充された結果、同一情報が複数箇所に重複し、
どこが定義元（source of definition）か不明確になっている。

主な重複・散在：
1. OPEN 追跡が `10_core_fact_spec.md` / `15_behavior_spec.md` / `80_risks.md` の3箇所に分散
2. 不変条件（IF-*）が4文書に散在し、全量を一覧できる場所がない
3. 通知最小セット（DEC-0005）のテーブルが `15_behavior_spec.md` と `20_share_envelope_spec.md` に同一複写
4. `00_constitution.md` の Product/Outcome が DEC-0007（Observer-first / LS primary）を未反映
5. `80_risks.md` の「暫定まとめ」が constitution + behavior_spec の複写
6. `00_constitution.md` の Related (Non-SSOT docs) に未作成4件のゴミ参照

## Goal

**「各情報の定義元を1箇所に確定し、他は参照に変える」**ことで、
SSOT 間の重複・矛盾を解消する。

## Non-goals
- 仕様の新規追加・変更・削除
- ファイル名やファイル構成の変更
- RFC/DEC 文書の変更

## Proposal (Minimal Diff)

### Change 1: OPEN 追跡の一本化
- **定義元**: `80_risks.md` OPEN セクション（未解決 + 解決済み）
- `10_core_fact_spec.md` OPEN Index → 参照ポインタに置換
- `15_behavior_spec.md` Section 7 → 参照ポインタに置換

### Change 2: 不変条件（IF）全量索引の追加
- `00_constitution.md` に全 IF の索引テーブルを追加（IF-ID / 概要 / 定義元）
- `15_behavior_spec.md` Section 6 の部分的 IF サマリ → 参照ポインタに置換
- 各仕様書の IF 定義本体はそのまま残す（索引は参照のみ）

### Change 3: 通知最小セットの重複解消
- **定義元**: `15_behavior_spec.md` Section 3.4
- `20_share_envelope_spec.md` Notification Reference → テーブル削除、定義元への参照に置換

### Change 4: Constitution の DEC-0007 反映
- `00_constitution.md` Product / Outcome を DEC-0007（LS primary, LKG auxiliary）に整合

### Change 5: 80_risks.md「暫定まとめ」の削除
- 9項目の挙動原則サマリは constitution / behavior_spec と丸かぶりのため削除

### Change 6: Constitution の未作成参照削除
- `00_constitution.md` Related (Non-SSOT docs) セクション削除（全4件が未作成）

### Change 7: Encounter 保証線の重複簡素化
- `10_core_fact_spec.md` Hard Rules 内の Encounter 保証線 → DEC-0008 参照に簡素化

## Impact on SSOT
- Constitution: Change（Product/Outcome 更新、IF 索引追加、Related 削除）
- Core Fact Spec: Change（OPEN Index 簡素化、Encounter 保証線簡素化）
- Behavior Spec: Change（Section 6, 7 をポインタに置換）
- Share Envelope Spec: Change（Notification Reference をポインタに置換）
- Testplan: No change
- Risks: Change（暫定まとめ削除）

## Safety Check (Must)
- No guessing introduced: Yes
- No claiming introduced: Yes
- Blank remains explicit: Yes

## Acceptance Criteria
- AC1: 全 OPEN の追跡先が `80_risks.md` に一本化されている
- AC2: `00_constitution.md` に全 IF の索引がある
- AC3: 通知最小セットのテーブルが `15_behavior_spec.md` にのみ存在する
- AC4: `00_constitution.md` Product/Outcome が DEC-0007 と整合している
- AC5: 仕様・不変条件・OPEN の情報が一切欠落していない

## 決定

DEC-0010 として採用する。
