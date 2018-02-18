use std::collections::HashMap;

use parser::parser::Token;

mod input_stream;      use self::input_stream::InputStream;
mod lexing_state;      use self::lexing_state::{LexingState};
mod shared_actions;    use self::shared_actions::{TSharedActions};
mod machines;
mod action;            use self::action::{Action};
mod matching_patterns;
mod tokens_tables;
mod shared_functions;
mod stack_state;       use self::stack_state::StackState;

pub struct Lexer {
    states_stack: Vec<LexingState>,

    tokens_tables: HashMap<&'static str, HashMap<&'static str, Token>>,
    shared_actions: TSharedActions,
    machines: HashMap<LexingState, Vec<Box<Action>>>,

    input_stream: InputStream,

    is_breaking: bool,
    // CORRESPOND @command_state in lexer.rl
    command_state: bool,

    cond: StackState,
    cmdarg: StackState,
    paren_nest: usize, // TODO seems like a Ruby 1.9 thing

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {
        let shared_actions = shared_actions::construct();

        Lexer {
            states_stack: vec![LexingState::LineBegin],
            tokens_tables: tokens_tables::construct(),

            shared_actions: shared_actions.clone(),
            machines: machines::construct(&shared_actions),

            input_stream: InputStream::new(input_string),

            is_breaking: false,

            command_state: false,
            cond: StackState::new(),
            cmdarg: StackState::new(),
            paren_nest: 0,

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

        // TODO NOTE
        // we're using `states_stack.last()`(top of the stack) as the corresponding to `@cs`(current state)
        // not sure if this will cause any issue
        self.command_state = ( self.states_stack.last().unwrap() == &LexingState::ExprValue ) || 
                             ( self.states_stack.last().unwrap() == &LexingState::LineBegin );

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

    // emit current slice as token from table
    // TODO naming
    fn emit_token_from_table(&mut self, table_name: &str) {
        let token_str = self.input_stream.current_token().unwrap().clone();

        let tokens_table = self.tokens_tables.get(table_name).unwrap();
        let token = tokens_table.get(token_str.as_str()).unwrap();

        self.tokens.push((*token).clone());
    }

    fn invoke_proc(&mut self, proc_name: &str) {
        let procedure = self.shared_actions.get(proc_name).expect("no such proc in shared_actions").clone();
        procedure(self);
    }
}
