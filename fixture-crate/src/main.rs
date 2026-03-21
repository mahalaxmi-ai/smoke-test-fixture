fn main() {
    println!("smoke test fixture");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
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
    fn test_multiply_positive() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_multiply_negative() {
        assert_eq!(multiply(-2, 5), -10);
    }

    #[test]
    fn test_multiply_zero() {
        assert_eq!(multiply(0, 7), 0);
    }

    #[test]
    fn test_multiply_both_negative() {
        assert_eq!(multiply(-3, -3), 9);
    }
}
