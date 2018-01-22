use std::io;
use std::fs::File;

use parser::Token;

mod lexing_state;
use lexer::lexing_state::LexingState;
mod input_stream;
use lexer::input_stream::InputStream;

pub struct Lexer {
    input: InputStream,

    state: LexingState,

    // token_buf: String,

    pub tokens: Vec<Token>,
}


// TODO move token_buf related fns to TokenBuf struct
impl Lexer {
    // fn token_buf_concat()
}

// LexingState related
// TODO move
impl Lexer {
    fn set_state(&mut self, new_state: LexingState) {
        self.state = new_state;
    }
}

impl Lexer {
    pub fn new(chars: io::Chars<File>) -> Lexer {
        Lexer {
            input: InputStream::new(chars.map(|c| c.unwrap()).collect()),

            state: LexingState::EXPR_BEG,

            // token_buf: String::new(),

            tokens: vec![],
        }
    }

    // TODO return Result
    pub fn lex(&mut self) {
        loop {
            let c = self.input.next();
            match c {
                None => { break; }
                Some(c) => {
                    self.lex_current_char();
                }
            }
        }
    }

    fn lex_current_char(&mut self){
        match self.input.current() {
            '0'...'9' => {
                self.parse_numeric();
            },
            c @ _ => {
                println!("unknown char: {}({})", c, c.escape_unicode())
            }
        }
    }

    // ORIGINAL parse_numeric
    // parse numeric starting from current char
    // NOTE including + -
    fn parse_numeric(&mut self) {
        self.set_state(LexingState::EXPR_END);

        let mut num_str = String::from( self.input.current().to_string() );
        loop {
            match self.input.next() {
                Some(c) => {
                    match c {
                        '0'...'9' => {
                            num_str.push(c);
                        },
                        _ => {
                            self.input.put_back(c);
                            break;
                        }
                    }
                },
                None => { break; },
            }
        }

        let num = num_str.parse::<i64>().unwrap();

        // TODO
        self.tokens.push(Token::T_INTEGER(num));
    }
}
