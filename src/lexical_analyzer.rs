use std::collections::HashMap;
use crate::{Config, DoubleBuffer};
use crate::token::Token;

enum State {
    Init,
    Id,
    Comment(CommentState),
    String(StringState),
    MultilineComment(MultilineCommentState),
    Integer(IntegerState),
    Float(FloatState),
    Operator(OperatorState),
    Delimiter(DelimiterState),
}

enum CommentState {
    Q0,
    Q1,

}

enum StringState {
    SingleQuoteQ0,
    SingleQuoteQ1,
    DoubleQuoteQ0,
    DoubleQuoteQ1,
    DoubleQuoteQ2,
}

enum MultilineCommentState {
    Q0,
    Q1,
    Q2,
}

enum IntegerState {
    Decinteger,
    HexintegerQ0,
    HexintegerQ1,
}

enum FloatState {
    Q0,
    Q1,
    DotStart,
    Exponent,
}

enum OperatorState {
    PlusMinus,
    Asterisk,
    Less,
}

enum DelimiterState {
    General,
    Equal,
}

enum Event {
    None,
    NonZeroDigit,
    Zero,
    Letter,
    X,
    Underscore,
    Hashtag,
    NewLine,
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
    Exponent,
    SingleQuote,
    DoubleQuote,
    Space,
    Other,
}

pub struct LexicalAnalyzer<'a> {
    state: State,
    event: Event,
    double_buffer: DoubleBuffer,
    symbol_table: &'a mut HashMap<String, Token>,
    line_counter: u32,
}

impl LexicalAnalyzer<'_> {
    pub fn new<'a>(config: Config, symbol_table: &'a mut HashMap<String, Token>) -> Result<LexicalAnalyzer, &'static str> {
        let state = State::Init;
        let event = Event::None;
        let double_buffer = DoubleBuffer::new(config)?;
        let line_counter = 1;


        Ok(LexicalAnalyzer { state, event, double_buffer, symbol_table, line_counter })
    }
}

impl Iterator for LexicalAnalyzer<'_> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {

            // Get the next character
            let c = match self.double_buffer.next() {
                None => return None,
                Some(c) => match c {
                    Ok(c) => c,
                    Err(err) => return Some(Err(String::from(err)))
                }
            };

            // Transform character into event
            self.event =
                if c.is_alphabetic() {
                    if c != 'e' {
                        Event::Letter
                    } else if (c == 'x') | (c == 'X') {
                        Event::X
                    } else {
                        Event::Exponent
                    }
                } else if c.is_numeric() {
                    if c != '0' {
                        Event::NonZeroDigit
                    } else {
                        Event::Zero
                    }
                } else if c == '#' {
                    Event::Hashtag
                } else if c == '\n' {
                    self.line_counter = self.line_counter + 1;
                    Event::NewLine
                } else if c == '_' {
                    Event::Underscore
                } else if c == '\'' {
                    Event::SingleQuote
                } else if c == '"' {
                    Event::DoubleQuote
                } else if c == '+' {
                    Event::Plus
                } else if c == '-' {
                    Event::Minus
                } else if c == '*' {
                    Event::Asterisk
                } else if c == '/' {
                    Event::Slash
                } else if c == '<' {
                    Event::Less
                } else if c == '>' {
                    Event::Greater
                } else if c == '(' {
                    Event::OpeningParenthesis
                } else if c == ')' {
                    Event::ClosingParenthesis
                } else if c == '[' {
                    Event::OpeningSquareBracket
                } else if c == ']' {
                    Event::ClosingSquareBracket
                } else if c == '{' {
                    Event::OpeningCurlyBracket
                } else if c == '}' {
                    Event::ClosingCurlyBracket
                } else if c == ',' {
                    Event::Coma
                } else if c == ':' {
                    Event::Colon
                } else if c == '.' {
                    Event::Dot
                } else if c == '=' {
                    Event::Equal
                } else if c == ' ' {
                    Event::Space
                } else {
                    Event::Other
                };

            // DFA
            match self.state {
                State::Init => {
                    match self.event {
                        Event::Letter | Event::Exponent | Event::X => self.state = State::Id,
                        Event::Hashtag => self.state = State::Comment(CommentState::Q0),
                        Event::SingleQuote => self.state = State::String(StringState::SingleQuoteQ0),
                        Event::DoubleQuote => self.state = State::String(StringState::DoubleQuoteQ0),
                        Event::NonZeroDigit => self.state = State::Integer(IntegerState::Decinteger),
                        Event::Zero => self.state = State::Integer(IntegerState::HexintegerQ0),
                        Event::Dot => self.state = State::Float(FloatState::DotStart),
                        Event::Plus |
                        Event::Minus => self.state = State::Operator(OperatorState::PlusMinus),
                        Event::Asterisk => self.state = State::Operator(OperatorState::Asterisk),
                        Event::Less => self.state = State::Operator(OperatorState::Less),
                        Event::Slash |
                        Event::Greater => {
                            self.state = State::Operator(OperatorState::Asterisk);
                            break;
                        }
                        Event::OpeningParenthesis |
                        Event::ClosingParenthesis |
                        Event::OpeningSquareBracket |
                        Event::ClosingSquareBracket |
                        Event::OpeningCurlyBracket |
                        Event::ClosingCurlyBracket |
                        Event::Coma |
                        Event::Colon => {
                            self.state = State::Delimiter(DelimiterState::General);
                            break;
                        }
                        Event::Equal => self.state = State::Delimiter(DelimiterState::Equal),
                        _ => self.double_buffer.reject()
                    }
                }

                State::Id => {
                    match self.event {
                        Event::NonZeroDigit | Event::Zero |
                        Event::Letter | Event::Exponent | Event::X |
                        Event::Underscore => (),
                        _ => break
                    }
                }

                State::Comment(ref state) => {
                    match state {
                        CommentState::Q0 => {
                            match self.event {
                                Event::Hashtag | Event::NewLine =>
                                    return Some(Err(
                                        format!("Illegal character after # on line {}", self.line_counter)
                                    )),
                                _ => {
                                    self.state = State::Comment(CommentState::Q1);
                                    self.double_buffer.reject();
                                }
                            }
                        }
                        CommentState::Q1 => {
                            match self.event {
                                Event::Hashtag =>
                                    return Some(Err(
                                        format!("Illegal character after # on line {}", self.line_counter)
                                    )),
                                Event::NewLine => {
                                    self.state = State::Init;
                                    self.double_buffer.reject();
                                }
                                _ => self.double_buffer.reject()
                            }
                        }
                    }
                }

                State::String(ref state) => {
                    match state {
                        StringState::SingleQuoteQ0 => {
                            match self.event {
                                Event::SingleQuote => return Some(Err(
                                    format!("Illegal character after ' on line {}", self.line_counter)
                                )),
                                _ => self.state = State::String(StringState::SingleQuoteQ1)
                            }
                        }
                        StringState::SingleQuoteQ1 => {
                            match self.event {
                                Event::SingleQuote => break,
                                _ => ()
                            }
                        }
                        StringState::DoubleQuoteQ0 => {
                            match self.event {
                                Event::DoubleQuote => self.state = State::String(StringState::DoubleQuoteQ2),
                                _ => self.state = State::String(StringState::DoubleQuoteQ1),
                            }
                        }
                        StringState::DoubleQuoteQ1 => {
                            match self.event {
                                Event::DoubleQuote => break,
                                _ => (),
                            }
                        }

                        StringState::DoubleQuoteQ2 => {
                            match self.event {
                                Event::DoubleQuote => self.state = State::MultilineComment(MultilineCommentState::Q0),
                                _ => return Some(Err(
                                    format!("Expected another double quote on line {}", self.line_counter)
                                )),
                            }
                        }
                    }
                }

                State::MultilineComment(ref state) => {
                    match state {
                        MultilineCommentState::Q0 => {
                            match self.event {
                                Event::DoubleQuote => self.state = State::MultilineComment(MultilineCommentState::Q1),
                                _ => self.double_buffer.reject()
                            }
                        }
                        MultilineCommentState::Q1 => {
                            match self.event {
                                Event::DoubleQuote => self.state = State::MultilineComment(MultilineCommentState::Q2),
                                _ => return Some(Err(
                                    format!("Expected another double quote on line {}", self.line_counter),
                                )),
                            }
                        }
                        MultilineCommentState::Q2 => {
                            match self.event {
                                Event::DoubleQuote => {
                                    self.state = State::Init;
                                    self.double_buffer.reject();
                                }
                                _ => return Some(Err(
                                    format!("Expected another double quote on line {}", self.line_counter),
                                )),
                            }
                        }
                    }
                }

                State::Integer(ref state) => {
                    match state {
                        IntegerState::Decinteger => {
                            match self.event {
                                Event::NonZeroDigit | Event::Zero => (),
                                Event::Dot => self.state = State::Float(FloatState::Q0),
                                Event::Exponent => self.state = State::Float(FloatState::Exponent),
                                _ => break,
                            }
                        }
                        IntegerState::HexintegerQ0 => {
                            match self.event {
                                Event::X => self.state = State::Integer(IntegerState::HexintegerQ1),
                                Event::Dot => self.state = State::Float(FloatState::Q0),
                                Event::Exponent => self.state = State::Float(FloatState::Exponent),
                                _ => break
                            }
                        }
                        IntegerState::HexintegerQ1 => {
                            match self.event {
                                Event::NonZeroDigit | Event::Zero => (),
                                _ => break
                            }
                        }
                    }
                }

                State::Float(ref state) => {
                    match state {
                        FloatState::Q0 => {
                            match self.event {
                                Event::NonZeroDigit |
                                Event::Zero => self.state = State::Float(FloatState::Q1),
                                Event::NewLine |
                                Event::Space => break,
                                _ => return Some(Err(
                                    format!("Expected Float or Hexinteger after dot on line {}", self.line_counter),
                                )),
                            }
                        }
                        FloatState::Q1 => {
                            match self.event {
                                Event::NonZeroDigit | Event::Zero => (),
                                _ => break,
                            }
                        }
                        FloatState::DotStart => {
                            match self.event {
                                Event::NonZeroDigit | Event::Zero => self.state = State::Float(FloatState::Q1),
                                _ => {
                                    self.state = State::Delimiter(DelimiterState::Equal);
                                    break;
                                }
                            }
                        }
                        FloatState::Exponent => {
                            match self.event {
                                Event::Plus |
                                Event::Minus |
                                Event::NonZeroDigit |
                                Event::Zero => self.state = State::Float(FloatState::Q1),
                                _ => return Some(Err(
                                    format!("Expected +, - or number after Exponent on line {}", self.line_counter),
                                )),
                            }
                        }
                    }
                }

                State::Operator(ref state) => {
                    match state {
                        OperatorState::PlusMinus => {
                            match self.event {
                                Event::Equal => {
                                    self.state = State::Delimiter(DelimiterState::General);
                                    break;
                                }
                                _ => break,
                            }
                        }
                        OperatorState::Asterisk => {
                            match self.event {
                                Event::Asterisk => break,
                                _ => {
                                    self.state = State::Operator(OperatorState::PlusMinus);
                                    break;
                                }
                            }
                        }
                        OperatorState::Less => {
                            match self.event {
                                Event::Equal => break,
                                _ => {
                                    self.state = State::Operator(OperatorState::Asterisk);
                                    break;
                                }
                            }
                        }
                    }
                }

                State::Delimiter(ref state) => {
                    match state {
                        DelimiterState::General => {}
                        DelimiterState::Equal => {
                            match self.event {
                                Event::Equal => {
                                    self.state = State::Operator(OperatorState::PlusMinus);
                                    break;
                                }
                                _ => break
                            }
                        }
                    }
                }
            }
        }

        // Back the forward pointer if needed to read lexeme
        match self.state {
            State::Id |
            State::Integer(_) |
            State::Float(_) => self.double_buffer.back(),
            State::Operator(ref state) => {
                match state {
                    OperatorState::PlusMinus => self.double_buffer.back(),
                    _ => (),
                }
            }
            State::Delimiter(ref state) => {
                match state {
                    DelimiterState::General => (),
                    DelimiterState::Equal => self.double_buffer.back(),
                }
            }

            _ => ()
        }

        // Read lexeme
        let lexeme = self.double_buffer.get_lexeme();

        // Return token
        let token = match self.state {
            State::Id => {
                match self.symbol_table.get(&lexeme) {
                    None => {
                        self.symbol_table.insert(lexeme.clone(), Token::Id(lexeme.clone()));
                        let token = self.symbol_table.get(&lexeme).unwrap().clone();

                        Some(Ok(token))
                    }
                    Some(token) => Some(Ok(token.clone()))
                }
            }
            State::String(_) => Some(Ok(Token::String(lexeme))),
            State::Integer(_) => Some(Ok(Token::Integer(lexeme))),
            State::Float(_) => Some(Ok(Token::Float(lexeme))),
            State::Operator(_) => Some(Ok(Token::Operator(lexeme))),
            State::Delimiter(_) => Some(Ok(Token::Delimiter(lexeme))),
            _ => Some(Err(format!("Shouldn't reach here")))
        };

        self.state = State::Init;

        return token;
    }
}