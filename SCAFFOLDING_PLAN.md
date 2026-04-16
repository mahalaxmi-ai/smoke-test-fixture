# Scaffolding Plan

## Current State

### Tech Stack

The repository uses **Rust** with a Cargo workspace configuration.

- **Root `Cargo.toml`**: Defines a workspace with one member (`fixture-crate`) using resolver version 2.
- **`fixture-crate/Cargo.toml`**: Declares a package named `fixture-crate` at version 0.1.0, using Rust edition 2021.
- **Source code**: `fixture-crate/src/main.rs` is the sole source file.

### Build System Validation

Both `Cargo.toml` files are syntactically valid. Running `cargo check` completes successfully with no errors or warnings.

### Scaffolding Checklist

| Category | Present | Details |
|---|---|---|
| Build system / package manager | Yes | Cargo workspace (`Cargo.toml` at root and `fixture-crate/Cargo.toml`) |
| Linter / formatter config | No | No `rustfmt.toml`, `clippy.toml`, or `.editorconfig` found |
| Test framework config | No | Cargo's built-in test runner is available but no test files or `#[cfg(test)]` modules were detected |
| CI/CD pipeline config | No | No `.github/workflows/`, `.gitlab-ci.yml`, or other CI configuration found |
| Environment variable templates | No | No `.env.example` or similar file found |
| `.gitignore` | Yes | Ignores `/target` and `Cargo.lock` |

## Recommended Next Steps

1. **Add a formatter configuration** — Create `rustfmt.toml` at the repository root to enforce consistent code style. A minimal starting point:
   - File: `rustfmt.toml`
   - Recommended tool version: rustfmt (ships with `rustup component add rustfmt`, stable channel)

2. **Add a linter configuration** — Create `clippy.toml` at the repository root (or configure via `Cargo.toml` `[lints]` table) to enable Clippy lints.
   - File: `clippy.toml`
   - Recommended tool version: clippy (ships with `rustup component add clippy`, stable channel)

3. **Add unit tests** — Create a `#[cfg(test)]` module in `fixture-crate/src/main.rs` or add a `fixture-crate/tests/` directory for integration tests. Cargo's built-in test runner (`cargo test`) requires no additional configuration.
   - Directory: `fixture-crate/tests/`

4. **Set up CI/CD** — Create a GitHub Actions workflow for automated build, lint, and test checks.
   - File: `.github/workflows/ci.yml`
   - Recommended steps: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
   - Runner: `ubuntu-latest` with `actions-rust-lang/setup-rust-toolchain@v1` (stable toolchain)

5. **Add an environment variable template** — If the application requires runtime configuration, create a `.env.example` documenting expected variables.
   - File: `.env.example`
   - Note: Only needed if the application uses environment-based configuration; currently no evidence of this in the codebase.

6. **Add an `.editorconfig`** — Standardize editor settings (indentation, line endings) across contributors.
   - File: `.editorconfig`
   - Recommended settings: `indent_style = space`, `indent_size = 4`, `end_of_line = lf`, `charset = utf-8`
