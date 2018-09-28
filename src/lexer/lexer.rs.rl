use regex::Regex;
use token::token::Token;

pub struct Lexer {
    input: String,

    tokens: Vec<Token>,

    current_state: String, // "line_begin"
    next_state: Option<String>,

    p: usize
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            tokens: vec![],
            current_state: String::from("line_begin"),
            next_state: None,
            p: 0,
        }
    }

    // TODO DOC
    // return a Token
    pub fn advance(&mut self) -> Option<Token> {
        println!("---\nlexer.advance");

        if !self.tokens.is_empty() { return Some(self.tokens.remove(0)); }

        let mut is_holding = true; // TODO NOTE

        loop {
            println!("  lexer#advance: looping...");
            println!("    current_state: {}", self.current_state);
            println!("    next_state: {:?}", self.next_state);
            println!("    is_holding: {:?}", is_holding);

            let mut is_breaking = false;

            // transfer pointers
            if !is_holding {
                self.p += 1;
            }

            if let Some(next_state) = self.next_state.clone() {
                // transfer to next state
                self.current_state = next_state;
                self.next_state = None;
            }

            match self.current_state.as_ref() {
                // %% write each scanners branch
                // NOTE
                // that includes
                //   finding the longest match
                //   invoking the action
                _ => { panic!("unreachable: cant match current_state"); }
            };

            if is_breaking { break; }
        }

        panic!("WIP");
    }

}
