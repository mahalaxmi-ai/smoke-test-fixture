# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Project Discovery Report (task-0)

### Top-Level Structure

| Path | Description |
|------|-------------|
| `fixture-crate/` | Rust library crate (workspace member) |
| `.git/` | Git repository data |
| `Cargo.toml` | Workspace root manifest |
| `S1-*.json` | Sprint manifest files (Phase 1/Phase 2) |
| `smoke_output.txt` | Smoke test output artifact |
| `verify_smoke_output.sh` | Verification script |
| `worker_*.txt` | Worker output files |
| `domain_test.txt`, `routing_test.txt` | Test domain/routing artifacts |

### Tech Stack

- **Language:** Rust
- **Build system:** Cargo (workspace, resolver v2)
- **Crates:** `fixture-crate v0.1.0`

### Build/Run Commands

- **Build check:** `cargo check`
- **Build:** `cargo build`
- **Test:** `cargo test`

### Verification Results

- Build: passes with no errors or warnings
- Existing TODO/FIXME/HACK markers: none found
- `smoke_output.txt` contains `SMOKE_TEST_PASS` as required

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
