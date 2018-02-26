// # Literal function name in definition (e.g. `def class`).
// # Keywords are returned as their respective tokens; this is used
// # to support singleton def `def self.foo`. Global variables are
// # returned as `tGVAR`; this is used in global variable alias
// # statements `alias $a $b`. Symbols are returned verbatim; this
// # is used in `alias :a :"b#{foo}"` and `undef :a`.
// #
// # Transitions to `expr_endfn` afterwards.
// #


use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_fname( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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
        // keyword
        // => { emit_table(KEYWORDS_BEGIN);
        //     fnext expr_endfn; fbreak; };
        action!("keyword", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords_begin");
            lexer.flag_breaking();
        }),

        //     constant
        //     => { emit(:tCONSTANT)
        //          fnext expr_endfn; fbreak; };
        action!("constant", |lexer: &mut Lexer| {
            let token =  Token::T_CONSTANT( lexer.input_stream.current_token_string() );
            lexer.emit_token(token);
            lexer.push_next_state(LexingState::ExprEndfn);
            lexer.flag_breaking();
        }),

        //     bareword [?=!]?
        //     => { emit(:tIDENTIFIER)
        //          fnext expr_endfn; fbreak; };
        box Action {
            regex: Regex::new(r"^[[:alpha:]][[:alnum:]]*[?=!]?").unwrap(),
            procedure: |lexer: &mut Lexer| {
                let token =  Token::T_IDENTIFIER( lexer.input_stream.current_token_string() );
                lexer.emit_token(token);
                lexer.push_next_state(LexingState::ExprEndfn);
                lexer.flag_breaking();
            }
        },

        //     global_var
        //     => { p = @ts - 1
        //          fnext expr_end; fcall expr_variable; };
        action!("global_var", |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_token();
            lexer.push_next_state(state!("expr_end"));
            lexer.push_next_state(LexingState::ExprVariable);
        }),

        //     # If the handling was to be delegated to expr_end,
        //     # these cases would transition to something else than
        //     # expr_endfn, which is incorrect.
        //     operator_fname      |
        //     operator_arithmetic |
        //     operator_rest
        //     => { emit_table(PUNCTUATION)
        //          fnext expr_endfn; fbreak; };
        // NOTE
        // separated into 3 actions,
        // since we dont have RegexGroup solution for calculation matching_patterns
        action!("operator_fname", |lexer: &mut Lexer|{ lexer.emit_token_from_table("punctation"); lexer.flag_breaking(); }),
        action!("operator_arithmetic", |lexer: &mut Lexer|{ lexer.emit_token_from_table("punctation"); lexer.flag_breaking(); }),
        action!("operator_rest", |lexer: &mut Lexer|{ lexer.emit_token_from_table("punctation"); lexer.flag_breaking(); }),

        //     '::'
        //     => { fhold; fhold; fgoto expr_end; };
        box Action {
            regex: Regex::new(r"^::").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_char();
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_end"))
            }
        },

        //     ':'
        //     => { fhold; fgoto expr_beg; };
        box Action {
            regex: Regex::new(r"^:").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_end"))
            }
        },

        //     '%s' c_any
        //     => {
        //       if version?(23)
        //         type, delimiter = tok[0..-2], tok[-1].chr
        //         fgoto *push_literal(type, delimiter, @ts);
        //       else
        //         p = @ts - 1
        //         fgoto expr_end;
        //       end
        //     };

        // TODO NOTE ignored version?(23), assuming 2.5
        box Action {
            regex: Regex::new(r"^%s.").unwrap(), // TODO using `.` for `c_any`
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                lexer.push_next_state(state!("expr_end"))
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
