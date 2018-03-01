use lexer::Lexer;
use parser::token::Token;

// TODO
#[derive( Debug )]
pub struct Node {}

pub struct Parser {
    lexer: Lexer,

    tokens: Vec<Token>,
    current_p: usize, // TODO NOTE
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),

            tokens: vec![],
            current_p: 0,
        }
    }

    pub fn parse(&mut self) -> Node {
        self.p_simple_numeric()
    }

    // TODO
    // shared match-and-consume
    // fn match_token() -> bool {
    // }

    // get a new one if necessary
    fn current_token(&mut self) -> Token {
        if self.tokens.get(self.current_p).is_none() {
            self.tokens.push(self.lexer.advance().expect("no token emitted after lexer.advance()"));
        }

        let token = (*self.tokens.get(self.current_p).expect("no current token for current_p")).clone();

        token
    }

    fn consume_current_token(&mut self) {
        self.current_p += 1;
    }

    fn p_simple_numeric(&mut self) -> Node {
        match self.current_token() {
            Token::T_INTEGER(i) => {
                self.consume_current_token();
                return Node {}
            },
            _ => { panic!("UNIMPL"); }
        }
    }
}
