use std::process::Command;

#[test]
fn test_add_basic_operations() {
    assert_eq!(fixture_crate::add(1, 2), 3);
    assert_eq!(fixture_crate::add(-1, 1), 0);
    assert_eq!(fixture_crate::add(0, 0), 0);
}

#[test]
fn test_multiply_basic_operations() {
    assert_eq!(fixture_crate::multiply(2, 3), 6);
    assert_eq!(fixture_crate::multiply(-2, 3), -6);
    assert_eq!(fixture_crate::multiply(0, 100), 0);
}

#[test]
fn test_binary_runs_without_panic() {
    let output = Command::new(env!("CARGO_BIN_EXE_fixture-crate"))
        .output()
        .expect("failed to execute fixture-crate binary");

    assert!(
        output.status.success(),
        "binary exited with non-zero status: {}",
        output.status
    );
}
