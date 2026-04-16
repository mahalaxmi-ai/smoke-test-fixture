use std::path::PathBuf;
use std::process;

fn run(args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 {
        return Err(format!("Usage: {} <manifest.json> [manifest2.json ...]", args[0]));
    }

    let mut has_errors = false;

    for arg in &args[1..] {
        let path = PathBuf::from(arg);
        match manifest_validator::validate_manifest_file(&path) {
            Ok(manifest) => {
                eprintln!(
                    "VALID: {} (sprint: {}, {} items)",
                    manifest.manifest_id,
                    manifest.sprint_id,
                    manifest.items.len()
                );
            }
            Err(e) => {
                eprintln!("INVALID: {arg}: {e}");
                has_errors = true;
            }
        }
    }

    if has_errors {
        Err("One or more manifests failed validation".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run(args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_no_args() {
        let args = vec!["manifest-validator".to_string()];
        let result = run(args);
        assert!(result.is_err());
        assert!(result
            .as_ref()
            .err()
            .map_or(false, |e| e.contains("Usage")));
    }

    #[test]
    fn test_run_nonexistent_file() {
        let args = vec![
            "manifest-validator".to_string(),
            "/nonexistent/file.json".to_string(),
        ];
        let result = run(args);
        assert!(result.is_err());
    }
}
