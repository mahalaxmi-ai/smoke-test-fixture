/// Returns the project version string.
pub fn version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_returns_valid_semver() {
        let v = version();
        let parts: Vec<&str> = v.split('.').collect();
        assert_eq!(parts.len(), 3, "version must have three dot-separated components");
        for part in &parts {
            part.parse::<u32>().expect("each version component must be a valid integer");
        }
    }
}
