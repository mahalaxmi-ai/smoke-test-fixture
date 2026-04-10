# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Project Verification Summary (task-0)

### File Tree
```
.
├── Cargo.toml                    # Rust workspace root
├── README.md                     # This file
├── .gitignore
├── fixture-crate/
│   ├── Cargo.toml
│   └── src/main.rs               # add() and multiply() functions with tests
├── S1-001-000-ROADMAP.json       # Sprint S1-001 manifest (1 item, no deps)
├── S1-002-000-CIRCULAR.json      # Sprint S1-002 circular dependency test manifest
├── S1-003-000-ROADMAP.json       # Sprint S1-003 two-phase roadmap
├── S1-003-001-PHASE1.json        # Phase 1 manifest
├── S1-003-002-PHASE2.json        # Phase 2 manifest
├── TEST-INVALID.json             # Invalid manifest for testing
├── VERIFICATION_SUMMARY.txt      # Prior verification output
├── verify_smoke_output.sh        # Smoke test verification script
├── domain_test.txt
├── routing_test.txt
├── smoke_output.txt
├── worker_a.txt
├── worker_b.txt
├── worker_c.txt
└── worker_files_test_report.txt
```

### Technology Stack
- **Language**: Rust (2021 edition via resolver = "2")
- **Build System**: Cargo workspace
- **Framework**: None (minimal fixture crate)
- **Test Framework**: Built-in Rust `#[cfg(test)]` module

### Sprint Manifest Structure
- **S1-001**: Single-item sprint with no dependencies
- **S1-002**: Circular dependency test — three items forming a cycle (S1-002-001 -> S1-002-002 -> S1-002-003 -> S1-002-001). This manifest is intentionally invalid and should fail dependency validation.
- **S1-003**: Two-phase sprint with Phase 2 depending on Phase 1

### Circular Dependency Manifest (S1-002-000-CIRCULAR.json)
The manifest contains a valid structure with an intentional circular dependency cycle:
- `S1-002-001` depends on `S1-002-002`
- `S1-002-002` depends on `S1-002-003`
- `S1-002-003` depends on `S1-002-001`

This forms a complete cycle and should be rejected by any topological sort or DAG validation.

### Incomplete or Missing Implementations
- No incomplete implementations found in source code.
- All test cases pass (see test results below).

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
