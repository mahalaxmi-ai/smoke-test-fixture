# manifest-validator

A Rust tool and library for validating requirement manifest JSON files. Detects missing fields, invalid version strings, unknown dependency references, and circular dependency cycles.

## Building

```sh
cargo build
```

## Testing

```sh
cargo test
```

## Running

Pass one or more manifest JSON files as arguments:

```sh
cargo run -- path/to/manifest.json
```

The tool exits with code 0 if all manifests are valid, or code 1 if any fail validation.

### Example

```sh
# Validates a manifest with circular dependencies (expected to fail)
cargo run -- S1-002-000-CIRCULAR.json
```

## Manifest Format

```json
{
  "manifest_id": "S1-001-000",
  "sprint_id": "S1-001",
  "title": "Sprint Title",
  "version": "1.0.0",
  "items": [
    {"id": "S1-001-001", "title": "First item"},
    {"id": "S1-001-002", "title": "Second item"}
  ],
  "dependencies": [
    {"from": "S1-001-001", "to": "S1-001-002"}
  ]
}
```

## Validation Rules

- All required fields (`manifest_id`, `sprint_id`, `title`, `version`, `items`) must be present and non-empty.
- Version must follow semver format (MAJOR.MINOR.PATCH).
- All dependency `from`/`to` references must correspond to existing item IDs.
- The dependency graph must be acyclic (no circular dependencies).

## Workspace

This repository is a Cargo workspace that also includes `fixture-crate`, a minimal smoke-test fixture.
