# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Project Structure

- **fixture-crate/**: Minimal Rust crate with `add` and `multiply` functions, plus comprehensive unit tests (10 tests covering positive, negative, zero, and boundary cases).
- **S1-002-000-CIRCULAR.json**: Sprint manifest with circular dependency cycle (S1-002-001 -> S1-002-002 -> S1-002-003 -> S1-002-001) for validation testing.
- **Cargo.toml**: Workspace root referencing `fixture-crate`.

## Verification Status

- `cargo check`: passes (no compilation errors)
- `cargo test`: 10/10 tests pass
- `cargo clippy -- -D warnings`: no warnings
- No TODO/FIXME/HACK markers in source
- No hardcoded secrets in source
- No unsafe `.unwrap()` calls in production code

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
