# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Sprint Manifest System

The project includes a two-phase sprint manifest system (S1-003):

- **S1-003-000-ROADMAP.json** — Roadmap manifest with 2 items and 1 dependency (v1.0.0)
- **S1-003-001-PHASE1.json** — Phase 1: Foundation Setup (infrastructure domain)
- **S1-003-002-PHASE2.json** — Phase 2: Feature Implementation (features domain, depends on Phase 1)

Phase 2 depends on Phase 1 completion before it can proceed.

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
