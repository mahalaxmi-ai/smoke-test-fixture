use std::process;

fn run() -> Result<(), String> {
    let greeting = smoke_fixture::greet("world")?;
    std::io::Write::write_all(&mut std::io::stdout(), greeting.as_bytes())
        .map_err(|e| format!("failed to write to stdout: {e}"))?;
    std::io::Write::write_all(&mut std::io::stdout(), b"\n")
        .map_err(|e| format!("failed to write to stdout: {e}"))?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
