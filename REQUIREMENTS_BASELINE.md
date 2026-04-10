# Requirements Baseline

## Project Summary

This repository is a **CI fixture** for the Mahalaxmi AI Terminal Orchestration system. It serves as the target project for smoke test scenarios, containing a minimal Rust workspace that orchestration workers operate against.

## Project Structure

```
/
├── .git/                        # Git repository
├── .gitignore                   # Git ignore rules
├── Cargo.toml                   # Rust workspace root (members: fixture-crate)
├── README.md                    # Project overview
├── S1-001-000-ROADMAP.json      # Sprint manifest (roadmap)
├── S1-002-000-CIRCULAR.json     # Sprint manifest (circular)
├── S1-003-000-ROADMAP.json      # Sprint manifest (roadmap)
├── S1-003-001-PHASE1.json       # Sprint manifest (phase 1)
├── S1-003-002-PHASE2.json       # Sprint manifest (phase 2)
├── TEST-INVALID.json            # Invalid test manifest
├── VERIFICATION_SUMMARY.txt     # Worker file verification report
├── domain_test.txt              # Domain test output
├── fixture-crate/               # Rust crate
│   ├── Cargo.toml               # Crate manifest (fixture-crate v0.1.0, edition 2021)
│   └── src/
│       └── main.rs              # Entry point
├── routing_test.txt             # Routing test output
├── smoke_output.txt             # Smoke test pass marker
├── verify_smoke_output.sh       # Smoke verification script
├── worker_a.txt                 # Worker A output (TEXT_A)
├── worker_b.txt                 # Worker B output (TEXT_B)
├── worker_c.txt                 # Worker C output (TEXT_C)
└── worker_files_test_report.txt # Worker files test report
```

## Languages and Frameworks

| Language | Evidence | Details |
|----------|----------|---------|
| Rust | `Cargo.toml` workspace + `fixture-crate/Cargo.toml` | Edition 2021, workspace with one member crate (`fixture-crate` v0.1.0) |
| Shell | `verify_smoke_output.sh` | Smoke test verification script |
| JSON | `S1-*.json`, `TEST-INVALID.json` | Sprint manifest definitions for the two-phase sprint system |

## Test Infrastructure

- **Smoke tests**: The file `smoke_output.txt` contains `SMOKE_TEST_PASS`, verified by `verify_smoke_output.sh`.
- **Worker verification**: `VERIFICATION_SUMMARY.txt` documents that `worker_a.txt`, `worker_b.txt`, and `worker_c.txt` were verified with expected content.
- **Domain and routing tests**: `domain_test.txt` and `routing_test.txt` exist as test output artifacts.
- **No unit test framework detected**: The Rust crate has no `[dev-dependencies]` and no `tests/` directory. Testing is done at the orchestration level, not within this fixture.

## Gaps and Missing Documentation

- **No CONTRIBUTING guide**: No contribution guidelines are present (expected for a CI-managed fixture).
- **No LICENSE file**: No license file found at the repository root.
- **No Rust tests**: The `fixture-crate` has no unit or integration tests; this is by design since the repo is a CI fixture, not a production crate.
- **No CI configuration in-repo**: CI is managed externally by the Mahalaxmi orchestration system rather than via in-repo workflow files.

## Branches

- `main` — README and fixture content
- `smoke-base` — Clean baseline branch that smoke tests reset to before each run

## Notes

This repository is managed by CI automation. Manual commits may interfere with smoke test reproducibility. The two-phase sprint manifest system (S1-003-001-PHASE1.json, S1-003-002-PHASE2.json) was created in the most recent commit.
