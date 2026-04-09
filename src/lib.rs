// Project: smoke-fixture workspace root library
// Edition: 2021, resolver v2
// This crate provides shared utilities for the workspace.

use std::fmt;

/// Represents the result of a health check on the project.
#[derive(Debug, PartialEq)]
pub enum HealthStatus {
    /// The project is operational.
    Ok,
    /// The project encountered an issue described by the contained message.
    Error(String),
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Ok => write!(f, "OK"),
            HealthStatus::Error(msg) => write!(f, "ERROR: {}", msg),
        }
    }
}

/// Performs a baseline health check, returning the project status.
///
/// # Errors
/// Returns `Err` with a description if the check fails.
pub fn health_check() -> Result<HealthStatus, String> {
    Ok(HealthStatus::Ok)
}

/// Validates that a project name is non-empty and contains only valid characters.
///
/// # Errors
/// Returns `Err` if the name is empty or contains invalid characters.
pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Project name must not be empty".to_string());
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(format!(
            "Project name '{}' contains invalid characters; only alphanumeric, '-', and '_' are allowed",
            name
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_returns_ok() {
        let result = health_check();
        assert!(result.is_ok());
        assert_eq!(result.expect("health check should succeed"), HealthStatus::Ok);
    }

    #[test]
    fn test_health_status_display_ok() {
        assert_eq!(format!("{}", HealthStatus::Ok), "OK");
    }

    #[test]
    fn test_health_status_display_error() {
        let status = HealthStatus::Error("something broke".to_string());
        assert_eq!(format!("{}", status), "ERROR: something broke");
    }

    #[test]
    fn test_validate_project_name_valid() {
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("my_project").is_ok());
        assert!(validate_project_name("project123").is_ok());
    }

    #[test]
    fn test_validate_project_name_empty() {
        let result = validate_project_name("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Project name must not be empty");
    }

    #[test]
    fn test_validate_project_name_invalid_chars() {
        let result = validate_project_name("my project");
        assert!(result.is_err());
    }
}
