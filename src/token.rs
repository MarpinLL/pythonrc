pub enum Token {
    Id(String)
}

impl Token {
    pub fn value(&self) -> u32 {
        match *self {
            Token::Id(_) => {
                400
            }
        }
    }
}