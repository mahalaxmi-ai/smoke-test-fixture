use std::process::Command;

fn cargo_bin_path() -> std::path::PathBuf {
    let mut path = std::env::current_exe()
        .expect("failed to get current exe path");
    // Move from deps/test_binary up to the target debug directory
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.join("fixture-crate")
}

#[test]
fn test_binary_runs_successfully() {
    let bin = cargo_bin_path();
    let output = Command::new(&bin)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", bin.display(), e));

    assert!(
        output.status.success(),
        "binary exited with non-zero status: {}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("smoke test fixture"),
        "unexpected output: {}",
        stdout
    );
}

#[test]
fn test_binary_output_is_deterministic() {
    let bin = cargo_bin_path();

    let output1 = Command::new(&bin)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", bin.display(), e));
    let output2 = Command::new(&bin)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", bin.display(), e));

    assert_eq!(
        output1.stdout, output2.stdout,
        "binary output should be deterministic"
    );
}

#[test]
fn test_binary_produces_no_stderr() {
    let bin = cargo_bin_path();
    let output = Command::new(&bin)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", bin.display(), e));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.is_empty(),
        "binary should not produce stderr output, got: {}",
        stderr
    );
}
