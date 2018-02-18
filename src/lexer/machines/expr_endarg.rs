// TRACKER
//   DONE

// # The rationale for this state is pretty complex. Normally, if an argument
// # is passed to a command and then there is a block (tLCURLY...tRCURLY),
// # the block is attached to the innermost argument (`f` in `m f {}`), or it
// # is a parse error (`m 1 {}`). But there is a special case for passing a single
// # primary expression grouped with parentheses: if you write `m (1) {}` or
// # (2.0 only) `m () {}`, then the block is attached to `m`.
// #
// # Thus, we recognize the opening `(` of a command (remember, a command is
// # a method call without parens) as a tLPAREN_ARG; then, in parser, we recognize
// # `tLPAREN_ARG expr rparen` as a `primary_expr` and before rparen, set the
// # lexer's state to `expr_endarg`, which makes it emit the possibly following
// # `{` as `tLBRACE_ARG`.
// #
// # The default post-`expr_endarg` state is `expr_end`, so this state also handles
// # `do` (as `kDO_BLOCK` in `expr_beg`).

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_endarg( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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
        //     e_lbrace
        //     => {
        //     if @lambda_stack.last == @paren_nest
        //         @lambda_stack.pop
        //         emit(:tLAMBEG, '{'.freeze)
        //     else
        //         emit(:tLBRACE_ARG, '{'.freeze)
        //     end
        //     fnext expr_value;
        //     };
        action_with_literal!(pattern_lit!("e_lbrace"), |lexer: &mut Lexer| {
            if !lexer.lambda_stack.is_empty() && lexer.lambda_stack.last().unwrap() == &lexer.paren_nest {
                lexer.lambda_stack.pop().unwrap();
                lexer.emit_token(Token::T_LAMBEG); // TODO original token has value '{'
            } else {
                lexer.emit_token(Token::T_LBRACE_ARG); // TODO original token has value '{'
            }
            lexer.push_next_state(state!("expr_value"));
        }),

        //     'do'
        //     => { emit_do(true)
        //         fnext expr_value; fbreak; };
        action_with_literal!("do", |lexer: &mut Lexer| {
            lexer.emit_do(true);
            lexer.push_next_state(state!("expr_value"));
            lexer.flag_breaking();
        }),

        //     w_space_comment;
        action!("w_space_comment", get_shared_action!("noop")),

        //     c_any
        //     => { fhold; fgoto expr_end; };
        action_with_literal!("c_any", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("expr_end"));
        }),

        //     c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),
    ]
}
