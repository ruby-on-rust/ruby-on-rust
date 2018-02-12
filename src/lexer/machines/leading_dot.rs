// leading_dot := |*
//     # Insane leading dots:
//     # a #comment
//     #  .b: a.b
//     c_space* %{ tm = p } ('.' | '&.')
//     => { p = tm - 1; fgoto expr_end; };

//     any
//     => { emit(:tNL, nil, @newline_s, @newline_s + 1)
//          fhold; fnext line_begin; fbreak; };
// *|;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

pub fn construct_machine_leading_dot( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {

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
            lexer.push_next_state(LexingState::LineBegin);
            lexer.flag_breaking();
        }),
    ]
}
