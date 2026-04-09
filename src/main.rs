// Binary entry point for the smoke-fixture workspace root package.
// This serves as the baseline executable verifying project setup.

use smoke_fixture::health_check;

fn main() {
    match health_check() {
        Ok(status) => {
            let msg = format!("smoke-fixture baseline: {}", status);
            assert!(!msg.is_empty(), "status message should not be empty");
        }
        Err(e) => {
            eprintln!("Health check failed: {}", e);
            std::process::exit(1);
        }
    }
}
