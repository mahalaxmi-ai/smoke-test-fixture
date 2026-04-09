// Project structure verification summary:
// - Workspace root: Cargo.toml (workspace with member "fixture-crate", resolver v2)
// - fixture-crate: Rust package v0.1.0 (edition 2021) with add/multiply functions and tests
// - smoke_output.txt: Contains SMOKE_TEST_PASS verification line
// - Project compiles successfully with no external dependencies

use std::process;

fn run() -> Result<(), std::io::Error> {
    let result = std::fs::read_to_string("smoke_output.txt")?;
    if result.trim() == "SMOKE_TEST_PASS" {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "smoke_output.txt does not contain expected content",
        ))
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
