# Repository Manifest

Generated: 2026-04-16

## Project Overview

This repository is a **CI smoke-test fixture** for the Mahalaxmi AI Terminal Orchestration system. It contains a minimal Rust workspace used as a target project for smoke test scenarios.

## Tech Stack

| Attribute        | Value                     |
|------------------|---------------------------|
| Language         | Rust                      |
| Framework        | None (binary crate)       |
| Package Manager  | Cargo                     |
| Workspace        | Yes (`resolver = "2"`)    |
| Edition          | 2021                      |

## File Manifest

| File Path | Size (bytes) | Type |
|-----------|-------------|------|
| `Cargo.toml` | 55 | Rust workspace config |
| `fixture-crate/Cargo.toml` | 68 | Rust package config |
| `fixture-crate/src/main.rs` | 2686 | Rust source |
| `README.md` | 836 | Documentation |
| `PROJECT_STATUS.md` | 1477 | Documentation |
| `.gitignore` | 19 | Git config |
| `S1-001-000-ROADMAP.json` | 309 | Orchestration scenario |
| `S1-002-000-CIRCULAR.json` | 823 | Orchestration scenario |
| `S1-003-000-ROADMAP.json` | 533 | Orchestration scenario |
| `S1-003-001-PHASE1.json` | 421 | Orchestration scenario |
| `S1-003-002-PHASE2.json` | 429 | Orchestration scenario |
| `TEST-INVALID.json` | 93 | Orchestration scenario |
| `verify_smoke_output.sh` | 714 | Shell script |
| `smoke_output.txt` | 15 | Test output |
| `domain_test.txt` | 13 | Test output |
| `routing_test.txt` | 10 | Test output |
| `worker_a.txt` | 6 | Worker output (contains TEXT_A) |
| `worker_b.txt` | 6 | Worker output (contains TEXT_B) |
| `worker_c.txt` | 6 | Worker output (contains TEXT_C) |
| `worker_files_test_report.txt` | 774 | Test report |
| `VERIFICATION_SUMMARY.txt` | 636 | Test verification |

## Summary

| Metric | Value |
|--------|-------|
| Total files | 21 |
| Languages detected | Rust, Shell (Bash), JSON, Markdown |
| Dependency files | `Cargo.toml` (workspace root), `fixture-crate/Cargo.toml` |
| Test files/directories | `verify_smoke_output.sh`, `*_test.txt` outputs, `TEST-INVALID.json` |
| CI/CD configuration | None detected in repository |

## Modules and Entry Points

- **Workspace root**: `Cargo.toml` defines a Cargo workspace with one member (`fixture-crate`).
- **fixture-crate**: Binary crate at `fixture-crate/src/main.rs` (entry point: `fn main`).

## Configuration Files

- `Cargo.toml` — Workspace definition
- `fixture-crate/Cargo.toml` — Package metadata (name: `fixture-crate`, version: `0.1.0`)
- `.gitignore` — Excludes `/target` and `Cargo.lock`

## Specification and Requirements Documents

- `README.md` — Describes the repository as a CI fixture for Mahalaxmi smoke tests. States the repo should not be modified manually.
- `PROJECT_STATUS.md` — Documents project structure, build status, and test results.
- `S1-*.json` files — Orchestration scenario definitions for smoke test runs.

## Directory Traversal Errors

No errors encountered during directory scan. All paths were accessible.
