// # After literal function name in definition. Behaves like `expr_end`,
// # but allows a tLABEL.
// #
// # Transitions to `expr_end` afterwards.
// #

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_endfn( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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
        //     label ( any - ':' )
        //     => { emit(:tLABEL, tok(@ts, @te - 2), @ts, @te - 1)
        //          fhold; fnext expr_labelarg; fbreak; };

        // foo: bar
        // ^   ^ ^
        // ts    te
        //     te-2
        // 
        // tok(@ts, @te-2) -> `foo:`

        box Action {
            regex: Regex::new(r"^[[:alpha:]][[:alnum:]]*[\\?!]?:[^:]").unwrap(),
            procedure: |lexer: &mut Lexer| {
                let slice = lexer.input_stream.token_string_from_range( lexer.input_stream.ts.unwrap(), lexer.input_stream.te.unwrap() - 2 );
                let token = Token::T_LABLE(slice);
                lexer.emit_token(token);
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(LexingState::ExprLabelarg);
                lexer.flag_breaking();
            }
        },

        //     w_space_comment;
        action!("w_space_comment", get_shared_action!("noop")),

        //     c_any
        //     => { fhold; fgoto expr_end; };
        action!("c_any", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_char();
            lexer.push_next_state(state!("expr_end"))
        }),

        //     c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),
    ]
}
