# configstitch

Merge layered configuration files into a single unified config.

## Overview

configstitch reads multiple configuration files (TOML, JSON) and merges them
using overlay semantics: values in later files override values in earlier files.
Maps are merged recursively; scalars and arrays are replaced wholesale.

This is useful for managing configuration that varies by environment:

    base.toml + production.toml → merged output

## Installation

    cargo install --path .

## Usage

    # Merge two TOML files, output JSON
    configstitch merge base.toml production.toml --format json

    # Merge with explicit output file
    configstitch merge base.toml staging.toml -o merged.toml

    # Validate merged config against a JSON schema
    configstitch validate merged.json --schema schema.json

    # Show diff between two configs
    configstitch diff base.toml production.toml

## Supported Formats

| Format | Read | Write | Status     |
|--------|------|-------|------------|
| TOML   | Yes  | Yes   | Stable     |
| JSON   | Yes  | Yes   | Stable     |
| YAML   | No   | No    | Planned    |

## Library Usage

configstitch can also be used as a library:

```rust
use configstitch::{merge_configs, Format};

let base = std::fs::read_to_string("base.toml").unwrap();
let overlay = std::fs::read_to_string("prod.toml").unwrap();

let merged = merge_configs(&base, &overlay, Format::Toml).unwrap();
println!("{}", merged);
```

## Architecture

- `src/main.rs` — CLI entry point using clap
- `src/lib.rs` — Public API re-exports
- `src/merge.rs` — Core merge logic (recursive map merge)
- `src/format.rs` — Format detection and conversion (TOML <-> JSON)
- `src/error.rs` — Error types
- `src/diff.rs` — Config diff display

## Contributing

1. Fork and clone
2. `cargo test` to verify the test suite passes
3. Make changes, add tests
4. `cargo clippy` and `cargo fmt` before submitting

## License

MIT
