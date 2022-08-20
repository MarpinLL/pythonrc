use std::{env, process};
use crate::config::Config;
use crate::input_system::InputBuffer;

mod config;
mod input_system;

fn main() {

    // Parse arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Create the input buffer
    let input_buffer = InputBuffer::new(config).unwrap_or_else(|err| {
        eprint!("Couldn't create the input buffer: {}", err);
        process::exit(1);
    });

    // Iterate through the buffer and get all characters
    for c in input_buffer {
        print!("{}", c);
    }
}
