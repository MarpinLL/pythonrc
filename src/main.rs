use std::{env, process};
use std::collections::HashMap;
use crate::config::Config;
use crate::input_system::DoubleBuffer;
use crate::lexical_analyzer::LexicalAnalyzer;
use crate::token::Token;

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
        ("else".to_string(), Token::Keyword("else".to_string())),
        ("import".to_string(), Token::Keyword("import".to_string())),
        ("return".to_string(), Token::Keyword("return".to_string())),
        ("for".to_string(), Token::Keyword("for".to_string())),
        ("as".to_string(), Token::Keyword("as".to_string())),
        ("def".to_string(), Token::Keyword("def".to_string())),
        ("elif".to_string(), Token::Keyword("elif".to_string())),
        ("if".to_string(), Token::Keyword("if".to_string())),
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

        match token {
            Token::Id(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }

            Token::String(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }

            Token::Operator(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }

            Token::Delimiter(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }

            Token::Integer(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }

            Token::Float(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }

            Token::Keyword(ref lexeme) => {
                println!("<{}, {}>", token.value(), lexeme);
            }
        }
    }
}
