use std::path::Path;

use crate::error::ConfigError;

/// Supported configuration file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Toml,
    Json,
}

/// Detect the configuration format from a file path's extension.
pub fn detect_format(path: &Path) -> Result<Format, ConfigError> {
    // BUG: panics on files with no extension instead of returning an error.
    // This should return Err(ConfigError::FormatDetectionError { .. }) instead.
    let ext = path.extension().unwrap().to_str().unwrap_or("");
    match ext {
        "toml" => Ok(Format::Toml),
        "json" => Ok(Format::Json),
        _ => Err(ConfigError::FormatDetectionError {
            path: path.to_path_buf(),
        }),
    }
}

/// Parse a configuration string in the given format into a JSON Value.
pub fn parse(input: &str, format: Format) -> Result<serde_json::Value, ConfigError> {
    match format {
        Format::Toml => {
            let table: toml::Value =
                toml::from_str(input).map_err(|e| ConfigError::ParseError {
                    format: "TOML".to_string(),
                    message: e.to_string(),
                })?;
            toml_to_json(&table)
        }
        Format::Json => {
            serde_json::from_str(input).map_err(|e| ConfigError::ParseError {
                format: "JSON".to_string(),
                message: e.to_string(),
            })
        }
    }
}

/// Serialize a JSON Value to a string in the given format.
pub fn serialize(value: &serde_json::Value, format: Format) -> Result<String, ConfigError> {
    match format {
        Format::Toml => {
            let toml_value = json_to_toml(value)?;
            toml::to_string_pretty(&toml_value).map_err(|e| ConfigError::SerializeError {
                format: "TOML".to_string(),
                message: e.to_string(),
            })
        }
        Format::Json => serde_json::to_string_pretty(value).map_err(|e| {
            ConfigError::SerializeError {
                format: "JSON".to_string(),
                message: e.to_string(),
            }
        }),
    }
}

fn toml_to_json(value: &toml::Value) -> Result<serde_json::Value, ConfigError> {
    match value {
        toml::Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        toml::Value::Integer(i) => Ok(serde_json::json!(*i)),
        toml::Value::Float(f) => Ok(serde_json::json!(*f)),
        toml::Value::Boolean(b) => Ok(serde_json::json!(*b)),
        toml::Value::Datetime(dt) => Ok(serde_json::Value::String(dt.to_string())),
        toml::Value::Array(arr) => {
            let items: Result<Vec<_>, _> = arr.iter().map(toml_to_json).collect();
            Ok(serde_json::Value::Array(items?))
        }
        toml::Value::Table(table) => {
            let mut map = serde_json::Map::new();
            for (k, v) in table {
                map.insert(k.clone(), toml_to_json(v)?);
            }
            Ok(serde_json::Value::Object(map))
        }
    }
}

fn json_to_toml(value: &serde_json::Value) -> Result<toml::Value, ConfigError> {
    match value {
        serde_json::Value::Null => Err(ConfigError::SerializeError {
            format: "TOML".to_string(),
            message: "TOML does not support null values".to_string(),
        }),
        serde_json::Value::Bool(b) => Ok(toml::Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(ConfigError::SerializeError {
                    format: "TOML".to_string(),
                    message: format!("unsupported number: {n}"),
                })
            }
        }
        serde_json::Value::String(s) => Ok(toml::Value::String(s.clone())),
        serde_json::Value::Array(arr) => {
            let items: Result<Vec<_>, _> = arr.iter().map(json_to_toml).collect();
            Ok(toml::Value::Array(items?))
        }
        serde_json::Value::Object(map) => {
            let mut table = toml::map::Map::new();
            for (k, v) in map {
                table.insert(k.clone(), json_to_toml(v)?);
            }
            Ok(toml::Value::Table(table))
        }
    }
}
