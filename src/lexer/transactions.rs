// TODO the name `transaction` is misleading, it's a machine consists of transaction`s` actually

use std::collections::HashMap;

use regex::Regex;

use lexer::Lexer;
use lexer::lexing_state::LexingState;
use lexer::action::{Action, ActionProc};
use lexer::matching_patterns;
use lexer::shared_actions;
use lexer::machines;

use parser::parser::Token;

pub fn construct() -> HashMap<LexingState, Vec<Box<Action>>> {
    let patterns = matching_patterns::construct();
    let shared_actions = shared_actions::construct();

    let mut transactions = HashMap::new();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: patterns.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(), // TODO clone?
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

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

    transaction!("expr_value", vec![
        action!("w_space_comment", get_shared_action!("noop")),

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

        // original
        //     # if a: Statement if.
        //     keyword_modifier
        //     => { emit_table(KEYWORDS_BEGIN)
        //          fnext expr_value; fbreak; };
        action!("keyword_modifier", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords_begin");
            lexer.state = LexingState::ExprValue;
            lexer.flag_breaking();
        }),

        // original action
        //     keyword
        action!("keyword", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_token();
            lexer.state = LexingState::ExprEnd;
        }),

        // original action for c_any
        action!("c_any", |lexer: &mut Lexer| {
                println!("action invoked for c_any");

                lexer.input_stream.hold_current_token();
                lexer.state = LexingState::ExprEnd;
            }
        )
    ]);

    transaction!("expr_end", vec![

        //
        // KEYWORDS
        //

        // original action for: keyword_with_end
        action!("keyword_with_end", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.flag_breaking();
        }),

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

        action!("c_nl", |lexer: &mut Lexer| {
            println!("action invoked for w_newline/c_nl");

            lexer.state = LexingState::LeadingDot;
        }),

        // original action for:
        //     c_eof

        action!("c_eof", get_shared_action!("do_eof"))

    ]);

    transaction!("leading_dot", vec![
        //   # Insane leading dots:
        //   # a #comment
        //   #  .b: a.b
        //   c_space* %{ tm = p } ('.' | '&.')
        //   => { p = tm - 1; fgoto expr_end; };

        // original action for: any
        //   any
        //   => { emit(:tNL, nil, @newline_s, @newline_s + 1)
        //        fhold; fnext line_begin; fbreak; };

        action!("any", |lexer: &mut Lexer| {
            // TODO
            lexer.input_stream.simulate_fhold();
            lexer.state = LexingState::LineBegin;
            lexer.flag_breaking();
        }),
    ]);


    transaction!("expr_variable", machines::construct_machine_expr_variable(&patterns, &shared_actions));

    transaction!("line_begin", machines::construct_machine_line_begin(&patterns, &shared_actions));

    transactions
}
