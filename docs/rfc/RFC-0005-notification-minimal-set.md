# RFC-0005: 通知内容の最小セット固定（OPEN-011）

## Status
Accepted
Owner: agent1
Date: 2026-02-05

## Context

OPEN-011（通知内容の最小セット）は、reveal/link の抑止設計を成立させるための未決だった。
DEC-0002 で Notified の成立条件（送達キューへの永続登録）は確定済みだが、**通知本文に何を入れるか**は未固定だった。

通知本文が過剰だと通知自体が漏洩源になり、逆に少なすぎると「見たらバレる」の抑止力が弱くなる。
よって、通知本文は「抑止に十分・悪用に不足」の最小集合を固定する必要がある。

## SR（推奨案）

**通知（Notified）本文の最小セットは次の4点のみとする。**

1. `occurred_at`（発生時刻: UTC, ms, ISO8601 Z）
2. `action_kind`（`REVEAL` / `LINK`）
3. `target_ref`（対象を示す不透明ID）
4. `accountability_token`（監査追跡用の責任トークン）

## 理由

### 「抑止に十分」

- `occurred_at` と `action_kind` で「いつ・何が行われたか」が観測できる
- `target_ref` で「どの対象に対する行為か」が追える
- `accountability_token` で監査系の追跡が可能になり、濫用の心理的・運用的抑止が働く

### 「悪用に不足」

- 閲覧者/実行者の個人情報を含めないため、通知から逆引きした攻撃を抑える
- 痕跡内容（payload/location/state）を含めないため、通知そのものが追加漏洩源にならない
- 精密位置・自由記述を禁止し、通知経由での推定・炎上誘発を抑える

## 明示的に禁止する内容

- 閲覧者/実行者の個人情報（氏名・電話・メール・住所・アカウントID・端末ID・IP 等）
- 痕跡の内容（payload/location/state 等）
- 精密位置・自由記述

## OPEN-010 との境界

この RFC は通知本文の最小セットのみを決定する。
**OPEN-010（Authorized Rescue の定義）**は未決のまま維持し、本 RFC では確定しない。

## 影響範囲（最小Diff）

- `docs/rfc/DEC-0005-notification-minimal-set.md`
- `docs/constitution/15_behavior_spec.md`
- `docs/constitution/10_core_fact_spec.md`
- `docs/constitution/80_risks.md`

## 決定

DEC-0005 として採用する。
