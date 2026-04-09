//! Smoke test fixture library crate.
//!
//! Provides basic arithmetic utilities used as a target for
//! Mahalaxmi orchestration smoke tests.

/// Adds two integers and returns their sum.
///
/// # Examples
///
/// ```
/// assert_eq!(smoke_fixture::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers and returns their product.
///
/// # Examples
///
/// ```
/// assert_eq!(smoke_fixture::multiply(4, 5), 20);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(0, 99), 0);
    }
}
