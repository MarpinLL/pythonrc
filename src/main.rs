use std::{env, process};
use std::collections::HashMap;
use crate::config::Config;
use crate::input_system::DoubleBuffer;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::token::{Keyword, Token};

mod config;
mod input_system;
mod lexical_analyzer;
mod token;

fn main() {

    // Parse arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Create a symbol table populated with the keywords
    let mut symbol_table = HashMap::from([
        ("else".to_string(), Token::Keyword("else".to_string(), Keyword::Else)),
        ("import".to_string(), Token::Keyword("import".to_string(), Keyword::Import)),
        ("return".to_string(), Token::Keyword("return".to_string(), Keyword::Return)),
        ("for".to_string(), Token::Keyword("for".to_string(), Keyword::For)),
        ("as".to_string(), Token::Keyword("as".to_string(), Keyword::As)),
        ("def".to_string(), Token::Keyword("def".to_string(), Keyword::Def)),
        ("elif".to_string(), Token::Keyword("elif".to_string(), Keyword::Elif)),
        ("if".to_string(), Token::Keyword("if".to_string(), Keyword::If)),
    ]);

    // Create lexical analyzer
    let lexical_analyzer = LexicalAnalyzer::new(config, &mut symbol_table).unwrap_or_else(|err| {
        eprint!("Problem creating the lexical analyzer: {}", err);
        process::exit(1);
    });


    // Start analysis
    for token in lexical_analyzer {
        let token = match token {
            Ok(token) => token,
            Err(err) => {
                eprint!("Problem reading the next token: {}", err);
                process::exit(1);
            }
        };

        println!("{:?}", token);
    }
}
