# smoke-test-fixture

This repository is a **CI fixture** for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai).

It exists solely to be used as the **target project** for Mahalaxmi smoke test scenarios. It contains a minimal Rust workspace so that orchestration workers have a real codebase to operate on.

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs. After each run, `scripts/reset-fixture.sh` in the main repo resets this fixture back to `smoke-base`.

## Verification Status

Audit performed: all 10 unit tests pass, no TODO/FIXME/HACK markers present, no hardcoded secrets, all error paths handled, S1-001-000-ROADMAP.json manifest validated against schema requirements.

## Do Not Modify Manually

This repo is managed by CI automation. Manual commits may interfere with smoke test reproducibility.
