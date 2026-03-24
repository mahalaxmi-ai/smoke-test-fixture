pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

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

fn main() {
    println!("smoke test fixture");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-4, -6), -10);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-3, 5), 2);
    }

    #[test]
    fn test_multiply_positive_numbers() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(5, 7), 35);
    }

    #[test]
    fn test_multiply_negative_numbers() {
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(3, -4), -12);
        assert_eq!(multiply(-3, -4), 12);
    }

    #[test]
    fn test_multiply_with_zero() {
        assert_eq!(multiply(0, 99), 0);
        assert_eq!(multiply(42, 0), 0);
        assert_eq!(multiply(0, 0), 0);
    }

    #[test]
    fn test_multiply_edge_cases() {
        assert_eq!(multiply(1, 42), 42);
        assert_eq!(multiply(100, 1), 100);
        assert_eq!(multiply(-1, 42), -42);
        assert_eq!(multiply(1000, 1000), 1_000_000);
        assert_eq!(multiply(-100, -100), 10_000);
    }
}
