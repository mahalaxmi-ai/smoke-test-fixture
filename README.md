# smoke-test-fixture

A minimal Rust workspace used as a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai). It provides a real codebase for orchestration workers to operate on during smoke test scenarios.

## Prerequisites

- **Rust toolchain** — edition 2021 or later (install via [rustup](https://rustup.rs))
- **Cargo** — included with the Rust toolchain

## Build Instructions

```bash
# Build the workspace
cargo build

# Run tests
cargo test

# Run the binary
cargo run -p fixture-crate
```

## Project Structure

```
.
├── Cargo.toml              # Workspace root (resolver v2)
├── fixture-crate/
│   ├── Cargo.toml          # Crate manifest (fixture-crate v0.1.0, edition 2021)
│   └── src/
│       └── main.rs         # Entry point, public helper functions, and unit tests
├── S1-*.json               # Sprint manifest files (Phase 1 / Phase 2 roadmaps)
├── verify_smoke_output.sh  # Smoke-test output verification script
└── README.md
```

### fixture-crate

The sole workspace member. It exposes two public arithmetic helpers (`add`, `multiply`) and includes a comprehensive unit test suite covering positive values, negative values, zero, and boundary conditions.

## Error Handling Patterns

Functions in this project use Rust's standard value-return pattern — pure functions that cannot fail return their result directly (no `Result` or `Option` wrapping). Because the public API surface consists of simple arithmetic on `i32`, there are no fallible operations or recoverable error paths. If the project grows to include I/O or fallible logic, prefer returning `Result<T, E>` with descriptive error types rather than panicking.

## Branches

| Branch | Purpose |
|--------|---------|
| `main` | Stable fixture content |
| `smoke-base` | Clean baseline that smoke tests reset to before each run |

## How Smoke Tests Use This Repo

1. A smoke test scenario clones or resets to `smoke-base`.
2. A Mahalaxmi orchestration cycle runs against this repo.
3. Outputs are validated by the test harness.
4. `scripts/reset-fixture.sh` (in the main Mahalaxmi repo) resets this fixture back to `smoke-base`.

## Configuration

No external configuration or environment variables are required. The project builds and runs with a default Rust toolchain and has no third-party dependencies.
