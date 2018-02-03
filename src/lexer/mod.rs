use std::collections::HashMap;

use regex::Regex;

use parser::parser::Token;

mod input_stream;      use lexer::input_stream::InputStream;
mod action;            use lexer::action::Action;
mod matching_patterns;

enum LexingState {
    // :line_begin    => lex_en_line_begin,
    // :expr_dot      => lex_en_expr_dot,
    // :expr_fname    => lex_en_expr_fname,
    // :expr_value    => lex_en_expr_value,
    // :expr_beg      => lex_en_expr_beg,
    // :expr_mid      => lex_en_expr_mid,
    // :expr_arg      => lex_en_expr_arg,
    // :expr_cmdarg   => lex_en_expr_cmdarg,
    // :expr_end      => lex_en_expr_end,
    // :expr_endarg   => lex_en_expr_endarg,
    // :expr_endfn    => lex_en_expr_endfn,
    // :expr_labelarg => lex_en_expr_labelarg,

    // :interp_string => lex_en_interp_string,
    // :interp_words  => lex_en_interp_words,
    // :plain_string  => lex_en_plain_string,
    // :plain_words   => lex_en_plain_string,

    LineBegin,
    ExprBegin,
    ExprEnd,
}

pub struct Lexer {
    // TODO refine
    patterns: HashMap<&'static str, Regex>,

    // TODO CONSTant
    state_actions: HashMap<&'static str, Vec<Box<Action>>>,

    input_stream: InputStream,
    state: LexingState,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {

        let patterns = matching_patterns::construct();

        // TODO macro-ize
        let mut state_actions = HashMap::new();

        state_actions.insert("line_begin", vec![

            // original action for c_any
            box Action {
                regex: patterns.get("c_any").unwrap().clone(), // TODO clone?
                procedure: |lexer: &mut Lexer| {
                    println!("action invoked for c_any");

                    lexer.input_stream.simulate_fhold();
                    lexer.state = LexingState::ExprBegin;
                }
            }

        ]);

        state_actions.insert("expr_begin", vec![
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

        state_actions.insert("expr_end", vec![

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
            }
        ]);

        Lexer {
            patterns,
            state_actions,
            input_stream: InputStream::new(input_string),
            state: LexingState::LineBegin,
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
    // TODO wrap in a Result
    fn advance(&mut self) {
        // TODO token queue

        // 
        self.exec();
    }

    // match-state-invoke-action loop
    fn exec(&mut self) {
        loop {
            match self.state {
                LexingState::LineBegin => {
                    self.lex_at_state_line_begin();
                },
                LexingState::ExprBegin => {
                    self.lex_at_state_expr_begin();
                }
                LexingState::ExprEnd => {
                    self.lex_at_state_expr_end();
                }
            }
        }
    }

    fn lex_at_state_line_begin(&mut self) {
        println!("\n--- lex_at_state_line_begin ---");

        let actions = &self.state_actions.get("line_begin").unwrap().clone();

        // find matching action and invoke proc
        // TODO separate and share
        let action= self.input_stream.longest_matching_action(&actions).expect("cant match any action");
        println!("matched action: {:?}", action.regex);
        let procedure = action.procedure;
        procedure(self);
    }

    fn lex_at_state_expr_begin(&mut self) {
        println!("\n--- lex_at_state_expr_begin ---");

        let actions = &self.state_actions.get("expr_begin").unwrap().clone();

        // find matching action and invoke proc
        // TODO separate and share
        let action= box self.input_stream.longest_matching_action(&actions).expect("cant match any action");
        println!("matched action: {:?}", action.regex);
        let procedure = action.procedure;
        procedure(self);
    }

    fn lex_at_state_expr_end(&mut self) {
        println!("\n--- lex_at_state_expr_end ---");

        let actions = &self.state_actions.get("expr_end").unwrap().clone();

        // find matching action and invoke proc
        // TODO separate and share
        let action= box self.input_stream.longest_matching_action(&actions).expect("cant match any action");
        println!("matched action: {:?}", action.regex);
        let procedure = action.procedure;
        procedure(self);
    }

    // simulate fgoto
    // pub fn calc_next_state_after_expr_begin(&mut self) {
    //     println!("\n--- lex_at_state_expr_begin ---");

    //     let actions = &self.state_actions.get("expr_begin").unwrap().clone();

    //     // find matching action and invoke proc
    //     // TODO separate
    //     let action= box self.input_stream.longest_matching_action(&actions).expect("cant match any action");
    //     println!("matched action: {:?}", action.regex);
    //     let procedure = action.procedure;
    //     procedure(self);
    // }

    fn emit_token(&mut self, token: Token) {
        println!("emitting token: {:?}", token);

        self.tokens.push(token);
    }

}
