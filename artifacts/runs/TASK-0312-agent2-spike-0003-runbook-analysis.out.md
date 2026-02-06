Added a SPIKE-0003 runbook, a stdlib-only log summarizer, and a results worksheet so the spike can flow from measurement to quantized output without touching SSOT or `ci.sh`.

- Repeatable measurement protocol with condition alignment, per-trial steps, and OS-only BG-LOCK/BG-LPM actions in `spikes/spike-0003/runbook.md`.
- jsonl summarizer that groups by os/proximity/duration/distance/app_state/low_power and writes `summary.md` + `summary.csv` in `spikes/spike-0003/analysis/summarize.py`.
- Spike results worksheet including runbook + analysis steps in `artifacts/runs/SPIKE-0003-sustained-proximity.out.md`.
- Task run log in `artifacts/runs/TASK-0312-agent2-spike-0003-runbook-analysis.out.md`.

Tests:
- `python3 spikes/spike-0003/analysis/summarize.py --demo --out-dir /tmp/spike-0003-demo`

Next steps:
1. Run the real trials per `spikes/spike-0003/runbook.md` and export the jsonl logs.
2. Run the summarizer on those logs and paste the generated summary into `artifacts/runs/SPIKE-0003-sustained-proximity.out.md`.