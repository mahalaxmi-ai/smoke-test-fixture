use crate::error::ConfigError;
use crate::format::{self, Format};

/// Merge two configuration strings using overlay semantics.
///
/// Values in `overlay` override values in `base`. Maps are merged recursively;
/// scalars and arrays are replaced wholesale.
pub fn merge_configs(
    base: &str,
    overlay: &str,
    format: Format,
) -> Result<String, ConfigError> {
    let base_val = format::parse(base, format)?;
    let overlay_val = format::parse(overlay, format)?;
    let merged = merge_values(base_val, overlay_val);
    format::serialize(&merged, format)
}

/// Recursively merge two JSON values.
///
/// - If both are objects: merge keys recursively.
/// - Otherwise: `overlay` replaces `base`.
fn merge_values(base: serde_json::Value, overlay: serde_json::Value) -> serde_json::Value {
    match (base, overlay) {
        (serde_json::Value::Object(mut base_map), serde_json::Value::Object(overlay_map)) => {
            for (key, overlay_val) in overlay_map {
                let merged_val = match base_map.remove(&key) {
                    Some(base_val) => merge_values(base_val, overlay_val),
                    None => overlay_val,
                };
                base_map.insert(key, merged_val);
            }
            serde_json::Value::Object(base_map)
        }
        (_base, overlay) => overlay,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_scalar_override() {
        let base = r#"{"port": 8080}"#;
        let overlay = r#"{"port": 9090}"#;
        let result = merge_configs(base, overlay, Format::Json).unwrap();
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["port"], 9090);
    }

    #[test]
    fn merge_nested_map() {
        let base = r#"{"database": {"host": "localhost", "port": 5432}}"#;
        let overlay = r#"{"database": {"host": "prod.internal"}}"#;
        let result = merge_configs(base, overlay, Format::Json).unwrap();
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["database"]["host"], "prod.internal");
        assert_eq!(v["database"]["port"], 5432);
    }
}
