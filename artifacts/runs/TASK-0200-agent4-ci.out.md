Reviewed `./ci.sh`; the always_on/optional/guard structure and HINTs are already clear and concrete, so I left it unchanged and preserved the no‑op guard. Logged today’s run in `artifacts/runs/TASK-0200-agent4-ci.out.md`. Ran `./ci.sh` and it’s green.

- Tests: `./ci.sh`
- TST-0200-1: pass (green)
- TST-0200-2: pass (HINTs already specific; no change)
- TST-0200-3: pass (no‑op guard intact)

If you want a small refactor to reduce command-format duplication in `./ci.sh`, say the word.