# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Project Structure

```
.
├── Cargo.toml                  # Workspace root (members: fixture-crate)
├── fixture-crate/
│   ├── Cargo.toml              # Package: fixture-crate v0.1.0 (edition 2021)
│   └── src/
│       └── main.rs             # Entry point with add() and multiply() functions
├── S1-*-*.json                 # Sprint manifest files (Phase 1 & Phase 2)
├── TEST-INVALID.json           # Invalid manifest for negative testing
├── VERIFICATION_SUMMARY.txt    # Smoke test verification summary
├── verify_smoke_output.sh      # Verification script
├── worker_a.txt                # Worker output fixtures
├── worker_b.txt
├── worker_c.txt
├── worker_files_test_report.txt
├── domain_test.txt
├── routing_test.txt
└── smoke_output.txt
```

## Build and Test

Requires Rust toolchain (edition 2021).

```sh
cargo build
cargo test
```

All 10 unit tests cover `add` and `multiply` functions including positive numbers, negative numbers, zero, and boundary conditions.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.
