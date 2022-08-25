use crate::{Config, DoubleBuffer};


pub struct Token {
    pub id: u32,
    pub lexeme: String,
}

enum State {
    Init,
    Id,
    Comment,
}

enum Event {
    None,
    Digit,
    Letter,
    Alpha,
    Underscore,
    Hashtag,
    NewLine,
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

            if c.is_alphabetic() {
                self.event = Event::Letter;
            } else if c.is_alphanumeric() {
                self.event = Event::Alpha;
            } else if c.is_numeric() {
                self.event = Event::Digit;
            } else if c == '#' {
                self.event = Event::Hashtag;
            } else if c == '\n' {
                self.event = Event::NewLine;
            } else {
                self.event = Event::NotRecognized;
            }

            match self.state {
                State::Init => {
                    match self.event {
                        Event::Hashtag => self.state = State::Comment,
                        _ => self.double_buffer.reject()
                    }
                }
                State::Comment => {
                    match self.event {
                        Event::NewLine => {
                            self.state = State::Init;

                            self.double_buffer.back();
                            token.lexeme = self.double_buffer.get_lexeme();

                            return Some(Ok(token));
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
}