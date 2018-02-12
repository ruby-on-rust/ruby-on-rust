// TRACKER
//   WIP

// # Literal function name in method call (e.g. `a.class`).
// #
// # Transitions to `expr_arg` afterwards.
// #
// expr_dot := |*
//     constant
//     => { emit(:tCONSTANT)
//           fnext *arg_or_cmdarg; fbreak; };

//     call_or_var
//     => { emit(:tIDENTIFIER)
//           fnext *arg_or_cmdarg; fbreak; };

//     bareword ambiguous_fid_suffix
//     => { emit(:tFID, tok(@ts, tm), @ts, tm)
//           fnext *arg_or_cmdarg; p = tm - 1; fbreak; };

//     # See the comment in `expr_fname`.
//     operator_fname      |
//     operator_arithmetic |
//     operator_rest
//     => { emit_table(PUNCTUATION)
//           fnext expr_arg; fbreak; };

//     w_any;

//     c_any
//     => { fhold; fgoto expr_end; };

//     c_eof => do_eof;
// *|;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_dot( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {

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
        //     constant
        //     => { emit(:tCONSTANT)
        //           fnext *arg_or_cmdarg; fbreak; };
        action!("constant", |lexer: &mut Lexer| {
            let token = Token::T_CONSTANT(lexer.input_stream.current_token_string());
            lexer.emit_token(token);
            
            // simulating `fnext *arg_or_cmdarg;`
            let next_state = lexer.arg_or_cmdarg();
            lexer.push_next_state(next_state);
            lexer.flag_breaking();
        }),

        //     call_or_var
        //     => { emit(:tIDENTIFIER)
        //           fnext *arg_or_cmdarg; fbreak; };
        action!("call_or_var", |lexer: &mut Lexer| {
            let token = Token::T_IDENTIFIER(lexer.input_stream.current_token_string());
            lexer.emit_token(token);

            // simulating `fnext *arg_or_cmdarg;`
            let next_state = lexer.arg_or_cmdarg();
            lexer.push_next_state(next_state);
            lexer.flag_breaking();
        }),
    ]
}
