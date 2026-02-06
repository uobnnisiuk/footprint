# RFC-0007: SSOT Rule ファイルリスト更新

## Status
Accepted
Owner: agent
Date: 2026-02-06

## Context

`00_constitution.md` の SSOT Rule セクションは以下の3ファイルのみを列挙している：

- `docs/constitution/00_constitution.md`
- `docs/constitution/10_core_fact_spec.md`
- `docs/constitution/20_share_envelope_spec.md`

しかし、その後に追加・拡充された以下の文書は自身で SSOT を名乗っている：

- `15_behavior_spec.md` — タイトルが「挙動仕様（SSOT）」
- `30_testplan.md` — 冒頭で「観測可能な受け入れ条件（Acceptance）のSSOT」と宣言
- `80_risks.md` — RISK / OPEN の一次管理台帳として機能

SSOT Rule のリストが実態と乖離しており、SSOT ファイルの追加・削除時に Change Control（RFC 必須）が適用されるべき対象が曖昧になっている。

## Goal

SSOT Rule のファイルリストを実態に合わせて6ファイルに更新し、Change Control の適用範囲を明確にする。

## Non-goals
- 各ファイルの内容変更
- SSOT Rule 以外のセクション（Product / Non-goals / Layering 等）の変更

## Proposal (Minimal Diff)

`00_constitution.md` の SSOT Rule セクションを以下に変更する：

```markdown
## SSOT Rule
This folder is the source of truth:
- docs/constitution/00_constitution.md
- docs/constitution/10_core_fact_spec.md
- docs/constitution/15_behavior_spec.md
- docs/constitution/20_share_envelope_spec.md
- docs/constitution/30_testplan.md（Acceptance SSOT）
- docs/constitution/80_risks.md（RISK/OPEN 管理）

Everything else is explanatory or derived.
```

## Impact on SSOT
- Constitution: Change（SSOT Rule セクションのみ）
- Core Fact Spec: No change
- Share Envelope Spec: No change
- Behavior Spec: No change（既に SSOT を自称しており、リストへの追加で正式化するのみ）
- Testplan: No change（同上）
- Risks: No change（同上）

## Safety Check (Must)
- No guessing introduced: Yes
- No claiming introduced: Yes
- Blank remains explicit: Yes

## Acceptance Criteria
- AC1: `00_constitution.md` の SSOT Rule に6ファイルが列挙されている
- AC2: 列挙されたファイルがすべて実在する

## 決定

DEC-0009 として採用する。
