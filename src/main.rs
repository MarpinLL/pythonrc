use std::{env, process};
use crate::config::Config;
use crate::input_system::DoubleBuffer;
use crate::lexical_analyzer::LexicalAnalyzer;

mod config;
mod input_system;
mod lexical_analyzer;

fn main() {

    // Parse arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lexical_analyzer = LexicalAnalyzer::new(config).unwrap_or_else(|err| {
        eprint!("Problem creating the lexical analyzer: {}", err);
        process::exit(1);
    });

    for token in lexical_analyzer {
        let token = match token {
            Ok(token) => token,
            Err(err) => {
                eprint!("Problem reading the next token: {}", err);
                process::exit(1);
            }
        };

        println!("{}", token.id);
    }
}
