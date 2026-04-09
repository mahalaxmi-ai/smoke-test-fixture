# Mahalaxmi Smoke Test Fixture

**Initialized:** 2026-04-09

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).
It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Requirements

Awaiting specification

## Project Structure

```
.
├── .gitignore
├── Cargo.toml
├── README.md
├── S1-001-000-ROADMAP.json
├── S1-002-000-CIRCULAR.json
├── S1-003-000-ROADMAP.json
├── S1-003-001-PHASE1.json
├── S1-003-002-PHASE2.json
├── TEST-INVALID.json
├── VERIFICATION_SUMMARY.txt
├── domain_test.txt
├── fixture-crate/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── routing_test.txt
├── smoke_output.txt
├── verify_smoke_output.sh
├── worker_a.txt
├── worker_b.txt
├── worker_c.txt
└── worker_files_test_report.txt
```

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.
