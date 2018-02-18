// TRACKER
//   WIP

// # The previous token was an identifier which was seen while in the
// # command mode (that is, the state at the beginning of #advance was
// # expr_value). This state is very similar to expr_arg, but disambiguates
// # two very rare and specific condition:
// #   * In 1.8 mode, "foo (lambda do end)".
// #   * In 1.9+ mode, "f x: -> do foo do end end".
// expr_cmdarg := |*

//     w_space* 'do'
//     => {
//       if @cond.active?
//         emit(:kDO_COND, 'do'.freeze, @te - 2, @te)
//       else
//         emit(:kDO, 'do'.freeze, @te - 2, @te)
//       end
//       fnext expr_value; fbreak;
//     };

//     c_any             |
//     # Disambiguate with the `do' rule above.
//     w_space* bareword |
//     w_space* label
//     => { p = @ts - 1
//           fgoto expr_arg; };

//     c_eof => do_eof;
// *|;

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_cmdarg( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(), // TODO clone?
                procedure: $procedure
            }
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

    vec![
        //     w_space+ e_lparen
        //     => {
        //       emit(:tLPAREN_ARG, '('.freeze, @te - 1, @te)
        //       if version?(18)
        //         fnext expr_value; fbreak;
        //       else
        //         fnext expr_beg; fbreak;
        //       end
        //     };

    ]
}
