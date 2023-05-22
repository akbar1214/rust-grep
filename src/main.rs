mod args;
mod finder;

use std::{env, process};


use crate::args::Config;
use crate::finder::execute;


fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = execute(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
