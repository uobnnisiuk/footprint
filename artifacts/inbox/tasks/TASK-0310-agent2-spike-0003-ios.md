# TASK-0310-agent2-spike-0003-ios

## 目的（SR）
SPIKE-0003 用に、iOS実機で BG BLE（central+peripheral）を動かし、JSONLログを端末内に残して共有できる最小iOSアプリを作る。
成果物は spike 用ディレクトリに隔離し、`./ci.sh` は壊さない。

## 前提（不変条件）
- この変更は spike 用（AGENTS.md §9.1）。main にマージしない前提で spike ブランチ上で作業する。
- “すれ違い3秒”は成功判定に含めない。持続的近接（同室5分/同フロア30分/同建物2h）の計測を主眼にする（docs/backlog/spike-backlog.md: SPIKE-0003）。
- SSOT（docs/constitution/*）は今回は更新しない（計測キット生成のみ）。

## 変更範囲（Diff最小）
- 新規: `spikes/spike-0003/ios/FootprintBleSpike/` 以下に全て隔離
- `./ci.sh` 変更禁止（spikeディレクトリはCI対象外でよい）

## 実装要件（最小アプリ仕様）
- Swift / SwiftUI アプリ（Xcodeでビルドできる構成）
- CoreBluetoothで以下を同時に動かす
  - advertise: Service UUID を含む広告（LocalNameも可）
  - scan: Service UUID フィルタでスキャン（重複許可）
- BG対応:
  - Info.plist: `UIBackgroundModes` に `bluetooth-central` と `bluetooth-peripheral`
  - 権限文言: `NSBluetoothAlwaysUsageDescription`
- ログ（JSONL）:
  - 端末内 Documents に `ble_spike.jsonl` を追記
  - 1行1JSON、UTF-8
  - 必須フィールド:
    - ts（RFC3339）
    - trial_id（Start Trialで生成）
    - device_role（A/B）
    - os="iOS"
    - app_state（foreground/background）
    - low_power（true/false）
    - proximity_tag（room/floor/building/other）※UIで選択して付与
    - distance_m_tag（1/3/10/other）※UIで選択
    - duration_target_min（5/30/120/other）※UIで選択
    - peer_id（初回起動で生成して永続、UUID等）
    - event_type（trial_start / first_detect / detect / trial_end）
    - rssi（取れるとき）
    - battery_pct（UIDevice batteryLevel を % に換算。monitoring有効化）
- UI（最低限）:
  - Start Trial ボタン（trial_id発行＋trial_startログ）
  - End Trial ボタン（trial_endログ）
  - “Share Logs” ボタン（ShareSheetで jsonl を共有できる）
  - 設定UI: device_role / proximity_tag / distance_m_tag / duration_target_min
- first_detect 判定:
  - trial中に初めてその peer（または任意）を検出した瞬間を first_detect としてログ
  - 以後の検出は detect
  - 少なくとも「trial開始から最初の検出までの時間」が後で計算できる粒度で残す

## 受け入れ（TST）
- TST-0310-1: iOS実機でビルドでき、前景で advertise/scan が動き、検出ログが出る
- TST-0310-2: アプリをBG（ホーム/ロック）にしても一定時間ログが増える（成立/不成立どちらでも“観測”できる）
- TST-0310-3: ログファイルを端末から共有できる

## 生成物
- `spikes/spike-0003/ios/FootprintBleSpike/**`
- `artifacts/runs/TASK-0310-agent2-spike-0003-ios.out.md`（実装要約・ビルド手順・注意点）

