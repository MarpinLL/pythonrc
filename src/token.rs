pub enum Token {
    Id(String),
    String(String),
    Operator(String),
    Delimiter(String),
    Integer(String),
    Float(String),
    Keyword(String),
}

impl Token {
    pub fn value(&self) -> &str {
        match *self {
            Token::Id(_) => {
                "Identifier"
            }
            Token::String(_) => {
                "String"
            }
            Token::Operator(_) => {
                "Operator"
            }
            Token::Delimiter(_) => {
                "Delimiter"
            }
            Token::Integer(_) => {
                "Integer"
            }
            Token::Float(_) => {
                "Float"
            }
            Token::Keyword(_) => {
                "Keyword"
            }
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Id(lexeme) => Token::Id(lexeme.clone()),
            Token::String(lexeme) => Token::String(lexeme.clone()),
            Token::Operator(lexeme) => Token::Operator(lexeme.clone()),
            Token::Delimiter(lexeme) => Token::Delimiter(lexeme.clone()),
            Token::Integer(lexeme) => Token::Integer(lexeme.clone()),
            Token::Float(lexeme) => Token::Float(lexeme.clone()),
            Token::Keyword(lexeme) => Token::Keyword(lexeme.clone()),
        }
    }
}