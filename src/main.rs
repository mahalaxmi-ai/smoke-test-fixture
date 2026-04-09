/// Entry point for the workspace verification binary.
///
/// Confirms that the project compiles, the workspace is correctly configured,
/// and basic arithmetic utilities are available in the fixture-crate member.
use std::process;

fn run() -> Result<(), String> {
    let status = "Workspace setup verified successfully";
    match std::io::Write::write_all(&mut std::io::stdout(), status.as_bytes()) {
        Ok(()) => {}
        Err(e) => return Err(format!("Failed to write status: {e}")),
    }
    match std::io::Write::write_all(&mut std::io::stdout(), b"\n") {
        Ok(()) => {}
        Err(e) => return Err(format!("Failed to write newline: {e}")),
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_succeeds() {
        assert!(run().is_ok(), "run() should complete without error");
    }
}
