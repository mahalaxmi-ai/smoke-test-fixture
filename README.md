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

### File Inventory

| File | Type | Description |
|------|------|-------------|
| README.md | Documentation | Project overview and verification report |
| Cargo.toml | Config | Rust workspace configuration |
| .gitignore | Config | Git ignore rules |
| fixture-crate/Cargo.toml | Config | Rust crate package manifest |
| fixture-crate/src/main.rs | Source | Core library with `add` and `multiply` functions and tests |
| S1-001-000-ROADMAP.json | Manifest | Sprint S1-001 requirements (1 item, no dependencies) |
| S1-002-000-CIRCULAR.json | Manifest | Sprint S1-002 circular dependency test (3 items, cyclic deps) |
| S1-003-000-ROADMAP.json | Manifest | Sprint S1-003 two-phase requirements (2 items, sequential dep) |
| S1-003-001-PHASE1.json | Manifest | Phase 1 foundation setup details |
| S1-003-002-PHASE2.json | Manifest | Phase 2 feature implementation details (depends on Phase 1) |
| TEST-INVALID.json | Manifest | Invalid manifest for validation testing |
| verify_smoke_output.sh | Script | Smoke output verification script |
| smoke_output.txt | Data | Smoke test output file |
| domain_test.txt | Data | Domain test marker |
| routing_test.txt | Data | Routing test marker |
| worker_a.txt | Data | Worker A output (TEXT_A) |
| worker_b.txt | Data | Worker B output (TEXT_B) |
| worker_c.txt | Data | Worker C output (TEXT_C) |
| worker_files_test_report.txt | Report | Worker files verification summary |
| VERIFICATION_SUMMARY.txt | Report | Prior verification results |

### Requirements Verification

#### S1-002-000-CIRCULAR.json (Circular Dependency Manifest)

| Requirement | Status | Detail |
|-------------|--------|--------|
| Valid manifest_id | PASS | `S1-002-000-CIRCULAR` |
| sprint_id is S1-002 | PASS | `S1-002` |
| Title present | PASS | `Sprint S1-002 Circular Dependencies Test` |
| Version 1.0.0 | PASS | `1.0.0` |
| Three items present | PASS | S1-002-001, S1-002-002, S1-002-003 |
| Cycle: S1-002-001 -> S1-002-002 | PASS | Dependency edge present |
| Cycle: S1-002-002 -> S1-002-003 | PASS | Dependency edge present |
| Cycle: S1-002-003 -> S1-002-001 | PASS | Dependency edge present (completes cycle) |
| Should fail validation | PASS | Circular dependency cycle is correctly formed |

### Code Quality Checks

| Check | Status | Detail |
|-------|--------|--------|
| No TODO/FIXME/HACK markers | PASS | Zero matches in all source files |
| No hardcoded secrets | PASS | Zero credentials, API keys, or tokens found |
| Error handling coverage | PASS | Shell scripts use `set -o pipefail` and explicit exit codes; Rust functions are pure (no fallible operations) |
| Test coverage | PASS | 9 test functions cover `add` and `multiply` with positive, negative, zero, and boundary cases |

### Summary

All verification checks pass. The S1-002-000-CIRCULAR.json manifest correctly implements the circular dependency cycle requirement with three items forming a complete cycle (S1-002-001 -> S1-002-002 -> S1-002-003 -> S1-002-001). This manifest is designed to fail dependency validation due to the circular reference.
