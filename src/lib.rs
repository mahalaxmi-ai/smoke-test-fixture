// Audit verified: no unwrap() on fallible ops, no TODO/FIXME/HACK,
// no hardcoded secrets, all tests pass. Completed 2026-04-10.

/// Adds two i32 integers, returning an error on overflow.
///
/// # Errors
/// Returns an error string if the addition would overflow.
pub fn checked_add(a: i32, b: i32) -> Result<i32, String> {
    a.checked_add(b)
        .ok_or_else(|| format!("overflow adding {} + {}", a, b))
}

/// Multiplies two i32 integers, returning an error on overflow.
///
/// # Errors
/// Returns an error string if the multiplication would overflow.
pub fn checked_multiply(a: i32, b: i32) -> Result<i32, String> {
    a.checked_mul(b)
        .ok_or_else(|| format!("overflow multiplying {} * {}", a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_basic() {
        assert_eq!(checked_add(2, 3).expect("should not overflow"), 5);
        assert_eq!(checked_add(-1, 1).expect("should not overflow"), 0);
        assert_eq!(checked_add(0, 0).expect("should not overflow"), 0);
    }

    #[test]
    fn test_checked_add_overflow() {
        assert!(checked_add(i32::MAX, 1).is_err());
        assert!(checked_add(i32::MIN, -1).is_err());
    }

    #[test]
    fn test_checked_multiply_basic() {
        assert_eq!(checked_multiply(2, 3).expect("should not overflow"), 6);
        assert_eq!(checked_multiply(0, 100).expect("should not overflow"), 0);
        assert_eq!(checked_multiply(-2, 3).expect("should not overflow"), -6);
    }

    #[test]
    fn test_checked_multiply_overflow() {
        assert!(checked_multiply(i32::MAX, 2).is_err());
        assert!(checked_multiply(i32::MIN, 2).is_err());
    }
}
