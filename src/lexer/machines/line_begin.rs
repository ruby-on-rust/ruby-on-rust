use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

pub fn construct_machine_line_begin( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {

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
        // original action for w_any
        action!("c_nl", |lexer: &mut Lexer| {
            println!("action invoked for c_any");
        }),

        // original action for c_any
        action!("c_any", |lexer: &mut Lexer| {
            println!("action invoked for c_any");

            lexer.input_stream.simulate_fhold();
            lexer.state = LexingState::ExprValue;
        }),

        // TODO
        // original action for eof
        action!("c_eof", get_shared_action!("do_eof")),
    ]
}
