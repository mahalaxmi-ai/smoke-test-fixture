# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Project Structure

### Sprint Manifest System

The repository contains a two-phase sprint manifest system:

- **S1-001-000-ROADMAP.json** — Sprint S1-001 single-item roadmap (baseline)
- **S1-002-000-CIRCULAR.json** — Sprint S1-002 circular dependency test case (3 items, cyclic deps)
- **S1-003-000-ROADMAP.json** — Sprint S1-003 two-phase roadmap (manifest_id: S1-003-000, version 1.0.0, 2 items, 1 dependency)
  - **S1-003-001-PHASE1.json** — Phase 1: Foundation Setup (domain: infrastructure, branch: feature/phase-1-foundation)
  - **S1-003-002-PHASE2.json** — Phase 2: Feature Implementation (domain: features, depends on S1-003-001)
- **TEST-INVALID.json** — Invalid manifest for negative testing

### Rust Fixture Crate

- **fixture-crate/** — Minimal Rust crate with `add` and `multiply` functions and comprehensive tests

### Worker and Verification Files

- **worker_a.txt**, **worker_b.txt**, **worker_c.txt** — Worker output verification files
- **VERIFICATION_SUMMARY.txt** — Worker file verification report
- **verify_smoke_output.sh** — Smoke test verification script
- **smoke_output.txt**, **domain_test.txt**, **routing_test.txt** — Test output files

## Verification Summary (task-0)

**Date:** 2026-04-10

### Files Examined

| File | Status |
|------|--------|
| S1-003-000-ROADMAP.json | Valid — manifest_id, sprint_id S1-003, title, version 1.0.0, 2 items, 1 dependency |
| S1-003-001-PHASE1.json | Valid — id, title, branch, repo_url, requirements, project_root, domain_id |
| S1-003-002-PHASE2.json | Valid — id, title, branch, repo_url, requirements, project_root, domain_id, depends on S1-003-001 |
| S1-001-000-ROADMAP.json | Valid — baseline sprint manifest |
| S1-002-000-CIRCULAR.json | Valid — circular dependency test manifest |
| TEST-INVALID.json | Valid — intentionally invalid for negative testing |
| fixture-crate/src/main.rs | Clean — no issues found |
| fixture-crate/Cargo.toml | Clean — standard Rust package config |
| VERIFICATION_SUMMARY.txt | Clean — worker verification report |
| verify_smoke_output.sh | Clean — verification script |
| worker_a.txt, worker_b.txt, worker_c.txt | Clean — worker outputs |
| smoke_output.txt, domain_test.txt, routing_test.txt | Clean — test outputs |

### Issues Found

None. All files pass quality checks:
- No unresolved markers (no TODO, FIXME, HACK, or placeholder comments)
- No hardcoded secrets, credentials, or API keys
- All functions have explicit error handling (Rust functions are infallible by design)
- Sprint manifest system S1-003 is complete with valid two-phase dependency chain

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
