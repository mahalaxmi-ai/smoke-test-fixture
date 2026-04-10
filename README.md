# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Verification Report (task-0)

**Project Structure:** Rust workspace with one crate (`fixture-crate`), edition 2021. Entry point: `fixture-crate/src/main.rs`. Build system: Cargo. Test framework: built-in `#[test]`.

**README:** Present. Describes this repo as a CI fixture for Mahalaxmi AI Terminal Orchestration smoke tests.

**Code Quality:** No TODO, FIXME, HACK, or placeholder comments found anywhere in the codebase.

**Build:** `cargo build` succeeds with no errors or warnings.

**Tests:** `cargo test` — 10 tests passed, 0 failed. Tests cover `add` and `multiply` functions with positive, negative, zero, and boundary inputs.

**Routing Test:** `routing_test.txt` exists with content `ROUTING_OK`.

**Overall Health:** Project is clean and fully functional.

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
