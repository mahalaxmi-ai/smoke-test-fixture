# smoke-fixture

A minimal Rust workspace used as the target project for [Mahalaxmi AI Terminal Orchestration](https://mahalaxmi.ai) smoke test scenarios.

## Project Structure

- **smoke-fixture** (root) — library crate with basic arithmetic utilities (`src/lib.rs`)
- **fixture-crate** — binary crate with arithmetic functions and comprehensive tests

## Build

```sh
cargo build
```

## Test

```sh
cargo test
```

## Branches

- `main` — this README and fixture content
- `smoke-base` — the clean baseline branch that smoke tests reset to before each run

## Usage

Smoke test scenarios clone or reset to `smoke-base`, run a Mahalaxmi orchestration cycle against this repo, then validate outputs.
