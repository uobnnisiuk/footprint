```md
# TASK-0101: agent1 — spec最小更新（OPEN整理）

## 目的（SR）
`docs/constitution/10_core_fact_spec.md` を最小Diffで更新し、現在の OPEN を追跡しやすい形に整理する。

## 前提（不変条件）
- IF-001: SSOTは `docs/constitution/` と `docs/rfc/`
- IF-002: 要件本文の追加は最小。新規の実装要求は増やさない
- IF-003: 未決は OPEN として明示し、曖昧語で隠さない
- IF-004: 仕様変更が必要なら mini RFC + DEC

## 変更範囲（Diff最小）
- 主対象: `docs/constitution/10_core_fact_spec.md`
- 必要最小で参照更新: `docs/constitution/15_behavior_spec.md` / `docs/risks.md`

## 実施内容
1) 現在の未決OPEN（OPEN-001,003,004,005,006,007,008,010,011,012,013）を確認
2) `10_core_fact_spec.md` に「OPEN index（参照先付き）」を追加/更新
3) OPENの参照先を `15_behavior_spec.md` と `docs/risks.md` に揃える
4) `./ci.sh` を実行し green を確認

## 受け入れ条件（TST）
- TST-0101-1: `10_core_fact_spec.md` から未決OPENの所在が追える
- TST-0101-2: 仕様の新規確定を勝手に行わない（未決は未決のまま明記）
- TST-0101-3: `./ci.sh` が green

## 生成物
- 実行ログ: `artifacts/runs/TASK-0101-agent1-spec.out.md`
- 変更ファイル（最小Diff）
```
