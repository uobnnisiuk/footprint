# DEC-0004: 遭遇カプセルの暗号化鍵（OPEN-007 解決）

## 日付
2026-02-05

## 決定内容（SR）

**遭遇カプセル（Encounter Capsule）は「Authorized Rescue 公開鍵」で暗号化する。復号可能者は救助側/権限者に限定する。**

### 仕様への落とし込み（最小の決定文）

1. Encounter Capsule は Authorized Rescue の公開鍵で暗号化する
2. カプセルの公開ヘッダに `recipient_key_id`（救助鍵の識別子）を含める — 誰の鍵で暗号化したかを明示
3. 通行人端末は復号不可（relay-only）— IF-BOUNDARY-001 と整合

### 新規不変条件

- **IF-CAPSULE-KEY-001**: 遭遇カプセルの暗号化鍵は Authorized Rescue の公開鍵とする。通行人は復号できない

### 未決定のまま残すもの

- Authorized Rescue の定義（組織・運用主体）は OPEN-010 で別途解決する
- 「鍵のクラスは救助機関」だけを先に固定し、具体的な鍵配布・ローテーション運用は OPEN-010 解決後に設計する

## 理由

1. **仕様との整合**: 15_behavior_spec.md Section 5.1 が「救助側/権限者だけが復号」と明記しており、最小 diff で整合する
2. **可用性**: 家族鍵は「家族が鍵を持っていない/用意できない/関係が複雑」で救助の初動に失敗しやすい
3. **運用の現実性**: 地域鍵は配布・更新・失効・漏洩時対応が運用重すぎて L1 前提としては不安定（C-008 の鍵運用難所そのもの）
4. **分離可能性**: OPEN-010（Authorized Rescue の定義）と独立に「鍵のクラス」だけ先に固定できる

## 却下案

- **家族鍵**: 可用性に弱い。家族が鍵を持たない/紛失する/複雑な家族関係で初動失敗のリスクが高い
- **地域鍵**: 配布・更新・失効・漏洩対応の運用負荷が重すぎる。広域災害時に鍵漏洩リスクが増大

## 関連
- 解消する OPEN: OPEN-007（遭遇カプセルの暗号化鍵は誰が持つか）
- 前提となる不変条件: IF-BOUNDARY-001（DEC-0003）, IF-RELAY-001, IF-REVEAL-001
- 残存 OPEN: OPEN-010（Authorized Rescue の定義 — 鍵のクラスは固定済み、運用主体は未定）
- 反映先:
  - `docs/constitution/15_behavior_spec.md`（Section 5, 6, 7）
  - `docs/constitution/20_share_envelope_spec.md`（sealed payload に recipient_key_id 追加）
  - `docs/constitution/80_risks.md`（OPEN-007 → 解決済み）
  - `docs/constitution/10_core_fact_spec.md`（OPEN Index）
  - `docs/backlog/spike-backlog.md`（P-L1 OPEN-007 更新）
