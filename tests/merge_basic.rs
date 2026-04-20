use configstitch::{merge_configs, Format};

#[test]
fn scalar_override() {
    let base = r#"{"port": 8080}"#;
    let overlay = r#"{"port": 9090}"#;
    let result = merge_configs(base, overlay, Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["port"], 9090);
}

#[test]
fn nested_map_merge() {
    let base = r#"{"database": {"host": "localhost", "port": 5432}}"#;
    let overlay = r#"{"database": {"host": "prod.internal"}}"#;
    let result = merge_configs(base, overlay, Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["database"]["host"], "prod.internal");
    assert_eq!(v["database"]["port"], 5432);
}

#[test]
fn array_replacement() {
    let base = r#"{"tags": ["a", "b"]}"#;
    let overlay = r#"{"tags": ["c"]}"#;
    let result = merge_configs(base, overlay, Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["tags"], serde_json::json!(["c"]));
}

#[test]
fn overlay_adds_new_key() {
    let base = r#"{"a": 1}"#;
    let overlay = r#"{"b": 2}"#;
    let result = merge_configs(base, overlay, Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["a"], 1);
    assert_eq!(v["b"], 2);
}

#[test]
fn toml_merge_preserves_unmodified_keys() {
    let base = "[database]\nhost = \"localhost\"\nport = 5432\n";
    let overlay = "[database]\nhost = \"prod\"\n";
    let result = merge_configs(base, overlay, Format::Toml).unwrap();
    assert!(result.contains("prod"));
    assert!(result.contains("5432"));
}
