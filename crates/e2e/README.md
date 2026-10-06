# crates/e2e

An internal runner for End-to-End (E2E) testing and fixture updating.

## Why not put these tests in `tests/*.rs`?

1. **Target mismatch**:
   - `cargo test` runs for the host machine (e.g. x86_64 Linux).
   - This plugin runs as a WebAssembly (`wasm32-unknown-unknown`) file inside the `dprint` CLI.
   - Cargo cannot build for another target during `cargo test` on stable Rust.
2. **Lifecycle separation**:
   - The E2E test needs a built `.wasm` file to run with the `dprint` CLI.
   - If we put these tests in `tests/*.rs`, `cargo test` (unit tests) will fail whenever the `.wasm` file is not built yet.
3. **Fixture updating (`bump`)**:
   - This tool also updates expected files (`bump`). Standard `#[test]` does not support updating fixtures.

Therefore, we keep this runner as a workspace tool and run it via xtask:
`cargo x build` -> `cargo x test-e2e`.

## Scope and test strategy

E2E tests serve as minimal integration smoke tests to ensure built Wasm binaries
run correctly inside the `dprint` CLI:

- `tests/v2-official`: Verifies baseline KDL v2 formatting (default).
- `tests/v1-zellij`: Verifies KDL v1 formatting via plugin configuration.
- `tests/mixed-versions`: Verifies mixed KDL versions formatted in the same project via version markers.

The `dprint check` step in each test naturally verifies that fixture `dprint.json`
files conform to plugin and CLI expectations.

Configuration option variations and edge cases are covered by spec tests in `tests/specs/`
(run via `cargo test`) to keep E2E fixtures lightweight and avoid frequent bump churn.

## Usage

```sh
# Run all E2E tests
cargo run --package e2e -- check

# Run a specific fixture test
cargo run --package e2e -- check v2-official

# Update all expected fixture files
cargo run --package e2e -- bump

# Update a specific fixture file
cargo run --package e2e -- bump v2-official
```
