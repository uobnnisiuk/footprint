# TASK-0311-agent3-spike-0003-android

## 目的（SR）
SPIKE-0003 用に、Android実機で（画面OFF/ロック中を含む）BLE advertise+scan を継続し、JSONLログを端末内に残して共有できる最小Androidアプリを作る。
成果物は spike 用ディレクトリに隔離し、`./ci.sh` は壊さない。

## 前提（不変条件）
- spike ブランチ上で作業（AGENTS.md §9.1）
- SSOT更新なし（計測キット生成のみ）
- OS制約の回避ハックは禁止。Android公式の範囲で「ロック中も動作」を実現する（= Foreground Service は可）

## 変更範囲（Diff最小）
- 新規: `spikes/spike-0003/android/FootprintBleSpike/` 以下に全て隔離
- `./ci.sh` 変更禁止（spikeディレクトリはCI対象外でよい）

## 実装要件（最小アプリ仕様）
- Kotlin / Android Studio で開ける Gradle プロジェクト
- BLE:
  - Advertise: Service UUID を含む
  - Scan: Service UUID フィルタ
- BG（ロック中）計測のため、Foreground Service を採用
  - 常駐通知を出しつつ advertise/scan を継続
- 権限:
  - Android 12+ の BLUETOOTH_SCAN / BLUETOOTH_ADVERTISE / BLUETOOTH_CONNECT
  - 位置権限が必要な場合は最小で（要否を明示）
- ログ（JSONL）:
  - appの files/ に `ble_spike.jsonl` 追記
  - 1行1JSON、UTF-8
  - 必須フィールド（iOSと揃える）:
    - ts, trial_id, device_role, os="Android", app_state（fg/service）, low_power_tag（UIで付与でも可）
    - proximity_tag, distance_m_tag, duration_target_min
    - peer_id（初回生成して永続）
    - event_type（trial_start / first_detect / detect / trial_end）
    - rssi（ScanResult）
    - battery_pct（BatteryManager）
- UI（最低限）:
  - Start Trial / End Trial
  - Start Service / Stop Service
  - Share Logs（ACTION_SENDでjsonl共有）
  - device_role / proximity_tag / distance_m_tag / duration_target_min をUIで選択

## 受け入れ（TST）
- TST-0311-1: Android実機でビルドでき、前景で検出ログが出る
- TST-0311-2: ロック中でも（Foreground Serviceで）一定時間ログが増える（成立/不成立どちらでも観測できる）
- TST-0311-3: ログを端末から共有できる

## 生成物
- `spikes/spike-0003/android/FootprintBleSpike/**`
- `artifacts/runs/TASK-0311-agent3-spike-0003-android.out.md`（ビルド手順・注意点）

