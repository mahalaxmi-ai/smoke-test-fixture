# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Verification Report (task-0)

**Files found:**
- `Cargo.toml` — workspace manifest (members: fixture-crate, resolver v2)
- `fixture-crate/Cargo.toml` — package manifest (fixture-crate v0.1.0, edition 2021)
- `fixture-crate/src/main.rs` — source entry point (add, multiply functions + 10 tests)
- `S1-001-000-ROADMAP.json` — sprint S1-001 roadmap manifest
- `S1-002-000-CIRCULAR.json` — sprint S1-002 circular dependency test manifest
- `S1-003-000-ROADMAP.json` — sprint S1-003 roadmap manifest
- `S1-003-001-PHASE1.json` — Phase 1 requirements
- `S1-003-002-PHASE2.json` — Phase 2 requirements
- `TEST-INVALID.json` — invalid manifest for testing
- `VERIFICATION_SUMMARY.txt`, `verify_smoke_output.sh` — verification artifacts
- `*.txt` worker output files (domain_test, routing_test, smoke_output, worker_a/b/c)

**Build status:** Compiles successfully (cargo build OK)
**Test status:** All 10 tests pass (cargo test OK)
**Error handling:** No unwrap() calls on fallible operations found
**Code quality:** No TODO/FIXME/HACK/placeholder comments found
**Security:** No hardcoded secrets, credentials, or API keys found
**Circular dependency manifest (S1-002-000-CIRCULAR.json):** Present and correctly structured with cycle S1-002-001 -> S1-002-002 -> S1-002-003 -> S1-002-001

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
