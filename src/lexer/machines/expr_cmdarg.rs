// TRACKER
//   DONE

// # The previous token was an identifier which was seen while in the
// # command mode (that is, the state at the beginning of #advance was
// # expr_value). This state is very similar to expr_arg, but disambiguates
// # two very rare and specific condition:
// #   * In 1.8 mode, "foo (lambda do end)".
// #   * In 1.9+ mode, "f x: -> do foo do end end".

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_cmdarg( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

    // with pre-defined pattern and shared action
    // DEMO
    //     action!("w_any", "noop")
    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    // get pattern lit from pattern name
    // "w_space" -> "[ \t]"
    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    // TODO NOT WORKING
    // debugging here https://play.rust-lang.org/?gist=96e8e5ab66bb5c008c2d89b1e7625d46&version=undefined
    // 
    // DEMO
    //     build_pattern!(r"{}+", "w_any")
    //     build_pattern!(r"{}+|{}", "w_space", "foo")
    // NOTE
    //     build_pattern!("{}") (without r prefix), is not supported
    // macro_rules! build_pattern {
    //     (
    //         // $x:expr,
    //         // $( $y:expr ),*
    //         $format_base:expr,
    //         $( $pattern_name:expr ),*
    //     ) => {
    //         format!(
    //             $format_base,
    //             $(
    //                 patterns.get($pattern_name).unwrap()
    //             )*
    //         );
    //     };
    // }

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
        action_with_literal!(format!(r"{}+{}", pattern_lit!("w_space"), pattern_lit!("e_lparen")), |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_LPAREN_ARG);
            // NOTE ignored version 18
            lexer.push_next_state(state!("expr_beg"));
        }),

        //     w_space* 'do'
        //     => {
        //       if @cond.active?
        //         emit(:kDO_COND, 'do'.freeze, @te - 2, @te)
        //       else
        //         emit(:kDO, 'do'.freeze, @te - 2, @te)
        //       end
        //       fnext expr_value; fbreak;
        //     };
        action_with_literal!(format!(r"{}*do", pattern_lit!("w_space")), |lexer: &mut Lexer| {
            if lexer.cond.is_active() {
                lexer.emit_token(Token::K_DO_COND);
            } else {
                lexer.emit_token(Token::K_DO);
            }
            lexer.push_next_state(LexingState::ExprValue);
            lexer.flag_breaking();
        }),

        //     c_any             |
        //     # Disambiguate with the `do' rule above.
        //     w_space* bareword |
        //     w_space* label
        //     => { p = @ts - 1
        //           fgoto expr_arg; };

        //     c_eof => do_eof;
        action_with_literal!(format!(r"({})|({}*{})|({}*{})",
            pattern_lit!("c_any"),
            pattern_lit!("w_space"), pattern_lit!("bareword"),
            pattern_lit!("w_space"), pattern_lit!("label"),
            )
        , |lexer: &mut Lexer| {
            lexer.input_stream.hold_current_token();
            lexer.push_next_state(LexingState::ExprArg);
        }),
    ]
}
