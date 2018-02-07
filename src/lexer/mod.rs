use std::collections::HashMap;

use parser::parser::Token;

mod input_stream;      use lexer::input_stream::InputStream;
mod lexing_state;      use lexer::lexing_state::{LexingState};
mod shared_actions;
mod transactions;
mod action;            use lexer::action::{Action};
mod matching_patterns;

pub struct Lexer {
    // TODO CONSTant
    transactions: HashMap<LexingState, Vec<Box<Action>>>,

    input_stream: InputStream,
    state: LexingState,

    is_breaking: bool,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {
        Lexer {
            state: LexingState::LineBegin,
            transactions: transactions::construct(),
            is_breaking: false,

            input_stream: InputStream::new(input_string),

            tokens: Vec::new(),
        }
    }

    // TODO return Result
    pub fn lex(&mut self) {
        loop {
            // TODO advance and advance and advance
            self.advance();

            // TODO not the correct way
            if (self.input_stream.no_more()) {
                break;
            }
        }
    }

    // return a token
    // 
    // TODO
    // then the current `emit` is not correct
    // every `exec()` should emit a token
    // 
    // TODO wrap in a Result
    // 

    fn advance(&mut self) {
        // TODO token queue

        // println!("--- advance ---");

        // 
        self.exec();
    }

    // match-state-invoke-action loop
    // 
    // exec machine until encounter break
    // 
    fn exec(&mut self) {
        loop {
            println!("\n--- exec looping, state: {:?} ---", self.state);

            if ( self.is_breaking == true ) {
                self.is_breaking = false;
                println!("breaking...");
                break;
            }

            // get actions
            let actions = &self.transactions.get(&self.state).unwrap().clone();

            // find matching action
            let action= self.input_stream.longest_matching_action(&actions).expect("cant match any action");
            println!("matched action: {:?}", action.regex);

            // invoke proc
            let procedure = action.procedure;
            procedure(self);
        }
    }

    fn flag_breaking(&mut self) {
        self.is_breaking = true;
    }

    fn emit_token(&mut self, token: Token) {
        println!("emitting token: {:?}", token);

        self.tokens.push(token);
    }
}
