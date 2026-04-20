use std::path::PathBuf;

/// Errors that can occur during configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to parse {format} input: {message}")]
    ParseError { format: String, message: String },

    #[error("failed to serialize to {format}: {message}")]
    SerializeError { format: String, message: String },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("cannot detect format for path: {path:?}")]
    FormatDetectionError { path: PathBuf },

    #[error("schema validation failed: {message}")]
    SchemaError { message: String },

    #[error("unsupported format: {format}")]
    UnsupportedFormat { format: String },
}
