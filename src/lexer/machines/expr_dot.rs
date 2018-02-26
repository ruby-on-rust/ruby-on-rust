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

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_dot( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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

        //     bareword ambiguous_fid_suffix
        //     => { emit(:tFID, tok(@ts, tm), @ts, tm)
        //           fnext *arg_or_cmdarg; p = tm - 1; fbreak; };
        // TODO NOTE
        // this is kinda of complex, since `ambiguous_fid_suffix` is an action embedding a proc
        // 
        //   ambiguous_fid_suffix =         # actual    parsed
        //       [?!]    %{ tm = p }      | # a?        a?
        //       [?!]'=' %{ tm = p - 2 }    # a!=b      a != b
        //   ;
        // 
        // so we will separate this action into 2
        // 
        // TODO NOTE
        // the challenge is, the original comment in lexer.rl
        // > 
        //   # These rules implement a form of manually defined lookahead.
        //   # The default longest-match scanning does not work here due
        //   # to sheer ambiguity.
        // and we haven't do anything about `non-longest-match scanning`,
        // still don't know if that will cause any issue
        // 

        box Action {
            regex: Regex::new(r"^[[:alpha:]][[:alnum:]]*[?!]").unwrap(),
            //                   ^                     ^
            //                   bareword
            procedure: |lexer: &mut Lexer| {
                let tm = lexer.input_stream.p;

                let token = Token::T_FID(lexer.input_stream.token_string_from_range( lexer.input_stream.ts.unwrap(), tm ));
                lexer.emit_token(token);

                let next_state = lexer.arg_or_cmdarg();
                lexer.push_next_state(next_state);

                // TODO NOTE
                // original p = tm - 1
                lexer.input_stream.p = tm;

                lexer.flag_breaking();
            }
        },
        box Action {
            regex: Regex::new(r"^[[:alpha:]][[:alnum:]]*[?!]=").unwrap(),
            procedure: |lexer: &mut Lexer| {
                let tm = lexer.input_stream.p - 2;

                let token = Token::T_FID(lexer.input_stream.token_string_from_range( lexer.input_stream.ts.unwrap(), tm ));
                lexer.emit_token(token);

                let next_state = lexer.arg_or_cmdarg();
                lexer.push_next_state(next_state);

                // TODO NOTE
                // original p = tm - 1
                lexer.input_stream.p = tm;

                lexer.flag_breaking();
            }
        },

        //     # See the comment in `expr_fname`.
        //     operator_fname      |
        //     operator_arithmetic |
        //     operator_rest
        //     => { emit_table(PUNCTUATION)
        //           fnext expr_arg; fbreak; };
        box Action {
            // TODO impl pattern_literals, for operator_fname, etc. and build pattern like this
            regex: Regex::new(r"^(\[\])|(\[\]=)|`|(-@)|(\+@)|(~@)|(!@)|(&)|(\|)|(&&)|(\|\|)|(\^)|(\+)|(-)|(\*)|(/)|(\*\*)|(~)|(<<)|(>>)|(%)|(=~)|(!~)|(==)|(!=)|(!)|(===)|(<)|(<=)|(>)|(>=)|(<=>)|(=>)").unwrap(),
            //                   ^                                   ^ ^                                                                  ^ ^                                                        ^
            //                   operator_fname                        _arithmetic                                                          _rest
            procedure: |lexer: &mut Lexer| {
                lexer.emit_token_from_table("punctuation");
                lexer.push_next_state(LexingState::ExprArg);
                lexer.flag_breaking();
            }
        },

        //     w_any;
        action!("w_any", get_shared_action!("noop")),

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
