use std::path::Path;

use configstitch::Format;

#[test]
fn detect_toml_extension() {
    let fmt = configstitch::format::detect_format(Path::new("config.toml")).unwrap();
    assert_eq!(fmt, Format::Toml);
}

#[test]
fn detect_json_extension() {
    let fmt = configstitch::format::detect_format(Path::new("config.json")).unwrap();
    assert_eq!(fmt, Format::Json);
}

#[test]
fn unknown_extension_returns_error() {
    let result = configstitch::format::detect_format(Path::new("config.xml"));
    assert!(result.is_err());
}
