use crate::{Config, DoubleBuffer};
use crate::token::Token;

#[derive(Debug)]
enum State {
    Init,
    Id,
    Comment,
}

#[derive(Debug)]
enum Event {
    None,
    Digit,
    Letter,
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
            } else {
                self.event = Event::NotRecognized;
            }


            // println!("{:?} : {:?} : {}", self.state, self.event, c);

            match self.state {
                State::Init => {
                    match self.event {
                        Event::Hashtag => self.state = State::Comment,
                        Event::Letter => self.state = State::Id,
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
            }
        }
    }
}