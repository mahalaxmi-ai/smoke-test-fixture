use crate::error::ConfigError;
use crate::format::{self, Format};

/// Compare two configuration strings and return a text diff.
///
/// Both inputs are parsed, normalized to sorted JSON, then compared line by line.
/// Returns an empty string if the configs are semantically identical.
pub fn diff_configs(a: &str, b: &str, format: Format) -> Result<String, ConfigError> {
    let val_a = format::parse(a, format)?;
    let val_b = format::parse(b, format)?;

    let text_a = serde_json::to_string_pretty(&val_a).map_err(|e| ConfigError::SerializeError {
        format: "JSON".to_string(),
        message: e.to_string(),
    })?;
    let text_b = serde_json::to_string_pretty(&val_b).map_err(|e| ConfigError::SerializeError {
        format: "JSON".to_string(),
        message: e.to_string(),
    })?;

    if text_a == text_b {
        return Ok(String::new());
    }

    let mut output = String::new();
    let lines_a: Vec<&str> = text_a.lines().collect();
    let lines_b: Vec<&str> = text_b.lines().collect();

    for (i, (la, lb)) in lines_a.iter().zip(lines_b.iter()).enumerate() {
        if la != lb {
            output.push_str(&format!("line {}: - {}\n", i + 1, la));
            output.push_str(&format!("line {}: + {}\n", i + 1, lb));
        }
    }

    // Handle length differences
    if lines_a.len() > lines_b.len() {
        for (i, line) in lines_a[lines_b.len()..].iter().enumerate() {
            output.push_str(&format!("line {}: - {}\n", lines_b.len() + i + 1, line));
        }
    } else if lines_b.len() > lines_a.len() {
        for (i, line) in lines_b[lines_a.len()..].iter().enumerate() {
            output.push_str(&format!("line {}: + {}\n", lines_a.len() + i + 1, line));
        }
    }

    Ok(output)
}
