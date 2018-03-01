// # Special newline handling for "def a b:"

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;
use lexer::literal::Literal;

use parser::token::Token;

pub fn construct_machine_expr_labelarg( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

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

        //   w_space_comment;
        action!("w_space_comment", get_shared_action!("noop")),

        //   w_newline
        //   => {
        //     if @in_kwarg
        //       fhold; fgoto expr_end;
        //     else
        //       fgoto line_begin;
        //     end
        //   };
        action!("w_newline", |lexer: &mut Lexer| {
            if lexer.in_kwarg {
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_end"));
            } else {
                lexer.push_next_state(state!("line_begin"));
            }
        }),

        //   c_any
        //   => { fhold; fgoto expr_beg; };
        action!("c_any", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("expr_beg"));
        }),

        //   c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),

    ]
}
