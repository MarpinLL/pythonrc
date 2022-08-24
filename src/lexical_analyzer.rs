use crate::{Config, DoubleBuffer};


pub struct Token {
    pub id: u32,
    pub lexeme: String,
}

enum State {
    Init,
    Id,
    Accepted,
    Rejected,
}

enum Event {
    None,
    Digit,
    Letter,
    Alpha,
    Underscore,
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
        let mut token = Token { id: 0, lexeme: "".to_string() };

        loop {
            let c = match self.double_buffer.next() {
                None => return None,
                Some(c) => match c {
                    Ok(c) => c,
                    Err(err) => return Some(Err(err))
                }
            };

            print!("{}", c);

            if c.is_alphabetic() {
                self.event = Event::Letter
            } else if c.is_alphanumeric() {
                self.event = Event::Alpha
            } else if c.is_numeric() {
                self.event = Event::Digit
            }

            match self.state {
                State::Init => {
                    match self.event {
                        Event::Letter => {
                            self.state = State::Id
                        }
                        _ => self.state = State::Init
                    }
                }
                State::Id => {
                    match self.event {
                        Event::None => {}
                        Event::Digit => {}
                        Event::Letter => {}
                        Event::Alpha => {}
                        Event::Underscore => {}
                        Event::NotRecognized => {
                            self.state = State::Accepted;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        match self.state {
            State::Init => None,
            State::Id => Some(Err("a")),
            State::Accepted => {
                self.double_buffer.back();
                token.id = 400;
                token.lexeme = self.double_buffer.get_lexeme();
                self.state = State::Init;
                Some(Ok(token))
            }
            State::Rejected => {
                self.state = State::Init;
                Some(Ok(token))
            }
        }
    }
}