# SPIKE-0003: sustained proximity

## Question
- Can BLE detection sustain within N minutes at room/floor/building separation?
- Metrics: T_detect and P(N) by os/proximity_tag/duration_target_min/distance_m_tag/app_state/low_power.

## Runbook (measurement)
- Runbook: `spikes/spike-0003/runbook.md`
- Condition set: same room (5 min), same floor (30 min), same building (120 min)

## Aggregation (analysis)
1. Collect jsonl logs from iOS and Android.
2. Run:
   - `python3 spikes/spike-0003/analysis/summarize.py <log1.jsonl> <log2.jsonl> ...`
3. Outputs:
   - `spikes/spike-0003/analysis/out/summary.md`
   - `spikes/spike-0003/analysis/out/summary.csv`

## Results
- Paste the contents of `spikes/spike-0003/analysis/out/summary.md` here.

## Findings / constraints
- (fill after measurement)

## Implications for spec
- (DEC candidate / OPEN updates)

## Evidence
- Log files: (paths / filenames)
- Summary outputs: `spikes/spike-0003/analysis/out/summary.md`, `spikes/spike-0003/analysis/out/summary.csv`
- Branch / commit: (fill in)
