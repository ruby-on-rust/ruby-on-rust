use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

pub fn construct_machine_line_begin( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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
        //       w_any;
        action!("w_any", get_shared_action!("noop")),

        //       '=begin' ( c_space | c_nl_zlen )
        //       => { @eq_begin_s = @ts
        //            fgoto line_comment; };
        action_with_literal!(
            format!(r"=begin({}|{})", pattern_lit!("c_space"), pattern_lit!("c_nl_zlen")),
            |lexer: &mut Lexer| {
                // TODO @eq_begin_s
                lexer.push_next_state(state!("line_comment"));
            }
        ),

        //       '__END__' ( c_eol - zlen )
        //       => { p = pe - 3 };
        action_with_literal!(
            // TODO c_eol - zlen
            r"__END__\n",
            |lexer: &mut Lexer| {
                panic!("UNIMPL");
            }
        ),

        //       c_any
        //       => { fhold; fgoto expr_value; };
        action!("c_any", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("expr_value"));
        }),

        //       c_eof => do_eof;
        action!("c_nl", get_shared_action!("do_eof")),
    ]
}
