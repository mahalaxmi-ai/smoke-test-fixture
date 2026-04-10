# Workspace Inventory

Generated: 2026-04-09

## Project Overview

This repository is a CI smoke-test fixture for Mahalaxmi AI Terminal Orchestration. It contains a minimal Rust workspace used as the target project for orchestration smoke test scenarios.

## Directory Structure

```
/
├── Cargo.toml                    # Workspace root manifest
├── README.md                     # Project documentation
├── .gitignore                    # Git ignore rules
├── fixture-crate/
│   ├── Cargo.toml                # Crate manifest (edition 2021)
│   └── src/
│       └── main.rs               # Main source file with add/multiply functions and tests
├── S1-001-000-ROADMAP.json       # Sprint manifest (roadmap, series 1)
├── S1-002-000-CIRCULAR.json      # Sprint manifest (circular dependency test)
├── S1-003-000-ROADMAP.json       # Sprint manifest (two-phase roadmap)
├── S1-003-001-PHASE1.json        # Sprint manifest (phase 1 requirements)
├── S1-003-002-PHASE2.json        # Sprint manifest (phase 2 requirements)
├── TEST-INVALID.json             # Invalid manifest for error-handling tests
├── VERIFICATION_SUMMARY.txt      # Worker file verification report
├── verify_smoke_output.sh        # Smoke test output verification script
├── smoke_output.txt              # Smoke test output artifact
├── domain_test.txt               # Domain routing test artifact
├── routing_test.txt              # Routing test artifact
├── worker_a.txt                  # Worker output file (TEXT_A)
├── worker_b.txt                  # Worker output file (TEXT_B)
├── worker_c.txt                  # Worker output file (TEXT_C)
└── worker_files_test_report.txt  # Worker files test report
```

## Source Files

### fixture-crate/src/main.rs

**Purpose:** Main entry point for the fixture crate. Contains arithmetic utility functions and a comprehensive test suite.

**Entry Point:** `fn main()` — prints a smoke test identifier string.

**Public Functions:**

| Function | Signature | Description |
|----------|-----------|-------------|
| `add` | `pub fn add(a: i32, b: i32) -> i32` | Returns the sum of two integers |
| `multiply` | `pub fn multiply(a: i32, b: i32) -> i32` | Returns the product of two integers |

**Test Coverage:** The `#[cfg(test)]` module includes 10 test functions covering:

- `add`: positive numbers, negative numbers, zero, boundary conditions
- `multiply`: positive numbers, negative numbers, zero, edge cases, required cases, specific required cases

### fixture-crate/Cargo.toml

**Purpose:** Crate-level manifest. Package name: `fixture-crate`, version `0.1.0`, Rust edition 2021. No external dependencies.

### Cargo.toml (root)

**Purpose:** Workspace manifest. Declares `fixture-crate` as the sole workspace member. Uses resolver version 2.

## Dependencies

No external crate dependencies. The project uses only Rust standard library types.

## External Configuration and Services

No external configuration files, environment variables, or service connections are referenced. The project is entirely self-contained.

## Sprint Manifest Files

The `S1-*.json` files are sprint manifest definitions consumed by the Mahalaxmi orchestration system. They define task routing, phasing, and dependency structures for smoke test scenarios. `TEST-INVALID.json` is an intentionally malformed manifest used to validate error handling in the orchestrator's manifest parser.

## Test Artifacts

The `.txt` files (`worker_a.txt`, `worker_b.txt`, `worker_c.txt`, `smoke_output.txt`, `domain_test.txt`, `routing_test.txt`) and `VERIFICATION_SUMMARY.txt` are outputs from prior smoke test runs. `verify_smoke_output.sh` is a shell script that validates these outputs.
