use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_leading_dot( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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
        //       # Insane leading dots:
        //       # a #comment
        //       #  .b: a.b
        //       c_space* %{ tm = p } ('.' | '&.')
        //       => { p = tm - 1; fgoto expr_end; };
        action_with_literal!(
            format!(r"{}*(\.|(&\.))", pattern_lit!("c_space")),
            |lexer: &mut Lexer| {
                panic!("UNIMPL");
            }
        ),

        //       any
        //       => { emit(:tNL, nil, @newline_s, @newline_s + 1)
        //            fhold; fnext line_begin; fbreak; };
        action!("any", |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_NL);
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("line_begin"));
            lexer.flag_breaking();
        })
    ]
}
