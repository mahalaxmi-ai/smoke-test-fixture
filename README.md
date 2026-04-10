# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Project Structure

| File / Directory | Purpose | Status |
|---|---|---|
| `Cargo.toml` | Rust workspace root; declares `fixture-crate` member | Valid |
| `fixture-crate/Cargo.toml` | Package manifest for `fixture-crate` (v0.1.0, edition 2021) | Valid |
| `fixture-crate/src/main.rs` | Source: `add` and `multiply` functions, `main` entry point, unit tests | Compiles, 10/10 tests pass |
| `S1-001-000-ROADMAP.json` | Sprint manifest / roadmap (scenario 1) | Present |
| `S1-002-000-CIRCULAR.json` | Sprint manifest — circular dependency test | Present |
| `S1-003-000-ROADMAP.json` | Sprint manifest — roadmap (scenario 3) | Present |
| `S1-003-001-PHASE1.json` | Sprint manifest — Phase 1 requirements | Present |
| `S1-003-002-PHASE2.json` | Sprint manifest — Phase 2 requirements | Present |
| `TEST-INVALID.json` | Invalid manifest for error-handling tests | Present |
| `smoke_output.txt` | Smoke test output artifact | Present |
| `domain_test.txt` | Domain routing test data | Present |
| `routing_test.txt` | Routing test data | Present |
| `worker_a.txt` | Worker output artifact (worker A) | Present |
| `worker_b.txt` | Worker output artifact (worker B) | Present |
| `worker_c.txt` | Worker output artifact (worker C) | Present |
| `worker_files_test_report.txt` | Worker file-creation test report | Present |
| `VERIFICATION_SUMMARY.txt` | Previous verification summary | Present |
| `verify_smoke_output.sh` | Shell script for verifying smoke outputs | Present |
| `.gitignore` | Git ignore rules | Present |

## Verification Report

- **Compilation**: `fixture-crate` compiles without errors or warnings.
- **Tests**: 10 tests executed, 10 passed, 0 failed.
- **Placeholder markers**: No `TODO`, `FIXME`, or `HACK` markers found in any source file.
- **Hardcoded secrets**: No passwords, API keys, tokens, or credentials found.
- **Error handling**: All functions (`add`, `multiply`) are infallible pure arithmetic operations; `main` performs only a print statement. No fallible operations require error handling.
- **Source quality**: Code is clean, well-documented, and fully tested.

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
