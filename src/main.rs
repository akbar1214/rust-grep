use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
