Updated `ci.sh` to make subdir failures emit a concrete rerun hint and to simplify subdir invocations, keeping the always_on/optional/guard phases intact. In `ci.sh`, `run_in_dir` now marks work as executed and reports `cd <dir> && <cmd>` on failure, and the npm/yarn + Android call sites now use the helper directly for clearer intent. Logged the run in `artifacts/runs/TASK-0200-agent4-ci.out.md`.

Tests run:
- `./ci.sh`

Next steps (optional):
1) Review the diff in `ci.sh`.
2) Commit if it looks good.