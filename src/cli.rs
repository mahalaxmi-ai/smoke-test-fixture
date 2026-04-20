use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::diff::diff_configs;
use crate::error::ConfigError;
use crate::format::{self, Format};
use crate::merge::merge_configs;

#[derive(Parser)]
#[command(name = "configstitch", about = "Merge layered configuration files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Merge multiple configuration files using overlay semantics
    Merge {
        /// Configuration files to merge (in order: base, then overlays)
        files: Vec<PathBuf>,

        /// Output format (toml or json)
        #[arg(short, long)]
        format: Option<String>,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Show differences between two configuration files
    Diff {
        /// First configuration file
        file_a: PathBuf,
        /// Second configuration file
        file_b: PathBuf,
    },

    /// Validate a configuration file
    Validate {
        /// Configuration file to validate
        file: PathBuf,

        /// JSON schema file to validate against
        #[arg(long)]
        schema: Option<PathBuf>,
    },
}

pub fn run() -> Result<(), ConfigError> {
    let cli = Cli::parse();

    match cli.command {
        Command::Merge {
            files,
            format: fmt_arg,
            output,
        } => {
            if files.len() < 2 {
                return Err(ConfigError::ParseError {
                    format: "CLI".to_string(),
                    message: "at least two files required for merge".to_string(),
                });
            }

            let fmt = match fmt_arg.as_deref() {
                Some("toml") => Format::Toml,
                Some("json") => Format::Json,
                Some(other) => {
                    return Err(ConfigError::UnsupportedFormat {
                        format: other.to_string(),
                    })
                }
                None => format::detect_format(&files[0])?,
            };

            let base = std::fs::read_to_string(&files[0])?;
            let mut result = base;

            for overlay_path in &files[1..] {
                let overlay = std::fs::read_to_string(overlay_path)?;
                result = merge_configs(&result, &overlay, fmt)?;
            }

            match output {
                Some(path) => std::fs::write(path, &result)?,
                None => print!("{result}"),
            }
        }

        Command::Diff { file_a, file_b } => {
            let fmt = format::detect_format(&file_a)?;
            let content_a = std::fs::read_to_string(&file_a)?;
            let content_b = std::fs::read_to_string(&file_b)?;

            let diff = diff_configs(&content_a, &content_b, fmt)?;
            if diff.is_empty() {
                println!("No differences.");
            } else {
                print!("{diff}");
            }
        }

        Command::Validate { file: _, schema: _ } => {
            todo!("schema validation not yet implemented");
        }
    }

    Ok(())
}
