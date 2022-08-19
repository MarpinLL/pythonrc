use std::env;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // Get filename from arguments
        let filename = match args.next() {
            None => return Err("Didn't get a filename"),
            Some(arg) => arg
        };

        Ok(Config { filename })
    }
}

