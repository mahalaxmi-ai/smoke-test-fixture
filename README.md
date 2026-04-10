# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Workspace Audit — 2026-04-10

### File Inventory

| File | Purpose |
|------|---------|
| `.gitignore` | Git ignore rules |
| `Cargo.toml` | Rust workspace manifest |
| `S1-001-000-ROADMAP.json` | Sprint roadmap definition |
| `S1-002-000-CIRCULAR.json` | Circular dependency test fixture |
| `S1-003-000-ROADMAP.json` | Sprint roadmap definition (phase 3) |
| `S1-003-001-PHASE1.json` | Phase 1 sprint requirements |
| `S1-003-002-PHASE2.json` | Phase 2 sprint requirements |
| `TEST-INVALID.json` | Invalid JSON test fixture |
| `VERIFICATION_SUMMARY.txt` | Verification output from prior runs |
| `domain_test.txt` | Domain routing test artifact |
| `fixture-crate/Cargo.toml` | Rust crate manifest for fixture |
| `fixture-crate/src/main.rs` | Rust main entry point |
| `routing_test.txt` | Routing test artifact |
| `smoke_output.txt` | Smoke test pass signal |
| `verify_smoke_output.sh` | Smoke output verification script |
| `worker_a.txt` | Worker A output artifact |
| `worker_b.txt` | Worker B output artifact |
| `worker_c.txt` | Worker C output artifact |
| `worker_files_test_report.txt` | Worker file test report |

### Security Scan

No hardcoded secrets, credentials, or API keys were found in any source files.

### Code Quality

No TODO, FIXME, or HACK markers were found in source files. The Rust source in `fixture-crate/src/main.rs` uses a minimal main function with no unhandled error paths.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.
