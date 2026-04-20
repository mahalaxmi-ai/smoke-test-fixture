pub mod cli;
pub mod diff;
pub mod error;
pub mod format;
pub mod merge;

pub use error::ConfigError;
pub use format::Format;
pub use merge::merge_configs;
