// # Like expr_beg, but no 1.9 label or 2.2 quoted label possible.
// #
// expr_value := |*
//     # a:b: a(:b), a::B, A::B
//     label (any - ':')
//     => { p = @ts - 1
//          fgoto expr_end; };

//     # "bar", 'baz'
//     ['"] # '
//     => {
//       fgoto *push_literal(tok, tok, @ts);
//     };

//     w_space_comment;

//     w_newline
//     => { fgoto line_begin; };

//     c_any
//     => { fhold; fgoto expr_beg; };

//     c_eof => do_eof;
// *|;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

pub fn construct_machine_expr_value( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {

    // TODO 
    // share these macros for every machine constructing

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: patterns.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(), // TODO clone?
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
        action!("w_space_comment", get_shared_action!("noop")),

        // original action for c_any
        action!("c_any", |lexer: &mut Lexer| {
            println!("action invoked for c_any");

            lexer.input_stream.hold_current_char();
            lexer.push_next_state(LexingState::ExprBeg);
        }),

        // TODO
        // original action for eof
    ]
}
