# TASK-0312-agent2-spike-0003-runbook-analysis

## 目的（SR）
SPIKE-0003 を「実測→数値化→RFC Accept/Reject判断」まで流せるよう、共通runbookとログ集計スクリプトを追加する。
（実機計測は人間が行うが、集計は自動化する。）

## 前提（不変条件）
- spike ブランチ上で作業（AGENTS.md §9.1）
- SSOT更新なし（結果が出た後に DEC/SSOT反映を検討）
- “成功/失敗（二値）”ではなく、T_detect と P(N) を算出できる形にする（SPIKE-0003定義に従う）

## 変更範囲（Diff最小）
- 新規: `spikes/spike-0003/` 配下のみ
- `./ci.sh` 変更禁止

## 追加物
1) `spikes/spike-0003/runbook.md`
- 実測条件（同室5分 / 同フロア30分 / 同建物2時間）を“どうやって揃えるか”の手順だけを書く
- 1試行の開始/終了、端末配置、proximity_tag の付け方、ログ回収方法
- iOS/Android それぞれの「BG-LOCK」「BG-LPM」再現手順（OS設定操作だけ、運用プロトコルは増やさない）

2) `spikes/spike-0003/analysis/summarize.py`
- 入力: iOS/Android の jsonl ログ（複数ファイル対応）
- 出力:
  - 条件（os, proximity_tag, duration_target_min, distance_m_tag, app_state/low_power）ごとの
    - 試行数、成功数（N分内に1回以上検出）、P(N)
    - T_detect の統計（min/median/mean/max）と raw 列（CSVでも可）
  - 生成物を `spikes/spike-0003/analysis/out/summary.md` と `summary.csv` に出す
- 計算ルール:
  - trial_start〜（first_detectが出るまで）の差分を T_detect
  - “N分内成功”は first_detect が N分以内に存在すること
- 依存は標準ライブラリのみ（pip不要）

3) `artifacts/runs/SPIKE-0003-sustained-proximity.out.md`（テンプレではなく“実測手順/集計手順込み”の結果台紙）
- 問い/結果/発見/仕様への示唆/証跡 を、AGENTS.md §9.1 の spike結果テンプレに従って作る
- 結果欄は空でも良いが、集計スクリプト出力（summary.md）を貼れば埋まる形にする

## 受け入れ（TST）
- TST-0312-1: runbook.md が「人間がこの通りやれば同じ条件で回せる」粒度になっている
- TST-0312-2: summarize.py がサンプル（ダミーでも）jsonlで動き、summary.md/csvを生成できる

## 生成物
- `spikes/spike-0003/runbook.md`
- `spikes/spike-0003/analysis/summarize.py`
- `artifacts/runs/SPIKE-0003-sustained-proximity.out.md`
- `artifacts/runs/TASK-0312-agent2-spike-0003-runbook-analysis.out.md`

