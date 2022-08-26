use crate::{Config, DoubleBuffer};
use crate::token::Token;

enum State {
    Init,
    Id,
    Comment,
    DelimiterEqual,
    OperatorPlus,
    OperatorAsterisk,
    OperatorLess,
}

enum Event {
    None,
    Digit,
    Letter,
    Underscore,
    Hashtag,
    NewLine,
    DoubleQuotes,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Less,
    Greater,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningSquareBracket,
    ClosingSquareBracket,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    Coma,
    Colon,
    Dot,
    Equal,
    NotRecognized,
}

pub struct LexicalAnalyzer {
    state: State,
    event: Event,
    double_buffer: DoubleBuffer,
}

impl LexicalAnalyzer {
    pub fn new(config: Config) -> Result<LexicalAnalyzer, &'static str> {
        let state = State::Init;
        let event = Event::None;
        let double_buffer = DoubleBuffer::new(config)?;


        Ok(LexicalAnalyzer { state, event, double_buffer })
    }
}

impl Iterator for LexicalAnalyzer {
    type Item = Result<Token, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = match self.double_buffer.next() {
                None => return None,
                Some(c) => match c {
                    Ok(c) => c,
                    Err(err) => return Some(Err(err))
                }
            };

            if c.is_alphabetic() {
                self.event = Event::Letter;
            } else if c.is_numeric() {
                self.event = Event::Digit;
            } else if c == '#' {
                self.event = Event::Hashtag;
            } else if c == '\n' {
                self.event = Event::NewLine;
            } else if c == '_' {
                self.event = Event::Underscore;
            } else if c == '"' {
                self.event = Event::DoubleQuotes;
            } else if c == '+' {
                self.event = Event::Plus;
            } else if c == '-' {
                self.event = Event::Minus;
            } else if c == '*' {
                self.event = Event::Asterisk;
            } else if c == '/' {
                self.event = Event::Slash;
            } else if c == '<' {
                self.event = Event::Less;
            } else if c == '>' {
                self.event = Event::Greater;
            } else if c == '(' {
                self.event = Event::OpeningParenthesis;
            } else if c == ')' {
                self.event = Event::ClosingParenthesis;
            } else if c == '[' {
                self.event = Event::OpeningSquareBracket;
            } else if c == ']' {
                self.event = Event::ClosingSquareBracket;
            } else if c == '{' {
                self.event = Event::OpeningCurlyBracket;
            } else if c == '}' {
                self.event = Event::ClosingCurlyBracket;
            } else if c == ',' {
                self.event = Event::Coma;
            } else if c == ':' {
                self.event = Event::Colon;
            } else if c == '.' {
                self.event = Event::Dot;
            } else if c == '=' {
                self.event = Event::Equal;
            } else {
                self.event = Event::NotRecognized;
            }

            match self.state {
                State::Init => {
                    match self.event {
                        Event::Hashtag => self.state = State::Comment,
                        Event::Letter => self.state = State::Id,
                        Event::Plus => self.state = State::OperatorPlus,
                        Event::Asterisk => self.state = State::OperatorAsterisk,
                        Event::Less => self.state = State::OperatorLess,
                        Event::Equal => self.state = State::DelimiterEqual,
                        Event::Minus | Event::Slash | Event::Greater => {
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }
                        Event::OpeningCurlyBracket | Event::OpeningSquareBracket |
                        Event::OpeningParenthesis | Event::ClosingParenthesis |
                        Event::ClosingCurlyBracket | Event::ClosingSquareBracket |
                        Event::Coma | Event::Colon | Event::Dot => {
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Delimiter(lexeme);

                            return Some(Ok(token));
                        }
                        _ => self.double_buffer.reject()
                    }
                }
                State::Comment => {
                    match self.event {
                        Event::NewLine => {
                            self.state = State::Init;
                            self.double_buffer.reject();
                        }
                        _ => ()
                    }
                }
                State::Id => {
                    match self.event {
                        Event::Digit | Event::Letter | Event::Underscore => (),
                        _ => {
                            self.state = State::Init;

                            self.double_buffer.back();
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Id(lexeme);

                            return Some(Ok(token));
                        }
                    }
                }
                State::OperatorPlus => {
                    match self.event {
                        Event::Equal => {
                            self.state = State::Init;

                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Delimiter(lexeme);

                            return Some(Ok(token));
                        }

                        _ => {
                            self.state = State::Init;

                            self.double_buffer.back();
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }
                    }
                }
                State::OperatorAsterisk => {
                    match self.event {
                        Event::Asterisk => {
                            self.state = State::Init;

                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }

                        _ => {
                            self.state = State::Init;

                            self.double_buffer.back();
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }
                    }
                }
                State::OperatorLess => {
                    match self.event {
                        Event::Equal => {
                            self.state = State::Init;

                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }

                        _ => {
                            self.state = State::Init;

                            self.double_buffer.back();
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }
                    }
                }

                State::DelimiterEqual => {
                    match self.event {
                        Event::Equal => {
                            self.state = State::Init;

                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Operator(lexeme);

                            return Some(Ok(token));
                        }

                        _ => {
                            self.state = State::Init;

                            self.double_buffer.back();
                            let lexeme = self.double_buffer.get_lexeme();
                            let token = Token::Delimiter(lexeme);

                            return Some(Ok(token));
                        }
                    }
                }

                _ => {}
            }
        }
    }
}