# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Repository Assessment (2026-04-09)

### Project Structure

```
.
├── Cargo.toml                       # Rust workspace root (resolver v2)
├── fixture-crate/
│   ├── Cargo.toml                   # fixture-crate v0.1.0, edition 2021
│   └── src/
│       └── main.rs                  # add(), multiply() functions + 10 unit tests
├── S1-001-000-ROADMAP.json          # Sprint manifest files
├── S1-002-000-CIRCULAR.json
├── S1-003-000-ROADMAP.json
├── S1-003-001-PHASE1.json
├── S1-003-002-PHASE2.json
├── TEST-INVALID.json
├── VERIFICATION_SUMMARY.txt
├── domain_test.txt
├── routing_test.txt
├── smoke_output.txt
├── verify_smoke_output.sh
├── worker_a.txt
├── worker_b.txt
├── worker_c.txt
├── worker_files_test_report.txt
├── .gitignore
└── README.md
```

### Build System

- **Type:** Rust workspace (Cargo)
- **Build status:** Pass (exit code 0)
- **Test status:** Pass — 10 tests, 0 failures

### Source Summary

The `fixture-crate` package provides two arithmetic functions (`add`, `multiply`) with comprehensive unit tests covering positive numbers, negative numbers, zero, and boundary conditions.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.
