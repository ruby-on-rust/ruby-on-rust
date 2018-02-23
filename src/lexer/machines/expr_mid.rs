// TRACKER
//   DONE

// # The rationale for this state is that several keywords accept value
// # (i.e. should transition to `expr_beg`), do not accept it like a command
// # (i.e. not an `expr_arg`), and must behave like a statement, that is,
// # accept a modifier if/while/etc.
// #

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_mid( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    // TODO merge action_with_literal! and pattern_lit!, so we can write code like:
    // pattern!("{}+|{}", "w_space", "foo")

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

    vec![
        //     keyword_modifier
        //     => { emit_table(KEYWORDS)
        //          fnext expr_beg; fbreak; };
        action!("keyword_modifier", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.push_next_state(state!("expr_beg"));
            lexer.flag_breaking();
        }),

        //     bareword
        //     => { p = @ts - 1; fgoto expr_beg; };
        action!("bareword", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_token();
            lexer.push_next_state(state!("expr_beg"));
        }),

        //     w_space_comment;
        action!("w_space_comment", get_shared_action!("noop")),

        //     w_newline
        //     => { fhold; fgoto expr_end; };
        action!("w_newline", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("expr_end"));
        }),

        //     c_any
        //     => { fhold; fgoto expr_beg; };
        action!("c_any", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("expr_beg"));
        }),

        //     c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),
    ]
}
