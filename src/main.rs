use std::{env, process};
use crate::config::Config;

mod config;

fn main() {

    // Parse arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
