use minigrep::{run, Config};
use std::{env, process};
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("[+] problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application Error : {}", e);
        process::exit(1)
    }
}
