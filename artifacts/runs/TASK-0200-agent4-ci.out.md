Added a specific failure hint when `./ci.sh` runs a command inside a directory that doesn’t exist, so CI errors point to a concrete fix without changing the overall gate flow.

- Updated `ci.sh` to fail fast with a targeted hint in `run_in_dir` when the directory is missing, keeping the always_on/optional/guard structure intact.
- Logged the run in `artifacts/runs/TASK-0200-agent4-ci.out.md`.

Tests run:
- `./ci.sh`

If you want, I can also capture the full CI output into the run log, or prepare a commit.