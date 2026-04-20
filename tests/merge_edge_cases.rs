use configstitch::{merge_configs, Format};

#[test]
fn empty_overlay_is_identity() {
    let base = r#"{"key": "value"}"#;
    let overlay = r#"{}"#;
    let result = merge_configs(base, overlay, Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["key"], "value");
}

#[test]
fn both_empty() {
    let result = merge_configs("{}", "{}", Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(v.as_object().unwrap().is_empty());
}

#[test]
fn type_conflict_overlay_wins() {
    // When base has a map and overlay has a scalar for the same key,
    // overlay wins (replaces the map).
    let base = r#"{"db": {"host": "localhost"}}"#;
    let overlay = r#"{"db": "sqlite://local.db"}"#;
    let result = merge_configs(base, overlay, Format::Json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["db"], "sqlite://local.db");
}
