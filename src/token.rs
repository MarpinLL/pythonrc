pub enum Token {
    Id(String),
    String(String),
    Operator(String),
    Delimiter(String),
    Integer(String),
    Float(String),
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
        }
    }
}