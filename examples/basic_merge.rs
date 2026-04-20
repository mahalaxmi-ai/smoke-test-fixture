use configstitch::{merge_configs, Format};

fn main() {
    let base = r#"
[database]
host = "localhost"
port = 5432

[logging]
level = "info"
"#;

    let production = r#"
[database]
host = "db.prod.internal"

[logging]
level = "warn"
"#;

    match merge_configs(base, production, Format::Toml) {
        Ok(merged) => {
            println!("Merged configuration:");
            println!("{merged}");
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
