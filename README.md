# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.

## Verification Report (task-0)

### (a) Project Structure
- **Workspace root**: `Cargo.toml` — workspace with member `fixture-crate`, resolver v2
- **Crate**: `fixture-crate` v0.1.0, edition 2021, no external dependencies
- **Source**: `fixture-crate/src/main.rs` — two public functions (`add`, `multiply`) and a `main` entry point
- **Tests**: 10 unit tests covering add/multiply with positive, negative, zero, and boundary inputs
- **Manifests**: Sprint manifest system (S1-003) with ROADMAP, PHASE1, and PHASE2 JSON files

### (b) Compilation Status
- `cargo check`: **PASS** — compiles without errors or warnings

### (c) Test Results
- `cargo test`: **PASS** — 10/10 tests passed, 0 failed, 0 ignored

### (d) Quality Constraint Violations
- No TODO, FIXME, HACK, or placeholder comments found in source files
- No hardcoded secrets, credentials, or API keys found
- No unhandled `unwrap()` or `expect()` calls in non-test code
- All functions use infallible operations (integer arithmetic) — no fallible paths to handle

### (e) Recommended Next Steps
- Project is in a clean, verified state and ready for feature development
- The sprint manifest (S1-003) defines a two-phase approach: Phase 1 (infrastructure) then Phase 2 (features, depends on Phase 1)
