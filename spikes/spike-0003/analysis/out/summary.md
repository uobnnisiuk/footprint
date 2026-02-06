# SPIKE-0003 summary

Generated: 2026-02-06T09:38:05.721017+00:00

## Inputs
- /home/user/footprint/spikes/spike-0003/analysis/out/demo_logs/demo_android.jsonl (2 lines)
- /home/user/footprint/spikes/spike-0003/analysis/out/demo_logs/demo_ios.jsonl (2 lines)

## Parse notes
- total_lines: 4
- parsed_lines: 4
- parse_errors: 0
- missing_trial_id: 0
- missing_ts: 0
- missing_trial_start: 0
- negative_t_detect: 0

Success = first_detect occurs within duration_target_min minutes.
P(N) denominator uses trials with numeric duration_target_min.

## Group summary
| os | proximity_tag | duration_target_min | distance_m_tag | app_state | low_power | trials | success_within_target | P(N) | t_detect_n | t_detect_min | t_detect_median | t_detect_mean | t_detect_max |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Android | near | 5 | 2 | service | normal | 1 | 0 | 0/1=0.00 | 1 | 8.00 | 8.00 | 8.00 | 8.00 |
| iOS | room | 5 | 1 | background | low | 1 | 1 | 1/1=1.00 | 1 | 2.50 | 2.50 | 2.50 | 2.50 |

Raw rows: /home/user/footprint/spikes/spike-0003/analysis/out/summary.csv
