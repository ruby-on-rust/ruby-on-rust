use std::collections::HashMap;

use parser::parser::Token;

mod input_stream;      use lexer::input_stream::InputStream;
mod lexing_state;      use lexer::lexing_state::{LexingState};
mod shared_actions;
mod machines;
mod action;            use lexer::action::{Action};
mod matching_patterns;
mod tokens_tables;

pub struct Lexer {
    states_stack: Vec<LexingState>,

    tokens_tables: HashMap<&'static str, HashMap<&'static str, Token>>,

    machines: HashMap<LexingState, Vec<Box<Action>>>,

    input_stream: InputStream,

    is_breaking: bool,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {
        Lexer {
            states_stack: vec![LexingState::LineBegin],
            tokens_tables: tokens_tables::construct(),
            machines: machines::construct(),
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

        // TODO HACKING NOT WORKING not the correct way
        if (self.input_stream.no_more()) {
            println!("no more..., breaking...");
            return;
        }

        // 
        self.exec();
    }

    // match-state-invoke-action loop
    // 
    // exec machine until encounter break
    // 
    fn exec(&mut self) {
        self.is_breaking = false;

        loop {
            println!("\n--- exec looping, states_stack: {:?} ---", self.states_stack);

            if ( self.is_breaking == true ) {
                println!("breaking...");
                break;
            }

            // ===

            // get actions
            let state = self.states_stack.pop().expect("states_stack is empty");
            let actions = self.machines.get(&state).unwrap().clone();

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

    fn push_next_state(&mut self, state: LexingState) {
        self.states_stack.push(state);
    }

    fn emit_token(&mut self, token: Token) {
        println!("emitting token: {:?}", token);

        self.tokens.push(token);
    }

    // take current_token
    // TODO naming
    fn emit_token_from_table(&mut self, table_name: &str) {
        let token_str = self.input_stream.current_token().unwrap().clone();

        let tokens_table = self.tokens_tables.get(table_name).unwrap();
        let token = tokens_table.get(token_str.as_str()).unwrap();

        self.tokens.push((*token).clone());
    }
}
