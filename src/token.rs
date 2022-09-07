#[derive(Debug, Clone)]
pub enum Token {
    Id(String),
    String(String),
    Operator(String),
    Delimiter(String),
    Integer(String),
    Float(String),
    Keyword(String, Keyword),
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Else,
    Import,
    Return,
    For,
    As,
    Def,
    Elif,
    If,
}