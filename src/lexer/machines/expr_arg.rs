// # The previous token emitted was a `tIDENTIFIER` or `tFID`; no space
// # is consumed; the current expression is a command or method call.
// #

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_arg( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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

        //     #
        //     # COMMAND MODE SPECIFIC TOKENS
        //     #

        //     # cmd (1 + 2)
        //     # See below the rationale about expr_endarg.
        //     w_space+ e_lparen
        //     => {
        //       if version?(18)
        //         emit(:tLPAREN2, '('.freeze, @te - 1, @te)
        //         fnext expr_value; fbreak;
        //       else
        //         emit(:tLPAREN_ARG, '('.freeze, @te - 1, @te)
        //         fnext expr_beg; fbreak;
        //       end
        //     };
        //     with embedded:
        //         e_lparen
        // 
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+\[").unwrap(),
            //                   ^         ^
            //                   w_space
            procedure: |lexer: &mut Lexer| {
                lexer.invoke_proc("e_lparen");

                // NOTE ignored version18

                // TODO NOTE originally the token `tLPAREN2` contains a value '('
                lexer.emit_token(Token::T_LPAREN_ARG);

                lexer.push_next_state(state!("expr_beg"));
                lexer.flag_breaking();
            }
        },

        //     # meth(1 + 2)
        //     # Regular method call.
        //     e_lparen
        //     => { emit(:tLPAREN2, '('.freeze)
        //          fnext expr_beg; fbreak; };
        // with embedded
        //     e_lparen
        //
        action!("e_lparen", |lexer: &mut Lexer| {
            lexer.invoke_proc("e_lparen");
            lexer.push_next_state(state!("expr_beg"));
            lexer.flag_breaking();
        }),

        //     # meth [...]
        //     # Array argument. Compare with indexing `meth[...]`.
        //     w_space+ e_lbrack
        //     => { emit(:tLBRACK, '['.freeze, @te - 1, @te)
        //          fnext expr_beg; fbreak; };
        // with embedded
        //     e_lbrack
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+\[").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.invoke_proc("e_lbrack");
                // TODO NOTE originally the token `tLBRACK` contains a value '['
                lexer.push_next_state(state!("expr_beg"));
                lexer.flag_breaking();
            }
        },

        //     # cmd {}
        //     # Command: method call without parentheses.
        //     w_space* e_lbrace
        //     => {
        //       if @lambda_stack.last == @paren_nest
        //         p = @ts - 1
        //         fgoto expr_end;
        //       else
        //         emit(:tLCURLY, '{'.freeze, @te - 1, @te)
        //         fnext expr_value; fbreak;
        //       end
        //     };
        // with embedded
        //     e_lbrace
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]*\(").unwrap(),
            procedure: |lexer: &mut Lexer| {

                lexer.invoke_proc("e_lbrace");

                // TODO maybe unwrap()
                if lexer.lambda_stack.is_empty() || ( lexer.lambda_stack.last().unwrap() == &lexer.paren_nest ) {
                    lexer.input_stream.hold_current_token();
                    lexer.push_next_state(state!("expr_end"))
                } else {
                    // TODO NOTE originally the token `tLCURLY` contains a value '{'
                    lexer.emit_token(Token::T_LCURLY);
                    lexer.push_next_state(state!("expr_value"));
                    lexer.flag_breaking();
                }
            }
        },

        //     #
        //     # AMBIGUOUS TOKENS RESOLVED VIA EXPR_BEG
        //     #

        //     # a??
        //     # Ternary operator
        //     '?' c_space_nl
        //     => {
        //       # Unlike expr_beg as invoked in the next rule, do not warn
        //       p = @ts - 1
        //       fgoto expr_end;
        //     };
        box Action {
            regex: Regex::new(r"^\?[ \n\t\r\f\v]").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                lexer.push_next_state(state!("expr_end"))
            }
        },

        //     # a ?b, a? ?
        //     # Character literal or ternary operator
        //     w_space* '?'
        //     => { fhold; fgoto expr_beg; };
        box Action {
            regex: Regex::new(r"^[ \n\t\r\f\v]*\?").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_beg"));
            }
        },

        //     # a %{1}, a %[1] (but not "a %=1=" or "a % foo")
        //     # a /foo/ (but not "a / foo" or "a /=foo")
        //     # a <<HEREDOC
        //     w_space+ %{ tm = p }
        //     ( [%/] ( c_any - c_space_nl - '=' ) # /
        //     | '<<'
        //     )
        //     => {
        //       if tok(tm, tm + 1) == '/'.freeze
        //         # Ambiguous regexp literal.
        //         diagnostic :warning, :ambiguous_literal, nil, range(tm, tm + 1)
        //       end

        //       p = tm - 1
        //       fgoto expr_beg;
        //     };

        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+((%|/)[^ \n\t\r\f\v=]|(<<))").unwrap(),
            //                   ^              ^ [%/] (c_any-c_space_nl-=)
            procedure: |lexer: &mut Lexer| {

                // handle w_space+ %{ tm = p }
                let current_slice = lexer.input_stream.current_token().unwrap();
                let w_space_regex = Regex::new(r"^[ \t\r\f\v]+").unwrap();
                let w_space_len = w_space_regex.captures(&current_slice).unwrap().get(0).unwrap().as_str().chars().count();
                let tm = lexer.input_stream.ts.unwrap() + w_space_len;

                if lexer.input_stream.slice_from_range(tm, tm + 1) == String::from("/") {
                    // # Ambiguous regexp literal.
                    // diagnostic :warning, :ambiguous_literal, nil, range(tm, tm + 1)
                    // TODO
                    panic!("TODO");
                }

                lexer.input_stream.p = tm - 1;
                lexer.push_next_state(state!("expr_beg"));
            }
        },

        //     # x *1
        //     # Ambiguous splat, kwsplat or block-pass.
        //     w_space+ %{ tm = p } ( '+' | '-' | '*' | '&' | '**' )
        //     => {
        //       diagnostic :warning, :ambiguous_prefix, { :prefix => tok(tm, @te) },
        //                  range(tm, @te)

        //       p = tm - 1
        //       fgoto expr_beg;
        //     };
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+(\+|-|\*|&|(\*\*))").unwrap(),
            procedure: |lexer: &mut Lexer| {

                // handle w_space+ %{ tm = p }
                let current_slice = lexer.input_stream.current_token().unwrap();
                let w_space_regex = Regex::new(r"^[ \t\r\f\v]+").unwrap();
                let w_space_len = w_space_regex.captures(&current_slice).unwrap().get(0).unwrap().as_str().chars().count();
                let tm = lexer.input_stream.ts.unwrap() + w_space_len;

                // TODO handle diagnostic

                lexer.input_stream.p = tm - 1;
                lexer.push_next_state(state!("expr_beg"));
            }
        },

        //     # x ::Foo
        //     # Ambiguous toplevel constant access.
        //     w_space+ '::'
        //     => { fhold; fhold; fgoto expr_beg; };
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+::").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_char();
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_beg"));
            }
        },

        //     # x:b
        //     # Symbol.
        //     w_space* ':'
        //     => { fhold; fgoto expr_beg; };
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]*:").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_beg"));
            }
        },

        //     w_space+ label
        //     => { p = @ts - 1; fgoto expr_beg; };
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+[[:alpha:]][[:alnum:]]*[\?!]?:").unwrap(),
            procedure: |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                lexer.push_next_state(state!("expr_beg"));
            }
        },

        //     #
        //     # AMBIGUOUS TOKENS RESOLVED VIA EXPR_END
        //     #

        //     # a ? b
        //     # Ternary operator.
        //     w_space+ %{ tm = p } '?' c_space_nl
        //     => { p = tm - 1; fgoto expr_end; };
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]+\?[ \n\t\r\f\v]").unwrap(),
            procedure: |lexer: &mut Lexer| {
                // handle w_space+ %{ tm = p }
                let current_slice = lexer.input_stream.current_token().unwrap();
                let w_space_regex = Regex::new(r"^[ \t\r\f\v]+").unwrap();
                let w_space_len = w_space_regex.captures(&current_slice).unwrap().get(0).unwrap().as_str().chars().count();
                let tm = lexer.input_stream.ts.unwrap() + w_space_len;

                lexer.input_stream.p = tm - 1;
                lexer.push_next_state(state!("expr_end"))
            }
        },

        //     # x + 1: Binary operator or operator-assignment.
        //     w_space* operator_arithmetic
        //                 ( '=' | c_space_nl )?    |
        //     # x rescue y: Modifier keyword.
        //     w_space* keyword_modifier            |
        //     # a &. b: Safe navigation operator.
        //     w_space* '&.'                        |
        //     # Miscellanea.
        //     w_space* punctuation_end
        //     => {
        //       p = @ts - 1
        //       fgoto expr_end;
        //     };
        // NOTE
        // separated into 4 actions
        // 
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]*((&)|(\|)|(&&)|(\|\|)|(\^)|(\+)|(-)|(\*)|(/)|(\*\*)|(~)|(<<)|(>>)|(%))[= \n\t\r\f\v]").unwrap(),
            procedure: |lexer: &mut Lexer| { lexer.input_stream.hold_current_token(); lexer.push_next_state(state!("expr_end")) }
        },
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]*((if)|(unless)|(while)|(until)|(rescue))").unwrap(),
            procedure: |lexer: &mut Lexer| { lexer.input_stream.hold_current_token(); lexer.push_next_state(state!("expr_end")) }
        },
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]*&\.").unwrap(),
            procedure: |lexer: &mut Lexer| { lexer.input_stream.hold_current_token(); lexer.push_next_state(state!("expr_end")) }
        },
        box Action {
            regex: Regex::new(r"^[ \t\r\f\v]*((,)|(=)|(->)|(\()|(\[)|(\])|(::)|(\?)|(:)|(\.)|(\.\.)|(\.\.\.))").unwrap(),
            procedure: |lexer: &mut Lexer| { lexer.input_stream.hold_current_token(); lexer.push_next_state(state!("expr_end")) }
        },

        //     w_space;
        action!("w_space", get_shared_action!("noop")),

        //     w_comment
        //     => { fgoto expr_end; };
        action!("w_space", |lexer: &mut Lexer| { lexer.push_next_state(state!("expr_end")) }),

        //     w_newline
        //     => { fhold; fgoto expr_end; };
        action!("w_newline", |lexer: &mut Lexer| { lexer.push_next_state(state!("expr_end")) }),

        //     c_any
        //     => { fhold; fgoto expr_beg; };
        action!("c_any", |lexer: &mut Lexer| { lexer.push_next_state(state!("expr_beg")); }),

        //     c_eof => do_eof;
        action!("w_space", get_shared_action!("do_eof")),
    ]
}
