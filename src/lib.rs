//! Smoke test fixture workspace library.
//!
//! Provides utility functions for the Mahalaxmi CI smoke test fixture.

/// Multiplies two i32 integers and returns their product.
///
/// # Arguments
/// * `a` - The first integer operand
/// * `b` - The second integer operand
///
/// # Returns
/// The product of `a` and `b`
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_positive() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(5, 7), 35);
    }

    #[test]
    fn test_multiply_negative() {
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(-3, -4), 12);
    }

    #[test]
    fn test_multiply_zero() {
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(42, 0), 0);
        assert_eq!(multiply(0, 0), 0);
    }

    #[test]
    fn test_multiply_identity() {
        assert_eq!(multiply(1, 42), 42);
        assert_eq!(multiply(i32::MAX, 1), i32::MAX);
    }
}
