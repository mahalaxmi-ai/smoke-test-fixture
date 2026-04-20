use std::process;

fn main() {
    if let Err(e) = configstitch::cli::run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
