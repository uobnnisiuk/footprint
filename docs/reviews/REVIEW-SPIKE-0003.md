# SPIKE-0003 レビュー: 持続的近接 BG BLE 検出確率検証

Reviewer: agent (automated review)
Date: 2026-02-06
Branch: `claude/review-spike-0003-TqeBT`
Scope: RFC-0003, runbook, iOS/Android spike アプリ, 分析スクリプト, タスク定義

---

## 総合評価

SPIKE-0003 は「持続的近接仮説（RFC-0003）」を実機で検証するための計測キット一式である。
RFC の論理構成・runbook・分析スクリプトの設計は良好だが、**iOS/Android 間で Service UUID が不一致**という致命的な問題がある。このまま実測に入ると **クロスプラットフォーム検出が一切できない**。

| 観点 | 評価 | 備考 |
|------|------|------|
| RFC-0003 (仮説設計) | **Good** | 論理構成が明確。能登地震の文脈と整合 |
| Runbook | **Good** | 再現可能な手順。ただし proximity_tag 命名の不整合あり |
| 分析スクリプト | **Good** | 標準ライブラリのみ、demo 動作確認済み |
| iOS アプリ | **Needs Fix** | BLE 基本機能あり。BG restoration 未実装 |
| Android アプリ | **Needs Fix** | Foreground Service 構成あり。Permission handling に問題 |
| クロスプラットフォーム整合性 | **Blocker** | Service UUID 不一致 |

---

## Blocker (実測前に修正必須)

### B-1: Service UUID が iOS/Android で不一致

iOS と Android で異なる Service UUID を使用しているため、**iOS-Android 間の BLE 検出が不可能**。

| Platform | Service UUID |
|----------|-------------|
| iOS | `6A8F0C6C-5ED6-4D2D-9EAD-1CC1E7E4F3A1` |
| Android | `0000fdde-0000-1000-8000-00805f9b34fb` |

- **ファイル**: `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike/BleManager.swift:5`
- **ファイル**: `spikes/spike-0003/android/FootprintBleSpike/app/src/main/java/com/footprint/ble/spike/BleConstants.kt:6`
- **影響**: SPIKE-0003 の条件マトリクスには iOS-Android 組合せが含まれる。UUID を統一しないと、このシナリオの計測が全て失敗する。
- **修正**: どちらか一方に統一する。iOS 側のカスタム UUID に寄せるのが安全（Android の `0000fdde-...` は Bluetooth SIG の 16bit UUID 空間で、意図しない衝突リスクがある）。

### B-2: proximity_tag の命名が iOS/Android で不一致

Runbook が OS ごとに異なる proximity_tag を指定している。

| 条件 | iOS proximity_tag | Android proximity_tag |
|------|-------------------|----------------------|
| 同室 | `room` | `near` |
| 同フロア | `floor` | `mid` |
| 同建物 | `building` | `far` |

- **ファイル**: `spikes/spike-0003/runbook.md:29-42`
- **影響**: `summarize.py` は `(os, proximity_tag, ...)` でグループ集計する。同一条件でも iOS と Android が別グループになり、クロスプラットフォーム比較が困難になる。
- **修正案**: 両 OS で `room/floor/building` に統一するか、分析スクリプト側で正規化マッピングを追加する。runbook の修正が最もシンプル。

---

## High (品質に大きく影響)

### H-1: iOS — CoreBluetooth State Restoration が未実装

CoreBluetooth の restore identifier は設定済みだが、`willRestoreState()` delegate が未実装。

- **ファイル**: `BleManager.swift:39,48`（restore identifier 設定箇所）
- **影響**: iOS がメモリ不足でアプリを終了した場合、BLE scanning/advertising が復元されない。長時間の BG 計測（同建物 120 分）でアプリが OS kill された場合、計測が中断する。
- **SPIKE への影響**: 「BG で2時間」の条件で、中間のアプリ kill を検知できず、見かけ上の検出失敗と区別できない。

### H-2: iOS — Trial 外でもスキャンが常時稼働

`centralManagerDidUpdateState()` で `poweredOn` になると即座にスキャンを開始するが、検出イベントは trial 中のみ記録される。

- **ファイル**: `BleManager.swift:143`
- **影響**: Trial 外でもスキャンが走り続け、電池消費の測定にノイズが入る。SPIKE-0003 の「電池判定: BG 連続動作で 10%/日以下」の評価が不正確になる。

### H-3: Android — Runtime Permission の非同期処理と Service 起動の競合

`ensurePermissions()` で権限をリクエストした直後に Service を起動するが、ユーザーの許可応答を待たない。

- **ファイル**: `MainActivity.kt`（ensurePermissions → startService の流れ）
- **影響**: 権限未付与のまま BLE API を呼び出し、`SecurityException` が発生する可能性がある。実測時に「計測開始したが実はスキャンが動いていない」状態が起き得る。

### H-4: Android — Bluetooth 状態変更のリスナーがない

Service 起動後に Bluetooth が OFF にされた場合、Service は検知・リカバリしない。

- **ファイル**: `BleForegroundService.kt`
- **影響**: 実測中に端末の Bluetooth が何らかの理由で OFF になると、以後の検出が止まるがログにエラーが残らない。

---

## Medium (改善推奨)

### M-1: 双方向検出の非対称性がログに表れない

両プラットフォームとも advertise + scan を同時実行するが、ログには「自分が相手を検出した」記録のみ残る。A が B を検出しても、B が A を検出したかは B のログを見ないとわからない。

- **影響**: 片方向しか検出できないケースの分析が、分析スクリプトの現在の集計ロジックだけでは困難。
- **推奨**: ログに検出元・検出先の peer_id を一貫して記録し、分析スクリプトでペアリングマッチングを追加する。

### M-2: Trial 自動終了機能がない

`duration_target_min` は metadata としてログに記録されるだけで、指定時間後の自動終了は実装されていない。

- **影響**: 手動操作に依存するため、オペレータが終了を忘れると計測条件が崩れる。
- **推奨**: Spike 用途では手動で十分だが、runbook に「アラームを設定する」等の注意を追記すると良い。

### M-3: 分析スクリプト — P(N) が duration_target_min 内の検出成功率のみ

現在の `summarize.py` は「N 分以内に first_detect が出たか」を P(N) としているが、RFC-0003 の定義「N 分の持続近接で少なくとも1回検出できる確率」は、trial 開始後 N 分間の任意の検出を含む。

- **ファイル**: `summarize.py:322-325`
- **影響**: duration_target_min を正確に設定すれば現在の実装でも問題ないが、「trial 開始 ≠ 近接開始」のケース（配置後に trial 開始するタイムラグ）で差が出る可能性がある。

### M-4: iOS — AllowDuplicatesKey=true のログ量

`CBCentralManagerScanOptionAllowDuplicatesKey: true` により、同一デバイスからの全 advertisement パケットが個別にログされる。

- **ファイル**: `BleManager.swift:120`
- **影響**: 2 時間の計測で数千〜数万行の detect イベントが生成される可能性がある。ストレージ圧迫と分析時のパフォーマンスに影響。
- **推奨**: detect イベントのサンプリング（例: 30 秒に 1 回のみ記録）を検討。

### M-5: TASK 生成物の一部が未作成

TASK-0310/0311 は `artifacts/runs/TASK-0310-*.out.md` / `TASK-0311-*.out.md`（ビルド手順・注意点）の生成を要求しているが、リポジトリに存在しない。

---

## Low (改善があれば望ましい)

### L-1: iOS — LogWriter のエラーが握りつぶされる

書き込み失敗時に `print()` で出力するのみで、ユーザーへの通知やフォールバックがない。

### L-2: Android — ログファイルのローテーションがない

JSONL ファイルが無制限に増大する。長期計測でストレージ問題が発生する可能性。

### L-3: 分析スクリプト — t_detect の単位表記

Group summary テーブルの `t_detect_min/median/mean/max` は分単位だが、列名 `t_detect_min` が「最小値（minimum）」と「分（minute）」で紛らわしい。

---

## RFC-0003 レビュー

### 評価: Good

RFC-0003 の論理構成は明確で、以下の点が評価できる。

1. **問題の再定義が的確**: 「すれ違い数秒」から「持続的近接」へのフレーム転換は、能登半島地震の実態（倒壊家屋、孤立集落、避難所）と整合する。
2. **確率モデルが検証可能**: P = 1 − (1 − p)^n は単純だが、パラメータ（p, スキャン間隔）を実測で埋められる構造。
3. **成功判定が明確**: P(5) ≥ 80%, P(30) ≥ 95% という閾値は、仮説の検証に十分な基準。
4. **Non-goals が適切**: BG すれ違い数秒の保証引き上げを明示的に除外している。
5. **Safety Check の3点（no guessing, no claiming, blank remains explicit）** が確認されている。

### 指摘事項

- 確率モデルの前提「各回のスキャンは独立」は、iOS BG の制約（State Preservation and Restoration, app nap）により崩れる可能性がある。実測で検証するため致命的ではないが、RFC に「独立性の仮定は実測で検証する」と明記するとより堅牢。
- HIGH_PROB という新しい保証レベルの定義（95%? 99%? 条件付き?）が曖昧。DEC-0008 更新時に数値基準を伴う定義が必要。

---

## Runbook レビュー

### 評価: Good

- 条件マトリクス（同室/同フロア/同建物）と手順が再現可能な粒度で記述されている。
- BG-LOCK / BG-LPM の OS 操作手順が明確。
- ログ回収手順（Share Logs）が簡潔。

### 指摘事項

- **B-2 再掲**: proximity_tag が iOS/Android で異なる命名。統一が必要。
- FG（フォアグラウンド）条件がマトリクスに含まれていない。BG との比較ベースラインとして FG 計測を含めると、BG 制約の影響を定量化しやすい。
- 「同時に Start Trial」の許容誤差（30 秒以内）は記載あるが、実測で T_detect が 30 秒未満のケースと区別できない可能性がある。runbook に「NTP 同期を推奨」等の注意があると良い。

---

## 分析スクリプト (summarize.py) レビュー

### 評価: Good

- 標準ライブラリのみ（pip 不要）で TASK-0312 の要件を満たす。
- `--demo` モードで動作確認済み（summary.md + summary.csv 生成を確認）。
- パースエラー、欠損フィールドの統計をレポートに含めており、データ品質の把握に有用。
- `normalize_low_power()` で iOS (`low_power: bool`) と Android (`low_power_tag: str`) の差異を吸収している。

### 指摘事項

- iOS は `app_state: "background"`, Android は `app_state: "service"` と異なるが、分析上は同じ「BG 動作中」を意味する。正規化マッピングがあると良い。
- `format_ratio()` の出力 `"0/1=0.00"` は Markdown テーブル内で読みやすいが、CSV 出力では P(N) が raw_rows に含まれていない（グループ集計のみ）。

---

## タスク定義の整合性

| 項目 | TASK-0310 (iOS) | TASK-0311 (Android) | 実装との整合 |
|------|-----------------|---------------------|-------------|
| JSONL フィールド | 要件通り | 要件通り | OK |
| BG 対応 | UIBackgroundModes | Foreground Service | OK |
| UI 要件 | Start/End/Share/Settings | Start/End/Share/Settings + Service制御 | OK |
| first_detect | 実装あり | 実装あり | OK |
| out.md 生成 | 未生成 | 未生成 | **Gap** (M-5) |
| Service UUID 統一 | 未指定 | 未指定 | **Blocker** (B-1) |

タスク定義自体に Service UUID の統一要件が記載されていなかった点が根本原因。タスクが独立して iOS/Android に割り振られた際、共通定数の合意プロセスが欠落した。

---

## 推奨アクション（優先順）

1. **[Blocker] Service UUID を統一する** — iOS 側の UUID `6A8F0C6C-...` に Android を合わせる
2. **[Blocker] proximity_tag を統一する** — 両 OS で `room/floor/building` に統一
3. **[High] iOS: `willRestoreState()` delegate を実装する** — 長時間 BG 計測の信頼性確保
4. **[High] iOS: Trial 外のスキャン停止** — 電池消費測定の正確性
5. **[High] Android: Permission grant を確認後に Service 起動** — 計測開始の信頼性
6. **[Medium] Runbook に FG ベースライン条件を追加** — BG 制約の影響定量化
7. **[Medium] detect イベントのサンプリング検討** — ログ量の制御

---

## 結論

SPIKE-0003 の設計方針（RFC-0003）は論理的に筋が通っており、計測キット（runbook + 分析スクリプト）の構成も適切である。しかし、**Service UUID の不一致**と **proximity_tag の命名不整合**という2つの Blocker が存在し、このまま実測に入ると iOS-Android 間の検出データが得られない。

Blocker 修正後は、実測に移行可能な状態にある。
