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

        // define these here to preserve value out of looping
        let mut is_holding = true; // TODO NOTE init as true, so done have to handle -1
        let mut matched_slice: Option<String> = None; 

        loop {
            let mut is_breaking = false;

            println!("  lexer#advance: looping...");
            println!("    current_state: {}", self.current_state);
            println!("    next_state: {:?}", self.next_state);
            println!("    is_holding: {:?}", is_holding);

            // transfer pointers
            if !is_holding {
                self.p += 1;
            }
            println!("    self.p: {:?}", self.p);
            is_holding = false;

            // transfer to next state
            if let Some(next_state) = self.next_state.clone() {
                self.current_state = next_state;
                self.next_state = None;
            }

            matched_slice = None;
            let mut matched_action_id: isize = -1;
            let mut matched_slice_start_pos = 0; // ts
            let mut matched_slice_end_pos = 0;   // te

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

            if let Some(some_matched_slice) = matched_slice.clone() {
                matched_slice_start_pos = self.p;
                matched_slice_end_pos = self.p + some_matched_slice.len() - 1;
                self.p = matched_slice_end_pos; // will +1 upon next loop, unless is_holding
            }

            match matched_action_id {
                // %% write matching action
                -1 | _ => { panic!("unreachable! no matched action to invoke"); }
            }

            if is_breaking { break; }
        }

        if !self.tokens.is_empty() { return Some(self.tokens.remove(0)); }
        panic!("no tokens");
    }

    fn get_input_slice(&self, start_p: usize, end_p: usize) -> String {
        self.input.chars().skip(start_p).take(end_p - start_p + 1).collect()
    }
}
