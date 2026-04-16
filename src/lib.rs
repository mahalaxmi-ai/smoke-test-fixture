use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::Path;

/// Errors that can occur during manifest validation.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    /// The manifest file could not be read.
    IoError(String),
    /// The manifest file contains invalid JSON.
    ParseError(String),
    /// A required field is missing or empty.
    MissingField(String),
    /// The version string does not follow semver format.
    InvalidVersion(String),
    /// A dependency references an item ID that does not exist in the manifest.
    UnknownDependency { from: String, to: String },
    /// A circular dependency cycle was detected.
    CircularDependency(Vec<String>),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::IoError(msg) => write!(f, "IO error: {msg}"),
            ValidationError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ValidationError::MissingField(field) => write!(f, "Missing required field: {field}"),
            ValidationError::InvalidVersion(v) => write!(f, "Invalid version format: {v}"),
            ValidationError::UnknownDependency { from, to } => {
                write!(f, "Unknown dependency: {from} -> {to}")
            }
            ValidationError::CircularDependency(cycle) => {
                write!(f, "Circular dependency detected: {}", cycle.join(" -> "))
            }
        }
    }
}

/// A single item within a requirement manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestItem {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub domain_id: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
}

/// A dependency edge between two manifest items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub from: String,
    pub to: String,
}

/// A requirement manifest containing items and their dependency relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest_id: String,
    pub sprint_id: String,
    pub title: String,
    pub version: String,
    pub items: Vec<ManifestItem>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

/// Parses a manifest from a JSON string.
///
/// Returns the parsed `Manifest` or a `ValidationError` if the JSON is malformed
/// or missing required fields.
pub fn parse_manifest(json: &str) -> Result<Manifest, ValidationError> {
    let manifest: Manifest =
        serde_json::from_str(json).map_err(|e| ValidationError::ParseError(e.to_string()))?;

    if manifest.manifest_id.is_empty() {
        return Err(ValidationError::MissingField("manifest_id".to_string()));
    }
    if manifest.sprint_id.is_empty() {
        return Err(ValidationError::MissingField("sprint_id".to_string()));
    }
    if manifest.title.is_empty() {
        return Err(ValidationError::MissingField("title".to_string()));
    }
    if manifest.version.is_empty() {
        return Err(ValidationError::MissingField("version".to_string()));
    }
    if manifest.items.is_empty() {
        return Err(ValidationError::MissingField("items".to_string()));
    }

    Ok(manifest)
}

/// Validates that a version string follows basic semver format (MAJOR.MINOR.PATCH).
///
/// Each component must be a non-negative integer.
pub fn validate_version(version: &str) -> Result<(), ValidationError> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return Err(ValidationError::InvalidVersion(version.to_string()));
    }
    for part in &parts {
        if part.parse::<u64>().is_err() {
            return Err(ValidationError::InvalidVersion(version.to_string()));
        }
    }
    Ok(())
}

/// Detects circular dependencies in a manifest's dependency graph.
///
/// Uses depth-first search to find cycles. Returns `Ok(())` if no cycles exist,
/// or `Err(ValidationError::CircularDependency(...))` with the cycle path.
pub fn detect_circular_dependencies(manifest: &Manifest) -> Result<(), ValidationError> {
    let item_ids: HashSet<&str> = manifest.items.iter().map(|i| i.id.as_str()).collect();

    for dep in &manifest.dependencies {
        if !item_ids.contains(dep.from.as_str()) || !item_ids.contains(dep.to.as_str()) {
            return Err(ValidationError::UnknownDependency {
                from: dep.from.clone(),
                to: dep.to.clone(),
            });
        }
    }

    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    for dep in &manifest.dependencies {
        adjacency
            .entry(dep.from.as_str())
            .or_default()
            .push(dep.to.as_str());
    }

    let mut visited: HashSet<&str> = HashSet::new();
    let mut rec_stack: HashSet<&str> = HashSet::new();
    let mut path: Vec<&str> = Vec::new();

    for item_id in &item_ids {
        if !visited.contains(item_id) {
            if let Some(cycle) =
                dfs_find_cycle(item_id, &adjacency, &mut visited, &mut rec_stack, &mut path)
            {
                return Err(ValidationError::CircularDependency(cycle));
            }
        }
    }

    Ok(())
}

fn dfs_find_cycle<'a>(
    node: &'a str,
    adjacency: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    rec_stack: &mut HashSet<&'a str>,
    path: &mut Vec<&'a str>,
) -> Option<Vec<String>> {
    visited.insert(node);
    rec_stack.insert(node);
    path.push(node);

    if let Some(neighbors) = adjacency.get(node) {
        for &neighbor in neighbors {
            if !visited.contains(neighbor) {
                if let Some(cycle) = dfs_find_cycle(neighbor, adjacency, visited, rec_stack, path) {
                    return Some(cycle);
                }
            } else if rec_stack.contains(neighbor) {
                let cycle_start = path.iter().position(|&n| n == neighbor).unwrap_or(0);
                let mut cycle: Vec<String> =
                    path[cycle_start..].iter().map(|s| s.to_string()).collect();
                cycle.push(neighbor.to_string());
                return Some(cycle);
            }
        }
    }

    path.pop();
    rec_stack.remove(node);
    None
}

/// Performs full validation of a manifest: parsing, version check, and cycle detection.
///
/// Returns the parsed `Manifest` on success, or the first `ValidationError` encountered.
pub fn validate_manifest(json: &str) -> Result<Manifest, ValidationError> {
    let manifest = parse_manifest(json)?;
    validate_version(&manifest.version)?;
    detect_circular_dependencies(&manifest)?;
    Ok(manifest)
}

/// Loads and validates a manifest from a file path.
///
/// Reads the file contents and delegates to `validate_manifest`.
pub fn validate_manifest_file(path: &Path) -> Result<Manifest, ValidationError> {
    let content =
        std::fs::read_to_string(path).map_err(|e| ValidationError::IoError(e.to_string()))?;
    validate_manifest(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_manifest_json() -> String {
        r#"{
            "manifest_id": "S1-001-000",
            "sprint_id": "S1-001",
            "title": "Test Manifest",
            "version": "1.0.0",
            "items": [
                {"id": "S1-001-001", "title": "Item 1"},
                {"id": "S1-001-002", "title": "Item 2"}
            ],
            "dependencies": [
                {"from": "S1-001-001", "to": "S1-001-002"}
            ]
        }"#
        .to_string()
    }

    fn circular_manifest_json() -> String {
        r#"{
            "manifest_id": "S1-002-000-CIRCULAR",
            "sprint_id": "S1-002",
            "title": "Circular Test",
            "version": "1.0.0",
            "items": [
                {"id": "S1-002-001", "title": "Item 1"},
                {"id": "S1-002-002", "title": "Item 2"},
                {"id": "S1-002-003", "title": "Item 3"}
            ],
            "dependencies": [
                {"from": "S1-002-001", "to": "S1-002-002"},
                {"from": "S1-002-002", "to": "S1-002-003"},
                {"from": "S1-002-003", "to": "S1-002-001"}
            ]
        }"#
        .to_string()
    }

    #[test]
    fn test_parse_manifest_success() {
        let result = parse_manifest(&valid_manifest_json());
        assert!(result.is_ok());
        let manifest = result.expect("should parse");
        assert_eq!(manifest.manifest_id, "S1-001-000");
        assert_eq!(manifest.items.len(), 2);
    }

    #[test]
    fn test_parse_manifest_invalid_json() {
        let result = parse_manifest("not valid json");
        assert!(result.is_err());
        match result {
            Err(ValidationError::ParseError(_)) => {}
            other => panic!("Expected ParseError, got {:?}", other),
        }
    }

    #[test]
    fn test_parse_manifest_missing_manifest_id() {
        let json = r#"{
            "manifest_id": "",
            "sprint_id": "S1",
            "title": "T",
            "version": "1.0.0",
            "items": [{"id": "a", "title": "b"}]
        }"#;
        let result = parse_manifest(json);
        assert_eq!(
            result.err(),
            Some(ValidationError::MissingField("manifest_id".to_string()))
        );
    }

    #[test]
    fn test_parse_manifest_missing_items() {
        let json = r#"{
            "manifest_id": "X",
            "sprint_id": "S1",
            "title": "T",
            "version": "1.0.0",
            "items": []
        }"#;
        let result = parse_manifest(json);
        assert_eq!(
            result.err(),
            Some(ValidationError::MissingField("items".to_string()))
        );
    }

    #[test]
    fn test_validate_version_valid() {
        assert!(validate_version("1.0.0").is_ok());
        assert!(validate_version("0.0.1").is_ok());
        assert!(validate_version("10.20.30").is_ok());
    }

    #[test]
    fn test_validate_version_invalid() {
        assert!(validate_version("1.0").is_err());
        assert!(validate_version("abc").is_err());
        assert!(validate_version("1.0.0.0").is_err());
        assert!(validate_version("1.a.0").is_err());
        assert!(validate_version("").is_err());
    }

    #[test]
    fn test_detect_no_circular_dependencies() {
        let manifest = parse_manifest(&valid_manifest_json()).expect("should parse");
        assert!(detect_circular_dependencies(&manifest).is_ok());
    }

    #[test]
    fn test_detect_circular_dependencies() {
        let manifest = parse_manifest(&circular_manifest_json()).expect("should parse");
        let result = detect_circular_dependencies(&manifest);
        assert!(result.is_err());
        match result {
            Err(ValidationError::CircularDependency(cycle)) => {
                assert!(cycle.len() >= 3, "Cycle should contain at least 3 nodes");
                assert_eq!(
                    cycle.first(),
                    cycle.last(),
                    "Cycle should start and end with the same node"
                );
            }
            other => panic!("Expected CircularDependency, got {:?}", other),
        }
    }

    #[test]
    fn test_detect_unknown_dependency() {
        let json = r#"{
            "manifest_id": "X",
            "sprint_id": "S1",
            "title": "T",
            "version": "1.0.0",
            "items": [{"id": "A", "title": "Item A"}],
            "dependencies": [{"from": "A", "to": "NONEXISTENT"}]
        }"#;
        let manifest = parse_manifest(json).expect("should parse");
        let result = detect_circular_dependencies(&manifest);
        match result {
            Err(ValidationError::UnknownDependency { from, to }) => {
                assert_eq!(from, "A");
                assert_eq!(to, "NONEXISTENT");
            }
            other => panic!("Expected UnknownDependency, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_manifest_success() {
        let result = validate_manifest(&valid_manifest_json());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_manifest_circular_fails() {
        let result = validate_manifest(&circular_manifest_json());
        assert!(result.is_err());
        match result {
            Err(ValidationError::CircularDependency(_)) => {}
            other => panic!("Expected CircularDependency, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_manifest_bad_version() {
        let json = r#"{
            "manifest_id": "X",
            "sprint_id": "S1",
            "title": "T",
            "version": "bad",
            "items": [{"id": "A", "title": "Item A"}]
        }"#;
        let result = validate_manifest(json);
        match result {
            Err(ValidationError::InvalidVersion(v)) => assert_eq!(v, "bad"),
            other => panic!("Expected InvalidVersion, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_manifest_file_nonexistent() {
        let result = validate_manifest_file(Path::new("/nonexistent/file.json"));
        match result {
            Err(ValidationError::IoError(_)) => {}
            other => panic!("Expected IoError, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_manifest_no_dependencies() {
        let json = r#"{
            "manifest_id": "X",
            "sprint_id": "S1",
            "title": "T",
            "version": "1.0.0",
            "items": [{"id": "A", "title": "Item A"}]
        }"#;
        let result = validate_manifest(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::MissingField("test".to_string());
        assert_eq!(format!("{err}"), "Missing required field: test");

        let err = ValidationError::CircularDependency(vec![
            "A".to_string(),
            "B".to_string(),
            "A".to_string(),
        ]);
        assert_eq!(
            format!("{err}"),
            "Circular dependency detected: A -> B -> A"
        );

        let err = ValidationError::IoError("not found".to_string());
        assert_eq!(format!("{err}"), "IO error: not found");

        let err = ValidationError::ParseError("bad json".to_string());
        assert_eq!(format!("{err}"), "Parse error: bad json");

        let err = ValidationError::InvalidVersion("x".to_string());
        assert_eq!(format!("{err}"), "Invalid version format: x");

        let err = ValidationError::UnknownDependency {
            from: "A".to_string(),
            to: "B".to_string(),
        };
        assert_eq!(format!("{err}"), "Unknown dependency: A -> B");
    }

    #[test]
    fn test_self_referencing_dependency() {
        let json = r#"{
            "manifest_id": "X",
            "sprint_id": "S1",
            "title": "T",
            "version": "1.0.0",
            "items": [{"id": "A", "title": "Item A"}],
            "dependencies": [{"from": "A", "to": "A"}]
        }"#;
        let result = validate_manifest(json);
        assert!(result.is_err());
        match result {
            Err(ValidationError::CircularDependency(cycle)) => {
                assert_eq!(cycle.first(), cycle.last());
            }
            other => panic!("Expected CircularDependency, got {:?}", other),
        }
    }
}
