`./ci.sh` already meets the cycle-2 goals (clear phases, actionable failure hints, no-op guard) so I kept the gate unchanged and just captured the run evidence. The run log is in `artifacts/runs/TASK-0200-agent4-ci.out.md`.

- Verified `./ci.sh` structure and hints; no minimal cleanup needed
- Ran `./ci.sh`; green with Rust tests executed

If you want a small readability tweak or additional hint wording, tell me which line(s) you want adjusted.