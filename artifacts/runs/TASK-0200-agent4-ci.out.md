Added an explicit executable check for `android/gradlew` in `ci.sh` so failures give a concrete HINT while keeping the always_on/optional/guard structure unchanged, and updated the run log to reflect the run.

- Files touched: `ci.sh` (new `android/gradlew` executable check + HINT), `artifacts/runs/TASK-0200-agent4-ci.out.md` (run log update)
- Tests: `./ci.sh` (green)