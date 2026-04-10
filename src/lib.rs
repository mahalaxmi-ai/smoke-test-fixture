/// Returns a greeting message for the given name.
///
/// # Errors
///
/// Returns an error if the provided name is empty.
pub fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("name must not be empty".to_string());
    }
    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_message() {
        let result = greet("world");
        assert_eq!(result, Ok("Hello, world!".to_string()));
    }

    #[test]
    fn greet_rejects_empty_name() {
        let result = greet("");
        assert!(result.is_err());
    }
}
