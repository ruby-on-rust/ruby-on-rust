use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;
use token::token::Token;
use lexer::literal::Literal;

pub struct Lexer {
    input: String,

    tokens: Rc<RefCell<Vec<Token>>>,

    current_state: String, // "line_begin"
    next_state: Option<String>,

    p: isize,

    pub literal_stack: Vec<Literal>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            tokens: Rc::new(RefCell::new(vec![])),
            current_state: String::from("line_begin"),
            next_state: None,
            p: -1,
            literal_stack: vec![]
        }
    }

    // TODO DOC
    // return a Token
    pub fn advance(&mut self) -> Option<Token> {
        println!("---\nlexer.advance");

        if !self.tokens.borrow().is_empty() { return Some(self.tokens.borrow_mut().remove(0)); }

        // define these here to preserve value out of looping
        // let mut is_holding = false;
        let mut matched_slice: Option<String> = None; 

        loop {
            // transfer pointers
            self.p += 1;

            // detect EOF
            if self.p as usize == self.input.len() {
                print!("EOF detected!");
                return None;
            }

            // refresh
            let mut is_breaking = false;
            // is_holding = false;

            println!("  lexer#advance: looping...");
            println!("    current_state: {}", self.current_state);
            println!("    next_state: {:?}", self.next_state);
            println!("    self.p: {:?}", self.p);

            // transfer to next state
            if let Some(next_state) = self.next_state.clone() {
                self.current_state = next_state;
                self.next_state = None;
            }

            matched_slice = None;
            let mut matched_action_id: isize = -1;
            let mut matched_slice_start_pos: usize = 0; // ts
            let mut matched_slice_end_pos: usize = 0;   // te

            match self.current_state.as_ref() {
                // %% write each scanners branch
                // NOTE
                // that includes
                //   finding the longest match
                //   setting
                //     matched_slice
                //     matched_action_id
                _ => { panic!("unreachable: cant match current_state {}", self.current_state.clone()); }
            };

            // matched
            if let Some(some_matched_slice) = matched_slice {
                matched_slice_start_pos = self.p as usize;
                matched_slice_end_pos = matched_slice_start_pos + some_matched_slice.len();

                // NOTE set a default transfering value for p, maybe override in action
                self.p = matched_slice_end_pos as isize - 1; // will +1 upon next loop

                println!("    matched with:");
                println!("      matched_slice: {:?}", some_matched_slice);
                println!("      ts: {}, te: {}", matched_slice_start_pos, matched_slice_end_pos);

                // invoke action
                match matched_action_id {
                    // %% write matching action
                    -1 | _ => { panic!("unreachable! no matched action to invoke"); }
                }
            } else {
                panic!("unreachable! matched nothing");
            }

            if is_breaking { break; }
        }

        if !self.tokens.borrow().is_empty() { return Some(self.tokens.borrow_mut().remove(0)); }
        panic!("no tokens");
    }

    pub fn emit_token(&mut self, token: Token) {
        self.tokens.borrow_mut().push(token);
    }

    fn get_input_slice(&self, start_p: usize, end_p: usize) -> String {
        self.input.chars().skip(start_p).take(end_p - start_p + 1).collect()
    }

    fn get_current_slice_as_token_from_table(&mut self, table_name: &str, current_slice: String) -> Token {
        match table_name {
            // %% write token tables matching
            _ => { panic!("unreachable! no such table"); }
        }
    }

}
