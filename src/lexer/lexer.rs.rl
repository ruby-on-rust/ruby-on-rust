use regex::Regex;
use token::token::Token;

pub struct Lexer {
    input: String,

    tokens: Vec<Token>,

    current_state: String,
    p: usize
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            tokens: vec![],
            current_state: String::from("line_begin"),
            p: 0,
        }
    }

    // TODO DOC
    // return a Token
    pub fn advance(&mut self) -> Token {
        println!("---\nlexer.advance");

        if !self.tokens.is_empty() { return self.tokens.remove(0); }

        // %% write exec

        panic!("WIP");
    }

}
