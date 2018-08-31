//   # Like expr_beg, but no 1.9 label or 2.2 quoted label possible.

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;
use lexer::literal::Literal;

use parser::token::InteriorToken as Token;

pub fn construct_machine_expr_value( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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

        //       # a:b: a(:b), a::B, A::B
        //       label (any - ':')
        //       => { p = @ts - 1
        //            fgoto expr_end; };
        action_with_literal!(
            format!(r"{}[^:]", pattern_lit!("label")),
            |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                lexer.set_next_state(state!("expr_end"));
            }
        ),

        //       # "bar", 'baz'
        //       ['"] # '
        //       => {
        //         fgoto *push_literal(tok, tok, @ts);
        //       };
        action_with_literal!(
            "['\"]",
            |lexer: &mut Lexer| {
                println!("WTF current_token {:?}", lexer.input_stream.current_token());
                let lit_type = lexer.input_stream.current_token().unwrap();
                let lit_delimiter = lexer.input_stream.current_token().unwrap();

                let ts = lexer.input_stream.ts.unwrap();
                let mut literal = Literal::new( lit_type, lit_delimiter, ts, None, false, false, false );
                for token in literal.consume_tokens_to_emit() { lexer.emit_token(token); }

                let next_state = lexer.push_literal(literal);
                lexer.set_next_state(next_state);
            }
        ),

        //       w_space_comment;
        action!("w_space_comment", get_shared_action!("noop")),

        //       w_newline
        //       => { fgoto line_begin; };
        action!("w_newline", |lexer: &mut Lexer| { lexer.set_next_state(state!("line_begin")); }),

        //       c_any
        //       => { fhold; fgoto expr_beg; };
        action!("c_any", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.set_next_state(state!("expr_beg"));
        }),

        //   c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),
    ]
}
