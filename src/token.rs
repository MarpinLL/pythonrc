pub enum Token {
    Id(String),
    Operator(String),
    Delimiter(String),
}

impl Token {
    pub fn value(&self) -> &str {
        match *self {
            Token::Id(_) => {
                "Identifier"
            }
            Token::Operator(_) => {
                "Operator"
            }
            Token::Delimiter(_) => {
                "Delimiter"
            }
        }
    }
}