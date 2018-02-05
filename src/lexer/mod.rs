use std::collections::HashMap;

use parser::parser::Token;

mod input_stream;      use lexer::input_stream::InputStream;
mod action;            use lexer::action::{Action, ActionProc};
mod lexing_state;      use lexer::lexing_state::{LexingState};
mod matching_patterns;

pub struct Lexer {
    // TODO CONSTant
    state_actions: HashMap<LexingState, Vec<Box<Action>>>,

    input_stream: InputStream,
    state: LexingState,

    is_breaking: bool,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {

        let patterns = matching_patterns::construct();

        let mut shared_actions: HashMap<&'static str, ActionProc> = HashMap::new();

        // original do_eof
        shared_actions.insert("do_eof", |lexer: &mut Lexer| {
                println!("action invoked for c_eof");
                lexer.flag_breaking();
            }
        );

        // TODO macro-ize
        let mut state_actions = HashMap::new();

        state_actions.insert(get_state!("line_begin"), vec![

            // original action for c_any
            box Action {
                regex: patterns.get("c_any").unwrap().clone(), // TODO clone?
                procedure: |lexer: &mut Lexer| {
                    println!("action invoked for c_any");

                    lexer.input_stream.simulate_fhold();
                    lexer.state = LexingState::ExprValue;
                }
            },

            // TODO
            // original action for eof
            box Action {
                regex: patterns.get("c_eof").unwrap().clone(), // TODO clone?
                procedure: shared_actions.get("do_eof").unwrap().clone()
            }
        ]);

        state_actions.insert(get_state!("expr_value"), vec![
            // original action for c_any
            box Action {
                regex: patterns.get("c_any").unwrap().clone(), // TODO clone?
                procedure: |lexer: &mut Lexer| {
                    println!("action invoked for c_any");

                    lexer.input_stream.simulate_fhold();
                    lexer.state = LexingState::ExprBegin;
                }
            }

            // TODO
            // original action for eof
        ]);

        state_actions.insert(get_state!("expr_begin"), vec![
            // original action for r"[+\-] w_any* [0-9]"
            // box Action {
            //     regex: Regex::new(r"^[+\-][[:space:]]*[0-9]").unwrap(), //r"[+\-] w_any* [0-9]"
            //     procedure: |lexer: &mut Lexer| {
            //         println!("proc in action");
            //     }
            // }

            // original action for c_any
            box Action {
                regex: patterns.get("c_any").unwrap().clone(),
                procedure: |lexer: &mut Lexer| {
                    println!("action invoked for c_any");

                    lexer.input_stream.hold_current_token();
                    lexer.state = LexingState::ExprEnd;
                }
            }
        ]);

        state_actions.insert(get_state!("expr_end"), vec![

            // original action for:
            //     [1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec

            box Action {
                // TODO handle error :trailing_in_number
                regex: patterns.get("int_dec").unwrap().clone(),
                procedure: |lexer: &mut Lexer| {
                    println!("action invoked for int_dec");

                    println!("current {:?}", lexer.input_stream.current_matched_token().unwrap());

                    let parsed_int = lexer.input_stream.current_matched_token().unwrap().parse::<i64>().unwrap();

                    lexer.emit_token(Token::T_INTEGER(parsed_int));
                }
            },

            // TODO NOT CORRESPONDING
            // original action for:
            //     w_newline

            box Action {
                regex: patterns.get("c_eol").unwrap().clone(),
                procedure: |lexer: &mut Lexer| {
                    println!("action invoked for c_eol");

                    // TODO
                    // any
                    // => { emit(:tNL, nil, @newline_s, @newline_s + 1)
                    //     fhold; fnext line_begin; fbreak; };

                    lexer.input_stream.simulate_fhold();
                    lexer.state = LexingState::LineBegin;
                    lexer.flag_breaking();
                }
            },

            // original action for:
            //     c_eof

            box Action {
                // TODO handle error :trailing_in_number
                regex: patterns.get("c_eof").unwrap().clone(),
                procedure: shared_actions.get("do_eof").unwrap().clone() // TODO clone?
            }

        ]);

        Lexer {
            state_actions,
            input_stream: InputStream::new(input_string),
            state: LexingState::LineBegin,
            is_breaking: false,
            tokens: Vec::new(),
        }
    }

    // TODO return Result
    pub fn lex(&mut self) {
        loop {
            // TODO advance and advance and advance
            self.advance();

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
            let actions = &self.state_actions.get(&self.state).unwrap().clone();

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
