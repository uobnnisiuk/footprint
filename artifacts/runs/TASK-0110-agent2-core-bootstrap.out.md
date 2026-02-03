Implemented `TASK-0110` in `core/` as requested.

- Added a Rust `core` crate with lib + CLI bin (`core/Cargo.toml:1`, `core/src/main.rs:5`).
- Implemented stdin Trace JSON intake, strict validation (required fields + unknown-field reject), canonical JSON handling, and L0 normalization (`core/src/lib.rs:148`, `core/src/lib.rs:257`).
- Implemented durable JSONL append-only persistence with path precedence:
  1) `--store`, 2) `FOOTPRINT_CORE_STORE_PATH`, 3) XDG/default fallback (`core/src/lib.rs:235`, `core/src/lib.rs:328`).
- Implemented stdout JSON result contract for success/error (`core/src/lib.rs:170`, `core/src/lib.rs:192`).
- Added integration and negative CLI tests per spec (`core/tests/cli_integration.rs:56`, `core/tests/cli_integration.rs:164`).

Validation run:
- `cargo test --manifest-path core/Cargo.toml` ✅
- `./ci.sh` ✅

If you want, next I can generate a commit message candidate for these new `core/**` files.