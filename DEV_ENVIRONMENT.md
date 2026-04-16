# Development Environment

## Prerequisites

- **Git** >= 2.30
- **Node.js** >= 18 LTS (if applicable to the project)
- **Python** >= 3.10 (if applicable to the project)
- **Rust** >= 1.70 (if applicable to the project)
- An editor or IDE that supports [EditorConfig](https://editorconfig.org)

Verify installed versions before proceeding:

```bash
git --version
node --version   # if using Node.js
python3 --version  # if using Python
rustc --version    # if using Rust
```

## Setup Instructions

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Install project dependencies (adjust for your language/runtime):

   ```bash
   # Node.js projects
   npm install

   # Python projects
   pip install -r requirements.txt

   # Rust projects
   cargo build
   ```

3. Confirm your editor picks up the `.editorconfig` settings (UTF-8, LF line endings, 4-space indentation, trailing whitespace trimming, final newline).

4. Run the test suite to verify everything is working:

   ```bash
   # Node.js
   npm test

   # Python
   pytest

   # Rust
   cargo test
   ```

## Quality Gates

All code contributed to this repository must satisfy the following constraints. Pull requests that violate any gate will be rejected.

### Explicit Error Handling

- Every function must handle all error paths explicitly.
- **No `unwrap()` on fallible operations** in Rust code. Use `?`, `match`, `unwrap_or_else`, or other explicit handling instead.
- **No bare `try/catch` with empty handlers** in JavaScript/TypeScript/Python. Every `catch`/`except` block must either handle the error meaningfully or re-throw it.
- **No untyped or generic exception swallowing.** Catch specific error types and take appropriate action.

### No Incomplete-Work Markers

- Code must not contain `TODO`, `FIXME`, or `HACK` comments. All work must be complete before merging.

### No Hardcoded Secrets

- No credentials, API keys, tokens, or secrets may appear in source files.
- Use environment variables or a secrets manager for sensitive configuration.
- Files such as `.env`, `credentials.json`, and similar must be listed in `.gitignore` and never committed.

### Coding Style

- The repository uses an `.editorconfig` file to enforce consistent formatting:
  - **Charset:** UTF-8
  - **Line endings:** LF
  - **Indentation:** 4 spaces (tabs for Makefiles)
  - **Trailing whitespace:** trimmed (except in Markdown)
  - **Final newline:** always present

### No Debug Output in Production Code

- Remove all debug logging (`println!`, `console.log`, `print()`, `Debug.Log`, etc.) before merging. Use a proper logging framework with appropriate log levels instead.
