use minigrep::{Config, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_sensitive = env::var("CASE_SENSITIVE").is_err();

    let config = Config::new(&args, case_sensitive).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
