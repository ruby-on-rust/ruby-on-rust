use std::collections::HashMap;

use lexer::Lexer;
use lexer::lexing_state::LexingState;
use lexer::action::{Action, ActionProc};
use lexer::matching_patterns;
use parser::parser::Token;

pub fn construct(shared_actions: HashMap<&'static str, ActionProc>) -> HashMap<LexingState, Vec<Box<Action>>> {
    let patterns = matching_patterns::construct();

    let mut transactions = HashMap::new();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: patterns.get($pattern_name).unwrap().clone(), // TODO clone?
                procedure: $procedure
            }
        };
    }

    macro_rules! transaction {
        ( $state:expr, $actions:expr ) => {
            let state = $state.parse::<LexingState>().unwrap();
            transactions.insert(state, $actions);
        };
    }

    transaction!("line_begin", vec![
        // original action for w_any
        action!("c_nl", |lexer: &mut Lexer| {
            println!("action invoked for c_any");
        }),

        // original action for c_any
        action!("c_any", |lexer: &mut Lexer| {
            println!("action invoked for c_any");

            lexer.input_stream.simulate_fhold();
            lexer.state = LexingState::ExprValue;
        }),

        // TODO
        // original action for eof
        action!("c_eof", shared_actions.get("do_eof").unwrap().clone()),
    ]);


    transaction!("expr_value", vec![
        // original action for c_any
        action!("c_any", |lexer: &mut Lexer| {
            println!("action invoked for c_any");

            lexer.input_stream.simulate_fhold();
            lexer.state = LexingState::ExprBegin;
        }),

        // TODO
        // original action for eof
    ]);

    transaction!("expr_begin", vec![
        // original action for c_any
        action!("c_any", |lexer: &mut Lexer| {
                println!("action invoked for c_any");

                lexer.input_stream.hold_current_token();
                lexer.state = LexingState::ExprEnd;
            }
        )
    ]);

    transaction!("expr_end", vec![

        // original action for:
        //     [1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec

        action!("int_dec", |lexer: &mut Lexer| {
            println!("action invoked for int_dec");

            println!("current {:?}", lexer.input_stream.current_matched_token().unwrap());

            let parsed_int = lexer.input_stream.current_matched_token().unwrap().parse::<i64>().unwrap();

            lexer.emit_token(Token::T_INTEGER(parsed_int));
            lexer.flag_breaking();
        }),

        // TODO NOT CORRESPONDING
        // original action for:
        //     w_newline

        action!("c_eol", |lexer: &mut Lexer| {
            println!("action invoked for c_eol");

            // TODO
            // any
            // => { emit(:tNL, nil, @newline_s, @newline_s + 1)
            //     fhold; fnext line_begin; fbreak; };

            lexer.input_stream.simulate_fhold();
            lexer.state = LexingState::LineBegin;
            lexer.flag_breaking();
        }),

        // original action for:
        //     c_eof

        action!("c_eof", shared_actions.get("do_eof").unwrap().clone())

    ]);

    transactions
}
